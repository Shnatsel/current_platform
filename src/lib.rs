/// The platform on which your code is running.
///
/// Also known as "target platform".
pub const CURRENT_PLATFORM: &str = env!("TARGET_PLATFORM");

/// The platform on which your code was compiled.
///
/// Also known as "host platform".
/// It will only be different from [`CURRENT_PLATFORM`](CURRENT_PLATFORM) if the code was cross-compiled.
///
/// This is rarely needed; if in doubt, use [`CURRENT_PLATFORM`](CURRENT_PLATFORM).
pub const COMPILED_ON: &str = env!("HOST_PLATFORM");
