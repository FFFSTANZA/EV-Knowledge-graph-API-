# Contributing to EV Knowledge Graph API

**Version:** 0.1.0
**Last Updated:** 2024-11-18

Thank you for your interest in contributing to the EV Knowledge Graph API! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Pull Request Process](#pull-request-process)
- [Documentation](#documentation)
- [Issue Reporting](#issue-reporting)
- [Community](#community)

---

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inspiring community for all. Please be respectful and constructive in your interactions.

### Expected Behavior

- Use welcoming and inclusive language
- Be respectful of differing viewpoints
- Accept constructive criticism gracefully
- Focus on what is best for the community
- Show empathy towards other community members

### Unacceptable Behavior

- Harassment, trolling, or discriminatory language
- Publishing others' private information
- Personal or political attacks
- Other conduct that could be considered inappropriate

---

## Getting Started

### Prerequisites

Before contributing, ensure you have:

- **Rust** 1.75 or later (`rustup` recommended)
- **Docker** and **Docker Compose**
- **Git** for version control
- **Neo4j** 5.x (via Docker)
- **Redis** 7.x (via Docker)

### First-Time Setup

1. **Fork the repository**

   Click the "Fork" button on GitHub to create your own fork.

2. **Clone your fork**

   ```bash
   git clone https://github.com/YOUR_USERNAME/EV-Knowledge-graph-API-.git
   cd EV-Knowledge-graph-API-
   ```

3. **Add upstream remote**

   ```bash
   git remote add upstream https://github.com/FFFSTANZA/EV-Knowledge-graph-API-.git
   ```

4. **Start development environment**

   ```bash
   docker-compose up -d neo4j redis
   cargo build
   cargo test
   ```

---

## Development Setup

### Project Structure

```
EV-Knowledge-graph-API/
├── crates/
│   ├── ev-api/          # REST API server
│   ├── ev-core/         # Domain models & business logic
│   ├── ev-graph/        # Neo4j graph operations
│   ├── ev-cache/        # Redis caching layer
│   └── ev-data/         # Data seeding & migrations
├── docs/                # Documentation
├── docker/              # Docker configurations
├── scripts/             # Utility scripts
└── tests/               # Integration tests
```

### Development Workflow

1. **Create a feature branch**

   ```bash
   git checkout -b feature/your-feature-name
   ```

   Branch naming conventions:
   - `feature/` - New features
   - `fix/` - Bug fixes
   - `docs/` - Documentation changes
   - `refactor/` - Code refactoring
   - `test/` - Test additions/fixes

2. **Make your changes**

   Follow our [coding standards](#coding-standards) and add tests for new functionality.

3. **Run tests**

   ```bash
   cargo test --workspace
   cargo clippy --all-targets --all-features
   cargo fmt --all -- --check
   ```

4. **Commit your changes**

   ```bash
   git add .
   git commit -m "feat: add vehicle filtering by range"
   ```

   See [commit message guidelines](#commit-messages).

5. **Keep your fork updated**

   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

6. **Push to your fork**

   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request**

   Go to GitHub and create a pull request from your fork to the main repository.

---

## How to Contribute

### Types of Contributions

#### 1. Bug Reports

Found a bug? Help us by:
- Checking if it's already reported in [Issues](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/issues)
- Creating a new issue with detailed information
- Including steps to reproduce, expected vs actual behavior
- Providing logs, error messages, and system information

#### 2. Feature Requests

Have an idea? We'd love to hear it!
- Check [existing feature requests](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)
- Create a new issue describing the feature
- Explain the use case and benefits
- Discuss implementation approaches if possible

#### 3. Code Contributions

Contributing code? Great!
- Pick an issue labeled `good first issue` or `help wanted`
- Comment on the issue to let others know you're working on it
- Follow our development workflow
- Submit a pull request

#### 4. Documentation

Documentation improvements are always welcome:
- Fix typos or improve clarity
- Add examples or use cases
- Translate documentation
- Improve API documentation

#### 5. Data Contributions

Help expand the Indian EV dataset:
- Add new vehicle models
- Update charging station information
- Add regional policies
- Correct existing data

---

## Coding Standards

### Rust Style Guide

We follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/) and enforce it with `rustfmt`.

#### General Rules

1. **Use `rustfmt` for formatting**

   ```bash
   cargo fmt --all
   ```

2. **Use `clippy` for linting**

   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```

3. **Follow Rust naming conventions**

   - `snake_case` for functions, variables, modules
   - `PascalCase` for types, traits, enums
   - `SCREAMING_SNAKE_CASE` for constants
   - Descriptive names over abbreviations

#### Code Examples

**Good:**
```rust
pub struct VehicleRepository {
    graph_db: Arc<GraphDb>,
}

impl VehicleRepository {
    pub async fn find_by_id(&self, vehicle_id: Uuid) -> Result<Vehicle, Error> {
        let query = "MATCH (v:Vehicle {id: $id}) RETURN v";
        // ... implementation
    }

    pub async fn search(&self, search_term: &str) -> Result<Vec<Vehicle>, Error> {
        // ... implementation
    }
}
```

**Bad:**
```rust
pub struct VehicleRepo {  // Too abbreviated
    db: Arc<GraphDb>,     // Unclear
}

impl VehicleRepo {
    pub async fn get(&self, id: Uuid) -> Result<Vehicle, Error> {  // Too generic
        // ... implementation
    }
}
```

### Error Handling

1. **Use `Result<T, Error>` for fallible operations**

   ```rust
   pub async fn create_vehicle(&self, vehicle: Vehicle) -> Result<Uuid, Error> {
       // ... implementation
   }
   ```

2. **Use custom error types from `ev-core`**

   ```rust
   use ev_core::Error;

   if vehicle_id.is_nil() {
       return Err(Error::Validation("Invalid vehicle ID".to_string()));
   }
   ```

3. **Propagate errors with `?` operator**

   ```rust
   let vehicle = self.repository.find_by_id(vehicle_id).await?;
   ```

### Documentation

1. **Document all public APIs**

   ```rust
   /// Find a vehicle by its unique identifier.
   ///
   /// # Arguments
   ///
   /// * `vehicle_id` - The unique identifier of the vehicle
   ///
   /// # Returns
   ///
   /// Returns the vehicle if found, or `Error::NotFound` if not found.
   ///
   /// # Examples
   ///
   /// ```
   /// let vehicle = repository.find_by_id(vehicle_id).await?;
   /// ```
   pub async fn find_by_id(&self, vehicle_id: Uuid) -> Result<Vehicle, Error> {
       // ... implementation
   }
   ```

2. **Use meaningful comments**

   ```rust
   // Calculate charging time based on battery capacity and charger power
   let charging_time_hours = battery_capacity_kwh / charger_power_kw;
   ```

### Async/Await

1. **Use async/await for I/O operations**

   ```rust
   pub async fn fetch_vehicles(&self) -> Result<Vec<Vehicle>, Error> {
       let query = "MATCH (v:Vehicle) RETURN v LIMIT 100";
       let result = self.graph_db.execute(query).await?;
       Ok(result)
   }
   ```

2. **Don't block the async runtime**

   ```rust
   // Good
   let result = tokio::task::spawn_blocking(|| {
       expensive_cpu_operation()
   }).await?;

   // Bad - blocks runtime
   let result = expensive_cpu_operation();
   ```

---

## Testing Guidelines

### Test Organization

```
crates/ev-api/
├── src/
│   └── handlers/
│       ├── vehicles.rs
│       └── mod.rs
└── tests/
    ├── integration/
    │   └── vehicles_test.rs
    └── common/
        └── mod.rs
```

### Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vehicle_validation() {
        let vehicle = Vehicle {
            id: Uuid::new_v4(),
            name: "Test Vehicle".to_string(),
            range_km: 300.0,
            // ...
        };

        assert!(vehicle.validate().is_ok());
    }

    #[tokio::test]
    async fn test_find_vehicle() {
        let repository = create_test_repository().await;
        let vehicle_id = Uuid::new_v4();

        let result = repository.find_by_id(vehicle_id).await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests

Create integration tests in the `tests/` directory:

```rust
// tests/integration/vehicles_test.rs
use ev_api::*;

#[tokio::test]
async fn test_create_and_retrieve_vehicle() {
    // Setup
    let app = create_test_app().await;

    // Create vehicle
    let response = app
        .post("/api/v1/vehicles")
        .json(&serde_json::json!({
            "name": "Test Vehicle",
            "range_km": 300.0
        }))
        .send()
        .await;

    assert_eq!(response.status(), 201);

    // Retrieve vehicle
    let vehicle_id = response.json::<ApiResponse<Vehicle>>()
        .await
        .data
        .unwrap()
        .id;

    let get_response = app
        .get(&format!("/api/v1/vehicles/{}", vehicle_id))
        .send()
        .await;

    assert_eq!(get_response.status(), 200);
}
```

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p ev-api

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_vehicle_validation

# Run integration tests only
cargo test --test integration_tests
```

### Test Coverage

We aim for:
- **Unit tests:** 80%+ code coverage
- **Integration tests:** All API endpoints
- **Critical paths:** 100% coverage

Check coverage:
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --workspace --out Html
```

---

## Pull Request Process

### Before Submitting

1. **Ensure all tests pass**

   ```bash
   cargo test --workspace
   ```

2. **Run linters**

   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt --all -- --check
   ```

3. **Update documentation**

   - Update relevant documentation files
   - Add docstring comments
   - Update CHANGELOG.md if applicable

4. **Rebase on latest main**

   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

### PR Template

When creating a pull request, use this template:

```markdown
## Description

Brief description of what this PR does.

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Related Issue

Fixes #(issue number)

## Changes Made

- List key changes
- Include any breaking changes
- Mention new dependencies

## Testing

Describe the tests you ran and how to reproduce them:

```bash
cargo test --package ev-api --test vehicle_tests
```

## Checklist

- [ ] My code follows the style guidelines of this project
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] Any dependent changes have been merged and published

## Screenshots (if applicable)

Add screenshots to help explain your changes.
```

### Review Process

1. **Automated checks must pass**
   - CI/CD pipeline
   - Code formatting
   - Linting
   - All tests

2. **Code review**
   - At least one maintainer approval required
   - Address all review comments
   - Resolve all conversations

3. **Merge**
   - Squash commits for clean history
   - Maintainer will merge after approval

---

## Commit Messages

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation only
- `style:` - Code style changes (formatting, missing semicolons, etc.)
- `refactor:` - Code change that neither fixes a bug nor adds a feature
- `perf:` - Performance improvement
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks, dependency updates
- `ci:` - CI/CD changes

### Examples

**Good:**
```
feat(vehicles): add filtering by battery chemistry

Add support for filtering vehicles by battery chemistry type (NMC, LFP, NCA).
Includes new query parameter `battery_chemistry` in vehicles endpoint.

Closes #123
```

```
fix(rate-limit): correct Redis key expiration

Fixed issue where rate limit counters were not expiring correctly,
causing permanent rate limiting for some clients.

Fixes #456
```

```
docs(api): update compatibility endpoint examples

Added Python and JavaScript examples for the compatibility check endpoint.
```

**Bad:**
```
updated stuff
```

```
Fixed a bug
```

```
WIP
```

---

## Documentation

### Documentation Standards

1. **Keep docs up-to-date**
   - Update docs in the same PR as code changes
   - Run spell check before committing

2. **Use clear, concise language**
   - Write for both beginners and experts
   - Include examples where helpful
   - Use diagrams for complex concepts

3. **Document all public APIs**
   - Include purpose, parameters, return values
   - Provide usage examples
   - Document error cases

### Documentation Structure

```
docs/
├── API.md                    # API reference
├── ARCHITECTURE.md           # System architecture
├── GETTING_STARTED.md        # Quick start guide
├── DEPLOYMENT.md             # Deployment guide
├── TROUBLESHOOTING.md        # Common issues
├── EXAMPLES.md               # Code examples
└── CONTRIBUTING.md           # This file
```

### Building Documentation

```bash
# Generate Rust docs
cargo doc --workspace --no-deps --open

# Check documentation
cargo doc --workspace --no-deps
```

---

## Issue Reporting

### Bug Reports

When reporting bugs, include:

1. **Description**
   - Clear, concise description of the bug
   - Expected vs actual behavior

2. **Steps to Reproduce**
   ```
   1. Start the API with Docker Compose
   2. Send GET request to /api/v1/vehicles
   3. Observe error response
   ```

3. **Environment**
   - OS: Ubuntu 22.04
   - Rust version: 1.75.0
   - Docker version: 24.0.5
   - Neo4j version: 5.15
   - Redis version: 7.2

4. **Logs and Error Messages**
   ```
   Include relevant logs, stack traces, or error messages
   ```

5. **Additional Context**
   - Screenshots if applicable
   - Related issues or PRs

### Feature Requests

When requesting features, include:

1. **Use Case**
   - Why is this feature needed?
   - What problem does it solve?

2. **Proposed Solution**
   - How should it work?
   - API design or interface

3. **Alternatives**
   - Other solutions you've considered

4. **Additional Context**
   - Examples from other projects
   - Mockups or diagrams

---

## Community

### Getting Help

- **GitHub Discussions:** [Ask questions](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/discussions)
- **GitHub Issues:** [Report bugs or request features](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/issues)
- **Documentation:** [Read the docs](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/tree/main/docs)

### Communication Channels

- Use GitHub Issues for bugs and feature requests
- Use GitHub Discussions for questions and general discussion
- Be respectful and constructive
- Help others when you can

### Recognition

Contributors are recognized in:
- `CONTRIBUTORS.md` file
- Release notes
- GitHub contributors page

---

## Development Tips

### Useful Commands

```bash
# Watch and auto-rebuild
cargo watch -x check -x test

# Run specific test with logging
RUST_LOG=debug cargo test test_name -- --nocapture

# Benchmark
cargo bench

# Check dependency tree
cargo tree

# Update dependencies
cargo update

# Audit dependencies for security
cargo audit
```

### IDE Setup

**VS Code:**
Install these extensions:
- rust-analyzer
- CodeLLDB (for debugging)
- Better TOML
- Error Lens

**RustRover/IntelliJ:**
- Install Rust plugin
- Enable Clippy inspections

### Debugging

```rust
// Add debug prints
dbg!(&vehicle);

// Or use tracing
tracing::debug!("Vehicle: {:?}", vehicle);
```

---

## Release Process

(For maintainers)

1. **Version Bump**
   - Update version in `Cargo.toml`
   - Update `CHANGELOG.md`

2. **Testing**
   ```bash
   cargo test --workspace --release
   cargo clippy --all-targets
   ```

3. **Build**
   ```bash
   cargo build --release
   ```

4. **Tag and Release**
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0"
   git push origin v0.2.0
   ```

5. **Publish Release Notes**
   - Create GitHub release
   - Include changelog
   - Attach binaries

---

## License

By contributing to this project, you agree that your contributions will be licensed under the same license as the project.

---

## Questions?

If you have questions about contributing, feel free to:
- Open a [GitHub Discussion](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/discussions)
- Comment on an existing issue
- Reach out to the maintainers

---

**Thank you for contributing to the EV Knowledge Graph API!** 🎉

Your contributions help make electric vehicle adoption easier for everyone in India.

---

**Last Updated:** 2024-11-18
**Version:** 0.1.0
