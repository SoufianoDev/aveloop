
# AveLoop — Research-First Design and Implementation of a Night Owl IDE with Python Tutor-Level Execution Visualization and High-Quality Material Design 3

## # OBJECTIVE

Design and implement AveLoop from scratch as a polished, functional, developer-oriented visual programming IDE.

The application must combine:

1. A professional code-editing experience.
2. Night Owl as AveLoop's official default theme and primary visual identity.
3. Accurate implementation of the original Night Owl color palette and syntax-color relationships.
4. Material Design 3 principles for component design, interaction states, accessibility, layout, and typography.
5. A research-based decision about Google's official Material Web component library.
6. Google's official Material Symbols or Material Icons for interface icons.
7. Python Tutor-level fidelity in execution visualization and interaction behavior.
8. Accurate pointer arrows connecting real variables to their corresponding memory targets.
9. A consistent execution-state model synchronizing the source code, stack, variables, heap, pointers, and console.
10. A deliberately designed interface that avoids generic AI-generated UI patterns.
11. A responsive, performant implementation with a maintainable architecture.
12. An extensible foundation for future execution engines and programming languages.

**This is a new interface implementation, not a redesign of an existing, finished interface.**

There is no prebuilt UI that you must preserve or modify. You are expected to design the interface from scratch.

However, inspect the available project files, development environment, and technical constraints before choosing the implementation architecture. Existing project infrastructure may contain useful constraints or requirements even though the interface itself must be designed from the ground up.

Do not assume that the best implementation is the one that produces the most code, uses the most libraries, or resembles a fashionable dashboard.

The objective is to build a credible developer tool whose visual design, interactions, execution model, and implementation quality are all deliberate.

The primary references are:

- **Theme:** Original Night Owl by Sarah Drasner.
- **Design system:** Official Material Design 3 guidance.
- **Component library:** Official Material Web, subject to a technical suitability assessment.
- **Icons:** Google's official Material Symbols and Material Icons.
- **Execution visualization:** Python Tutor.
- **Editor:** The best currently suitable code-editor solution, selected after research.
- **Visual quality:** Product-specific interface design, supported by design research, real-world references, usability principles, and iterative verification.

Do not start implementation by immediately generating HTML and CSS.

Research first, establish the design direction, define the architecture, and then implement and verify the result.

---

## # SOURCE OF TRUTH AND DECISION PRIORITY

Different references have different responsibilities.

Use the following hierarchy:

**Night Owl**
- Determines the primary color identity.
- Defines the intended editor background, foreground, syntax colors, and recognizable theme character.
- Guides the surrounding IDE palette.

**Material Design 3**
- Informs component design.
- Guides spacing, typography hierarchy, interaction states, accessibility, shape, and motion.
- Does not automatically replace the Night Owl palette.

**Python Tutor**
- Defines the primary behavioral reference for execution visualization.
- Informs stack, heap, variable, pointer, source-line, and navigation behavior.

**Official editor documentation**
- Determines the capabilities, configuration, maintenance status, and limitations of the selected editor.

**Official Material Symbols or Material Icons**
- Determines the available icon assets, names, and implementation methods.

**Official Material Web documentation**
- Determines whether its components are suitable for this implementation.

**Research into interface quality and frontend design**
- Informs the design process, quality criteria, usability, visual hierarchy, and rejection of generic interface patterns.

**AveLoop's product requirements**
- Determine which features must exist and how they must work.

Do not let one reference override another reference's responsibilities without a documented reason.

Do not treat screenshots as sufficient evidence of implementation details.

Do not assume that a visually similar interface has equivalent behavior.

Do not invent technical capabilities, research findings, or test results.

---

## # MANDATORY REQUIREMENT: RESEARCH BEFORE IMPLEMENTATION

Research must be performed before implementing the interface.

Do not immediately generate the complete application and then attempt to justify the design afterward.

First, investigate the official references, current libraries, source implementations, technical constraints, and relevant design practices.

Research must cover all of the following:

1. Night Owl.
2. Python Tutor.
3. Material Design 3.
4. Material Web.
5. Material Symbols and Material Icons.
6. Current code-editor options.
7. Techniques for building high-quality, product-specific developer interfaces.
8. Common causes of poor AI-generated UI and methods for preventing them.
9. Browser performance, rendering, accessibility, and interaction reliability.

### Research rules

- Prioritize official documentation and original source repositories.
- Open relevant documentation and inspect actual source files.
- Search for real implementation examples when official documentation does not answer a question.
- Examine community discussions when they reveal useful problems or implementation details.
- Verify important claims against primary sources.
- Check current repository status, releases, maintenance, and compatibility.
- Do not assume that a familiar library is still maintained.
- Do not assume that a fashionable design practice is appropriate for an IDE.
- Do not rely exclusively on search-result snippets.
- Do not invent research results.
- Do not claim to have tested an interface if you only inspected its source.
- Do not claim to have inspected source code that you did not actually open.
- Record the important decisions and the evidence supporting them.

The purpose of this research is to make informed decisions before implementation, not to produce a long document that does not affect the final result.

---

## # 1. NIGHT OWL — OFFICIAL DEFAULT THEME

Night Owl is the **official default theme of AveLoop**.

This is a mandatory product requirement.

It is not an optional skin, a temporary reference, or a suggestion to use a vaguely similar dark palette.

The entire IDE must be designed around Night Owl's visual identity.

### Official sources

Original Night Owl repository:

https://github.com/sdras/night-owl-vscode-theme

Original README:

https://github.com/sdras/night-owl-vscode-theme/blob/main/README.md

Theme source directory:

https://github.com/sdras/night-owl-vscode-theme/tree/main/themes

Original Night Owl theme definition:

https://github.com/sdras/night-owl-vscode-theme/blob/main/themes/Night%20Owl-color-theme.json

Night Owl Light theme:

https://github.com/sdras/night-owl-vscode-theme/blob/main/themes/Night%20Owl-Light-color-theme.json

Official VS Code Marketplace listing:

https://marketplace.visualstudio.com/items?itemName=sdras.night-owl

VS Code color-theme documentation:

https://code.visualstudio.com/docs/configure/themes

Theme creation background:

https://css-tricks.com/creating-a-vs-code-theme/

Inspect the current repository and locate the original theme files.

If a file has moved, use the current repository structure rather than assuming an old path is still correct.

### 1.1. Inspect the original theme definition

Read the actual Night Owl theme JSON and investigate its color definitions.

Identify the original values and relationships for:

- Editor background.
- Editor foreground.
- Default text.
- Comments.
- Keywords.
- Strings.
- Numeric literals.
- Functions.
- Variables.
- Constants.
- Types.
- Operators.
- Punctuation.
- Parameters.
- Properties.
- Preprocessor directives, where applicable.
- Selection background.
- Cursor.
- Current-line highlighting.
- Line numbers.
- Active line numbers.
- Indentation guides.
- Bracket matching.
- Search highlights.
- Error and warning decorations.
- Sidebar.
- Activity bar.
- Panels.
- Inputs.
- Buttons.
- Borders.
- Dividers.
- Focus indicators.
- Disabled states.
- Selected states.
- Hover states.

Inspect token scopes rather than assuming that all code tokens use only a handful of colors.

Where the original theme distinguishes different token types, preserve those distinctions when the selected editor supports them.

Do not reconstruct the palette from memory or copy colors from an arbitrary screenshot.

### 1.2. Apply Night Owl to the entire IDE

Night Owl must influence the visual appearance of:

- Application header.
- Navigation.
- Toolbars.
- Code editor.
- Execution workspace.
- Stack frames.
- Variables.
- Heap and memory regions.
- Arrays and structures.
- Pointer arrows.
- Inspector panels.
- Playback controls.
- Console.
- Status bar.
- Menus.
- Tooltips.
- Dialogs, if needed.
- Inputs.
- Buttons.
- Scrollbars.
- Borders and dividers.
- Focus and selection states.

Do not use Night Owl only for syntax highlighting.

The editor and surrounding interface must look like parts of the same application.

### 1.3. Preserve the original Night Owl identity

Use the original Night Owl source as the starting point.

Do not substitute:

- Material Dark.
- Material Palenight.
- Dracula.
- Nord.
- One Dark.
- An arbitrary blue-gray palette.
- A generic neon-cyan theme.

Do not invent a new palette and label it Night Owl.

Do not let a component library silently replace the original theme with its own default colors.

Night Owl's syntax colors must remain faithful to the original theme even when they include colors that are not used as dominant application accents.

Distinguish syntax highlighting from the surrounding UI palette.

Do not arbitrarily recolor Night Owl syntax tokens just to make every color match the buttons or navigation.

### 1.4. Create a centralized theme-token architecture

Create a clear theme layer using CSS custom properties or an equivalent centralized mechanism.

Separate:

**Original Night Owl values**
- Values derived directly from the original theme.

**AveLoop-specific semantic tokens**
- Additional colors required for execution visualization, memory states, controls, and component interactions.

Organize tokens for:

- Application background.
- Editor background and foreground.
- Primary surfaces.
- Secondary surfaces.
- Sidebar.
- Panel backgrounds.
- Borders.
- Dividers.
- Primary text.
- Secondary text.
- Disabled text.
- Selection.
- Focus.
- Active execution line.
- Pointer arrows.
- Selected variables.
- Changed values.
- Stack frames.
- Heap objects.
- Array elements.
- Console output.
- Warnings.
- Errors.

The exact token names are implementation decisions.

Avoid scattering arbitrary hexadecimal colors throughout the stylesheet.

Avoid defining multiple competing palettes for the same component.

### 1.5. Night Owl and Material Design 3

Night Owl determines the palette.

Material Design 3 informs the design and behavior of the components.

Apply Material 3 principles without allowing its default color scheme to replace Night Owl.

If Material Web is used, customize supported component tokens and properties.

Prefer documented customization mechanisms over fragile CSS overrides.

Do not use a default Material theme provider that overwrites the application's Night Owl colors.

### 1.6. Verify Night Owl fidelity

Verify that:

1. The editor background and foreground match the intended Night Owl palette.
2. Supported syntax tokens use the correct Night Owl colors.
3. Selection and current-line highlighting remain readable.
4. The surrounding application is consistent with the same theme.
5. Material components do not introduce an unrelated palette.
6. Pointer arrows remain visible.
7. Memory regions remain distinguishable.
8. Focus indicators are visible.
9. Error and warning states remain understandable.
10. The application is recognizable as a Night Owl-themed IDE.

Document significant deviations from the original theme.

---

## # 2. PYTHON TUTOR — BEHAVIORAL RESEARCH

Python Tutor is the primary reference for execution visualization.

The objective is not to copy its exact appearance.

The objective is to understand and reproduce the behavior that makes execution state understandable.

### Official sources

Python Tutor:

https://pythontutor.com/

Python visualizer:

https://pythontutor.com/visualize.html

C visualizer:

https://pythontutor.com/c.html

C++ visualizer:

https://pythontutor.com/cpp.html

C/C++ visualization article:

https://pythontutor.com/articles/c-cpp-visualizer.html

Python visualization article:

https://pythontutor.com/articles/python-visualizer.html

Official source repository:

https://github.com/pgbovine/OnlinePythonTutor

Developer architecture overview:

https://github.com/pgbovine/OnlinePythonTutor/blob/master/v3/docs/developer-overview.md

Inspect the current repository and identify the files responsible for the relevant features.

Investigate:

- Trace generation.
- Trace data structures.
- Source-line highlighting.
- Step navigation.
- Stack frames.
- Variables and values.
- Heap and memory representation.
- Arrays and individual elements.
- Structs and fields.
- Pointer relationships.
- Arrow generation.
- Function calls and returns.
- Variable initialization.
- Memory allocation and deallocation.
- Address representation.
- Invalid memory states, where supported.
- Layout recalculation.
- Animation.
- Event handling.
- State restoration.

Do not assume that all of these features are implemented identically across every language supported by Python Tutor.

Verify the behavior relevant to C.

### 2.1. Inspect the live application

Interact with the original application when accessible.

Create small test programs that demonstrate:

1. Variable assignment.
2. Variable modification.
3. Function calls.
4. Function returns.
5. Arrays.
6. Pointers to array elements.
7. Pointers to interior array elements.
8. Pointer reassignment.
9. Pointer arithmetic.
10. Multiple pointers referencing the same object.
11. NULL pointers.
12. Dynamic allocation.
13. Memory deallocation.
14. Structs.
15. Pointers to structs.
16. Backward navigation.
17. Forward navigation.
18. Direct selection of an execution step.

Investigate invalid, uninitialized, or out-of-bounds states where the original visualizer supports them.

For each interaction, observe:

- Which elements appear.
- Which elements disappear.
- Which values change.
- How stack frames change.
- How memory objects are represented.
- Where arrows originate and terminate.
- How the visualization responds to backward navigation.
- Whether the diagram uses animation.
- How the layout behaves when its contents change.

Document actual observations.

Do not claim that a particular feature is supported by Python Tutor unless you have verified it.

### 2.2. Inspect implementation details

Read relevant source files rather than relying exclusively on screenshots.

Identify the actual implementation responsibilities for:

- Runtime-state representation.
- Rendering stack and heap objects.
- Mapping pointer relationships to visual elements.
- Arrow positioning.
- Source-line synchronization.
- Timeline navigation.
- Restoring previous states.

Use these findings to guide AveLoop's architecture.

Do not assume that Python Tutor's internal architecture must be copied exactly.

---

## # 3. PYTHON TUTOR SOURCE REUSE AND LICENSING

Investigate the public Python Tutor source code for implementation techniques that can inform AveLoop.

Before directly reusing source code, inspect:

- The applicable license.
- Attribution requirements.
- Dependencies.
- Compatibility.
- Architectural assumptions.
- Maintenance implications.

Do not copy the complete Python Tutor application into AveLoop without a documented reason and a clear understanding of the licensing requirements.

Prefer a clean implementation based on researched behavior.

If a specific algorithm or component is reused, document its origin and comply with its license.

Do not present independently written code as original Python Tutor source code.

Behavioral fidelity is required; an undocumented source-code transplant is not.

---

## # 4. WEB RESEARCH — COMMUNITY DISCUSSIONS AND IMPLEMENTATION EXAMPLES

Official sources should remain the primary authority.

However, real-world discussions can reveal problems and implementation details that official documentation does not explain.

Investigate relevant discussions on:

GitHub:

https://github.com/

X / Twitter:

https://x.com/

Reddit:

https://www.reddit.com/

Search for discussions about:

- Python Tutor internals.
- C memory visualization.
- Pointer-arrow rendering.
- Stack and heap diagrams.
- Execution trace models.
- Browser-based developer tools.
- Night Owl theme implementations.
- Night Owl ports and token-mapping issues.
- Material Design 3 desktop interfaces.
- Material Web maintenance and limitations.
- Material Symbols implementation.
- Modern browser-based code editors.
- Code-editor accessibility.
- Frontend performance.
- Dense IDE layout design.
- Poorly designed AI-generated interfaces.
- Interface quality-review methods.
- Product-specific visual design.
- Design systems for developer tools.

Use community sources to discover useful examples, edge cases, and problems.

Verify technical claims against official sources or reproducible evidence.

Do not treat a Reddit comment, social-media opinion, or third-party blog as definitive technical authority.

Do not stop after finding one example.

---

## # 5. RESEARCH THE SKILLS AND METHODS REQUIRED FOR HIGH-QUALITY UI IMPLEMENTATION

**Do not treat interface quality as a side effect of writing CSS.**

Research the actual methods, design practices, technical skills, and evaluation techniques that can produce a high-quality developer interface.

The objective is to identify the specific practices that improve AveLoop's visual quality, usability, consistency, and performance.

Do not rely on vague instructions such as:

- Make it modern.
- Make it beautiful.
- Make it clean.
- Make it premium.
- Make it professional.
- Make it unique.

These descriptions are insufficient without concrete design decisions and verification criteria.

### 5.1. Research relevant frontend design practices

Investigate reliable resources covering:

- Product-specific interface design.
- Visual hierarchy.
- Information density.
- Layout composition.
- Typography and text readability.
- Color relationships.
- Spacing systems.
- Alignment.
- Component consistency.
- Interaction design.
- Keyboard navigation.
- Accessibility.
- Responsive behavior.
- Visual feedback.
- Error states.
- Empty states.
- Loading states.
- Focus states.
- Motion and animation.
- Browser rendering performance.
- Component architecture.
- CSS organization.
- Design tokens.
- Interface testing.
- Iterative visual refinement.

Prioritize official design-system documentation, established usability guidance, browser documentation, reputable engineering articles, and actual implementations of mature developer tools.

Inspect how established code editors and developer tools organize dense information without making the interface overwhelming.

Use these references to learn from specific decisions, not to copy an entire product's appearance.

### 5.2. Research how to avoid generic AI-generated UI

Investigate the common characteristics of low-quality, generic, AI-generated interface designs.

Use relevant sources such as:

https://m3.material.io/

https://m3.material.io/foundations/accessible-design

https://blog.interfacekit.io/what-makes-a-website-look-ai-generated

https://github.com/

Search for additional credible examples and practical interface-review methods.

Treat third-party commentary as supporting evidence rather than an official design standard.

Identify recurring problems, including:

- Generic dashboard layouts that do not match the product.
- Excessive use of identical cards.
- Decorative elements without a functional purpose.
- Unnecessary gradients.
- Glassmorphism without a product-specific reason.
- Neon glow effects.
- Overuse of rounded containers.
- Oversized headings in compact developer workspaces.
- Excessive whitespace that wastes useful working area.
- Repeated icon-and-heading patterns.
- Arbitrary colors.
- Inconsistent spacing.
- Inconsistent typography.
- Too many visual accents competing for attention.
- Decorative charts or statistics unrelated to the product.
- Controls that look functional but do nothing.
- Empty panels that have no meaningful purpose.
- Inconsistent hover, focus, and disabled states.
- Unclear navigation.
- Poor contrast.
- Inaccessible interaction states.
- Animations that add distraction without improving understanding.
- Generic copy that could belong to any application.
- A visually polished initial screenshot with broken interactions elsewhere.

Do not assume that every rounded corner, card, gradient, or common font is inherently bad.

The problem is using design patterns without a clear reason or without considering the product.

Choose components and visual treatments according to AveLoop's actual requirements.

### 5.3. Establish a product-specific visual direction

Before implementation, define the visual character of AveLoop.

The interface should communicate:

- A serious developer tool.
- A workspace for understanding program execution.
- A coherent Night Owl identity.
- Clear relationships between source code and runtime state.
- High information density without visual clutter.
- Technical precision.
- Reliable, predictable interactions.

Night Owl already establishes a strong visual foundation.

Do not add a competing visual identity merely to make the application appear more distinctive.

Make the product-specific character emerge from:

- The relationship between the editor and execution diagram.
- The arrangement of stack and memory regions.
- The treatment of pointer relationships.
- The hierarchy of execution controls.
- The readability of source code.
- The way runtime changes are presented.
- The quality of interaction feedback.
- The consistency of the Night Owl palette.
- The precision of spacing and alignment.

The visualization itself should provide much of the interface's identity.

### 5.4. Research interface-quality evaluation methods

Investigate how to evaluate an interface beyond a single screenshot.

Use practical criteria such as:

- Visual hierarchy.
- Alignment.
- Information density.
- Text readability.
- Consistency.
- Interaction clarity.
- Keyboard usability.
- Accessibility.
- Functional completeness.
- Responsive behavior.
- State consistency.
- Performance.
- Absence of unnecessary decoration.

Determine how to review the interface at different execution steps, different panel configurations, different viewport sizes, and different interaction states.

A polished initial screen does not prove that the interface is well designed.

### 5.5. Convert research into implementation decisions

Do not create a research section that has no effect on the implementation.

For every important finding, identify the corresponding decision.

Examples:

- If research shows that excessive card nesting reduces visual hierarchy, avoid unnecessary nested cards.
- If the execution diagram requires strong spatial relationships, prioritize a coherent visualization workspace over disconnected dashboard panels.
- If a component creates unnecessary interaction complexity, simplify it.
- If an animation obscures pointer movement, remove or reduce it.
- If a layout wastes space needed by code and memory diagrams, adjust the proportions.
- If a control is not essential, do not add it merely to make the interface appear more feature-rich.

The final interface must demonstrate the results of this research.

---

## # 6. REQUIRED RESEARCH DOCUMENT

Create a research document named:

`TECHNICAL_RESEARCH.md`

If the project requires a single-file HTML deliverable, keep this research document separate from the application.

Complete the relevant research before implementing the interface.

Document:

1. Official sources inspected.
2. Relevant Python Tutor source files and their responsibilities.
3. Python Tutor interactions actually tested.
4. The proposed execution-state model.
5. The pointer-arrow rendering strategy.
6. Original Night Owl theme files inspected.
7. Night Owl colors and token mappings.
8. The distinction between original theme values and AveLoop-specific tokens.
9. Material Design 3 principles selected.
10. Material Web's current maintenance status and suitability.
11. The official icon set selected.
12. Current code-editor options investigated.
13. The selected editor and the evidence supporting the decision.
14. Relevant interface-design practices researched.
15. Common poor-UI patterns deliberately avoided.
16. Performance considerations.
17. Accessibility considerations.
18. Licensing and dependency considerations.
19. Technical limitations.
20. Decisions that remain provisional.

Include links to the sources that informed important decisions.

Do not invent findings or claim that uninspected sources were reviewed.

The document should be concise enough to remain useful, but detailed enough to explain the technical and design decisions.

---

## # 7. MATERIAL DESIGN 3 — REQUIRED RESEARCH

Material Design 3 guides component design and interaction behavior.

It does not replace Night Owl as the primary palette.

### Official sources

Material Design:

https://m3.material.io/

Foundations:

https://m3.material.io/foundations

Components:

https://m3.material.io/components

Accessibility:

https://m3.material.io/foundations/accessible-design

Material Web repository:

https://github.com/material-components/material-web

Material Web documentation:

https://github.com/material-components/material-web/tree/main/docs

Material Web introduction:

https://github.com/material-components/material-web/blob/main/docs/intro.md

Material Web quick start:

https://github.com/material-components/material-web/blob/main/docs/quick-start.md

Research the current official documentation.

Investigate the aspects most relevant to a desktop developer tool:

- Color roles.
- Typography.
- Spacing.
- Shape.
- Surface hierarchy.
- Buttons.
- Icon buttons.
- Inputs.
- Menus.
- Tooltips.
- Tabs.
- Sliders.
- Dividers.
- Dialogs, if required.
- Focus states.
- Hover states.
- Pressed states.
- Disabled states.
- Keyboard interaction.
- Accessibility.
- Motion.
- Responsive layout.

### 7.1. Apply Material Design 3 selectively

Do not mechanically turn every region into a card.

Do not use Material 3 as an excuse to build a generic dashboard.

The interface must reflect the needs of a code editor and execution visualizer.

Use Material 3 to improve:

- Consistency.
- Readability.
- Interaction feedback.
- Focus visibility.
- Accessibility.
- Component behavior.
- Layout hierarchy.
- Spacing.
- Typography.

Prefer a clear workspace layout over excessive containers.

### 7.2. Respect Night Owl

Do not apply a default Material color scheme that overwrites Night Owl.

If using Material Web, customize the documented component tokens and properties.

Verify that component styles preserve the original Night Owl palette.

Avoid fragile CSS overrides when supported customization mechanisms exist.

---

## # 8. MATERIAL WEB — OFFICIAL COMPONENT LIBRARY EVALUATION

Investigate Google's official Material Web library:

https://github.com/material-components/material-web

Documentation:

https://github.com/material-components/material-web/tree/main/docs

The library provides web components based on Material Design 3.

However, its current maintenance status and technical suitability must be evaluated before use.

### 8.1. Research its current state

Inspect:

- Current maintenance status.
- Recent releases.
- Repository activity.
- Supported browsers.
- Dependencies.
- Installation requirements.
- CDN compatibility.
- Build requirements.
- Component availability.
- Bundle size.
- Accessibility behavior.
- Theming APIs.
- Night Owl compatibility.
- Single-file HTML compatibility.

Do not assume that the library is actively developed merely because it is official.

Do not assume that it is unsuitable merely because it has maintenance limitations.

Base the decision on the current evidence and the project's actual requirements.

### 8.2. Make a documented decision

Choose one of these approaches:

**A. Selected Material Web components**

Use official components when they provide meaningful benefits and remain technically appropriate.

**B. Native implementation guided by Material Design 3**

Implement the necessary controls directly when that is simpler, more maintainable, or better suited to the project.

**C. A carefully justified combination**

Use official components for selected controls and implement the remaining components directly.

Explain the choice in `TECHNICAL_RESEARCH.md`.

Do not import the entire library unnecessarily.

Do not introduce overlapping component libraries without a clear reason.

Do not use outdated components simply because they are familiar.

The implementation must remain visually coherent with Night Owl regardless of the chosen approach.

---

## # 9. OFFICIAL MATERIAL SYMBOLS AND MATERIAL ICONS

Use Google's official icon ecosystem for the main interface controls.

### Official sources

Material Symbols and Icons repository:

https://github.com/google/material-design-icons

Official icon library:

https://fonts.google.com/icons

Material Icons documentation:

https://developers.google.com/fonts/docs/material_icons

Material Web icon documentation:

https://github.com/material-components/material-web/blob/main/docs/components/icon.md

Material Web icon-button documentation:

https://github.com/material-components/material-web/blob/main/docs/components/icon-button.md

### 9.1. Research the available icon systems

Understand the differences between:

- Classic Material Icons.
- Material Symbols.
- Material Web's icon component.

Do not assume they use identical APIs.

Evaluate the official styles:

- Outlined.
- Rounded.
- Sharp.

Select one consistent style for AveLoop's primary interface.

Material Symbols Outlined may be a suitable starting point, but verify the visual result.

### 9.2. Verify every icon

Use appropriate official icons for:

- Play.
- Pause.
- Step forward.
- Step backward.
- First step.
- Last step.
- Reset.
- Expand.
- Collapse.
- Settings.
- Search.
- Console.
- Variables.
- Stack.
- Heap or memory.
- Editor controls.
- Navigation.
- More options.

Only implement controls that are actually required.

Do not add unnecessary toolbar buttons simply to fill space.

Verify every icon name using the official library.

Do not guess names.

### 9.3. Icon implementation requirements

- Use the chosen official icon set consistently.
- Load the correct font or SVG assets.
- Configure font families and variable-font settings correctly.
- Use appropriate icon sizes.
- Align icons with their labels.
- Provide accessible names for icon-only controls.
- Add tooltips where useful.
- Verify focus, hover, pressed, and disabled states.
- Avoid missing glyphs.
- Avoid arbitrary mixtures of icon styles.

Do not use emojis, arbitrary Unicode characters, or text glyphs as substitutes for interface icons.

---

## # 10. CODE EDITOR — RESEARCH AND SELECTION

**Do not use CodeMirror 5. Do not make CodeMirror 5 the default recommendation.**

The CodeMirror 5 repository was archived in April 2026 and is now read-only.

Research current editor options and choose an appropriate solution based on the requirements of AveLoop.

### 10.1. Research candidate editors

At minimum, investigate relevant current documentation for:

CodeMirror:

https://codemirror.net/

CodeMirror 6 repository:

https://github.com/codemirror/dev

Monaco Editor:

https://github.com/microsoft/monaco-editor

Monaco Editor documentation:

https://microsoft.github.io/monaco-editor/

You may investigate additional suitable editors if they offer meaningful benefits.

Do not assume that CodeMirror 6 or Monaco must be selected.

Do not select an editor solely because it is familiar or commonly used.

### 10.2. Compare the actual requirements

Evaluate:

- Current maintenance status.
- Browser support.
- JavaScript and CSS integration.
- Single-file HTML compatibility.
- Dependency requirements.
- Bundle size.
- Loading performance.
- Memory consumption.
- C syntax highlighting.
- Theme customization.
- Line numbers.
- Current-line highlighting.
- Keyboard behavior.
- Selection and copy/paste.
- Bracket matching.
- Tab handling.
- Whitespace preservation.
- Accessibility.
- API stability.
- Ease of integration with AveLoop's execution-state model.

Document the comparison and explain the final choice.

### 10.3. Preserve exact source text

The editor must support a real editing experience.

A space must remain a space.

A literal tab must remain a literal tab when configured.

Do not silently normalize tabs into spaces.

Do not collapse consecutive spaces.

Do not automatically reformat source code.

Preserve line endings where practical and avoid unnecessary transformations when loading, editing, or saving source text.

Configure Tab behavior explicitly.

Tab must not unexpectedly move keyboard focus away from the editor.

If the selected editor cannot preserve the required whitespace behavior without substantial workarounds, account for that limitation before selecting it.

### 10.4. Required editor features

Implement and verify:

- C syntax highlighting.
- Line numbers.
- Current-line indication.
- Cursor movement.
- Selection.
- Copy and paste.
- Enter.
- Backspace and Delete.
- Space insertion.
- Tab insertion.
- Shift+Tab behavior.
- Bracket matching.
- Monospaced typography.
- Clear focus states.
- Night Owl syntax highlighting.

Do not display a Vim keymap unless it is genuinely implemented.

Editing code must not falsely imply that the modified source has been executed.

Because the prototype uses a predefined execution trace, clearly indicate when the trace may no longer match the edited source.

---

## # 11. PRODUCT IDENTITY

The product name is:

**AveLoop — Visual Code Execution**

AveLoop is a general-purpose visual programming IDE.

C is the initial demonstration language, not the permanent product boundary.

Keep the architecture extensible to additional languages.

Do not make the interface look like a C-only utility.

Do not use "Engine" as the primary product name.

Night Owl is AveLoop's official default theme.

Do not present Night Owl as an optional decorative skin.

---

## # 12. IMPLEMENTATION SCOPE AND ARCHITECTURE

The current deliverable is a frontend-only prototype.

Preserve the required single-file HTML delivery format unless the available project environment imposes a documented technical constraint that makes it unsuitable.

The interface must be designed from scratch.

Do not assume that an existing UI implementation exists.

Do not spend time preserving nonexistent components or rewriting nonexistent stylesheets.

Before implementation:

1. Inspect the available project files.
2. Determine the expected entry point and delivery format.
3. Identify any actual project constraints.
4. Complete the relevant research.
5. Establish the visual direction.
6. Define the execution-state model.
7. Select the editor.
8. Select the icon implementation.
9. Determine whether Material Web is appropriate.
10. Implement the interface.

Do not introduce a backend, database, server-side compiler, or external execution service as part of this task.

Do not claim to execute arbitrary C code.

Use a deterministic, predefined execution trace.

Clearly label the execution as simulated.

Keep trace data separate from rendering logic so that a real execution adapter can be added later.

---

## # 13. PYTHON TUTOR-LEVEL EXECUTION VISUALIZATION

Do not reproduce only Python Tutor's general layout.

Implement the interaction principles that make execution state understandable.

The selected execution step must determine the visible source line, stack, variables, heap, pointers, and console output.

### 13.1. Source-line synchronization

For every execution step:

- Identify the corresponding source line.
- Highlight the correct line.
- Keep the highlight synchronized with the selected state.
- Update it during forward and backward navigation.
- Update it immediately during direct timeline seeking.

Never leave the editor highlighting a line from a different execution state.

### 13.2. Stack frames

Represent function frames as identifiable runtime regions.

Support the demonstration's:

- Main function.
- Function calls.
- Function parameters.
- Local variables.
- Variable initialization.
- Scope changes.
- Function returns.

Variables must belong to the correct frame.

When a function is called, render its frame.

When it returns, update the stack consistently with the selected trace state.

Backward navigation must restore the previous stack configuration.

### 13.3. Memory visualization

Represent the memory regions required by the demonstration.

Distinguish between:

- Global storage.
- Stack storage.
- Heap allocations.
- Arrays.
- Array elements.
- Structs.
- Struct fields.
- Pointer variables.
- Pointer targets.

Do not imply that every variable resides on the heap.

Do not merge a pointer variable visually with the object it references.

Use stable identities for memory objects across trace steps.

### 13.4. Pointer-arrow accuracy

Pointer arrows are a core feature of AveLoop.

Each arrow must represent an actual relationship in the selected execution state.

A pointer relationship must identify:

- The source pointer variable.
- The containing frame or memory location.
- The target memory region.
- The target object.
- The target element or offset, where applicable.
- The pointer's validity state.

The arrow must originate from the rendered pointer variable.

The arrow must terminate at the correct rendered target.

If a pointer targets an interior array element, the arrow must identify that element rather than pointing vaguely to the whole array.

If multiple pointers refer to the same object, render each relationship.

If a pointer changes its target, update its arrow.

If a pointer becomes NULL, remove its previous target connection.

If an allocation disappears, remove arrows to that allocation unless the trace explicitly models a dangling pointer.

Never invent pointer relationships.

Never leave stale arrows on screen.

### 13.5. SVG-based arrow geometry

Use SVG or an equally reliable vector-rendering mechanism.

Use stable DOM identifiers for pointer variables and memory targets.

Calculate arrow endpoints from the actual rendered geometry of the source and target elements.

Account for:

- Container offsets.
- Scrolling.
- Resizing.
- Panel changes.
- Text wrapping.
- Stack-frame insertion and removal.
- Heap-layout changes.
- Array-size changes.
- Inspector switching.
- Elements appearing and disappearing.

Use `ResizeObserver` or an appropriate equivalent.

Schedule geometry updates carefully, using animation frames when necessary.

Avoid continuous full-layout recalculation when nothing has changed.

Do not hard-code coordinates for a single screenshot.

Route arrows to avoid obscuring labels and values whenever practical.

Multiple arrows may share a target, but their endpoints must remain correct.

Arrow colors must be legible against the Night Owl palette.

Do not introduce an unrelated accent palette merely to make arrows visible.

### 13.6. Pointer transitions

When the selected execution step changes:

- Update the pointer relationships.
- Update arrow geometry.
- Keep the diagram consistent with the new state.
- Allow the user to follow meaningful changes.
- Respect reduced-motion preferences.

Animations must never obscure the final pointer relationship.

Pausing must leave the diagram stable.

Direct seeking must show the correct state immediately.

Backward and forward navigation must restore the corresponding relationships.

### 13.7. C memory semantics

Represent the demonstration's C memory behavior accurately.

Where relevant, support:

- Pointers to array elements.
- Pointer arithmetic.
- Interior pointers.
- Multiple pointers to the same allocation.
- NULL pointers.
- Struct pointers.
- Allocation and deallocation.
- Uninitialized values.
- Invalid or out-of-bounds states.

Only represent an error state when the selected trace actually models it.

Memory addresses must be explicitly illustrative because the prototype does not execute the C program.

Do not fabricate actual runtime addresses.

Do not imply that AveLoop has detected real memory errors in arbitrary user code.

---

## # 14. AUTHORITATIVE EXECUTION-STATE MODEL

Use one authoritative execution state to drive the complete visualization.

The state model must contain the information required to render each selected step.

Conceptually, it may include:

- Step index.
- Source line.
- Execution event.
- Stack frames.
- Variables.
- Variable values.
- Memory regions.
- Array elements.
- Struct fields.
- Pointer relationships.
- Allocation states.
- Console output.

Choose the actual schema based on the researched requirements.

Use stable identifiers for runtime entities.

Do not maintain unrelated copies of the same execution state in separate UI components.

The editor, stack, variables, heap, pointers, and console must all derive from the same selected state.

Snapshots are acceptable for a deterministic frontend prototype.

Every snapshot must remain internally consistent.

The model must be extensible to future execution adapters and additional programming languages.

---

## # 15. DEMONSTRATION PROGRAM

Use a compact C program that demonstrates meaningful execution changes.

Insertion sort may be suitable if it provides a clear and coherent trace.

Do not choose a complex example merely to make the interface look impressive.

The demonstration should provide enough steps to illustrate:

1. Initial state.
2. Main stack frame.
3. Variable initialization.
4. Array initialization.
5. Pointer assignment.
6. Pointer-arrow appearance.
7. Loop progression.
8. Array-element access.
9. Array-element modification.
10. Variable changes.
11. Pointer reassignment or pointer arithmetic, if included.
12. Console output.
13. Program completion or function return.

Every state must be consistent with the selected C example.

Do not invent impossible memory states.

If the example uses dynamic allocation, represent allocation and lifetime consistently.

If it uses pointers, ensure those pointers have meaningful relationships to visualize.

---

## # 16. NIGHT OWL VISUAL IMPLEMENTATION

The interface must feel like a carefully designed Night Owl developer workspace.

Avoid a generic dashboard layout.

Do not introduce a second dominant visual identity.

Use Night Owl as the foundation for:

- Application background.
- Editor.
- Side panels.
- Execution workspace.
- Stack and memory regions.
- Console.
- Toolbars.
- Playback controls.
- Buttons.
- Inputs.
- Tooltips.
- Menus.
- Dividers.
- Focus states.
- Selection states.

Preserve the original syntax-color identity.

Use AveLoop-specific semantic colors only where additional distinctions are necessary.

### 16.1. Runtime colors

The visualization may require additional colors for:

- Current execution.
- Changed values.
- Pointer relationships.
- Selected variables.
- Memory allocation.
- Invalid memory states.
- Warnings.
- Errors.

These must be treated as AveLoop-specific semantic tokens.

Do not replace the original Night Owl palette.

Do not assign arbitrary colors to every variable or memory object.

Use labels, layout, and shape in addition to color.

Maintain readable contrast.

### 16.2. Visual hierarchy

Prioritize the information needed to understand program execution.

The editor and execution diagram are the primary workspace.

Playback controls should be easy to find without dominating the interface.

Secondary information should remain accessible without competing with the source and memory visualization.

Do not make every panel equally prominent.

Do not use decoration to compensate for weak hierarchy.

---

## # 17. ANTI-GENERIC-UI REQUIREMENTS

**The interface must not look like a generic AI-generated dashboard.**

This requirement must affect actual design decisions and the final review.

Do not use a generic SaaS dashboard as the default layout.

Do not start with a grid of cards and attempt to transform it into an IDE afterward.

### 17.1. Avoid unnecessary decoration

Unless a specific functional purpose justifies it, avoid:

- Large gradient backgrounds.
- Glassmorphism.
- Neon glow.
- Decorative blurred blobs.
- Excessive shadows.
- Arbitrary accent colors.
- Repeated oversized rounded cards.
- Nested cards inside cards.
- Huge headings.
- Oversized icons.
- Decorative charts.
- Unrelated statistics.
- Empty visual regions.
- Excessive badges.
- Floating elements that obscure the workspace.
- Animations applied to every interaction.

Do not use these patterns simply because they are common in AI-generated designs.

### 17.2. Avoid generic component repetition

Do not make every interface section look identical.

A stack frame, code editor, memory object, console, toolbar, and timeline have different purposes.

Design them according to their information and interaction requirements.

Use consistent design tokens without forcing every component into the same visual shape.

### 17.3. Prioritize product semantics

Every important visual element must have a clear reason to exist.

Ask:

- What information does this element communicate?
- Why is it located here?
- What does its visual emphasis mean?
- Does it help the user understand execution?
- Does it improve interaction?
- Is it redundant?
- Would removing it make the interface clearer?

Remove elements that do not improve functionality, comprehension, navigation, or visual hierarchy.

### 17.4. Avoid superficial polish

A screenshot that looks polished is insufficient.

The interface must also work correctly when:

- The execution step changes.
- Variables change.
- Stack frames appear or disappear.
- Heap allocations change.
- Pointer arrows move.
- The console receives output.
- The user resizes a panel.
- The user scrolls.
- The user changes inspector views.
- The viewport becomes narrower.

Design quality must survive real interaction.

### 17.5. Maintain a restrained developer-tool aesthetic

Use:

- Deliberate alignment.
- Consistent spacing.
- Readable typography.
- Appropriate information density.
- Clear component hierarchy.
- Restrained animation.
- Meaningful interaction feedback.
- Night Owl's established palette.
- Material 3 principles where appropriate.

Do not mistake minimalism for removing useful information.

Do not mistake complexity for professional quality.

The interface must be purpose-built for understanding code execution.

---

## # 18. APPLICATION LAYOUT

Design the application as a coherent desktop workspace.

The exact arrangement should be determined by the requirements and research, not by a generic dashboard template.

### 18.1. Header

Include the AveLoop identity and essential application controls.

Keep it compact.

Use official Material icons or symbols.

Avoid unnecessary navigation items.

### 18.2. Code editor

Provide:

- C syntax highlighting.
- Line numbers.
- Current execution line.
- Real editing interactions.
- Night Owl colors.

The editor must remain readable and must not be reduced to a decorative code screenshot.

### 18.3. Execution visualization

Provide a dedicated stack-and-memory workspace.

Show the relevant:

- Stack frames.
- Variables.
- Heap allocations.
- Arrays.
- Structs.
- Pointer arrows.

Use coherent spatial relationships.

Do not represent every object as an unrelated card.

### 18.4. Inspector

Provide organized views for:

- Variables.
- Stack.
- Heap or memory.
- Trace events.

Inspector switching must not reset execution state.

Do not introduce redundant views that repeat the same information without adding value.

### 18.5. Playback controls

Provide:

- Previous step.
- Play or pause.
- Next step.
- Timeline.
- Current step indicator.

Add first-step, last-step, reset, and playback-speed controls when appropriate.

Every visible control must have real behavior.

### 18.6. Console

Show only the output available at the selected trace step.

Backward navigation must restore the correct earlier output.

Use Night Owl-compatible colors.

### 18.7. Status bar

Display compact, useful information such as:

- Language.
- Cursor position.
- Selected execution step.
- Simulation status.

Do not fill the status bar with decorative metadata.

---

## # 19. FUNCTIONAL REQUIREMENTS

All visible controls must work.

Implement and verify:

- Previous-step navigation.
- Next-step navigation.
- Play and pause.
- Playback speed.
- Direct timeline seeking.
- Source-line highlighting.
- Variable updates.
- Stack updates.
- Heap updates.
- Pointer-arrow updates.
- Console updates.
- Inspector switching.
- Console collapse.
- Pane resizing, if provided.
- Editor keyboard interactions.
- Relevant keyboard shortcuts.

Prevent:

- Duplicate playback timers.
- Stale event listeners.
- Invalid step indices.
- Stale pointer arrows.
- Stale variable values.
- Incorrect console output.
- Contradictory execution states.
- Controls that look functional but do nothing.

Do not include a control merely for visual completeness.

---

## # 20. PERFORMANCE AND RESPONSIVENESS

Performance must be researched, implemented, and tested.

Do not assume that a small HTML file is automatically fast.

Do not assume that a component library is automatically efficient.

Do not optimize only for the initial screenshot.

### 20.1. Implementation architecture

Keep the logic organized around clear responsibilities:

- Editor initialization.
- Night Owl theme tokens.
- Trace data.
- State selection.
- Timeline navigation.
- Stack rendering.
- Memory rendering.
- Pointer rendering.
- Console rendering.
- Event handling.
- Layout observation.
- Playback.

Use the simplest architecture that meets the requirements.

Do not introduce unnecessary abstractions.

Do not create a large framework around a small prototype without a measurable benefit.

### 20.2. Rendering performance

Avoid rebuilding the entire interface when only a small portion changes.

Update only the components that need to change.

Avoid unnecessary DOM replacement.

Avoid excessive layout recalculation.

Recalculate arrow geometry when relevant elements move or change.

Do not continuously redraw arrows when their geometry is unchanged.

Avoid redundant timers and event listeners.

### 20.3. Responsive behavior

Support narrower viewports without making the desktop workspace unnecessarily sparse.

Maintain usable editor and visualization dimensions.

Prevent pointer arrows from becoming detached when panels change layout.

Ensure that scrolling and resizing do not permanently break the diagram.

### 20.4. Motion

Use motion only when it improves comprehension or interaction feedback.

Respect `prefers-reduced-motion`.

Avoid continuous decorative animation.

Do not animate elements in ways that obscure code or memory relationships.

---

## # 21. ACCESSIBILITY AND INTERACTION QUALITY

Accessibility must be considered during implementation, not added at the end.

Verify:

- Keyboard navigation.
- Visible focus indicators.
- Accessible names for icon-only buttons.
- Sufficient text contrast.
- Understandable disabled states.
- Readable typography.
- Appropriate interactive target sizes.
- Clear control semantics.
- Reduced-motion support.
- Focus behavior in the editor.
- Focus behavior when switching panels.
- Usability without relying exclusively on color.

Do not make pointer relationships understandable only through color.

Use direction, labels, and spatial positioning to reinforce meaning.

Follow applicable official accessibility guidance.

Do not claim accessibility compliance without appropriate verification.

---

## # 22. VERIFICATION REQUIREMENTS

Do not stop after generating the first working version.

Do not judge the result only by looking at the initial screen.

Inspect the application during actual interactions and refine it.

### 22.1. Night Owl fidelity

Compare the implementation against the original Night Owl theme definition.

Verify:

- Background.
- Foreground.
- Syntax colors.
- Comments.
- Keywords.
- Strings.
- Numbers.
- Functions.
- Variables.
- Line numbers.
- Selection.
- Current-line highlighting.
- Cursor.
- Side panels.
- Borders.
- Buttons.
- Interactive states.

Verify that the surrounding application remains consistent with Night Owl.

Document deviations.

### 22.2. Material Design 3

Review:

- Spacing.
- Typography.
- Component hierarchy.
- Surface hierarchy.
- Focus indicators.
- Interaction states.
- Accessibility.
- Motion.
- Responsiveness.

Verify that Material component defaults do not replace Night Owl.

### 22.3. Icons

Verify:

- Official icon names.
- Successful loading.
- Consistent icon style.
- Correct sizing.
- Alignment.
- Accessible labels.
- Hover states.
- Focus states.
- Disabled states.
- No missing glyphs.

### 22.4. Code editor

Verify:

- Editor initialization.
- C syntax highlighting.
- Night Owl token mapping.
- Cursor movement.
- Selection.
- Copy and paste.
- Enter.
- Backspace.
- Delete.
- Space insertion.
- Tab insertion.
- Shift+Tab.
- Whitespace preservation.
- Focus behavior.

Verify that the chosen editor works with the project's delivery constraints.

### 22.5. Python Tutor behavioral fidelity

Compare AveLoop's actual behavior with the researched Python Tutor interactions.

Verify:

- Source-line synchronization.
- Stack-frame changes.
- Variable changes.
- Heap changes.
- Pointer relationships.
- Function entry and exit.
- Forward navigation.
- Backward navigation.
- Direct step selection.
- Console output restoration.

Document meaningful differences.

### 22.6. Pointer-arrow accuracy

Test:

1. Pointer to a valid array element.
2. Arrow origin attached to the correct pointer variable.
3. Arrow endpoint attached to the correct target element.
4. Pointer reassignment.
5. Pointer arithmetic.
6. Multiple pointers referencing the same object.
7. NULL state.
8. Allocation removal.
9. Explicitly represented dangling pointer.
10. Backward restoration.
11. Forward restoration.
12. Window resizing.
13. Panel resizing.
14. Scrolling.
15. Inspector switching.
16. Array-layout changes.

Every arrow must represent the selected trace state.

### 22.7. State consistency

For each selected step, verify that:

- The source line is correct.
- Variables contain the correct values.
- Stack frames are correct.
- Heap objects are correct.
- Array elements are correct.
- Pointer relationships are correct.
- Console output is correct.

All components must represent the same state.

### 22.8. Performance

Inspect for:

- Unnecessary DOM updates.
- Repeated layout calculations.
- Redundant timers.
- Duplicate event listeners.
- Unnecessary animation.
- Excessive dependencies.
- Layout instability.
- Arrow-rendering performance issues.

Use actual browser measurements when available rather than claiming performance improvements based only on code inspection.

### 22.9. Visual-quality review

Conduct a deliberate design review after implementation.

Evaluate the interface at:

- Initial state.
- Middle execution step.
- Final execution step.
- A step with changed variables.
- A step with changed pointer relationships.
- A step with a modified stack.
- A step with different heap contents.
- Collapsed-console state.
- Alternate inspector views.
- Resized layout.
- Narrower viewport.

Ask:

- Does the interface look specifically designed for AveLoop?
- Is the execution diagram the visual center of the workspace?
- Is the hierarchy clear?
- Is the information density appropriate?
- Are components aligned?
- Are colors consistent?
- Are any decorative elements unnecessary?
- Are any panels redundant?
- Are the controls understandable?
- Does the application still look coherent when the state changes?
- Does any part resemble a generic dashboard unrelated to the product?
- Does the design remain faithful to Night Owl?

Do not rely on a generic claim that the interface is "modern" or "beautiful."

Identify actual problems and fix them.

### 22.10. Runtime stability

Check for:

- JavaScript errors.
- Invalid DOM references.
- Duplicate IDs.
- Broken event listeners.
- Incorrect timers.
- Stale arrows.
- Missing fonts.
- Failed dependencies.
- Unexpected CSS overrides.
- Layout failures.
- Incorrect state restoration.

Fix discovered defects before finishing.

---

## # 23. ITERATIVE DESIGN AND IMPLEMENTATION LOOP

Use a closed-loop process rather than a single generation attempt.

Follow this sequence:

**1. Research**

Inspect official references, source code, current library status, and design-quality guidance.

**2. Define**

Establish the visual direction, component hierarchy, theme tokens, editor choice, execution-state model, and implementation constraints.

**3. Implement**

Build the complete interface from scratch.

**4. Run**

Launch the application in the intended environment.

**5. Inspect**

Evaluate the real rendered interface, not just the source code.

**6. Interact**

Test the controls, editor, timeline, stack, heap, arrows, and responsive behavior.

**7. Identify problems**

Prioritize issues affecting comprehension, correctness, visual hierarchy, accessibility, and performance.

**8. Fix**

Make targeted improvements without introducing unrelated redesigns.

**9. Re-test**

Verify that the fixes work and have not broken existing behavior.

**10. Repeat**

Continue until the acceptance criteria are met or a specific environmental limitation prevents further verification.

Do not stop after the first implementation.

Do not merely produce a list of visual problems without fixing them.

Do not introduce a new design direction at every iteration.

Maintain the Night Owl identity and the product-specific design goals throughout the process.

---

## # 24. ACCEPTANCE CRITERIA

The implementation is complete only when the following requirements are satisfied.

### Research

1. Web research was completed before implementation.
2. The original Night Owl theme source was inspected.
3. Python Tutor's relevant source and documentation were investigated.
4. Python Tutor interactions were tested where possible.
5. Material Design 3 guidance was researched.
6. Material Web was evaluated based on its current state.
7. Official Material icon options were researched.
8. Current code-editor alternatives were compared.
9. Relevant interface-design and frontend-quality practices were researched.
10. Research findings informed actual implementation decisions.

### Product and theme

11. AveLoop is designed from scratch.
12. Night Owl is the official default theme across the interface.
13. The original Night Owl syntax-color identity is preserved as closely as the selected editor allows.
14. The interface does not use an unrelated dominant palette.
15. Original theme values and AveLoop-specific tokens are distinguishable.
16. The product name and purpose are clear.

### Design quality

17. The interface is product-specific rather than a generic dashboard.
18. Visual hierarchy is clear.
19. Information density is appropriate for a developer tool.
20. Components are consistent without being unnecessarily repetitive.
21. Decorative elements serve a purpose.
22. The design remains coherent during real interactions.
23. Accessibility and keyboard usability are considered.
24. The implementation has undergone iterative visual review.

### Components and editor

25. Material Design 3 was applied deliberately.
26. Material Web was evaluated rather than blindly imported.
27. Official Material Symbols or Material Icons are used consistently.
28. No emojis or arbitrary Unicode substitutes are used for interface icons.
29. The selected editor is justified by current research.
30. CodeMirror 5 is not selected as the implementation's required editor.
31. The editor supports real editing interactions.
32. Whitespace behavior is correct.
33. Night Owl syntax highlighting is implemented and verified.

### Execution visualization

34. The trace is deterministic and internally consistent.
35. All visualization components derive from one selected execution state.
36. Source-line highlighting is synchronized.
37. Stack and heap representations are correct for the trace.
38. Pointer arrows represent actual relationships in the selected state.
39. Arrow endpoints remain attached to the correct elements.
40. Backward and forward navigation restore the correct state.
41. Console output remains synchronized.
42. Layout changes do not permanently break the visualization.

### Performance and verification

43. All visible controls have working behavior.
44. Rendering work is not repeated unnecessarily.
45. Runtime errors and major layout defects are resolved.
46. Actual interactions were tested.
47. The application does not falsely claim to execute arbitrary C code.
48. Remaining limitations are documented.

A screenshot that merely resembles Python Tutor does not satisfy the task.

A visually attractive interface with incorrect pointer arrows does not satisfy the task.

A generic dashboard with a Night Owl editor embedded inside it does not satisfy the task.

A polished initial screen with broken interactions does not satisfy the task.

A large amount of code does not prove that the implementation is good.

The final result must demonstrate technical correctness, coherent visual design, reliable interactions, and deliberate implementation decisions.

---

## # 25. OUTPUT FORMAT

Deliver the **complete, runnable HTML implementation**.

Do not return isolated snippets that require manual assembly.

Do not return only a design proposal.

Do not return only a screenshot or static mockup.

Do not assume that an existing UI should be modified; the interface must be designed from scratch.

Preserve the required single-file HTML delivery format unless a genuine technical constraint prevents it. If an external dependency is needed, document it and explain how the application is launched.

Provide `TECHNICAL_RESEARCH.md` as a separate artifact when the environment supports file creation.

After implementation, report:

1. The main implementation decisions.
2. The official sources actually inspected.
3. The Python Tutor interactions actually investigated.
4. The Night Owl source files inspected.
5. How the original Night Owl colors were applied.
6. Which AveLoop-specific theme tokens were added.
7. The Material Design 3 decisions.
8. Whether Material Web was used and why.
9. Which icon set was selected and how it was integrated.
10. Which code editor was selected and why.
11. Which editor alternatives were investigated.
12. How the execution state is represented.
13. How pointer-arrow geometry is calculated.
14. How navigation restores the correct state.
15. Which design-quality practices influenced the interface.
16. Which generic UI patterns were deliberately avoided.
17. Which performance and accessibility checks were performed.
18. Which interactions were tested.
19. Remaining limitations.

Be precise about what was actually verified.

Do not claim that unimplemented features work.

Do not claim that arbitrary C code is executed.

Do not claim complete equivalence with Python Tutor's execution backend.

Do not claim exact Night Owl reproduction if important colors or token mappings were changed without documenting those changes.

Do not claim performance improvements without appropriate evidence.

**Final priority:** research quality, Night Owl fidelity, Python Tutor behavioral fidelity, accurate memory relationships, reliable pointer arrows, thoughtful Material Design 3 implementation, official icon usage, editor suitability, strong performance, and product-specific interface quality.

The goal is not to make AveLoop look like a typical AI-generated interface.

The goal is to build a coherent, credible, technically correct visual execution IDE whose design choices are justified by the product itself.
