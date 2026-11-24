# T014 — GPUI Text Primitives and Material 3 Component Surface

- **Task:** T014 (`specs/001-ave-loop-alpha/tasks.md` line 49)
- **Issue:** #14
- **Status:** EVALUATED from the pinned dependency sources. No runtime UI verification was performed; editor stress behaviour and M3 visual conformance remain follow-up work.
- **Date:** 2026-11-24
- **Branch:** `feat/gpui-kit-fast`
- **Governing requirements:** `spec.md` §7.1, FR-002–FR-005, FR-080–FR-092, `plan.md` §3.3 and Phase 2/4 work items

## 1. Pinned inputs

| Crate | Version | Role |
|---|---|---|
| `gpui-kit` | 0.10.0 | facade: GPUI, `base`, `component`, `assets` |
| `gpui-fast` | 0.1.17 | GPUI core/backend used via the `gpui-fast` feature |
| `gpui-base` | 0.10.0 | input/editor engine, base widgets |
| `gpui-component` | 0.10.0 | styled component library (the M3 surface) |
| `tree-sitter-c` / `tree-sitter-cpp` | 0.24.2 / 0.23.4 | grammars already in `Cargo.lock` (T013) |

AveLoop currently enables `gpui-kit` features `gpui-fast`, `component`, `assets` (`Cargo.toml`). The tree-sitter grammar features are **not** enabled yet.

## 2. GPUI text primitives

All APIs below exist in the pinned `gpui-fast` 0.1.17 source (`src/text_system.rs`, `src/text_system/line.rs`, `src/text_system/line_layout.rs`).

| Editor need | Primitive |
|---|---|
| Shape a single line with style runs | `TextSystem::shape_line` (`text_system.rs:669`) |
| Shape multi-line text | `TextSystem::shape_text` (`text_system.rs:713`) |
| Lay out a line to a width | `TextSystem::layout_line` (`text_system.rs:849`) |
| Draw a shaped line, background, underline | `ShapedLine::paint`, `paint_background`, `paint_with_underline_handler` (`line.rs:109`, `:161`, `:130`) |
| Caret iteration across a shaped line | `ShapedLineCursor` (`line.rs:269`) |
| Soft-wrapped line and its layout | `WrappedLine` (`line.rs:450`), `WrappedLineLayout` (`line_layout.rs:298`) |
| Point → text index (click/drag) | `WrappedLineLayout::index_for_position` (`line_layout.rs:369`), `ShapedLineLayout::index_for_x` (`line_layout.rs:63`) |
| Text index → point (caret placement) | `WrappedLineLayout::position_for_index` (`line_layout.rs:449`) |
| Syntax/colour spans | `TextRun` (`text_system.rs:1071`) |
| Font metrics for gutter and caret geometry | `advance`, `em_width`, `ch_width`, `ascent`, `descent`, `line_gap`, `cap_height`, `x_height` (`text_system.rs:400`–`480`, `:1221`–`1246`) |
| Font fallback and OpenType features | `text_system/font_fallbacks.rs`, `font_features.rs`; `add_fonts`, `resolve_font` |
| Grapheme-aware segmentation | `Grapheme` (`text_system.rs:85`) |
| IME composition | `Window::set_input_handler` (`window.rs:5796`) with `InputHandler` / `PlatformInputHandler` |

Conclusion: the primitives needed for a source editor — shaping, run-based highlighting, wrapped layout, bidirectional hit-testing, caret geometry, IME, and grapheme handling — are present. AveLoop does not need to implement text layout itself.

## 3. Reusable editor widget

`gpui-base` ships an editor engine and `gpui-component` exposes it as a styled widget:

- `gpui_base::input::{EditorState, Editor}`, re-exported as `gpui_kit::component::input::{EditorState, Editor}`.
- Editor submodules: `decorations.rs` (`GutterDecoration`, `GutterDecorationCollection`, range/overview-ruler decorations), `diagnostics.rs` (`Diagnostic`), `display_map/` (`DisplayMap`, `FoldRange`), `highlighting.rs`, `indent.rs`, `language.rs`, `lsp/`, `search.rs`, `auto_close.rs`.
- `gpui-component/src/highlighter/` provides the tree-sitter highlighter (`highlighter.rs`, `languages.rs`) and a syntect fallback. `languages.rs` maps `c` → `tree_sitter_c::LANGUAGE` + `HIGHLIGHT_QUERY` and `cpp`/`c++` → `tree_sitter_cpp`.

This covers FR-002 (edit/save), FR-003 (cursor, selection, undo/redo, clipboard), FR-004 (line numbers, C/C++ highlighting), FR-005 (dirty marker via state), and diagnostics display.

Risks recorded, not hidden:

- The widget is young (0.10.x) and was not stress-tested here with large files, long lines, or IME composition.
- Editor behaviour must be verified on all three OSes before the editor task is considered done; Linux-only success is not enough.
- LSP support exists but is out of Alpha scope (`spec.md` §8); it must not leak into the Alpha contract.

## 4. Required Material 3 component surface

Required surface derived from `spec.md` US-08, FR-002–FR-092, and the Alpha workspace description. Availability checked against `gpui-component` 0.10.0 module exports.

| Required surface | Available component(s) | Status |
|---|---|---|
| Window chrome, title bar, border | `title_bar`, `window_border` | present |
| App toolbar / transport bar | `toolbar`, `button` (Default, Primary, Secondary, Danger, Info, Success, Warning, Ghost, Link, Text, Custom) | present |
| Project sidebar | `sidebar` | present |
| Status bar (toolchain, session, capabilities) | `status_bar` | present |
| Resizable panels and splits | `resizable`, `dock` | present |
| Open-file tabs | `tab` | present |
| Code editor | `input::Editor` + `EditorState` | present, needs grammar features |
| Syntax highlighting (C17/C++20) | `highlighter` (tree-sitter) | present, features off by default |
| Find in file | `input::search` | present |
| Console / build output | `text`, `label`, `scroll`, `virtual_list` | compose (no dedicated log view) |
| Progress, spinners, skeletons | `progress`, `spinner`, `skeleton`, `shimmer` | present |
| Timeline scrubber | `slider` | present |
| Trace list / event list | `list`, `virtual_list`, `table` | present |
| Call stack and variable tree | `tree`, `table`, `accordion`, `collapsible`, `description_list`, `group_box` | present |
| Pointer/relationship graph | none — product-specific | build on `canvas` + text primitives |
| Timeline visualization | none — product-specific | build on `div`/`canvas` + text primitives |
| Capability/state badges and disclosure | `badge`, `tag`, `tooltip`, `hover_card`, `alert`, `notification` | present |
| Dialogs and sheets | `dialog`, `sheet` | present |
| Forms and inputs | `form`, `input`, `textarea`, `select`, `combobox`, `checkbox`, `switch`, `radio`, `stepper`, `setting`, `searchable_list` | present |
| Menus and context menus | `menu`, `native_menu` | present |
| Command palette | `command` | present |
| Keyboard shortcut hints | `kbd` | present |
| Empty states | `empty` | present |
| Theme tokens, motion, typography | `theme` (`semantic_tokens`, `MotionTokens`, mono font handling), `ThemeRegistry` | present and already used by `src/presentation/theme.rs` |

## 5. Gaps and decisions

1. **Reuse, do not fork.** Build on `gpui-kit`/`gpui-component` and add AveLoop-specific presentation in `src/presentation/`. No fork, mirror, or vendored patch of GPUI/GPUI Kit is created (`plan.md` line 505). Any upstream defect is reported upstream first.
2. **Two product-specific visualizations are unavoidable:** the pointer-relationship graph and the execution timeline. They are AveLoop identity, not generic design-system parts, and are built from `canvas`/`div` plus the text primitives in §2. Text primitives are sufficient for labels, offsets, and state annotations.
3. **Enable the grammars explicitly.** `gpui-kit` exposes `tree-sitter-c` and `tree-sitter-cpp` passthrough features; both crates are already in `Cargo.lock`. The editor task must enable them; until then, highlighting for C/C++ is unavailable despite the widget supporting it.
4. **M3 naming is not 1:1.** The component library exposes semantic variants (`Primary`, `Secondary`, `Ghost`, `Text`, `Custom`) rather than the exact M3 filled/tonal/outlined/elevated set. A small mapping table plus a visual audit is required before claiming "Material 3 behavior" per US-08.
5. **No dedicated console/log widget.** The build-output area composes `virtual_list` + monospace `text`; acceptable, but it must be verified with large compiler output.
6. **Editor maturity risk.** The editor is the largest reuse bet in Alpha. Before the editor task closes, it needs: large-file scroll performance, IME composition on each OS, multi-cursor not required, clipboard round-trip, and dirty-state integration with the domain `SourceBuffer`.

## 6. Verification status

| Item | Status |
|---|---|
| Text primitives present and sufficient for editor scope | EVALUATED (pinned source) |
| Editor widget present with gutter, diagnostics, folding, search, tree-sitter languages | EVALUATED (pinned source) |
| M3 component surface coverage | EVALUATED (module inventory) |
| Component rendering, M3 visual conformance, editor stress/IME behaviour | NOT VERIFIED — runtime UI work |
| C/C++ highlighting with features enabled | NOT VERIFIED — features currently disabled |

This task is documentation-only by design; it created no fork and changed no dependency versions.

## 7. Reproduce

```sh
K=~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f
grep -n "pub fn shape_line\|pub fn shape_text\|pub fn layout_line" $K/gpui-fast-0.1.17/src/text_system.rs
grep -n "index_for_position\|position_for_index" $K/gpui-fast-0.1.17/src/text_system/line_layout.rs
grep -n "pub struct EditorState\|pub use editor" $K/gpui-base-0.10.0/src/input/mod.rs
grep -n "tree_sitter_c::\|tree_sitter_cpp::" $K/gpui-component-0.10.0/src/highlighter/languages.rs
grep -n "pub mod " $K/gpui-component-0.10.0/src/lib.rs
grep -n "tree-sitter-c\|tree-sitter-cpp" $K/gpui-kit-0.10.0/Cargo.toml
```
