# GPUI Kit and GPUI Fast Integration Report

**Date:** 2026-11-23\
**Status:** Integrated & Verified on Linux x86_64 (Wayland / Intel Vulkan)\
**Branch:** `feat/gpui-kit-fast`\
**Crates:** `gpui-kit` v0.10.0, `gpui-fast` v0.1.17\

---

## 1. Upstream Audit

### 1.1 GPUI Kit (`gpui-kit` v0.10.0)
- **Repository:** https://github.com/longbridge/gpui-kit
- **License:** Apache-2.0
- **Purpose:** Comprehensive native UI component framework tailored for GPUI desktop applications.
- **Architecture (3-Layered):**
  1. `gpui-base`: Low-level unstyled behaviors, layout primitives, focus trapping, virtual lists, scrollbars, and accessibility.
  2. `gpui-component`: 75+ polished UI controls (Buttons, Badges, Tabs, Sidebars, StatusBars, Tags, Menus, Dialogs, Dock Layouts, Code Editors).
  3. `gpui-kit`: Top-level facade re-exporting GPUI core, base, components, and embedded assets.
- **GPUI Backend:** Relies on `gpui-pre` v0.3.12 (a Zed GPUI weekly release snapshot), rather than bare `gpui` 0.2.2.
- **Feature Flag `gpui-fast`:** Contains a built-in feature flag `features = ["gpui-fast"]` that transparently switches the underlying GPUI engine from upstream `gpui-pre` to `gpui-fast`.
- **Theming:** Includes a tokenized theme engine (`ThemeRegistry`, `Theme`, `ThemeColor`, `SemanticThemeTokens`) capable of dynamically parsing JSON theme definitions.

### 1.2 GPUI Fast (`gpui-fast` v0.1.17)
- **Repository:** https://github.com/longbridge/gpui-fast
- **License:** Apache-2.0
- **Purpose:** High-performance retained-mode fork of Zed's GPUI.
- **Key Capabilities:**
  1. **Retained Mode:** Reuses unchanged views, layout nodes, and text measurements, reducing main-thread CPU usage by 73% to 88% per frame in high-frequency rendering scenarios.
  2. **Scroll Layers:** GPU tile caching and row reuse during scrolling on Wayland, X11, Windows, and macOS.
  3. **Window Composition:** Multi-surface window composition for embedding native subsurfaces (Wayland subsurfaces, Windows WebView2, macOS WKWebView) beneath GPUI overlays.
- **Value to AveLoop:** High-frequency debugger stepping, trace scrubbing, variable inspection updates, and timeline playback run smoothly without CPU spikes or UI jank (aligning with NFR-007 / NFR-008).

---

## 2. Compatibility Assessment

| Dimension | Assessment | Notes |
|---|---|---|
| **GPUI Revision** | Replaced bare `gpui` 0.2.2 with `gpui-pre` 0.3.12 via `gpui-kit` | `gpui-kit` v0.10.0 packages the matching `gpui-pre` snapshot. Combining bare 0.2.2 with `gpui-kit` causes crate duplication and type incompatibilities. |
| **Rust Toolchain** | Verified with `rustc 1.96.0` / `cargo 1.96.0` | Compiles cleanly with standard `edition = "2021"`. `const { assert!(gpui_kit::GPUI_FAST) }` compile-time check passes. |
| **Linux (x86_64)** | **VERIFIED-FACT** | Compiled and launched on Wayland. Loaded `libvulkan_intel.so`, opened `/dev/dri/renderD128`, created `wayland-cursor-rs` surfaces. |
| **macOS** | **UNVERIFIED** (Architecturally supported) | Handled upstream via Metal / AppKit. Blocked on non-macOS host. |
| **Windows** | **UNVERIFIED** (Architecturally supported) | Handled upstream via Direct3D / Win32. Blocked on non-Windows host. |

---

## 3. Integration Map

```
AveLoop Application
├── domain/ (GPUI-independent, zero UI dependencies)
│   ├── buffer.rs      (Source file buffers, line counts)
│   ├── execution.rs   (ExecutionPoint, ExecutionTrace, StopReason, step navigation)
│   └── session.rs     (SessionState lifecycle)
├── presentation/ (GPUI Kit + GPUI Fast integration)
│   ├── theme.rs       (Night Owl JSON definition, loaded into ThemeRegistry)
│   └── workspace.rs   (Workspace root view, Button, Tag, StatusBar, Scrollable)
└── main.rs            (Application lifecycle, window creation, GPUI_FAST assertion)
```

### Allocation of Responsibilities:
- **GPUI Fast:** Owns all GPU surface rendering, draw event batching, retained layout tree caching, and Wayland compositor interaction.
- **GPUI Kit (`gpui-component`):** Supplies interactive controls (`Button`, `Tag`, `StatusBar`, `ScrollableElement`, `ThemeRegistry`), enforcing keyboard navigation and focus management.
- **AveLoop Presentation Layer:** Retains authority over layout structure, editor presentation, and Night Owl palette tokens.
- **AveLoop Domain Layer:** Strictly isolated from UI framework types, maintaining execution invariants.

---

## 4. Implementation Summary

1. **Cargo Configuration:**
   - Single package `aveloop`, dual library/binary targets (`src/lib.rs`, `src/main.rs`).
   - Pinned `gpui-kit = { version = "0.10.0", features = ["gpui-fast", "component", "assets"] }`.
   - Enabled `gpui-fast` retained-mode backend compile-time assertion in `src/main.rs`.
2. **Night Owl Theming:**
   - Defined `NIGHT_OWL_THEME` JSON containing exact colors from the design reference (`#011627` canvas, `#0b2942` tab/accents, `#7e57c2` purple controls, syntax highlights).
   - Injected into `ThemeRegistry::global_mut(cx)` and activated with `Theme::change(ThemeMode::Dark, None, cx)`.
3. **Workspace View:**
   - Header with AveLoop title, session status tag, and Step Back / Step Forward controls.
   - Code buffer view with active execution line highlighting and scrollbar support.
   - Execution inspector panel displaying step counter, program counter, and stop reason.
   - Bottom status bar reflecting file name, current line, and backend indicators.

---

## 5. Verification Results

| Step | Command | Result | Evidence |
|---|---|---|---|
| **Compilation** | `cargo check` | PASS (0 errors, 0 warnings) | 0.56s check |
| **Unit Tests** | `cargo test` | PASS (4 passed, 0 failed) | `domain_tests` (3) + `presentation_tests` (1) |
| **Linter** | `cargo clippy -- -D warnings` | PASS (clean) | 0 warnings |
| **Formatting** | `cargo fmt --check` | PASS (clean) | Exit 0 |
| **Runtime Hardware Launch** | `./target/debug/aveloop` | PASS | PID 43899 bound to Wayland socket, loaded `libvulkan_intel.so`, opened `/dev/dri/renderD128` |

---

## 6. Limitations and Future Work
- Windows (T006) and macOS (T007) verification remains blocked on host architecture.
- Future tasks will replace the preliminary C++ code viewer with `gpui-kit`'s integrated code editor / syntax highlighter component once Tree-sitter grammars are integrated.
