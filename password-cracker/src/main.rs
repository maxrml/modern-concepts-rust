use memmap2::Mmap;
use std::fs::File;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::collections::{HashSet, HashMap};
use std::time::Instant;
use rayon::prelude::*;
use md5;
use std::convert::TryInto;
use hex;
use std::ptr;

const MAX_LEN: usize = 255;

#[inline(always)]
fn hex_to_digest(s: &str) -> [u8; 16] {
    let bytes = hex::decode(s).expect("Invalid hex string");
    bytes[..16].try_into().expect("Slice with incorrect length")
}

#[inline(always)]
unsafe fn check_candidate_unsafe(
    candidate: *const u8,
    len: usize,
    target_set: &HashSet<[u8; 16]>,
    found_count: &AtomicUsize,
    found_passwords: &Arc<Mutex<HashMap<[u8; 16], (Vec<u8>, std::time::Duration)>>>,
    start: Instant,
) {
    let digest = md5::compute(std::slice::from_raw_parts(candidate, len));
    let digest_arr: [u8; 16] = digest.into();

    if target_set.contains(&digest_arr) {
        let mut lock = found_passwords.lock().unwrap();
        if !lock.contains_key(&digest_arr) {
            lock.insert(digest_arr, (std::slice::from_raw_parts(candidate, len).to_vec(), start.elapsed()));
            found_count.fetch_add(1, Ordering::Relaxed);
        }
    }
}

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    let target_hashes = vec![
        "32c5c26e20908ebd80269d32f51cb5bb",
        "648d5d9cc7cafe536fdbc6331f00c6a0",
        "d31daf6579548a2a1bf5a9bd57b5bb89",
    ];
    let target_set: HashSet<[u8; 16]> = target_hashes.iter().map(|s| hex_to_digest(s)).collect();
    let total_targets = target_set.len();

    let found_count = AtomicUsize::new(0);
    let found_passwords = Arc::new(Mutex::new(HashMap::new()));

    let file = File::open("rockyou.txt")?;
    let mmap = unsafe { Mmap::map(&file)? };
    let data = mmap.as_ref();

    let lines: Vec<&[u8]> = data.split(|&b| b == b'\n').collect();

    lines.par_chunks(5000).for_each(|chunk| {
        for line in chunk {
            if found_count.load(Ordering::Relaxed) >= total_targets { return; }
            let len = line.len();
            if len == 0 || len > MAX_LEN { continue; }

            unsafe {
                let candidate_ptr = line.as_ptr();

                check_candidate_unsafe(candidate_ptr, len, &target_set, &found_count, &found_passwords, start);
                if found_count.load(Ordering::Relaxed) >= total_targets { return; }

                let special_chars: [u8; 3] = [b'!', b'+', b'#'];
                let digits: [u8; 10] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];

                let mut buf = [0u8; MAX_LEN + 1];

                for &ch in &special_chars {
                    if found_count.load(Ordering::Relaxed) >= total_targets { return; }

                    if len + 1 <= MAX_LEN {
                        ptr::copy_nonoverlapping(candidate_ptr, buf.as_mut_ptr().add(1), len);
                        buf[0] = ch;
                        check_candidate_unsafe(buf.as_ptr(), len + 1, &target_set, &found_count, &found_passwords, start);
                    }

                    if len + 1 <= MAX_LEN {
                        ptr::copy_nonoverlapping(candidate_ptr, buf.as_mut_ptr(), len);
                        buf[len] = ch;
                        check_candidate_unsafe(buf.as_ptr(), len + 1, &target_set, &found_count, &found_passwords, start);
                    }
                }

                for &d in &digits {
                    if found_count.load(Ordering::Relaxed) >= total_targets { return; }

                    if len + 1 <= MAX_LEN {
                        ptr::copy_nonoverlapping(candidate_ptr, buf.as_mut_ptr().add(1), len);
                        buf[0] = d;
                        check_candidate_unsafe(buf.as_ptr(), len + 1, &target_set, &found_count, &found_passwords, start);
                    }

                    if len + 1 <= MAX_LEN {
                        ptr::copy_nonoverlapping(candidate_ptr, buf.as_mut_ptr(), len);
                        buf[len] = d;
                        check_candidate_unsafe(buf.as_ptr(), len + 1, &target_set, &found_count, &found_passwords, start);
                    }
                }
            }
        }
    });

    let elapsed_total = start.elapsed();

    let found = found_passwords.lock().unwrap();
    if found.is_empty() {
        println!(" Kein passendes Passwort gefunden.");
    } else {
        println!("===== Ergebnisse =====");
        for (digest, (password, duration)) in found.iter() {
            println!(" Hash: {} => Passwort: {} (Gefunden in {:?})",
                     hex::encode(digest),
                     String::from_utf8_lossy(password),
                     duration);
        }
    }
    println!("Gesamte Suchzeit: {:?}", elapsed_total);

    Ok(())
}
