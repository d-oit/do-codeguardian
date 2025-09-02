use std::process::Command;

fn main() {
    // Ensure all dependencies are properly resolved
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=Cargo.lock");

    // Pre-build all features to ensure CodeQL can analyze them
    let features = vec![
        "default",
        "git",
        "security",
        "logging",
        "hashing",
        "cargo-audit",
        "ml",
        "ast",
        "ml-enhanced",
        "full",
    ];

    for feature in features {
        let status = Command::new("cargo")
            .args(["check", "--features", feature, "--all-targets"])
            .status();

        if let Ok(exit_status) = status {
            if !exit_status.success() {
                println!("cargo:warning=Failed to check feature: {}", feature);
            }
        }
    }

    // Generate version information
    let output = Command::new("git").args(["rev-parse", "HEAD"]).output();

    if let Ok(output) = output {
        if output.status.success() {
            let git_hash = String::from_utf8_lossy(&output.stdout);
            println!("cargo:rustc-env=GIT_HASH={}", git_hash.trim());
        }
    }
}
