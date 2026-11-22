# AveLoop — Agent Handoff

Canonical continuity record for every agent working on this repository. Read this file completely before acting. If this file disagrees with verified repository state, the repository wins — correct this file and record the discrepancy.

## Current State

- **Last updated:** 2026-11-22T23:50Z (UTC; host local time is UTC+1)
- **Repository:** `SoufianoDev/aveloop` (`https://github.com/SoufianoDev/aveloop.git`)
- **Project root:** `/home/soufiano/SoufianoDev/AveLoop`
- **Active branch:** `main` (tracking `origin/main`)
- **Last verified HEAD:** `f259eadb8e840fe5c812898519022b4206d396c0`
- **Working tree:** clean before this checkpoint; this checkpoint adds `HANDOFF.md`, `AGENTS.md`, and a `.gitignore` line
- **Active task:** agent continuity protocol (this checkpoint)
- **Current phase:** Phase 3 — publish checkpoint
- **Overall status:** documentation baseline published; continuity checkpoint being published; no implementation started
- **Current objective:** publish `HANDOFF.md` + `AGENTS.md`, then hand off T001

## Immediate Next Action

Stage `HANDOFF.md`, `AGENTS.md`, and `.gitignore` by explicit path, commit them, push `main` to `origin/main`, verify the remote SHA equals the local HEAD, and confirm a clean working tree. Then deliver the T001 kickoff report — do not start T001.

## Verified Project State

Confirmed during this session (2026-11-22T23:47–23:50Z):

- `git rev-parse HEAD` = `git rev-parse origin/main` = `f259eadb8e840fe5c812898519022b4206d396c0`; `git rev-list --left-right --count origin/main...HEAD` = `0 0`.
- `git diff --check` exits 0.
- Working tree clean at checkpoint start.
- Tracked files are only `.gitignore`, `README.md`, and `specs/001-ave-loop-alpha/{spec,plan,tasks,complete}.md`. There is no Rust source, `Cargo.toml`, or lockfile in the repository.
- `prototype/` is gitignored (`prototype/*`) and untracked; `prototype/index.html` is a design reference and must not be modified.
- `README.md` is a one-line stub (`# aveloop`).
- Issues: #104 OPEN, #108 OPEN, #128 CLOSED (DUPLICATE), #130 CLOSED (DUPLICATE), #1 OPEN (`phase:setup`, `spec-kit-task`, 0 comments, no prerequisites).
- Installed planning skill: `/home/soufiano/.agents/skills/planning-with-files/SKILL.md`, version 3.25.2.
- `gh issue close --reason duplicate` is rejected by this CLI version (accepts only `completed`/`not planned`); use `gh api -X PATCH repos/SoufianoDev/aveloop/issues/<n> -f state=closed -f state_reason=duplicate`.

## User-Confirmed Decisions

### USER-CONFIRMED

- **2026-11-22 — Per-platform toolchain policy.** Linux uses an existing suitable GCC/G++ toolchain and GDB; macOS uses Apple's existing Clang and LLDB; Windows uses an existing compatible MSVC or MinGW-w64 toolchain and its matching debugger. Authority: `specs/001-ave-loop-alpha/spec.md` §7.2–7.3; `plan.md` §3.3–3.4; `tasks.md` T009–T011, T029–T035.
- **2026-11-22 — No automatic toolchain installation.** AveLoop does not bundle, download, or install a compiler, debugger, or LLVM distribution. Authority: `spec.md` NFR-009 and §7.2.
- **2026-11-22 — LLVM/Clang 23.1.4 is no longer mandatory.** The former requirement is recorded as superseded, not silently deleted. Authority: `spec.md` supersession note; `plan.md` §3.3.
- **2026-11-22 — DAP is a protocol, not a debugger.** GDB/MI remains a documented fallback; backend capabilities are negotiated and verified, never assumed; `supportsStepBack` is never assumed. Authority: `spec.md` §7.3; `plan.md` §3.4.
- **2026-11-22 — Execution truth.** Execution history is reconstructed only from observations actually recorded by the debugger. Authority: `spec.md` FR-047 and §7.3.
- **2026-11-22 — Commit/push authorization.** Commit and push the four specification files directly to `main`; no PR, no merge, no force-push, no history rewrite. Delivered as `f259ead`.
- **2026-11-22 — Issue reconciliation authorization.** Close only #128 and #130 as duplicates of #104 and #108. Do not close the canonicals; do not modify unrelated issues. Done.
- **2026-11-23 (local) — Mandatory continuity protocol.** Tracked root `HANDOFF.md`, root agent instructions, and Planning with Files remain the operating system for all AveLoop work.
- **2026-11-23 (local) — Stop after the T001 kickoff report.** Do not begin T001 implementation in this operation.

### AGENT-PROPOSED

- Add `.planning/` to `.gitignore` so agent planning metadata cannot be staged accidentally. Not yet explicitly approved.
- Constitution path candidates: `.specify/memory/constitution.md` (Spec Kit default) or `docs/constitution.md` (repo `docs/` convention). Awaiting user choice.
- T002 ("Create `specs/001-ave-loop-alpha/` …") appears already satisfied; verify and close rather than re-execute.

### VERIFIED-FACT

- HEAD/remote synchronization, clean tree, `git diff --check` exit 0, and issue states as listed above.
- Repository contents as listed above.
- Skill install path and version.

### UNVERIFIED

- Live GDB DAP initialization and negotiated capability set on this distribution.
- GDB/MI variable and thread inspection behavior.
- Apple `lldb-dap` with Command Line Tools only.
- Windows MSVC debugger integration without the Visual Studio IDE.
- Install sizes on macOS and Windows.
- GDB/LLDB runtime RSS.
- Whether the planning skill's `PreCompact` hook is wired in this OpenCode environment.

### SUPERSEDED

- Clang/LLVM 23.1.4 as a mandatory cross-platform toolchain.
- Bundled toolchain distribution.
- NFR-007 wording "DAP response" → "debugger-protocol response".

## Baseline (pre-handoff, reported by the preceding agent)

This baseline records work that happened **before `HANDOFF.md` existed**. It is a report, not an append-only log. Verify against repository and GitHub evidence before relying on details.

- Four specification files were revised to v1.1.0 (per-platform toolchain policy, DAP/MI separation, per-platform acceptance/risks/sources).
- `complete.md` regenerated from `spec.md` + `plan.md` + `tasks.md` using the existing markers; reported byte-identical.
- Trailing-whitespace cleanup converted 12 header lines to CommonMark backslash breaks; `git diff --check` reported exit 0.
- Issues #104/#108 metadata repaired (`Phase: false — undefined` → Phase 10); completion criteria strengthened.
- #128 and #130 retitled, scoped as superseded duplicates, commented, and closed with `state_reason=duplicate`.
- Documentation commit `f259ead` created and pushed.
- Pre-handoff planning notes archived outside the repository to `/tmp/opencode/aveloop-planning-notes-2026-11-22/`.

Verified this session: `f259ead` exists locally and on the remote, the tree is clean, and the issue states match. The detailed edit history above was **not** re-verified line by line.

## Completed Work

| Work | Evidence |
|---|---|
| Specification baseline v1.1.0 published | Commit `f259ead`; GitHub shows 4 changed files |
| Canonical issues verified and strengthened | #104, #108 bodies |
| Duplicates reconciled | #128, #130 CLOSED/DUPLICATE with comments |
| Planning skill restored and a single active plan created | `.planning/2026-11-22-aveloop-continuity-t001-kickoff/` |
| Continuity artifacts written | `HANDOFF.md`, `AGENTS.md`, `.gitignore` |

## Active and Pending Work

- **Phase 3 (active):** stage, commit, and push the continuity checkpoint.
- **Phase 4 (pending):** deliver the T001 kickoff report. No implementation.
- **T001 (next task, not started):** establish the project constitution.

## Files Changed

| Path | Change | Purpose | Verification |
|---|---|---|---|
| `HANDOFF.md` | added | Canonical cross-agent continuity record | Pending commit |
| `AGENTS.md` | added | Root agent instructions pointing to `HANDOFF.md` | Pending commit |
| `.gitignore` | modified | Exclude local `.planning/` metadata | Pending commit |
| `.planning/2026-11-22-aveloop-continuity-t001-kickoff/*` | untracked, ignored | Local planning files | Not tracked by design |
| `specs/001-ave-loop-alpha/{spec,plan,tasks,complete}.md` | committed in `f259ead` | Authoritative Alpha baseline | Verified clean |

## GitHub Activity

| Issue | Relationship | State | Notes |
|---|---|---|---|
| #104 | Canonical T061 | OPEN | Criteria transferred from #128; Phase 10 |
| #108 | Canonical T063 | OPEN | Criteria transferred from #130; Phase 10 |
| #128 | Duplicate of #104 | CLOSED / DUPLICATE | Retitled, scoped with link, 1 comment |
| #130 | Duplicate of #108 | CLOSED / DUPLICATE | Retitled, scoped with link, 1 comment |
| #1 | T001 | OPEN | `phase:setup`, `spec-kit-task`, no prerequisites |
| #118 / #133 | T066 canonical / duplicate | OPEN | #133 marked superseded, not closed |
| #75 / #93 / #129 / #134 / #79 | Earlier reconciliations | — | Unchanged this session |

URLs: `https://github.com/SoufianoDev/aveloop/issues/<n>`.

## Execution and Verification Results

| Check | Result | Evidence |
|---|---|---|
| `git status --short` | PASS — empty before checkpoint | H0005 |
| `git rev-parse HEAD` vs `origin/main` | PASS — identical `f259ead` | H0005 |
| `git rev-list --left-right --count` | PASS — `0 0` | H0005 |
| `git diff --check` | PASS — exit 0 | H0005 |
| Issue state verification | PASS — 104/108 open, 128/130 duplicate-closed | H0006 |
| Planning skill located and read | PASS — v3.25.2 at `~/.agents/skills` | H0002, H0003 |
| `pwf_status` before init | PASS — no active plan (expected) | H0004 |
| Plan initialization | PASS — plan dir created | H0007 |

## Failed Attempts

| Attempt | Why it failed | What not to repeat |
|---|---|---|
| `gh issue close --reason duplicate` | CLI accepts only `completed`/`not planned` | Use the `gh api -X PATCH` route with `state_reason=duplicate` |
| `find / -name 'SKILL.md' -path '*planning*'` | Exceeded 60 s timeout | Search by directory name or known install roots |

## Unresolved Questions and Blockers

**Requires user decision**

- Constitution location for T001: `.specify/memory/constitution.md` vs `docs/constitution.md`. Neither directory exists yet.

**Technical uncertainty**

- GDB DAP live handshake and capability negotiation.
- GDB/MI inspection depth.
- `lldb-dap` with Command Line Tools only.
- MSVC debugger integration without the IDE.
- macOS/Windows install sizes; GDB/LLDB RSS.

**Environment blockers**

- GPUI spike (T005) reported blocked by low disk (~2.4 GB free) and RAM (~1.1 GiB). Reported, not re-measured this session.
- Windows and macOS verification cannot run on this Linux machine.

## Activity Log (Append-Only)

Entries H0001–H0013 record this session. Pre-handoff work is summarized in **Baseline**, not fabricated here.

- [H0001] [2026-11-22T23:47Z] [continuity] ACTION: Received and accepted the mandatory continuity protocol.
  - Command/tool: user instruction
  - Target: repository root
  - Result: task set = restore planning, create `HANDOFF.md` + `AGENTS.md`, publish, hand off T001
  - Evidence: user message
  - State change: operation started; product scope unchanged

- [H0002] [2026-11-22T23:47Z] [continuity] ACTION: Located the installed planning-with-files skill.
  - Command/tool: `find / -type d -name 'planning-with-files'`
  - Target: filesystem
  - Result: `/home/soufiano/.agents/skills/planning-with-files`
  - Evidence: directory listing
  - State change: install path established; no guessed path used

- [H0003] [2026-11-22T23:47Z] [continuity] ACTION: Read the skill instructions.
  - Command/tool: Read + grep
  - Target: `SKILL.md`, version 3.25.2
  - Result: restore-before-create contract, phase markers, `PreCompact` hook, named-plan resolution
  - Evidence: SKILL.md lines 1–160 and hook table
  - State change: planning procedure known

- [H0004] [2026-11-22T23:47Z] [continuity] ACTION: Checked for an active plan.
  - Command/tool: `pwf_status`
  - Target: project
  - Result: `{"exists":false}` — no planning files present
  - Evidence: tool output
  - State change: confirmed a new plan is required, not a competing plan

- [H0005] [2026-11-22T23:47Z] [continuity] ACTION: Verified Git state.
  - Command/tool: `git status --short`, `git rev-parse HEAD`, `git rev-parse origin/main`, `git rev-list --left-right --count`, `git diff --check`
  - Target: `main`
  - Result: clean tree; both refs `f259ead`; `0 0`; exit 0
  - Evidence: shell output
  - State change: baseline confirmed

- [H0006] [2026-11-22T23:47Z] [continuity] ACTION: Verified issue states.
  - Command/tool: `gh issue view 104/108/128/130/1`
  - Target: GitHub
  - Result: #104/#108 OPEN; #128/#130 CLOSED/DUPLICATE; #1 OPEN
  - Evidence: `gh` JSON output
  - State change: none; reconciliation confirmed

- [H0007] [2026-11-22T23:48Z] [continuity] ACTION: Created the single active plan.
  - Command/tool: `pwf_init name=aveloop-continuity-t001-kickoff`
  - Target: `.planning/2026-11-22-aveloop-continuity-t001-kickoff/`
  - Result: `task_plan.md`, `findings.md`, `progress.md` created
  - Evidence: tool output with `plan_id`
  - State change: plan active

- [H0008] [2026-11-22T23:48Z] [continuity] ACTION: Recorded the host clock offset.
  - Command/tool: `date -u +%FT%TZ`
  - Target: host
  - Result: `2026-11-22T23:48:04Z` while local date is 2026-11-23
  - Evidence: shell output
  - State change: UTC chosen for all log timestamps

- [H0009] [2026-11-22T23:49Z] [continuity] ACTION: Filled the planning files.
  - Command/tool: write
  - Target: plan `task_plan.md`, `findings.md`, `progress.md`
  - Result: goal, four phases, decisions, findings, error log recorded
  - Evidence: file contents
  - State change: Phases 1–2 complete, Phase 3 in progress

- [H0010] [2026-11-22T23:50Z] [continuity] ACTION: Wrote the canonical handoff.
  - Command/tool: write
  - Target: `HANDOFF.md`
  - Result: current state, provenance-classified decisions, baseline, activity log, resume instructions
  - Evidence: this file
  - State change: continuity record exists in the repository root

- [H0011] [2026-11-22T23:50Z] [continuity] ACTION: Wrote root agent instructions.
  - Command/tool: write
  - Target: `AGENTS.md`
  - Result: short instructions pointing to `HANDOFF.md` and Planning with Files
  - Evidence: file contents
  - State change: future agents have a canonical entry point

- [H0012] [2026-11-22T23:50Z] [continuity] ACTION: Excluded local planning metadata from Git.
  - Command/tool: edit
  - Target: `.gitignore`
  - Result: `.planning/` added; `prototype/*` preserved
  - Evidence: file diff
  - State change: planning files cannot be staged accidentally

- [H0013] [2026-11-22T23:50Z] [continuity] ACTION: Inspected the intended changeset.
  - Command/tool: `git status --short`, `git diff --check`, `git diff`
  - Target: working tree
  - Result: only `HANDOFF.md`, `AGENTS.md`, `.gitignore`; no source or manifest changes
  - Evidence: shell output
  - State change: ready to stage

## Resume Instructions

1. Read repository-level agent instructions (`AGENTS.md`).
2. Read `HANDOFF.md` completely.
3. Restore the active Planning with Files plan (`.planning/2026-11-22-aveloop-continuity-t001-kickoff/`, or the newest named plan).
4. Verify current Git state and relevant remote state.
5. Verify the important claims in this handoff.
6. Identify discrepancies before taking action.
7. Continue from **Immediate Next Action**.

If this file conflicts with verified repository evidence, the repository wins. Correct this file and record the discrepancy in the Activity Log.
