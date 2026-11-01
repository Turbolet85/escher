proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → blitz-net"
    change: "Replace «no test — observed absent» and its search: `packages/blitz-net/src/lib.rs` now holds a `#[cfg(test)] mod tests` of 3 unit tests of a load's delivery (`delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors`, `http_error_body_is_not_delivered_as_css`), the last two each over a test-only `TcpListener` bind on `127.0.0.1:0`; its `file:` read still has no test named for it — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md."
    sidecar: "§1 blitz-net: the observed-absent claim retired, blitz-net now holds 3 unit tests of a load's delivery, two of them over a test-only loopback bind."
    rationale: "The chunk adds tests where the plan states none: report.md:28 names the module and its two binds, report.md:103 counts `unit blitz-net 3`; the body's absence claim stands in no category file, so no cascade reaches it."
    basis: "packages/blitz-net/src/lib.rs:465-546"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → blitz-paint"
    change: "Inline unit tests exist in gradient.rs, css_box.rs and, for its blend layers, layers.rs (3); `mix-blend-mode` and `isolation` now paint on every document and no check paints a blended element to a frame or names `convert_blend_mode`, and no frame reads an inline box holding only block children, whose own shadow, background and border `render_element` skips, its flag read by blitz-dom's `inline_with_only_block_children_tests` (4); the blend's frame gap takes the owner P5 names."
    sidecar: "§1 blitz-paint: layers.rs's unit tests named, and the frame gap stated for painted blending and for the inline box's skipped decorations."
    rationale: "New paint paths with no test at the frame level the plan's pixel tests read: report.md:104 and report.md:106 mark frame ✗, report.md:172 gives the blend's ✗ an owner at P5; the body's closed list of files with unit tests also misses layers.rs."
    basis: "packages/blitz-paint/src/layers.rs:104-181"
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → blitz-test-harness"
    change: "`settle.rs` pins the load counter, its forward of `NetHandler::error` (`a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it`), the settle loop and the `NotSettled` messages; the rest of the line stands."
    sidecar: "§4 blitz-test-harness: settle.rs's unit tests now pin the counter's forward of a failed load."
    rationale: "report.md:39 marks `test-plan.md:207` changed for the forward of `error`; report.md:25 and report.md:103 name the path and its unit test, and the line does not yet name it."
    basis: "packages/blitz-test-harness/src/settle.rs:456-473"
  - detector: D-retired
    severity: warning
    section: "§9 CI Integration → Local pre-push gate"
    change: "`bash .github/scripts/ci-leg.sh fast` — fmt → clippy → test → ci-scripts → embolden, stopping at the first red; `embolden` is the `blitz-paint/font-embolden` witness, `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, a leg with no CI job of its own, macOS exercising the feature there."
    sidecar: "§9 Local pre-push gate: `fast` gains its last step `embolden`, the gap «no local gate builds font-embolden» retired."
    rationale: "report.md:33 retires the gap in §2 and §9, and the body's four-step chain restates it by leaving the witness out; report.md:41 marks `test-plan.md:388` ×2 changed and report.md:64 states the new leg. The gap's worded sites in the detail files are the cascade's; this is the one body occurrence."
    basis: ".github/scripts/ci-leg.sh:8-9"
  - detector: D-new-detail
    severity: warning
    section: "§3 Test Harness Contract → blitz-test-harness (Harness)"
    file: new
    near: "test-plan.md:131"
    change: "- **`text_contrast` past a blend:** `text_contrast` reads `None` past an element on its walk whose `mix-blend-mode` is other than `normal` (`paints_more_than_colour`, through `resolved_style_value`), a blended pair no longer read as plain colours; `Session::lint`'s `contrast` reads the same, no second formula, and no stand row moves."
    sidecar: "§3 Harness: `text_contrast` reads `None` past a blended element, `Session::lint`'s `contrast` with it."
    rationale: "report.md:24 states the change, report.md:44 marks `test-plan.md:131` changed; §3 has no category file, so the new fact is this document's."
    basis: "packages/blitz-test-harness/src/contrast.rs:154-156"
  - detector: D-new-detail
    severity: warning
    section: "§3 Test Harness Contract → blitz-test-harness (Harness)"
    file: new
    near: "the bullet **Pump semantics:** of §3"
    change: "- **A failed load and `settle`:** the crate-private load counter forwards `NetHandler::error` to the handler it wraps, so a failed, empty or non-CSS stylesheet stops counting as a pending render-blocking resource and a harness document no longer reads `NotSettled { busy: Loads }` for it for good; `settle` itself is unedited."
    sidecar: "§3 Harness: the load counter forwards `NetHandler::error`, so a failed stylesheet no longer holds a harness document at `NotSettled(Loads)`."
    rationale: "report.md:25 says the harness and driver contracts owe the sentence, report.md:85 states the red it closes; §3 has no category file."
    basis: "packages/blitz-test-harness/src/settle.rs:121-125"
  - detector: D-new-detail
    severity: warning
    section: "§3 → Session lifecycle"
    file: new
    near: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:5"
    change: "- **session-start, a failed load:** a failed load ends the wait: `act` and an acting verb of `run` return where they would otherwise have run to `LOAD_WAIT`, the failed request no longer in flight, `settle_waiting` unedited; `NotSettled(Loads)` at the bound stays the reading of a load still unanswered."
    sidecar: "§3 Session lifecycle session-start: a failed load ends the `LOAD_WAIT` wait, `settle_waiting` unedited."
    rationale: "report.md:25 states the sentence the driver contract owes, report.md:49 marks `session-lifecycle.md:5` @c739 changed; report.md:103 names `stand_load_wait` among the path's witnesses."
    basis: "packages/blitz-test-harness/src/settle.rs:121-125"
  - detector: D-new-detail
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract, Proof"
    file: new
    near: "test-plan.md:177"
    change: "- **Proof, re-counted with the upstream take:** `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` unmoved at passed 237 · failed 0 · ignored 7 over 51 files, and `run all` reads passed 733 · failed 0 · ignored 12, +6 over 727 (`key_press_after_container_lost_its_box` 3, a new file with no `stand_` prefix picked up by `run all` with no script change, and `render_blocking_stylesheet` +3) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md."
    sidecar: "§3 Proof: `run all` re-counted 733 · 0 · 12, `run stand` unmoved at 237 · 0 · 7 over 51 files."
    rationale: "report.md:37 gives the reading and names `test-plan.md:177` as the line to follow with it, leaving the older offsets at `test-plan.md:176` as no-change; report.md:156 holds the gate's reading."
    basis: "report.md:37"
note: "D-tests-framework clean: every test command is the built-in `cargo test` harness or `python3 -m unittest`, and report.md:30 adds no dependency. D-tests-obs-harness clean: report.md:64 leaves `scripts/agent-run.sh` and the status and verdict shapes unmoved, and report.md:28 adds no log site, so obs-plan §3 was not opened. Retired items (1), (2), (4), (5) and (6) have no standing occurrence in this body or the two keyed contracts; their worded sites in the detail files are the cascade's. Not proposed, for a human read: §2 Pure-restyle paths says mutations insert full damage, and report.md:11 narrows the restyle hint on mutation without stating that damage moved, so the claim could be neither confirmed nor retired from the report. The D-new-detail on Session lifecycle lands in an existing key file, not a new one, its `file: new` following the form this pass prescribes."
