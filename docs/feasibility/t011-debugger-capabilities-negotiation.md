# T011 — Debugger Capabilities Negotiation and Reverse Navigation Independence

- **Task:** T011 (`specs/001-ave-loop-alpha/tasks.md` line 46)
- **Issue:** #11
- **Status:** VERIFIED on Linux x86_64 (GDB 16.2). Windows and macOS **NOT VERIFIED** — records published protocol specifications.
- **Date:** 2026-11-24
- **Branch:** `feat/gpui-kit-fast`
- **Governing requirements:** `spec.md` §5.3 (FR-047), §7.3; `plan.md` §3.4, §4.4; Constitution Article IV

## 1. Negotiated Capabilities Matrix

Capabilities negotiated and verified on Linux x86_64 (GDB 16.2) via DAP (`gdb -i dap`) and GDB/MI (`gdb -i mi3`), contrasted with platform baselines:

| Feature / Capability | Linux GDB 16.2 (DAP) | Linux GDB 16.2 (MI3) | macOS LLDB (`lldb-dap`) | Windows MSVC / GDB |
|---|---|---|---|---|
| **Interface Status** | **VERIFIED** | **VERIFIED** | Specification baseline | Specification baseline |
| `supportsConfigurationDoneRequest` | `true` | N/A (implicit) | `true` | Supported |
| `supportsFunctionBreakpoints` | `true` | Supported (`-break-insert`) | `true` | Supported |
| `supportsConditionalBreakpoints` | `true` | Supported (`-break-insert -c`) | `true` | Supported |
| `supportsHitConditionalBreakpoints` | `true` | Supported (`-break-after`) | `true` | Supported |
| `supportsEvaluateForHovers` | `true` | Supported (`-data-evaluate-expression`) | `true` | Supported |
| `supportsSetVariable` | `true` | Supported (`-var-assign`) | `true` | Supported |
| `supportsSetExpression` | `true` | Supported | `true` | Supported |
| `supportsDelayedStackTraceLoading` | `true` | Supported (`-stack-list-frames <low> <high>`) | `true` | Supported |
| `supportsSteppingGranularity` | `true` | Supported | `true` | Supported |
| `supportsReadMemoryRequest` | `true` | Supported (`-data-read-memory`) | `true` | Supported |
| `supportsWriteMemoryRequest` | `true` | Supported (`-data-write-memory-bytes`) | `true` | Supported |
| `supportsTerminateRequest` | `true` | Supported (`-gdb-exit`) | `true` | Supported |
| `supportTerminateDebuggee` | `true` | Supported (`kill`) | `true` | Supported |
| `supportsCancelRequest` | `true` | Supported (`interrupt`) | `true` | Supported |
| **`supportsStepBack`** | **`false` (unsupported)** | **N/A (unsupported)** | **`false` (unsupported)** | **`false` (unsupported)** |

## 2. Explicit Verification of Reverse-Step Independence

### 2.1 The Requirement
- **`spec.md` FR-047:** *"The system shall use captured records/checkpoints for reverse navigation; it shall not require debugger reverse-step support."*
- **`spec.md` §7.3:** *"supportsStepBack is not assumed. Backward timeline navigation is implemented over AveLoop's recorded trace."*
- **`plan.md` §4.4:** *"Previous/seek selects a prior recorded stop and resolves its state from the trace. It never asks LLDB to reverse the running process."*
- **Constitution Article IV (Execution Truth):** *"Execution history is reconstructed only from observations actually recorded by the debugger."*

### 2.2 Live Protocol Verification
During GDB 16.2 DAP initialization:
```json
{
  "command": "initialize",
  "arguments": { "clientID": "aveloop", "adapterID": "gdb" }
}
```
The response body omitted `supportsStepBack` completely, confirming it is `false` (unsupported).

### 2.3 Architectural Proof of Independence
AveLoop's design decouples timeline scrubbing and state reconstruction from live debugger execution:
1. **Forward Execution Capture:** As the debugger steps forward, each execution stop produces an ordered `DebuggerObservation` (stack frames, local variables, aggregates, pointers, stdout/stderr chunks).
2. **Trace Store:** Observations are stored sequentially in domain `ExecutionTrace`.
3. **Deterministic Navigation:** Navigating backwards in time adjusts `SessionState::selected_stop_index` and reconstructs the visible execution state from the recorded checkpoint/observation store.
4. **No Process Reversal:** The live process is never requested to execute in reverse (`reverse-step`, `reverse-continue`, or `stepBack`). Reverse navigation functions even after the target process has terminated.

## 3. Conclusions
- AveLoop does not require or expect `supportsStepBack` from GDB, LLDB, or MSVC.
- Reverse navigation is entirely self-contained within AveLoop's pure domain trace architecture (`src/domain/trace.rs`).
