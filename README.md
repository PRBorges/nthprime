# Nthprime

Calculating the nth prime number, with memoization and thread-safety.
The 0th prime is 2.

## Usage

```rust
use nthprime::NthPrime;
let ntp = NthPrime::new(1_000);
let prime = ntp.nth(500);
```

Read my blog post: [Calculating the nth prime in Rust, with memoization and thread-safety](https://www.prborges.com/2023/nth-prime-thread-safe-rust/).
