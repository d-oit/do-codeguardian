# CodeGuardian Agent Guidelines

## Build & Test Commands
- **Build**: `cargo build` (dev) / `cargo build --release` (prod)
- **Test**: `cargo test` (all) / `cargo test <test_name>` (single test)
- **Lint**: `cargo clippy -- -D warnings`
- **Format**: `cargo fmt` (100 char width, 4 spaces, reorder imports)
- **Bench**: `cargo bench`

## Code Style (Rust 2021)
- **Naming**: snake_case (functions/vars), PascalCase (types), SCREAMING_SNAKE_CASE (consts)
- **Error Handling**: `anyhow::Result<T>` (app), `thiserror::Error` (libs)
- **Security-First**: Validate inputs, safe defaults, prevent resource exhaustion
- **Memory Safety**: Leverage ownership, avoid unsafe unless necessary
- **KISS**: Keep designs simple and avoid unnecessary complexity

### SOLID Principles
- **Single Responsibility**: Each module, struct, or function should have one primary responsibility. In CodeGuardian, this ensures analyzers focus on specific security checks without overlapping concerns.
- **Open-Closed**: Code should be open for extension but closed for modification. Use Rust traits to extend functionality, such as adding new analyzers without altering existing code.
- **Liskov Substitution**: Subtypes must be substitutable for their base types. In Rust, trait implementations should maintain behavioral consistency, allowing interchangeable components in the analysis pipeline.
- **Interface Segregation**: Clients should not depend on interfaces they do not use. Define small, focused traits in CodeGuardian to avoid forcing implementations to provide unnecessary methods.
- **Dependency Inversion**: Depend on abstractions, not concretions. Use traits for dependencies in CodeGuardian, enabling flexible injection of components like different output formats or caching strategies.

## Key Dependencies
- **Async**: Tokio 1.40, Clap 4.4 (CLI)
- **Serialization**: Serde, TOML
- **Security**: Regex, Blake3 hashing
- **ML**: FANN neural networks (optional)
- **Git**: git2 0.20 (optional)

## Testing Patterns
- **Unit Tests**: `cargo test` in module files
- **Integration**: `cargo test --test integration_tests`
- **E2E**: `cargo test --test e2e_*` (cli, workflow, performance)
- **Single Test**: `cargo test test_function_name`

## Security Considerations
- Input validation, path canonicalization, resource limits (10MB files)
- Audit trails, secure defaults, ML data protection
- Memory bounds, timeout handling, no unsafe code

**Remember**: Think step-by-step, analyze first, validate changes, no regressions, no false positive results, File size: ≤ 600 ine of code
