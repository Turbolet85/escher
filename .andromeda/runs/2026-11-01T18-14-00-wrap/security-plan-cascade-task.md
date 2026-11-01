# Task — security-plan · 2026-11-01-upstream-sync-ui-graph

## The session
Took twelve upstream `DioxusLabs/blitz` commits by no-commit cherry-pick from `upstream/main` at `e42ee1f25`, merge base `23b9b26a9`, 36 of 57 commits now taken and 21 left out; each is an engine behaviour change for every document unless the report says otherwise, and no dependency, manifest, lock or `deny.toml` line moved.
Fixed escher's own key-press panic: `BaseDocument::deallocate_anonymous_block` clears `layout_parent` on the dropped block's children, so a key press on a focused control whose anonymous block was dropped no longer panics, and the panic limit retires.
Made failed stylesheets stop blocking render through a new `NetHandler::error`, forwarded by the harness's `CountedHandler`, so a failed load now ends the harness and driver wait.
Added page-reachable surface: `URLSearchParams` with `URL.prototype.searchParams`, painted `mix-blend-mode` and `isolation`, and attribute replay to `CustomWidget`.
Moved the gates: a new `embolden` step ends the `fast` leg, `matrix_test` lost its free-disk step, and `blitz-net` gained two test-only TCP loopback binds; no port is served.
Measured green: CI#41333172440 on `45c333f16`, 17 of 17 checks, audit `advisories ok` and `licenses ok`.

## Reached files
- packages/blitz-dom/assets/default.css · merge surface
- packages/blitz-dom/src/document.rs · merge surface
- packages/blitz-dom/src/layout/construct.rs · merge surface
- packages/blitz-dom/src/layout/damage.rs · merge surface
- packages/blitz-dom/src/lib.rs
- packages/blitz-dom/src/mutator.rs · merge surface
- packages/blitz-dom/src/net.rs · merge surface
- packages/blitz-dom/src/node/custom_widget.rs
- packages/blitz-dom/src/node/element.rs · merge surface
- packages/blitz-dom/src/node/node.rs · merge surface
- packages/blitz-dom/src/resolved_style.rs · merge surface
- packages/blitz-dom/src/stylo.rs · merge surface
- packages/blitz-net/src/lib.rs · merge surface
- packages/blitz-paint/src/blend.rs · merge surface
- packages/blitz-paint/src/layers.rs · merge surface
- packages/blitz-paint/src/lib.rs · merge surface
- packages/blitz-paint/src/render.rs · merge surface
- packages/blitz-paint/src/render/background.rs
- packages/blitz-paint/src/render/box_shadow.rs
- packages/blitz-test-harness/src/contrast.rs
- packages/blitz-test-harness/src/settle.rs
- packages/blitz-traits/src/net.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js · merge surface
- packages/blitz-vibey-script/tests/url_search_params.rs · merge surface
- packages/dioxus-native-dom/src/dioxus_document.rs · merge surface
- tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
- tests/blitz-tests/tests/render_blocking_stylesheet.rs · merge surface
- tests/blitz-tests/tests/select_popup.rs
- wpt/runner/timeout-quarantine.txt · merge surface
- .github/workflows/ci.yml
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py

## Touches
- Error Handling, the key-press panic limit at `.andromeda/security-plan-detail/error-handling.md:48`: retired; that line is also an owner row naming this entry, to be re-pointed or dropped.
- Threat Model Summary, the panic's boundary sentence at `.andromeda/security-plan-detail/threat-model-summary.md:11`: retired together with the Error Handling row, for `hidden`, `style="display:none"` and CSS alone.
- Error Handling, the row stating that `NetProvider::fetch` does not call the handler on error: stale, since `blitz-net`'s error arm now calls `handler.error(url, e.to_string())` (`packages/blitz-net/src/lib.rs:313`) and the waker moves after the handler (`:316`).
- Error Handling, the row on failed resource loads being logged and dropped: a failed stylesheet is now also answered, `ResourceHandler<StylesheetHandler>::error` responding `Err(error)` so that `pending_critical_resources` loses the request.
- Input Validation, the fetched-stylesheet row: a load that errs, or an empty or non-CSS body, no longer blocks render.
- Panic paths, the URL-handling row on `resolve_url`: the stylesheet `href` path no longer goes through it, `doc.url.resolve_relative` taking its place (`packages/blitz-dom/src/mutator.rs:1392-1394`); the panicking resolver's other callers are not said to move.
- Graceful degradation and debug-only rows on stale ids and layout-parent consistency: through the panic's fix, a stale `layout_parent` naming a dropped block stops existing, curing every reader of it.
- Graceful degradation, the rows on tests guarding against stale-id panics: `key_press_after_container_lost_its_box` and `select_popup`'s row 3 now witness this path, the row-3 skip gone.
- API Security, the no TCP or UDP port claim at `.andromeda/security-plan-detail/api-security.md:8` and the body's line: read as no served port, and it takes a word saying two `blitz-net` tests bind TCP on loopback.
- API Security, settle and loads in flight: a failed load now ends the wait, through `CountedHandler`'s forward of `error` (`packages/blitz-test-harness/src/settle.rs:121-125`); `settle_waiting` is unedited, and `LOAD_WAIT` still bounds a load that neither answers nor fails.
- Threat Model, the remote-content vector: a failing stylesheet no longer holds render, a page's CSS reaches painted blend and isolation, and a key press past a dropped block no longer ends the process.
- Threat Model, the script-execution vector: `URLSearchParams` is a realization on the registered script crossing, no boundary widening, a new script-facing surface in the same boundary.
- Input Validation, JS API arguments: gains `URLSearchParams`'s TypeError arms and `usv` coercion, and a native helper deleted before any page script runs.
- Input Validation, paint values: `render_element`'s border box and culling now stand on the scaled size (`e42ee1f25`), and a box flagged `IS_INLINE_WITH_ONLY_BLOCK_CHILDREN` skips its own shadow, background and border.
- Input Validation, the example-app row on the widget's `color` attribute: through `a26c6ddeb`, a widget now receives each existing attribute at `set_custom_widget`, not only later changes.
- Logging & Monitoring, the rows on network logging and on `Resource load failed`: no tracing site added, but a failed stylesheet's error text now travels through `ResourceLoadResponse` to that line; the `url` and `error` fields stay in escher-telemetry's scrub set.
- Dependency Security, the upstream-sync history at `.andromeda/security-plan-detail/dependency-security.md:24`: the last take's record stays as history, with a new sentence for this take beside it.
- Dependency Security, the `matrix_test` free-disk step at `.andromeda/security-plan-detail/dependency-security.md:52`: removed, stated three times on that line.
- Dependency Security, the `Stand checks repeated` free-disk step at `.andromeda/security-plan-detail/dependency-security.md:51`: unchanged, its watch at 3 green runs with the retirement condition met and no cure shown.
- Supply chain, the lockfile-verification list of the leg script's locked commands: the new `embolden` leg belongs to that list.
- Citations into `.github/scripts/ci-leg.sh`, the `:20-69` range in Pinning and Supply chain: moved, lines added at `:8-9` and `:28-29`.
- Citations into `.github/workflows/ci.yml` past the removed block at base lines `:392-398`, among them `:402` and `:415`: moved.
- Citation `.github/scripts/test_ci_workflows.py:193-199` for the SHA-pin assertion: moved, lines added at `:110-116`.
- Citations into `packages/blitz-dom/src/document.rs` in Error Handling and Logging, among them `:621-624`, `:1634-1641`, `:1824-1845`, `:1829-1835`, `:1905` and `:2428-2435`: moved by added lines.
- Citations into `packages/blitz-dom/src/net.rs` past `:144-147`, namely `:347-370`, `:372-375` and `:576-579`, and into `packages/blitz-dom/src/mutator.rs:1517-1521`: moved.
- Citations into `packages/blitz-dom/src/node/node.rs` (`:154-158`, `:195-201`, `:357-364`, `:1169-1178`), `packages/blitz-dom/src/stylo.rs:449` and `packages/blitz-dom/src/layout/construct.rs` (`:894`, `:1058`): moved.
- Citations into `packages/blitz-net/src/lib.rs` (`:304-315`, `:307-312`, `:439-442`), `packages/blitz-paint/src/render.rs:892-899` and `packages/blitz-paint/src/render/background.rs:202-221`: moved or widened.
- Any line in the plan's detail that cites other lines of a reached file past an added range above moves by the same rule; the report's `New text, by line` holds the added ranges.
- `.andromeda/security-plan-detail/error-handling.md:9`: its left-out token at character 1816 is a true claim about another thing, re-read with no change to it.

## New facts
- F1 · api-security · `packages/blitz-net/src/lib.rs:526` and `:534` each call `std::net::TcpListener::bind("127.0.0.1:0")` inside its `#[cfg(test)] mod tests`, served by no binary, reached by no caller, no crossing, owed registration as test-only under the session rule of 2026-10-15.
- F2 · api-security · A failed load now ends the harness and driver wait: `CountedHandler` forwards `NetHandler::error`, where a failed stylesheet otherwise read `NotSettled(Loads)` for good.
- F3 · dependency-security · This take's lock record: twelve commits taken from `upstream/main` at `e42ee1f25`, merge base `23b9b26a9`, 36 of 57 taken and 21 left out, manifests, `Cargo.lock` and `deny.toml` byte-identical, no package entered, audit `advisories ok` and `licenses ok`.
- F4 · dependency-security · The new `embolden` leg, `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, is a locked cargo leg, last in `FAST_LEGS`, with no CI job of its own.
- F5 · input-validation · `URLSearchParams` and `URL.prototype.searchParams`, with `href` and `search` live both ways, are page-reachable through `url_search_params.js`, first of ten bootstrap scripts, validated by TypeError arms and `usv`, its native `parse_search_params` registered as `__blitz_parse_search_params` and deleted from `globalThis` before any page script.
- F6 · input-validation · A stylesheet `href` that cannot be resolved starts no load, no panic, by `doc.url.resolve_relative` (`packages/blitz-dom/src/mutator.rs:1392-1394`).
- F7 · input-validation · `mix-blend-mode` and `isolation` from page CSS now paint, validated by Stylo's parse and mapped by `convert_blend_mode` (`packages/blitz-paint/src/blend.rs:5-26`), with a viewport-wide blend layer pushed by `render_element`; no test paints a blended element to a frame.
- F8 · input-validation · `ElementData::set_custom_widget` replays each existing attribute, parsed page content included, to `Widget::attribute_changed` with no old value, behind the `custom-widget` feature (`packages/blitz-dom/src/node/element.rs:902-906`).
- F9 · error-handling · `NetHandler::error` is new on the public trait, a default that does nothing (`packages/blitz-traits/src/net.rs:36-38`), so a wrapping implementer that does not forward it drops the failure silently; `ResourceHandler<StylesheetHandler>::error` answers `Err(error)` and a failed, empty or non-CSS stylesheet stops blocking render.
- F10 · error-handling · `BaseDocument::deallocate_anonymous_block` takes the block's `layout_children` and sets `layout_parent` to `None` on each child still in the slab naming the block (`packages/blitz-dom/src/document.rs:1493-1503`), witnessed by `key_press_after_container_lost_its_box`, 3 tests in both layout modes, and by `select_popup`'s row 3, both red before the fix.
- F11 · error-handling · `flush_queued_mounted_events` skips an element removed since its event was queued, through `try_element_to_node_id` (`packages/dioxus-native-dom/src/dioxus_document.rs:235-239`).
- F12 · logging-monitoring · A failed load's error text stops in `ResourceLoadResponse` and the `Resource load failed` line, no carrier takes it, and escher's sink prints `waiting_nodes` and the message with `url` and `error` in its scrub set.

## Untouched
- Authentication & Authorization, the driver socket's and MCP server's absence of handshake, token or peer check: `packages/escher-driver` holds no diff.
- Threat Model groups boundary-session-socket and boundary-mcp-stdio: `packages/escher-driver` holds no diff, and no new carrier or exit crosses them.
- Input Validation, the driver rows on the socket, state directory, recording directory, `Check::from_bytes`, session label, command schema and `escher-session` argv: `packages/escher-driver` holds no diff.
- escher's log sink, its target allowlist and scrub set in `packages/escher-telemetry`: no diff, and no scrub set moved.
- Input Validation, the CLI rows for `agent-run.sh`, `cold-agent.sh`, `apt-install.sh` and `coverage_floor.py`: none of them reached.
- Data Protection, kept and committed checks: no recorded check or its medium changed, and no carrier was added.
- Data Protection, the browser's sqlite history and its deletion under `apps/browser`: not reached.
- Data Protection, TLS through reqwest's `native-tls` in `packages/blitz-net/Cargo.toml`: no manifest moved.
- Dependency Security, the audit tool's configuration, `deny.toml`'s ignores and licence table, and the pins and inline versions in manifests: byte-identical to the base.
- Secret Management, secret storage and what counts as secret: `.github/workflows/publish-browser.yml` and the operator's login not reached, and no env read added.
- The upstream-only workflows `.github/workflows/publish-browser.yml`, `.github/workflows/wpt.yml` and `.github/workflows/wpt-post-results.yml`, and their repository guards: not reached.
- CLI and markdown vectors of `apps/readme`, `examples/screenshot.rs`, `examples/paint_bench.rs` and `examples/preact_script.rs`, and the fixture rows under `examples/assets`: not reached.
- The graceful-degradation row that `try_element_to_node_id` returns an option, `packages/dioxus-native-dom/src/mutation_writer.rs:41-49`: states the option, unedited; only a new caller's guard is added elsewhere.
- Compliance Controls, Security Anti-Patterns and the Security Decisions Log: each holds no recorded intent, and the take's ratification lands in architecture's decisions, not here.
