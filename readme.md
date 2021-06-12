# rust-crypto-example

This repo shows rust-crypto examples.

## Feature

1. Secret Zeroize

Crate:

[zeroize](https://crates.io/crates/zeroize) to clear secret in memory.

[secrecy](https://crates.io/crates/secrecy) for secret management.

[Pin](https://doc.rust-lang.org/std/pin/struct.Pin.html) 

Reference:

[A pitfall of Rust's move/copy/drop semantics and zeroing data](https://benma.github.io/2020/10/16/rust-zeroize-move.html)

[Sanitize memory and CPU registers for sensitive data](https://github.com/rust-lang/rust/issues/17046)


2. Random Function

Crate:

[rand](https://crates.io/crates/rand)

[rdrand](https://crates.io/crates/rdrand) 

[ring](https://crates.io/crates/ring)

Reference:

[Rust Rand Book](https://rust-random.github.io/book/)

[Is CryptoRng useful](https://github.com/rust-random/rand/issues/543)

3. Const Time Compare

Crate:

[constant_time_eq](https://crates.io/crates/constant_time_eq)

Reference:

[How to compare strings in constant time?](https://stackoverflow.com/questions/44691363/how-to-compare-strings-in-constant-time)

[Annotate blocks that must run in constant time regardless of inputs](https://github.com/rust-lang/rfcs/issues/847)

## Reference

General:

[cryptocoding](https://github.com/veorq/cryptocoding)

[Guidelines for Mitigating Timing Side Channels Against Cryptographic Implementations](https://software.intel.com/content/www/us/en/develop/articles/software-security-guidance/secure-coding/mitigate-timing-side-channel-crypto-implementation.html)

## Known limitation
This package is only the sample code to show the concept. It does not have a full validation such as robustness functional test and fuzzing test. It does not meet the production quality yet. Any codes including the API definition, the libary and the drivers are subject to change.
