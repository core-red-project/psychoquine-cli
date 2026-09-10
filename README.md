# PsychoQuine

![Banner](PsychoQuine.png)

![Version](https://img.shields.io/badge/version-0.2.0-blue)
![License](https://img.shields.io/badge/License-MIT-green)
[![CI](https://github.com/core-red-project/psychoquine-cli/workflows/CI/badge.svg)](https://github.com/core-red-project/psychoquine-cli/actions)

<p align="center">
  <strong>Self-Replicating ✦ 18 Languages ✦ Live Verification</strong><br>
  <em>Universal resource-agnostic Quine generator and verifier.</em>
</p>

<p align="center">
  <a href="#about">About</a> ✦
  <a href="#features">Features</a> ✦
  <a href="#installation">Installation</a> ✦
  <a href="#usage">Usage</a> ✦
  <a href="#architecture">Architecture</a> ✦
  <a href="#contributing">Contributing</a>
</p>

---

## About

**PsychoQuine** is an autonomous CLI and meta-programming engine for generating legitimate, mathematically sound quines across 18 canonical programming languages.

Quines are often treated as brittle one-offs or esoteric curiosities. PsychoQuine treats self-replication as a formal engineering discipline, providing exact reflexive generation, live child-process sandbox verification, payload injection, and runtime discovery.

Each generated program strictly satisfies Kleene's Second Recursion Theorem: it takes zero external input and outputs an exact copy of its own source code ($\text{execute}(Q) \equiv Q$), verified byte-for-byte via SHA-256 hashing.

### Philosophy

> *"A quine that cannot self-verify is not a quine."*

This is a Core Red Project, part of the Sxnnyside Project's experimental branch.

## Features

- **18 Language Engines**: Canonical quines for Python, JavaScript, C, C#, Rust, Bash, Perl, PHP, Ruby, Go, Lua, OCaml, Pascal, Scheme, Common Lisp, DOS Batch, Brainfuck, and HQ9+.
- **Zero-Tolerance Verification**: Sandboxed live execution testing ensuring 100% byte-for-byte identity against system runtimes.
- **Resource & Payload Injection**: Embed arbitrary text, external files, or ASCII banners into quines without breaking self-replication.
- **Kleene Recursion Explainer**: Mathematical breakdown and technical invariants inspector for any target language (`explain`).
- **Host Runtime Doctor**: Automatic PATH discovery identifying available compilers and interpreters on the host system.
- **Safe Simulation**: Dry-run mode (`--dry-run`) projecting byte size, expansion ratios, and SHA-256 hashes without compiling.
- **Shell Autocompletion**: Native completion script generation for Bash, Zsh, Fish, PowerShell, and Elvish.

## Installation

### Prerequisites

- Rust toolchain 1.85+ (`cargo`, `rustc`)

### From Source

```bash
git clone https://github.com/core-red-project/psychoquine-cli.git
cd psychoquine-cli

cargo build --release
```

The compiled binary will be located at `target/release/psychoquine`.

## Usage

```bash
# Generate and print a Python quine to stdout
psychoquine generate python

# Generate a Rust quine with payload and verify execution
psychoquine generate rust "Core Red Project" --verify

# Embed an external file into a C quine and save to disk
psychoquine generate c --payload-file ./data.txt -o quine.c

# Simulate generation without running compilers
psychoquine rust --dry-run

# Audit host system compilers and runtimes
psychoquine doctor

# Explain the Kleene recursion mechanics of a language
psychoquine explain python

# Generate shell completions (e.g. Zsh)
psychoquine completions zsh > ~/.zfunc/_psychoquine
```

## Architecture

```
psychoquine-cli/
├── src/
│   ├── domain/          # Pure entities: Quine, Language, Payload, VerificationReport
│   ├── engines/         # 18 canonical QuineEngine trait drivers
│   ├── application/     # Use cases: generate, verify, explain, doctor
│   ├── infrastructure/  # OS adapters: child sandbox, PATH detector, terminal
│   └── presentation/    # CLI layer: Clap v4 commands, view rendering
├── tests/               # End-to-end sandbox integration tests
└── Cargo.toml           # Standalone Rust crate configuration
```

For developer recipes and local tasks, see the task runner configuration in `Justfile` (`just --list`).

## Contributing

Contributions are accepted. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Before contributing, read the [Code of Conduct](CODE_OF_CONDUCT.md).

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

<p align="center">
  <strong>PsychoQuine</strong> — Core Red Project<br>
  <em>&copy; 2026 Sxnnyside Project</em>
</p>
