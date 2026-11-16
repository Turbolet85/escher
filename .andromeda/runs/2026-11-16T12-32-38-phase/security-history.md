@history_item n=1 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** (Dependency Security: 4 rows) — ~ decision/engine-dependency-pins put Taffy at the git pin `e90751fd`; ~ decision/no-git-dependencies, ~ decision/no-git-dependency (Taffy alone takes a git source, pinned at a full commit rev, no patch section, no lock checksum, still passing lock, audit and MSRV legs) and ~ decision/upstream-sync-dependency-set (lock-only move of Taffy's own git pin allowed). This chunk moves that rev to `a3997d55`: the engine-dependency-pins row's rev is the one word that changes at wrap, and the exception's shape (full rev, no patch, no checksum, legs green) must hold as written.

@history_item n=2 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** (Dependency Security: 3 rows) — gave decision/no-git-dependency and decision/no-git-dependencies their Taffy-only exception and ~ decision/upstream-sync-dependency-set the git-pinned-content clause, on the operator's «Apply all three». The ground this chunk's Taffy bump stands on: no other crate may take a git source, and the lock is read name by name with the audit leg green after the take.

@history_item n=3 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the 28-name falsy predicate; no git dependency; the sync's dependency delta) — moved Taffy from git rev `7d33901c` to the registry 0.15 and set the founder's rule of no git dependency in the root manifest. The origin of the rule whose Taffy exception this chunk keeps; precedent that a Taffy source move is recorded as a per-sync lock record with its features byte for byte.

@history_item n=4 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** (the 2026-10-18 sync record) — upstream's Taffy-main commits (git Taffy `ea07c4c3`) were not taken, under the founder's 2026-10-13 rule of no git dependency. History of why Taffy-main content was held back until the exception; this chunk's five Taffy bumps are the later form of that held-back content.

@history_item n=5 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** (the per-sync lock record of the second partial take) — again left out upstream's four Taffy-main commits, recording the lock and `deny.toml` byte-identical and the audit green. Precedent for the per-sync lock record this chunk owes.

@history_item n=6 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the per-sync lock record of the ten-commit take, and tokio past RUSTSEC-2026-0386) — six Taffy-main commits not taken, the pins stated; a lock-only semver-compatible update recorded as the audit rule's remedy. Precedent for a cherry-pick take whose lock moves only edges, the shape decision/upstream-sync-dependency-set now names.

@history_item n=7 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the per-sync lock record of the eleven-commit take) — the sync read at `ebacd68bd` took eleven commits, the pins stated (Stylo 0.22.0, Taffy registry 0.15, Dioxus 0.7.3, winit beta.3). This chunk's pick `ebacd68bd` (hit test of nodes with no computed styles) was that sync's read head, not among its takes; the record form to repeat.

@history_item n=8 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`multi_doc`'s inputs, the attribute runner's unwrap, the traversal lock and the moved `inline` path) — registered the WPT attribute test runner's unwrap (`wpt/runner/src/test_runners/attr_test.rs`) under Error Handling and moved the inline-box height guard and the layout `unreachable!()` cites to `layout/inline/layout.rs`. This chunk touches both files: decision/inline-box-heights-finite must still hold, and the runner's panic-path row and citations follow the picks.

@history_item n=9 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (this take's lock record) — twelve commits taken, manifests, lock and `deny.toml` byte-identical, pins stated, audit green. The latest record of the form; also its «a trait's do-nothing default can hide a fix through a wrapper» trap (the failed-load entry of the same chunk) applies to any trait signature a Taffy bump changes (e.g. `cache_get` taking `&self`): sweep the implementers.

@history_item n=10 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the key-press panic after a hidden ancestor) — recorded a panic in `position_in_containing_block` (`node.rs`) through `handle_dom_event`'s `absolute_position`, reachable by a remote page. This chunk reconciles `Node::absolute_position` with upstream's scroll-origin change; the history of that reading's panic path is its trap.

@history_item n=11 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic retired) — `deallocate_anonymous_block` clears each child's `layout_parent`, curing every reader of a stale layout parent (+ decision/dropped-anonymous-block-clears-parent, guard `key_press_after_container_lost_its_box`). The `absolute_position` and `node.rs` edits of this take must keep that cure and its guard green.

@history_item n=12 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** (three of the driver's fixed texts restated) — escher fixed in the engine the scrolled-box bounds limit and the hit reaching content scrolled out of a scrolling box, and dropped those limits from `covered`'s and the help texts. These are the ratified readings (own-scroll bounds, hit stop at the padding box) this chunk keeps against upstream's scroll-shifted readings; were they lost, the fixed texts would read false again.

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through transforms, and `off-screen`'s meaning) — decision/visible-region-through-transforms: each clipping box read through its transforms by `padding_box_in_view`, callers `in_view`, `crop` and the lint's `clip_region`; `off-screen`'s meaning restated. A change to scrolled-box bounds or scroll origins moves what `visible_region` and `off-screen` read; guards `visible_region_transformed`, `stand_act_transformed`, `transformed_client_rect`.

@history_item n=14 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a non-invertible transform takes no hit) — `Node::hit_inner` answers none for the subtree of a non-invertible transform (decision/non-invertible-transform-no-hit, guard `hit_non_invertible_transform`). This take rewrites `hit_inner`'s `matches_self` content test; the early return must survive the conflict resolution.

@history_item n=15 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§Input Validation: the `inert` read and the toggle per role) — `inert` takes no hit at `hit_inner` (decision/inert-subtree). Another gate inside the `hit_inner` this take edits, to be kept.

@history_item n=16 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (`disabled` on form controls, the password caret by the bullets; refusal texts) — the password caret placed by bullets laid out through `layout_as_text_input`, «scrolled as paint scrolls it» (decision/password-caret-placed-by-bullets); `off-screen` gained the transformed-scrolling-box sentence. Scroll-origin changes reach the caret's scroll and the `off-screen` reading.

@history_item n=17 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (the password click caret a known 0.1.0 limit; `covered`'s second limit) — `covered`'s measured hit limits (a one-axis clip) stated in fixed text. The hit limits are the readings this take may move; contract/refusal-causes/covered must still read true.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`covered`'s closed-option clause and the select verbs' help) — `covered`'s meaning gained the closed-select option clause (contract/refusal-causes/covered). Fixed text that must stay true over this take's hit changes.

@history_item n=19 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** (six verbs; the detected causes after `validate`) — fixed the refusal order disabled, off-screen, covered, with `scroll` saying `in_view` (decision/target-refusal-order). Off-screen and covered read the scroll and hit readings this chunk reconciles.

@history_item n=20 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the shared target check and the actions read) — one `check_target` (`action_point`, `in_view`) shared by the id-taking verbs and `Session::actions` (decision/actions-read-shares-target-check). Any moved hit or bounds reading moves both the refusals and the actions answer together.

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's visible region, paint and scroll) — `scroll_into_view` on an open list's row writes the list's `scroll_y` at once and moves no chain or viewport; wheel over/outside the list; `visible_region` answers the list's scrollport (decision/select-list-scroll-own-offset, decision/select-list-wheel, decision/visible-region-open-list-row). This chunk edits escher's `scroll_into_view` (the `scroll_state` triple, skipping a box whose min and max are zero); the list arm must stay as ratified.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`size` on a `multiple` select) — `apply_row_gesture` calls `scroll_into_view` on the current row where `carries_valid_size` (decision/multiple-select-size-rows). Another `scroll_into_view` caller the triple-reading seam edit reaches.

@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list's readers and its close add no panic) — public select readers call no `layout_style()`, a row with no styles reading none; a select made inert, disabled, hidden or removed with its list open closes with no panic (decision/select-row-layout-none-without-styles, decision/select-list-closes-when-select-unfit). The take's measured seam fix (`is_scroll_container` reading false for a node with no computed styles) exists because upstream's overflow read panicked in exactly that test; the no-panic rows bind it.

@history_item n=24 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (which select opens a list, what a row takes, and hostile markup) — decision/select-list-open-gate and decision/select-list-hostile-markup-finite (every box finite, 20 rows max). Scroll-range changes that may run negative under `ScrollableOverflow` must keep the list's boxes finite.

@history_item n=25 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's state, keys, pointer and close) — a page scroll or a box move closes the list through `reconcile_select_list`, the `fresh` field keeping a just-opened list (decision/select-list-closing, decision/select-list-pointer-release-chooses). A changed scroll origin or box position reading can trip this close.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what else of the list crosses the carriers) — the list crosses only as a row's own `bounds` in snapshot and diff and as pixels (decision/select-list-crossing). The chunk's boundary «driver answers change only where engine readings change» rests on such bounds readings staying as measured.

@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a shot of an open list holds labels as pixels) — decision/open-list-shot-pixels, a hosted shot pinned equal to an in-process one by `cli_commands`. A ratified widening; the take must add no new crossing beside it.

@history_item n=28 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (eight causes, `advance` on every session, the fade on the document's clock) — the overlay scrollbar fade reads the document's clock, time reads in `scrolling.rs` and `events/pointer.rs` go through the `Clock` with no epoch `unwrap` (decision/scrollbar-fade-on-document-clock, decision/advance-moves-virtual-clock-exactly). This take touches `node/scrollbar.rs`, `scrolling.rs` and `events/pointer.rs`; no wall-clock read or epoch unwrap may re-enter from upstream.

@history_item n=29 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** (the range input row) — a main-button pointerdown sets the value from the hit's fraction of the content box (decision/range-pointer-actions). `events/pointer.rs` and the hit/content-box readings this take edits feed it.

@history_item n=30 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (unknown alignment flags; merge citation re-point) — an unknown alignment flag maps to Taffy's `AlignContent::NORMAL` rather than panicking. A Taffy-API-dependent no-panic claim a Taffy bump can move.

@history_item n=31 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (root-manifest citations re-pointed after the line-61 insert) — the taffy rev pin citations in root `Cargo.toml` once read stale after an insert. This chunk edits that taffy rev line; its citations must be re-read.

@history_item n=32 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (dependency security after the sync) — first statement that the one git dependency, taffy, is pinned by commit rev, and that a sync's entering crates are reviewed by hand. Precedent for the rev-pinned git Taffy this chunk keeps.

@history_item n=33 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** (cargo-deny audit leg, SHA-pinned ci.yml) — cargo-deny 0.20.2, the root `deny.toml`, per-ID ignores only with a written reason (decision/cargo-deny-audit-tool, decision/audit-ignores-per-id-with-reason, decision/dependency-audit-ci-gate). The audit leg the git Taffy must still pass.

@history_item n=34 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (signing and PR-token jobs repository-guarded; CI legs locked) — `--locked` on every CI cargo leg (decision/locked-builds). The rev move in `Cargo.toml` and `Cargo.lock` must agree so locked legs pass.

@history_item n=35 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (the whole-workspace audit, the licence gate) — the audit over the whole workspace with all features and the licence gate on every push (decision/dependency-audit-cargo-deny, decision/licence-gate). Binds the git-sourced Taffy at its new rev.

@history_item n=36 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (advisories published after a green audit) — a CI audit can go red on new advisories over an unchanged lock; run `bash .github/scripts/ci-leg.sh audit` locally before a push (decision/local-audit-before-push). The trial measured the audit green earlier; re-run before the push.

@history_item n=37 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the zip ignores gain RUSTSEC-2026-0429) — nine per-ID ignores stand, each with a written reason (decision/advisory-ignores-by-id). A new advisory on the moved graph is answered by a semver-compatible update or a per-ID ignore, never a blanket allow.

@history_item n=38 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (each kept answer whole at rest, the committed checks, the replay) — the two committed stand-Counter checks are compared byte for byte with a fresh recording (decision/committed-counter-checks, decision/kept-check-answer-whole). Any reading the take moves on the Counter screen breaks them; the measurement reads them passing unchanged.

@history_item n=39 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (culling on the scaled size, the inline box's paint skip) — `render_element`'s culling on the scaled size and `IS_INLINE_WITH_ONLY_BLOCK_CHILDREN` skipping its own paint. Engine paint/layout readings near the `layout/inline/layout.rs` and `layout/mod.rs` code this take edits.
