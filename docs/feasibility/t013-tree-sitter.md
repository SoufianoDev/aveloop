# T013 Tree-sitter C/C++ Grammar Integration

- **Status:** Linux verified; Windows and macOS CI verification pending.
- **Date:** 2026-11-24
- **Branch:** `feat/gpui-kit-fast`

## Integrated Crates

| Crate | Pinned Version | License |
|---|---:|---|
| `tree-sitter` | 0.27.2 | MIT |
| `tree-sitter-c` | 0.24.2 | MIT |
| `tree-sitter-cpp` | 0.23.4 | MIT |

All three crate manifests declare MIT licenses. MIT is compatible with AveLoop's Apache-2.0 package license. No vendored or proprietary grammar requirement was introduced.

## Linux Verification

The `t013_verify` probe was built and run with Rust 1.96.0 on Linux x86_64.

- C17 parse round-trip: PASS
- C++20 parse round-trip: PASS
- C function and if-statement node kinds: PASS
- C++ template and function-definition node kinds: PASS

## Cross-Platform CI

The `.github/workflows/t013-tree-sitter.yml` workflow builds and runs `t013_verify` on:

- Linux x86_64
- Windows x86_64
- macOS arm64
