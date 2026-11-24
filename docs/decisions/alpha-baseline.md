# Alpha Baseline Decisions

**Status:** Accepted\
**Date:** 23 November 2026\
**Authority:** `specs/001-ave-loop-alpha/spec.md` §14\
**Scope:** AveLoop Alpha

This record mirrors the specification's decisions register and adds the reason each decision exists. The specification remains authoritative; where this file and `spec.md` disagree, `spec.md` wins.

## Approved product decisions

| Decision | Reason | Source |
|---|---|---|
| Native desktop visual execution IDE | The product is a synchronized visual explanation of recorded execution, not an editor with a debugger panel | §1.1–1.2 |
| Rust-only implementation | One language and one memory model; GPUI is Rust-native | §11.1 |
| GPUI UI framework | Selected native framework | §11.2 |
| Linux, Windows, macOS as co-equal targets in one shared codebase | Parity is a product invariant, not a porting goal | §0.1.2, INV-008 |
| C17 and C++20 as the Alpha languages | Bounded, testable language scope | §7.1 |
| Material Design 3 as the real design system | Component and token foundation, not decoration | §8.3 |
| Night Owl as the application-wide default theme | Covers all chrome, not only syntax highlighting | §8.4 |
| HTML prototype is a preview/reference, not production UI or runtime | Its simulated boundaries are intentional prototype limits | §0.1.5, §8.2 |
| Python Tutor is a behavioral reference, not a product blueprint | Interaction inspiration without inheriting its model | §14 |
| One authoritative selected execution state | All panels must derive from one state | INV-002 |
| Real execution data only for claims presented as actual runtime facts | Fabricated completeness is a correctness defect | INV-001 |
| No Go, WebView production UI, or CodeMirror 5 | Rejected stacks | §14 |
| No forced multi-crate architecture | Modularity comes from boundaries, not crate count | §11.4 |

## Alpha baseline technical decisions

These are implementation selections required to make the Alpha executable. They are validated by the delivery gates in `plan.md`; they are not product requirements.

| Decision | Reason | Source |
|---|---|---|
| One Rust package / one application crate, modular monolith | Smallest structure that supports clear module boundaries | §11.3–11.4, `plan.md` §2.1 |
| Platform-native C/C++ toolchain discovery: GCC/G++ on Linux, Apple Clang on macOS, MSVC or MinGW-w64 on Windows | Use what each platform already provides; install nothing | §7.2 |
| Platform-native debugger integration: GDB on Linux, Apple LLDB on macOS, the debugger matching the Windows toolchain | One adapter per real backend; DAP is a protocol, GDB/MI is a fallback | §7.3 |
| Source-line / debugger-stop granularity | Alpha scope is source-level stepping, not instruction-level | §7.3, `plan.md` §4.2 |
| Recorded trace/checkpoint reconstruction for backward navigation | Backward navigation replays recorded states; it does not reverse the live process | `plan.md` §4.4 |
| Local execution under the user's OS permissions; not a hardened sandbox | Explicit consent and process control, no security-boundary claim | §2.2, FR-144 |
| Native GPUI editor for the bounded Alpha editing scope | No embedded browser editor; the editor is part of the product | `plan.md` §3.5 |
| Tree-sitter for C/C++ syntax parsing/highlighting; no LSP in Alpha | Sufficient for highlighting without an LSP dependency | `plan.md` §8 |
| Runtime memory visualization limited to debugger-observable/reachable entities and verified events | No allocation history the backend cannot supply | §6.4, `plan.md` §4.5 |
| GPUI Kit and GPUI Fast adoption (`gpui-kit` v0.10.0 with `gpui-fast` v0.1.17) | Rich component library (M3 compatible) and retained-mode CPU reduction (73-88%) | `docs/feasibility/gpui-kit-fast-integration.md` |
| GPUI Kit Material 0.2.0 adoption with `full` and `serde`, excluding `embedded-symbols` and `gpui-fast` (0.3.0 requires Kit/Base 0.11 and is not adopted) | Reuses a complete GPUI-native M3 component library without replacing Kit/Fast or embedding the full icon catalog | `plan.md` §8; T014 addendum |
| Windows toolchain selection is pair-based: MinGW-w64 GCC/G++ + GDB, or MSVC + CDB/DbgEng; both complete → prefer MinGW-w64/GDB; neither → report and install nothing; explicit configuration wins and is never silently substituted | A compiler without its compatible debugger cannot deliver a debug session, so compiler-only priority would select an unusable toolchain | USER-CONFIRMED 2026-11-24; §7.2–§7.3, `plan.md` §3.3–§3.4 |

## Superseded proposals

| Superseded statement | Replaced by | Reason |
|---|---|---|
| “C is the leading candidate”; “the first language remains open” | C17 and C++20 | Alpha language scope was approved | §0.1.1 |
| “Linux-first” | Three co-equal platforms | Parity is an invariant | §0.1.2 |
| Preliminary six/eight-crate Cargo workspace | One package, modular monolith | Crate count was not justified | §0.1.3 |
| Cargo workspace as a prerequisite | One Cargo package | A workspace is optional, not required | §0.1.4 |
| HTML prototype as the implementation | Reference asset only | Prototype boundaries are simulated | §0.1.5 |
| Debugger variable inspection implies complete heap knowledge | Capability-limited, verified views | Inspection is not allocation history | §0.1.6 |
| Clang/LLVM 23.1.4 as the mandatory cross-platform compiler baseline | Per-platform toolchain policy | No bundled toolchain; platforms use native compilers | §0.1.7, §7.2 |
| LLDB as the sole debugger backend | Per-platform debugger policy | GDB on Linux, LLDB on macOS, Windows matching toolchain | §0.1.7, §7.3 |

## Rejected alternatives

| Alternative | Why rejected |
|---|---|
| Linux-only Alpha | Contradicts the co-equal platform requirement |
| Separate platform-specific application codebases | Duplicates logic and breaks one behavior contract |
| Six/eight crates created up front | Structure without demonstrated benefit |
| Go execution backend | Adds a second runtime and ownership model |
| HTML/CSS/JS/WebView production UI | Not a native desktop application |
| Fake or static traces presented as real execution | Violates execution truth |
| Assuming debugger step-back exists | Capability is negotiated, never assumed |
| Claiming complete process heap visibility from variable inspection | Overstates what the backend establishes |
| Treating unresolved pointers as proven dangling pointers | “Dangling” requires an observed lifetime end |
| Creating or maintaining a local GPUI Kit Material fork or vendored upstream modification | The published upstream crate is consumed; a fork duplicates maintenance and hides dependency compatibility issues |

## Deferred beyond Alpha

- LSP, completion, and refactoring.
- Full STL/container visualization parity.
- Complete allocation instrumentation across custom allocators.
- True reverse execution.
- Remote or cloud execution.
- Collaboration and a plugin marketplace.
- Additional languages.
- Additional CPU architectures beyond the Alpha matrix.

## Consequences

- Any proposal that reopens a rejected or superseded item must state why the original reason no longer holds and update `spec.md` first.
- Technical decisions in this record are provisional until their delivery gates pass; failing a gate reopens the decision rather than the requirement.
- The constitution (`.specify/memory/constitution.md`) governs engineering conduct; this record governs scope.
