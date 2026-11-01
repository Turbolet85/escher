proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 314
    section: "## Input Validation"
    change: Keep the line and its citation, and insert after «culled before painting» the clause ", `render_element`'s border box and culling standing on the scaled size since `e42ee1f25`, `location` and `box_position` kept for the transform; a box flagged `IS_INLINE_WITH_ONLY_BLOCK_CHILDREN` paints no shadow, background or border of its own, pinned by `inline_with_only_block_children_tests` 4, no frame read".
    sidecar: Paint values, Geometry - culling now stands on the scaled size (`e42ee1f25`), and a box flagged `IS_INLINE_WITH_ONLY_BLOCK_CHILDREN` skips its own shadow, background and border.
    rationale: The report's `e42ee1f25` entry moves `render_element`'s border box and culling onto the scaled size, a port, and its `f76777766` entry adds the skip through `paints_own_box`; the Coverage bullet records unit tests 4 and frame ✗ (report.md:19, report.md:22, report.md:106).
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 329
    section: "## Input Validation"
    change: Add after this line - **Boundary:** Paint values / - **What to validate:** `mix-blend-mode` and `isolation` from page CSS / - **How (stack-specific):** In every document, remote pages included, both now paint; the values pass Stylo's parse, `convert_blend_mode` maps the parsed value and `render_element` pushes a viewport-wide blend layer and pops it last; `Node`'s stacking-context arm reads `mix-blend-mode` other than `normal`; the added lines hold no log site; pinned at the layers by `layers.rs`'s 3 unit tests, no test painting a blended element to a frame or naming `convert_blend_mode` (packages/blitz-paint/src/blend.rs:5-26; packages/blitz-paint/src/render.rs:485-506; packages/blitz-dom/src/node/node.rs:1382-1384; packages/blitz-paint/src/layers.rs:104-181) — an engine change for every document, ratified — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
    sidecar: Paint values - a new row for `mix-blend-mode` and `isolation`, now painted from page CSS, validated by Stylo's parse, with no frame test.
    rationale: The report's `6fca6308d` entry makes both paint, and its Coverage bullet records validation by Stylo's parse, unit `layers.rs` 3 and ✗ for any frame test (report.md:15, report.md:104); D-security-input holds, the boundary being validated.
    basis: packages/blitz-paint/src/blend.rs:5-26
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 395
    section: "## Input Validation"
    change: Add after this line - **Boundary:** JS API arguments / - **What to validate:** `URLSearchParams` and `URL.prototype.searchParams`, `href` and `search` live both ways / - **How (stack-specific):** A page's script reaches them through `url_search_params.js`, first of the ten bootstrap scripts, a realization on the registered script crossing, no boundary widening; bad calls and receivers throw TypeError and values pass `usv`; the native `parse_search_params`, registered as `__blitz_parse_search_params`, is deleted from `globalThis` before any page script; no log site and no carrier; pinned by `url_search_params` 4 (packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4; packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:6-14; packages/blitz-vibey-script/src/runtime/bootstrap.rs:167-184; packages/blitz-vibey-script/tests/url_search_params.rs:61-78) — an engine change for every document, ratified — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
    sidecar: JS API arguments - a new row for `URLSearchParams` and `URL.prototype.searchParams`, a realization on the registered script crossing, validated by TypeError arms and `usv`, the native helper deleted before any page script.
    rationale: The report's `91ef65e21` entry and Coverage bullet name the surface and its validation, and Expected amendments owes §Input Validation this realization (report.md:17, report.md:96, report.md:102); D-security-input holds, the boundary being validated.
    basis: packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 449
    section: "## Input Validation"
    change: Add after this line - **Boundary:** Markup attributes / - **What to validate:** a custom widget's attributes at `set_custom_widget` / - **How (stack-specific):** Behind the `custom-widget` feature, `ElementData::set_custom_widget` replays each existing attribute, parsed page content included, to `Widget::attribute_changed` with no old value, so a widget receives them at set-up and not only on a later change, the value's check staying the widget's own, as the example's `color` parse above; pinned by `custom_widget_replays_initial_attributes` (packages/blitz-dom/src/node/element.rs:902-906; packages/blitz-dom/src/mutator.rs:1656-1727) — an engine change for every document behind the feature, ratified — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
    sidecar: Markup attributes - a new row for `set_custom_widget`'s replay of each existing attribute to `Widget::attribute_changed`, behind `custom-widget`.
    rationale: The report's `a26c6ddeb` entry adds the replay behind `custom-widget`, and New text lists its test (report.md:18, report.md:213).
    basis: packages/blitz-dom/src/node/element.rs:902-906
    fact: F8
note: F6 is left to the new-detail pass, no picked entity carrying it, and the task's touches on the Fetched stylesheets row and URL-handling row name rows not in this group's listing; report.md:58 places the limit of a focused control that no longer shows in a11y-plan §5, so it is not proposed here; D-security-deps and D-security-auth read clean, the report holding no dependency and no identity, token or key change.
