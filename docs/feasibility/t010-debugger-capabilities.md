# T010 — Platform Debugger Capabilities Across Platforms

- **Task:** T010 (`specs/001-ave-loop-alpha/tasks.md` line 45)
- **Issue:** #10
- **Status:** VERIFIED on Linux x86_64 (GDB 16.2 via DAP and GDB/MI). Windows and macOS **NOT VERIFIED** — no physical/virtual hosts in this environment; those rows record published vendor baseline and specifications.
- **Date:** 2026-11-24
- **Branch:** `feat/gpui-kit-fast`
- **Governing requirements:** `spec.md` §5.2 (FR-026, FR-027), §5.3 (FR-040..FR-047), §6, §7.3; `plan.md` §3.4, §4

## 1. Environment

| Component | Observed |
|---|---|
| Debugger (`gdb --version`) | `GNU gdb (Ubuntu 16.2-8ubuntu1) 16.2` |
| Python support | Python 3 embedded in GDB (required for GDB DAP) |
| Compiler (`gcc --version`) | `gcc (Ubuntu 14.2.0-19ubuntu2) 14.2.0` |
| C++ Compiler (`g++ --version`) | `g++ (Ubuntu 14.2.0-19ubuntu2) 14.2.0` |
| Target triple | `x86_64-linux-gnu` |
| Debug information format | DWARF 5 (GCC 14.2 default under `-g -O0`) |
| Kernel | Linux x86_64 (Ubuntu) |

No compiler, debugger, or package was downloaded or installed for this task. All verification used the system-installed tools.

## 2. Linux GDB Capabilities Verification

The required capabilities from `spec.md` §5.2, §5.3, §6, and `tasks.md` T010 were verified against C17 and C++20 binaries built with `-g -O0`:

| Capability | CLI / MI / DAP Command | Result | Verification Notes |
|---|---|---|---|
| **Debugger launch** | `gdb -batch -ex "run"`, `-exec-run`, DAP `launch` | **PASS** | Target process spawned, PID assigned, libraries loaded (`ld-linux`, `libc`), thread group created. |
| **Breakpoint** | `break helper`, `-break-insert helper`, DAP `setBreakpoints` | **PASS** | Symbol resolved to virtual address and source location (`probe.c:16`), hit count tracked. |
| **Source stepping** | `step` (into), `next` (over), `-exec-next`, DAP `next`/`stepIn` | **PASS** | Advances source lines cleanly. Line bounds respected. |
| **Call stack** | `backtrace`, `-stack-list-frames`, DAP `stackTrace` | **PASS** | Accurate frame stack: `#0 helper` at `probe.c:16`, `#1 main` at `probe.c:25` with exact frame pointers and instruction addresses. |
| **Variables (primitives)** | `info locals`, `info args`, `-stack-list-variables`, DAP `variables` | **PASS** | Variable names, types, and values reported accurately without optimization erasure. |
| **Aggregate children** | `print *n`, `-var-create` + `-var-list-children`, DAP `variables` | **PASS** | Struct fields `pt.x`, `pt.y`, nested aggregates, and C++ `std::vector` / `std::string` pretty-printed and expanded deterministically. |
| **Pointer values** | `print n`, `print n->next`, DAP `variables` | **PASS** | Distinct address values (`0x7fffffffdaf0`), NULL pointers (`0x0`), and dereferencing verified. |
| **stdout/stderr capture** | Inferior stream forwarding / PTY / pipe | **PASS** | Standard output and standard error captured during execution without buffer corruption. |
| **Termination** | `-thread-group-exited`, `exited-normally`, DAP `terminated` | **PASS** | Exit status (code 0) captured cleanly; inferior process terminates without zombies. |

## 3. Protocol Interfaces (DAP vs MI)

### 3.1 Debug Adapter Protocol (GDB DAP)

GDB 16.2 provides native DAP support via `gdb -i dap`. A live DAP handshake was performed:

```json
{
  "seq": 1,
  "type": "request",
  "command": "initialize",
  "arguments": {
    "clientID": "aveloop",
    "adapterID": "gdb"
  }
}
```

Capabilities negotiated from GDB 16.2:
- `supportsTerminateRequest`: `true`
- `supportTerminateDebuggee`: `true`
- `supportsCancelRequest`: `true`
- `supportsDelayedStackTraceLoading`: `true`
- `supportsDisassembleRequest`: `true`
- `supportsValueFormattingOptions`: `true`
- `supportsEvaluateForHovers`: `true`
- `supportsSetExpression`: `true`
- `supportsSetVariable`: `true`
- `supportsConfigurationDoneRequest`: `true`
- `supportsBreakpointLocationsRequest`: `true`
- `supportsReadMemoryRequest`: `true`
- `supportsWriteMemoryRequest`: `true`
- `supportsSingleThreadExecutionRequests`: `true`
- `supportsSteppingGranularity`: `true`
- **`supportsStepBack`**: **`false`** (absent). In accordance with `spec.md` §7.3 and FR-047, AveLoop does **not** rely on debugger reverse stepping; reverse navigation is reconstructed strictly from recorded execution checkpoints.

### 3.2 GDB Machine Interface (MI3)

GDB/MI was verified as a fully functional alternative / fallback interface via `gdb -q -i mi3`:
- Breakpoint insertion: `^done,bkpt={number="1",...}`
- Inferior execution: `*stopped,reason="breakpoint-hit",frame={...}`
- Variable objects: `-var-create var_n * "n"` -> `-var-list-children var_n` creates and traverses hierarchical variable trees with stable variable tokens.
- Frame inspection: `-stack-list-frames` returns typed frame structures.

## 4. Cross-Platform Debugger Matrix

| Platform | Target Debugger | Protocol | Status | Notes |
|---|---|---|---|---|
| **Linux x86_64** | GNU gdb 16.2 | DAP (`-i dap`) or GDB/MI (`-i mi3`) | **VERIFIED** | System-installed GDB 16.2 on Ubuntu; all 9 debugger operations verified. |
| **macOS arm64** | Apple LLDB | `lldb-dap` | **NOT VERIFIED** | Requires physical/virtual Apple Silicon host. Based on Apple developer toolchain specification. |
| **Windows x86_64** | MSVC debugger / MinGW GDB | VS debug engine / GDB DAP | **NOT VERIFIED** | Requires Windows host. Based on MSVC / MinGW-w64 documentation. |

## 5. Next Steps

- **T011:** Formally document the backend capability negotiation and confirm that AveLoop's state reconstruction design operates independently of `supportsStepBack`.
- **T012:** Verify process cancellation and child-process cleanup behavior on Linux.
