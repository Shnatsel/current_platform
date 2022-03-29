use std::env;

fn main() {
    // Cargo sets the host and target env vars for build scripts, but not crates:
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    export_var("HOST_PLATFORM", &query_host_triple());
    export_var("TARGET_PLATFORM", &query_target_triple());
}

fn export_var(name: &str, value: &str) {
    println!("cargo:rustc-env={}={}", name, value);
}

fn query_target_triple() -> String {
    env::var("TARGET").unwrap()
}

fn query_host_triple() -> String {
    env::var("HOST").unwrap()
}