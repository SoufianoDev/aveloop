# AveLoop Alpha — Task Breakdown

**Document ID:** AVE-TASKS-001\
**Version:** 1.1.0\
**Status:** Derived from `spec.md` and `plan.md`; not implementation authorization\
**Date:** 22 November 2026

---

## Task conventions

- `[ ]` means not started.
- `[P]` marks work that can proceed in parallel once its stated prerequisites are met.
- `[USn]` maps work to the corresponding user story in `spec.md`.
- Tasks follow the Spec Kit structure: Setup, Foundational, prioritized User Stories, and Polish.
- Each task produces a reviewable outcome with a verification method.
- The list groups cohesive work. It is not a file-by-file checklist.
- File paths are introduced only when the relevant responsibility is implemented; no empty placeholder directories.
- No task authorizes repository creation, fork creation, commits, pushes, PRs, or upstream changes.

---

# Phase 1 — Setup

- [x] T001 Establish the AveLoop project constitution covering Rust-only implementation, shared cross-platform code, modular-monolith boundaries, execution truth, testing, dependency governance, and honest capability reporting.
- [x] T002 Create `specs/001-ave-loop-alpha/` and place the authoritative `spec.md`, `plan.md`, and `tasks.md` artifacts there.
- [x] T003 [P] Record approved decisions, superseded proposals, and rejected alternatives in `docs/decisions/alpha-baseline.md`.
- [x] T004 Inspect the existing repository and preserve `index.html` as a prototype/reference asset; do not convert it into production UI.

**Gate:** one authoritative product baseline exists; no production scaffold is created yet.

---

# Phase 2 — Foundational feasibility

These are mandatory before production foundation work. They are verification tasks, not excuses to silently drop a target platform.

All compiler and debugger verification uses toolchains already installed on each platform. No compiler, debugger, or LLVM distribution is bundled, downloaded, or installed by the project.

- [x] T005 Verify a minimal GPUI application builds and launches on Linux x86_64; record the exact GPUI revision, Rust toolchain, system dependencies, and runtime result.
- [x] T006 [P] Verify the same GPUI application builds and launches on Windows x86_64.
- [x] T007 [P] Verify the same GPUI application builds and launches on macOS arm64.
- [x] T008 Verify GPUI text rendering, keyboard input, focus, clipboard, window resizing, and required Linux Wayland/X11 configurations across the target matrix.
- [x] T009 Verify the installed platform C/C++ toolchain builds C17 and C++20 debug binaries on Linux (GCC/G++), Windows (MSVC or MinGW-w64), and macOS (Apple Clang), documenting each platform's linker/runtime prerequisites.
- [x] T010 Verify platform debugger launch, breakpoint, source-level stepping, stack, variables, aggregate children, pointer values, stdout/stderr, and termination on all three platforms: GDB on Linux, LLDB on macOS, the compiler-matched debugger on Windows.
- [x] T011 Record actual debugger capabilities per platform and backend (DAP or GDB/MI); explicitly verify that the design does not depend on `supportsStepBack`.
- [x] T012 Verify process cancellation and child-process cleanup behavior on each OS.
- [x] T013 [P] Verify Tree-sitter C/C++ grammar integration and license compatibility.
- [x] T014 Evaluate GPUI text primitives and the required Material 3 component surface; document build/reuse decisions without creating a fork.
- [ ] T015 Produce the cross-platform feasibility report. Do not proceed to production foundation until every Alpha target has a verified path or the target matrix is explicitly re-approved.

**Gate:** GPUI, compiler, debugger, process control, and required platform prerequisites have evidence on Linux, Windows, and macOS.

---

# Phase 3 — Native application and architecture foundation

- [ ] T016 Pin the Rust toolchain, exact GPUI revision, and direct dependency versions; establish reproducible Cargo metadata and lockfile policy.
- [ ] T017 Create one Cargo package and a minimal native GPUI application that launches on all Alpha targets.
- [ ] T018 Establish the modular-monolith boundaries and composition root; keep domain independent of GPUI, DAP, filesystem, and OS process APIs.
- [ ] T019 Add shared diagnostics, structured logging, error classification, and background-work cancellation conventions.
- [ ] T020 [P] Add CI build, formatting, lint, and test checks for Linux x86_64, Windows x86_64, and macOS arm64.
- [ ] T021 Add architecture checks that prevent presentation/GPUI and infrastructure/DAP types from leaking into domain models.
- [ ] T022 Define and validate the initial `ave.toml` manifest contract, versioning, relative-path rules, and actionable invalid-project errors.

**Gate:** one shared application builds and launches on all targets; module boundaries are real and tested rather than represented by empty folders.

---

# Phase 4 — User Story 1: Project workflow and native source editing

- [ ] T023 [US1] Implement create/open project use cases and cross-platform project/source path handling.
- [ ] T024 [US1] Implement native GPUI workspace composition for editor, visualization, inspector, transport, and output regions.
- [ ] T025 [US1] Implement the bounded native editor: text input, cursor, selection, editing, undo/redo, clipboard, line numbers, save, and unsaved state.
- [ ] T026 [US1] Integrate Tree-sitter C/C++ parsing and syntax highlighting.
- [ ] T027 [US1] Bind buffers to source revisions; preserve trace provenance and mark a trace stale when its source diverges.
- [ ] T028 [US1] Verify create/open/edit/save/reopen and keyboard/focus behavior on all three platforms.

**Acceptance:** users can create/open a project and edit/save C17 and C++20 source in the native application on all Alpha targets.

---

# Phase 5 — User Story 2: Build and launch real C/C++ programs

- [ ] T029 [US2] Implement platform C/C++ compiler discovery — project configuration, `CC`/`CXX`, `cc`/`c++` alias resolution, GCC/G++, Apple Clang, MSVC, MinGW-w64 — with version/target reporting and platform prerequisite diagnostics.
- [ ] T030 [US2] Implement deterministic C17/C++20 compiler command construction from `ave.toml` for the discovered compiler family, including debug profile and provenance.
- [ ] T031 [US2] Implement build process execution, cancellation, stdout/stderr capture, exit status, and cleanup.
- [ ] T032 [US2] Map compiler diagnostics to source locations where available and distinguish build, launch, and runtime failures.
- [ ] T033 [US2] Implement explicit launch consent and clear disclosure that Alpha programs run locally with the user's OS permissions and are not sandboxed.
- [ ] T034 [US2] Verify real C17 and C++20 sample builds and failed-build behavior with the per-platform toolchains on Linux, Windows, and macOS.

**Acceptance:** real programs compile with debug information; failures are classified correctly; no failed build creates a valid-looking trace.

---

# Phase 6 — User Story 3: Debug sessions, trace capture, and historical navigation

- [ ] T035 [US3] Implement debugger transport and session lifecycle for the selected backend (GDB DAP or GDB/MI on Linux, `lldb-dap` on macOS, the compiler-matched Windows debugger), including initialize, capability negotiation, launch, configuration, and termination.
- [ ] T036 [US3] Implement source-level breakpoint/step operations and bounded retrieval of stack frames, scopes, variables, and aggregate children.
- [ ] T037 [US3] Capture stdout/stderr and ordered debugger observations while preserving source, build, session, thread, and frame provenance.
- [ ] T038 [US3] Implement trace records, derived events, bounded storage, and checkpoints without conflating observations, events, and snapshots.
- [ ] T039 [US3] Implement deterministic selected-state reconstruction from recorded trace data.
- [ ] T040 [US3] Implement first/previous/next/last, seeking, play/pause, and playback speed over recorded stops.
- [ ] T041 [US3] Implement cancellation, process-tree cleanup, protocol timeouts, malformed-response handling, and stale-session message rejection.
- [ ] T042 [US3] Verify capture and historical navigation using real C17/C++20 programs on all three target platforms and their platform debuggers; verify no reverse-debugger request is required.

**Acceptance:** real execution yields an ordered, provenance-bound trace; earlier states can be selected from recorded data without claiming reverse execution.

---

# Phase 7 — User Story 4: Runtime inspection and synchronized visualization

- [ ] T043 [US4] Implement the authoritative selected-state owner and atomic publication contract consumed by all runtime panels.
- [ ] T044 [US4] Implement Stack and Variables views from resolved frames, scopes, and debugger observations.
- [ ] T045 [US4] Implement arrays, structures, C++ class/struct members, and explicit known/unavailable/optimized-out value states.
- [ ] T046 [US4] Implement pointer and reference visualization: null, verified targets, aliases, and interior offsets only when supported by evidence.
- [ ] T047 [US4] Implement the Heap observable-object view, with explicit coverage limits and verified allocation/deallocation labels only.
- [ ] T048 [US4] Implement unresolved/unknown pointer states and prevent unsupported dangling-pointer or complete-heap claims.
- [ ] T049 [US4] Add invariant tests proving that source, Stack, Variables, Heap, Pointer, Trace, Timeline, and attributable Output show the same selected stop.
- [ ] T050 [US4] Verify synchronization during forward/backward navigation, seek, playback, session replacement, missing data, and reconstruction failure.

**Acceptance:** runtime visualizations are truthful, synchronized, and explicit about incomplete debugger visibility.

---

# Phase 8 — User Story 5: Trace inspector, output, and workspace behavior

- [ ] T051 [US5] Implement the ordered Trace inspector and bind row selection to the authoritative selected-state use case.
- [ ] T052 [US5] Implement stdout/stderr display with honest output-to-state attribution.
- [ ] T053 [US5] Implement console copy, wrapping, follow-latest, resize, collapse, maximize, and restore.
- [ ] T054 [US5] Implement transport shortcuts and ensure editing mode stops playback without destroying the recorded trace.
- [ ] T055 [US5] Verify trace row, timeline, source highlight, runtime panels, and output remain synchronized.

**Acceptance:** trace/output navigation and workspace interactions preserve the single-state invariant.

---

# Phase 9 — User Story 6: Material 3, Night Owl, and accessibility

- [ ] T056 [US6] Define Material 3 semantic tokens and map the Night Owl palette to application-wide roles.
- [ ] T057 [P] [US6] Implement shared typography, shape, spacing, elevation, focus, and state-layer primitives.
- [ ] T058 [US6] Implement the required GPUI-native generic components and product-specific visualization components using the same token system.
- [ ] T059 [US6] Apply Night Owl consistently across editor chrome, workspace, panels, dialogs, console, timeline, and visualizations.
- [ ] T060 [US6] Complete responsive layout, keyboard-only, visible-focus, contrast, non-color status, and accessibility review.

**Acceptance:** the UI is a coherent Material 3 implementation with Night Owl, not a collection of unrelated default controls.

---

# Phase 10 — Cross-platform verification and release hardening

- [ ] T061 Add the full CI matrix for build, tests, formatting, lint, and dependency checks on all Alpha targets.
- [ ] T062 [P] Document per-platform compiler and debugger installation, discovery, prerequisites, and troubleshooting for Linux, Windows, and macOS using official vendor guidance, including that AveLoop never installs them.
- [ ] T063 Run and retain evidence for the complete create/edit/build/debug/record/navigate/inspect workflow on Linux.
- [ ] T064 [P] Run and retain evidence for the same full workflow on Windows.
- [ ] T065 [P] Run and retain evidence for the same full workflow on macOS.
- [ ] T066 Add long-trace, memory-bound, reconstruction, protocol-neutral adapter fault-injection, DAP-specific malformed-payload, timeout, and cancellation tests.
- [ ] T067 Complete dependency/license review — including confirmation that no compiler, debugger, or LLVM distribution is bundled — and inspect platform-specific code for unnecessary divergence.
- [ ] T068 Review all UI and documentation claims about heap completeness, pointer lifetime, reverse execution, and sandboxing.
- [ ] T069 Complete requirement-to-test traceability for every P0 requirement.
- [ ] T070 Run `/speckit.analyze`; resolve all Critical and High findings at the owning artifact, then rerun.
- [ ] T071 Run the Alpha acceptance checklist on all three target platforms and publish verified capabilities and known limitations.

**Final gate:** no P0 requirement lacks test/evidence; all three platforms pass the same Alpha workflow; no undocumented product or architecture divergence remains.

---

## Requirement coverage

| Requirement group | Task coverage |
|---|---|
| Project/source FR-001–008 | T022–T028 |
| Build/execute FR-020–028 | T029–T034 |
| Trace FR-040–050 | T035–T042 |
| Selected state FR-060–065 | T039–T050 |
| Runtime FR-080–092 | T043–T050 |
| Navigation/visualization FR-100–106 | T040, T043–T055 |
| Output FR-120–123 | T037, T052–T055 |
| Safety FR-140–145 | T033, T041, T066, T068 |
| Cross-platform NFRs | T005–T015, T017, T020, T028, T034, T042, T061–T065 |
| Material 3/Night Owl | T056–T060 |
| Release acceptance | T061–T071 |

---

## Spec Kit workflow and governance

Use the current Spec Kit workflow:

1. `/speckit.constitution` — establish project-wide principles once.
2. `/speckit.specify` — maintain the product contract in `spec.md`.
3. `/speckit.clarify` — resolve genuine ambiguity; do not reopen user-approved decisions.
4. `/speckit.plan` — maintain technical approach and research decisions in `plan.md`.
5. `/speckit.checklist` — generate a requirements-quality checklist.
6. `/speckit.tasks` — generate/reconcile dependency-ordered tasks from the spec and plan.
7. `/speckit.analyze` — read-only cross-artifact consistency analysis. Fix findings in the artifact that owns the issue and rerun.
8. `/speckit.implement` — only after explicit implementation authorization.
9. `/speckit.converge` — compare implementation against artifacts and append remaining work as appropriate.

The current documents have been structured according to this workflow, but `/speckit.analyze` has **not** been executed against the repository. Do not claim an analysis pass until the command has actually run.