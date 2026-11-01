# Apply — security-plan · part 1 of 2

## Verdicts
- fanout-security-plan--cascade-g2.md 2 · applied · .andromeda/security-plan.md:107#1
- fanout-security-plan--cascade-g4.md 2 · applied · .andromeda/security-plan.md:209#1
- fanout-security-plan.md 1 · applied · .andromeda/security-plan.md:326#1
- fanout-security-plan.md 2 · applied · .andromeda/security-plan.md:324#1
- fanout-security-plan--cascade-g1.md 1 · applied · .andromeda/security-plan-detail/threat-model-summary.md:11#1
- fanout-security-plan--cascade-g1.md 2 · applied · .andromeda/security-plan-detail/threat-model-summary.md:19+#1
- fanout-security-plan--cascade-g2.md 1 · applied · .andromeda/security-plan-detail/input-validation.md:113#1
- fanout-security-plan--cascade-g2.md 3 · applied · .andromeda/security-plan-detail/input-validation.md:113+#1
- fanout-security-plan--cascade-g2.md 4 · applied · .andromeda/security-plan-detail/input-validation.md:146+#1
- fanout-security-plan--cascade-g3.md 1 · applied · .andromeda/security-plan-detail/input-validation.md:314#1
- fanout-security-plan--cascade-g3.md 3 · applied · .andromeda/security-plan-detail/input-validation.md:395+#1
- fanout-security-plan--cascade-g3.md 4 · applied · .andromeda/security-plan-detail/input-validation.md:201+#1
- fanout-security-plan--cascade-g4.md 1 · applied · .andromeda/security-plan-detail/api-security.md:8#1
- fanout-security-plan--cascade-g4.md 3 · applied · .andromeda/security-plan-detail/api-security.md:31+#1
- fanout-security-plan--cascade-g4.md 4 · applied · .andromeda/security-plan-detail/dependency-security.md:24+#1
- fanout-security-plan--cascade-g4.md 5 · applied · .andromeda/security-plan-detail/dependency-security.md:52#1
- fanout-security-plan--cascade-g4.md 6 · applied · .andromeda/security-plan-detail/dependency-security.md:52+#1

## Edits
- line: .andromeda/security-plan.md:107
  old: «must be valid UTF-8, else the load errs.»
  new: «must be valid UTF-8, else the load errs; a failed, empty or non-CSS stylesheet blocks no render.»
- line: .andromeda/security-plan.md:209
  old: «no TCP or UDP port; the one listener is local, the driver»
  new: «no served TCP or UDP port, two `blitz-net` tests alone binding TCP on loopback, served by no binary; the one listener is local, the driver»
- line: .andromeda/security-plan.md:326
  old: «a failed fetch is logged when tracing is enabled, and otherwise discarded; the handler is not called on error (packages/blitz-net/src/lib.rs:304-315)»
  new: «a failed fetch is logged when tracing is enabled and handed to the handler through `NetHandler::error`, a default that does nothing, with or without tracing (packages/blitz-net/src/lib.rs:304-315; packages/blitz-traits/src/net.rs:36-38)»
- line: .andromeda/security-plan.md:324
  old: «pending image waiters for the URL are removed (packages/blitz-dom/src/document.rs:1824-1845)»
  new: «pending image waiters for the URL are removed (packages/blitz-dom/src/document.rs:1824-1845); a failed stylesheet's request leaves `pending_critical_resources`, so it blocks no render (packages/blitz-dom/src/net.rs:144-146)»
- line: .andromeda/security-plan-detail/threat-model-summary.md:11
  old: «each held under Input Validation. A remote page can also reach a key-press panic at `packages/blitz-dom/src/node/node.rs:1788` (`position_in_containing_block`), the focused control's anonymous block dropped because an ancestor gained `hidden`, listed under Error Handling's panic paths (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»
  new: «each held under Input Validation.»
- after: .andromeda/security-plan-detail/threat-model-summary.md:19
  anchor: «  - **Trust boundary:** The default script fetcher supports only `file:` and `data:` URLs; any other scheme returns `UnsupportedScheme` (packages/blitz-vibey-script/src/fetch.rs:48-70). A `file:` URL loads only for a `file:` document: `document_may_load` refuses it for any other document, remote pages included, with a `FetchError`, checked by JS `fetch()`, module `import` and `<script src>` alike — a narrowing of the boundary, not a widening (packages/blitz-vibey-script/src/fetch.rs:40-46; packages/blitz-vibey-script/src/runtime/fetch.rs:40-44; packages/blitz-vibey-script/src/document.rs:163-169). A `file:` URL is read from the local filesystem with `std::fs::read_to_string` (packages/blitz-vibey-script/src/fetch.rs:54-59). Every event object created by the runtime sets `isTrusted` to true (packages/blitz-vibey-script/src/dom/event.rs:118). Same-origin or CORS checks are observed absent — searched: `same.origin|same_origin|cross.origin|CORS` over the 32 s10 slice files.»
  add: « A page's script also reaches `URLSearchParams` and `URL.prototype.searchParams`, `href` and `search` live both ways, through `url_search_params.js`, the first of ten bootstrap scripts: a realization on this registered crossing, no boundary widening; its native helper `__blitz_parse_search_params` is deleted from `globalThis` before any page script runs, and its argument checks are held under Input Validation (packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4; packages/blitz-vibey-script/src/runtime/bootstrap.rs:167-184) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md.»
- line: .andromeda/security-plan-detail/input-validation.md:113
  old: «else the load responds `Err("Invalid UTF8")` (packages/blitz-dom/src/net.rs:149-151; packages/blitz-dom/src/net.rs:266-268)»
  new: «else the load responds `Err("Invalid UTF8")` (packages/blitz-dom/src/net.rs:149-151; packages/blitz-dom/src/net.rs:266-268); in every document, remote pages included, a failed, empty or non-CSS stylesheet blocks no render: a load that errs reaches `ResourceHandler<StylesheetHandler>::error`, which responds `Err(error)`, so `pending_critical_resources` loses the request; pinned by `failed_stylesheet_unblocks_rendering` and by `empty_or_non_css_stylesheet_unblocks_rendering` over an empty body, an HTML page and the byte 0xFF (packages/blitz-dom/src/net.rs:144-146; tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-67; tests/blitz-tests/tests/render_blocking_stylesheet.rs:69-91) — an engine change for every document — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- after: .andromeda/security-plan-detail/input-validation.md:113
  anchor: «  - **How (stack-specific):** Must be valid UTF-8, else the load responds `Err("Invalid UTF8")` (packages/blitz-dom/src/net.rs:149-151; packages/blitz-dom/src/net.rs:266-268)»
  add: «- **Boundary:** Stylesheet `href`»
  add: «  - **What to validate:** URL»
  add: «  - **How (stack-specific):** In every document, remote pages included: an `href` that cannot be resolved starts no load and blocks no render, read through `doc.url.resolve_relative` and not `resolve_url`, which panics (the Panic paths); pinned by `invalid_stylesheet_url_does_not_block_rendering` (packages/blitz-dom/src/mutator.rs:1392-1394; tests/blitz-tests/tests/render_blocking_stylesheet.rs:93-107) — an engine change for every document — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- after: .andromeda/security-plan-detail/input-validation.md:146
  anchor: «  - **How (stack-specific):** Validated by parsing; invalid declarations are ignored per CSSOM (packages/blitz-dom/src/resolved_style.rs:162-182; packages/blitz-dom/src/resolved_style.rs:218-251; packages/blitz-dom/src/cssom.rs:716-757)»
  add: «- **Boundary:** CSS declarations»
  add: «  - **What to validate:** `mix-blend-mode` · `isolation`»
  add: «  - **How (stack-specific):** In every document, remote pages included: page CSS paints both, validated by Stylo's parse and mapped by `convert_blend_mode`, `pub(crate)` in blitz-paint's private `blend` module; `render_element` pushes a viewport-wide blend layer and pops it last, and `Node`'s stacking-context arm reads a `mix-blend-mode` other than `normal`; 3 unit tests in `layers.rs` hold the layers, and no fork or upstream test paints a blended element to a frame or names `convert_blend_mode` (packages/blitz-paint/src/blend.rs:5-26; packages/blitz-paint/src/render.rs:485-506; packages/blitz-dom/src/node/node.rs:1382-1384; packages/blitz-paint/src/layers.rs:104-181) — an engine change for every document — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- after: .andromeda/security-plan-detail/input-validation.md:201
  anchor: «  - **How (stack-specific):** Accepted only if it parses as `u64` (packages/blitz-dom/src/mutator.rs:1525-1536)»
  add: «- **Boundary:** Markup attributes»
  add: «  - **What to validate:** a custom widget's attributes at `set_custom_widget`»
  add: «  - **How (stack-specific):** Behind the `custom-widget` feature, `ElementData::set_custom_widget` replays each existing attribute, parsed page content included, to `Widget::attribute_changed` with no old value, so a widget receives them at set-up and not only on a later change, the value's check staying the widget's own, as the example's `color` parse (the Widget and TodoMVC row); pinned by `custom_widget_replays_initial_attributes` (packages/blitz-dom/src/node/element.rs:902-906; packages/blitz-dom/src/mutator.rs:1656-1727) — an engine change for every document behind the feature, ratified — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- line: .andromeda/security-plan-detail/input-validation.md:314
  old: «is culled before painting (packages/blitz-paint/src/render.rs:412-427)»
  new: «is culled before painting, `render_element`'s border box and culling standing on the scaled size, `location` and `box_position` kept for the transform (packages/blitz-paint/src/render.rs:412-427); a box flagged `IS_INLINE_WITH_ONLY_BLOCK_CHILDREN` paints no shadow, background or border of its own, pinned by `inline_with_only_block_children_tests` 4, no frame read (packages/blitz-dom/src/layout/construct.rs:587-599; packages/blitz-dom/src/document.rs:4096-4296) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- after: .andromeda/security-plan-detail/input-validation.md:395
  anchor: «  - **How (stack-specific):** Rejects BigInt in number conversion and non-object dictionaries with TypeError (packages/blitz-vibey-script/src/geometry.js:27-46)»
  add: «- **Boundary:** JS API arguments»
  add: «  - **What to validate:** `URLSearchParams` and `URL.prototype.searchParams`, `href` and `search` live both ways»
  add: «  - **How (stack-specific):** A page's script reaches them through `url_search_params.js`, first of the ten bootstrap scripts, a realization on the registered script crossing, no boundary widening; bad calls and receivers throw TypeError and values pass `usv`; the native `parse_search_params`, registered as `__blitz_parse_search_params`, is deleted from `globalThis` before any page script; no log site and no carrier; pinned by `url_search_params` 4 (packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4; packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:6-14; packages/blitz-vibey-script/src/runtime/bootstrap.rs:167-184; packages/blitz-vibey-script/tests/url_search_params.rs:61-78) — an engine change for every document, ratified — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- line: .andromeda/security-plan-detail/api-security.md:8
  old: «no TCP or UDP port; the one listener is local:»
  new: «no served TCP or UDP port: two `blitz-net` tests each call `std::net::TcpListener::bind("127.0.0.1:0")` in the crate's `#[cfg(test)] mod tests`, served by no binary, reached by no caller, so no crossing, registered as test-only (packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534), as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md; the one listener is local:»
- after: .andromeda/security-plan-detail/api-security.md:31
  anchor: « — on a harness-built document the harness's `VirtualClock`, which only `tick`, `advance_time` and the driver's `advance` move — so two shots of one state read the same pixels until an `advance` moves the fade, a document built with no clock still fading by wall time (ratified by the founder, 2026-10-18) (packages/blitz-test-harness/src/screenshot.rs:37-103) — nor do the three session-level verbs, whose one argument is `app` and whose result fields are `label`, `pid`, `served`, `idle_expiry_s` and `stopped`, nor, since 2026-10-12-self-description, the tool-level `help`, which takes no argument and whose seven result fields are fixed tables — and its validation reads no clock; the command line's own bounds, 30 s for a started host to answer and 30 s on each read of a call's answer, bound a transport and wait on no load; a step that ran and did not go quiet is a result (`settled: false` with its busy class) and not a refusal, and time on a held instance moves only when a caller asks for it with `advance`, 1 to 60000 ms a call — an acting driver command waits on a load in flight for at most `LOAD_WAIT`, the bound the `settled` field's help states, and `snapshot`, `screenshot` and `lint` wait on none — as measured at escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md; the wait has one served witness on the stand, the load page (`stand::boot_load`, `escher-session start load`): its one fetch of the fixed `http://stand.test/load.css` is answered in process by `LoadNet` after `LOAD_ANSWER_DELAY` (200 ms) from a thread of its own with fixed CSS bytes — no `blitz_net`, socket, listener, print or env read — and, `LoadNet` overriding no `is_noop`, the harness counts the load and the page's click waits it out within `LOAD_WAIT` (`stand_load_page`); every other stand boot holds `net_provider: None` (examples/seven_guis/src/stand.rs:141-175)»
  add: « — a failed load ends the wait: `CountedHandler` forwards `NetHandler::error` (packages/blitz-test-harness/src/settle.rs:121-125), so a failed stylesheet reads no `NotSettled(Loads)` for good on a harness document or on the driver, and `settle_waiting` returns where it would otherwise have run to `LOAD_WAIT` — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- after: .andromeda/security-plan-detail/dependency-security.md:24
  anchor: «- The upstream sync read at `ebacd68bd` took eleven commits, `d63ed2c8c` · `6587e7e79` · `e6f4615ec` · `a7ea9631b` · `b5b163450` · `7fc9ee3af` · `2963b65a2` · `cee6b8731` · `53e9d81c6` · `6fee0db25` · `efdc5472a`, by no-commit cherry-picks of upstream's text (`e795a088c`), sixteen left out with their measured reasons, the merge base staying `23b9b26a9`: `Cargo.toml`, `Cargo.lock`, `deny.toml` and every crate manifest are byte-identical to the tree before it, so no package enters, leaves or moves; the pins stay — Stylo "0.22.0", Taffy the registry "0.15", Dioxus "0.7.3", winit `=0.31.0-beta.3`, no git dependency; the audit leg reads `advisories ok` and `licenses ok`, every standing ignore encountered and none added — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/lock.md»
  add: «- The upstream sync read at `e42ee1f25` took twelve commits, `fe33b778b` · `8b1a0a3ce` · `dee854c76` · `f6edb9701` · `6fca6308d` · `7a3dd8918` · `91ef65e21` · `a26c6ddeb` · `f76777766` · `6ef4b532d` · `ae774d7f4` · `e42ee1f25`, by no-commit cherry-picks (`45c333f16`), thirty-six of 57 taken and twenty-one left out with their measured reasons, the merge base staying `23b9b26a9`: `Cargo.toml`, `Cargo.lock`, `deny.toml` and every crate manifest are byte-identical to the tree before it, so no package enters, leaves or moves; the pins stay — Stylo "0.22.0", Taffy the registry "0.15", Dioxus "0.7.3", winit `=0.31.0-beta.3`, no git dependency; the audit leg reads `advisories ok` and `licenses ok` — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- line: .andromeda/security-plan-detail/dependency-security.md:52
  old: «the same action's step on the `matrix_test` job (.github/workflows/ci.yml:392-394) is skipped on every matrix row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm`, and the android row runs `ubuntu-latest` (`completed · skipped` on `Test (android)`, CI#41190771843) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
  new: «the `matrix_test` job holds no such step: `Test (android)` reads none in CI#41333172440, and `test_only_stand_repeat_frees_disk` (.github/scripts/test_ci_workflows.py:110-115) pins the action to `Stand checks repeated` — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- after: .andromeda/security-plan-detail/dependency-security.md:52
  anchor: «- **CI action, the `matrix_test` free-disk step:** the same action's step on the `matrix_test` job (.github/workflows/ci.yml:392-394) is skipped on every matrix row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm`, and the android row runs `ubuntu-latest` (`completed · skipped` on `Test (android)`, CI#41190771843) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
  add: «- **Lockfile verification, the `embolden` leg:** `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted` (.github/scripts/ci-leg.sh:28-29) is a locked cargo leg, in `LEGS` and last in `FAST_LEGS` (.github/scripts/ci-leg.sh:8-9), with no CI job of its own, macOS exercising the feature there — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»

## Patterns
[[pattern]]
id = "sec1-01"
fixed = 'no TCP or UDP port; the one listener is local,'

[[pattern]]
id = "sec1-02"
fixed = 'the handler is not called on error'

[[pattern]]
id = "sec1-03"
fixed = 'can also reach a key-press panic'

[[pattern]]
id = "sec1-04"
fixed = 'no TCP or UDP port; the one listener is local:'

[[pattern]]
id = "sec1-05"
fixed = 'is skipped on every matrix row'
