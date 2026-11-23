@history_item n=1 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (a password painted masked; the shot one more reader of bounds) — made the driver's `screenshot` one more reader of the snapshot model's bounds and of nothing else (an element shot by stable id is its bounds rounded outward and cropped to what can be seen), and stated that a shot changes nothing: no focus moves, `has_changes` stays false, the next diff is empty, both layout modes; it also put a password's typed text in no painted frame. This is where `decision/screenshot-changes-nothing`, `contract/driver-screenshot/element-shot` and the screenshot clause of `decision/snapshot-model-single-source` / `decision/readers-of-one-snapshot-model` come from. The canvas shot taken on each CI host must cross only as a shot already does, change nothing, and read no second model; painted frames are a11y surface, so a kept canvas-shot check stays a reader, never an actor.

@history_item n=2 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (how an open list's labels leave; a shot of a row) — ratified that a page's option and group labels leave an open list only as painted pixels in a `screenshot` and the snapshot's `name`, with no log site, and that a shot of a row crops to it and leaves the list open (`a_shot_of_a_row_crops_to_it_and_leaves_the_list_open`). It is the precedent that a shot's pixels are a crossing the founder ratified. The canvas chunk's boundary («a shot's pixels cross only as they already do», «the kept check keeps no pixels») rests on it: no committed table may carry page text as pixels beyond that crossing.

@history_item n=3 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the snapshot reads of an open list; an open list has no motion) — recorded no `expanded` node key (`NODE_FIELDS` then thirteen), which is the source of `absence/snapshot-list-open-state`. It stated the highlighted row as seen only in a screenshot, a limit later retired by 2026-11-20-select-list-and-selection-debts. It also recorded motion on an open list as observed absent, needing no `advance` (`absence/select-list-motion`). A shot comparison across hosts depends on a still frame. No motion and no `advance` on a booted fixture are why one shot reads one state.

@history_item n=4 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — built `Harness::text_contrast` and `stand_contrast`'s tables beside a painted-pixel witness held «within 1 per channel» of the declared colours, both layout modes. It is the plan's one prior per-channel tolerance on painted pixels, and so a precedent for the TOLERANCE branch of this chunk's founder fork, and for keeping a reading as a stated table rather than as kept pixels.

@history_item n=5 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (3. A11y Assertion Harness Contract; 6. Visual Design Verification; 7. Screen Reader Support) — `~ decision/wcag-contrast-thresholds` and the lint rows now read an open list's highlighted row from its painted engine-constant pair rather than from computed styles. `~ contract/snapshot/open-list-highlighted-row` / `~ decision/snapshot-reads-the-tree` add `highlighted` to the tree-read readings, retiring «the row shows only in a screenshot». So the plan has a reader of painted pairs, but none of whole-frame bytes. A canvas frame read across hosts touches the paint path those readers trust. The screenshot is no longer the only witness of a highlight.

@history_item n=6 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (select_popup_painted also runs as embolden; blending paints with no frame test) — `select_popup_painted` also runs in the `embolden` step of the `fast` leg, macOS exercising `blitz-paint/font-embolden` in CI. `mix-blend-mode` / `isolation` paint with no frame test, the order under a blend read and not measured. These are the plan's prior notes on what painted output is exercised on which host: a painted witness already differs by build feature and platform. A cross-host canvas shot must name its build (plain vs emboldening) and leaves the blend-frame absence unclosed.

@history_item n=7 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — made the snapshot model consume the accessibility tree (role, name and focus from `accessibility_tree`, no second role or name mapping), the root of `decision/snapshot-model-single-source` and `decision/readers-of-one-snapshot-model`. Each later reader (text form 2026-10-06-compact-snapshot-serialization, diff 2026-10-07-change-tracking-and-diff, MCP 2026-10-10-mcp-surface, node keys' help 2026-10-12-self-description, parts 2026-10-20-large-screen-in-parts, screenshot 2026-10-12-headless-screenshot) joined under the same rule. The canvas widget painted through `Widget::paint` must reach any shot through the same model, adding no reader of its own.

@history_item n=8 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the workspace takes the Dioxus crates with `default-features = false`, so a crate gets `accessibility` only by naming it (`decision/dioxus-accessibility-feature-by-name`). In a Dioxus document every element node carries its stable id as `author_id`, with no TextRun / document-root / Window node carrying one (`contract/accessibility-tree/author-id`). The tree is built and refreshed through `Document::accessibility_tree`. Its trap is that the windowed stand ships no AccessKit. A canvas fixture booted alone is a Dioxus document under the same author-id contract; an element shot addresses it by that stable id.

@history_item n=9 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on `dioxus-native-dom` unconditionally, with no `[features]` table (`decision/driver-requires-accessibility`), because its executor reads the snapshot. The feature reaches every binary depending on it, both seven_guis binaries included, while neither builds a platform adapter (`accesskit_xplat` / `accesskit_winit` in neither graph). The matrix's `cargo test --all --tests` on macOS and Windows compiles this accessibility code for a canvas check. Any dependency the measurement adds must keep the adapter out of the graph.

@history_item n=10 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the founder ratified the shell's tree refresh on change and the engine's changed-set contract as one item (the source of `decision/a11y-tree-refresh-on-change` and `decision/changed-set-marks`, «Ratified, one item»), leaving the windowed witness owed. A canvas shot taken through the driver must leave that contract as it stands.

@history_item n=11 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — made the engine's change flag truthful: the changed set is marked by a mutation, a focus or checked change and a text input, and not by node creation, hover, active, scroll or layout. The tree is refreshed only when the set is non-empty, and the diff is one more reader of the snapshot model. This is the original form of `decision/changed-set-marks` and `decision/a11y-tree-refresh-on-change`. It is why a shot leaving `has_changes` false and the next diff empty is checkable at all.

@history_item n=12 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle (`Harness::settle`, the driver's `act`) neither reads nor drains the changed set, and a freshly booted harness document holds a non-empty set, so a check that reads `has_changes()` as «this step changed something» drains first (`decision/harness-settle-leaves-changed-set`). Trap for this chunk: a canvas fixture booted alone and then shot reads `has_changes` true from its boot unless drained. Any check that the shot «changes nothing» must drain before the shot.

@history_item n=13 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (2. A11y Strategy) — `~ decision/changed-set-marks`: a scroll that closes a list marks its select, and any other scroll does not. This is the first of three rewordings that give the row its current text.

@history_item n=14 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (2. A11y Strategy) — `~ decision/changed-set-marks`: a single select's highlight moving to another row now marks the changed set. It is the second rewording of the row this chunk's shot must leave untouched.

@history_item n=15 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (2. A11y Strategy; 5. Keyboard Navigation) — `~ decision/changed-set-marks`: a post-layout focus loss marks the set too, «layout alone» not marking it, which is the row's current text. The same wrap's `~ decision/tab-and-shift-tab-move-focus` and `+ decision/focus-fixup-gives-up-unshown-focus` (a focused control that no longer shows gives focus up after the next layout) mean a layout can now move focus and mark the set. A canvas fixture's boot and layout before a shot sit inside this contract.

@history_item n=16 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (2. A11y Strategy: refresh) — `~ decision/a11y-tree-refresh-on-change` and `~ decision/accessibility-tree-through-document-method`: the tree is pushed again in a redraw after style resolve, only when the changed set is non-empty, the set then cleared, and a poll pushes no tree. A headless shot renders without that redraw. The shot's «changes nothing» holds against this redraw-site contract, the latest move of the rows the chunk rests on.

@history_item n=17 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (6. Visual Design Verification; 4. ARIA Patterns & Roles) — `~ contract/accessibility-tree/hidden-and-inert-nodes`, `~ decision/hidden-content-excluded-from-a11y-tree` and `~ decision/hidden-content-excluded`: `visibility: collapse` now excludes a node from the tree and the snapshot as `visibility: hidden` does. All three were ratified by the founder at 2026-11-21-dependency-and-ci-debts. A canvas that is hidden, collapsed or inert has no snapshot node, so an element shot by its id is refused as any id-addressed call is.

@history_item n=18 marker=2026-11-21-dependency-and-ci-debts
**2026-11-21-dependency-and-ci-debts** (ratification of the upstream sync's card operations) — the founder ratified `decision/hidden-content-excluded`, `contract/accessibility-tree/hidden-and-inert-nodes` and `decision/hidden-content-excluded-from-a11y-tree` as landed. Their PROVISIONAL status ended, so the hidden-content rows this chunk is bound by stand ratified.

@history_item n=19 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — set the exception that a select's option stays in the tree and the snapshot under `display: none`, with `hidden`, `inert` and `visibility: hidden` still excluding it. That exception is carried by `contract/accessibility-tree/hidden-and-inert-nodes` and `decision/hidden-content-excluded-from-a11y-tree`. It is the clause the picked hidden-content rows hold beyond the plain exclusion.

@history_item n=20 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled`) — a truthy `inert` takes focusability, hits and the tree away from its subtree, and removing it restores them. This is the source of `decision/inert-subtree-excluded` (guard `inert_subtree`), PROVISIONAL then and ratified at its Epoch 2 close entry. The 28-name falsy clear (2026-10-13-upstream-sync-polish-and-ship) removes a framework-written `inert: false`. A canvas fixture must not carry inertness that would drop it from the snapshot its shot reads.

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the driver on a `multiple` select) — `type` on a `multiple` select focuses it with no click, and the driver's closed sets hold no verb, argument, key name, result field, cause, session error or exit for it. 2026-10-31-select-popup widened this to any select (`Target.select`, `focus_select`), giving `decision/driver-type-focuses-select-no-click`. It is the precedent for the closed sets this chunk's boundary («no new driver verb, argument, carrier, result field, refusal cause, session error, exit») keeps closed.

@history_item n=22 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — created ci.yml's `a11y` job (`ci-leg.sh a11y`) in the slow tier behind the four fast jobs, with no required status check. This is the root of `decision/a11y-ci-gate` and `decision/a11y-test-leg`. A kept canvas-shot check placed by name in this leg would move its list; the scope places it in the matrix's `cargo test --all --tests` and the linux test leg instead.

@history_item n=23 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — widened the a11y leg to six files on the founder's F4 (no new job, no required status check, no repository setting). It re-homed the platform adapter on the stand binary and its windowed AT witness to a residual, the origin of `decision/stand-builds-no-platform-adapter`'s deferral. The leg's membership is set by the founder's word, so a canvas check joins it only by such a word.

@history_item n=24 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — every `stand_*.rs` file also runs three times in the `stand-repeat` leg under the zero-failure budget (the stand-repeat half of `decision/a11y-test-leg`). A kept canvas-shot check named `stand_*`, as `stand_canvas_shot`, `stand_canvas_placement` and `stand_canvas_text` are, runs three rounds there by its prefix. A byte-equal or toleranced reading must be deterministic across rounds as well as across hosts.

@history_item n=25 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg runs fourteen files, set by the founder's word «Плюс проверки дерева», eight of them also in `stand-repeat`. It is the current membership of `decision/a11y-test-leg`, which no later entry changed (14 files, 114 tests at 2026-10-31-select-popup, unmoved at 2026-11-01-upstream-sync-ui-graph). The baseline this chunk must leave unmoved, or move only by the founder's word.

@history_item n=26 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the stand-repeat `--test` list's membership is held jointly by `stand_requirement_sweep`'s leg check and `test_stand_repeat_leg_runs_three_rounds`. The first reads `ci-leg.sh` as one text by `contains` and cannot see a name dropped from one list alone. Trap if the chunk touches the leg lists: a membership change reads red only when both sites lose the name.

@history_item n=27 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (1. A11y Scope Summary) — `~ decision/capability-statement-reach-by-hand`: the capability statement's pin now reads the a11y leg's file count stated in `docs/capabilities.md` `## Accessibility reach`. Any change to the a11y leg's files (see `decision/a11y-test-leg`) must move that published count with it, or the pin reads red.

@history_item n=28 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — named the owner of the windowed AT witness at the platform adapter as the residual re-carried past 0.2.0, an active assistive technology this version does not reach. It published the accessibility reach in `docs/capabilities.md` (tier 0, SC 2.1.1 · 1.4.3 · 2.4.3, AT reading unmeasured). This is the deferral `decision/stand-builds-no-platform-adapter` and `absence/assistive-technology-reach` restate. The canvas widget has no AT reading here either, and none is promised.

@history_item n=29 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the residual owner clauses re-said) — the stand's platform adapter and its windowed AT witness are owned by residual `res-llkgqygv`, re-carried at 0.3.0 (target: next). It is the current owner of the deferral behind `decision/stand-builds-no-platform-adapter` and `absence/assistive-technology-reach`.

@history_item n=30 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (no stand binary builds a platform adapter, on this take) — is the latest of the plan's adapter re-reads, before it 2026-10-24, 2026-10-27, 2026-10-31 and 2026-11-04-dioxus-runtime-probe's: the stand's graph holds neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree --locked -p seven_guis -e normal --prefix none`, `^accesskit_(xplat|winit) ` 0 rows). The plan re-reads it at every manifest or lock change. If the measurement or its kept check moves a manifest or `Cargo.lock`, this probe is owed again.

@history_item n=31 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the plan's text into rows (241 rows) against the code. Every picked row's handle dates from this conversion. Prior prose entries above name sections, not handles, so they are matched to the rows by area.
