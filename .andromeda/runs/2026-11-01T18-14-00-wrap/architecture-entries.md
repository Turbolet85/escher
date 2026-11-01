## 2026-11-01-upstream-sync-ui-graph — the citations re-pointed past the take
**Section:** §Established Decisions, §Conventions, §Standard Contracts, §Occupied Resources, §Infrastructure Patterns, §Cross-Cutting Patterns and §Inherited Defaults (architecture.md) · §Conventions (conventions.md) · §Cross-Cutting Patterns (cross-cutting-patterns.md) · §Design Philosophy (design-philosophy.md) · §Established Decisions (established-decisions.md) · §Existing Scopes (existing-scopes.md) · §Inherited Defaults (inherited-defaults.md) · §Occupied Resources (occupied-resources.md) · §Project Intent (project-intent.md) · §Stack and Technologies (stack-and-technologies.md) · §Standard Contracts (standard-contracts.md)
**Change:**
- Citation digits only: outside the rows the entries below name, every changed line reads as it did but for its digits, and no value moved; conventions.md and inherited-defaults.md change in nothing else.
- The offsets of the citations that moved: `document.rs` +2, +11, +22, +24, +32 or +269; `mutator.rs` -26, -16, +2, +6, +72, +74 or +84; `construct.rs` -4, +3, +5 or +18; `damage.rs` +18 or +20; `node/node.rs` +3, +11 or +13; `stylo.rs` +3 or +8; `blitz-dom`'s `net.rs` +4; `blitz-dom`'s `lib.rs` -2; `element.rs` +3; `blitz-traits`' `net.rs` +3; `blitz-net`'s `lib.rs` -3 or -1; `render.rs` +2, +5, +39 or +40; `blitz-paint`'s `lib.rs` +1; `layers.rs` +3; `background.rs` +1; `settle.rs` +6; `bootstrap.rs` +8; `dioxus_document.rs` +4; `render_blocking_stylesheet.rs` +69; `select_popup.rs` -4; `ci-leg.sh` +2; `test_ci_workflows.py` +7 or +23; `ci.yml` -8.
- Some ranges moved at their end with their start unmoved, among them `ci.yml`'s `34-431` to `34-423`, `ci-leg.sh`'s `18-69` to `18-71`, `test_ci_workflows.py`'s `103-501` to `103-524` and `select_popup.rs`'s `202-1186` to `202-1182`.
**Why:** this chunk edited each of these sources, all on the report's Modified list; among the edits, `matrix_test` lost its free-disk step, `ci-leg.sh` gained `embolden`, `blitz-dom`'s `lib.rs` lost a two-line comment and `select_popup`'s row-3 skip went.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — a failed stylesheet through `NetHandler::error`
**Section:** §Cross-Cutting Patterns (cross-cutting-patterns.md: service wiring, a new row) · §Design Philosophy (design-philosophy.md: embedder services, a new row) · §Established Decisions (established-decisions.md: [Resource loading]) · §Standard Contracts (standard-contracts.md: Document core, a new row; Provider traits; HTML and networking crates) · §Infrastructure Patterns (architecture.md: Runtime threading and event flow)
**Change:**
- `NetHandler::error(self: Box<Self>, url: String, error: String)` reads beside `bytes` on the public trait, a default that does nothing.
- blitz-net's `Provider`: was waking the waker for `doc_id` then calling `handler.bytes`; now calling `handler.bytes` on a success or `handler.error(url, e.to_string())` on a failure, then waking the waker.
- `ResourceHandler<StylesheetHandler>::error` responds `Err(error)`, so `pending_critical_resources` loses the request and a failed, empty or non-CSS stylesheet stops blocking render, on every document; `net.rs`'s other resource kinds keep the default.
- A wrapping handler must forward the method or the failure stops at the wrapper, as blitz-test-harness's `CountedHandler` forwards it.
- [Resource loading] adds that a bad `href`, `doc.url.resolve_relative` refusing it, starts no load.
**Why:** the report measures upstream's #1318, `f6edb9701`, one of the twelve, an engine change for every document. Its Decisions name the trap: a trait default method can hide a take's fix, since the do-nothing default compiled silently through `CountedHandler`, so a later take sweeps the implementers of a trait it extends.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the queued `mounted` event's guard
**Section:** §Cross-Cutting Patterns (cross-cutting-patterns.md: event processing, the new detail block `crosscut-event-processing`) · §Standard Contracts (standard-contracts.md: Dioxus DOM bridge, one row amended, one new)
**Change:**
- New rows: `flush_queued_mounted_events` skips an element removed since its event was queued, reading it through `try_element_to_node_id`.
- The `vdom_state.try_element_to_node_id(ElementId)` clause adds that this guard also reads it.
- Upstream's test `element_removed_before_its_mounted_event_is_flushed` is recorded as left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0.
**Why:** the report measures upstream's #1305, `6ef4b532d`, one of the twelve, and the test left out; the operator's word on the phase card returns the test with `286874cc1` through a `CARRY:` (inputs#I6), an owner this row does not name.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — restyle on mutation
**Section:** §Cross-Cutting Patterns (cross-cutting-patterns.md: invalidation and state integrity, the new detail block `crosscut-invalidation-and-state-integrity`) · §Standard Contracts (standard-contracts.md: Mutation, query and CSSOM, a new row; Scrolling, selection, tree, a new row)
**Change:**
- `set_attribute`, `clear_attribute`, removal and insertion restyle through `DocumentMutator::restyle_for_attribute_change` and `restyle_for_child_list_change`, with no blanket `RestyleHint::restyle_subtree()`; the changed-set insert in `remove_and_drop_all_children` stands first.
- Public `BaseDocument::snapshot_node` takes `attr: &LocalName`, its two callers both in `mutator.rs`.
- `append_text_to_node` also damages the parent.
- `TElement::borrow_data` filters data with no primary style, and the style-resolve context is scoped.
**Why:** the report measures upstream's #1276, `fe33b778b`, one of the twelve, an engine change for every document, and the parent's damage from upstream's #1319; it reads that `standard-contracts.md` owed the restyle helpers a line.
**Kept:** the rows naming `snapshot_node_and` stand: another symbol, which the report reads no change on.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the inline box's paint skip
**Section:** §Design Philosophy (design-philosophy.md: the `Node` sentence) · §Standard Contracts (standard-contracts.md: Nodes and widgets, a new row)
**Change:**
- `Node`: was carrying tree-structure information and one recorded style fact; now also one box-construction fact, the flag `NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN`, which `collect_layout_children_with_wrap` sets and the public `Node::is_inline_with_only_block_children` reads.
- `render_element` skips such a box's own shadow, background and border through `paints_own_box`, with no frame test; an engine change for every document.
**Why:** the report measures upstream's #1319, `f76777766`, one of the twelve; its Coverage row reads the flag's unit tests in `document.rs` and no frame test. The frame gap's placement is the operator's, a `CARRY:` on «Upstream sync ahead of determinism and exploration» (Q2, inputs#I10); these rows name no owner.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the Stylo prefs behind a `Once`
**Section:** §Established Decisions (established-decisions.md: [Document defaults]) · §Occupied Resources (occupied-resources.md: process-wide state and threads, a new row)
**Change:**
- [Document defaults] adds `pub(crate) fn init_stylo_prefs`, behind a `Once`, setting the prefs, called from `UserAgentStylesheet::parse` and `BaseDocument::new`, its body the Stylo 0.22 pref names, upstream's 0.23 `set_pref!` names not written.
- New row: the `Once` `INIT` is process-wide; the first call sets Stylo's prefs before the user-agent sheet is parsed, a later call does nothing, and it has no log site.
**Why:** the report measures upstream's #1310, `7a3dd8918`, one of the twelve, and that upstream's 0.23 names were not written, the 0.22 names standing; a process-wide static belongs to the category §Occupied Resources registers.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — `background-clip: text` left out
**Section:** §Established Decisions (established-decisions.md: [Unsupported features])
**Change:**
- Adds that `background-clip: text` is not supported, `BackgroundClip::Text` being `gecko`-only on Stylo 0.22, and that upstream `ac6f9f582`, which paints it, stays left out.
**Why:** the report measures `ac6f9f582` tried on the trial and left out for that reason, one of the twenty-one left out at `e42ee1f25`.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the merge surfaces' reading and owner at `e42ee1f25`
**Section:** §Established Decisions (established-decisions.md: [Scrolling and input], the range model and keyboard activation; the seven, the eight, the document's clock, the select model and the select list; architecture.md: the eight's line, the select list's line)
**Change:**
- New clauses: at upstream `main` `e42ee1f25` the twelve taken reach the range model's surface in `node/element.rs`, `layout/construct.rs` and `mutator.rs`, and keyboard activation's in `node/element.rs` (`ElementData::set_custom_widget`'s replay), leaving `events/keyboard.rs`, `events/mod.rs` and `events/pointer.rs` untouched.
- The merge surfaces' owner, in five detail rows and two body lines: was Epoch 3's «Upstream sync ahead of the UI graph», the eight's body line reading Epoch 3's sync; now «Upstream sync ahead of determinism and exploration». The `inputs#I5` citations on the seven, the clock and the select model go with it.
- The seven's `.andromeda/residuals.md` line: was re-read row by row with its owner quoted, read at the 2026-10-27 chunk; now each row given a named outcome and nothing sent, read at this chunk's `evidence/readings.md`.
- The eight: `hit_inner`'s conflict was with the non-invertible answer for Epoch 3's sync; now for the next sync, `0980cce5f` being left out at `e42ee1f25`.
**Why:** the sync that owned these lines has run; the operator's word on the phase card (inputs#I6) moves the merge surfaces and this take's resolutions to a `CARRY:` on «Upstream sync ahead of determinism and exploration», each flagged upstreamable and nothing sent, and the report re-points these owner rows to the owner that card names.
**Kept:** the range model's and keyboard activation's earlier merge-surface records stand, this reach beside them.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the key-press panic's fix
**Section:** §Established Decisions (established-decisions.md, a new row; architecture.md, the new line it shares with the twelve) · §Existing Scopes (existing-scopes.md: blitz-tests' engine-fact targets) · §Project Intent (project-intent.md, a new row)
**Change:**
- New row: a dropped anonymous block clearing its children's `layout_parent` stands beside the eight, a direct engine change for every document, ratified: `BaseDocument::deallocate_anonymous_block` takes the block's `layout_children` and sets `layout_parent` to `None` on each child still in the slab naming the block.
- A key press with focus on a control whose anonymous block was dropped does not panic, for `hidden`, `style="display:none"` and CSS alone; every reader of a stale `layout_parent` is cured with it, the id no longer stale; focus stays and the key still acts, `DocumentMutator::flush` reading `is_inert()` alone.
- Flagged upstreamable, nothing sent; its merge surface a line owned by «Upstream sync ahead of determinism and exploration».
- Body: the fix is one more direct engine change for every document, ratified, on the same terms.
- `key_press_after_container_lost_its_box` joins the targets, 3 tests, a button's and a select's, the select's choice still moving past its dropped block, and a control in no anonymous block reading the same; no `stand_` prefix.
- New row: upstream's own tree at `e42ee1f25` panics the same way, at `packages/blitz-dom/src/node/node.rs:1740:27`, focused by `Tab`.
**Why:** the fix stands on the founder's answer F1, option A, «Убрать падение, фокус остаётся (Рекомендую)», by dialog in the escher overseer session, 2026-11-01, his chosen label relayed verbatim by the operator (inputs#I3); that F1 A says focus stays and keys still act is the operator's reading of his fork text, not a new word of his (Q1, inputs#I10). A master body takes only the bare `ratified`. Routine under the playbook's *An engine behaviour change for every document*. Standing trap from the report: a probe of a panic can read green because its input never reaches the path, so assert the fixture first; upstream's tree passed focused by a click, its click not focusing a button.
**Kept:** the focus-fixup rows stand unedited, the fixup under `inert` unmoved. The limit F1 A leaves, a focused control that no longer shows keeping focus and taking keys, is not stated in this master; the answer card lists it under a11y-plan §5.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the twelve commits ratified for their take
**Section:** §Established Decisions (established-decisions.md, a new row; architecture.md, the new line it shares with the panic's fix)
**Change:**
- New row: twelve more behaviour changes for every document are upstream's, not counted among escher's direct engine changes, each ratified for its take, by its commit, under the playbook pattern for an engine behaviour change for every document: `fe33b778b` `8b1a0a3ce` `dee854c76` `f6edb9701` `6fca6308d` `7a3dd8918` `91ef65e21` `a26c6ddeb` `f76777766` `6ef4b532d` `ae774d7f4` `e42ee1f25`.
- No boundary is widened in them, `URLSearchParams` a realization on the registered script crossing.
- Body: the twelve commits of the upstream take read at `e42ee1f25` are engine behaviour changes for every document, ratified for their take.
**Why:** the take stands on the founder's answer F2, option A, «Двенадцать коммитов (Рекомендую)», by dialog in the escher overseer session, 2026-11-01, his chosen label relayed verbatim by the operator (inputs#I3); that F2 A names the twelve is the operator's reading of his fork text, not a new word of his (Q1, inputs#I10). A master body takes only the bare `ratified`. Routine under the playbook's *An engine behaviour change for every document*; the answer card reads no boundary widening in the twelve, as the fork stated it to him (inputs#I4).
**Kept:** the seven and the eight stay counted as they were, the twelve standing apart from escher's own direct engine changes.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — `AGENTS.md` stays out
**Section:** §Established Decisions (established-decisions.md, a new row)
**Change:**
- New row: `AGENTS.md` from upstream `22f0cd000` stays out of the tree: no build reads it and its instructions contradict this fork's.
**Why:** the report records it going into the trial and staying out for those reasons, the operator's lean (inputs#I5); it is among the twenty-one left out.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — a table box's layout damage
**Section:** §Established Decisions (established-decisions.md: [Layout])
**Change:**
- `ALL_DAMAGE` also comes, through `compute_layout_damage`'s `table_needs_rebuild`, from a table box's position, padding or border change.
**Why:** the report measures upstream's #1312, `8b1a0a3ce`, one of the twelve, with no test of its own, the workspace its witness.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — `<picture>` not `display: contents`
**Section:** §Established Decisions (established-decisions.md: [Layout])
**Change:**
- Adds that `<picture>` is not `display: contents`, `packages/blitz-dom/assets/default.css` holding no `picture` rule and every select rule standing.
**Why:** the report measures upstream's #1317, `dee854c76`, one of the twelve, its rule leaving the sheet and `picture_display_tests` its witness.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the culling bounds' port
**Section:** §Established Decisions (established-decisions.md: [Painting], the new detail block `dec-painting`)
**Change:**
- New row: `render_element`'s border box and culling stand on the scaled size, `location` and `box_position` kept for the transform, an upstream engine change for every document, upstream #1342, a port of `e42ee1f25`, upstream's own standing on the untaken `ac6f9f582`.
**Why:** the report measures `e42ee1f25` as a port for that reason, one of the twelve, with no test of its own, the workspace its witness.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — blending painted
**Section:** §Existing Scopes (existing-scopes.md: blitz-paint's modules) · §Standard Contracts (standard-contracts.md: the new detail block `contract-layout`; Paint and shell, two new rows)
**Change:**
- blitz-paint's modules gain `blend`, private, its `pub(crate) convert_blend_mode` converting Stylo's blend mode.
- New row: `Node`'s stacking-context arm reads a `mix-blend-mode` other than `normal`; an engine change for every document.
- New row: `mix-blend-mode` and `isolation` paint in every document: `LayerManager::maybe_with_layer` and `maybe_push_layer` take a `BlendMode`, and `render_element` pushes a viewport-wide blend layer and pops it last, so an open list, through the `paint_select_list` call, paints last, past the blend layers.
- New row: the blend has no frame witness: no fork or upstream test paints a blended element to a frame or names `convert_blend_mode`, its unit tests the 3 of `layers.rs`; owner: a `CARRY:` on «Upstream sync ahead of determinism and exploration».
**Why:** the report measures upstream's #1316, `6fca6308d`, one of the twelve, and its Coverage row reads the frame gap, which the operator's word required an owner for (inputs#I8). The owner is the operator's placement (Q2, inputs#I10).
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — `URLSearchParams`
**Section:** §Existing Scopes (existing-scopes.md: blitz-vibey-script's integration tests; the `runtime` module's bootstrap scripts) · §Standard Contracts (standard-contracts.md: JavaScript, one row amended, one new; architecture.md: JavaScript)
**Change:**
- JS bootstrap scripts: was nine; now ten, `url_search_params` first in `BOOTSTRAP_JS`.
- blitz-vibey-script's integration tests gain `tests/url_search_params.rs`, 4 tests of `URLSearchParams` and `URL.prototype.searchParams`.
- The bootstrap's definitions gain `URLSearchParams`, in the detail's list and the body's, with `URL.prototype.searchParams`, `href` and `search` live both ways.
- New row: a global class, a realization on the registered script crossing; its native `parse_search_params`, registered as `__blitz_parse_search_params`, is deleted from `globalThis` before any page script; two WPT files stand in `timeout-quarantine.txt` with it.
- Body: `__blitz_parse_search_params`, the bootstrap's own, is deleted before any page script.
**Why:** the report measures upstream's #1314, `91ef65e21`, one of the twelve, ten bootstrap scripts where nine stood, and two WPT files joining the quarantine; `URLSearchParams` is a realization on the registered crossing, no widening.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — Cells' screen bytes
**Section:** §Existing Scopes (existing-scopes.md: blitz-tests' served-page checks, `stand_cells_served`)
**Change:**
- Cells' screen: was 47,949 bytes; now 47,089, in 740 lines and 5 parts as it read, its citation adding this chunk's report.
**Why:** the report measures 47,089 bytes, 740 lines and 5 parts through the binary, at the base and on this tree, so the take moves nothing and the old reading has been stale since an earlier chunk. Standing trap: a plan quoting a flagged-old reading did not re-read it.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the test-only binds and no served port
**Section:** §Occupied Resources (occupied-resources.md: network ports and listeners, one row amended, one new; architecture.md: network ports and listeners) · §Inherited Defaults (architecture.md: API style)
**Change:**
- Was no network port, no TCP or UDP port; now no served network port, no TCP or UDP port a binary serves, blitz-net's tests alone binding TCP on loopback.
- Test-only binds: was one; now three, blitz-net's two and escher-driver's Unix-domain one.
- New row: `blitz-net`'s `#[cfg(test)] mod tests` binds `std::net::TcpListener::bind("127.0.0.1:0")` once each in `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`, served by no binary and reached by no caller, so no crossing and no served port.
- `still no second listener` now reads `still no second served listener`; API style's `no network (TCP or UDP) listener` now reads `no served network (TCP or UDP) listener`.
**Why:** the report measures two new test-only binds, both TCP on loopback, where the plan leaned on one, and reads «no TCP or UDP port» as no served one; the session rule of 2026-10-15 owes the registration as test-only.
**Kept:** escher-driver's Unix-domain test bind and the driver session's socket stand as they read. `inherited-defaults.md` holds no API style row and no such wording, so the body's line is the one site.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — `matrix_test`'s free-disk step
**Section:** §Occupied Resources (occupied-resources.md: CI infrastructure) · §Standard Contracts (standard-contracts.md: CI contracts, a new row)
**Change:**
- Was a `jlumbroso/free-disk-space` step also standing in `matrix_test`, skipped on every matrix row, read `completed · skipped` on `Test (android)` in CI#41190771843; now the job holds no such step, `Test (android)` reading none in CI#41333172440.
- `Stand checks repeated`'s step: was this workflow's one measured run of the action; now its one run of the action, pinned by `test_only_stand_repeat_frees_disk`.
- New row: `test_only_stand_repeat_frees_disk` pins that only the `stand-repeat` job frees disk.
**Why:** the report retires the step: `ci.yml`'s `matrix_test` no longer holds it, `Test (android)` reading no such step in the run, and `test_ci_workflows.py` gains the pin.
**Kept:** `Stand checks repeated`'s step stands; the report reads its watch's retirement condition met, 3 of 3 green, and not as a cure of CI#41190771843's red.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the upstream line at `e42ee1f25`
**Section:** §Project Intent (project-intent.md: the `Upstream sync:` line; architecture.md: the `Upstream sync:` line)
**Change:**
- Upstream `main` last read: was `ebacd68bd`; now `e42ee1f25`.
- Taken: was twenty-four of forty; now thirty-six of the fifty-seven commits past the merge base.
- Left out: was sixteen; now twenty-one.
- project-intent.md adds the twelve together (`45c333f16`), each by commit and upstream number, names the left-out `22f0cd000` · `ac6f9f582` · `e5929f9e2` · `0674e8efa` · `784a567e4` and sixteen others, and cites this chunk's `evidence/lock.md` beside the last take's; the body's citation moves to it.
**Why:** the report measures `upstream/main` at `e42ee1f25`, 57 commits past merge base `23b9b26a9`, 36 taken and 21 left out, each with its measured reason. The twelve stand on the founder's answer F2, option A, «Двенадцать коммитов (Рекомендую)», by dialog in the escher overseer session, 2026-11-01, his chosen label relayed verbatim by the operator (inputs#I3); that F2 A names the twelve is the operator's reading of his fork text, not a new word of his (Q1, inputs#I10).
**Kept:** the merge base is unmoved; Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3", no git dependency. Earlier takes' records elsewhere read as history, no change.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the orphan comment gone
**Section:** §Stack and Technologies (stack-and-technologies.md: Layout's Role)
**Change:**
- The clause «the comment at lib.rs:96-97 heading none» goes; the sentence now ends on no `pub use` of it.
- Its citation: was `lib.rs:96-127`; now `lib.rs:115-125`, where `taffy_node_id` and `dom_node_id` stand.
**Why:** the report retires the orphan «Re-export taffy» comment, `blitz-dom/src/lib.rs`'s two lines removed with `pub use clock` beneath unmoved.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the `embolden` step of `fast`
**Section:** §Stack and Technologies (stack-and-technologies.md: the `fast` leg) · §Standard Contracts (standard-contracts.md: CI contracts, the leg list, the `fast` order, the gap row, a new row)
**Change:**
- `fast`: was fmt → clippy → test → ci-scripts; now fmt → clippy → test → ci-scripts → embolden.
- The leg list gains `embolden`, the last step of `fast`.
- Was no local gate building `blitz-paint/font-embolden`, CI's macOS leg alone exercising it, owner a CARRY on «Upstream sync ahead of the UI graph»; now it is built locally by the `embolden` step, the feature flag on `cargo test -p blitz-tests` the local witness, and the owner clause goes.
- New row: `.github/scripts/ci-leg.sh` holds the leg, `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, with no CI job, macOS exercising the feature there, pinned by `test_fast_leg_ends_on_the_font_embolden_witness`.
**Why:** the report retires the gap: the owed thing landed, so its owner row answers to nothing.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — the resolved `width` and `height`
**Section:** §Standard Contracts (standard-contracts.md: Mutation, query and CSSOM, a new row)
**Change:**
- New row: the resolved `width` and `height` of `resolved_style_value` read `node.unrounded_layout()`, so a reader gets sub-pixel values where `final_layout` rounds; an engine change for every document.
**Why:** the report measures upstream's #1339, `ae774d7f4`, one of the twelve, with no test of its own, the workspace its witness.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — `CustomWidget`'s attribute replay
**Section:** §Standard Contracts (standard-contracts.md: Nodes and widgets)
**Change:**
- Adds that `ElementData::set_custom_widget` replays each existing attribute to `Widget::attribute_changed` with no old value, behind `custom-widget`.
**Why:** the report measures upstream's #1322, `a26c6ddeb`, one of the twelve.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — `text_contrast` past a blend
**Section:** §Standard Contracts (standard-contracts.md: the test harness's contrast row; the visual lint's `contrast` reading)
**Change:**
- `Harness::text_contrast`'s `None` list: was ending on an element whose `opacity` is below 1, whose `filter` list is not empty or whose `background-image` holds a layer other than `none`; now also one whose `mix-blend-mode` is other than `normal`, through `paints_more_than_colour`.
- The `contrast` reading reads a node past such an element as not measured, the same reading with no second formula, and no stand row reads it.
**Why:** the report measures `Harness::text_contrast` reading `None` past an element on the walk with a `mix-blend-mode` other than `normal`, and `Session::lint`'s `contrast` reading the same, no second formula; no stand row moves.
**Kept:** architecture.md's own line names the reader and states no `None` condition, so it takes no edit.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/

## 2026-11-01-upstream-sync-ui-graph — a failed load ending the wait
**Section:** §Standard Contracts (standard-contracts.md: the test harness's load counter; the load wait)
**Change:**
- The counter's handler `CountedHandler` forwards `NetHandler::error`, so a failed load reaches the document on a harness or driver session and ends the wait, returning where it would otherwise run to `LOAD_WAIT`.
- `settle_waiting`'s loop: was until the load is answered or `LOAD_WAIT` has passed; now until the load is answered, a failed load answering it, or `LOAD_WAIT` has passed.
**Why:** the report measures `CountedHandler` dropping `NetHandler::error`, so `f6edb9701`'s fix stopped at the wrapper and a failed stylesheet read `NotSettled(Loads)` for good on a harness document and on the driver; the forward closes it, red before and green after, a companion of that commit, and the harness and driver contracts owed the sentence. `settle_waiting` itself is unedited.
**Ref:** .andromeda/runs/2026-11-01T18-14-00-wrap/
