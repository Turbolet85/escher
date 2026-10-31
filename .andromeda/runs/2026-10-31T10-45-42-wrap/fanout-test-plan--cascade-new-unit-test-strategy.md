proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 63
    section: "## 4. Unit Test Strategy"
    change: "- **blitz-dom (form):** since 2026-10-31-engine-debts `form.rs`'s standing test module gains two tests over the select's submitted entries — `a_multiple_select_submits_its_set_less_its_disabled_rows`, a `multiple` set less its disabled rows, and `a_single_select_submits_its_choice_through_a_group`, a single choice through a group — and these use neither `.unwrap()` nor `.expect(` (packages/blitz-dom/src/form.rs:558-592; packages/blitz-dom/src/form.rs:594-617) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts: new blitz-dom (form) group, the two tests over the select's submitted entries in form.rs's test module"
    rationale: "The report's Crates / modules line says form.rs's test module already stood and gained two tests, its New text, by line section names them, and its Decisions & corrections says the new unit tests under packages/ use neither .unwrap() nor .expect(; no group of this file holds a form.rs test, so a new group sits beside the blitz-dom (node) group"
    basis: "packages/blitz-dom/src/form.rs:558"
    fact: F19
