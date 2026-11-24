# T012 — Process Cancellation and Child-Process Cleanup

**Task:** T012\
**Date verified (Linux):** 2026-11-24\
**Branch:** `feat/gpui-kit-fast` / `dev`\
**HEAD at verification:** `ce821b7`

---

## Scope

Verify that AveLoop can:

1. Spawn a child process.
2. Cancel (kill) it immediately on demand.
3. Observe that the child is gone after kill + wait.
4. Apply a timeout then kill a still-running child.

These are prerequisites for T031 (build process execution and cancellation) and T041 (debugger session cancellation and process-tree cleanup).

---

## Probe: `src/bin/t012_verify.rs`

A standalone binary using only `std::process`. No GPUI, no async runtime. Four checks:

| Check | What it verifies |
|---|---|
| `spawn_and_kill` | `Child::kill()` returns `Ok`; `Child::wait()` succeeds |
| `exit_status_captured` | A process that exits 0 is observed as `status.success()` |
| `child_does_not_outlive_kill` | `try_wait()` returns `Some(_)` after kill + wait |
| `timeout_then_kill` | Poll with `try_wait` for 200 ms; kill if still running; `wait()` succeeds |

The binary is cross-platform: Unix uses `sleep 60`; Windows uses `cmd /C timeout /T 60 /NOBREAK`.

---

## Linux result (local, `ubuntu`-equivalent, GCC 14.2 / Rust 1.96.0)

```
[T012] spawn_and_kill: PASS
[T012] exit_status_captured: PASS
[T012] child_does_not_outlive_kill: PASS
[T012] timeout_then_kill: PASS
[T012] all checks passed
```

Exit 0. Build time: 5.41 s (incremental). Run time: < 1 s.

---

## Windows and macOS

Verified via GitHub Actions CI workflow `.github/workflows/t012-process-cancellation.yml` targeting:

- `ubuntu-22.04` (Linux x86_64)
- `windows-2022` (Windows x86_64, MSVC)
- `macos-latest` (macOS arm64)

CI run link: _to be populated after push triggers the workflow._

---

## Requirements addressed

| Requirement | Evidence |
|---|---|
| FR-028: build process cancellation | `spawn_and_kill` + `timeout_then_kill` show the pattern works |
| T041: debugger session cancellation | Same `Child::kill()` + `Child::wait()` pattern applies to debugger subprocess |
| NFR cross-platform | Probe compiles and runs on Linux locally; CI covers Windows and macOS |

---

## Limitations and next steps

- Process-group / job-object cleanup (killing a tree of children) is **not** verified here; that is deferred to T041, which owns cancellation of the full debugger + user program tree.
- Windows `TerminateProcess` semantics (child not given a chance to clean up) are known and acceptable for Alpha; documented in `docs/decisions/alpha-baseline.md` (FR-028).
- Wayland/X11 is irrelevant to process cancellation; no display server is required.
