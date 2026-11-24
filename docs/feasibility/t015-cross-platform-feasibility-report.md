# T015 — Consolidated Cross-Platform Feasibility Report

- **Task:** T015 (`specs/001-ave-loop-alpha/tasks.md` line 50) — Phase 2 exit gate
- **Issue:** #15
- **Date:** 2026-11-24
- **Branch:** `feat/gpui-kit-fast`
- **Gate text:** "platform feasibility matrix is completed with evidence. A platform blocker is resolved or the Alpha release target is explicitly re-baselined before production foundation work. Do not silently proceed with Linux-only behavior." (`plan.md`)
- **Verdict:** **GATE NOT MET.** The report is complete; three gaps remain, one of which is an unresolved architecture question (Windows debugger backend). Production foundation work must not start until the user chooses remediation or re-baselines.

## 1. Evidence levels

Every row below is labelled with what was actually executed, not what was intended.

| Level | Meaning |
|---|---|
| **LIVE** | Executed on that OS; observed output recorded |
| **PARTIAL** | Executed, but only part of the required behaviour was exercised |
| **COMPILE** | Build/type-check only; nothing was launched |
| **FORMAT** | Artefact or file-format inspection only; no execution |
| **SPEC** | Vendor documentation baseline; nothing executed |
| **EVAL** | Source-code evaluation; no runtime |

## 2. Consolidated matrix

| Alpha capability | Linux x86_64 | macOS arm64 | Windows x86_64 |
|---|---|---|---|
| GPUI build | **LIVE** (`t005-gpui-linux.md`) | **COMPILE** (`cargo check`, run 45292424668) | **COMPILE** (`cargo check`, run 45292424668) |
| GPUI launch / window | **LIVE** (Wayland + X11) | not executed | not executed |
| Input, focus, clipboard, resize (T008) | **LIVE**, one Wayland clipboard caveat | not executed | not executed |
| C17 + C++20 debug build and run (T009) | **LIVE** (GCC 14.2) | **LIVE** (Apple Clang) | **LIVE** (MSVC 19.44) |
| Debugger launch, breakpoint, stepping, stack, variables, pointers, aggregate children, stdout/stderr, termination (T010) | **LIVE** (GDB 16.2, DAP + MI) | **PARTIAL** (`lldb --batch`: breakpoint, run, backtrace only) | **FORMAT** (`dumpbin /HEADERS` CodeView/PDB only) |
| Capability negotiation + `supportsStepBack: false` (T011) | **LIVE** (DAP + MI3) | **PARTIAL** (`lldb-dap` `initialize` response) | none |
| Process spawn, kill, reap, timeout-kill (T012) | **LIVE** (4/4) | **LIVE** (CI run 45294000672) | **LIVE** (CI run 45294000672) |
| Tree-sitter C/C++ parse + node kinds (T013) | **LIVE** | **LIVE** (run 45297491219) | **LIVE** (run 45297491219) |
| Text primitives, editor widget, M3 surface (T014) | **EVAL** (pinned source) | **EVAL** (shared source) | **EVAL** (shared source) |

## 3. Verified path per Alpha target

| Target | Path chosen | Path status |
|---|---|---|
| Native windowing and UI | `gpui-kit` 0.10.0 → `gpui-fast` 0.1.17 (GPUI 0.2.2), one Rust package | Verified on Linux; compiles on macOS/Windows; runtime unproven there |
| C17 / C++20 toolchain | GCC/G++ 14.2 (Linux), Apple Clang (macOS), MSVC 19.44 (Windows), invoked as external processes | **Verified on all three** |
| Editor | `gpui-base`/`gpui-component` `Editor` + `EditorState`, tree-sitter C/C++ grammars 0.24.2/0.23.4 (MIT) | Grammar verified on all three; editor widget source-evaluated only |
| Process control | `std::process::{Command, Child}` kill/wait with timeout supervision | **Verified on all three** |
| Debugger adapter | GDB DAP/MI on Linux; `lldb-dap` on macOS; **Windows: undecided** | Linux verified; macOS partial; Windows unresolved |
| Visual design | `gpui-component` M3-styled components + Night Owl semantic tokens | Source-evaluated; no visual audit |

## 4. Gap register

### G1 — Windows debugger backend is unresolved (blocking, architectural)

T010 requires "the compiler-matched debugger on Windows". The only Windows evidence is `dumpbin /HEADERS` confirming a CodeView/PDB record. No debugger was launched, no breakpoint set, no capability negotiated.

This is not just missing evidence: **no Windows debugger backend has been chosen.** MSVC's DAP adapter (`cppvsdbg`) ships only inside Visual Studio, not in the Windows SDK. The realistic options are:

- **GDB (MinGW-w64)** on Windows — keeps one adapter family (GDB DAP/MI) across Linux and Windows, but then the Windows toolchain must be MinGW-w64 rather than MSVC, which changes T009's verified path.
- **CDB + a custom MI/CLI adapter** — keeps MSVC, but adds an adapter AveLoop must own, and CDB has no DAP.
- **A commercial/third-party adapter** — dependency and licensing cost not yet evaluated.

`spec.md` allows "DAP or GDB/MI" per platform, so a Windows-specific backend is permissible, but the choice is an Alpha-scope decision the user must make. This is the only gap that changes architecture rather than adding verification.

### G2 — macOS debugger depth (blocking, verification only)

Verified: breakpoint, launch, backtrace via `lldb --batch`, plus `lldb-dap` `initialize` capability response. Not verified: source-level stepping, variables, pointer values, aggregate children, stdout/stderr separation, termination. Remediation is a CI script extension; no architecture change.

### G3 — macOS and Windows GPUI runtime (blocking, verification only)

`cargo check --workspace --tests` proves the crate graph compiles for Metal and Win32. It does not prove the application opens a window, receives input, or renders text on those platforms. GPUI's macOS and Windows backends are the least-exercised code in the stack, so compile success is weak evidence.

Remediation: a CI job that actually launches `aveloop` (or a minimal probe binary) and asserts a window exists and a frame was produced. Risk: headless GPU availability on hosted runners (Metal on macOS arm64 VMs, WARP/D3D on Windows) is plausible but unproven; if the hosted runners cannot render, the honest fallback is physical-host evidence or an explicit re-baseline.

### G4 — T008 Wayland clipboard read (minor, documented)

Clipboard read returns `None` on Wayland without `wl-clipboard`, and offers may require focus. X11 verified. Documented in `t008-gpui-input.md`; not a gate blocker because the write path and the editor's own clipboard flow are verified and the Alpha fallback is an in-app clipboard buffer.

### G5 — T014 follow-ups (non-blocking for the gate)

`tree-sitter-c`/`tree-sitter-cpp` passthrough features are still disabled in `Cargo.toml`; M3 variant names are not 1:1 with the design system; editor large-file/IME behaviour is unverified. All belong to the editor task, not to feasibility.

## 5. Corrections issued by this report

1. `docs/feasibility/cross-platform-matrix.md` marked T008, T010, and T011 as **VERIFIED** on macOS and Windows. The underlying CI run only performed `cargo check`, `lldb --batch`, and `dumpbin`. The matrix has been corrected to state evidence levels.
2. `docs/feasibility/t012-process-cancellation.md` still said the CI link was "to be populated". It is populated now: run 45294000672, all three legs success.

## 6. Decision required

The gate cannot pass on current evidence. Three options, in order of preference:

- **A — Close the gaps (recommended).** Extend the feasibility CI: live macOS/Windows launch probe, full macOS LLDB session, and a Windows debugger session after the backend decision. Scope: CI scripts and one probe binary; no product code. This is the only option that preserves the cross-platform Alpha.
- **B — Re-baseline Alpha to Linux-only.** Requires explicit user approval and edits to `spec.md`, `plan.md`, and `tasks.md`. Contradicts US-09 and the stated product promise.
- **C — Accept compile/format evidence.** Would leave the gate text unsatisfied and contradict T010/T011 as written. Not recommended; if chosen, it must be recorded as a deliberate, dated re-baseline, not as "verified".

G1 must be answered even if A is chosen, because the answer determines whether the Windows toolchain stays MSVC or moves to MinGW-w64.

## 7. Reproduce

```sh
gh run view 45292424668 --json jobs --jq '.jobs[] | "\(.name) \(.conclusion)"'
git show origin/ci/platform-matrix-verification:.github/workflows/feasibility-cross-platform.yml
gh run view 45294000672 --json jobs --jq '.jobs[] | "\(.name) \(.conclusion)"'
gh run view 45297491219 --json jobs --jq '.jobs[] | "\(.name) \(.conclusion)"'
```

## 8. Addendum (2026-11-24): gate re-evaluation after T016–T019

The verdict in §6 is the record at `bd4216d`. Route A (close the gaps) was chosen by the
user, and T016–T019 closed the three blocking gaps with CI evidence. No product code
changed.

| Gap | Closure | Evidence |
|---|---|---|
| G1 Windows debugger backend undecided | Pair-based policy user-confirmed and implemented; real detection on `windows-2022` selects MinGW-w64 + GDB; live session verified | T016 run 45304616166 (8/8 + detection); T017 run 45305339035 |
| G2 macOS debugger depth | Full LLDB session: launch, breakpoint, four steps, variables, aggregate, pointer, backtrace, streams, clean exit | T018 run 45305339035 |
| G3 macOS/Windows GPUI runtime | Real window opened and exited on both runners | T019 run 45304616214 |

**Revised gate verdict:** met for the Alpha target matrix, with these recorded residuals:

1. MSVC + CDB scripted variable/stack assertions are not observable under shell
   redirection on `windows-2022`; the CDB job verifies explicit-configuration
   honouring, build, launch, and stream capture only. MinGW-w64 + GDB is the verified
   Windows path and the policy's default preference.
2. GPUI input, focus, clipboard, and resize remain Linux-verified only; macOS and
   Windows are verified for build and runtime launch.
3. The Wayland clipboard read remains unverified (no `wl-clipboard`), as recorded in §4.

Detail: `docs/feasibility/t016-t019-gate-closure.md`; matrix:
`docs/feasibility/cross-platform-matrix.md`.
