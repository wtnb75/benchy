# Benchmark support library for Rust

## Features

- iterator style
- result analysis
    - count, mean, mode
    - median, N-percentile (error 1-2%)
- getrusage
    - CPU time (usr/sys)
    - Memory (RSS usage, min/maj fault, swap)
    - I/O (block in/out, message send/recv)
    - Context switch (signals, voluntary/involuntary switches)

## Usage

Cargo.toml

```toml
[dependencies]
benchy = "0.1.0"
```

example

```rust
use benchy::Benchy;
use std::time::Duration;

for i in Benchy::new_duration(Duration::from_secs(5)) {
  // some works
}
// auto print result to stdout (5 secs after)
```

(TODO)

```
let mut bench = Benchy::new_duration(Duration::from_secs(5));

for i in bench {
  // some works
}
// show detailed result (5 secs after)
println!("median(50-th percentile): {}", bench.data.median().unwrap());
println!("90-th percentile: {}", bench.data.percentile(90.0).unwrap());
println!("95-th percentile: {}", bench.data.percentile(95.0).unwrap());
println!("99-th percentile: {}", bench.data.percentile(99.0).unwrap());

// rusage difference (end - start)
println!("rusage = {} {:?}", bench.usage, bench.usage);
```
