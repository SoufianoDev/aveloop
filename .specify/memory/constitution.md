# AveLoop Constitution

**Version:** 1.0.0\
**Ratified:** 23 November 2026\
**Last amended:** 23 November 2026\
**Status:** Active\
**Task:** T001\
**Location:** `.specify/memory/constitution.md`

## Preamble

This constitution records the engineering principles that govern how AveLoop is built. It governs conduct and structure; it does not restate the product contract.

Authority order:

1. `specs/001-ave-loop-alpha/spec.md` — what the Alpha must do.
2. `specs/001-ave-loop-alpha/plan.md` — how the Alpha is delivered.
3. `specs/001-ave-loop-alpha/tasks.md` — the ordered work breakdown.
4. This constitution — how the work must be conducted.

Where this document and the product contract disagree about product behavior, the product contract wins. Where they disagree about engineering conduct, this document wins. Every principle cites its source.

Requirement words follow `spec.md` §0: **MUST**, **SHOULD**, **MAY**, **OUT**.

## Article I — Rust-only implementation

- The native AveLoop application **MUST** be implemented in Rust. (`spec.md` §11.1, §14)
- Go, WebView production UI, and CodeMirror 5 are **OUT**. (`spec.md` §14)
- The HTML prototype is a reference asset, not production UI or a runtime; it **MUST NOT** be embedded, converted to a WebView, or mechanically ported. (`spec.md` §0.1.5, §8.2)
- Rust standard-library facilities and owned focused components **SHOULD** be preferred over new dependencies. (`plan.md` §8)

**Rationale.** One language and one memory model across the whole application. GPUI is Rust-native; mixing runtimes would duplicate ownership and failure handling.

## Article II — Shared cross-platform code

- Linux, Windows, and macOS **MUST** be treated as co-equal targets of one application and one shared codebase. (`spec.md` §0.1.2, §10.1, INV-008)
- Separate `linux-app`, `windows-app`, or `macos-app` application trees and duplicated application logic are **OUT**. (`plan.md` §2.2)
- Product behavior, domain types, use cases, trace semantics, and GPUI views **MUST** be shared. (`plan.md` §2.2)
- OS-specific code **MAY** exist only in narrow adapters at real platform boundaries. (`spec.md` §11.9, `plan.md` §2.3)
- Platform divergence **MUST** be documented and tested. (`spec.md` INV-008)
- No platform **MAY** be treated as primary or first. (`spec.md` §0.1.2)

**Rationale.** Parity is a product invariant, not a later porting goal. A shared core with narrow adapters keeps one behavior contract.

## Article III — Modular-monolith boundaries

- Alpha **MUST** ship as one Cargo package and one application crate, organized as a modular monolith. (`spec.md` §11.3–11.4, §14)
- A multi-crate workspace **MUST NOT** be the default structure. (`spec.md` §0.1.3–0.1.4)
- Dependency direction **MUST** be: `main` → `presentation` → `application` → `domain`, with `infrastructure` implementing ports and `platform` adapters only at OS boundaries. (`plan.md` §2.3)
- `domain` **MUST NOT** import GPUI, DAP, filesystem, compiler, or OS process types. (`plan.md` §2.3)
- `main` composes concrete implementations and **MUST NOT** contain product logic. (`plan.md` §2.3)
- A module **MAY** become a crate only when dependency ownership, independent testing or reuse, or build isolation provides a concrete benefit, and the extraction is approved. (`spec.md` §11.10)

**Rationale.** Modularity comes from boundaries and dependency direction, not from crate count. Crate extraction is a cost that must be justified.

## Article IV — Execution truth

- Runtime information presented as actual **MUST** originate from the real compiler, debugger, or session, or from a documented deterministic derivation of its recorded data. (`spec.md` INV-001)
- Synthetic fixtures **MUST** be visibly identified and **MUST NOT** be mixed into a real session. (`spec.md` INV-001)
- Exactly one selected execution state **MUST** exist at any moment; source highlighting, stack, variables, heap, pointers, trace selection, and attributable output **MUST** derive from it. (`spec.md` INV-002)
- State transitions **MUST** be published atomically; on reconstruction failure the last valid state is retained and the failure reported. (`spec.md` INV-003)
- Every trace **MUST** be bound to the exact source snapshot and build inputs that produced it; editing source after capture **MUST** mark the trace stale. (`spec.md` INV-007)
- Known-empty, null, uninitialized, optimized-out, unavailable, invalid, unresolved, and unknown **MUST** remain distinct. (`spec.md` INV-004)
- Backward navigation **MUST** reconstruct prior states from AveLoop-owned trace data and **MUST NOT** claim to reverse the live process. (`plan.md` §4.4, `spec.md` §2.2)
- Variable names, addresses, labels, and UI positions **MUST NOT** be treated as stable identity. (`spec.md` INV-005)

**Rationale.** The product promise is verified runtime truth. A smaller accurate visualization is preferred to a complete-looking fabricated one. (`spec.md` §1.2)

## Article V — Testing

- Domain, application, infrastructure, presentation, and end-to-end tests **MUST** exist as described in `plan.md` §7.
- End-to-end acceptance **MUST** use real compiled C17 and C++20 programs; synthetic fixtures **MAY** be used only for unit tests. (`plan.md` §7)
- Every P0 requirement **MUST** map to a test or retained evidence. (`tasks.md` T069; final gate)
- A CI matrix **MUST** be treated as necessary but insufficient: each OS requires a real desktop smoke test and at least one full build/debug/record/navigate/inspect run. (`plan.md` §7)
- Tests **SHOULD** assert invariants and observable behavior rather than internal implementation details.

**Rationale.** The Alpha's claims are verified per platform. Passing CI without a real workflow run does not establish platform support.

## Article VI — Dependency governance

- Every dependency **MUST** be classified as: use upstream, reuse selected primitives, fork, build in AveLoop, or reject. (`plan.md` §8)
- Before adoption, the project **MUST** record purpose and exact feature used, license, maintenance evidence, API stability, platform support, transitive cost, GPUI compatibility, and exit cost. (`plan.md` §8)
- Dependencies **MUST** be justified, license-reviewed, and pinned or otherwise reproducible. (`spec.md` NFR-012)
- AveLoop **MUST NOT** bundle, download, or install a compiler, debugger, or LLVM distribution. (`spec.md` NFR-009, §7.2)
- The GPUI Material fork **MUST NOT** be created or modified as part of this plan. (`plan.md` §8)
- A dependency that only saves a few lines **SHOULD** be rejected in favor of the standard library or an existing dependency.

**Rationale.** Toolchain and dependency surface are long-lived risks. Every addition carries license, maintenance, and platform cost.

## Article VII — Honest capability reporting

- The UI **MUST** disclose backend capability and coverage limitations. (`spec.md` FR-092)
- A value, target, allocation lifetime, or source location the backend cannot establish **MUST** be reported as a limitation, not drawn as a definitive fact. (`spec.md` INV-010)
- Debugger capabilities **MUST** be negotiated at session start and verified per backend; no capability **MUST** be assumed because a protocol defines a related message. (`spec.md` §7.3)
- Complete heap knowledge, pointer-lifetime certainty, and hardened sandbox isolation **MUST NOT** be claimed. (`spec.md` §2.2, FR-144)
- “Dangling” **MUST** be used only for a target whose lifetime end was actually observed; otherwise the state is unresolved or unknown. (`spec.md` §2.2)
- Process addresses **MUST** be labeled as addresses and symbolic identifiers as symbolic IDs; physical addresses **MUST NOT** be invented. (`spec.md` INV-006)
- Accessibility limitations **MUST** be documented; full conformance **MUST NOT** be claimed without an audit. (`spec.md` §8.5)

**Rationale.** Users act on what the tool shows. Overstating certainty is a correctness defect, not a cosmetic one.

## Governance

- **Amendment.** Propose a change in an Issue or an ADR under `docs/decisions/`, state the affected article and reason, and obtain approval before editing this file.
- **Versioning.** MAJOR for removing or redefining a principle, MINOR for adding a principle, PATCH for clarification without semantic change.
- **Compliance.** Every task and pull request is reviewed against these articles. A violation blocks release until fixed or the principle is amended.
- **Precedence.** This constitution does not override `spec.md`, `plan.md`, or `tasks.md` on product scope, and it does not authorize implementation beyond the task breakdown.
- **Derived documents.** `specs/001-ave-loop-alpha/complete.md` is generated; this constitution is not part of that concatenation and is maintained independently.

---

*Ratified as part of T001. Source authority: `AVE-SPEC-001` v1.1.0 and `AVE-PLAN-001` v1.1.0.*
