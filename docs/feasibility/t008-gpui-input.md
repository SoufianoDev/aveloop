# T008 — GPUI Input, Focus, Clipboard, and Window Behavior

- **Task:** T008 (`specs/001-ave-loop-alpha/tasks.md` line 43)
- **Issue:** #8
- **Status:** VERIFIED on Linux x86_64 (Wayland and X11), with one Wayland clipboard caveat.
- **Date:** 2026-11-23
- **Branch:** `feat/gpui-kit-fast`
- **Crates:** `gpui-kit` 0.10.0, `gpui-fast` 0.1.17

## 1. Environment

- `rustc 1.96.0` / `cargo 1.96.0`
- Kernel `6.14.0-37-generic`, glibc 2.41 (Ubuntu)
- Wayland session: `WAYLAND_DISPLAY=wayland-0`, `DISPLAY=:0`, `XDG_SESSION_TYPE=wayland`
- X11 session: `Xephyr :99 -screen 1024x768 -ac`, `WAYLAND_DISPLAY` unset, `DISPLAY=:99`
- External clipboard tool: `xsel` (X11). No `wl-clipboard` package is installed.

## 2. Build

```
cargo build --bin t008_verify
Finished `dev` profile [unoptimized + debuginfo] target(s) in 32.41s
```

First clean build of the GPUI dependency tree takes several minutes; incremental builds are ~30 s.

## 3. Automated interaction tests — PASS

`cargo test --test t008_interaction_tests -- --nocapture`:

```
running 2 tests
test test_t008_window_resizing_and_layout_reflow ... ok
test test_t008_text_input_focus_and_keyboard ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
```

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

## 4. Runtime probe

### 4.1 Wayland

```
[T008] WAYLAND_DISPLAY: Some("wayland-0")
[T008] DISPLAY: Some(":0")
[T008] Programmatic clipboard write & read: FAILED (None)
[T008] Verification window opened successfully.
[T008] Automated probe mode: closing application.
exit=0
```

### 4.2 X11 (Xephyr)

```
[T008] WAYLAND_DISPLAY: None
[T008] DISPLAY: Some(":99")
[T008] Programmatic clipboard write & read: PASSED
vulkan: No DRI3 support detected - required for presentation
[T008] Verification window opened successfully.
[T008] Automated probe mode: closing application.
exit=0
```

### 4.3 Real system clipboard round-trip (X11)

Two directions, verified against an external client:

| Direction | Command | Observed |
|---|---|---|
| App writes → external reads | `xsel -b -o` on `:99` while the app runs | `AveLoop T008 Clipboard Verification` |
| External writes → app reads | `printf 'External Clipboard Text' \| xsel -b -i`, then `AVELOOP_PROBE_READ_ONLY=1` | `Clipboard read from external owner: "External Clipboard Text"` |

### 4.4 Quit path

`cx.quit()` called directly inside the launch callback was swallowed: the process never exited and had to be killed by `timeout` (exit 124). Deferring the quit to the end of the effect cycle (`cx.defer`) makes both backends exit cleanly with code 0.

Reading the clipboard from inside a deferred callback panics on X11 (`RefCell already borrowed`, `x11/client.rs:1904`), so the probe reads before the event loop starts and only defers the quit.

## 5. Verification status

| Requirement | Status | Basis |
|---|---|---|
| Text rendering | VERIFIED | Tests pass; window renders on both backends |
| Keyboard input | VERIFIED | Typing and backspace assertions |
| Focus | VERIFIED | Focus assertion after click |
| Clipboard (app path) | VERIFIED | Copy/paste handlers round-trip in tests |
| Clipboard (system, X11) | VERIFIED | Both directions against `xsel` |
| Clipboard (system, Wayland) | **NOT VERIFIED** | Own-write read returns `None`; no `wl-clipboard` installed to act as external owner |
| Window resizing / reflow | VERIFIED | Bounds and visibility assertions |
| Wayland configuration | VERIFIED | Window opens, clean exit |
| X11 configuration | VERIFIED | Window opens, clipboard both ways, clean exit |
| macOS / Windows | **NOT VERIFIED** | Host is Linux |

## 6. Remaining caveat

Wayland clipboard reads return `None` in this environment, both for the app's own write and for an externally owned selection. Two plausible causes are not yet separated:

1. No Wayland clipboard client (`wl-copy`/`wl-paste`) is installed, so no independent owner could be observed.
2. Wayland delivers selection offers only to a focused client; the probe window is not focused in this session.

Re-test with `wl-clipboard` on a focused Wayland session before relying on paste in production. The X11 path is fully verified.

<!-- coderabbit-ai-generated:v1 -->

<!-- coderabbit-ai-generated-section:v1:d9d995f25eb47b82d3ec5816c0f34083b0521c7e9a87588276e213ea6a417ec1 -->

## 7. Follow-up verification

Sections 1–6 record the prior Wayland and X11 runs. The current probe schedules automatic shutdown on the foreground executor after the native event loop starts. Clipboard checks still run during launch to avoid the X11 borrow panic described above.

In the virtual display checks, the standalone binary opened a native window and the automated command below exited with status 0 on X11 (Xvfb) and Wayland (Weston nested in Xvfb). The resize test confirmed that both the viewport and the rendered workspace grew. Wayland system clipboard behavior, macOS, and Windows remain unverified.

## 8. Reproduce

```bash
cargo test --test t008_interaction_tests -- --nocapture
cargo build --bin t008_verify

# In a Wayland session:
AVELOOP_PROBE_EXIT=1 timeout 60 ./target/debug/t008_verify

# From a terminal with access to a running X11 test display:
env -u WAYLAND_DISPLAY DISPLAY=:99 AVELOOP_PROBE_EXIT=1 timeout 60 ./target/debug/t008_verify
```

The timeout is a failure safeguard. These commands leave the system clipboard untouched. Set `AVELOOP_PROBE_CLIPBOARD=1` to opt in to a write and read check; this can replace the clipboard contents. Set `AVELOOP_PROBE_READ_ONLY=1` to read an external selection without writing. Read-only mode takes precedence when both options are set.

To reproduce the external X11 round trip, run the following against an isolated X11 display:

```bash
# Leave the window running while reading its clipboard from another client.
env -u WAYLAND_DISPLAY DISPLAY=:99 AVELOOP_PROBE_CLIPBOARD=1 ./target/debug/t008_verify &
DISPLAY=:99 xsel -b -o

# Read an externally provided value without replacing it.
printf 'External Clipboard Text' | DISPLAY=:99 xsel -b -i
env -u WAYLAND_DISPLAY DISPLAY=:99 AVELOOP_PROBE_READ_ONLY=1 AVELOOP_PROBE_EXIT=1 timeout 60 ./target/debug/t008_verify
```

A successful exit alone does not verify the clipboard. Check for the corresponding clipboard output. The startup probe may not be able to claim ownership on Wayland; use the interactive Copy and Paste buttons in a focused session to check clipboard behavior there.

<!-- /coderabbit-ai-generated:v1 -->
