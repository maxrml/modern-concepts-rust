#![feature(core_intrinsics)]
#![allow(internal_features)]
#![allow(unused_unsafe)]

use memmap2::Mmap;
use std::fs::File;
use std::sync::atomic::{AtomicUsize, Ordering, AtomicPtr};
use std::time::Instant;
use rayon::prelude::*;
use md5::{Md5, Digest};
use std::ptr;
use std::intrinsics::{likely, unlikely};
use sysinfo::{System, SystemExt, ProcessExt, PidExt}; // sysinfo import

const MAX_LEN: usize = 255;

struct FoundResult {
    candidate: [u8; MAX_LEN + 1], // Staatisch Allokierter Puffer
    len: usize,                   // Länge des Strings
    duration: std::time::Duration,
}

const TARGET_DIGESTS: [u128; 3] = [
    0x32c5c26e20908ebd80269d32f51cb5bb,
    0x648d5d9cc7cafe536fdbc6331f00c6a0,
    0xd31daf6579548a2a1bf5a9bd57b5bb89,
];

/// Speichert den fertigen Kandidaten, falls der Slot noch leer ist.
unsafe fn store_variant(
    candidate: &[u8],
    start: Instant,
    found_count: &AtomicUsize,
    slot: &AtomicPtr<FoundResult>,
) -> bool {
    if slot.load(Ordering::Relaxed).is_null() {
        let mut res_box = Box::new(FoundResult {
            candidate: [0u8; MAX_LEN + 1],
            len: candidate.len(),
            duration: start.elapsed(),
        });
        ptr::copy_nonoverlapping(candidate.as_ptr(), res_box.candidate.as_mut_ptr(), candidate.len());
        let res_ptr = Box::into_raw(res_box);
        if slot.compare_exchange(ptr::null_mut(), res_ptr, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            found_count.fetch_add(1, Ordering::Relaxed);
            return true;
        } else {
            let _ = Box::from_raw(res_ptr);
        }
    }
    false
}

/// Vergleicht den Digest mit den Zielwerten und ruft bei Übereinstimmung store_variant auf.
#[inline(always)]
unsafe fn check_digest_variant(
    digest: u128,
    candidate: &[u8],
    start: Instant,
    found_count: &AtomicUsize,
    results: &[AtomicPtr<FoundResult>],
) -> bool {
    let mut hit = false;
    if likely(digest == TARGET_DIGESTS[0]) {
        if store_variant(candidate, start, found_count, &results[0]) {
            hit = true;
        }
    }
    if likely(digest == TARGET_DIGESTS[1]) {
        if store_variant(candidate, start, found_count, &results[1]) {
            hit = true;
        }
    }
    if likely(digest == TARGET_DIGESTS[2]) {
        if store_variant(candidate, start, found_count, &results[2]) {
            hit = true;
        }
    }
    hit
}

/// Hilfsfunktion für die Suffix-Varianten, welche den bereits mit dem Kandidaten initialisierten MD5-Kontext klont,
/// das Affix anhängt und den Hash zurückgibt.
#[inline(always)]
unsafe fn compute_suffix_hash(base_hasher: &Md5, aff: u8) -> u128 {
    let mut h = base_hasher.clone();
    h.update(&[aff]);
    let d_arr: [u8; 16] = h.finalize_reset().into();
    u128::from_be_bytes(d_arr)
}

/// Hilfsfunktion für Präfix-Varianten. Hier muss ein neuer MD5-Kontext erstellt werden.
#[inline(always)]
unsafe fn compute_prefix_hash(aff: u8, candidate_slice: &[u8]) -> u128 {
    let mut h = Md5::new();
    h.update(&[aff]);
    h.update(candidate_slice);
    let d_arr: [u8; 16] = h.finalize_reset().into();
    u128::from_be_bytes(d_arr)
}

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let num_targets = TARGET_DIGESTS.len();
    let found_count = AtomicUsize::new(0);
    let results: Vec<AtomicPtr<FoundResult>> = (0..num_targets)
        .map(|_| AtomicPtr::new(ptr::null_mut()))
        .collect();

    // Memory Mapping of "rockyou.txt"
    let file = File::open("rockyou.txt")?;
    let mmap = unsafe { Mmap::map(&file)? };
    let data = mmap.as_ref();
    let lines: Vec<&[u8]> = data.split(|&b| b == b'\n').collect();
    let affixes: &[u8] = b"!#+0123456789";

    lines.par_chunks(30000).for_each(|chunk| {
        for line in chunk {
            if unlikely(found_count.load(Ordering::Relaxed) >= num_targets) {
                break;
            }
            let len = line.len();
            if len == 0 || len > MAX_LEN {
                continue;
            }
            let candidate_slice = line;
            unsafe {
                // --- Original-Kandidat ---
                let mut hasher = Md5::new();
                hasher.update(candidate_slice);
                let orig_digest_arr: [u8; 16] = hasher.finalize_reset().into();
                let orig_digest = u128::from_be_bytes(orig_digest_arr);
                if likely(orig_digest == TARGET_DIGESTS[0]
                    || orig_digest == TARGET_DIGESTS[1]
                    || orig_digest == TARGET_DIGESTS[2])
                {
                    let _ = store_variant(candidate_slice, start, &found_count, &results[0]);
                    continue;
                }

                // --- Suffix-Varianten ---
                let mut variant_buf = [0u8; MAX_LEN + 1];
                variant_buf[..candidate_slice.len()].copy_from_slice(candidate_slice);
                let mut base_hasher = Md5::new();
                base_hasher.update(candidate_slice);
                for &aff in affixes {
                    if unlikely(found_count.load(Ordering::Relaxed) >= num_targets) {
                        break;
                    }
                    variant_buf[candidate_slice.len()] = aff;
                    let variant_slice = &variant_buf[..candidate_slice.len() + 1];
                    let digest_u128 = compute_suffix_hash(&base_hasher, aff);
                    if likely(digest_u128 == TARGET_DIGESTS[0]
                        || digest_u128 == TARGET_DIGESTS[1]
                        || digest_u128 == TARGET_DIGESTS[2])
                    {
                        let _ = check_digest_variant(digest_u128, variant_slice, start, &found_count, &results);
                    }
                }

                // --- Präfix-Varianten ---
                let mut variant_buf = [0u8; MAX_LEN + 1];
                variant_buf[1..candidate_slice.len() + 1].copy_from_slice(candidate_slice);
                for &aff in affixes {
                    if unlikely(found_count.load(Ordering::Relaxed) >= num_targets) {
                        break;
                    }
                    variant_buf[0] = aff;
                    let variant_slice = &variant_buf[..candidate_slice.len() + 1];
                    let digest_u128 = compute_prefix_hash(aff, candidate_slice);
                    if likely(digest_u128 == TARGET_DIGESTS[0]
                        || digest_u128 == TARGET_DIGESTS[1]
                        || digest_u128 == TARGET_DIGESTS[2])
                    {
                        let _ = check_digest_variant(digest_u128, variant_slice, start, &found_count, &results);
                    }
                }
            }
        }
    });

    let elapsed_total = start.elapsed();

    let mut found_any = false;
    for (i, target) in TARGET_DIGESTS.iter().enumerate() {
        let slot = &results[i];
        let ptr = slot.load(Ordering::SeqCst);
        if !ptr.is_null() {
            found_any = true;
            unsafe {
                let res = Box::from_raw(ptr);
                let candidate_str = std::str::from_utf8(&res.candidate[..res.len]).unwrap_or("Invalid UTF-8");
                println!(
                    "Target Hash: {:032x} => Password: {} (Found in {:?})",
                    target,
                    candidate_str,
                    res.duration
                );
            }
        }
    }
    if !found_any {
        println!("No passwords were found");
    }
    println!("Total Time: {:?}", elapsed_total);

    // Ausgabe des aktuell verwendeten Speichers mithilfe von sysinfo
    let mut sys = System::new_all();
    sys.refresh_all();
    let pid = sysinfo::get_current_pid().expect("Could't detect current pid");
    if let Some(process) = sys.process((pid.as_u32() as usize).into()) {
        let mem_kb = process.memory();
        let mem_mb = mem_kb as f64 / 1024.0;
        let mem_gb = mem_mb / 1024.0;
        println!(
            "used Ram: {} Byte (~{:.2} kB, ~{:.2} mB)",
            mem_kb, mem_mb, mem_gb
        );
    } else {
        println!("Process not found.");
    }

    Ok(())
}
