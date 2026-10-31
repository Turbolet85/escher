proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 13
    section: "## 8. PII Scrubbing & Compliance"
    change: "After line 13 add the item: - A `multiple` select's chosen set leaves only as form entries on `input` and `change`, one `(name, FormValue::Text(value))` per chosen option (`multiple_select_values`), as the snapshot's `value`, which reads `select_value`, the first chosen option in tree order, and as the options' tree nodes, never as a log line or field: the added lines hold no log site, print or env read, the added-lines census reading 0 (packages/dioxus-native-dom/src/dioxus_document.rs:31-45; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)."
    sidecar: "pii-scrubbing-compliance: states where a `multiple` select's chosen set crosses, as form entries, the snapshot's `value` and tree nodes, and that it is never logged or fielded (2026-10-31-engine-debts)."
    rationale: "F6: the report's Coverage of new surfaces gives the set's exits for the `multiple` select, Symbols / APIs gives `multiple_select_values` and `value` reading `select_value`, and Schema / config gives entry 4 at 0 on the added lines."
    basis: packages/dioxus-native-dom/src/dioxus_document.rs:31-45
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 13
    section: "## 8. PII Scrubbing & Compliance"
    change: "After line 13 add the item: - An `optgroup`'s non-empty `label` becomes its `Role::Group` name in the accessibility tree, inside the ratified name crossing, and it is not painted; the added lines hold no log site, print or env read, the added-lines census reading 0 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)."
    sidecar: "pii-scrubbing-compliance: states that an `optgroup`'s `label` crosses as a `Role::Group` name inside the ratified name crossing, unpainted (2026-10-31-engine-debts)."
    rationale: "F7: the report's Symbols / APIs F7 gives the `Role::Group` name from the non-empty `label` and the unpainted label, and Coverage of new surfaces places it inside the ratified name crossing."
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 13
    section: "## 8. PII Scrubbing & Compliance"
    change: "After line 13 add the item: - The engine's `change` on a checkbox or radio, sent after `input` with the same value, a checkbox always and a radio only where it was not checked before the click, carries the value class `input` already carries; a page's script now hears one `change`, from the engine, the script runtime's synthesis retired; the added lines hold no log site, print or env read, the added-lines census reading 0 (packages/blitz-dom/src/events/pointer.rs:793-799; packages/blitz-dom/src/events/pointer.rs:811; packages/blitz-dom/src/events/pointer.rs:827-836; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)."
    sidecar: "pii-scrubbing-compliance: states that the engine's `change` on a checkbox or radio carries the value class of `input`, and that a page's script hears one `change` (2026-10-31-engine-debts)."
    rationale: "F8: the report's Symbols / APIs F10 gives the sender, the same value and the one `change` heard, Retired names the runtime's synthesis, and Coverage of new surfaces gives the value class."
    basis: packages/blitz-dom/src/events/pointer.rs:793-799
    fact: F8
note: "No scoped detector fired for this group: the report adds no log site touching user data, and the free-disk step adds no upload, so the two artifacts at .andromeda/obs-plan.md:282 stand."
