proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 259
    section: "## Standard Contracts"
    change: "In the snapshot's `bounds` clause, `BaseDocument::get_client_bounding_rect` stays the reader «for every element» but a row or group of an open list, whose `bounds` the builder takes first from `BaseDocument::select_list_bounds(node)`, its place in the painted list; with no list open the clause stands as written."
    sidecar: "Snapshot `bounds`: an open list's row or group reads `select_list_bounds` before the client rect."
    rationale: "Report, Changes › Symbols / APIs › Bridge: the snapshot's builder reads `select_list_bounds(node)` before the client rect (`snapshot.rs:152-155`), so «for every element» no longer holds where a list stands; Schema / config: a row's own `bounds` crosses in the snapshot and the diff."
    basis: "packages/dioxus-native-dom/src/snapshot.rs:152-155"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 278
    section: "## Standard Contracts"
    change: "- `is_select`, public and re-exported from dioxus-native-dom: the driver's select test, its one caller `locate` in `execute.rs`; and the snapshot's builder reads `select_list_bounds(node)` before the client rect for a row of an open list (packages/dioxus-native-dom/src/lib.rs:37; packages/dioxus-native-dom/src/snapshot.rs:152-155)."
    sidecar: "Bridge: new public `is_select`; the builder's `select_list_bounds` read recorded."
    rationale: "Report, Changes › Symbols / APIs › Bridge and Driver: new public `is_select`, re-exported, `execute.rs` `locate` its one caller; the task's F18."
    basis: "packages/dioxus-native-dom/src/lib.rs:37"
    fact: F18
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 279
    section: "## Standard Contracts"
    change: "The option clause keeps a closed select's rule and gains: `closed_select_of` answers `None` for a select whose list is open, so a row reads its own `bounds` from `BaseDocument::select_list_bounds` while the list stands and its select's client rect again once the list closes; `closed_select_of` is the one caller of `is_multiple_select` within the workspace, which stays re-exported."
    sidecar: "Bridge: `closed_select_of` answers `None` for an open list; `is_multiple_select`'s one workspace caller stated."
    rationale: "Report, Changes › Symbols / APIs › Bridge and Driver, and the `closed-option` row naming standard-contracts:279; `stand_act_popup`'s `escape_closes_with_the_value_unchanged_and_the_rows_back_at_the_selects_box` holds the return to the select's box."
    basis: "packages/dioxus-native-dom/src/snapshot.rs:152-155"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 279
    section: "## Standard Contracts"
    change: "- The snapshot carries no `expanded` node key, by the operator's call (inputs#I6), and `NodeState` gains no field for the list, `NODE_FIELDS` staying thirteen."
    sidecar: "Snapshot: no `expanded` node key and no `NodeState` field for the list, recorded."
    rationale: "Report, Changes › Symbols / APIs › Bridge: `NodeState` gains no field, `NODE_FIELDS` stays thirteen; Decisions & corrections: no `expanded` node key, inputs#I6; the task's F19."
    fact: F19
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 312
    section: "## Standard Contracts"
    change: "«fifteen named blitz-tests targets outside that prefix» reads «seventeen named blitz-tests targets outside that prefix», `select_popup` and `select_popup_painted` joining them and `test_stand_repeat_leg_runs_three_rounds` asserting both; `stand_act_popup` joins by the `stand_*.rs` prefix."
    sidecar: "CI contracts: `stand-repeat` names seventeen targets outside the prefix, fifteen until 2026-10-31-select-popup."
    rationale: "Report, Counts / qualifiers moved › `repeat-names`, which names standard-contracts:312 (`ci-leg.sh:59-60`); Harness / gate surface: `test_ci_workflows.py:367` asserts the two words and `stand_act_popup` joins by its prefix."
    basis: ".github/scripts/ci-leg.sh:59-60"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 319
    section: "## Standard Contracts"
    change: "- No local gate builds `blitz-paint/font-embolden`, which CI's macOS leg alone exercises; `--features blitz-paint/font-embolden` on `cargo test -p blitz-tests` is the local witness for a paint change under that feature, `--test select_popup_painted` so reproducing macOS's two readings on linux (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "CI contracts: no local gate builds `font-embolden`; its local witness recorded."
    rationale: "Report, Decisions & corrections: a local gate block read green over a defect only macOS showed, and the flag reproduced macOS's two numbers on linux; Spec claims disproved › `os-sensitive`; the task's F25."
    fact: F25
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture.md
    near: 183
    section: "## Standard Contracts"
    change: "«the public `closed_select_of` and `is_multiple_select` are the select tests the driver reads» reads «the public `closed_select_of` and `is_select` are the select tests the driver reads», `is_multiple_select` stays public with no driver reader, and the line's last clause gains that an open list's rows read their places in the list."
    sidecar: "Bridge's broad line: `is_select` registered as the driver's select test; an open list's rows read their list bounds."
    rationale: "Report, Changes › Symbols / APIs › Driver: `is_select` is read by `execute.rs` `locate` and `is_multiple_select` by `closed_select_of` alone within the workspace, still re-exported; Counts › `focus-select` and `closed-option` both name architecture:183; a new public symbol the body's registry does not name."
    basis: "packages/dioxus-native-dom/src/lib.rs:37"
