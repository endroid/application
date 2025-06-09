# AGENT.md - gRPC Learning Rust Project

## Build/Test Commands
- **Build**: `cargo build` or `cargo check` (faster type checking)
- **Run server**: `cargo run --bin grpc-server`
- **Run demo client**: `cargo run --bin demo-client`
- **Test**: `cargo test` (run all tests), `cargo test <test_name>` (single test)
- **Format**: `cargo fmt`
- **Lint**: `cargo clippy`

## Architecture
- **Type**: Rust gRPC microservice with client/server binaries in `src/bin/`
- **Proto files**: `proto/` directory contains `.proto` definitions, `src/proto/` contains generated bindings
- **Services**: `src/services/` folder contains service implementations (sudoku_solver_service.rs, factorial_calculator_service.rs)
- **Sudoku solver**: `src/sudoku/` contains complete Sudoku solving library with backtracking algorithm
- **Library**: `src/lib.rs` exposes public API, modules organized by responsibility
- **Generated code**: `build.rs` uses `tonic-build` to generate Rust bindings from `.proto` files
- **Server**: Runs on `0.0.0.0:50051` with reflection support for debugging

## Code Style
- **Imports**: Use `crate::` for internal modules, organized by external/internal/generated
- **Modules**: Generated proto code via `tonic::include_proto!()` pattern  
- **Services**: Implement with `#[tonic::async_trait]` and `#[derive(Debug, Default)]`
- **Error handling**: Use `tonic::Status` for gRPC errors, `Box<dyn std::error::Error>` for general errors
- **Naming**: snake_case for fields/functions, PascalCase for types/services, kebab-case for binaries
- **Async**: Use `tokio::main` and async/await patterns throughout
