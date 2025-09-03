# 📦 Installation Guide

This guide covers all the ways to install CodeGuardian on your system, from pre-built binaries to building from source.

## System Requirements

### Minimum Requirements
- **Operating System**: Linux, macOS, or Windows
- **Memory**: 512MB RAM (2GB+ recommended for large codebases)
- **Storage**: 100MB+ for cache and temporary files
- **Network**: Required for GitHub integration and dependency analysis

### Recommended Requirements
- **Memory**: 2GB+ RAM
- **CPU**: Multi-core processor
- **Storage**: 1GB+ for large project analysis
- **Network**: Stable internet connection

## Installation Methods

### Method 1: Cargo (Recommended)

#### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))

#### Installation
```bash
cargo install do-do-codeguardian
```

#### Verification
```bash
do-codeguardian --version
```

### Method 2: Pre-built Binaries

Download binaries from [GitHub Releases](https://github.com/d-oit/do-codeguardian/releases):

#### Linux
```bash
# Download latest release
curl -L -o do-codeguardian.tar.gz https://github.com/d-oit/do-codeguardian/releases/latest/download/do-codeguardian-linux-x64.tar.gz
tar -xzf do-codeguardian.tar.gz
sudo mv do-codeguardian /usr/local/bin/

# Verify installation
do-codeguardian --version
```

#### macOS (Intel)
```bash
curl -L -o do-codeguardian.tar.gz https://github.com/d-oit/do-codeguardian/releases/latest/download/do-codeguardian-macos-x64.tar.gz
tar -xzf do-codeguardian.tar.gz
sudo mv do-codeguardian /usr/local/bin/
```

#### macOS (Apple Silicon)
```bash
curl -L -o do-codeguardian.tar.gz https://github.com/d-oit/do-codeguardian/releases/latest/download/do-codeguardian-macos-arm64.tar.gz
tar -xzf do-codeguardian.tar.gz
sudo mv do-codeguardian /usr/local/bin/
```

#### Windows (PowerShell)
```powershell
# Download the latest release
Invoke-WebRequest -Uri "https://github.com/d-oit/do-codeguardian/releases/latest/download/do-codeguardian-windows-x64.zip" -OutFile "do-codeguardian.zip"

# Extract and move to PATH
Expand-Archive -Path "do-codeguardian.zip" -DestinationPath "."
Move-Item -Path "do-codeguardian.exe" -Destination "C:\Windows\System32\"
```

### Method 3: Docker

#### Prerequisites
- Docker installed ([Install Docker](https://docs.docker.com/get-docker/))

#### Pull Official Image
```bash
docker pull do-codeguardian/do-codeguardian:latest
```

#### Usage
```bash
# Run analysis on current directory
docker run --rm -v $(pwd):/workspace do-codeguardian/do-codeguardian check /workspace

# With custom configuration
docker run --rm -v $(pwd):/workspace -v $(pwd)/do-codeguardian.toml:/etc/do-codeguardian.toml do-codeguardian/do-codeguardian check /workspace
```

#### Building Custom Docker Image
```dockerfile
FROM do-codeguardian/do-codeguardian:latest

# Add custom tools or configurations
COPY custom-rules.json /etc/do-codeguardian/rules.json
COPY custom-config.toml /etc/do-codeguardian.toml

# Set environment variables
ENV CODEGUARDIAN_CONFIG=/etc/do-codeguardian.toml
```

### Method 4: Package Managers

#### Homebrew (macOS/Linux)
```bash
# Add tap
brew tap do-codeguardian/tap

# Install
brew install do-codeguardian

# Update
brew upgrade do-codeguardian
```

#### APT (Ubuntu/Debian)
```bash
# Add repository
curl -fsSL https://apt.do-codeguardian.dev/gpg.key | sudo gpg --dearmor -o /usr/share/keyrings/do-codeguardian-archive-keyring.gpg
echo "deb [signed-by=/usr/share/keyrings/do-codeguardian-archive-keyring.gpg] https://apt.do-codeguardian.dev stable main" | sudo tee /etc/apt/sources.list.d/do-codeguardian.list

# Install
sudo apt update
sudo apt install do-codeguardian
```

#### Chocolatey (Windows)
```powershell
choco install do-codeguardian
```

#### AUR (Arch Linux)
```bash
# Using yay
yay -S do-codeguardian

# Or using pacman with AUR helper
git clone https://aur.archlinux.org/do-codeguardian.git
cd do-codeguardian
makepkg -si
```

### Method 5: Build from Source

#### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Git
- Build tools (gcc, make, etc.)

#### Clone and Build
```bash
# Clone repository
git clone https://github.com/d-oit/do-codeguardian
cd do-codeguardian

# Build in debug mode
cargo build

# Build optimized release
cargo build --release

# Install locally
cargo install --path .
```

#### Development Build
```bash
# Clone with submodules if any
git clone --recursive https://github.com/d-oit/do-codeguardian
cd do-codeguardian

# Run tests during build
cargo build --release --tests

# Install with debug symbols
cargo install --path . --debug
```

## Post-Installation Setup

### 1. Verify Installation
```bash
# Check version
do-codeguardian --version

# View help
do-codeguardian --help

# Test basic functionality
do-codeguardian check --help
```

### 2. Initialize Configuration
```bash
# Create basic configuration
do-codeguardian init

# Create security-focused configuration
do-codeguardian init --template security

# Create CI-optimized configuration
do-codeguardian init --template ci
```

### 3. Set Up Environment Variables (Optional)
```bash
# For GitHub integration
export GITHUB_TOKEN=your_github_token_here

# For custom configuration
export CODEGUARDIAN_CONFIG=/path/to/do-codeguardian.toml

# For ML model path
export CODEGUARDIAN_ML_MODEL=/path/to/model.fann
```

### 4. Configure Shell Completion (Optional)
```bash
# Bash
do-codeguardian --completion bash >> ~/.bashrc

# Zsh
do-codeguardian --completion zsh >> ~/.zshrc

# Fish
do-codeguardian --completion fish > ~/.config/fish/completions/do-codeguardian.fish

# PowerShell
do-codeguardian --completion powershell >> $PROFILE
```

## Platform-Specific Notes

### Linux
- **Dependencies**: Ensure `libssl-dev` and `pkg-config` are installed
- **Permissions**: May need to run with `sudo` for system-wide installation
- **Path**: Add `~/.cargo/bin` to PATH if using Cargo installation

### macOS
- **Xcode**: Install Xcode command line tools: `xcode-select --install`
- **Homebrew**: Use Homebrew for managing dependencies
- **Security**: May need to allow downloaded binaries in System Preferences

### Windows
- **Visual Studio**: Install Visual Studio Build Tools for C++ compilation
- **Path**: Add Cargo bin directory to system PATH
- **Antivirus**: May flag Rust binaries - add exceptions as needed

### Docker
- **Performance**: Native installation is faster than Docker
- **File Permissions**: Use consistent user IDs between host and container
- **Volume Mounting**: Mount project directories with appropriate permissions

## Troubleshooting Installation

### Common Issues

**"cargo: command not found"**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**"Permission denied" on Linux/macOS**
```bash
# Install to user directory
cargo install --root ~/.local do-codeguardian

# Or use sudo for system installation
sudo cargo install do-do-codeguardian
```

**"SSL certificate error"**
```bash
# Update CA certificates
# Ubuntu/Debian
sudo apt install ca-certificates

# CentOS/RHEL
sudo yum install ca-certificates
```

**"Build failed" when building from source**
```bash
# Update Rust
rustup update stable

# Clean and rebuild
cargo clean
cargo build --release
```

### Getting Help
```bash
# Check installation logs
cargo install do-do-codeguardian --verbose

# Verify system compatibility
do-codeguardian doctor

# Get detailed help
do-codeguardian --help
```

## Next Steps

After successful installation:

1. **[Quick Start Guide](quick-start.md)** - Run your first analysis
2. **[Configuration Guide](configuration.md)** - Customize CodeGuardian for your needs
3. **[CI/CD Integration](ci-cd-integration.md)** - Integrate with your development pipeline

---

<div align="center">

**[⬅️ Back to User Guide](../README.md)** | **[🚀 Quick Start Guide](quick-start.md)** | **[⚙️ Configuration Guide](configuration.md)**

</div>
