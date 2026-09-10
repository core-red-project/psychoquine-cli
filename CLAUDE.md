# CLAUDE.md — Architecture & Contributor Guide
Owner: Sxnnyside Project
Realm: Core Red Project
Repository: https://github.com/core-red-project/psychoquine-cli

---

## 1. Project Overview & Topology

- **Project:** `psychoquine`
- **Purpose:** Universal resource-agnostic Quine generator, verifier, and educational metaprogramming toolkit based on Kleene's Second Recursion Theorem and the canonical Wikipedia Quines catalog.
- **Repository Topology:** **Monolithic** (Single deployable Rust binary crate with an integrated library target at root).
- **Stack Profile:** **Rust** (Cargo, Rust 2021 edition, stable pinned via `rust-toolchain.toml`).

---

## 2. Standard Command Surface (Task Runner)

This repository strictly exposes its command surface via `just` (`Justfile`). Do not invoke raw ad-hoc tooling commands directly if a `just` recipe exists.

| Command | Description | Implementation |
| :--- | :--- | :--- |
| `just install` | Bootstrap toolchains & components | `rustup component add rustfmt clippy && cargo check` |
| `just dev [ARGS]` | Run local binary with args | `cargo run -- <ARGS>` |
| `just build` | Compile optimized release binary | `cargo build --release` |
| `just test` | Run complete integration & unit test suite | `cargo test` |
| `just typecheck` | Strict compiler type and syntax validation | `cargo check --all-targets` |
| `just lint` | Static analysis with strict pedantic warnings | `cargo clippy --all-targets -- -D warnings` |
| `just format` | Apply deterministic code formatting | `cargo fmt` |
| `just format-check`| Verify formatting without modifying files | `cargo fmt --check` |
| `just deny` | Verify security advisories and licenses | `cargo deny check` |
| `just check` | **Full Quality Gate** (format, lint, typecheck, test, deny) | Runs entire pipeline (required for CI and PRs) |
| `just clean` | Remove build artifacts and caches | `cargo clean` |

---

## 3. Architecture & Code Organization

PsychoQuine enforces strict **Clean Architecture (Hexagonal Architecture)**. No layer may import from a layer outside its declared boundary:

```
src/
├── domain/                      # Core Domain Layer (Zero external I/O or CLI dependencies)
│   ├── quine.rs                 # Quine model, SHA-256 calculation, and metrics
│   ├── language.rs              # 18 Wikipedia languages, metadata, and Kleene principles
│   ├── payload.rs               # Payload models (None, Text, Banner) and Sxnnyside ASCII art
│   ├── report.rs                # VerificationReport model
│   └── error.rs                 # DomainError typed hierarchy (via thiserror)
│
├── engines/                     # Domain Drivers (18 Wikipedia Quine Implementations)
│   ├── mod.rs                   # QuineEngine trait definition
│   ├── python.rs, rust.rs, c.rs, javascript.rs, bash.rs, perl.rs, php.rs, ruby.rs
│   └── go.rs, csharp.rs, lua.rs, ocaml.rs, pascal.rs, scheme.rs, lisp.rs, batch.rs, brainfuck.rs, hq9plus.rs
│
├── application/                 # Application Use Cases
│   ├── mod.rs
│   ├── registry.rs              # EngineRegistry dynamically registering all 18 engines
│   ├── generate_usecase.rs      # GenerateQuineUseCase (dry-run, live execution, verification)
│   ├── verify_usecase.rs        # VerifyQuineUseCase (isolated execution, SHA-256 comparison)
│   ├── explain_usecase.rs       # ExplainQuineUseCase (Kleene theorem and syntax breakdown)
│   └── doctor_usecase.rs        # DoctorUseCase (PATH auditing for runtimes)
│
├── infrastructure/              # Secondary Adapters (OS, Sandbox, Filesystem, Terminal)
│   ├── mod.rs
│   ├── detector.rs              # Host PATH scanner for compilers & interpreters
│   ├── sandbox.rs               # Isolated child process sandbox (tempfile RAII)
│   └── terminal.rs              # ANSI colors, TTY checks, and NO_COLOR compliance
│
└── presentation/                # Primary Adapters (CLI & Terminal Views)
    ├── args.rs                  # Clap v4 derive models with global flags
    ├── commands/                # Subcommand handlers (generate, verify, explain, doctor, list, completions)
    └── view.rs                  # Terminal tables, ASCII banners, JSON serialization
```

---

## 4. Coding Conventions & Invariants

1. **Quine Mathematical Identity:** Every quine generated must satisfy $\text{execute}(Q) \equiv Q$ byte-for-byte with 100% exact SHA-256 identity.
2. **Stream Separation:**
   - Generated quine source code or structured JSON MUST be output to `stdout`.
   - All banners, metrics, compiler logs, and verification diagnostics MUST be routed strictly to `stderr`.
3. **No Unchecked Unwraps in Shipped Code:**
   - Production execution flows must return `Result<T, DomainError>`.
   - Use `?` operator and pattern matching.
4. **Clippy & Formatting:**
   - All code must pass `cargo clippy --all-targets -- -D warnings`.
   - All code must pass `cargo fmt --check`.
5. **Deterministic Dependencies:**
   - `Cargo.lock` must be committed.
   - All third-party crates must be licensed permissively (MIT, Apache-2.0, MPL-2.0, Unicode-3.0) and pass `cargo deny check`.

---

## 5. Review Criteria for AI and Human Contributors

Before submitting any Pull Request:
1. Run `just check` locally and ensure it exits with code 0.
2. If adding or modifying a language engine, ensure an integration test is added to `tests/quine_execution.rs` verifying self-replication with and without payloads.
3. Keep commit messages following the **Conventional Commits** specification (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`).
