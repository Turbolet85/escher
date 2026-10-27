proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 33
    section: "## Conventions"
    change: "In the Visibility and naming bullet, replace \"blitz-vibey-script re-exports only `ScriptDocument`, `DefaultScriptFetcher`, `FetchError`, `ScriptFetcher`\" with \"blitz-vibey-script re-exports only `ScriptDocument`, `DefaultScriptFetcher`, `FetchError`, `ScriptFetcher` and, since 2026-10-27-upstream-sync-engine-debts, upstream's `document_may_load`, the check that lets a `file:` URL load only for a `file:` document\", the rest of the bullet unchanged"
    sidecar: "conventions › Visibility and naming: blitz-vibey-script's crate-root re-exports gain `document_may_load` (upstream `cee6b8731`, taken by the 2026-10-27 upstream sync on the founder's word)"
    rationale: "Report Changes › Symbols / APIs: `pub fn document_may_load(document_url: Option<&Url>, url: &Url) -> bool` (`fetch.rs:40-46`), new and public, re-exported at the added line `lib.rs:43` (New text, by line: blitz-vibey-script `lib.rs` added 43, inside the cited re-export range); `FetchError` was already re-exported and only gains a variant, so the list of four names stops being the whole list"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:16
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 53
    section: "## Conventions"
    change: "In the Tests bullet, after \"scenarios run in both layout modes `for incremental in [false, true]` (…)\" add \", except upstream's `pseudo_element_flow_classification` (taken 2026-10-27-upstream-sync-engine-debts, three tests), which does not assert both layout modes, `incremental_oracle` green beside it\", the rest of the bullet unchanged"
    sidecar: "conventions › Tests: the both-layout-modes convention records upstream's `pseudo_element_flow_classification` target as asserting one mode (2026-10-27 upstream sync)"
    rationale: "Report Changes › Coverage of new surfaces: pseudos voting in flow classification → tests integ (`pseudo_element_flow_classification` 3, both layout modes not asserted by the file; `incremental_oracle` green); Crates / modules: blitz-tests gains that auto-discovered target; without the carve-out the bullet reads as if every scenario file in blitz-tests loops both modes"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:87
note: conventions#4.1's `// === … ===` delimiter claim cites `runtime.rs:2056` and `dom/element.rs:224`, whose code moved with the split (the vibey-mods citation rows); whoever re-points them should confirm the delimiters survived the move. The both-modes rule in CLAUDE.md's Critical Warnings and test-plan is test-plan's to weigh against the new target, not this group's.
