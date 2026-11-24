# Cross-Platform Feasibility Verification Report (macOS & Windows)

- **Tasks:** T006 (Windows GPUI), T007 (macOS GPUI), T009 (C17 & C++20 toolchains), T010 & T011 (Debugger & Reverse-step independence), T016–T019 (gate closure)
- **Status:** **VERIFIED with recorded residuals.** Toolchain (T009), process cancellation (T012), Tree-sitter (T013), GPUI launch (T019), and debugger sessions (T017 Windows MinGW-w64/GDB, T018 macOS LLDB) are LIVE on the stated targets. Windows toolchain selection is pair-based and validated in CI (T016). Residuals: MSVC+CDB scripted variable/stack assertions are not captured under redirection on `windows-2022`; GPUI input/clipboard beyond launch is Linux-only; Wayland clipboard read unverified. See `docs/feasibility/t016-t019-gate-closure.md`.
- **Date:** 2026-11-24
- **Branch:** `ci/platform-matrix-verification` / `dev`
- **CI Run Reference:** [GitHub Actions Run 45292424668](https://github.com/SoufianoDev/aveloop/actions/runs/45292424668)

## 1. Summary Matrix

| Task | Capability | Linux x86_64 (GCC 14.2 / GDB 16.2) | macOS arm64 (Apple Clang / LLDB) | Windows x86_64 (MSVC 19.44 / PDB) | Overall Status |
|---|---|---|---|---|---|
| **T005–T007** | GPUI Application Build & Launch | **PASS** (Local Wayland/X11, launched) | **PASS** (window opened, Run 45304616214) | **PASS** (window opened, Run 45304616214) | **VERIFIED** |
| **T008** | GPUI Text, Input, Clipboard, Resize | **PASS** (Linux interaction tests) | **NOT EXECUTED** — shared-core assumption only | **NOT EXECUTED** — shared-core assumption only | **LINUX VERIFIED** |
| **T009** | C17 & C++20 Debug Compilation & Run | **PASS** (`-std=c17`, `-std=c++20`) | **PASS** (Clang `-std=c17`, `-std=c++20`) | **PASS** (MSVC `/std:c17`, `/std:c++20`) | **VERIFIED** |
| **T010** | Debugger Breakpoints, Stepping, Symbols | **PASS** (GDB 16.2 DAP & MI) | **PASS** (LLDB: launch, breakpoint, 4 steps, variables, aggregate, pointer, backtrace, exit, Run 45305339035) | **PASS** (MinGW-w64 + GDB live session, Run 45305339035; MSVC + CDB launch/capture only) | **VERIFIED** |
| **T011** | Protocol Negotiation & Reverse-Step | **PASS** (`supportsStepBack: false`) | **PARTIAL** (`lldb-dap` `initialize` response only) | **NOT VERIFIED** (no negotiation) | **LINUX VERIFIED** |
| **T016** | Windows Compiler-Debugger Pair Selection | n/a | n/a | **PASS** (8/8 checks + real detection, Run 45304616166) | **VERIFIED** |
| **T017–T018** | Live Debugger Session | **PASS** (T010) | **PASS** (Run 45305339035) | **PASS** (MinGW-w64 + GDB, Run 45305339035) | **VERIFIED** |
| **T019** | GPUI Runtime Launch | **PASS** (T005) | **PASS** (Run 45304616214) | **PASS** (Run 45304616214) | **VERIFIED** |

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

### 2.3 Gate closure (T016–T019)

- **T016:** `windows-2022`, run [45304616166](https://github.com/SoufianoDev/aveloop/actions/runs/45304616166): 8/8 selection checks; real host pair `C:\mingw64\bin\gcc.exe` + `gdb.exe`.
- **T017:** `windows-2022`, run [45305339035](https://github.com/SoufianoDev/aveloop/actions/runs/45305339035): breakpoint at `probe.c:4`, three steps to line 7, `$1 = 42`, `$2 = {value = 7, pad = 0}`, `#0 main () at probe.c:7`, both markers, `exited normally`. MSVC+CDB job: explicit configuration honoured, program launched and captured, scripted variable/stack output not capturable.
- **T018:** `macos-latest`, run [45305339035](https://github.com/SoufianoDev/aveloop/actions/runs/45305339035): steps across lines 4→7, `(int) local_int = 42`, `(Child) (value = 7, pad = 0)`, `frame #0`, both markers, `Process exited with status = 0`.
- **T019:** run [45304616214](https://github.com/SoufianoDev/aveloop/actions/runs/45304616214): `[T008] Verification window opened successfully.` on both Windows and macOS with a clean exit.

Full detail: `docs/feasibility/t016-t019-gate-closure.md`.
