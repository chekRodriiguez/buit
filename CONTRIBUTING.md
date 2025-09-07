# Contributing to BUIT

Thank you for your interest in contributing to BUIT! This document provides guidelines for contributing to the project.

## How to Contribute

### Reporting Issues
- Use the GitHub issue tracker to report bugs
- Provide clear reproduction steps
- Include relevant system information (OS, Rust version, etc.)

### Suggesting Features
- Open an issue with the "feature request" label
- Describe the use case and expected behavior
- Consider if the feature fits BUIT's scope (OSINT/reconnaissance)

### Adding New Modules
New OSINT modules are always welcome! Here's how to add one:

1. Create a new file in `src/modules/` (e.g., `my_module.rs`)
2. Implement the required functions:
   ```rust
   use crate::cli::MyModuleArgs;
   use anyhow::Result;
   
   pub async fn run(args: MyModuleArgs) -> Result<()> {
       // Your implementation here
       Ok(())
   }
   ```
3. Add the module to `src/modules/mod.rs`
4. Add CLI arguments in `src/cli.rs`
5. Add the command handler in `src/main.rs`

### Code Style
- Use `cargo fmt` to format code
- Run `cargo clippy` to catch common issues
- Write clear, self-documenting code
- Add error handling with proper messages

### Testing
- Test your changes thoroughly
- Ensure all existing functionality still works
- Add tests for new features when appropriate

### Pull Request Process
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-module`)
3. Make your changes
4. Test your changes
5. Commit with clear messages
6. Push to your fork
7. Create a pull request

## Development Setup

```bash
# Clone the repository
git clone https://github.com/BuuDevOff/BUIT.git
cd BUIT

# Build and test
cargo build
cargo test
cargo run -- --help
```

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and improve
- Remember that this is an educational/security research tool

## Questions?

Feel free to open an issue if you have questions about contributing or need help with development.

---

Thanks for helping make BUIT better for the security community!