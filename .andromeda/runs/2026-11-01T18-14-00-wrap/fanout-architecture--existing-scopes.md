proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 58
    section: "## Existing Scopes"
    change: "blitz-paint's module list opens with the new private module: Modules blend (private, since 2026-11-01-upstream-sync-ui-graph, its `pub(crate)` `convert_blend_mode` turning Stylo's blend mode into the paint `BlendMode`: packages/blitz-paint/src/lib.rs:6; packages/blitz-paint/src/blend.rs:5-26), color, debug_overlay, and the rest of the list as it stands."
    sidecar: "existing-scopes: blitz-paint's modules gain the private `blend`."
    rationale: "The report (.andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md) at :29 names a new private module `blend` in blitz-paint, at :15 its `convert_blend_mode`, and at :95 carries `blend` among blitz-paint's modules to this section; line 58's list lacks it."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:29
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 61
    section: "## Existing Scopes"
    change: "After the integration-test list and its citations, add: and, since 2026-11-01-upstream-sync-ui-graph, tests/url_search_params.rs, 4 tests: construction and form encoding, mutations and live iteration, invalid inputs and receivers, and `URL.prototype.searchParams` live both ways with `href` and `search` (packages/blitz-vibey-script/tests/url_search_params.rs:15-95)."
    sidecar: "existing-scopes: blitz-vibey-script's integration tests gain `url_search_params`, 4 tests."
    rationale: "The report (.andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md) at :29 names the new test target `url_search_params` in blitz-vibey-script, at :102 its 4 tests, and at :95 carries it among that crate's tests to this section; line 61 lists three test files."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:95
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 137
    section: "## Existing Scopes"
    change: "`stand_cells_served`'s screen reads 47,089 bytes in 740 lines walked in 5 parts, where it read 47,949 bytes; the closing provenance adds: the 47,089 at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md."
    sidecar: "existing-scopes: the stale Cells screen figure corrected, 47,949 to the measured 47,089 bytes."
    rationale: "The report (.andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md) at :87 measures Cells through the binary at the base and on this tree at 47,089 bytes, 740 lines, 5 parts, names this line as stating 47,949 flat, and disposes: amend all three. The take did not move it; the reading was stale since an earlier chunk."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:87
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 146
    section: "## Existing Scopes"
    change: "The bootstrap clause reads: with ten JS bootstrap scripts under `runtime/bootstrap/` — `url_search_params`, first in the concatenation, `css`, `dom_exception`, `fetch`, `fonts`, `interfaces`, `node_constants`, `style`, `stylesheets`, `window` — concatenated into `BOOTSTRAP_JS`, `url_search_params`'s native `parse_search_params` registered as `__blitz_parse_search_params` and deleted from `globalThis` before any page script (packages/blitz-vibey-script/src/runtime/bootstrap.rs:167-184; packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4), geometry kept a separate `GEOMETRY_JS`; the rest of the line as it stands."
    sidecar: "existing-scopes: blitz-vibey-script's bootstrap scripts nine to ten, `url_search_params` first, its native helper named."
    rationale: "The report (.andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md) at :46 reads this line nine to ten, `url_search_params` first, changed; at :17 places the script first in `BOOTSTRAP_JS` with its native helper registered and deleted before any page script; at :95 carries it."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:46
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 153
    section: "## Existing Scopes"
    change: "New item after this line: - blitz-tests' `key_press_after_container_lost_its_box`, one file and one target, 3 tests in both layout modes: a key on a button whose anonymous block was dropped does not panic, a key on a select past its dropped block still moves its choice, and a key on a control in no anonymous block reads the same, the box lost by `hidden`, inline `display:none` and a sheet's rule alone; no `stand_` prefix and no `a11y` or `stand-repeat` entry, so the workspace test leg and `run all` run it (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-187 — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)."
    sidecar: "existing-scopes: blitz-tests gains the new check `key_press_after_container_lost_its_box`, 3 tests."
    rationale: "The report (.andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md) at :27 names the new check, 3 tests in both layout modes, no `stand_` prefix; at :23 and :114 its behaviour; at :36 and :60 places it in the workspace tally and in no repeat leg; at :95 owes a new sentence, the check named nowhere."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:95
note: "This file took two Write calls, not the one asked: the second dropped a phrase in the line-153 proposal that claimed more than the check asserts, and made rationale paths repo-relative. No Retired item stands in this file; line 79's harness token and line 105's net-error citation read no-change at .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:39 and :48; D-arch-resources and D-arch-decisions fire on nothing in this category, no dependency moved and the test-only binds owed to Occupied Resources."
