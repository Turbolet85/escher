proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 240
    section: "## Standard Contracts"
    change: "The bootstrap clause reads: a JS bootstrap, `BOOTSTRAP_JS`, a `concat!` of ten `include_str!` scripts under `runtime/bootstrap/`, `url_search_params` first, defines history, fetch, DOMException, `URLSearchParams`, CSSOM classes and interface objects."
    sidecar: "JavaScript contract: `BOOTSTRAP_JS` moves from nine scripts to ten, `url_search_params` first, and its list of definitions gains `URLSearchParams`."
    rationale: "The report's take of `91ef65e21` puts `url_search_params.js` first in `BOOTSTRAP_JS`, ten scripts where nine stood, and its Counts mark standard-contracts.md:240 changed where it lists globals."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:46
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 177
    section: "## Standard Contracts"
    change: "The broad line's clause reads: a JS bootstrap defines history, fetch, DOMException, `URLSearchParams` and the CSSOM classes."
    sidecar: "JavaScript contract, broad line: the bootstrap's definitions gain `URLSearchParams`, matching the detail."
    rationale: "The broad line repeats the list of definitions that standard-contracts.md:240 details, and the report's `91ef65e21` adds a global class to that bootstrap."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:17
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 241
    section: "## Standard Contracts"
    change: "- `URLSearchParams` is a global class with `URL.prototype.searchParams`, `href` and `search` live both ways, a realization on the registered script crossing; its native `parse_search_params`, registered as `__blitz_parse_search_params`, is deleted from `globalThis` before any page script (packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4; packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:29-105; packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:123-146; packages/blitz-vibey-script/src/runtime/bootstrap.rs:41-46; packages/blitz-vibey-script/src/runtime/bootstrap.rs:167-184)"
    sidecar: "JavaScript contract: adds the `URLSearchParams` global, its live `URL` properties and the native helper deleted before any page script."
    rationale: "The report's `91ef65e21` adds this page-visible surface, and the JavaScript contract is the entity that lists the bootstrap's globals."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:17
    fact: F21
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 266
    section: "## Standard Contracts"
    change: "The clause reads: `DioxusDocument` exposes `vdom`, `inner`, `vdom_state.try_element_to_node_id(ElementId)`, which `flush_queued_mounted_events` also reads to skip an element removed since its event was queued."
    sidecar: "Dioxus DOM bridge: `try_element_to_node_id` gains the mounted-event guard as a reader."
    rationale: "The report's Counts mark standard-contracts.md:266 changed for the guard's reader, through `6ef4b532d`."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:56
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 284
    section: "## Standard Contracts"
    change: "- `flush_queued_mounted_events` skips an element removed since its event was queued (packages/dioxus-native-dom/src/dioxus_document.rs:235-239); upstream's test `element_removed_before_its_mounted_event_is_flushed` is left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0."
    sidecar: "Dioxus DOM bridge: adds the mounted-event guard and the upstream test left out on Dioxus 0.7."
    rationale: "The report's `6ef4b532d` adds the guard and leaves the test out, and the bridge entity holds the mounted-event and element-mapping contract."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:20
    fact: F22
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 324
    section: "## Standard Contracts"
    change: "The leg list opens: `.github/scripts/ci-leg.sh {leg}` — legs `embolden` (no CI job, the last step of `fast`) · `fmt` · `clippy`, the rest of the list as it stands."
    sidecar: "CI contracts: the leg list gains `embolden`."
    rationale: "The report's Harness bullet adds `embolden` to `LEGS` and its Counts mark standard-contracts.md:324 changed."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:41
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 327
    section: "## Standard Contracts"
    change: "The `fast` clause reads: `fast` (fmt → clippy → test → ci-scripts → embolden, stopping at the first red: the local pre-push gate)."
    sidecar: "CI contracts: `fast` gains `embolden` as its last step."
    rationale: "The report states the `fast` order as fmt → clippy → test → ci-scripts → embolden and marks standard-contracts.md:327 changed."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:41
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 333
    section: "## Standard Contracts"
    change: "- `.github/scripts/ci-leg.sh` holds the leg `embolden`, `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, with no CI job, macOS exercising the feature there, pinned by `test_fast_leg_ends_on_the_font_embolden_witness` (.github/scripts/ci-leg.sh:8-9; .github/scripts/ci-leg.sh:28-29; .github/scripts/test_ci_workflows.py:361-375)"
    sidecar: "CI contracts: adds the `embolden` leg's command, its lack of a CI job and its pin."
    rationale: "The report's Harness bullet adds this leg and pin, and the CI contracts entity holds the legs of `ci-leg.sh` and their pins."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:64
    fact: F23
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 333
    section: "## Standard Contracts"
    change: "- `test_only_stand_repeat_frees_disk` pins that only the `stand-repeat` job frees disk (.github/scripts/test_ci_workflows.py:110-115)"
    sidecar: "CI contracts: adds the pin that only the `stand-repeat` job frees disk."
    rationale: "The report's Harness bullet adds this pin beside the removal of `matrix_test`'s free-disk step, and the CI contracts entity holds the workflow pins."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:64
    fact: F24
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 334
    section: "## Standard Contracts"
    change: "The line reads, with no owner clause: `blitz-paint/font-embolden` is built locally by the `embolden` step of `fast`, `--features blitz-paint/font-embolden` on `cargo test -p blitz-tests` the local witness for a paint change under that feature, `--test select_popup_painted` so reproducing macOS's two readings on linux (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md); no CI job runs that leg, CI's macOS leg exercising the feature there."
    sidecar: "CI contracts: the font-embolden gap closes as the `embolden` step of `fast`, and its owner clause goes because the owed thing landed."
    rationale: "The report's Retired bullet names this gap in §Standard Contracts, its Counts read the line as now the `embolden` step of `fast`, and its owner rows are answered with nothing where the owed thing landed."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:33
note: "F11 to F20 fit no entity picked in this group, so the new-detail pass places them; the report does not state where `embolden` sits in `LEGS`, so its first place in the list at line 324 is for reading and claims no order."
