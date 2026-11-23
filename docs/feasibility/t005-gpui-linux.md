# T005 — Linux x86_64 GPUI Feasibility Verification

- **Task:** T005 (`specs/001-ave-loop-alpha/tasks.md` line 40)
- **Status:** VERIFIED
- **Date:** 2026-11-23
- **Platform:** Linux x86_64 (Ubuntu 24.04 LTS, Kernel 6.11)
- **Compositor/Display:** Wayland (`WAYLAND_DISPLAY=wayland-0`, `DISPLAY=:0`)

## 1. Toolchain & Dependencies

- **Rust toolchain:** `rustc 1.96.0 (ac68faa20 2026-05-25)` / `cargo 1.96.0 (30a34c682 2026-05-25)`
- **GPUI revision:** `gpui v0.2.2` (crates.io)
- **Patch note:** Transitive dependency `zed-async-tar v0.5.0-zed` pulled legacy `xattr v0.2.3` which references `libc::ENOATTR`. On modern glibc / Linux x86_64, `ENOATTR` is not defined in `libc`. Fixed via `[patch.crates-io]` replacing `libc::ENOATTR` with `libc::ENODATA` in `xattr v0.2.3`.
- **System runtime libraries:**
  - `libwayland-client.so.0.23.1`
  - `libvulkan.so.1.4.304`
  - `/usr/lib/x86_64-linux-gnu/libvulkan_intel.so`
  - `libxkbcommon.so.0.0.0`
  - `libxkbcommon-x11.so.0.0.0`
  - `libfontconfig.so.1.14.2`
  - `libxcb.so.1.1.0`
  - Direct rendering device: `/dev/dri/renderD128`

## 2. Minimal Application

```rust
use gpui::*;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions::default(),
            |_window, cx| {
                cx.new(|_cx| HelloWorld {
                    text: "Linux GPUI Spike".into(),
                })
            },
        )
        .unwrap();
    });
}
```

## 3. Runtime Verification Evidence

1. **Compilation:** `cargo build -j 2` finished in 5m 42s with zero warnings/errors.
2. **Process launch:** Started PID 197317.
3. **Display connection:** Successfully bound to Wayland socket `wayland-0`.
4. **Hardware acceleration:** `lsof -p 197317` verified active memory maps to `/usr/lib/x86_64-linux-gnu/libvulkan_intel.so` and open file descriptors on `/dev/dri/renderD128`.
5. **Surface management:** Allocated `/memfd:wayland-cursor-rs` for cursor surface.
6. **Execution:** Rendered without panic or graphics initialization failures.
