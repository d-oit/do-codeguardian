# Cross-Platform Requirements and Setup

This document outlines the platform-specific requirements and configurations for building and running CodeGuardian across different operating systems and architectures.

## Supported Platforms

CodeGuardian supports the following target platforms:

### Native Builds (Full Testing)
- **Linux x86_64**: `x86_64-unknown-linux-gnu`
- **Linux ARM64**: `aarch64-unknown-linux-gnu` (using ARM64 runners)
- **Windows x86_64**: `x86_64-pc-windows-msvc`
- **macOS x86_64**: `x86_64-apple-darwin`
- **macOS ARM64**: `aarch64-apple-darwin` (using ARM64 runners)

### Cross-Compiled Builds (Build Validation Only)
- **Linux musl**: `x86_64-unknown-linux-musl`

## Platform-Specific Requirements

### Linux (Ubuntu/Debian-based)

#### Native Build (x86_64)
- **Rust**: Standard Rust toolchain installation
- **Dependencies**: Standard system libraries
- **Testing**: Full test suite execution

#### Native Build (ARM64)
- **Runner**: Ubuntu 22.04 ARM64
- **Rust**: Standard Rust toolchain for ARM64
- **Dependencies**: Standard system libraries
- **Testing**: Full test suite execution

#### Cross-Compilation (musl)
- **Cross-compilation tool**: `cross` (installed via `cargo install cross`)
- **System dependencies**:
  ```bash
  sudo apt-get install gcc-x86-64-linux-musl
  ```
- **Notes**: Produces statically linked binaries suitable for minimal environments

### Windows

#### Native Build (x86_64)
- **Rust**: MSVC toolchain (`x86_64-pc-windows-msvc`)
- **Dependencies**: Windows SDK (automatically handled by rustup)
- **Testing**: Full test suite execution
- **Binary naming**: Executable has `.exe` extension

### macOS

#### Native Build (x86_64)
- **Rust**: Standard Rust toolchain
- **Dependencies**: Xcode command line tools
- **Testing**: Full test suite execution

#### Native Build (ARM64)
- **Runner**: macOS 14 (Apple Silicon)
- **Rust**: Standard Rust toolchain
- **Dependencies**: Xcode command line tools
- **Testing**: Full test suite execution
- **Notes**: Uses native ARM64 runners for optimal performance

## Build Configuration

### Cargo Features
All builds use the default feature set unless otherwise specified:
- `git`, `security`, `logging` (default features)

### Release Profile
All builds use the release profile with:
- Optimization level 3
- LTO enabled
- Codegen units: 1
- Panic: abort
- Strip: enabled

### Development Profile
For local development:
- Optimization level 0
- Debug assertions enabled
- Incremental compilation enabled

## CI/CD Pipeline

### GitHub Actions Matrix
The cross-platform CI uses the following matrix:

```yaml
matrix:
  include:
    - os: ubuntu-latest
      target: x86_64-unknown-linux-gnu
      cross: false
    - os: ubuntu-22.04-arm64
      target: aarch64-unknown-linux-gnu
      cross: false
    - os: ubuntu-latest
      target: x86_64-unknown-linux-musl
      cross: true
    - os: windows-latest
      target: x86_64-pc-windows-msvc
      cross: false
    - os: macos-13
      target: x86_64-apple-darwin
      cross: false
    - os: macos-14
      target: aarch64-apple-darwin
      cross: false
```

### Build Steps
1. **Setup Rust**: Install target-specific toolchain
2. **Install Tools**: Cross-compilation tools for Linux targets
3. **Cache Dependencies**: Cargo registry and target directories
4. **Build**: Release build for specified target
5. **Test**: Run tests on native platforms only
6. **Validate**: Check binary functionality and artifacts

## Known Limitations

### Cross-Compiled Targets
- **Testing**: Cannot run full test suite on cross-compiled binaries
- **Functionality**: Basic binary validation only (--version, --help)
- **Performance**: No performance benchmarking on cross-compiled targets

### Platform-Specific Behaviors
- **Windows**: Executable extension (.exe) handling
- **macOS ARM64**: Requires Apple Silicon or Rosetta 2 compatibility
- **Linux musl**: Statically linked, may have larger binary size

## Troubleshooting

### Common Issues

#### Cross-compilation fails
- Ensure `cross` tool is installed: `cargo install cross`
- Check system dependencies are installed
- Verify target architecture support

#### Windows builds fail
- Ensure MSVC toolchain is selected: `rustup target add x86_64-pc-windows-msvc`
- Install Windows SDK if needed

#### macOS ARM64 builds fail
- Ensure Xcode command line tools are installed
- For Intel Macs, ensure Rosetta 2 is available

### Performance Considerations
- **Native builds**: Full optimization and testing
- **Cross-compiled builds**: Build validation only
- **CI timeouts**: 45-minute timeout for cross-platform builds

## Distribution

### Binary Naming Convention
- Linux: `do-codeguardian`
- Windows: `do-codeguardian.exe`
- macOS: `do-codeguardian`

### Archive Naming
- `{target}/do-codeguardian*` (matches all platform variants)

### Retention
- CI artifacts: 30 days
- Release artifacts: 90 days

## Future Enhancements

### Planned Improvements
- **Extended testing**: Run subset of tests on cross-compiled binaries
- **Performance benchmarking**: Cross-platform performance comparisons
- **Binary optimization**: Platform-specific optimization flags
- **Additional targets**: FreeBSD, NetBSD, illumos, Android, iOS

### Additional Targets
- **FreeBSD**: `x86_64-unknown-freebsd`
- **NetBSD**: `x86_64-unknown-netbsd`
- ** illumos**: `x86_64-unknown-illumos`
- **Android**: `aarch64-linux-android`
- **iOS**: `aarch64-apple-ios`

## Maintenance

### Updating Toolchains
- Monitor Rust release channels
- Update GitHub Actions versions regularly
- Test new targets before adding to CI matrix

### Dependency Management
- Keep cross-compilation tools updated
- Monitor system dependency changes
- Update platform-specific documentation