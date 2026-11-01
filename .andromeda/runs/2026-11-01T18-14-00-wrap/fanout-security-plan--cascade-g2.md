proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 113
    section: '## Input Validation'
    change: 'On the Fetched stylesheets row''s How line, after its present sentence, add: Since 2026-11-01-upstream-sync-ui-graph a failed, empty or non-CSS stylesheet stops blocking render: a load that errs reaches `ResourceHandler<StylesheetHandler>::error`, which responds `Err(error)`, so `pending_critical_resources` loses the request; pinned by `failed_stylesheet_unblocks_rendering` and by `empty_or_non_css_stylesheet_unblocks_rendering` over an empty body, an HTML page and the byte `\xff` (packages/blitz-dom/src/net.rs:144-146; tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-67; tests/blitz-tests/tests/render_blocking_stylesheet.rs:69-91) — an engine change for every document — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md'
    sidecar: 'Input Validation, Fetched stylesheets: a failed, empty or non-CSS stylesheet no longer blocks render (upstream f6edb9701).'
    rationale: 'report.md:14 has `f6edb9701` answer a failed load with `Err(error)` and a failed, empty or non-CSS stylesheet stop blocking render; the row states only the encoding check, which still holds, and not what the failed load now does.'
    basis: packages/blitz-dom/src/net.rs:144-146
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 107
    section: '## Input Validation'
    change: 'Fetched stylesheets — must be valid UTF-8, else the load errs; a failed, empty or non-CSS stylesheet no longer blocks render.'
    sidecar: 'Input Validation body, Fetched stylesheets: the broad line takes the same sentence.'
    rationale: 'The broad line the Fetched stylesheets row details, the same move as report.md:14.'
    basis: packages/blitz-dom/src/net.rs:144-146
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 113
    section: '## Input Validation'
    change: 'Add after the Fetched stylesheets row, three lines in its shape: - **Boundary:** Stylesheet `href` / - **What to validate:** URL / - **How (stack-specific):** In every document, remote pages included: an `href` that cannot be resolved starts no load and does not panic, `doc.url.resolve_relative` standing where `resolve_url` stood on this path, and so blocks no render; pinned by `invalid_stylesheet_url_does_not_block_rendering` (packages/blitz-dom/src/mutator.rs:1392-1394; tests/blitz-tests/tests/render_blocking_stylesheet.rs:93-107) — an engine change for every document — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md'
    sidecar: 'Input Validation: new row, a stylesheet `href` that cannot be resolved starts no load, no panic (upstream f6edb9701).'
    rationale: 'report.md:14 states a bad `href` starts no load, `resolve_url` giving way to `doc.url.resolve_relative`; no row of the stylesheet boundary holds it.'
    basis: packages/blitz-dom/src/mutator.rs:1392-1394
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 146
    section: '## Input Validation'
    change: 'Add after the CSS declarations Property/value pairs row, three lines in its shape: - **Boundary:** CSS declarations / - **What to validate:** `mix-blend-mode` · `isolation` / - **How (stack-specific):** In every document, remote pages included: page CSS now paints both, validated by Stylo''s parse and mapped by `convert_blend_mode`, `pub(crate)` in blitz-paint''s private `blend` module; `render_element` pushes a viewport-wide blend layer and pops it last, and `Node`''s stacking-context arm reads a `mix-blend-mode` other than `normal`; unit tests in `layers.rs` hold the layers, and no fork or upstream test paints a blended element to a frame or names `convert_blend_mode` (packages/blitz-paint/src/blend.rs:5-26; packages/blitz-paint/src/render.rs:485-506; packages/blitz-dom/src/node/node.rs:1382-1384; packages/blitz-paint/src/layers.rs:104-181) — an engine change for every document — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md'
    sidecar: 'Input Validation: new row, `mix-blend-mode` and `isolation` from page CSS paint, validated by Stylo''s parse, no frame test (upstream 6fca6308d).'
    rationale: 'report.md:15 has both properties paint on every document and report.md:104 gives the validation as Stylo''s parse and the missing frame test, whose owner is P5''s by report.md:172.'
    basis: packages/blitz-paint/src/blend.rs:5-26
    fact: F7
note: 'F5 (JS API arguments) and F8 (the widget replay) have no picked entity that can carry them, so they are the new-detail pass''s; the Touches on JS API arguments, Paint values and the example-app `color` row are not in this listing; the `resolve_url` panic row F6 bears on is error-handling''s.'
