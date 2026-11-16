@history_item n=1 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (normal alignment keyword) — mapped align-items/justify-items/align-self/justify-self `normal` to Taffy's `NORMAL` keyword, resolved by the aligned box's layout mode (row decision/normal-self-alignment-taffy). A Taffy bump past #1372 must keep that keyword mapping compiling and meaning the same; re-check it against the new Taffy API.

@history_item n=2 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** (engine's into-view scroll widened) — widened `scroll_into_view` to write every scrolling box on the target's containing-block chain, innermost first and nested ones at once, then the viewport, as one method for every document (founder-ratified). It added `visible_region` and first measured the own-offset `bounds` misreading. Rows decision/scroll-into-view-whole-chain and decision/nested-scroll-written-instantly come from here. The take's seam edit, where `scroll_into_view` reads `scroll_state` as `(current, min, max)` and skips a box whose min and max are both zero, has to keep this whole-chain behaviour.

@history_item n=3 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (Taffy's tree traits on `LayoutPassState`; the bounds reader renamed) — moved Taffy's nine tree traits onto the crate-private `LayoutPassState`, so `BaseDocument` implements none (row contract/layout/taffy-traits). It also brought in `BaseDocument::physical_unrounded_geometry` with two bodies, one per `writing-mode` build, each summing `location − scroll_offset` over the chain. This is the reader where the scope keeps upstream's scroll-shifted second body out. The Taffy bump touches those trait impls, and `layout/writing_mode.rs` takes upstream's fourth `physical_layout` argument.

@history_item n=4 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (`writing-mode` opt-in; stylo_taffy exports) — made `writing-mode` a feature in no default list, forwarded down to stylo_taffy's (row decision/writing-mode-opt-in). It also widened stylo_taffy's exports with `WritingMode` and `WritingModeExt` (row contract/layout/stylo-taffy-exports). The take conflicts in `writing_mode.rs`, which compiles only under that feature, so both builds must be checked. The dropped `stylo_taffy::CopyStyleGetters` import touches that export list.

@history_item n=5 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (second sync: Parley's pin form) — made parley a registry version, leaving taffy the one git dependency pinned by `rev`. It noted that the Taffy rev is spelled in no master, so a Taffy bump amends nothing there. This is the precedent for this take, where the rev line is the only word that moves.

@history_item n=6 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (two engine limits measured) — measured that a click naming a scrolled box lands off its centre by the scroll offset, and that a control where a scrolled-out row extends reads `covered`. Both were left unfixed, pinned by `stand_act_scroll` and `stand_act_obstructed`, with `covered`'s text stating them. This is the history behind the readings the reversed-scroll-origin change must not bring back.

@history_item n=7 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** (a scrolled box reads its own bounds; a hit stops where a box clips) — changed `physical_unrounded_geometry`, in both bodies, to subtract the scroll offset of every box above the node and never the node's own. It made `hit`/`hit_inner` test a node's own area where the box stands and stop at the padding box of an overflow-clipping box. Both are founder-ratified direct engine changes for every document, flagged upstreamable (rows decision/client-rect-own-box, client-rect-ignores-own-scroll, scrolled-box-bounds, hit-stops-at-clip-padding-box). These are exactly the readings the scope keeps against upstream's scroll-shifted `hit_inner` `matches_self` and geometry body.

@history_item n=8 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** (driver texts restated; the pinning checks) — restated `covered` and the helps once the two limits were fixed. It registered the guards `scrolled_box_client_rect`, `hit_clipped_at_scrolling_box` and `scrolled_box_absolute_position`, and `stand_act_scroll`'s `crud-list` reading the same rect at 12 and 14 Creates. These are the checks the take must keep green unchanged.

@history_item n=9 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (taffy from the registry) — moved taffy from a git rev to registry "0.15", leaving no git dependency. It set the trap that CLAUDE.md's coupled-pins warning no longer names a taffy git rev. This is background to the no-git rule whose Taffy exception this take keeps, and to how the coupled-pin guidance reads once Taffy is git again.

@history_item n=10 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (legs and gates) — added the `stand-repeat` leg with a zero-failure budget and no retry, the per-crate coverage floors (escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90), the whole-workspace `cargo deny` audit and upstream's blitz-dom clippy line. Rows decision/stand-repeat-zero-failure-budget and per-crate-coverage-floors come from here. The trial did not run `coverage`, so the floors are an open gate for this take.

@history_item n=11 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (hit own-box, one-axis and transformed-and-scrolled readings) — measured the hit's answer at an `img`, an `iframe` and a text input's own box. It measured that a one-axis clip stops the hit at the whole padding box while `visible_region` narrows one axis, refused `covered` with neither reader moved (row decision/one-axis-clip-readings-kept, guard `stand_act_obstructed`). It also measured that a box both transformed and scrolled is hit where its rows show. Reconciling the hit in `node.rs` must not move these pinned readings.

@history_item n=12 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (every stand task keyed) — first read a scrolled box under a vertical writing mode under both builds. That is the check covering the `writing_mode.rs` body where the take keeps escher's own-scroll rule.

@history_item n=13 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the sync at `23b9b26a9`) — recorded the merge base `23b9b26a9`, with the Taffy-main commits past it waiting so Taffy stayed registry "0.15". This begins the deferral chain that ends in this take.

@history_item n=14 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the geometry readers) — made the hit walk also stop at a `contain: paint` box. It made `Node::absolute_position` subtract only its containing blocks' scroll through `position_in_containing_block`, with `scroll_into_view` passing `(0.0, 0.0)`, and read `get_client_bounding_rect` through transforms. These were ratified at the Epoch 2 close (rows decision/absolute-position-ancestor-scroll-only, absolute-position-containing-blocks, hit-stops-at-paint-clip, client-rect-under-transforms; contract/scroll-hit/absolute-position). The scope keeps escher's `absolute_position` reading, and the take's `scroll_into_view` seam edit sits beside that `(0.0, 0.0)` call.

@history_item n=15 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** (a partial take) — first took an upstream commit alone by a no-commit cherry-pick with the merge base unmoved, and kept the four Taffy-main commits out because they put a git Taffy in the manifest. This is the precedent for the take's no-commit cherry-picks (row decision/upstream-sync-merge-base).

@history_item n=16 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the visual lint and `text_extent`) — added `BaseDocument::text_extent`, read from inline layout, and the lint's clip region built on `visible_region` (row contract/basedocument-readers/text-extent). The take edits `layout/inline/layout.rs` and the scrollable-overflow measure, which these readers rest on.

@history_item n=17 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the dropped-node hit skip; recorded style facts) — made `Node::hit_inner` skip a child that no longer resolves, in its hoisted, paint-children and text-cluster loops (rows decision/hit-and-paint-skip-dropped-nodes, contract/scroll-hit/hit-dropped-node, guard `hit_test_dropped_nodes`). It recorded `Node::out_of_flow` (row decision/node-holds-structure) and moved `layout/inline.rs` to `layout/inline/layout.rs`. Upstream's `ebacd68bd` (hit test elements with no computed styles) lands in the same `hit_inner` and must sit beside this skip.

@history_item n=18 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the upstream take at `f9cbb2756`) — kept the Taffy-main commits waiting for escher's next version, with Taffy staying registry "0.15" and no git dependency. This is part of the deferral history of row decision/deferred-upstream-dependency-bumps.

@history_item n=19 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the scrollbar hazard) — recorded the measured hazard that right after a scroll, a pointerdown in the shown overlay scrollbar's band is a thumb drag that dispatches nothing, so a driver click needs an `advance` past the fade (row decision/click-after-scroll-advances-past-fade). The take edits `node/scrollbar.rs` and the scroll range, which this band and the shared scrollbar geometry rest on.

@history_item n=20 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the merge surfaces read at `ebacd68bd`) — read upstream at `ebacd68bd`, one of this chunk's own picks. It recorded the range model's and keyboard activation's merge surfaces there, including the `mutator.rs` `set_attribute` conflict resolution. This is the prior reading of the same upstream point for `mutator.rs` and `node/element.rs`.

@history_item n=21 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (six upstream behaviour changes ratified for this take) — set the pattern that upstream behaviour changes for every document are ratified per take and not counted among escher's direct engine changes (row decision/upstream-behaviour-changes-ratified-per-take). The reversed-scroll-origin change is recorded under this pattern on the founder's ratification for this take.

@history_item n=22 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`::before`/`::after` in flow classification) — added the upstream flow-classification vote, so a pseudo-only inline-block exports its line baseline (row decision/pseudo-elements-vote-in-flow-classification). This is a layout rule the Taffy bumps, including the collapse-through margins and float segment changes, must leave green.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the eight engine changes of this chunk) — recorded that upstream `0980cce5f` has no hunk on `visible_region`, while its `hit_inner` hunk is a textual conflict with escher's non-invertible-transform answer. The merge surface went on a line for the next upstream sync. This is the known conflict this take's `node.rs` resolution must keep (rows decision/hit-focus-form-engine-changes, non-invertible-transform-takes-no-hit).

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through transforms) — made `visible_region` read each clipping box through its transforms via `padding_box_in_view`, with `transformed_bounds` `pub(crate)` (row decision/visible-region-through-transforms). `visible_region` is the read-only seeing reader the take must keep consistent with the reconciled hit and bounds readings.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a non-invertible transform takes no hit) — made `Node::hit_inner` answer `None` for the whole subtree under a transform whose determinant is 0 or not finite (rows contract/scroll-hit/hit-degenerate-transform, decision/non-invertible-transform-takes-no-hit). That is the `hit_inner` code `0980cce5f` conflicts with.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's exceptions to scrolling and the hit) — made `scroll_into_view` write only an open list row's `scroll_y` and return. It made `hit_with_scrollbar` answer the list first with `hit_inner` holding none of it, and made `visible_region` answer a row's list scrollport (rows decision/select-row-scroll-into-view, open-select-list-hit-first, contract/scroll-hit/hit-with-scrollbar, contract/scroll-hit/scroll-into-view). The take's `scroll_into_view` and `hit_inner` edits must keep these exceptions.

@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's state and placement in the engine) — made `resolve` call `reconcile_select_list` after layout. It added `select_popup`'s test `a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes`, the test where upstream's overflow read panicked on a node with no computed styles. That panic is why the take adds the private `is_scroll_container` seam (rows decision/resolve-reconciles-select-list, select-list-closes-without-event).

@history_item n=28 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (taffy is no re-export of blitz-dom) — corrected the row to say taffy is part of blitz-dom's public API through its signatures, with no `pub use` (row decision/taffy-layout). A Taffy bump therefore changes blitz-dom's public signatures.

@history_item n=29 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the merge surfaces' reading and owner at `e42ee1f25`) — recorded `0980cce5f` as left out at `e42ee1f25`, with `hit_inner`'s conflict against the non-invertible answer carried to the next sync. It moved the merge-surface owner to «Upstream sync ahead of determinism and exploration». This is the prior deferral of `0980cce5f` that this take finally lands.

@history_item n=30 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic's fix) — made `deallocate_anonymous_block` clear its children's stale `layout_parent`. It recorded that upstream's own tree panicked in `node/node.rs`, with the trap that a panic probe must assert its fixture first (rows decision/anonymous-block-dealloc-detaches, anonymous-block-drop-clears-layout-parent). The take edits `node/node.rs` beside this fix.

@history_item n=31 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the Stylo prefs behind a `Once`) — added `init_stylo_prefs` behind a `Once`, written with Stylo 0.22 pref names rather than upstream's 0.23 names (row decision/stylo-prefs-set-once). It explains why the take's `document.rs` conflict resolves to escher's side, with no `sideways` pref in stylo 0.22.

@history_item n=32 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed stylesheet through `NetHandler::error`) — set the trap that a trait default method can silently hide a take's fix, because it compiles through a wrapper. A take that extends a trait must sweep its implementers. This applies to the take's Taffy trait changes, such as `cache_get` taking `&self`.

@history_item n=33 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the twelve commits ratified for their take) — ratified upstream behaviour changes per take, by commit, under the playbook pattern, kept apart from escher's own direct engine changes. This is the form the take's seven picks are recorded in.

@history_item n=34 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Conventions) — gave decision/both-layout-modes its exception: upstream-taken blitz-tests targets are kept as taken even when they run one layout mode. This governs the take's upstream test `inline_box_scrollable_overflow.rs`.

@history_item n=35 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Established Decisions) — restated decision/upstream-take-engine-changes for the 29 picks read at `563d3db73`. This is the row a later take's reading supersedes in place.

@history_item n=36 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** (Established Decisions: no-git-dependencies) — added the Taffy-only exception: the taffy crate alone may take a git source and must still pass the lock, audit and MSRV legs. The take keeps that exception as it stands, with only the rev moving.

@history_item n=37 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** (Project Intent: deferred bumps) — had the Taffy-main content arrive through registry Taffy 0.15.x save #1303, with the later git Taffy chain waiting for its own chunk. This is part of the deferral history of the row this take resolves.

@history_item n=38 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** (Stack and Technologies: taffy-layout) — changed decision/taffy-layout to Taffy 0.15 from upstream's git source, pinned to one revision. The rev now moves from `e90751fd` to `a3997d55`.

@history_item n=39 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** (Established Decisions: no-git-dependencies) — reworded decision/no-git-dependencies to say every dependency is a registry version, with the Taffy except standing. The take keeps it unchanged.

@history_item n=40 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** (Project Intent: deferred bumps) — set decision/deferred-upstream-dependency-bumps to Taffy git pinned before #1372. It deferred Taffy from #1372 on, which left blitz-dom uncompiled, to its own take where the reversed-scroll-origin change is asked. This chunk is that take, and the row is owed a rewrite once it lands.

@history_item n=41 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** (the git-Taffy take's layout changes ratified) — ratified eight upstream commits taken with git Taffy `e90751fd` for that take alone. It kept `0980cce5f` and `ebacd68bd` out, their reversed-scroll-origin change unratified, and stated that "the next Taffy take asks its own". This is the direct precedent and the constraint this take's own ratification answers.

@history_item n=42 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (Standard Contracts) — added `BaseDocument::layout_reason` with the closed `ShiftBy` (Scroll, Transform) and the `PlacedBy` and `SizeCause` sets (rows contract/base-document/layout-reason, contract/layout-reason/*). A scroll range that can now run negative moves the scroll shifts this reader reports, so its readings must be re-checked.
