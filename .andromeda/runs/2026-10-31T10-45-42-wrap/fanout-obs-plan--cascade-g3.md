proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 97
    section: "## 6. Log Coverage"
    change: "Add under **blitz-shell, blitz-paint**: `  - `packages/blitz-shell/src/event.rs` gained the crate's first unit test, an inline `#[cfg(test)]` module, held out of the added-lines census, so that census's 0 does not speak for it (packages/blitz-shell/src/event.rs:128-172; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)`"
    sidecar: "log-coverage: blitz-shell's first unit test recorded, held out of the 2026-10-31-engine-debts added-lines census."
    rationale: "The report's Crates / modules bullet names the test module at `:128-172` and its Schema / config bullet holds `blitz-shell/src/event.rs` out of entry 4, so a reader of that census's 0 needs the carve-out beside the crate's log sites."
    basis: packages/blitz-shell/src/event.rs:128-172
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 126
    section: "## 6. Log Coverage"
    change: "Add after the entry: `- observed absent — a silence-census hit in the five new checks `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event` · searched: the silence census `grep -lE 'println!…'` over the chunk's checks, exit 1, its last line 0 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md, its gate block)`"
    sidecar: "log-coverage: the five new 2026-10-31-engine-debts checks recorded silent."
    rationale: "The report's Outcome gate block reads the silence census at exit 1, last line 0, and its New checks bullet names these five targets; no standing entity records them."
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 126
    section: "## 6. Log Coverage"
    change: "Add after the entry: `- observed absent — a log site, print, env read, listener, wall-clock read or panic path on the added lines of the transformed `visible_region` read, the non-invertible hit answer, the inert focus fixup, the tree's focus fallback, native `toggled`, the `multiple` select, `optgroup` and the engine's checkable `change` · searched: the added-lines census, entry 4, reading 0, `packages/blitz-shell/src/event.rs` held out, its addition an inline test (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)`"
    sidecar: "log-coverage: the 2026-10-31-engine-debts engine changes recorded as adding no log site, print, env read, listener, wall-clock read or panic path."
    rationale: "The report's Schema / config bullet states entry 4's reading over the added lines, and its Coverage of new surfaces rows mark these engine changes instrumentation n/a or no log site added."
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 126
    section: "## 6. Log Coverage"
    change: "Add after the entry: `- observed absent — a log site, field or print in the new readers `BaseDocument::select_values`, `select_of` and `clear_select_typeahead` and `dioxus_native_dom::closed_select_of` and `is_multiple_select`, instrumentation n/a · searched: the added-lines census, entry 4, reading 0 (packages/blitz-dom/src/document.rs:736-744; packages/blitz-dom/src/document.rs:746-750; packages/blitz-dom/src/document.rs:752-755; packages/dioxus-native-dom/src/snapshot.rs:242-252; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)`"
    sidecar: "log-coverage: the five new public readers of 2026-10-31-engine-debts recorded as adding no log site, field or print."
    rationale: "The report's Coverage of new surfaces row for these readers reads instrumentation n/a, its Schema / config bullet states entry 4 at 0, and its Symbols / APIs bullets and New text list state the coordinates."
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 126
    section: "## 6. Log Coverage"
    change: "Add after the entry: `- observed absent — a log site, print or env read in Cells' `cells-grid` Tab stop, key handler and `MountedData` handles and its `.cell.current` outline, instrumentation n/a · searched: the report's reading of the chunk's added lines, entry 4 at 0 (examples/seven_guis/src/tasks/cells.rs:551-554; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)`"
    sidecar: "log-coverage: Cells' `cells-grid` keyboard surface recorded as adding no log site, print or env read."
    rationale: "The report's Coverage of new surfaces row for `cells-grid` reads instrumentation n/a, its Schema / config bullet states no log site, print or env read on the added lines, and its Symbols / APIs bullet places the outline at `:551-554`."
    fact: F5
note: "No standing log-coverage claim moved and no scoped detector fired; the body's silence block at obs-plan.md:87-98, including the reader count at `:92`, lies outside this group's entities."
