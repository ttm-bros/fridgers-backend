# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Fridgers Backend is a Rust web service for managing smart fridges and their contents. The project uses Actix-web for the REST API and follows Clean Architecture principles with distinct layers.

## Build and Development Commands

### Building
```bash
# Build all workspace members
cargo build

# Build in release mode
cargo build --release

# Build specific binary
cargo build --bin rest-server
```

### Running
```bash
# Run the REST server (runs on http://127.0.0.1:8080)
cargo run --bin rest-server

# The server requires environment variables - copy .env.template to .env first
cp .env.template .env
```

### Testing
```bash
# Run all tests
cargo test

# Run tests for a specific package
cargo test -p fridgers-backend-domain
cargo test -p fridgers-backend-use-case
cargo test -p fridgers-backend-config

# Run a specific test
cargo test test_name
```

### Code Quality
```bash
# Check code without building
cargo check

# Run clippy linter
cargo clippy

# Format code
cargo fmt
```

## Architecture

The codebase follows **Clean Architecture** with a workspace structure separating concerns into distinct layers:

### Layer Structure (Dependency Direction: Apps → Interface → Use-Case → Domain)

1. **Domain Layer** (`src/domain/`)
   - Core business entities and domain logic
   - No external dependencies
   - Currently minimal/placeholder

2. **Use-Case Layer** (`src/use-case/`)
   - Application business rules and orchestration
   - Defines `Error` enum mapping HTTP status codes (400, 401, 403, 404, 409, 412, 500)
   - Independent of frameworks and external systems

3. **Interface Layer** (`src/interface/`)
   - Adapters for external communication
   - `rest-controller/`: REST API controllers (currently minimal)
   - `rdb-gateway/`: Database gateway/repository implementations

4. **Infrastructure Layer** (`src/infrastructure/`)
   - `config/`: Environment-based configuration management
     - Uses `dotenvy` for .env file loading
     - Uses `envy` with prefixed environment variables (`LOG_`, `SERVER_`, `DB_`)
     - Provides `Config::from_env()` for application configuration
     - Config includes: LogConfig, ServerConfig, DbConfig

5. **Apps Layer** (`src/apps/`)
   - `rest-server/`: Main application binary
     - Actix-web HTTP server
     - Middleware: request tracing spans, access logging
     - Currently has placeholder greeting endpoints

### Error Handling Pattern

The project uses a layered error handling approach:
- Each layer defines its own `Error` enum and `Result<T>` type alias
- Infrastructure errors convert to use-case errors via `From` trait implementations
- Use-case `Error` variants map to HTTP status codes (see `src/use-case/src/error.rs`)

### Configuration

Configuration is loaded from environment variables with prefixes:
- `LOG_LEVEL`: Logging level (e.g., "debug", "info")
- `SERVER_URL`, `SERVER_PORT`: Server configuration
- `DB_URL`, `DB_PORT`: Database configuration

See `.env.template` for all available configuration options.

### API Documentation

OpenAPI specification is maintained in `docs/fridgers.openapi.yaml` covering:
- Liveness probe endpoint
- User operations
- Group operations
- Fridge operations
- Item operations

## Rust Toolchain

This project uses Rust 1.92.0 as specified in `rust-toolchain.toml`. The toolchain will be automatically installed when using rustup.
