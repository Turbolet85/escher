# Apply — test-plan · part 1 of 4

## Verdicts
- fanout-test-plan--cascade-g1.md 13 · applied · .andromeda/test-plan.md:40#1
- fanout-test-plan--cascade-g4.md 12 · applied · .andromeda/test-plan.md:256#1
- fanout-test-plan.md 1 · applied · .andromeda/test-plan.md:30#1, .andromeda/test-plan.md:30#2
- fanout-test-plan--cascade-g1.md 15 · folded into fanout-test-plan.md 1
- fanout-test-plan.md 2 · applied · .andromeda/test-plan.md:32#1
- fanout-test-plan--cascade-g1.md 9 · folded into fanout-test-plan.md 2
- fanout-test-plan.md 3 · applied · .andromeda/test-plan.md:207#1
- fanout-test-plan--cascade-g4.md 6 · folded into fanout-test-plan.md 3
- fanout-test-plan.md 4 · applied · .andromeda/test-plan.md:388#1
- fanout-test-plan--cascade-g5.md 2 · folded into fanout-test-plan.md 4
- fanout-test-plan.md 5 · applied · .andromeda/test-plan.md:132+#1
- fanout-test-plan.md 6 · applied · .andromeda/test-plan.md:133+#1
- fanout-test-plan.md 7 · refused · a key file, main writes the in-line clause on session-lifecycle.md:5
- fanout-test-plan.md 8 · applied · .andromeda/test-plan.md:177+#1

## Edits
- line: .andromeda/test-plan.md:40
  old: «tests/selection.rs the script Selection API and tests/preact.rs runs»
  new: «tests/selection.rs the script Selection API, tests/url_search_params.rs the global `URLSearchParams` and `URL.prototype.searchParams`, and tests/preact.rs runs»

- line: .andromeda/test-plan.md:256
  old: «tests/selection.rs covers the script Selection API;»
  new: «tests/selection.rs covers the script Selection API; tests/url_search_params.rs the global `URLSearchParams` and `URL.prototype.searchParams`;»

- line: .andromeda/test-plan.md:30
  old: «no test — observed absent · searched: `#\[test\]|#\[cfg\(test\)\]` over packages/blitz-net/src/lib.rs (no match); its `file:` read»
  new: «its `#[cfg(test)] mod tests` pins a load's delivery — `delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`, the last two each over a test-only `TcpListener::bind` on `127.0.0.1:0`, served by no binary and reached by no caller (packages/blitz-net/src/lib.rs:465-546); its `file:` read»

- line: .andromeda/test-plan.md:30
  old: «has no test named for it either (packages/blitz-net/src/lib.rs:159-170) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md»
  new: «has no test named for it (packages/blitz-net/src/lib.rs:159-170) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»

- line: .andromeda/test-plan.md:32
  old: «inline unit tests exist in gradient.rs and css_box.rs;»
  new: «inline unit tests exist in gradient.rs, css_box.rs and, for its blend layers, layers.rs; `mix-blend-mode` and `isolation` paint on every document and no check paints a blended element to a frame or names `convert_blend_mode`, and no frame reads an inline box holding only block children, whose own shadow, background and border `render_element` skips, its flag read by blitz-dom's `inline_with_only_block_children_tests` — owner of both gaps: a `CARRY:` on «Upstream sync ahead of determinism and exploration»;»

- line: .andromeda/test-plan.md:207
  old: «`settle.rs` pins the load counter, the settle loop»
  new: «`settle.rs` pins the load counter, its forward of `NetHandler::error`, the settle loop»

- line: .andromeda/test-plan.md:388
  old: «fmt → clippy → test → ci-scripts, stopping at the first red»
  new: «fmt → clippy → test → ci-scripts → embolden — `embolden` the `blitz-paint/font-embolden` witness, `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, a leg with no CI job of its own, macOS exercising the feature there (.github/scripts/ci-leg.sh:8-9; .github/scripts/ci-leg.sh:28-29) — stopping at the first red»

- after: .andromeda/test-plan.md:132
  anchor: « the crate also re-exports keyboard-types 0.8's `Key`, `Modifiers` and, since 2026-10-17-upstream-sync-engine-debts's move to keyboard-types "0.8", `NamedKey` (`pub use keyboard_types::{Key, Modifiers, NamedKey};`) — the two types `press_with` takes and the named-key enum a 0.8 `Key::Named` holds — so a crate that depends on the harness, escher-driver among them, names a key with no dependency of its own (packages/blitz-test-harness/src/lib.rs:32). The harness's time is a `VirtualClock` its documents read: `time` reads it in seconds and `pump` resolves at it; `tick(dt)` advances it, a negative or non-finite `dt` moving nothing, and `advance_time(Duration)` advances it by exactly the duration, each then pumping; `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move it (ratified by the founder, 2026-10-18) (packages/blitz-test-harness/src/harness.rs:152-174)»
  add: «- **`text_contrast` past a blend:** `text_contrast` reads `None` past an element on its walk whose `mix-blend-mode` is other than `normal` (`paints_more_than_colour`, through `resolved_style_value`); `Session::lint`'s `contrast` reads the same, no second formula, and the `None` moves no stand row (packages/blitz-test-harness/src/contrast.rs:154-156)»

- after: .andromeda/test-plan.md:133
  anchor: «- **Pump semantics:** pump polls the document with no waker context and resolves at the harness time; dispatch and dispatch_recorded do not pump (packages/blitz-test-harness/src/harness.rs:157-189); `pump()` applies pending changes after a mutation through `base_mut().mutate()` (tests/blitz-tests/tests/dir_attribute.rs:72-79; tests/blitz-tests/tests/oof_dynamic_cb.rs:37-43); `settle` repeats that pass and keeps poll's answer — a pass is quiet when poll answered `false`, the hover node did not change and no load finished in it; after a quiet pass a request in flight or a pending render-blocking resource returns `NotSettled { busy: Loads }` at once, otherwise `Settled { passes, animating }`; `SETTLE_PASS_LIMIT` passes with none quiet return `Render`, `Loads` or `Layout`, read off the last pass; it reads no clock but the harness's, advances no time, waits on no load, neither reads nor drains the changed set and logs nothing, and `pump` and every input helper are unchanged — none settles (packages/blitz-test-harness/src/settle.rs:135-197; as measured at escher-0.1.0/chunks/2026-10-07-settle-detection/report.md)»
  add: «- **A failed load and `settle`:** the crate-private load counter forwards `NetHandler::error` to the handler it wraps, so a failed stylesheet reaches the document and stops counting as a pending render-blocking resource, and a harness document does not read `NotSettled { busy: Loads }` for it (packages/blitz-test-harness/src/settle.rs:121-125)»

- after: .andromeda/test-plan.md:177
  anchor: « re-counted with the select list's three new targets, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 237 · failed 0 · ignored 7 over 51 files — +15 passed (`stand_act_popup` 12, a new file its `stand_` prefix picks up, `stand_keyboard` +2 and `stand_lint` +1) — and `run all` reads passed 727 · failed 0 · ignored 12, `select_popup` and `select_popup_painted` carrying no `stand_` prefix and picked up by it with no script change — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
  add: «- **Proof, the newest reading:** `run stand`'s `run.end` reads passed 237 · failed 0 · ignored 7 over 51 files and `run all`'s reads passed 733 · failed 0 · ignored 12, `key_press_after_container_lost_its_box`, with no `stand_` prefix, picked up by `run all` with no script change — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»

## Patterns
[[pattern]]
id = "tp1-01"
fixed = 'over packages/blitz-net/src/lib.rs (no match)'

[[pattern]]
id = "tp1-02"
fixed = 'inline unit tests exist in gradient.rs and css_box.rs'

[[pattern]]
id = "tp1-03"
fixed = 'fmt → clippy → test → ci-scripts, stopping'
