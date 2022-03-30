use std::env;

fn main() {
    // Cargo sets the host and target env vars for build scripts, but not crates:
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    // So we just re-export them to the crate code under a non-clashing name.
    export_var("HOST_PLATFORM", &env::var("HOST").unwrap());
    export_var("TARGET_PLATFORM", &env::var("TARGET").unwrap());
    // By default Cargo only runs the build script when a file changes.
    // This should make it re-run on target change (hopefully!)
     println!("cargo:rerun-if-changed-env=TARGET")
}

fn export_var(name: &str, value: &str) {
    println!("cargo:rustc-env={}={}", name, value);
}
