proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 48
    section: "## Error Handling"
    change: The whole row now reads «Key press after a container lost its box — `BaseDocument::deallocate_anonymous_block` takes the block's `layout_children` and sets `layout_parent` to `None` on each child still in the slab naming the block (packages/blitz-dom/src/document.rs:1493-1503), so the id stops being stale and a key press with focus on a control whose anonymous block was dropped no longer panics, the box lost by `hidden`, by `style="display:none"` or by CSS alone, both layout modes (`key_press_after_container_lost_its_box`, 3 tests; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:122-134; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:136-160; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:162-187), and `select_popup` presses the key in every row (tests/blitz-tests/tests/select_popup.rs:513), both red before; focus stays and the key still acts, so a focused control that no longer shows keeps focus and takes keys, the limit a11y-plan §5 states; an engine change for every document, architecture §Established Decisions its entry; upstream's tree panics the same way at `e42ee1f25`, focused by `Tab` — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
    sidecar: Retired the key-press panic and `select_popup`'s row-3 skip, the row restated as the fix, `deallocate_anonymous_block` clearing the stale `layout_parent`; its CARRY owner on «Upstream sync ahead of the UI graph» answered by the landing, the fork answered «Убрать падение, фокус остаётся (Рекомендую)» (inputs#I3).
    rationale: The report's Retired bullet names the key-press panic and `select_popup`'s row-3 skip, and its `panic` row marks this line changed, retired together with `threat-model-summary.md:11`, its owner row re-pointed to nothing where the owed thing landed (.andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:23, :26, :33, :50, :62, :96, :139); the row still says a key press panics, not fixed, and that `select_popup` skips it.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:23
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 54
    section: "## Error Handling"
    change: Where the row says «this holds of the close alone, a key pressed after an ancestor gained `hidden` being the panic of this section's `Key press after a hidden ancestor` row — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md» it now says «a key pressed after an ancestor gained `hidden` no longer panics either, this section's `Key press after a container lost its box` row — the close as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, the key press at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»; the rest of the row stands.
    sidecar: Select list closing no longer names the key-press panic as standing, its cross-reference following the retired row to its new name.
    rationale: This passage still describes the retired panic as standing and points at the row that retires; the report's sweep lists line 48 alone, and this sentence is a further occurrence of the same claim (.andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:23, :33, :50).
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:33
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 44
    section: "## Error Handling"
    change: New row after this line, in its own group under the `values · errors` origin marker the rows around it carry, reading «Mounted event of a removed element — `flush_queued_mounted_events` skips an element removed since its event was queued, through `try_element_to_node_id` (packages/dioxus-native-dom/src/dioxus_document.rs:235-239); upstream's `element_removed_before_its_mounted_event_is_flushed` is left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0 — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
    sidecar: New row, a mounted event queued for an element since removed is skipped through `try_element_to_node_id`, from `6ef4b532d`, upstream's test left out.
    rationale: The take adds a skip beside the file's mounted-handle panic row and the body's graceful-degradation list, which states `try_element_to_node_id` only as an option and stays unchanged (.andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:20, :56).
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 56
    section: "## Error Handling"
    change: New row after this line, in its own group under the `values · errors` origin marker the rows around it carry, reading «Failed stylesheet load — `NetHandler::error`, taking `url` and `error`, is on the public trait, a default that does nothing (packages/blitz-traits/src/net.rs:36-38); `blitz-net`'s error arm calls `handler.error(url, e.to_string())` (packages/blitz-net/src/lib.rs:313), the waker after the handler (packages/blitz-net/src/lib.rs:316); `ResourceHandler<StylesheetHandler>::error` responds `Err(error)` (packages/blitz-dom/src/net.rs:144-146), so `pending_critical_resources` loses the request and a failed, empty or non-CSS stylesheet stops blocking render (tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-67; tests/blitz-tests/tests/render_blocking_stylesheet.rs:69-91); blitz-test-harness's `CountedHandler` forwards the call (packages/blitz-test-harness/src/settle.rs:121-125), so a failed load ends the wait on a harness document and on the driver, which without the forward read `NotSettled(Loads)` for good; a bad `href` starts no load, `resolve_url` giving way to `doc.url.resolve_relative` (packages/blitz-dom/src/mutator.rs:1392-1394; tests/blitz-tests/tests/render_blocking_stylesheet.rs:93-107); the error text stops in `ResourceLoadResponse` and the `Resource load failed` line, `url` and `error` in escher-telemetry's scrub set, and no carrier takes it — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
    sidecar: New row, the failed-stylesheet error path from `f6edb9701` and the `CountedHandler` forward, where a failed load now reaches its handler, stops blocking render and ends the wait.
    rationale: The chunk gives a failed load a handler path it did not have, with a new end to `NotSettled(Loads)` on the driver, and no row of this file states it (.andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14, :25, :31, :103, :112).
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14
note: .andromeda/security-plan.md:326 says of `NetProvider::fetch` «the handler is not called on error», which `f6edb9701` makes false (.andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14); it lies in the body and no row of this file holds it, so it is no proposal here.
