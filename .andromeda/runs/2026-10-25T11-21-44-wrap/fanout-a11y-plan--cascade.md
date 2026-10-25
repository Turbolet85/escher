proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 17
    section: "## 1. A11y Scope Summary"
    change: "in the Dioxus crates entity, replace «the adapter on the stand binary and its windowed AT witness owned by a `.andromeda/residuals.md` line (target: next)» with «the adapter on the stand binary and its windowed AT witness owned by a `.andromeda/residuals.md` line that reads `re-carried:0.2.0`, the witness at an active assistive technology, which this version does not reach»; the rest of the entity stays as it is"
    sidecar: "a11y-scope-summary — Dioxus crates: the windowed AT witness's owner corrected from a residuals line «(target: next)» to the line that reads `re-carried:0.2.0`"
    rationale: "the report's Spec claims disproved by measurement, pattern `atowner`, lists a11y-scope-summary.md:17 as changed: the residuals line for the stand's platform accessibility adapter and its windowed witness reads `re-carried:0.2.0`, and requirements.md:42-43 owes the witness at an active assistive technology, which this version does not reach"
    basis: .andromeda/residuals.md:14
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-strategy.md
    near: 9
    section: "## 2. A11y Strategy"
    change: "in the tree-lifecycle entity, replace its closing «— and is owned by a `.andromeda/residuals.md` line (target: next)» with «— and is owned by a `.andromeda/residuals.md` line that reads `re-carried:0.2.0`, the witness at an active assistive technology, which this version does not reach»"
    sidecar: "a11y-strategy — tree lifecycle: the windowed witness at the platform adapter now owned by the residuals line that reads `re-carried:0.2.0`, not «(target: next)»"
    rationale: "the report's `atowner` row names a11y-strategy.md:9 (its owner clause «and is owned by a `.andromeda/residuals.md` line (target: next)») as changed; the line reads `re-carried:0.2.0` (residuals.md:14), and requirements.md:42-43 says this version does not reach an active assistive technology"
    basis: .andromeda/residuals.md:14
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-strategy.md
    near: 21
    section: "## 2. A11y Strategy"
    change: "in the three-ARIA-states entity, replace «and the windowed witness, at an active assistive technology, is owned by a `.andromeda/residuals.md` line (target: next) on the founder's F1» with «and the windowed witness, at an active assistive technology, is owned by a `.andromeda/residuals.md` line that reads `re-carried:0.2.0`, which this version does not reach, on the founder's F1»; line 20's owner of the native checked-state fix, «(target: next)», stays as it is"
    sidecar: "a11y-strategy — the three ARIA states: the AT witness's owner corrected to the residuals line that reads `re-carried:0.2.0`; the native checked-state fix's owner unmoved"
    rationale: "the report's `atowner` row names a11y-strategy.md:21 as changed; the task's Untouched list keeps the native checked state fix's owner, the other half, unmoved"
    basis: escher-0.2.0/requirements.md:42
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 9
    section: "## 1. A11y Scope Summary"
    change: "replace the Dioxus crates bullet's close «— the adapter and its windowed AT witness owned by a `.andromeda/residuals.md` line (target: next).» with «— the adapter and its windowed AT witness owned by a `.andromeda/residuals.md` line that reads `re-carried:0.2.0`, an active assistive technology this version does not reach.»"
    sidecar: "a11y-plan §1 Dioxus crates: the AT witness's owner now the residuals line that reads `re-carried:0.2.0`"
    rationale: "the broad line a11y-scope-summary#1.9 details; the report's `atowner` row lists .andromeda/a11y-plan.md:9 as changed"
    basis: .andromeda/residuals.md:14
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 27
    section: "## 2. A11y Strategy"
    change: "replace the tree-lifecycle bullet's close «its windowed witness at the platform adapter owed — owned by a `.andromeda/residuals.md` line (target: next).» with «its windowed witness at the platform adapter owed — owned by a `.andromeda/residuals.md` line that reads `re-carried:0.2.0`, an active assistive technology this version does not reach.»"
    sidecar: "a11y-plan §2 tree lifecycle: the windowed witness's owner now the residuals line that reads `re-carried:0.2.0`"
    rationale: "the broad line a11y-strategy#1.1 details; the report's `atowner` row lists .andromeda/a11y-plan.md:27 as changed"
    basis: .andromeda/residuals.md:14
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 35
    section: "## 2. A11y Strategy"
    change: "replace «— that fix and the windowed AT witness owned by `.andromeda/residuals.md` lines (target: next).» with «— that fix owned by a `.andromeda/residuals.md` line (target: next), the windowed AT witness by one that reads `re-carried:0.2.0`, an active assistive technology this version does not reach.»"
    sidecar: "a11y-plan §2 role derivation: the AT witness's owner split out to the residuals line that reads `re-carried:0.2.0`; the native checked-state fix keeps «(target: next)»"
    rationale: "the broad line a11y-strategy#2.3 details; the report's `atowner` row lists .andromeda/a11y-plan.md:35 as changed, and the task moves only the AT witness half"
    basis: .andromeda/residuals.md:14
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 18
    section: "## 1. A11y Scope Summary"
    change: "add after the Dioxus crates entity: «- **Capability statement** — `docs/capabilities.md` («What Escher can do») publishes escher 0.2.0's accessibility reach under `## Accessibility reach`: tier 0, SC 2.1.1 · 1.4.3 · 2.4.3 and no other criterion, the `a11y` leg's fourteen files at 14 · 84 · 0 · 0 with no required status check, what an assistive technology reads unmeasured, and the named limits (the focus fixup not built, a `multiple` select and an `optgroup`'s options not modelled, `aria-activedescendant` unread, the native checked state never stated in the tree); it is the statement Viola's planning reads, through a pointer naming that path and commit `e7e71221d`; its pin `.github/scripts/test_capability_statement.py` checks the verb, cause and table counts and the README line only, none of the accessibility claims — as measured at escher-0.2.0/chunks/2026-10-25-capability-statement/report.md»"
    sidecar: "a11y-scope-summary — new entity: the capability statement publishes the a11y reach, its pin reading none of the a11y claims"
    rationale: "the task's F1; the report's Outcome (a11y) row states the statement's tier, leg tally, AT reading and limits, its Changes name the pin's four cases (none an accessibility claim), and Cross-project claims name the Viola pointer at e7e71221d"
    fact: F1
note: neither scoped detector fires — the report adds no interactive UI element (D-a11y-surface) and changes no a11y violation or obs log schema (D-a11y-obs-schema); the three body proposals mirror the detail ones and drop out if a body pass carries the same lines.
