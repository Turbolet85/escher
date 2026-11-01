proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 58
    section: "## Existing Scopes"
    change: "blitz-paint's module list opens with the new private module: `Modules blend (private, its `pub(crate) convert_blend_mode` converting Stylo's blend mode: packages/blitz-paint/src/blend.rs:5-26), color, debug_overlay, filters, ...`, the rest of the record as it stands."
    sidecar: "existing-scopes: blitz-paint's modules gain the private `blend`, from `6fca6308d`."
    rationale: "report.md:29 names the new private module `blend` in blitz-paint and report.md:15 its `convert_blend_mode`; report.md:95 expects `blend` among blitz-paint's modules. F26's other half, `url_search_params`, lands by the two D-cascade proposals at lines 61 and 146."
    basis: ".andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:29"
    fact: F26
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 61
    section: "## Existing Scopes"
    change: "blitz-vibey-script's integration tests read `tests/dom.rs, tests/preact.rs, tests/selection.rs and tests/url_search_params.rs`, the last 4 tests of `URLSearchParams` and `URL.prototype.searchParams` (packages/blitz-vibey-script/tests/url_search_params.rs:15-95)."
    sidecar: "existing-scopes: blitz-vibey-script's integration tests gain `url_search_params`, 4 tests, from `91ef65e21`."
    rationale: "report.md:29 names the new test target `url_search_params` in blitz-vibey-script, report.md:102 its 4 tests, and report.md:95 expects it among blitz-vibey-script's tests."
    basis: ".andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:102"
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 137
    section: "## Existing Scopes"
    change: "`stand_cells_served`'s screen reads `47,089 bytes in 740 lines walked in 5 parts` in place of 47,949 bytes, and the closing measured-at clause gains `, the 47,089 at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md`."
    sidecar: "existing-scopes: Cells' screen 47,949 becomes 47,089 bytes, measured on the base and this tree, stale since an earlier chunk and not moved by this take."
    rationale: "report.md:87 measures 47,089 bytes, 740 lines, 5 parts through the binary on a scratch clone at the base and on this tree, and disposes: amend all three sites, this one flat."
    basis: ".andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:87"
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 146
    section: "## Existing Scopes"
    change: "`with ten JS bootstrap scripts under `runtime/bootstrap/` — `url_search_params`, which `BOOTSTRAP_JS` takes first, and `css`, `dom_exception`, `fetch`, `fonts`, `interfaces`, `node_constants`, `style`, `stylesheets`, `window` — concatenated into `BOOTSTRAP_JS``, the rest of the record as it stands."
    sidecar: "existing-scopes: nine JS bootstrap scripts become ten, `url_search_params` first in `BOOTSTRAP_JS`, from `91ef65e21`."
    rationale: "report.md:46 reads `existing-scopes.md:146` ×2 nine to ten, `url_search_params` first, changed; report.md:17 places the script first in `BOOTSTRAP_JS`."
    basis: ".andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:46"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 153
    section: "## Existing Scopes"
    change: "The record's list gains, before its final period: `; `key_press_after_container_lost_its_box`, 3 tests — a key on a control whose anonymous block was dropped, by `hidden`, `style=\"display:none\"` or CSS alone, not panicking, a button's and a select's, the select's choice still moving past its dropped block, and a control in no anonymous block reading the same; no `stand_` prefix (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-187)`."
    sidecar: "existing-scopes: blitz-tests' engine-fact targets gain `key_press_after_container_lost_its_box`, 3 tests, the panic's check."
    rationale: "report.md:27 names the new check, 3 tests in both layout modes with no `stand_` prefix; report.md:23 and report.md:114 state what it holds; report.md:95 owes it a new sentence in existing-scopes, the check named nowhere."
    basis: ".andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:27"
    fact: F27
note: F26 is one proposal at line 58 for its `blend` half, its `url_search_params` half standing on the D-cascade proposals at lines 61 and 146; no body line of §Existing Scopes moves.
