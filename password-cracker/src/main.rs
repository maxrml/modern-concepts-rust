#![feature(core_intrinsics)]
#![allow(internal_features)]
#![allow(unused_unsafe)]

use memmap2::Mmap;
use std::fs::File;
use std::sync::atomic::{AtomicUsize, AtomicPtr, Ordering};
use std::time::Instant;
use rayon::prelude::*;
use md5::{Md5, Digest};
use std::ptr;
use std::intrinsics::{likely, unlikely};
use sysinfo::{System, SystemExt, ProcessExt};
use std::env;

const MAX_LEN: usize = 255;

struct FoundResult {
    digest: u128,
    candidate: [u8; MAX_LEN + 1],
    len: usize,
    duration: std::time::Duration,
}

const TARGET_DIGESTS: [u128; 3] = [
    0x32c5c26e20908ebd80269d32f51cb5bb,
    0x648d5d9cc7cafe536fdbc6331f00c6a0,
    0xd31daf6579548a2a1bf5a9bd57b5bb89,
];

/// Runs the cracker against the given wordlist path and returns printed lines
fn crack_wordlist(path: &str) -> Vec<String> {
    let start_total = Instant::now();
    let num_targets = TARGET_DIGESTS.len();
    let found_count = AtomicUsize::new(0);
    let results: Vec<AtomicPtr<FoundResult>> = (0..num_targets)
        .map(|_| AtomicPtr::new(ptr::null_mut()))
        .collect();

    let file = File::open(path).expect("Cannot open wordlist");
    let mmap = unsafe { Mmap::map(&file).expect("Cannot mmap file") };
    let data = mmap.as_ref();
    let lines: Vec<&[u8]> = data.split(|&b| b == b'\n').collect();
    let affixes: &[u8] = b"!#+0123456789";

    lines.par_chunks(30_000).for_each(|chunk| {
        for &line in chunk {
            if unlikely(found_count.load(Ordering::Relaxed) >= num_targets) {
                break;
            }
            let len = line.len();
            if len == 0 || len > MAX_LEN {
                continue;
            }
            unsafe {
                // Original candidate
                let mut h = Md5::new(); h.update(line);
                let orig_digest = u128::from_be_bytes(h.finalize_reset().into());
                if likely(
                    orig_digest == TARGET_DIGESTS[0] ||
                        orig_digest == TARGET_DIGESTS[1] ||
                        orig_digest == TARGET_DIGESTS[2]
                ) {
                    let _ = store_result(orig_digest, line, &start_total, &found_count, &results);
                    continue;
                }
                // Suffix variants
                let mut buf = [0u8; MAX_LEN + 1];
                buf[..len].copy_from_slice(line);
                let mut base = Md5::new(); base.update(line);
                for &aff in affixes {
                    if unlikely(found_count.load(Ordering::Relaxed) >= num_targets) { break; }
                    buf[len] = aff;
                    let d = compute_suffix(&base, aff);
                    if likely(
                        d == TARGET_DIGESTS[0] ||
                            d == TARGET_DIGESTS[1] ||
                            d == TARGET_DIGESTS[2]
                    ) {
                        let _ = store_result(d, &buf[..len+1], &start_total, &found_count, &results);
                    }
                }
                // Prefix variants
                let mut buf = [0u8; MAX_LEN + 1];
                buf[1..len+1].copy_from_slice(line);
                for &aff in affixes {
                    if unlikely(found_count.load(Ordering::Relaxed) >= num_targets) { break; }
                    buf[0] = aff;
                    let d = compute_prefix(aff, line);
                    if likely(
                        d == TARGET_DIGESTS[0] ||
                            d == TARGET_DIGESTS[1] ||
                            d == TARGET_DIGESTS[2]
                    ) {
                        let _ = store_result(d, &buf[..len+1], &start_total, &found_count, &results);
                    }
                }
            }
        }
    });

    // Collect output with correct durations
    let mut output = Vec::new();
    for (i, &tg) in TARGET_DIGESTS.iter().enumerate() {
        let ptr = results[i].load(Ordering::SeqCst);
        if !ptr.is_null() {
            unsafe {
                let res = Box::from_raw(ptr);
                let pwd = std::str::from_utf8(&res.candidate[..res.len]).unwrap_or("invalid utf8");
                output.push(format!(
                    "Target Hash: {:032x} => Password: {} (Found in {:.6}s)",
                    res.digest,
                    pwd,
                    res.duration.as_secs_f64()
                ));
            }
        }
    }
    // Total run time
    let total = start_total.elapsed().as_secs_f64();
    output.push(format!("Total Time: {:.6}s", total));
    // Memory usage
    let mut sys = System::new_all(); sys.refresh_all();
    if let Some(p) = sys.process(sysinfo::get_current_pid().unwrap()) {
        output.push(format!("used Ram: {:.2} MB", p.memory() as f64 / 1024.0));
    }
    output
}

unsafe fn store_result(
    digest: u128,
    candidate: &[u8],
    start: &Instant,
    found_count: &AtomicUsize,
    results: &[AtomicPtr<FoundResult>],
) -> bool {
    for (i, &tg) in TARGET_DIGESTS.iter().enumerate() {
        if digest == tg && results[i].load(Ordering::Relaxed).is_null() {
            let mut b = Box::new(FoundResult {
                digest,
                candidate: [0; MAX_LEN + 1],
                len: candidate.len(),
                duration: start.elapsed(),
            });
            ptr::copy_nonoverlapping(candidate.as_ptr(), b.candidate.as_mut_ptr(), b.len);
            let ptr_box = Box::into_raw(b);
            if results[i].compare_exchange(ptr::null_mut(), ptr_box, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                found_count.fetch_add(1, Ordering::Relaxed);
                return true;
            } else {
                let _ = Box::from_raw(ptr_box);
            }
        }
    }
    false
}

unsafe fn compute_suffix(base: &Md5, aff: u8) -> u128 {
    let mut h = base.clone();
    h.update(&[aff]);
    u128::from_be_bytes(h.finalize_reset().into())
}

unsafe fn compute_prefix(aff: u8, slice: &[u8]) -> u128 {
    let mut h = Md5::new();
    h.update(&[aff]);
    h.update(slice);
    u128::from_be_bytes(h.finalize_reset().into())
}

fn main() -> std::io::Result<()> {
    let wordlist = env::args().nth(1).unwrap_or_else(|| "rockyou.txt".into());
    let results = crack_wordlist(&wordlist);
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::crack_wordlist;
    use std::fs;
    use std::env::temp_dir;

    #[test]
    fn check_passwords_in_output() {
        let mut path = temp_dir();
        path.push("test_wl.txt");
        fs::write(&path, "+ ann1792\n123mango\nMeatloaf9\n").unwrap();
        let out = crack_wordlist(path.to_str().unwrap());
        for pwd in &["+ ann1792", "123mango#", "Meatloaf9"] {
            assert!(out.iter().any(|l| l.contains(pwd)), "Expected '{}' in output, got {:?}", pwd, out);
        }
    }
}
