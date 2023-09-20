use nthprime::NthPrime;
use rand::Rng;
use std::{sync::Arc, thread};

#[test]
fn test_memoized_primes() {
    let ntp = NthPrime::new(10_000);
    assert_eq!(ntp.nth(0), 2);
    assert_eq!(ntp.nth(1), 3);
    assert_eq!(ntp.nth(10), 31);
    assert_eq!(ntp.nth(1_000), 7927);
    assert_eq!(ntp.nth(100), 547);
    assert_eq!(ntp.nth(3_000), 27457);
    assert_eq!(ntp.nth(10_000), 104_743);
}

#[test]
fn test_primes_not_memoized() {
    let ntp = NthPrime::new(1_000);
    assert_eq!(ntp.nth(3_000), 27457);
    assert_eq!(ntp.nth(10_000), 104_743);
    assert_eq!(ntp.nth(78_498), 1_000_003);
}

#[test]
fn test_multithreaded_same_as_single_threaded() {
    let n_threads = 200;
    let max_pos = 10_000;
    let max_n = 13_000;
    let mut rng = rand::thread_rng();

    // Generate n_threads random_args n and their corresponding nth(n) primes
    let ntp = NthPrime::new(max_pos);
    let mut random_args = Vec::with_capacity(n_threads);
    let mut single_threaded_primes = Vec::with_capacity(n_threads);
    for _ in 0..n_threads {
        let n = rng.gen_range(0..max_n);
        random_args.push(n);
        single_threaded_primes.push(ntp.nth(n));
    }

    // Generate  again the nth(n) primes, each in a separate thread, and store them  in multithreaded_primes
    let ntp = Arc::new(NthPrime::new(max_pos));
    let random_args = Arc::new(random_args);
    let mut multi_threaded_primes = Vec::with_capacity(n_threads);
    let mut handles = Vec::with_capacity(n_threads);
    for i in 0..n_threads {
        let shared_args = random_args.clone();
        let shared_ntp = ntp.clone();
        let handle = thread::spawn(move || shared_ntp.nth(shared_args[i]));
        handles.push(handle);
    }
    for handle in handles {
        multi_threaded_primes.push(handle.join().unwrap());
    }

    // Check that multithreaded_primes are the same as the single_threaded
    for i in 0..n_threads {
        assert_eq!(single_threaded_primes[i], multi_threaded_primes[i]);
    }
}
