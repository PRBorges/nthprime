use std::sync::RwLock;

pub struct NthPrime {
    pub memo_table: RwLock<Vec<u64>>,
    max_pos: usize,
}

impl NthPrime {
    pub fn new(max_pos: usize) -> NthPrime {
        NthPrime {
            memo_table: RwLock::new(vec![2, 3]),
            max_pos: usize::max(1, max_pos),
        }
    }

    // nth(n) is the nth 0-indexed prime
    pub fn nth(&self, n: usize) -> u64 {
        if n <= self.max_pos {
            if n >= self.memo_table.read().unwrap().len() {
                self.fill_table_to(n);
            }
            return self.memo_table.read().unwrap()[n];
        }
        
        self.fill_table_to(self.max_pos);
        self.nth_not_memoized(n)
    }

    // Memoize primes upto and including the nth
    fn fill_table_to(&self, end: usize) {
        loop {
            let (last_memoized, last_pos) = self.get_last_and_pos();
            if last_pos >= end {
                break;
            }
            let next_prime = self.prime_from(&[], last_memoized + 2);
            {
                // Memoize next_prime if not already memoized by another thread
                let mut locked_table = self.memo_table.write().unwrap();
                if last_pos + 1 == locked_table.len() {
                    locked_table.push(next_prime);
                }
            }
        }
    }

    // nth_not_memoized(n) is the nth prime, which can not be stored in memo_table
    // memo_table is assumed to be full
    fn nth_not_memoized(&self, n: usize) -> u64 {
        let mut last_found = self.memo_table.read().unwrap()[self.max_pos];
        let mut primes_not_in_table = Vec::with_capacity(n - self.max_pos);
        for _ in self.max_pos..n {
            last_found = self.prime_from(&primes_not_in_table, last_found + 2);
            primes_not_in_table.push(last_found);
        }
        last_found
    }

    // prime_from(primes_not_in_table,start) is the lowest prime p >= start
    // p is prime if  it does not have a factor neither in memo_table nor in primes_not_in_table
    fn prime_from(&self, primes_not_in_table: &[u64], start: u64) -> u64 {
        let mut candidate = start;
        loop {
            let highest_possible_factor = (candidate as f64).sqrt().ceil() as u64;
            if !has_factor_in(
                &self.memo_table.read().unwrap()[1..],
                highest_possible_factor,
                candidate,
            ) && !has_factor_in(primes_not_in_table, highest_possible_factor, candidate)
            {
                break;
            }
            candidate += 2;
        }

        candidate
    }

    // Gets the last prime memoized and its position in memo_table
    fn get_last_and_pos(&self) -> (u64, usize) {
        let locked_table = self.memo_table.read().unwrap();
        let last_pos = locked_table.len() - 1;
        (locked_table[last_pos], last_pos)
    }
}

// has_factor_in(factors,limit,x) == some f in factors divides x, with f <= limit
fn has_factor_in(factors: &[u64], limit: u64, x: u64) -> bool {
    factors
        .iter()
        .take_while(|&f| *f <= limit)
        .any(|f| x % f == 0)
}
