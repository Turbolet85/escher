## 2026-11-01-upstream-sync-ui-graph — no served port, two test-only TCP binds
**Section:** §API Security (security-plan.md: the opening line; api-security.md: the opening row)
**Change:**
- The opening row reads no served TCP or UDP port (was no TCP or UDP port): two `blitz-net` tests each call `std::net::TcpListener::bind("127.0.0.1:0")` in the crate's `#[cfg(test)] mod tests`, served by no binary, reached by no caller, so no crossing, registered as test-only.
- The master's opening line reads no served TCP or UDP port (was no TCP or UDP port), two `blitz-net` tests alone binding TCP on loopback, served by no binary.
**Why:** the chunk measures two new binds, both TCP on loopback, in one test module, with no new served port; read as no served one, the claim changes by a word saying the two tests bind, and session rule 2026-10-15 owes their registration as test-only in the security plan.
**Kept:** the driver session's Unix-domain socket stays the one listener, and the `mcp` role binds nothing. The row's searches for `TcpListener` over other file sets stand unedited, adopted observations.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — a failed load reaches the handler and ends the wait
**Section:** §API Security (api-security.md: Settle and loads in flight); §Error Handling (security-plan.md: Internal logging; error-handling.md: a new group, Failed load answered through the handler)
**Change:**
- Settle and loads in flight gains: a failed load ends the wait. `CountedHandler` forwards `NetHandler::error`, so a failed stylesheet reads no `NotSettled(Loads)` for good on a harness document or on the driver, and `settle_waiting` returns where it would otherwise have run to `LOAD_WAIT`.
- New row: `NetHandler::error` stands on the public trait, a default that does nothing, so a wrapping implementer that does not forward it compiles and drops the failure silently; `ResourceHandler<StylesheetHandler>::error` responds `Err(error)`, so `pending_critical_resources` loses the request and a failed, empty or non-CSS stylesheet does not block render.
- The master's `NetProvider::fetch` line: was a failed fetch logged when tracing is enabled and otherwise discarded, the handler not called on error; now logged when tracing is enabled and handed to the handler through `NetHandler::error`, a default that does nothing, with or without tracing.
- The master's failed-loads line gains: a failed stylesheet's request leaves `pending_critical_resources`, so it blocks no render.
**Why:** `f6edb9701`, one of the twelve commits the founder took by his answer to fork F2, option A (2026-11-01, by dialog in the overseer session, relayed verbatim by the operator), adds the method and has `blitz-net`'s error arm call it, which makes the old master claim false. Its fix stopped at `CountedHandler` on every harness-built document until the chunk's forward, a companion of that commit. Trap for later takes: a trait's do-nothing default can hide a fix through a wrapper no conflict or gate touches, so sweep the implementers of a trait a take extends.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — citations re-pointed to the chunk's tree
**Section:** api-security.md · authentication-authorization.md · dependency-security.md · error-handling.md · input-validation.md · logging-monitoring.md · threat-model-summary.md · security-plan.md (§Data Protection, §Dependency Security, §Error Handling, §Logging & Monitoring)
**Change:**
- 61 lines change in their citation digits alone: api-security 2, authentication-authorization 1, dependency-security 2, error-handling 2, input-validation 37, logging-monitoring 1, threat-model-summary 1, the master 15. The rows that also change in wording carry their moved digits too, and their own entries name that wording.
**Why:** the take and the chunk's own edits moved the cited lines; the digits follow, and none of these 61 lines changes a claim.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — this take's lock record
**Section:** §Dependency Security (dependency-security.md: the upstream sync records)
**Change:**
- New record: the upstream sync read at `e42ee1f25` took twelve commits, `fe33b778b` · `8b1a0a3ce` · `dee854c76` · `f6edb9701` · `6fca6308d` · `7a3dd8918` · `91ef65e21` · `a26c6ddeb` · `f76777766` · `6ef4b532d` · `ae774d7f4` · `e42ee1f25`, by no-commit cherry-picks (`45c333f16`), thirty-six of 57 taken and twenty-one left out with their measured reasons, the merge base staying `23b9b26a9`.
- `Cargo.toml`, `Cargo.lock`, `deny.toml` and every crate manifest are byte-identical to the tree before it, so no package enters, leaves or moves.
- The pins stay: Stylo "0.22.0", Taffy the registry "0.15", Dioxus "0.7.3", winit `=0.31.0-beta.3`, no git dependency. The audit leg reads `advisories ok` and `licenses ok`.
**Why:** the take is the founder's answer to fork F2, option A, twelve commits (2026-11-01, by dialog in the overseer session, relayed verbatim by the operator). The chunk measures the manifests, the lock and `deny.toml` unmoved and the audit green.
**Kept:** the record of the last take stands beside this one as its history.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the `matrix_test` free-disk step retired
**Section:** §Dependency Security › Supply chain integrity (dependency-security.md: CI action, the `matrix_test` free-disk step)
**Change:**
- Was the same action's step on the `matrix_test` job, skipped on every matrix row, its `if:` naming `ubuntu-24.04` and `ubuntu-24.04-arm` while the android row runs `ubuntu-latest`; now the job holds no such step: `Test (android)` reads none in the chunk's CI run, and `test_only_stand_repeat_frees_disk` pins the action to `Stand checks repeated`.
**Why:** the chunk removed the step from `ci.yml`, and the run bears it out.
**Kept:** the row on `Stand checks repeated`'s free-disk step stands unedited, still not shown to be the cure for the runner out of disk.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the `embolden` leg, locked
**Section:** §Dependency Security › Supply chain integrity (dependency-security.md: a new row, Lockfile verification, the `embolden` leg)
**Change:**
- New row: `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted` is a locked cargo leg, in `LEGS` and last in `FAST_LEGS`, with no CI job of its own, macOS exercising the feature there.
- The master's Lockfile verification line takes no new name, only its citation moving; the leg stands in the detail row.
**Why:** the chunk adds the leg to `ci-leg.sh` with `--locked` and pins its place at the end of `fast` with a check.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the key-press panic retired
**Section:** §Error Handling (error-handling.md: Key press after a hidden ancestor, Anonymous block dropped, Select list closing); §Threat Model Summary (threat-model-summary.md: the remote web content vector)
**Change:**
- The row `Key press after a hidden ancestor` goes, no tombstone (was a panic in `position_in_containing_block` a remote page can reach, not fixed, owned by a CARRY on «Upstream sync ahead of the UI graph»).
- Now `Anonymous block dropped`: `BaseDocument::deallocate_anonymous_block` takes the block's `layout_children` and sets `layout_parent` to `None` on each child still in the slab naming the block, so the id does not go stale and every reader of a stale `layout_parent` is cured with it. A key press with focus on a control whose anonymous block was dropped does not panic, for `hidden`, `style="display:none"` and CSS alone; focus stays and the key still acts, `DocumentMutator::flush` unedited. Witnessed by `key_press_after_container_lost_its_box`, 3 tests in both layout modes, and by row 3 of `select_popup`.
- `Select list closing`: was a claim of the close alone, a key pressed after an ancestor gained `hidden` being the panic; now that key does not panic, `select_popup` pressing it in every row.
- The threat model's sentence on the remote key-press panic goes (was a limit on the vector); its sentence on the engine-painted select list stands.
**Why:** the fix is escher's own, on the founder's answer to fork F1, option A, the panic removed with focus staying (2026-11-01, by dialog in the overseer session, relayed verbatim by the operator). That F1 A says focus stays and keys still act is the operator's reading, entered as his on the founder's recorded answers and not a new word of the founder's; the ratification stays those answers. A direct engine change for every document.
**Kept:** this plan writes no row on the limit F1 A leaves, a focused control that no longer shows keeping focus and taking keys; the answer card places it in a11y-plan §5.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the mounted event of a removed element
**Section:** §Error Handling (error-handling.md: a new row, Mounted event of a removed element)
**Change:**
- New row: `flush_queued_mounted_events` skips an element removed since its event was queued, through `try_element_to_node_id`, an engine behaviour for every document. The upstream test `element_removed_before_its_mounted_event_is_flushed` is left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0.
**Why:** `6ef4b532d`, one of the twelve commits of the founder's answer to fork F2, option A (2026-11-01, by dialog in the overseer session, relayed verbatim by the operator). Its test cannot read on Dioxus 0.7 and returns with `286874cc1`.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — a stylesheet that fails blocks no render
**Section:** §Input Validation (security-plan.md: Fetched stylesheets, Stylesheet `href`; input-validation.md: Fetched stylesheets, a new row, Stylesheet `href`)
**Change:**
- Fetched stylesheets keeps its UTF-8 check and gains, in every document, remote pages included: a failed, empty or non-CSS stylesheet blocks no render, a load that errs reaching `ResourceHandler<StylesheetHandler>::error`, which responds `Err(error)`, so `pending_critical_resources` loses the request. Pinned by `failed_stylesheet_unblocks_rendering` and by `empty_or_non_css_stylesheet_unblocks_rendering` over an empty body, an HTML page and the byte 0xFF.
- New row and broad line, Stylesheet `href`, in every document, remote pages included: an `href` that cannot be resolved starts no load and blocks no render, read through `doc.url.resolve_relative` and not `resolve_url`. Pinned by `invalid_stylesheet_url_does_not_block_rendering`.
- Both rows read an engine change for every document.
**Why:** `f6edb9701`, one of the twelve commits of the founder's answer to fork F2, option A (2026-11-01, by dialog in the overseer session, relayed verbatim by the operator), answers a failed load and stops a bad `href` starting one.
**Kept:** `resolve_url` itself still panics on a URL it cannot resolve, standing under Panic paths; this path no longer goes through it.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — page CSS paints blending and isolation
**Section:** §Input Validation (input-validation.md: a new CSS declarations row, `mix-blend-mode` · `isolation`)
**Change:**
- New row, in every document, remote pages included: page CSS paints both, validated by Stylo's parse and mapped by `convert_blend_mode`, `pub(crate)` in blitz-paint's private `blend` module. `render_element` pushes a viewport-wide blend layer and pops it last, and `Node`'s stacking-context arm reads a `mix-blend-mode` other than `normal`. 3 unit tests in `layers.rs` hold the layers, and no fork or upstream test paints a blended element to a frame or names `convert_blend_mode`. An engine change for every document.
**Why:** `6fca6308d`, one of the twelve commits of the founder's answer to fork F2, option A (2026-11-01, by dialog in the overseer session, relayed verbatim by the operator). The frame gap is placed by the operator, a CARRY on «Upstream sync ahead of determinism and exploration», his placement and not the founder's word.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — a custom widget's attributes replayed at set-up
**Section:** §Input Validation (input-validation.md: a new Markup attributes row, a custom widget's attributes at `set_custom_widget`)
**Change:**
- New row, behind the `custom-widget` feature: `ElementData::set_custom_widget` replays each existing attribute, parsed page content included, to `Widget::attribute_changed` with no old value, so a widget receives them at set-up and not only on a later change, the value's check staying the widget's own. Pinned by `custom_widget_replays_initial_attributes`. An engine change for every document behind the feature, ratified.
**Why:** `a26c6ddeb`, one of the twelve commits of the founder's answer to fork F2, option A (2026-11-01, by dialog in the overseer session, relayed verbatim by the operator), on which the row reads ratified.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — culling on the scaled size, the inline box's paint skip
**Section:** §Input Validation (input-validation.md: Paint values, Geometry)
**Change:**
- Geometry keeps its cull of non-finite or beyond f32::MAX bounds and gains: `render_element`'s border box and culling stand on the scaled size, `location` and `box_position` kept for the transform.
- A box flagged `IS_INLINE_WITH_ONLY_BLOCK_CHILDREN` paints no shadow, background or border of its own, pinned by `inline_with_only_block_children_tests` 4, no frame read.
**Why:** `e42ee1f25`, a port since upstream's stands on an untaken commit, and `f76777766`, both of the twelve commits of the founder's answer to fork F2, option A (2026-11-01, by dialog in the overseer session, relayed verbatim by the operator). The paint skip's frame gap is placed by the operator, a CARRY on «Upstream sync ahead of determinism and exploration», his placement and not the founder's word.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — `URLSearchParams` on the script crossing
**Section:** §Input Validation (input-validation.md: a new JS API arguments row); §Threat Model Summary (threat-model-summary.md: the script execution vector)
**Change:**
- New row: a page's script reaches `URLSearchParams` and `URL.prototype.searchParams`, `href` and `search` live both ways, through `url_search_params.js`, first of the ten bootstrap scripts. Bad calls and receivers throw TypeError and values pass `usv`. The native `parse_search_params`, registered as `__blitz_parse_search_params`, is deleted from `globalThis` before any page script. No log site and no carrier. Pinned by `url_search_params` 4. An engine change for every document, ratified.
- The script vector's trust boundary gains the same surface: a realization on this crossing, no boundary widening, the helper deleted before any page script runs and its argument checks held under Input Validation.
**Why:** `91ef65e21`, one of the twelve commits of the founder's answer to fork F2, option A (2026-11-01, by dialog in the overseer session, relayed verbatim by the operator), on which the row reads ratified. No boundary widening is read: a realization on the registered script crossing, as the fork stated it to him.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — where a failed load's error text stops
**Section:** §Logging & Monitoring (logging-monitoring.md: a new group, A failed load's error text)
**Change:**
- New row: the text stops in `ResourceLoadResponse` and the `Resource load failed` line, no carrier takes it. On that line the sink prints `waiting_nodes` and every other field, the message included, `[redacted]`, `url` and `error` also in its scrub set. `NetHandler::error` holds no `tracing` site.
**Why:** the chunk measures the stop and the scrub set, neither moved. The report's own line has the sink printing the message; this pass read the sink's format code, which redacts it, so the row states that read and does not carry the report's wording. A read of this pass, no ruling.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/
