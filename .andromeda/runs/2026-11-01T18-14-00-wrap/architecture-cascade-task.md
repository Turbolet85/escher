# Task — architecture · 2026-11-01-upstream-sync-ui-graph

## The session
Took twelve upstream DioxusLabs/blitz commits by no-commit cherry-pick, `upstream/main` read at `e42ee1f25`, merge base `23b9b26a9` unmoved, thirty-six of 57 now taken and twenty-one left out; each is an engine behaviour change for every document unless stated; manifests, lock and `deny.toml` are byte-identical.
Added escher's own changes: the key-press panic's fix in `BaseDocument::deallocate_anonymous_block`, `Harness::text_contrast` reading `None` past a blend, `CountedHandler` forwarding `NetHandler::error`, and the new check `key_press_after_container_lost_its_box`.
Changed the gates: a new `embolden` leg, last in `fast`, two new CI-script pins, and `matrix_test`'s free-disk step removed; CI read green, 17 of 17 checks, on `CI#41333172440` at `45c333f16`.
Retired: the key-press panic, `matrix_test`'s free-disk step, the gap that no local gate built `blitz-paint/font-embolden`, `select_popup`'s row-3 skip, the orphan Re-export taffy comment, and `Node`'s `TODO: mix-blend-mode`.
Measured: Cells at 47,089 bytes, 740 lines and 5 parts, disproving the standing 47,949; two test-only TCP loopback binds in `blitz-net`'s tests; no served port, env read, print, `tracing` site or wall-clock read added.

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
- Upstream sync line (body line 291, project-intent line 9): read `ebacd68bd` becomes `e42ee1f25`, twenty-four of forty becomes thirty-six of fifty-seven, sixteen left out becomes twenty-one, the evidence pointing at this chunk's lock record; the merge base and the waiting Stylo, Taffy and Dioxus versions stand.
- Upstream history in established-decisions lines 67 and 69: the last take's record stays true; this take takes a new sentence beside it.
- Lines owned by this entry (body line 62, established-decisions lines 75, 76, 77, 79, 85, standard-contracts line 334): each re-pointed to the owner the card names, or to nothing where the owed thing landed; the merge surfaces go to the entry Upstream sync ahead of determinism and exploration.
- `fast` leg's order (stack-and-technologies line 76, standard-contracts lines 324 and 327): fmt, clippy, test, ci-scripts, then `embolden`.
- `font-embolden` gap (standard-contracts line 334): closed, now the `embolden` step of `fast`; established-decisions line 32 and standard-contracts line 118 state the feature itself and do not move.
- Free-disk steps: occupied-resources line 110's `matrix_test` step is removed; occupied-resources line 104 and stack-and-technologies line 73 name `Stand checks repeated`'s step, which stands, its watch at 3 of 3 green, retirement condition met, not shown to be a cure.
- `text_contrast`'s `None` conditions (standard-contracts line 131, body line 163): the list gains an element on the walk with a `mix-blend-mode` other than `normal`.
- Bootstrap scripts (existing-scopes line 146): nine becomes ten, `url_search_params` first; standard-contracts line 240's list of globals gains `URLSearchParams`.
- Test-only binds (occupied-resources line 8): one becomes three; line 11 is a search over another file set and does not move.
- The no-network-port wording (body line 205, inherited-defaults' API style): reads as no served port, since two tests now bind TCP on loopback; reaches them through the new test-only binds.
- `NetHandler` (standard-contracts lines 103, 9, 135): `error` stands beside `bytes`, per resource kind; body line 157 only lists the trait and does not move.
- Stylesheet citations through `NetHandler` (conventions line 53, established-decisions line 45, existing-scopes line 105): digits move.
- Render-blocking rule (dec-resource-loading, established-decisions line 45): a head stylesheet still blocks while pending, and a failed one no longer stays pending, through `f6edb9701`.
- Load wait (standard-contracts line 227, `LOAD_WAIT`): a failed load ends the wait where it would have run to the bound; `settle_waiting` is unedited.
- Harness settle contract (contract-test-harness, standard-contracts line 135): the settle reader's load report changes through the `CountedHandler` forward.
- `Widget::attribute_changed` (standard-contracts line 82): existing attributes now replay on `set_custom_widget`; the other rows name the feature and do not move.
- Resolved style (standard-contracts line 28): `width` and `height` now read sub-pixel.
- Mounted events (standard-contracts line 266): `try_element_to_node_id` gains the guard as a reader.
- Orphan comment (stack-and-technologies line 22): gone, `pub use clock` unmoved; occupied-resources line 59 is escher-telemetry's own `lib.rs`, another file sharing the digits, and does not move.
- Cells byte count (existing-scopes line 137): 47,949 becomes 47,089, 740 lines, 5 parts, stale since an earlier chunk, not moved by this take.
- Test inventories in existing-scopes: blitz-paint's modules, blitz-vibey-script's tests and blitz-tests' targets each gain a name; blitz-test-harness lib tests move 22 to 24 where a site counts them, line 79 sharing the token only.
- Target counts through the new check: `run all` moves, 727 to 733; `run stand` 237 over 51 files and the stand-repeat's seventeen targets stay, the check holding no `stand_` prefix and joining no leg.
- Snapshot reading (conventions' DOM API patterns): `BaseDocument::snapshot_node` takes a new `attr`, callers at `packages/blitz-dom/src/mutator.rs:288` and `:429`; `snapshot_node_and` at cross-cutting-patterns line 40, established-decisions line 124 and standard-contracts line 17 is another symbol and does not move.
- Invalidation rules (crosscut-invalidation-and-state-integrity): restyle reach narrows and `append_text_to_node` also damages the parent, through `fe33b778b` and `f76777766`; the changed-set insert in `remove_and_drop_all_children` stays first.
- Focus fixup under `inert` (body lines 60, 76, 145, established-decisions lines 76 and 123, standard-contracts line 42): re-read, unmoved; `DocumentMutator::flush` still reads `is_inert()` alone.
- Anonymous-block mentions (cross-cutting-patterns line 39, established-decisions line 116, standard-contracts lines 9 and 57): re-read, true as they stand, sharing the token with the panic's fix.
- Painting group (dec-painting, established-decisions line 161): `packages/blitz-paint/src/render.rs` and `packages/blitz-dom/src/document.rs`, its named merge surface, both took this take; the line's word isolation there is another sense.
- Open select list painting (contract-paint-and-shell): `paint_select_list`'s call holds no hunk, the list still painting last, now past the blend layers.
- `timeout-quarantine` (established-decisions line 197): two WPT files join it; the line names the file and states no count, so it does not move.
- Line citations: digits into every reached file above move, notably `packages/blitz-dom/src/net.rs`, `packages/blitz-dom/src/document.rs`, `packages/blitz-dom/src/mutator.rs`, `packages/blitz-dom/src/stylo.rs`, `packages/blitz-net/src/lib.rs`, `.github/workflows/ci.yml`, `.github/scripts/ci-leg.sh`, `.github/scripts/test_ci_workflows.py`, for instance `ci-scripts` at standard-contracts line 325.

## New facts
- F1 · established-decisions · The twelve upstream behaviour changes of this take are ratified as engine changes for every document, each by its commit: `fe33b778b` `8b1a0a3ce` `dee854c76` `f6edb9701` `6fca6308d` `7a3dd8918` `91ef65e21` `a26c6ddeb` `f76777766` `6ef4b532d` `ae774d7f4` `e42ee1f25`.
- F2 · established-decisions · The panic's fix is a direct engine change for every document, ratified and flagged upstreamable with nothing sent: `deallocate_anonymous_block` takes the block's `layout_children` and sets `layout_parent` to `None` on each child still naming the block, so a key press on a control whose block was dropped no longer panics, for `hidden`, `style="display:none"` and CSS alone; focus stays and the key still acts, and its merge surface is owned on the next sync's entry.
- F3 · project-intent · Upstream's own tree panics the same way, at `packages/blitz-dom/src/node/node.rs:1740:27` at `e42ee1f25`, focused by `Tab`.
- F4 · established-decisions · `AGENTS.md` from `22f0cd000` stays out of the tree: no build reads it and its instructions contradict this fork's.
- F5 · established-decisions · `background-clip: text` is unsupported: `BackgroundClip::Text` is `gecko`-only on Stylo 0.22, so `ac6f9f582` was tried and left out.
- F6 · established-decisions · `pub(crate) fn init_stylo_prefs`, behind a `Once`, is called from `UserAgentStylesheet::parse` and `BaseDocument::new`, its body the Stylo 0.22 pref names, upstream's 0.23 `set_pref!` names not written.
- F7 · established-decisions · A failed, empty or non-CSS stylesheet stops blocking render, and a bad `href` starts no load, `resolve_url` giving way to `doc.url.resolve_relative`.
- F8 · established-decisions · `<picture>` is no longer `display: contents`, upstream's `picture` rule leaving `packages/blitz-dom/assets/default.css`, every select rule staying.
- F9 · established-decisions · `compute_layout_damage`'s `table_needs_rebuild` takes `ALL_DAMAGE` when a table box's position, padding or border changes.
- F10 · established-decisions · `render_element`'s border box and culling stand on the scaled size, `location` and `box_position` kept for the transform, a port of `e42ee1f25` since upstream's stands on the untaken `ac6f9f582`.
- F11 · standard-contracts · `DocumentMutator::restyle_for_attribute_change` and `restyle_for_child_list_change` replace the blanket `RestyleHint::restyle_subtree()` on `set_attribute`, `clear_attribute`, removal and insertion.
- F12 · standard-contracts · `TElement::borrow_data` filters data with no primary style, and the style-resolve context is scoped.
- F13 · standard-contracts · `NetHandler::error(self: Box<Self>, url, error)` is new on the public trait, a default that does nothing; `blitz-net`'s error arm calls it and its waker moves after the handler.
- F14 · standard-contracts · `ResourceHandler<StylesheetHandler>::error` responds `Err(error)`, so `pending_critical_resources` loses the request.
- F15 · standard-contracts · blitz-test-harness's `CountedHandler` forwards `NetHandler::error`, so a failed load reaches the document on a harness or driver session and ends the wait rather than reading `NotSettled(Loads)` for good.
- F16 · standard-contracts · `mix-blend-mode` and `isolation` now paint: private module `blend` in blitz-paint with `pub(crate) convert_blend_mode`, `LayerManager::maybe_with_layer` and `maybe_push_layer` taking a `BlendMode`, and `render_element` pushing a viewport-wide blend layer and popping it last.
- F17 · standard-contracts · No fork or upstream test paints a blended element to a frame or names `convert_blend_mode`; the unit tests are `packages/blitz-paint/src/layers.rs`'s 3.
- F18 · standard-contracts · `Node`'s stacking-context arm reads a `mix-blend-mode` other than `normal`.
- F19 · standard-contracts · `Harness::text_contrast` reads `None` past a blended element through `paints_more_than_colour`, `Session::lint`'s `contrast` reading the same with no second formula and no stand row moving.
- F20 · standard-contracts · `NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN` and the public reader `Node::is_inline_with_only_block_children`, set in `collect_layout_children_with_wrap`; `render_element` skips such a box's own shadow, background and border through `paints_own_box`, with no frame test.
- F21 · standard-contracts · `URLSearchParams` is a global class with `URL.prototype.searchParams`, `href` and `search` live both ways, a realization on the registered script crossing; its native `parse_search_params`, registered as `__blitz_parse_search_params`, is deleted from `globalThis` before any page script.
- F22 · standard-contracts · `flush_queued_mounted_events` skips an element removed since its event was queued; upstream's test `element_removed_before_its_mounted_event_is_flushed` is left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0.
- F23 · standard-contracts · `.github/scripts/ci-leg.sh` holds the leg `embolden`, `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, with no CI job, macOS exercising the feature there, pinned by `test_fast_leg_ends_on_the_font_embolden_witness`.
- F24 · standard-contracts · `test_only_stand_repeat_frees_disk` pins that only the `stand-repeat` job frees disk.
- F25 · occupied-resources · `blitz-net`'s `#[cfg(test)] mod tests` binds `std::net::TcpListener::bind("127.0.0.1:0")` twice, `packages/blitz-net/src/lib.rs:526` and `:534`, test-only, served by no binary, reached by no caller, no crossing.
- F26 · existing-scopes · `packages/blitz-paint/src/blend.rs` is a new private module of blitz-paint, and `url_search_params` a new JS bootstrap script and test target of blitz-vibey-script.
- F27 · existing-scopes · `tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs` is a new blitz-tests target, 3 tests in both layout modes, no `stand_` prefix, a key still moving a select's choice past its dropped block.

## Untouched
- Manifest-held decisions (dependency pinning, build profiles, default features, release versioning, publishability, the crate list and `members`): no `Cargo.toml`, `Cargo.lock` or `deny.toml` moved, and no crate was added or removed.
- Driver session contract and decisions other than the load wait (twelve verbs, eight causes, the `v3` wire, the command line, recording, state directory, Unix-domain socket): `packages/escher-driver` holds no diff.
- Telemetry bootstrap, logging rules and the scrub set (`SAFE_FIELDS`, `CONTENT_FIELDS`, the panic hook): `packages/escher-telemetry` holds no diff, and no log site was added.
- Headless stand (`seven_guis::stand`, `LoadNet`, `LOAD_ANSWER_DELAY`, the boots): `examples/seven_guis` holds no diff.
- Environment variables: no env read added.
- Select model and open-list machinery in `node::select` and `node::select_list`, `SelectListView`'s constants included: the sheet's select rules stand and neither module is reached.
- Browser app, rdme, their stores and navigation providers: nothing under `apps/` is reached.
- Shell, windowing, the accessibility adapter, keyboard shortcuts: `packages/blitz-shell` and `packages/accesskit_xplat` are not reached.
- WPT runner's budgets, suites and output files: only its quarantine list moved, not `wpt/runner/src`.
- Node identity (`NodeId`'s packing and ordering): the panic's fix stops an id going stale and leaves `packages/blitz-traits/src/node_id.rs` as it stands.
- HTML fixtures under `examples/assets`: no fixture is reached.
