proposals:
  - detector: D-new-detail
    severity: warning
    fact: F1
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 125
    section: ## 6. Log Coverage
    change: "- observed absent — a print, an env read or a subscriber in upstream's blitz-tests target `pseudo_element_flow_classification` (3 tests), its readings going into assertions only · searched: the silence census over the file, reading 0 · 0 · 0 (env · print · subscriber) (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)"
    sidecar: "§6 Log Coverage absence · logs: upstream's new target `pseudo_element_flow_classification` recorded silent (0 env · 0 print · 0 subscriber), 2026-10-27-upstream-sync-engine-debts"
    rationale: "The report's Crates / modules bullet adds the auto-discovered target `pseudo_element_flow_classification`, its Counts bullet `silence` reads it 0 env · 0 print · 0 subscriber, and Outcome's sink re-check reads the new test 0 · 0 · 0; the block's neighbours record each chunk's test additions' silence in this form"
  - detector: D-new-detail
    severity: warning
    fact: F2
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 125
    section: ## 6. Log Coverage
    change: "- observed absent — a log site, span, field, log field or env read in source added by the upstream take read at `ebacd68bd` (eleven commits): `:open` matching (`ElementData::flush_open_state`), the `::before`/`::after` vote in flow classification, IME enabled at a focused input's editor construction, the `file:` refusal (`document_may_load`), the `CSSStyleDeclaration` property accessors and the windowed shell's redraw route · searched: the fixed-sha census over the eleven commits' `packages/*.rs`, reading its two named hits, and 0 wall-clock reads in blitz-dom outside `clock.rs`; `packages/escher-telemetry/` and `packages/escher-driver/` byte-unchanged, `host_log` and `mcp_log` green under both builds (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/sink.md)"
    sidecar: "§6 Log Coverage absence · logs: the upstream take read at `ebacd68bd` recorded silent — its six engine and script changes add no log site, span, field, log field or env read in source, 2026-10-27-upstream-sync-engine-debts"
    rationale: "The report's Symbols / APIs bullets name the six changes and state no new env read in source, print or log site; Outcome's sink re-check reads the census's two named hits, 0 wall-clock reads outside `clock.rs`, `host_log` and `mcp_log` green under both builds and escher-telemetry unchanged; the unchanged-and-re-read bullet states escher-telemetry and escher-driver byte-unchanged (evidence/sink.md)"
  - detector: D-new-detail
    severity: warning
    fact: F4
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 125
    section: ## 6. Log Coverage
    change: "- observed absent — a print or log site in blitz-vibey-script `tests/dom.rs`'s three added tests (27 → 30 `#[test]`); its one added env read is test-only: `temp_dir_with_files` reads `TMPDIR` through `std::env::temp_dir()`, test code only and not secret, the second test-only `TMPDIR` site beside escher-driver's client unit test (packages/blitz-vibey-script/tests/dom.rs:863; as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/sink.md)"
    sidecar: "§6 Log Coverage absence · logs: blitz-vibey-script `tests/dom.rs`'s additions recorded with no print or log site and one test-only `TMPDIR` read (`temp_dir_with_files`), 2026-10-27-upstream-sync-engine-debts"
    rationale: "The report's Symbols / APIs bullet states no new print or log site and one new test-only env read, `TMPDIR` through `std::env::temp_dir()` in `tests/dom.rs` `temp_dir_with_files` (the call at `:863`), a second site beside escher-driver's client unit test; its Counts bullets `dom-rs` (27 → 30) and `tmpdir` (1 → 2 sites); the block's silence entries state each addition's env reads beside its silence, as the `mcp_tests.rs` entry does"
note: F1 and F2 restate in this file's §6 absence block what the Touches extend in the body's §3 silence census (obs-plan.md:87-98, the take-read entry at :98) — if P2 lands the new target and the take's silence there, drop F1 and F2 here as duplicates; F4's registration home is architecture §Occupied Resources and security-plan's environment values (the report's `tmpdir` pattern), this entry only the silence record beside it.
