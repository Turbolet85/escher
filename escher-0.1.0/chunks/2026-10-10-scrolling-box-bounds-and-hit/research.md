# Codebase Research — 2026-10-10-scrolling-box-bounds-and-hit

## Scope
- **Depth:** deep · **Reads:** 27 · **Globs/Greps:** 24
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole, its one Session Addition included (an entry whose own exit is 124 never reads green; no entry of this chunk is bounded that way) · `.claude/rules/testing.md` — its seven Session Additions read; the one of 2026-10-10 applies (search `tests/` and `examples/*/tests/` for every check that pins the changed reading before calling it unaffected — done, §Sweeps)
- **Platform issues consulted:** none — no runner-only bullet stands in scope (the one `not green` read at Setup is a run cancelled by the workflow's own rule), and no entry of this chunk reads CI outside the operator leg
- **External inputs:** `inputs#I1` — the operator's directive given with the take-up: technical forks are the operator's, asked in dialog; a fork that changes what an agent meets beyond restating the three limit sentences, or that widens a boundary, is the founder's and is put whole in one file

## Files inspected
- `packages/blitz-dom/src/document.rs` (1556-1612, 2229-2390) — the non-`writing-mode` body of `physical_unrounded_geometry` (`:2257-2270`); `get_client_bounding_rect` (`:2273-2290`); `offset_rect` (`:2300-2351`); `node_client_rects` and `inline_fragment_rects` (`:2357-2389`); the document-level `hit` and `element_from_point` (`:1570-1607`)
- `packages/blitz-dom/src/layout/writing_mode.rs` (340-420) — the `writing-mode` body of `physical_unrounded_geometry` (`:375-401`), which subtracts the node's own scroll offset on two paths: the containing-block walk (`:385-386`) and the vertical-chain path (`:398-399`)
- `packages/blitz-dom/src/node/node.rs` (1326-1567, 715-735, 1623-1634, 1862-1880) — `hit` and `hit_inner` (`:1342-1543`); `absolute_position` (`:1626-1634`) and its callers in the file
- `packages/blitz-dom/src/scrolling.rs` (690-807) — the tail of `scroll_into_view` (`:716-738`) and `visible_region` (`:749-807`)
- `packages/blitz-dom/src/events/driver.rs` (313-345), `packages/blitz-dom/src/events/mod.rs` (93-112), `packages/dioxus-native-dom/src/events.rs` (199-232) — what the event paths do with a client rect and with an absolute position
- `packages/blitz-dom/src/tree.rs` (53-66) — the viewport's scroll is a field of its own, not the root element's scroll offset
- `packages/blitz-dom/src/accessibility.rs` (grep) — no geometry: the file holds no bound, rect, layout or position reader
- `packages/blitz-paint/src/render.rs` (336-470) — paint's clip rule (`:349-358`) and the box it clips children to (`:442-451`)
- `packages/escher-driver/src/execute.rs` (238-349) — `locate`, `action_point`, `in_view`, `covered`
- `packages/escher-driver/src/schema.rs`, `refusal.rs`, `json.rs`, `lib.rs`, `error.rs` (greps and ranges) — the three sentences, their copies and the texts before them
- `packages/blitz-test-harness/src/inspect.rs` (50-70, 96-105, 134-150), `input.rs` (237-245) — the harness's `hit`, `layout_rect_of`, `scroll_into_view`
- `tests/blitz-tests/tests/stand_act_scroll.rs` (whole), `stand_act_obstructed.rs` (340-532), `stand_act_spans.rs` (528-627), `stand_act_ids.rs` (46-77) — the pinned readings and the tables around them
- `examples/seven_guis/src/app.rs`, `tasks/crud.rs`, `tasks/flight_booker.rs`, `tasks/timer.rs` (the `overflow` rules) — which stand boxes clip
- `packages/blitz-dom/Cargo.toml` (13-38), root `Cargo.toml` (1-29), `.github/workflows/ci.yml` (11-13)
- `escher-0.1.0/chunks/2026-10-10-driver-cli/evidence/engine-findings.md` (whole) — the measured readings of both defects

## What research established

### The bounds defect
- **The frame that produces the shift.** Both bodies of `BaseDocument::physical_unrounded_geometry` walk from the node up its containing-block chain and add `location − scroll_offset` for every box on the walk, **the node itself included** (`document.rs:2263-2267`; `writing_mode.rs:383-388`, and again at `:398-399`). A box's own scroll offset moves its content, not the box, so that one term is the defect. `get_client_bounding_rect` returns the result less the viewport's scroll (`document.rs:2281-2289`), and the snapshot's `bounds` is that rect (`packages/dioxus-native-dom/src/snapshot.rs:127`).
- **The equality the fix needs, on the readings already measured** (evidence file, both layout modes): `crud-list` after 12 Creates and a `scroll` naming the last row reads `(24, 94.796875, 393, 514)` where the box stands at `(24, 112.796875, 393, 532)` — the difference is the list's own scroll offset, 18; at 14 Creates it is 76; the fixture's `fx-box` reads `(8, -269, 792, -169)` at scroll offset 300 where it stands at `(8, 31, 792, 131)`. With the node's own term left out of the sum, each reads the box's standing figures. Still true at HEAD: `cargo test -p blitz-tests --locked --test stand_act_scroll --test stand_act_obstructed` — 6 passed and 5 passed, exit 0, run at this step (the per-package build, so the `document.rs` body); the source tree is byte-identical to `177d1652`'s (`git diff --quiet 177d1652 HEAD -- packages tests examples wpt apps Cargo.toml Cargo.lock`, exit 0), whose workspace run is green (CI#38047227075 — the `writing_mode.rs` body).
- **Two bodies, two runners — re-derived** (`cargo tree -e features -i blitz-dom --locked`, counting `feature "writing-mode"` lines): `-p blitz-tests` 0 · `-p seven_guis` 0 · `--workspace` 1. `wpt/runner` names the feature (`wpt/runner/Cargo.toml:15`) and is a workspace member, so a workspace build compiles the `writing_mode.rs` body and a per-package build the `document.rs` body. The same check file runs over a different reader under each runner; a fix in one body alone reads green under one runner and red under the other.
- **Three callers lean on the shifted reading** — they take a box's geometry as the origin of that box's *content*, where the box's own scroll offset belongs:
  - `visible_region` adds the offset back: "A box's own scroll offset moves its content, not the box" (`scrolling.rs:774-780`). It is escher's own code and the one place that already compensates.
  - `inline_fragment_rects` places an inline element's fragments from its inline root's geometry (`document.rs:2379-2381`): fragments move with the root's own scroll.
  - `offset_rect` subtracts the offset parent's geometry from a fragment's viewport rect (`document.rs:2338-2343`): the parent's own scroll offset is in both terms and cancels.
  - Two more callers read only the layout half and are indifferent: `resolved_style.rs:509`, `:521`.
- **So the fix has two honest sites** — decision 1, asked at P4: in the shared reader (both bodies), with the three content-origin callers taking the offset themselves; or in `get_client_bounding_rect` alone, which adds the node's own offset back and leaves the reader and its callers as they are.
- **Who reads a client rect** (code-graph, `calls`, production callers): the snapshot (`snapshot.rs:127`), a script's `getBoundingClientRect` (`packages/blitz-vibey-script/src/dom/element.rs:1333`) and `getClientRects` through `node_client_rects` (`element.rs:1352`), a Dioxus element's `get_client_rect` (`packages/dioxus-native-dom/src/events.rs:212`), and the pointer event's element-relative coordinates — client point less the target's rect (`packages/blitz-dom/src/events/driver.rs:328-331`). For a target that is itself a scrolled box each of these reads shifted today and true after the fix. An inline element takes the fragment path and is not touched by a fix in `get_client_bounding_rect`.
- **The root.** The viewport's scroll is its own field (`tree.rs:55-64`), subtracted separately; a nested fix decides nothing for the root unless the root element carries a scroll offset of its own. Not measured: whether it ever does.
- **The accessibility tree reads no geometry** (`accessibility.rs` holds no bounds reader — grep `bound|layout|rect|position`: one hit, a label lookup). The bounds fix moves nothing there.

### The hit defect
- **The frame that produces it.** `hit_inner` moves the point into the node's scrolled content space first (`node.rs:1376-1377`: `x − location.x + scroll_offset.x`) and then tests the node's own area as `0 ..= size + scroll_offset` (`:1386-1389`). For a box scrolled by `s` that range starts `s` above the box: a point up to `s` above a scrolled box matches the box itself. The walk then goes on into the node's paint children whenever the point lies in the node's own area, its scrollable overflow or its overflow rect (`:1391-1418`, `:1458-1485`) — with no test of whether the node clips.
- **The equality the fix needs**, on the fixture already measured: the button before the box reads `8, 8, 73, 31`, the box stands at `31 to 131` scrolled by 300, row 7 reads `11 to 51`; the hit at the button's centre answers `fx-row-7` today and must answer the button. Local to the box that point is `y = 19.5 − 31 + 300 = 288.5`, inside `0 ..= 100 + 300`, so both the own-area test and the descent admit it.
- **Why the button after the box is hit as itself today:** paint children are walked last to first (`node.rs:1458`), so a later sibling is tested before the box's overflowing rows. That reading is right by order, not by a clip.
- **Paint already holds the rule the walk lacks:** a node clips its children when it is not the root element and either overflow axis is not `visible` — and also for an image, a sub-document, a text input and `contain: paint` (`render.rs:349-358`); the clip is the padding box, the content box for a text input (`:442-451`). `visible_region` uses the overflow half of the same rule, per axis (`scrolling.rs:763-773`). Which of these the hit stops at is decision 2.
- **Who hits** (code-graph, production callers): pointer move and pointer down (`packages/blitz-dom/src/events/pointer.rs:299`, `:413`, `:423`), hover (`document.rs:1846`), text selection (`document.rs:2486`), a script's `elementFromPoint` and `elementsFromPoint` (`packages/blitz-vibey-script/src/dom/document.rs:330`, `:347`), the harness's `hit` (`inspect.rs:96`) and through it the driver's `covered` (`execute.rs:338`). Overlay scrollbar thumbs are resolved in the same descent, inside the node's own area (`node.rs:1422-1429`).
- **Hoisted children** (positive and negative z-index) are hit from their stacking context, not from their scrolling ancestor (`node.rs:1440-1455`, `:1487-1502`); a clip in the child loop does not reach them. What a clipping box that is itself a stacking context does with its hoisted children is the plan's to state and the implementation's to measure.

### The driver
- **No logic change is needed in escher-driver.** A target's centre is the centre of the snapshot's `bounds` (`execute.rs:270-278`); `off-screen` is read from `visible_region` (`:324-332`); `covered` from the raw hit at the page point (`:336-349`). Both fixes reach it through the engine.
- **`off-screen` does not move with the bounds fix, by construction:** `visible_region` narrows by the padding box of each clipping box on the chain, and under either site of decision 1 that box's origin is the same figure — today by the add-back, after by the reader. The check that holds it is `scroll_into_view_nested`'s `reader_holds_the_target` (`tests/blitz-tests/tests/scroll_into_view_nested.rs:117-120`).
- **The three sentences and every copy of them:** `Cause::Covered`'s meaning (`refusal.rs:71-75`; copies `refusal.rs:254`, `json.rs:574`), the help of `changed` (`schema.rs:198-202`; copy `:435`), the help of `snapshot`'s `text` (`schema.rs:229-232`; copy `:427`). The crate's own docs state both limits (`lib.rs:56`), and a private doc comment gives the hit defect as its reason (`execute.rs:323`). Before the limits were added the three read (`git show e144b44d^`): "the elements on the screen before and after the step whose role, name, state, bounds or parent differ, as they read after it" · "the screen as one text: one line per element, nested by indent" · "another element is hit at the point the action would land".
- **Which of the three reaches an answer line:** `covered`'s meaning, in every `covered` refusal's JSON (`json.rs`, `Refusal::to_json`). The two help texts are in the table and are served by nothing yet.

### A second reader with the same shift — not on this entry
`Node::absolute_position` sums `location − scroll_offset` over the same chain, the node's own offset included (`node.rs:1626-1634`). The snapshot does not read it. Its callers (code-graph, `calls`): `scroll_into_view`, which hands the node's own offset in to cancel it (`scrolling.rs:719-720`); the synthetic click's point — the centre of the box (`node.rs:1872-1874`, reached from `events/pointer.rs:730` and `dioxus-native-dom/src/events.rs:652`); the IME cursor area of a focused text input (`node.rs:727-732`); the event path's position of its target (`events/mod.rs:103`); the harness's `layout_rect_of` (`inspect.rs:57-68`) and its debug dump (`:142`); and one test (`tests/blitz-tests/tests/stale_dirty_descendants.rs:81`, `:90`, `:100`). Not measured: what each reads for a scrolled box. It is a finding, asked at P4 and never absorbed.

### The stand
- **Boxes that clip:** `#task-body` (`overflow: auto`, `app.rs:333-336`) — the box every task's screen sits in; `.list` (`overflow-y: auto`, `crud.rs:174-183`); `.flight-type-row` and `.progress-track` (`overflow: hidden`, `flight_booker.rs:148-155`, `timer.rs:129-135`); `#home` (`overflow-y: auto`, `app.rs:193-200`, not a lean task).
- **Not measured:** whether any stand control lies outside the padding box of a clipping ancestor. A control that did would already be refused `off-screen` by the driver, which reads `visible_region` before the hit.

## Graph impact (from the code-graph query; rust plane, built — the index resolves the reader to its `writing_mode.rs` body)
- **physical_unrounded_geometry** — 6 callers: `get_client_bounding_rect` @ `document.rs:2281`, `offset_rect` @ `document.rs:2339`, `inline_fragment_rects` @ `document.rs:2379`, `resolved_style_value` @ `resolved_style.rs:509` and `:521`, `visible_region` @ `scrolling.rs:776` — a fix in the reader is a change for all six; three need the offset re-applied, two are indifferent, one is the subject.
- **get_client_bounding_rect** — 5 production callers (`node_client_rects` @ `document.rs:2360`, `adjust_element_coords` @ `events/driver.rs:328`, `get_bounding_client_rect` @ `blitz-vibey-script/src/dom/element.rs:1333`, `get_client_rect` @ `dioxus-native-dom/src/events.rs:212`, the snapshot's `visit` @ `snapshot.rs:127`) and 7 call sites in four test files (`inline_fragment_rects.rs`, `scroll_into_view_nested.rs`, `stand_snapshot.rs`, `text_selection_anonymous_block.rs`).
- **hit_inner** — 2 entry callers (`hit_with_scrollbar` @ `document.rs:1833`, `Node::hit` @ `node.rs:1343`) and its three recursive sites; **BaseDocument::hit / hit_with_scrollbar** — 7 production call sites (above). No signature changes: neither fix adds a parameter.
- **visible_region** — 1 production caller (`in_view` @ `execute.rs:328`) and 1 test (`scroll_into_view_nested.rs:120`).
- **absolute_position** — 6 production callers and 3 test sites (above); not modified by this plan unless decision 4 says so.

## Sweeps
- `scrolled out of a scrolling box|shifted by its (own )?scroll offset|read(s)? (its bounds )?shifted` over `packages`, `examples`, `tests`, `scripts`, `README.md`: 6 files · 6 changed (`refusal.rs`, `schema.rs`, `json.rs`, `lib.rs`, `stand_act_scroll.rs`, `stand_act_obstructed.rs`) · 0 no-change. The same pattern over `CLAUDE.md`, `.claude/docs`, `.claude/rules`: 3 files (`CLAUDE.md`, `.claude/docs/services/escher-driver.md`, `.claude/docs/services/dioxus-native-dom.md`) — leaves the wrap re-derives, never this chunk.
- `scroll\(|scroll_into_view|scroll_offset|wheel_at|scroll_to\(|scroll_by\(` over `tests/blitz-tests/tests`, `examples/seven_guis/tests`, `packages/*/tests`: 11 files · 2 changed (`stand_act_scroll.rs`, `stand_act_obstructed.rs`) · 9 no-change (`session_common/mod.rs` — the builder; `stand_act_spans.rs` — its scroll step compares the span's counts with the returned diff's lengths, `:352-354`, and pins no count of its own; `scroll_into_view_nested.rs`, `scrollbars.rs`, `scrollbar_drag.rs`, `fragment_navigation.rs`, `touch_action.rs`, `paint_tree_incremental.rs`, `harness_smoke.rs` — engine checks the gate re-runs, each expected green). `examples/seven_guis/tests/cli_commands.rs:270-281` scrolls the CRUD list and asserts `in_view` and the click after it, no `changed` list.
- `stand_act_scroll.rs` holds **three** places that state the shifted reading, not one: the pinning test the freight names (`:358-445`), a second pinning test (`a_click_naming_a_scrolled_box_is_refused_off_screen_while_the_box_is_in_view`, `:447-508`), and the first test's comment that the list's box is read before the scroll because a scrolled box reads shifted (`:175-178`); the file's header says the limit too (`:11-13`). `stand_act_obstructed.rs` holds one pinning test (`:357-424`), a comment inside it that the box's own bounds are not read because they are not true (`:392-393`), and its header (`:13`).

## Patterns detected
- **The engine method itself, for every document** (`scrolling.rs:659-739`, `scroll_into_view`): widened in place on the founder's ratification, no second method beside it; its check is one engine-level file, `tests/blitz-tests/tests/scroll_into_view_nested.rs` (7 tests, parsed pages at 800 × 600, both layout modes).
- **A pinned defect is restated in place when its fix lands** (`stand_act_scroll.rs:358-360`, `stand_act_obstructed.rs:357-360`): each pinning test's own doc comment says it turns red by design and is then restated.
- **A fixture beside the stand** (`stand_act_scroll.rs:52-70`, `box_fixture`; `stand_act_obstructed.rs`, `boxed_rows_fixture`): an in-file component booted with `stand::options(incremental)`, every actionable element keyed by `id`, the fixture's condition asserted before the behaviour.
- **A schema text and the test that holds it move together** (`schema.rs:198-202` with `:435`; `refusal.rs:71-75` with `:254` and `json.rs:574`).

## Conventions to follow
- **Both layout modes**: every check loops `for incremental in [false, true]` and names the mode in its failure message (`stand_act_scroll.rs:166-167`).
- **A failure message names a mode and a task or fixture, never an id, a coordinate or what a screen reads** (`stand_act_scroll.rs:13-15`).
- **One test target per file directly under `tests/blitz-tests/tests/`**, no `[[test]]` table added (pinned by `.github/scripts/test_blitz_tests_targets.py`); an engine-level check carries no `stand_` prefix and no `cfg(unix)` gate.
- **A spec citation beside the engine change** (`document.rs:2254-2255`, `:2292-2295`: CSSOM View named at the reader); public items carry `///`, and rustdoc warnings are errors (`bash .github/scripts/ci-leg.sh doc`).
- **Every `tracing` call in blitz-dom sits under `#[cfg(feature = "tracing")]`**; neither fix needs a log site.

## New files to create
- `tests/blitz-tests/tests/scrolled_box_client_rect.rs` — the engine-level check of the bounds fix, on parsed pages, both layout modes
- `tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs` — the engine-level check of the hit fix, on parsed pages, both layout modes
- `tests/blitz-tests/tests/scrolled_box_absolute_position.rs` — the second reader measured and pinned as it reads (added at the P5 review, on the operator's answer to fork 2)
- `escher-0.1.0/chunks/2026-10-10-scrolling-box-bounds-and-hit/evidence/` — the red-then-green readings, the mutation controls and the conformance reading

## Files to modify
- `packages/blitz-dom/src/document.rs` — the bounds reader's first body and its callers in the file
- `packages/blitz-dom/src/layout/writing_mode.rs` — the bounds reader's second body
- `packages/blitz-dom/src/scrolling.rs` — the visible region's own-offset cancellation
- `packages/blitz-dom/src/node/node.rs` — the hit walk
- `packages/escher-driver/src/refusal.rs` — the meaning of the covered cause and its held copy
- `packages/escher-driver/src/schema.rs` — two help texts and their held copies
- `packages/escher-driver/src/json.rs` — the held copy of the covered meaning
- `packages/escher-driver/src/lib.rs` — the crate docs that state both limits
- `packages/escher-driver/src/execute.rs` — one private doc comment
- `tests/blitz-tests/tests/stand_act_scroll.rs` — two pinning tests restated, one comment and the header
- `tests/blitz-tests/tests/stand_act_obstructed.rs` — one pinning test restated, one comment and the header

## Open questions
- Where the bounds fix sits — the shared reader with its three content-origin callers, or `get_client_bounding_rect` alone → blocks: plan-decision (it decides whether `writing_mode.rs` and `scrolling.rs` are edited at all)
- Which boxes stop a hit — every box that clips by `overflow`, scroll containers alone, or everything paint clips → blocks: plan-decision
- Whether a conformance reading is taken: `WPT_DIR` is unset and no web-platform-tests checkout was found on this host (`$HOME/dev/*/wpt`, `$HOME/dev/projects/wpt`, `$HOME/wpt`); the fork's CI runs none; what a clone and a release run cost here is not measured → blocks: plan-decision

(A fourth is stated under "A second reader with the same shift": `Node::absolute_position` is left and given a route owner, measured and pinned, or fixed here.)

## How the open questions closed (P4 and the P5 review)
- The bounds fix sits in the shared reader — the operator, `inputs#I2`.
- A hit stops at every box that clips by `overflow`, by the overflow half of paint's own predicate (`packages/blitz-paint/src/render.rs:352-358`; `:324` carries upstream's note that the two axes are not told apart) — the operator, `inputs#I4`. This phase had placed the fork with the founder; the operator answered it as a technical fork and said why.
- The second reader is measured and pinned in this chunk, its fix owned on the route — the operator, `inputs#I4`.
- A conformance reading is taken, report-only and bounded — the operator, `inputs#I4`, reversing `inputs#I2` on the fact read at P4: the runner carries its script engine unconditionally (`wpt/runner/Cargo.toml:17`, `wpt/runner/src/test_runners/harness_test.rs:1-3`), a suite is any path under the checkout (`wpt/runner/src/main.rs:239-244`), and only the checkout is missing. Read at P5 for it: the runner writes its results under `wpt/output/` and deletes that directory at the start of each run (`main.rs:492-496`, `:831-844`); `target/wpt-checkout`, `target/wpt-reading` and `wpt/output` are ignored by git (`git check-ignore -q`, exit 0 each); the host has 32 logical cores and `taskset` (`nproc`; `command -v taskset`); `contain: paint` is read by paint (`render.rs:327-336`).
