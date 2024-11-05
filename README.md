# miniz-oxide-slowdown-repro

This repository is a reproduction of a slowdown in `miniz-oxide` when compiled against Rust 1.82 (or later).

## assumptions / design

- We use a seeded RNG to generate our corpus (32MB of random data) used for compression.
- We pre-allocate our output buffer to the maximum possible size (32MB) to avoid reallocations at least for the
  compressed data.
- We use `flate2` with the `rust_backend`) which utilizes `miniz-oxide`.
- We use Zlib with the default compression level (6).
- We do 32 rounds of compressing the input corpus to ensure we have a good sample size.
- We set our release profile (in `Cargo.toml`) to use all the goodies -- fat LTO, `codegen-units = 1`, etc -- to ensure
  we're (theoretically) generating the most optimized binary possible.

## steps to run

```shell
# This requires that you use rustup so automatically download/utilize the correct version(s) of Rust.
#
# Both 1.80 and 1.81 should report relatively close times for compressing the corpus:
cargo +1.80.0 run --release
cargo +1.81.0 run --release

# And then 1.82 should be significantly slower:
cargo +1.82.0 run --release
```

## results

```shell
toby@consigliera:~/src/miniz-oxide-slowdown-repro$ cargo +1.80.0 run --release
    Finished `release` profile [optimized] target(s) in 0.06s
     Running `target/release/miniz-oxide-slowdown-repro`
Took 23.863655ms to generate 32MB of random data.
Compressed 32MB of random data to 33559723 bytes in 842.367412ms.
Compressed 32MB of random data to 33559723 bytes in 830.114081ms.
Compressed 32MB of random data to 33559723 bytes in 830.797112ms.
Compressed 32MB of random data to 33559723 bytes in 830.294565ms.
Compressed 32MB of random data to 33559723 bytes in 830.416938ms.

toby@consigliera:~/src/miniz-oxide-slowdown-repro$ cargo +1.81.0 run --release
    Finished `release` profile [optimized] target(s) in 0.07s
     Running `target/release/miniz-oxide-slowdown-repro`
Took 23.404064ms to generate 32MB of random data.
Compressed 32MB of random data to 33559723 bytes in 794.716176ms.
Compressed 32MB of random data to 33559723 bytes in 786.134402ms.
Compressed 32MB of random data to 33559723 bytes in 781.398314ms.
Compressed 32MB of random data to 33559723 bytes in 786.829447ms.
Compressed 32MB of random data to 33559723 bytes in 787.045553ms.

toby@consigliera:~/src/miniz-oxide-slowdown-repro$ cargo +1.82.0 run --release
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/miniz-oxide-slowdown-repro`
Took 23.777673ms to generate 32MB of random data.
Compressed 32MB of random data to 33559723 bytes in 1.278123141s.
Compressed 32MB of random data to 33559723 bytes in 1.269700129s.
Compressed 32MB of random data to 33559723 bytes in 1.258582504s.
Compressed 32MB of random data to 33559723 bytes in 1.257916238s.
Compressed 32MB of random data to 33559723 bytes in 1.25774714s.

toby@consigliera:~/src/miniz-oxide-slowdown-repro$ cargo +beta run --release
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/miniz-oxide-slowdown-repro`
Took 23.144375ms to generate 32MB of random data.
Compressed 32MB of random data to 33559723 bytes in 1.273616581s.
Compressed 32MB of random data to 33559723 bytes in 1.267410571s.
Compressed 32MB of random data to 33559723 bytes in 1.263568728s.
Compressed 32MB of random data to 33559723 bytes in 1.258374865s.
Compressed 32MB of random data to 33559723 bytes in 1.259059571s.

toby@consigliera:~/src/miniz-oxide-slowdown-repro$ cargo +nightly run --release
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/miniz-oxide-slowdown-repro`
Took 23.540405ms to generate 32MB of random data.
Compressed 32MB of random data to 33559723 bytes in 1.265218248s.
Compressed 32MB of random data to 33559723 bytes in 1.250433046s.
Compressed 32MB of random data to 33559723 bytes in 1.24980345s.
Compressed 32MB of random data to 33559723 bytes in 1.24978543s.
Compressed 32MB of random data to 33559723 bytes in 1.250133197s.
```
