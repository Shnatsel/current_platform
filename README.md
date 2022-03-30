# `current_platform`

**Find out what platform your code is running on, in Rust:**

```rust
use current_platform::CURRENT_PLATFORM;

fn main() {
    println!("Running on {}", CURRENT_PLATFORM);
}
```

Will print: `Running on x86_64-unknown-linux-gnu` (or whatever you run it on).

The target triple for the platform where the code was compiled is also included
as `COMPILED_ON`. It is only different from the `CURRENT_PLATFORM` if the code
was [cross-compiled.](https://en.wikipedia.org/wiki/Cross_compiler)
This is rarely useful, and is only included for completeness.
If in doubt, use `CURRENT_PLATFORM`.

Platform information is resolved **at compile time,**
based on the platform for which your code is compiled.
This crate incurs zero runtime cost.

This crate is intentionally minimal and only provides the target triples.
You can find out other properties of the platform using crates such as
[`platforms`](https://docs.rs/platforms/latest/platforms/)
 (auto-generated, always up to date) or
[`target-lexicon`](https://docs.rs/target-lexicon/latest/target_lexicon/)
(more detailed but may be missing newly added or obscure platforms).