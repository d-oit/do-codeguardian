# Installation

## Prerequisites

- Rust 1.70 or later
- Git
- (Optional) GitHub CLI for enhanced features

## Installation Methods

### From Source

```bash
git clone https://github.com/your-org/codeguardian.git
cd codeguardian
cargo build --release
```

### Using Cargo

```bash
cargo install codeguardian
```

## Configuration

After installation, initialize CodeGuardian:

```bash
codeguardian init
```

This creates a default `codeguardian.toml` configuration file.
