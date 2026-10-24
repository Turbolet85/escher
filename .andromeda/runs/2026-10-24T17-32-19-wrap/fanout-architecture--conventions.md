proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 61
    section: "## Conventions"
    change: "In the `fixtures/mod.rs` item, after \"and a page with one reader stays in its check's file\" add \"— a page moves in, unchanged, once a second check reads it, as `ITEMS` and `many_items` did from `stand_snapshot_parts` when `stand_act_spans` became their second reader (since 2026-10-24-stand-requirement-sweep)\" and add the cite `tests/blitz-tests/tests/fixtures/mod.rs:64-86` to the item's citation list."
    sidecar: "conv-tests: the shared fixture module's page rule now cites `many_items`/`ITEMS`, moved in from `stand_snapshot_parts` when `stand_act_spans` became their second reader (2026-10-24-stand-requirement-sweep)."
    rationale: "The report's Changes say `fixtures/mod.rs` gained `pub const ITEMS` and `pub fn many_items()` (64-86), \"moved unchanged from `stand_snapshot_parts.rs`\", and that `stand_act_spans` now reads `fixtures::many_items` too. That is the module's page rule applied, and it is measured evidence for it. The sweep's fact 10 left conventions.md:61 unchanged because the line lists no pages. This proposal adds only the rule's citation, not a page list."
    basis: tests/blitz-tests/tests/fixtures/mod.rs:64-86
note: "D-arch-resources and D-arch-decisions find nothing for Conventions: the report lands no crate, dependency, env var, listener or verb (its two new test state directories `hl-client` and `hl-client-rec` belong to occupied-resources.md:25, fact 9), and Retired is none. Line 61's existing cite `fixtures/mod.rs:87-130` already reads past the 23 inserted lines."
