# T008 — GPUI Input, Focus, Clipboard, and Window Behavior

- **Task:** T008 (`specs/001-ave-loop-alpha/tasks.md` line 43)
- **Issue:** #8
- **Status:** PARTIALLY VERIFIED on Linux x86_64 (Wayland). X11 session, macOS, and Windows not verified.
- **Date:** 2026-11-23
- **Branch:** `feat/gpui-kit-fast`
- **Crates:** `gpui-kit` 0.10.0, `gpui-fast` 0.1.17

## 1. Environment

- `rustc 1.96.0` / `cargo 1.96.0`
- Kernel `6.14.0-37-generic`, glibc 2.41 (Ubuntu)
- `WAYLAND_DISPLAY=wayland-0`, `DISPLAY=:0`, `XDG_SESSION_TYPE=wayland`
- Resources at run time: 30 GB free disk, 1.9 GiB available RAM

## 2. Build

```
cargo build --bin t008_verify
Finished `dev` profile [unoptimized + debuginfo] target(s) in 57.83s
```

First clean build of the GPUI dependency tree (`gpui-fast`, `gpui-component`, `gpui-kit`) takes several minutes; incremental builds are seconds.

## 3. Automated interaction tests — PASS

`cargo test --test t008_interaction_tests -- --nocapture`:

```
running 2 tests
test test_t008_window_resizing_and_layout_reflow ... ok
test test_t008_text_input_focus_and_keyboard ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
```

What the tests assert, using the GPUI test harness:

| Behavior | Assertion | Result |
|---|---|---|
| Focus | Clicking the input marks it focused | PASS |
| Text input | Typing `int main()` lands in the field value | PASS |
| Backspace | Backspace removes the trailing character | PASS |
| Clipboard write via UI | Copy button writes the field value and sets status | PASS |
| Clipboard read back | Harness clipboard holds `int main(` | PASS |
| Clipboard paste via UI | Paste button sets status and clipboard result | PASS |
| Resize | Window bounds change 600×400 → 1024×768 | PASS |
| Reflow | Controls remain visible after resize | PASS |

## 4. Runtime probe — window opens, clean quit fails

`AVELOOP_PROBE_EXIT=1 timeout 60 ./target/debug/t008_verify`:

```
[T008] Starting GPUI interaction verification...
[T008] WAYLAND_DISPLAY: Some("wayland-0")
[T008] DISPLAY: Some(":0")
[T008] Verification window opened successfully.
[T008] Automated probe mode: closing application.
```

Observed facts:

- The application initializes and opens a real 720×480 window on Wayland with GPUI Fast (`const { assert!(gpui_kit::GPUI_FAST) }` passes at compile time).
- `cx.quit()` did not terminate the process: the probe was killed by `timeout` (exit 124) after 60 s.
- The probe's programmatic clipboard block printed no `PASSED` line, so `cx.read_from_clipboard()` returned nothing at that point in startup.

## 5. Verification status

| Requirement | Status | Basis |
|---|---|---|
| Text rendering | VERIFIED (test harness + window) | Tests pass; window renders |
| Keyboard input | VERIFIED (test harness) | Typing and backspace assertions |
| Focus | VERIFIED (test harness) | Focus assertion after click |
| Clipboard (app path) | VERIFIED (test harness) | Copy/paste handlers round-trip |
| Clipboard (real system clipboard) | **NOT VERIFIED** | Probe read returned nothing |
| Window resizing / reflow | VERIFIED (test harness) | Bounds and visibility assertions |
| Wayland configuration | VERIFIED | Window opened on `wayland-0` |
| X11 configuration | **NOT VERIFIED** | Session is Wayland; X11 not exercised |
| Clean application exit | **NOT VERIFIED** | Probe required a timeout kill |
| macOS / Windows | **NOT VERIFIED** | Host is Linux |

## 6. Follow-ups (not authorized work, recorded for the owner)

1. Real system clipboard round-trip: read via `wl-paste`/`xclip` while the app is running, or fix the probe's read timing.
2. Quit path: determine whether `cx.quit()` inside the run closure is a harness misuse or a GPUI Fast behavior, and verify a window close terminates the process.
3. X11 session run to satisfy the "Wayland/X11 configurations" clause.

## 7. Reproduce

```bash
cargo test --test t008_interaction_tests -- --nocapture
cargo build --bin t008_verify
AVELOOP_PROBE_EXIT=1 timeout 60 ./target/debug/t008_verify
```
