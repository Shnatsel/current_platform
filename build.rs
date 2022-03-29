use std::{io::BufRead, process::Command, env};

fn main() -> Result<(), std::io::Error>  {
    export_platform_triple(&query_host_triple()?);
    Ok(())
}

fn export_platform_triple(triple: &str) {
    println!("cargo:rustc-env=HOST_PLATFORM={}", triple);
}

fn query_host_triple() -> Result<String, std::io::Error> {
    let path_to_rustc = env::var("CARGO_BUILD_RUSTC").unwrap();
    let host_triple = Command::new(path_to_rustc)
        .arg("-vV")
        .output()?
        .stdout
        .lines()
        .map(|l| l.unwrap()) //rustc should not produce non-utf8 output
        .find(|l| l.starts_with("host: "))
        .map(|l| l[6..].to_string())
        .expect(
            "Failed to find the host platform in rustc output!\
                Please report a bug in `host_platform` crate.",
        );
    Ok(host_triple)
}