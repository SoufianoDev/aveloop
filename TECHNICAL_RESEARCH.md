# AveLoop — TECHNICAL_RESEARCH.md

Research-first record for the AveLoop Night Owl IDE prototype. Completed **before**
implementation. Only sources actually opened are listed; nothing below is invented.

## 1. Official sources inspected

- Night Owl theme JSON (raw file via GitHub blob view):
  `https://github.com/sdras/night-owl-vscode-theme/blob/main/themes/Night%20Owl-color-theme.json`
  — full `colors` map + `tokenColors` array read (editor bg/fg, selection,
  line highlight, cursor, line numbers, syntax scopes, error/warning, sidebar,
  status bar, buttons, inputs). Confirmed repo path `themes/` still current.
- Night Owl README / marketplace / VS Code theme docs / CSS-Tricks background:
  listed in prompt; palette ground truth taken from the JSON above, not screenshots.
- Python Tutor C/C++ visualizer article:
  `https://pythontutor.com/articles/c-cpp-visualizer.html` — read in full. Covers
  memory regions (globals/stack/heap/read-only), Valgrind-tracked uninitialized
  memory (`?`), interior pointers, address/byte-level toggles, out-of-bounds
  skull states, type punning, structs/unions, parameter passing, `scanf` support.
- Python Tutor developer overview:
  `https://github.com/pgbovine/OnlinePythonTutor/blob/master/v3/docs/developer-overview.md`
  — referenced for trace/render separation (trace generation vs. frontend
  rendering). Live `visualize.html`/`c.html` interaction was not drivable from
  this environment, so behavioral claims rest on the article above, not on
  live testing (stated as a limitation in §19).
- Material Web intro: `.../material-web/blob/main/docs/intro.md` — read. Confirms
  web-component model, token-based theming (`--md-sys-color-*` CSS custom
  properties), drop-in `<md-*>` replacements for native elements.
- Material Design 3: `m3.material.io/foundations/accessible-design` fetch was
  JS-blocked; M3 knowledge applied from the public spec structure instead
  (color roles, type scale, shape, state layers 8/12%, focus indicators,
  48/40px targets, `prefers-reduced-motion`). No M3 claim below depends on the
  blocked page.
- CodeMirror: `https://codemirror.net/` — read. Confirms CodeMirror 6 modular
  packages, Lezer parsers, `lang-cpp` coverage, a11y/mobile support, MIT license.
- Editor comparison evidence: `codemirror.net/examples/bundle/` (bundle/size
  guidance), evcc PR `#30855` (Monaco ~932 KB → CodeMirror ~101 KB gzipped),
  faun.dev + pistack.xyz + toolkit.best comparisons (Aug 2026 data: Monaco
  ~2.5–2.9 MB / workers required / no mobile support; CM6 ~150 KB, no workers,
  tree-shakeable; CodeMirror 5 archived April 2026).
- Material Symbols: Google Fonts `Material Symbols Outlined` variable font;
  icon names verified against the fonts.google.com catalog naming
  (`play_arrow`, `pause`, `skip_next`, `skip_previous`, `first_page`,
  `last_page`, `restart_alt`, `terminal`, `memory`, `data_object`,
  `account_tree`, `speed`, `close`, `unfold_less`, `unfold_more`).

## 2. Python Tutor source files and responsibilities (from article + repo layout)

| File / area | Responsibility |
|---|---|
| C backend (Valgrind-based tracer) | Executes code, emits one JSON snapshot per step; tracks init-bits so uninitialized bytes render as `?`; reports OOB |
| Trace JSON (`line`, `stack`, `heap`, `globals`, `stdout`, `event`) | Single authoritative state per step; frontend never keeps parallel copies |
| `visualize.html` frontend render | Draws globals/stack/heap tables, stack frames under caller, arrows pointer→target, slider/buttons/line highlight |
| Arrow/legend layer | Arrows computed from rendered DOM geometry; interior-element targets (e.g. `arr[1]`) anchor to the element cell, not the whole object |

## 3. Python Tutor interactions investigated

Via the C/C++ article (live stepping not available here): assignment fill-in of
`?`, call pushes frame / return pops it, pointer→array-element arrows incl.
interior targets, `malloc` adds heap object / `free` removes it, NULL removes
arrow, multi-pointer aliasing to one object, address/byte toggles, OOB skull,
backward slider restores prior snapshot exactly, `printf` output accumulates and
rewinds. AveLoop reproduces the snapshot + slider + regions + arrows subset;
Valgrind accuracy, OOB skull, byte view are documented non-goals (§19).

## 4. Proposed execution-state model

`TRACE = { meta, steps[] }`; each step is a full snapshot:
`{ i, line, event, stdout, frames[{id, func, params, vars[{id,name,kind,type,
value,ref,state}]}], heap[{id,label,kind,cells[],state}], pointers[{src, dst}] }`.
Stable ids (`f-main`, `v-p`, `h-malloc1`, `arr[1]`). UI is a pure function
`render(step)`; transport only changes the index. `ref`/`pointers` separate the
variable from its target (never merged). Extensible: a future engine adapter
only needs to emit `steps[]`; add `lang`/`arch` fields per language.

## 5. Pointer-arrow rendering strategy

SVG overlay (`<svg>` absolutely positioned over the workspace). One canonical,
pre-designed arrow spec instantiated per relationship — never restyled
structurally: 1.5px teal (`--ave-ptr`) path, round caps, triangular head from a
single marker def. Endpoints from `getBoundingClientRect()` of origin dot
(`[data-ptr-src]`, path starts at the dot center so the joint is seamless) and
target (`[data-ptr-target]`); the tip lands 1–2px inside the target border so
the head visibly touches instead of floating short, with a horizontal tangent
at both ends. UX rule (user feedback): arrows whose target lies left of their origin slant
right-to-left toward the element (`flip:true`, control point measured left of
center) instead of looping right through the value text; verified geometrically
in-page before adopting. Shape catalog (user-designed): standard library arcs
for element targets and cross-column jumps, plus a dedicated hook shape for
whole-object targets above their source (e.g. step 12 `ppt → pt`) — sweeps out
into the inter-column gap (clear of heap content by measurement), curls back
with the head pointing up-left into the element. One fixed routing template: capped S-curve for cross-column
jumps, small local loops in-frame; targets left of their source anchor to the
target's value side; curves sharing a target get a per-index spread. Orange
(`--ave-ptr2`, second marker def) appears ONLY on hover — via CSS on the path
itself plus JS row→path linking — changing appearance alone, never structure.
Recomputed on: step change, `ResizeObserver` (workspace +
panels), scroll (capture phase), inspector tab switch, resize. Batched via `requestAnimationFrame`; skipped when
geometry key unchanged. NULL/freed → no path (previous path removed, never
stale). Interior targets anchor to the indexed cell. Color: Night Owl
`#7fdbca`/`#ffcb8b` pair with label + direction (not color-only). Honors
`prefers-reduced-motion` (no arrow animation, instant geometry).

## 6–8. Night Owl files, colors, token mapping

Inspected: `themes/Night Owl-color-theme.json` (1859 lines). Ground-truth values:

| Role | Original |
|---|---|
| editor bg / fg | `#011627` / `#d6deeb` |
| comment (italic) | `#637777` |
| keyword / storage (italic) | `#c792ea` |
| string | `#ecc48d` |
| number / escape | `#F78C6C` |
| function name | `#82AAFF` |
| variable / support type | `#c5e478` |
| operator / keyword.control | `#7fdbca` |
| param | `#7fdbca` / `#baebe2` instance |
| type / class | `#ffcb8b` / `#c5e478` |
| constant / support fn | `#82AAFF` |
| selection / line hl / cursor | `#1d3b53` / `#28707d29` / `#80a4c2` |
| line nr / active | `#4b6479` / `#C5E4FD` |
| indent guide | `#5e81ce52` |
| error / warning | `#EF5350` / `#b39554` |
| sidebar/activity/status bg | `#011627`, fg `#5f7e97`/`#89a4bb` |
| button | `#7e57c2cc`→`#7e57c2`, fg `#ffffffcc` |
| input bg/border | `#0b253a` / `#5f7e97` |
| border/divider | `#262A39`–`#122d42` |

AveLoop mapping: original values → `--owl-*` custom properties verbatim;
AveLoop-only semantics → `--ave-*` (`--ave-exec-line`, `--ave-changed`,
`--ave-ptr`, `--ave-ptr-2`, `--ave-freed`, `--ave-ok`). Syntax colors are never
reassigned to match buttons; UI accents stay in the violet/blue family already
present in the theme. Documented deviation: `tags.preprocessorName` does not
exist in `@lezer/highlight@1.2.2` (verified in-page), so `#include` lines fall
back to default foreground with string-colored paths — no invented tag mapping.

## 9. Material Design 3 principles selected

State layers (hover +8%, focus ring, pressed), 4dp spacing scale, type scale
(title/body/label + mono for code), restrained shape (6–12px, no pill-everything),
icon buttons 40px targets, tabs with indicator, slider with value readout,
tooltips, visible focus on all controls, `prefers-reduced-motion` support.
M3 informs behavior/spacing/a11y; it never supplies the palette. Timeline is
a native-built M3 discrete slider per the Sliders guidelines (m3 spec pages
are JS-gated; anatomy/behavior confirmed via search excerpts + Android
`Slider.md` + Flutter M3 slider docs): two-tone track with handle gap,
vertical bar handle that narrows when pressed, 4dp end stop indicator,
per-stop ticks, rounded-rect value bubble on drag/focus, snap-to-stop,
`role=slider` with full keyboard (arrows/Home/End) and synced value text.

## 10. Material Web: current status and suitability → OPTION A FOR THE SLIDER (revised per user request)

Evaluated via `docs/intro.md`, `docs/components/slider.md` (full API + token
table), and the raw `tokens/_md-comp-slider.scss` (supported-token allowlist).
`@material/web@2.5.0` (Apache-2.0) serves a working ESM build of
`slider/slider.js` from esm.sh; `<md-slider min max value step ticks labeled>`
registers cleanly in a single-file page, exposes composed `input` events,
built-in keyboard + ARIA, and documents every theming token used
(`--md-slider-active/inactive-track-color`, `-handle-color`,
`-hover/pressed/focus-handle-color`, `-hover/pressed-state-layer-color`,
`-label-container/text-color`, `-with-tick-marks-active/inactive-container-color`
— no invented tokens). Night Owl is preserved purely through those tokens.
Decision revised: **official `<md-slider>` for the timeline**; the previously
built native M3 slider is retained verbatim as the offline fallback (both stay
synced, verified in headless Chrome). Handle rendered as the M3 Expressive
linear bar (not the default circle) purely through documented tokens —
`--md-slider-handle-width:4px`, `-handle-height:30px`, `-handle-shape:2px`,
plus 16px active/inactive track heights — matching the official slider anatomy
reference. No other Material Web components are
imported — restraint, not wholesale adoption.

Evaluated via `docs/intro.md` + repo knowledge: official M3 web components,
token-themed, framework-agnostic. Original concern (npm/bundler weight, default
scheme fighting Night Owl, single-file fit) was resolved per-component:
`slider/slider.js` loads standalone from esm.sh and themes cleanly through
the documented tokens above, so the earlier Option-B-only stance is superseded
for this one component. Rest of the chrome (buttons/tabs/tooltips) remains
hand-built on Night Owl tokens — zero reason to import the whole library.

## 11. Icon set selected

**Material Symbols Outlined** (variable font, single consistent style) loaded
from Google Fonts CDN with `font-display: swap`; ligature names listed in §1.
Every icon-only button has `aria-label` + tooltip. No emojis/Unicode glyphs.
Offline fallback: text labels remain (ligature text hidden only when font loads).

## 12–13. Editor options and selection → CodeMirror 6

| Option | Status | Verdict |
|---|---|---|
| CodeMirror 5 | Archived/read-only April 2026 | **Excluded** (per prompt + maintenance) |
| CodeMirror 6 | Maintained, modular, ~150 KB w/ 1 lang, no workers, mobile+a11y, MIT, `lang-cpp` covers C, ESM CDN (`esm.sh`) works single-file | **Selected** |
| Monaco | Maintained but ~2.5–2.9 MB, requires workers + bundler config, desktop-first/no mobile, own theming API | Rejected: unsuitable for single-file deliverable |
| Ace | Stable but legacy model, many stale modes | Rejected |

Night Owl editor theme implemented via CM6 `HighlightStyle` + `EditorView.theme`
using the §8 token map; `indentWithTab` so Tab inserts a literal tab; whitespace
preserved (no reformatting); exec-line via line decoration; edit-vs-trace
mismatch shows a stale-trace banner (prototype runs a predefined trace and
never claims to execute user code).

## 14. Interface-design practices researched

Product-specific IDE layout (editor + memory canvas as co-primary, transport as
slim rail, inspector as tabbed side panel); density over card grids; one accent
family; alignment/spacing tokens; mono for values; change-highlight (`--ave-changed`)
+ labels rather than rainbow variables; keyboard-first transport; empty/focus/
disabled states; iterative review at multiple steps/viewports (performed in §22
checklist of prompt).

## 15. Poor-UI patterns deliberately avoided

No gradient/glass/neon/blob decoration; no card-grid dashboard; no nested cards
(stack frames/heap objects are flat regions, not cards-in-cards); no decorative
charts/stats/badges; compact 13–15px headings; restrained 6–10px radii; every
control wired (no dead buttons); no animation on every interaction (only
play stepping + focus transitions); no second palette competing with Night Owl.

## 16. Performance considerations

Single render-per-step with targeted DOM updates; geometry cached + rAF-batched;
no continuous redraw timers (single playback timer, guarded against duplicates);
CM6 incremental parsing; zero framework/dependency weight besides CM6 ESM +
icon font; layout-stable regions (reserved heap/stack areas minimize shift).

## 17. Accessibility considerations

Full keyboard transport (←/→/Home/End/Space, `?`-documented in tooltip/status);
visible `:focus-visible` rings; icon buttons labelled; contrast checked against
`#011627` (body `#d6deeb` ~14:1; muted `#89a4bb` ~7:1; pointers reinforced with
labels/direction); reduced-motion disables stepping animation + arrow
transitions; editor keeps screen-reader/keyboard behavior of CM6; no
color-only pointer meaning.

## 18. Licensing and dependencies

CodeMirror 6 (MIT), Lezer parsers (MIT), perfect-arrows 0.3.7 (MIT,
dependency-free, geometry-only — no DOM opinions, so Night Owl tokens,
markers, and hover styling stay ours), Material Web 2.5.0 slider component
(Apache-2.0, single-component import), Material Symbols (Apache 2.0, font),
Night Owl theme values (MIT, sdras/night-owl-vscode-theme — token values reused
as data, attribution in README comment of `index.html`). No copied Python Tutor
code (behavioral reimplementation only; OPT license therefore not triggered).
CDN: `esm.sh` (CM6) + Google Fonts (symbols). App degrades to a built-in
plain-textarea editor if CDN is unreachable.

## 19. Technical limitations

Single-file + CDN requires network for CM6/icons (fallback editor included).
Trace is predefined/symbolic: addresses shown as illustrative (`0xA1xx` per region)
and labeled SIMULATED; no real C execution, no Valgrind accuracy, no OOB
detection, no byte-level view, no scanf interactivity. Live pythontutor stepping
not performed from this environment — behavioral parity claims rest on the
official article. Explicit dangling-pointer rendering out of scope (trace nulls
`r` after `free`); freed block shown as a labelled tombstone with arrows removed.

## 20. Provisional decisions (plus post-review UI refinements)

Arrow-geometry library (added per user request after hand-rolled curves proved
hard to perfect): evaluated `perfect-arrows` 0.3.7 (2.8k★, MIT, zero deps,
self-contained ESM, exports `getArrow` returning start/control/end points —
verified via registry + esm.sh build inspection), LeaderLine 1.0.8 (rejected:
manages its own overlay/observers, fighting the controlled per-step render;
heavier concept), jsPlumb (rejected: full diagramming toolkit, overkill for
~6 arrows). Selected perfect-arrows, pinned via esm.sh importmap alongside the
editor packages; only the curve math comes from the lib — overlay, endpoints,
markers, colors, hover all remain ours, and the previous cubic is retained as
the offline fallback, so the visual contract is identical either way.

CM6 via `esm.sh` pinned versions + `?external=` dedupe (verified required:
without externalized `@codemirror/state` etc., esm.sh serves per-package
copies and `EditorState.create` throws "Unrecognized extension value";
verified fixed in headless Chrome). `indentWithTab` is a KeyBinding object,
not an Extension — it must go inside `keymap.of()` (found by bisection after
misreading the generic error hint). `bracketMatching()` added from
`@codemirror/language`. Illustrative address scheme (`0xA1xx`
stack / `0xB2xx` heap) kept clearly fake; single demo program
(pointer walk + `malloc` + struct + call) — replaceable by swapping `TRACE`
without touching renderers.

Post-review refinements (user feedback, Python Tutor screenshot comparison):
exec-line left bar removed as an AI-design tell — current line now uses
background tint plus a gutter arrow marker (`GutterMarker`), matching real
editor/PT conventions. Pointer arrows start at visible origin dots on the
variable row, use tight local loops in-frame and wide curves only for
stack→heap jumps, thinner strokes and smaller heads. Prominent
SIMULATED/subtitle/statusbar badges removed per user request; honesty is
preserved structurally instead — Trace-tab footer notes the demo trace and
illustrative addresses, and the UI never claims to compile or run user code.
