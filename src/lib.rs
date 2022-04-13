//! **Find out what platform your code is running on:**
//!
//! ```no_run
//! use current_platform::CURRENT_PLATFORM;
//!
//! fn main() {
//!     println!("Running on {}", CURRENT_PLATFORM);
//! }
//! ```
//! will print `Running on x86_64-unknown-linux-gnu` on desktop Linux.
//!
//! Platform information is resolved **at compile time,**
//! based on the platform for which your code is compiled.
//! It incurs **zero runtime cost.**
//!
//! The target triple for the platform where the code was compiled is also included,
//! see [`COMPILED_ON`](COMPILED_ON).
//!
//! This crate is intentionally minimal and only provides the target triple.
//! You can find out other properties of the platform using crates such as
//! [`platforms`](https://docs.rs/platforms/latest/platforms/)
//!  (auto-generated, always up to date) or
//! [`target-lexicon`](https://docs.rs/target-lexicon/latest/target_lexicon/)
//! (more detailed but may be missing newly added or obscure platforms).

/// The platform on which your code is running.
///
/// Also known as "target platform".

// I know what you're thinking:
// "I should have simply put `env!` in my binary instead of using this crate!"
// Not so fast: this variable is only available here because we exported it
// from build.rs, and trust me, if you have build.rs already used for something,
// figuring out the re-run logic for your build.rs is gonna be a pain.
// Hence this crate to provide a separate build.rs with its own self-contained `rerun-if`.
pub const CURRENT_PLATFORM: &str = env!("TARGET_PLATFORM");

/// The platform on which your code was compiled.
///
/// Also known as "host platform".
/// It will only be different from [`CURRENT_PLATFORM`](CURRENT_PLATFORM)
/// if the code was [cross-compiled.](https://en.wikipedia.org/wiki/Cross_compiler)
///
/// This is rarely needed; if in doubt, use [`CURRENT_PLATFORM`](CURRENT_PLATFORM).
pub const COMPILED_ON: &str = env!("HOST_PLATFORM");
