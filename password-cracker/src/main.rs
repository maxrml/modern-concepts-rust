#![feature(core_intrinsics)]

use memmap2::Mmap;
use std::fs::File;
use std::sync::atomic::{AtomicUsize, Ordering, AtomicPtr};
use std::time::Instant;
use rayon::prelude::*;
use md5::{Md5, Digest};
use std::ptr;
use std::intrinsics::likely;

const MAX_LEN: usize = 255;

// Wir speichern das Ergebnis in einem statisch-allokierten Puffer, um Allocationen zu vermeiden.
struct FoundResult {
    candidate: [u8; MAX_LEN + 1], // Kandidaten-Bytes im Puffer
    len: usize,                  // Tatsächliche Länge der Zeichenkette
    duration: std::time::Duration,
}

// Die drei Ziel-Digests (müssen exakt so bleiben!)
const TARGET_DIGESTS: [[u8; 16]; 3] = [
    [0x32, 0xc5, 0xc2, 0x6e, 0x20, 0x90, 0x8e, 0xbd, 0x80, 0x26, 0x9d, 0x32, 0xf5, 0x1c, 0xb5, 0xbb],
    [0x64, 0x8d, 0x5d, 0x9c, 0xc7, 0xca, 0xfe, 0x53, 0x6f, 0xdb, 0xc6, 0x33, 0x1f, 0x00, 0xc6, 0xa0],
    [0xd3, 0x1d, 0xaf, 0x65, 0x79, 0x54, 0x8a, 0x2a, 0x1b, 0xf5, 0xa9, 0xbd, 0x57, 0xb5, 0xbb, 0x89],
];

#[inline(always)]
unsafe fn check_target(
    digest: [u8; 16],
    candidate: *const u8,
    candidate_len: usize,
    start: Instant,
    found_count: &AtomicUsize,
    slot: &AtomicPtr<FoundResult>,
) -> bool {
    // Falls der Slot noch leer ist, erstellen wir ein Ergebnis und kopieren den Kandidaten hinein.
    if slot.load(Ordering::Relaxed).is_null() {
        let mut res_box = Box::new(FoundResult {
            candidate: [0u8; MAX_LEN + 1],
            len: candidate_len,
            duration: start.elapsed(),
        });
        ptr::copy_nonoverlapping(candidate, res_box.candidate.as_mut_ptr(), candidate_len);
        let res_ptr = Box::into_raw(res_box);
        if slot.compare_exchange(ptr::null_mut(), res_ptr, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            found_count.fetch_add(1, Ordering::Relaxed);
            return true;
        } else {
            // Falls jemand anderes den Slot belegt hat, geben wir den Speicher frei.
            let _ = Box::from_raw(res_ptr);
        }
    }
    false
}

#[inline(always)]
unsafe fn check_digest_inlined(
    digest: [u8; 16],
    candidate: *const u8,
    candidate_len: usize,
    start: Instant,
    found_count: &AtomicUsize,
    results: &[AtomicPtr<FoundResult>],
) -> bool {
    let mut hit = false;
    // Manuelles Unrollen für die 3 Zielwerte (Branch-Hinweise mit likely)
    if likely(digest == TARGET_DIGESTS[0]) {
        if check_target(digest, candidate, candidate_len, start, found_count, &results[0]) {
            hit = true;
        }
    }
    if likely(digest == TARGET_DIGESTS[1]) {
        if check_target(digest, candidate, candidate_len, start, found_count, &results[1]) {
            hit = true;
        }
    }
    if likely(digest == TARGET_DIGESTS[2]) {
        if check_target(digest, candidate, candidate_len, start, found_count, &results[2]) {
            hit = true;
        }
    }
    hit
}

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let num_targets = TARGET_DIGESTS.len();
    let found_count = AtomicUsize::new(0);
    let results: Vec<AtomicPtr<FoundResult>> = (0..num_targets)
        .map(|_| AtomicPtr::new(ptr::null_mut()))
        .collect();

    // Memory-map die rockyou.txt
    let file = File::open("rockyou.txt")?;
    let mmap = unsafe { Mmap::map(&file)? };
    let data = mmap.as_ref();

    // Splitte die Datei nach Zeilen (hier wird ein Vec erstellt, was man noch weiter optimieren könnte)
    let lines: Vec<&[u8]> = data.split(|&b| b == b'\n').collect();

    // Die 13 Affixe: '!', '+', '#' und '0'..'9'
    let affixes: &[u8] = b"!#+0123456789";

    // Verarbeite Zeilen in parallelen, disjunkten Chunks (Rayon)
    lines.par_chunks(5000).for_each(|chunk| {
        for line in chunk {
            if found_count.load(Ordering::Relaxed) >= num_targets {
                break;
            }
            let len = line.len();
            if len == 0 || len > MAX_LEN {
                continue;
            }
            unsafe {
                let candidate_ptr = line.as_ptr();

                // Original-Kandidat verarbeiten
                let mut hasher = Md5::new();
                hasher.update(std::slice::from_raw_parts(candidate_ptr, len));
                let orig_digest = hasher.finalize_reset();
                let orig_digest_arr: [u8; 16] = orig_digest.into();
                if check_digest_inlined(orig_digest_arr, candidate_ptr, len, start, &found_count, &results) {
                    continue;
                }

                // Lege einen stack-allokierten Puffer an (ohne Allocation) für Variantenerstellung
                let mut buf: [u8; MAX_LEN + 1] = [0u8; MAX_LEN + 1];

                // Suffix-Varianten: Kopiere den Kandidaten in den Puffer
                ptr::copy_nonoverlapping(candidate_ptr, buf.as_mut_ptr(), len);
                for &aff in affixes {
                    if found_count.load(Ordering::Relaxed) >= num_targets {
                        break;
                    }
                    buf[len] = aff;
                    let mut h = Md5::new();
                    h.update(&buf[..len + 1]);
                    let d = h.finalize_reset();
                    let d_arr: [u8; 16] = d.into();
                    check_digest_inlined(d_arr, buf.as_ptr(), len + 1, start, &found_count, &results);
                }

                // Präfix-Varianten: Kopiere den Kandidaten ab Index 1
                ptr::copy_nonoverlapping(candidate_ptr, buf.as_mut_ptr().add(1), len);
                for &aff in affixes {
                    if found_count.load(Ordering::Relaxed) >= num_targets {
                        break;
                    }
                    buf[0] = aff;
                    let mut h = Md5::new();
                    h.update(&buf[..len + 1]);
                    let d = h.finalize_reset();
                    let d_arr: [u8; 16] = d.into();
                    check_digest_inlined(d_arr, buf.as_ptr(), len + 1, start, &found_count, &results);
                }
            }
        }
    });

    let elapsed_total = start.elapsed();

    // Ergebnisse ausgeben
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
                    "✅ Target Hash: {} => Passwort: {} (Gefunden in {:?})",
                    hex::encode(target),
                    candidate_str,
                    res.duration
                );
            }
        }
    }
    if !found_any {
        println!("❌ Kein passendes Passwort gefunden.");
    }
    println!("Gesamte Suchzeit: {:?}", elapsed_total);
    Ok(())
}
