proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 54
    section: "## 2. Test Strategy"
    change: "Replace `148 `.rs` files, 147 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md (145 and 144 before` with `153 `.rs` files, 152 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md (148 and 147 before `visible_region_transformed.rs`, `hit_non_invertible_transform.rs`, `open_state.rs`, `checkable_change_event.rs` and `text_range_no_change_event.rs`, each its own auto-discovered target with no manifest line; 145 and 144 before`."
    sidecar: "2026-10-31-engine-debts: blitz-tests `.rs` files 148 to 153 and test targets 147 to 152, the five new checks each auto-discovered with no manifest line."
    rationale: "Report Changes, Counts: `blitz-tests` `.rs` files 148 to 153 and test targets 147 to 152, `rs-files` naming `test-strategy.md:54` as the one changed site; New checks (5) names the files; Harness / gate surface leaves `BlitzTestsTargetsTest`'s manifest unmoved."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:55
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 53
    section: "## 2. Test Strategy"
    change: "Replace `read through `mod fixtures;` by 24 checks — 22 `stand_*.rs` (` with `read through `mod fixtures;` by 28 checks (24 until 2026-10-31-engine-debts, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md) — of the 24, 22 `stand_*.rs` (`, the list of the 24 kept as it reads."
    sidecar: "2026-10-31-engine-debts: `tests/fixtures/mod.rs` readers 24 to 28; `session_common`'s 31 unmoved."
    rationale: "Report Changes, Counts: fixture readers 24 to 28 (`evidence/census.md`), `fixture-readers` naming `test-strategy.md:53` as changed and `test-strategy.md:52`'s `session_common` 31 as true."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:68
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 28
    section: "## 2. Test Strategy"
    change: "Replace `and states each limit as pinned by its check or as asserted by no check (the `select` model's limits);` with `and states each limit as pinned by its check or as asserted by no check, the `select` model's limits no longer that example: since 2026-10-31-engine-debts `select_interaction` pins Enter and Space opening nothing on a single select and `stand_act_select` an option under a closed select's group refused `covered`;`."
    sidecar: "2026-10-31-engine-debts: the `select` model's limits are no longer the example of a limit no check asserts; `select_interaction` and `stand_act_select` pin them."
    rationale: "Report Expected amendments for test-plan §2 carries `the select's limits now asserted`; Symbols / APIs F8 states both behaviours and New text lists `enter_and_space_open_nothing_on_a_single_select` and `an_option_under_a_closed_selects_group_is_refused_covered`. No sweep pattern named this site; found by this detector's own sweep."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:138
note: "Two points the report leaves open, neither source opened here: it does not name the four new `mod fixtures;` readers, which `evidence/census.md` holds, so the 28's list at line 53 stays partial until they are added; and it does not say whether `docs/capabilities.md`'s `The select model's other limits` (new text 208-211) stand as asserted by no check, so line 28's replacement makes no claim about them."
