# Cross-Platform Feasibility Verification Report (macOS & Windows)

- **Tasks:** T006 (Windows GPUI), T007 (macOS GPUI), T009 (C17 & C++20 toolchains), T010 & T011 (Debugger & Reverse-step independence)
- **Status:** **PARTIALLY VERIFIED.** Toolchain (T009), process cancellation (T012) and Tree-sitter (T013) are LIVE on all three targets. GPUI on macOS/Windows is COMPILE-only (`cargo check`), and the debugger rows are PARTIAL (macOS) and FORMAT-ONLY (Windows). See `docs/feasibility/t015-cross-platform-feasibility-report.md` for the corrected evidence levels and the open gaps.
- **Date:** 2026-11-24
- **Branch:** `ci/platform-matrix-verification` / `dev`
- **CI Run Reference:** [GitHub Actions Run 45292424668](https://github.com/SoufianoDev/aveloop/actions/runs/45292424668)

## 1. Summary Matrix

| Task | Capability | Linux x86_64 (GCC 14.2 / GDB 16.2) | macOS arm64 (Apple Clang / LLDB) | Windows x86_64 (MSVC 19.44 / PDB) | Overall Status |
|---|---|---|---|---|---|
| **T005–T007** | GPUI Application Build & Launch | **PASS** (Local Wayland/X11, launched) | **COMPILE ONLY** (`cargo check`, Run 45292424668) | **COMPILE ONLY** (`cargo check`, Run 45292424668) | **LINUX VERIFIED** |
| **T008** | GPUI Text, Input, Clipboard, Resize | **PASS** (Linux interaction tests) | **NOT EXECUTED** — shared-core assumption only | **NOT EXECUTED** — shared-core assumption only | **LINUX VERIFIED** |
| **T009** | C17 & C++20 Debug Compilation & Run | **PASS** (`-std=c17`, `-std=c++20`) | **PASS** (Clang `-std=c17`, `-std=c++20`) | **PASS** (MSVC `/std:c17`, `/std:c++20`) | **VERIFIED** |
| **T010** | Debugger Breakpoints, Stepping, Symbols | **PASS** (GDB 16.2 DAP & MI) | **PARTIAL** (`lldb --batch`: breakpoint, run, backtrace; no stepping/variables/pointers) | **FORMAT ONLY** (`dumpbin` CodeView/PDB header; no debugger launched) | **LINUX VERIFIED** |
| **T011** | Protocol Negotiation & Reverse-Step | **PASS** (`supportsStepBack: false`) | **PARTIAL** (`lldb-dap` `initialize` response only) | **NOT VERIFIED** (no negotiation) | **LINUX VERIFIED** |

## 2. Platform Evidence

### 2.1 macOS arm64 (Runner: `macos-latest`, Job `138104278537`)
- **System:** Darwin Kernel arm64 (Apple Silicon)
- **Compilers:** Apple Clang / Clang++ (Xcode toolchain)
- **T009 Builds:**
  - C17: `clang -std=c17 -g -O0 -pedantic-errors` compiled and output `macOS C17 ok: 201710`
  - C++20: `clang++ -std=c++20 -g -O0 -pedantic-errors` compiled concepts and output `macOS C++20 ok: 42`
- **T010 & T011 Debugger:**
  - LLDB batch execution verified breakpoint insertion, process launch, and backtrace on arm64 Mach-O binaries.
  - `supportsStepBack` verified as `false`; reverse navigation is handled purely via AveLoop `ExecutionTrace`.
- **T007 GPUI Build:**
  - `cargo check --workspace --tests` succeeded in 3m 7s targeting macOS Metal backend.

### 2.2 Windows x86_64 (Runner: `windows-2022`, Job `138104278195`)
- **System:** Microsoft Windows Server 2022 Datacenter x86_64
- **Compilers:** Microsoft Visual C/C++ Optimizing Compiler 19.44 (`cl.exe`), MSVC `link.exe`
- **T009 Builds:**
  - C17: `cl /std:c17 /Zi /Od /W4 c17_win.c /Fe:c17_win.exe` compiled and output `Windows C17 ok`
  - C++20: `cl /std:c++20 /Zi /Od /W4 /EHsc cxx20_win.cpp /Fe:cxx20_win.exe` compiled concepts and output `Windows C++20 ok: 42`
- **T010 & T011 Debugger:**
  - `dumpbin /HEADERS` confirmed valid CodeView (CV) debug directory and PDB record format.
- **T006 GPUI Build:**
  - `cargo check --workspace --tests` succeeded in 3m 50s targeting Windows Win32 backend.
