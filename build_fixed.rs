use std::process::Command;

fn main() {
    // Generate version information
    let output = Command::new("git").args(["rev-parse", "HEAD"]).output();

    if let Ok(output) = output {
        if output.status.success() {
            let git_hash = String::from_utf8_lossy(&output.stdout);
            println!("cargo:rustc-env=GIT_HASH={}", git_hash.trim());
        }
    }

    // Only rerun if Cargo.toml or Cargo.lock changes
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=Cargo.lock");
}
