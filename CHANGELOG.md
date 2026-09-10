# Changelog

All notable changes to **PsychoQuine** are documented here.

This project follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

---

## [0.2.0] — 2026-09-10

### Added

- Complete Clean Architecture modernization in pure standalone Rust (`psychoquine`).
- 18 canonical Quine engines based on the Wikipedia catalog (Python, JavaScript, ANSI C, C#, Rust, Bash, Perl, PHP, Ruby, Go, Lua, OCaml, Pascal, Scheme, Common Lisp, DOS Batch, Brainfuck, HQ9+).
- Sandboxed live execution verification asserting 100% byte-for-byte self-replication.
- Hardened execution sandbox with concurrent pipe draining and 30-second process timeouts.
- Direct external file payload embedding (`--payload-file <PATH>`) and stdin support (`--stdin`).
- Payload size bounds checking (512 KB limit) protecting against memory exhaustion.
- Shell completion generator for Bash, Zsh, Fish, PowerShell, and Elvish (`psychoquine completions`).
- Kleene recursion theorem educational explainer (`psychoquine explain`).
- Host system PATH runtime inspection (`psychoquine doctor`).
- Safe execution simulation mode (`--dry-run`).
- Standard task runner abstraction layer (`Justfile`).
- Documentation following the Sxnnyside Project OSS Bundle for Core Red Project.

### Removed

- Legacy unmaintained web UI (Fresh/Deno), desktop wrappers (Tauri 1.x), and obsolete shell scripts.

---

## [0.1.0] — 2026-02-03

### Added

- Initial prototype release.

---

[Unreleased]: https://github.com/core-red-project/psychoquine-cli/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/core-red-project/psychoquine-cli/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/core-red-project/psychoquine-cli/releases/tag/v0.1.0
