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
use std::intrinsics::likely;

const MAX_LEN: usize = 255;

// Wir speichern das Ergebnis in einem statisch-allokierten Puffer, um dynamische Allocationen zu vermeiden.
struct FoundResult {
    candidate: [u8; MAX_LEN + 1], // Enthält den Kandidaten (ggf. mit Affix)
    len: usize,                  // Tatsächliche Länge der Zeichenkette im Buffer
    duration: std::time::Duration,
}

// Die drei Ziel-Digests (müssen exakt so bleiben!)
const TARGET_DIGESTS: [[u8; 16]; 3] = [
    [0x32, 0xc5, 0xc2, 0x6e, 0x20, 0x90, 0x8e, 0xbd, 0x80, 0x26, 0x9d, 0x32, 0xf5, 0x1c, 0xb5, 0xbb],
    [0x64, 0x8d, 0x5d, 0x9c, 0xc7, 0xca, 0xfe, 0x53, 0x6f, 0xdb, 0xc6, 0x33, 0x1f, 0x00, 0xc6, 0xa0],
    [0xd3, 0x1d, 0xaf, 0x65, 0x79, 0x54, 0x8a, 0x2a, 0x1b, 0xf5, 0xa9, 0xbd, 0x57, 0xb5, 0xbb, 0x89],
];

/// Speichert eine Variante (Suffix oder Präfix) – nur im Erfolgsfall wird kopiert.
unsafe fn store_variant(
    candidate: &[u8],
    aff: u8,
    is_suffix: bool,
    start: Instant,
    found_count: &AtomicUsize,
    slot: &AtomicPtr<FoundResult>,
) -> bool {
    // Bestimme die Gesamtlänge der Variante: Original + 1 Byte Affix
    let new_len = candidate.len() + 1;
    if slot.load(Ordering::Relaxed).is_null() {
        let mut res_box = Box::new(FoundResult {
            candidate: [0u8; MAX_LEN + 1],
            len: new_len,
            duration: start.elapsed(),
        });
        if is_suffix {
            // Kopiere den Original-Kandidaten an den Anfang
            ptr::copy_nonoverlapping(candidate.as_ptr(), res_box.candidate.as_mut_ptr(), candidate.len());
            // Anhängen des Affix
            res_box.candidate[candidate.len()] = aff;
        } else {
            // Präfix: Schreibe das Affix an Position 0
            res_box.candidate[0] = aff;
            // Kopiere den Original-Kandidaten ab Index 1
            ptr::copy_nonoverlapping(candidate.as_ptr(), res_box.candidate.as_mut_ptr().add(1), candidate.len());
        }
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

/// Vergleicht den berechneten Digest mit den drei Zielwerten und ruft bei Übereinstimmung store_variant auf.
#[inline(always)]
unsafe fn check_digest_variant(
    digest: [u8; 16],
    candidate: &[u8],
    is_suffix: bool,
    start: Instant,
    found_count: &AtomicUsize,
    results: &[AtomicPtr<FoundResult>],
) -> bool {
    let mut hit = false;
    if likely(digest == TARGET_DIGESTS[0]) {
        if store_variant(candidate, candidate[candidate.len() - 1], is_suffix, start, found_count, &results[0]) {
            hit = true;
        }
    }
    if likely(digest == TARGET_DIGESTS[1]) {
        if store_variant(candidate, candidate[candidate.len() - 1], is_suffix, start, found_count, &results[1]) {
            hit = true;
        }
    }
    if likely(digest == TARGET_DIGESTS[2]) {
        if store_variant(candidate, candidate[candidate.len() - 1], is_suffix, start, found_count, &results[2]) {
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

    // Splitte die Datei nach Zeilen (hier wird ein Vec erstellt)
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
            // Der Kandidat als Slice aus dem Memory-map (keine Kopie)
            let candidate_slice = line;
            unsafe {
                // Original-Kandidat verarbeiten
                let mut hasher = Md5::new();
                hasher.update(candidate_slice);
                let orig_digest = hasher.finalize_reset();
                let orig_digest_arr: [u8; 16] = orig_digest.into();
                // Prüfe den Original-Kandidaten (ohne Affix)
                if orig_digest_arr == TARGET_DIGESTS[0]
                    || orig_digest_arr == TARGET_DIGESTS[1]
                    || orig_digest_arr == TARGET_DIGESTS[2]
                {
                    // Falls Match, kopieren wir den Original-Kandidaten in einen FoundResult
                    let _ = unsafe { store_variant(candidate_slice, 0, true, start, &found_count, &results[0]) }; // Hier als Beispiel Target 0
                    continue;
                }

                // Suffix-Varianten: Berechne den Hash direkt ohne Kopie
                for &aff in affixes {
                    if found_count.load(Ordering::Relaxed) >= num_targets {
                        break;
                    }
                    let mut h = Md5::new();
                    h.update(candidate_slice);
                    h.update(&[aff]);
                    let d = h.finalize_reset();
                    let d_arr: [u8; 16] = d.into();
                    // Wenn ein Ziel-Digest passt, wird in store_variant dann der Kandidat mit Affix kopiert.
                    if d_arr == TARGET_DIGESTS[0]
                        || d_arr == TARGET_DIGESTS[1]
                        || d_arr == TARGET_DIGESTS[2]
                    {
                        // Wir bauen die Variante (Original + Suffix) erst jetzt auf.
                        let mut variant = Vec::with_capacity(candidate_slice.len() + 1);
                        variant.extend_from_slice(candidate_slice);
                        variant.push(aff);
                        let _ = check_digest_variant(d_arr, &variant, true, start, &found_count, &results);
                    }
                }

                // Präfix-Varianten: Berechne den Hash direkt ohne Kopie
                for &aff in affixes {
                    if found_count.load(Ordering::Relaxed) >= num_targets {
                        break;
                    }
                    let mut h = Md5::new();
                    h.update(&[aff]);
                    h.update(candidate_slice);
                    let d = h.finalize_reset();
                    let d_arr: [u8; 16] = d.into();
                    if d_arr == TARGET_DIGESTS[0]
                        || d_arr == TARGET_DIGESTS[1]
                        || d_arr == TARGET_DIGESTS[2]
                    {
                        let mut variant = Vec::with_capacity(candidate_slice.len() + 1);
                        variant.push(aff);
                        variant.extend_from_slice(candidate_slice);
                        let _ = check_digest_variant(d_arr, &variant, false, start, &found_count, &results);
                    }
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
                    "Target Hash: {} => Password: {} (Found in {:?})",
                    hex::encode(target),
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
    Ok(())
}
