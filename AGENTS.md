# AveLoop — Agent Instructions

Applies to every coding agent and every session in this repository.

1. **Read [`HANDOFF.md`](HANDOFF.md) first, completely**, before inspecting, editing, or running anything. It is the canonical cross-agent continuity record.
2. **Use Planning with Files** (installed skill `planning-with-files`) as the only planning system. Restore the active plan under `.planning/`; never create a competing plan for the same task.
3. **Update `HANDOFF.md` continuously**, not only at session end: after discoveries, file changes, GitHub operations, command results, failures, phase transitions, commits, and before pausing or compacting.
4. **Record decision provenance.** Mark each decision `USER-CONFIRMED`, `AGENT-PROPOSED`, `VERIFIED-FACT`, `UNVERIFIED`, or `SUPERSEDED`. Never promote an agent proposal to a requirement without explicit user approval.
5. **Verify before resuming.** Confirm branch, HEAD, remote SHA, working tree, untracked files, issue states, and the previous task's status. Verified repository state overrides `HANDOFF.md`; correct the file and log the discrepancy.
6. **Keep the activity log append-only.** Never delete or rewrite prior entries; update only the current-state summary in place.
7. **Do not rely on conversation history or compaction summaries.** Anything important must be on disk in `HANDOFF.md`, the planning files, or the authoritative specification.
8. **Never stage `.planning/`** or other agent metadata. Stage explicit paths only.
9. **Never commit secrets** to `HANDOFF.md` or anywhere else.
10. **Product authority** is `specs/001-ave-loop-alpha/spec.md`, `plan.md`, and `tasks.md`. `complete.md` is derived — regenerate it, never edit it independently.
11. **Do not start a task before the current one is authorized and handed off.** The next task is identified in `HANDOFF.md` → Immediate Next Action.
12. **Whenever `tasks.md` is modified, reconcile every affected task with its canonical GitHub Issue in the same workflow.** Synchronize the title, scope, prerequisites, and completion criteria. Resolve mappings using the Spec Kit task field; never infer them from issue numbering.

Full protocol: `HANDOFF.md`.
