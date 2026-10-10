# Report — 2026-10-10-scrolling-box-bounds-and-hit

**Chunk:** Scrolling-box bounds and hit — a scrolled box reads its own bounds where it shows, and a hit never reaches content scrolled out of its box; both fixed in the engine
**Date:** 2026-10-10T15:32Z
**Commits:** `5cc38cba chore(2026-10-10-scrolling-box-bounds-and-hit): operator pre-CI commit, for the run this chunk verdict reads` (the one commit since `last_wrap` 2026-10-10T13:05:37Z beside the last wrap's own `1b195c9a`; `git log --format='%h %s' 1b195c9a..HEAD`, 1 row). The chunk's base is `1b195c9a`, the parent of that pre-CI commit.

## Changes (structured — detectors read this)

- **Files:** 15 source and check files against the base (`git diff --name-status 1b195c9a`, rows outside `.andromeda/`, `.claude/` and the version folder: 12 modified · 3 added — the same 15 `gate.py scope` reads as changed).
  - Engine, 4 modified: `packages/blitz-dom/src/document.rs` · `packages/blitz-dom/src/layout/writing_mode.rs` · `packages/blitz-dom/src/scrolling.rs` · `packages/blitz-dom/src/node/node.rs`.
  - Driver, 5 modified, text only: `packages/escher-driver/src/refusal.rs` · `schema.rs` · `json.rs` · `lib.rs` · `execute.rs`.
  - Checks, 3 new: `tests/blitz-tests/tests/scrolled_box_client_rect.rs` (331 lines, 6 tests) · `tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs` (396 lines, 9 tests) · `tests/blitz-tests/tests/scrolled_box_absolute_position.rs` (78 lines, 1 test).
  - Checks, 3 modified: `tests/blitz-tests/tests/stand_act_scroll.rs` · `tests/blitz-tests/tests/stand_act_obstructed.rs` · `tests/blitz-tests/tests/stand_actionable_keys.rs` (outside research's lists — the scope record, under Deviations).
  - Chunk evidence, 4 new: `evidence/controls.md` · `evidence/conformance.md` · `evidence/readings.md` · `evidence/operator-pass.md`; and `scope-record.md`.
  - One file of another run was rewritten in the operator pass, on the operator's word: `.andromeda/runs/2026-10-10T13-07-20-phase/relay-3.md` (one token; under Decisions & corrections).
- **Symbols / APIs:** no symbol added or removed and no signature changed; no port, socket, env var, wire request, verb, argument, result field or refusal cause added. What four engine functions and three driver texts ANSWER changed:
  - `BaseDocument::physical_unrounded_geometry` (`pub(crate)`), both bodies — `document.rs` (compiled without blitz-dom's `writing-mode` feature; new text 2256-2261, 2271-2277) and `layout/writing_mode.rs` (compiled with it; new text 375-380, 391-397, 409-410): the position it returns is the box's own. The scroll offset of every box on the containing-block chain above the node is still subtracted; the node's own no longer is, on the containing-block walk of both bodies and on the vertical-chain path of the second. It keeps its six callers (research.md §Graph impact: `get_client_bounding_rect`, `offset_rect`, `inline_fragment_rects`, `resolved_style_value` ×2, `visible_region`) — not a sole-caller change.
  - `BaseDocument::get_client_bounding_rect` (`pub`), unedited: for a box that is itself scrolled it now answers the rect the box stands at, the same before and after its content is scrolled; for every other node it answers as before. Its readers, all unedited: the snapshot's `bounds` (`dioxus-native-dom`), a script's `getBoundingClientRect` and `getClientRects`, a Dioxus element's `get_client_rect`, a pointer event's element-relative coordinates.
  - `BaseDocument::offset_rect` (`pub`; new text `document.rs` 2348-2350, 2354-2356) and `BaseDocument::inline_fragment_rects` (`pub`; 2392-2393, 2395-2397): each now applies the scroll offset of the box it takes as its content's origin itself; each answers what it answered before (the two guard cases of `scrolled_box_client_rect`, green on the engine as built and after).
  - `BaseDocument::visible_region` (`pub`; new text `scrolling.rs` 774): its add-back of a clipping box's own offset is gone — it takes the reader's position as the box's origin; it answers what it answered before (guard case, and the unedited `scroll_into_view_nested`).
  - `Node::hit` / `hit_inner` (`node.rs`; new text 1359, 1377-1382, 1385-1386, 1389-1391, 1395-1396, 1447-1472), reached through `BaseDocument::hit`, `hit_with_scrollbar` and `element_from_point` — 7 production call sites kept (research.md §Graph impact): two changes. (1) A node's own area is tested where the box stands — on the point before the scroll offset is added — where it was tested in scrolled coordinates as `0 ..= size + scroll offset`. (2) A node that clips by `overflow` — either axis of its computed `overflow` not `visible`, the root element excepted, the two axes not told apart: the overflow half of paint's predicate — stops the walk at its padding box: for a point outside that box none of its positive-z hoisted children, paint children, negative-z hoisted children or inline content is reached, and the node itself is still answered when the point lies in its border box. Inside the padding box the order, the `visibility` and `pointer-events` rules and the thumb resolution are as they were.
  - **What the hit walk does not stop at** (decided, `inputs#I4`, option 1B): a `contain: paint` box whose `overflow` is `visible` — measured, still hit through (`hit_clipped_at_scrolling_box`, its last test); an image's, a sub-document's and a text input's own box — not measured.
  - **A box clipped on one axis** (`overflow-x: clip` beside `overflow-y: visible`, computing as written — asserted by the fixture): the hit stops at its whole padding box, as paint clips it, while `visible_region` narrows such a box on its clipped axis only. A target lying outside the box along its visible axis reads in view by the visible region and is not reached by a hit.
  - `Node::absolute_position` (`pub`), **not edited** (decided, `inputs#I4`, option 2B): measured and pinned as it reads — for a box that is itself scrolled it answers the box's position less the box's own scroll offset (a box at (40, 50) scrolled by (70, 120) reads (−30, −70), both layout modes, both builds). The harness's `layout_rect`, `layout_rect_of` and `center_of` are built on it and read a scrolled box the same way. Its callers are unedited.
  - `escher_driver::Cause::Covered`'s meaning (`refusal.rs` new text 73-74; held copies 254-255 and `json.rs` 574-575): "another element is hit at the point the action would land; a hit reaches content clipped by `contain: paint`, so an element lying where such content extends can read `covered` though nothing shows over it". It is in every `covered` refusal's JSON line.
  - The help of the result field `changed` (`schema.rs` new text 200; held copy 426): "the elements on the screen before and after the step whose role, name, state, bounds or parent differ, as they read after it". The help of `snapshot`'s `text` (227; held copy 421): "the screen as one text: one line per element, nested by indent". Both are the words that stood before 2026-10-10-driver-cli.
  - Crate docs of escher-driver (`lib.rs` new text 51-56): the paragraph that stated two readings as off now states that an element's `bounds` are where it stands, that a hit stops where a box clips by `overflow`, and one reading as off — the `contain: paint` one. A private doc comment (`execute.rs` 323) restated. `execute.rs` differs from the base in `///` lines only (the gate `git diff -U0 1b195c9a… -- packages/escher-driver/src/execute.rs | …`, green at `exit 1`, `last line 0`).
- **Crates / modules:** none added or removed. Changed: `blitz-dom` (four files, behaviour) · `escher-driver` (five files, fixed texts and docs; no logic) · `blitz-tests` (three new test targets, three edited). Unchanged, held by the preservation gate (`git diff --quiet 1b195c9a… -- Cargo.toml Cargo.lock deny.toml … .github`, green): every manifest and the lock, `examples/seven_guis`, `dioxus-native-dom`, `blitz-test-harness`, `blitz-paint`, `blitz-vibey-script`, `escher-telemetry`, the driver's `session` · `command` · `cli` · `host` · `client` · `wire` · `error`, the engine's `accessibility.rs` and `events/`, both shared test modules, `wpt/runner`, `wpt/WPT_COMMIT`, `scripts`, `.github`.
- **Dependencies:** none added, none bumped — the same gate (`Cargo.toml`, `Cargo.lock`, `deny.toml` and three crate manifests read equal to the base).
- **Schema / config:** no key, shape, wire request or scrub set changed. Three fixed `&'static str` texts of the driver's schema changed (above); the cause set is eight, in its order (`cargo test -p escher-driver --locked`, 50 unit tests green).
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - Workspace test run (`target/ci-logs/test.log`, written by the `fast` leg on the committed tree `5cc38cba`, counted by parsing its `test result:` lines): **165 result lines · 776 passed · 0 failed · 11 ignored**, from 162 · 760 · 0 · 11. +3 result lines and +16 passed are the three new targets, 6 + 9 + 1. Stated at `test-plan.md:329` (the one line of the seven masters holding "162 result lines": `grep -rn "162 result lines"` over the masters and `.andromeda/registries/`, 1 hit).
  - blitz-tests files directly under `tests/blitz-tests/tests/`: **106 `.rs` files, 105 test targets and the unbuilt `all.rs`** (`ls tests/blitz-tests/tests/*.rs | wc -l` = 106), from 103 and 102. Stated at `test-plan.md:66` (`grep -n -E "[0-9]+ test targets"`, 1 hit in test-plan, 0 in architecture and obs-plan).
  - None — verified, re-measured unchanged: the stand run, `run.end` 119 passed · 0 failed · 6 ignored over 31 files (stated at `test-plan.md:116`); escher-driver's unit tests, 50; the a11y leg's three targets, 6 + 6 + 3; `stand_act_scroll` 6 tests and `stand_act_obstructed` 5 (three tests renamed, none added or removed — the old names stand in no master and no leaf: `grep -rn -E "reads_its_bounds_shifted|refused_off_screen_while_the_box_is_in_view|reads_covered_only_before_its_box"` over the seven masters, `CLAUDE.md`, `.claude/docs`, `.claude/rules`, 0 hits); `stand_actionable_keys`' pinned counts 2 · 3 · 676.
  - **Two qualifiers flip from "measured and unfixed" to fixed:** "a scrolled box's own `bounds` read shifted by its scroll offset" and "a hit reaches content scrolled out of a scrolling box". Sites in the masters, by sweep at this wrap (pattern · hits per master body + key files): `shifted` — architecture 2 (`:133`, `:136`) · test-plan 2 (`:26`, `:181`); `own scroll offset` — architecture 3 (`:124`, `:133`, `:136`); `scrolled out of` — architecture 1 (`:133`) · security-plan 1 (`:339`) · test-plan 1 (`:26`); `pinned as (it|they) read` — architecture 1 (`:263`) · test-plan 1 (`:181`); 0 in the other masters and in every key file. Each hit is a site for the fan-out to read; none was edited here. Three leaves state the limits too (`grep -rn -l` of the limit phrases over `CLAUDE.md`, `.claude/docs`, `.claude/rules`): `CLAUDE.md` · `.claude/docs/services/escher-driver.md` · `.claude/docs/services/dioxus-native-dom.md` — the cascade's.
- **Dev-tool versions:** none — no host tool was installed, upgraded or read changed. `taskset` and `git` were used as found.
- **Harness / gate surface:** none — no agent-run script, CI step, leg, status or verdict shape changed (`scripts` and `.github` read equal to the base). The headless `Harness` crate is unedited; what its `layout_rect`, `layout_rect_of` and `center_of` read for a scrolled box is now measured (above), not changed.
- **Cross-project / external claims:**
  - **The fork's CI:** CI#38062991988 on `5cc38cba18577fee37732305a96d53bae6a1e99d`, `verdict: green · checks 16/16`, wall 667 s (`ci.py conclusion --sha HEAD --wait 1800`, recorded in `evidence/operator-pass.md`). It is the one witness of the MSRV build and of the windows, macos, ios and android legs. The verdict was taken on `5cc38cba`; this wrap's commit adds to that tree.
  - **web-platform-tests** (`github.com/web-platform-tests/wpt`) at `71b4d5f0eb7628a5d5f7cd2ee868ce1b1b5dc010`, the commit `wpt/WPT_COMMIT` pins, fetched at depth 1 into `target/wpt-checkout/` (ignored by git): the suite `css/cssom-view` run before and after the two fixes — 966 → 967 subtests passed of 2292 run; one subtest moved fail to pass (`css/cssom-view/getBoundingClientRect-scroll.html`, "getBoundingClientRect for a scrolled scroll container"), none moved pass to fail (`evidence/conformance.md`). Report-only; it grades no criterion.
  - **Upstream** (`DioxusLabs/blitz`): both fixes are flagged upstreamable, cut against upstream `main` at `7832c177`; no upstream pull request was opened. Not read: the upstream issue #1083 the standing ruling names.
  - **Two specifications cited in engine comments**, written from the plan's naming and not fetched at this chunk: CSSOM View `getBoundingClientRect` (`drafts.csswg.org/cssom-view/#dom-element-getboundingclientrect`) and CSS Overflow, the overflow properties (`drafts.csswg.org/css-overflow-3/#overflow-properties`). Neither was fetched by the chunk or by this wrap; the operator fetched both on 2026-10-10 at 15:36Z and reads both anchors as resolving (`inputs#I7`).
  - **Inputs** (`inputs.py verify`, at this wrap's first firing: `inputs: 5 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 4 · uncited 1 · unparsed 0`; at its resume, after the two snaps: `inputs: 7 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 5 · uncited 0 · unparsed 0`):
    - `I1` · message: the operator, as the arguments of /andromeda-phase, 2026-10-10 · copy message · n/a — a message has no live source
    - `I2` · message: the operator, in the question dialog of /andromeda-phase P4, 2026-10-10 · copy message · n/a
    - `I3` · message: the operator, as the review of the plan at /andromeda-phase P5, 2026-10-10 · copy message · n/a
    - `I4` · `../additional/escher-overseer/relays/scrolling-box-fork-answers.md` · copy no-repo · unchanged
    - `I5` · message: the operator, as the approval word at /andromeda-phase P5, 2026-10-10 · copy message · n/a — printed `UNCITED` before this report existed; cited here: `inputs#I5` is the word "The covered clause stands as you wrote it, kept only if the check case measures that reading", and the check case measured it (Symbols / APIs, the `covered` meaning).
    - `I6` · `../additional/escher-overseer/relays/snapshot-states-route-adaptation.md` · copy no-repo · unchanged — snapped at this wrap's resume; read by the route-resolve only: the founder's ruling of 2026-10-10 that the states a snapshot does not read come into 0.1.0 as their own entry ahead of "MCP surface" (`inputs#I6`). No fact of the Changes above rests on it.
    - `I7` · message: the operator, as the arguments of /andromeda-wrap-session, 2026-10-10 · copy message · n/a — snapped at this wrap's resume (`inputs#I7`): the resume word, four route owners asked for, and the two anchors' reading above.
    No entry reads drifted, vanished or broken; no `UNPARSED:` row.
- **Reverted / negative API facts:** none. (A throwaway test target was written under `tests/blitz-tests/tests/`, run once to measure the Home cards and deleted before the gates re-ran; it is in no commit.)
- **Insufficient fixes (written, kept, not the remedy):** none. Two neighbouring readings are left by decision, not by a fix that fell short, and each is owed a route owner: `Node::absolute_position` (measured and pinned, option 2B) and the hit's remainder under option 1B (`contain: paint` measured; an image, a sub-document and a text input's own box not measured).
- **Spec claims disproved by measurement:**
  - **The plan's statement of what a11y-plan holds** (`plan.md` step 7 and its Implementation notes: "a11y-plan §5 records that a click on a plain button is read, not measured, to clear focus"). Read at this wrap: a11y-plan §5 → Focus restoration holds one click line, `a11y-plan.md:154`, "Clicking a non-interactive area clears focus", and no line on a plain button (`grep -n -i -E "plain button|clears? focus|focus cleared"` over the seven masters: 1 hit, that line). **What the chunk measured** (the restated `stand_act_obstructed` test, both layout modes, both builds): after an accepted pointer click on a plain `button` no element reads `focused` in the snapshot and the accessibility tree's focus carries no author id. The master is not contradicted; it holds no statement of this reading, which is now a measured one.
  - **A premise two of the stand's own checks rested on, never stated in a master:** that a raw click at the centre of any Home card lands on the card. Measured at the stand's viewport, Home unscrolled: `#home` is a scrolling box 600 high, six cards lie inside it, the seventh (Cells) at 615 to 684 — below the box; on the fixed engine a hit there does not land on it (`evidence/readings.md`). `layout-templates.md:10` describes Home as a column of task cards and does not say they all show.
- **Expected amendments (from plan):** nine entries; the sweep behind each line is the one named under Counts / qualifiers moved unless it says otherwise (seven masters and every file under `.andromeda/registries/`, hits as body + key files).
  - architecture §Standard Contracts → Dioxus DOM bridge, the `bounds` clause — **carried**: Symbols / APIs (the reader's two bodies, `get_client_bounding_rect`). Sites: `physical_unrounded_geometry` — architecture 1 (`:136`) · test-plan 1 (`:323`); `shifted` at `architecture.md:136`.
  - architecture §Standard Contracts → Scrolling, selection, tree — **carried**: Symbols / APIs (`visible_region`, the hit walk and what it does not stop at, the one-axis case, `absolute_position` as measured). Sites: `visible_region` — architecture 2 (`:124`, `:133`) · layout-templates 1 (`:45`) · test-plan 1 (`:323`); `absolute_position` — 0 in every master and key file (the measured reading has no site yet); `own scroll offset` at `architecture.md:124`.
  - architecture §Standard Contracts → Driver session — **carried**: Symbols / APIs (the three texts, the crate docs) and Counts / qualifiers moved (the two qualifiers). Sites: `architecture.md:133` (hit by `shifted`, `own scroll offset`, `scrolled out of`, `visible_region`); `stand_act_scroll|stand_act_obstructed` — architecture 2 · layout-templates 1 · test-plan 8 + 1 key file.
  - architecture §Established Decisions → Scrolling and input — **carried**: Symbols / APIs (two engine changes for every document), Cross-project (upstreamable, base `7832c177`), and the authorities under Decisions & corrections (the founder's ruling of 2026-10-10 as scope.md CARRY 1 holds it; the operator's answers `inputs#I4`). Sites: the section by its heading; `hit walk|hit reaches|hit-test|hit test` — architecture 8.
  - security-plan §Error Handling → Error format (typed errors), the escher-driver refusal item — **carried**: Symbols / APIs (three fixed texts restated, the cause set eight) and Schema / config. Site: `security-plan.md:339` (`scrolled out of`, 1 hit; `covered` — security-plan 2).
  - test-plan §5 → Session ↔ held instance; §4 → escher-driver; §1 and §9; §9 → Engine features by runner; §2 → WPT conformance — **carried**: Counts / qualifiers moved (165 · 776; 106 files, 105 targets; the two qualifiers), Symbols / APIs (both bodies of the reader now hold a check of ours — `scrolled_box_client_rect`, read under both builds), Cross-project (the first conformance reading on this host, with its cost in `evidence/conformance.md`), and Outcome (the restated checks). Sites: `test-plan.md:181` (§5), `:26` and `:66` (§1), `:329` (§9 local baseline), `:323` (§9 Engine features by runner; `writing-mode|writing_mode` — test-plan 3), `:57` (§2 WPT conformance, by `grep -n "WPT conformance"`), the key file `registries/contracts/test-plan/session-lifecycle.md` (2 lines hit by `covered|stand_act_scroll|stand_act_obstructed`, the only key file with a hit).
  - design-system §Depth Strategy → Engine depth behavior — **carried**: Symbols / APIs (the hit walk's edge rule). Site: `design-system.md:215`, the one hit-testing sentence under the heading at `:212` (`hit walk|hit reaches|hit-test|hit test` — design-system 1); it cites `packages/blitz-dom/src/node/node.rs:1440-1502`, lines this chunk moved.
  - layout-templates §Surface: desktop-native → IA notes — **carried**: Outcome (the layouts criterion: `crud-list` reads the same four figures after its content is scrolled, the scroll's diff does not name it, a click naming it lands on the row at its centre at 12 Creates and at 14). Sites: `crud-list` — layout-templates 2 (`:10`, `:45`); and the Home finding above bears on `:10`.
  - obs-plan §3 Observability Harness Contract → Logging stack, the no-subscriber census — **carried**: Files (three new check files) and Coverage (none installs a subscriber, none reads an env var; none declares `common` or `session_common`). Site: `obs-plan.md:70` (`subscriber` — obs-plan 16 + 1 key file; the census sentence is the hit that counts check files).
  - Beside the list, the plan's **route pins owed at the wrap** (not amendments; P5's): an owner for the fix of `Node::absolute_position` with the measured figures; an owner for the remainder under option 1B; the merge-surface blocks on "Upstream sync ahead of polish and ship" re-measured over the four engine files; the `watch:` tally.
- **Coverage of new surfaces** (no new external surface, hot-path op or UI element; the changed ones):
  - the bounds reader, both bodies → validation n/a · instrumentation n/a (no log site; the engine-lines gate reads 0 added `tracing::`, print or panic lines) · PII n/a · tests integ (`scrolled_box_client_rect`, 6, per-package and workspace builds; `stand_act_scroll` restated) · a11y n/a (the accessibility tree reads no geometry) · tokens n/a — **not covered by a check:** a scrolled box under a vertical writing mode, the second body's vertical-chain path (new text 409-410). Measured at this wrap's resume, raised at Validate: no fixture of the three new files sets a `writing-mode` style, the body returns before that path where no box on the chain is vertical, and the one blitz-tests file that sets a vertical `writing-mode`, `inline_fragment_rects`, scrolls nothing
  - the hit walk → validation n/a · instrumentation n/a (same gate) · PII n/a · tests integ (`hit_clipped_at_scrolling_box`, 9, both builds; `stand_act_obstructed` restated; standing `paint_order`, `scrollbars`, `scrollbar_drag`, `fragment_navigation`, `touch_action`, `text_selection_anonymous_block`, `stale_dirty_descendants` green unedited) · a11y focus✓ (after the accepted click no row reads focused; Tab order unmoved — `stand_accessibility_ids`, the a11y leg 6 + 6 + 3) · tokens n/a — **not covered by a check:** a box that is both transformed and scrolled
  - the driver's three texts → validation n/a (fixed strings; a refusal holds nothing of the call) · instrumentation n/a (the command span keeps its eight fields — `stand_act_spans` green) · PII: no id, name, value or path in a text (`cargo test -p escher-driver --locked`, 50; `host_log` under both builds, 0 needles) · tests unit + integ (`cli_commands`, `cli_flow`) · a11y n/a · tokens n/a
  - three new check files → install no subscriber, read no env var, carry no `cfg(unix)` gate, declare no shared module

## Deviations from intent

- **A standing check outside research's lists was edited.** `stand_actionable_keys::the_non_lean_tasks_are_measured` went red at the first full gate pass (the stand run and the `fast` leg, one cause): it opened the Cells task by a raw harness click on a Home card lying below Home's scrolling box, which landed only through the hit defect step 6 fixes. The check now scrolls each card into view before its click (new text 161-164). The count it pins did not move; no stand file was edited. Research's sweep for checks that pin the changed reading searched scroll verbs and the limit phrases and did not reach it; the plan had left "whether any stand node lies outside its clipping ancestor" unmeasured and named the stand gates as the reader.
- **Step 6, a transformed box.** The plan tests a box's own area "before the scroll offset is added" and leaves the transform handling as it is; the walk inverse-transforms its point after the offset is added. The own-area and padding-box tests are made on the unscrolled point carried through the same inverse transform (paint pushes the clip before it translates the content by the scroll offset). For a box with no transform this is the plan's point exactly. No check reads a box that is both transformed and scrolled.
- **Step 5, the loop's shape.** Both bodies keep the walk from the node itself and leave the node's own offset out inside it; starting the walk at the containing block would have put `layout_data()` on an added line of the `writing_mode.rs` body, which the engine-lines gate forbids.
- **Step 7, the fixture test of `stand_act_scroll`.** Every row of the fixture rewrites one count, so "lands on the row at its centre" is asserted as: one row's bounds hold the box's centre, the hit there answers that row, and the click naming the box rewrites the count. On the stand's CRUD the row is identified by deleting it.
- **Step 1, the offset-rect case** reads two pages where the plan names one: the positioned inline root itself as the offset parent, and a positioned scrolling box around a paragraph.
- **Mutation control M3** turned red, beside its named case, three tests of the unedited `scroll_into_view_nested` (`evidence/controls.md` §4) — a wider red than the plan named, not a different one.
- **The scope record** — `gate.py scope` at this wrap: `scope: clean — changed 15 · listed 14 · recorded 1 (companion 1 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 56`.
  - companion: `tests/blitz-tests/tests/stand_actionable_keys.rs` · serves step 6 · self

## Decisions & corrections

- **The founder's ruling** (2026-10-10, relayed by the overseer; scope.md CARRY 1): both defects fixed in the engine, each a direct change, ahead of "MCP surface". **The operator's answers** (`inputs#I2`, `inputs#I4`): the bounds fix in the shared reader; a hit stops at every box that clips by `overflow`, paint's overflow half; the second reader measured and pinned, its fix owned on the route; a conformance reading report-only and bounded. **The operator's approval word** (`inputs#I5`): the `covered` clause kept only if the check case measures that reading — it did.
- **The operator pass, on the operator's word** (given in this session, 2026-10-10; `evidence/operator-pass.md`): entry by entry in plan order, "Stop and tell me if any entry is red". The hygiene entry read `refused 1 files` at its first firing — the phase's run-dir copy of the operator's own P5 review message named a file by a home path outside the repository. The pass stopped; the operator ruled the one token rewritten to `<overseer>/relays/scrolling-box-fork-answers.md`, nothing else (629 → 585 bytes; both sha256 in the record). The verbatim input copy `inputs/I3-relay-3.md.txt` keeps the message as given. This is the second operator pass in two chunks stopped by a host path in a committed run-dir text (2026-10-10-driver-cli: a temp-dir path in the phase's forks file).
- **The operator's word for this wrap:** "run P1 only and stop" — the report is authored; no fan-out, no gate, no flip, no commit followed in that session. **Resumed on the operator's word** (`inputs#I7`, 2026-10-10, a later session): "resume from report.md" — the report is reused as authored, with four lines amended at the resume (this one, the two specifications' line, the Inputs list — entries `I6` and `I7` — and the bounds reader's Coverage line, which gained an unchecked path found at Validate); the fan-out, the gates, the flip and the commit follow in the same run dir. The route-resolve reads the founder's ruling `inputs#I6` and the operator's four owner requests (`inputs#I7`).
- **Sweep hazards found:**
  - A check that leaned on the hit defect holds none of the words a sweep for it searches (`scroll`, the limit phrases, the stand file names): it is a raw `Harness::click` on an element lying outside its clipping ancestor's box. It was found by the gates, not by a pattern.
  - `\b162\b|\b760\b` over the masters returns 13 lines of which one states the count (`test-plan.md:329`); the other twelve are the digits of `path:162` citations. The count is found by `162 result lines`.
  - The masters are one-line bullets hundreds of characters long: a hit's line number names a whole contract, so two patterns hitting `architecture.md:133` are one site read twice, not two sites.
- **Process corrections, this session:** a hand-written loop polling a backgrounded script's log never matched and ran to the Bash tool's ceiling while the script's own completion notice had already arrived — a backgrounded command is waited on by its notice, not polled; the Bash guard refused a `cat` heredoc with a file target (a script goes through the Write tool) and a `cd` out of the project root inside a compound command (a subshell passes) — one re-issue each.
- **Measured in passing, for whoever owns it:** the conformance checkout (1.1 G), its result files and the runner's release build are left under `target/`, ignored by git, not deleted.

## Outcome

**Acceptance criteria**, each re-asserted against the diff:

- (arch) a scrolled box reads the same client rect before and after, a nested scrolled box moves by the outer offset alone, under both builds and both layout modes — **met**: `scrolled_box_client_rect` green under `cargo test -p blitz-tests --locked --test …` and `cargo test --workspace --locked --test …`; both reader bodies carry the change (new text in `document.rs` and in `writing_mode.rs`); controls M1 and M2 each red under its own build only.
- (arch) the three re-based callers read as before — **met**: the four guard cases green on the engine as built and after; the twelve standing files of the standing-checks entry green unedited.
- (design) a point outside the padding box of a box that clips by `overflow` never answers its content; one-axis pinned; inside the box unchanged — **met**: `hit_clipped_at_scrolling_box` 9 green under both builds; `paint_order`, `scrollbars`, `scrollbar_drag`, `fragment_navigation` green unedited.
- (layouts) `crud-list` reads the same four figures after its content is scrolled, the diff does not name it, a click naming it lands on the row at the box's centre at 12 Creates and at 14; the last row is still refused `off-screen` and a `scroll` brings it in — **met**: `stand_act_disabled`, `stand_act_obstructed`, `stand_act_scroll` green.
- (a11y) the button lying where a scrolled-out row extends is clicked, its handler runs and no row's; no row reads focused; the button's own focus asserted as measured — **met**: nothing reads focused after the click (measured, both modes, both builds).
- (a11y) Tab order and the a11y leg unmoved — **met**: `stand_snapshot_state`, `stand_accessibility_ids` green unedited; `ci-leg.sh a11y` 6 + 6 + 3.
- (security) refusal order `disabled` · `off-screen` · `covered` and nothing run on a refusal — **met**: the three refusal checks and the six standing driver-action checks green.
- (security) three fixed strings, cause count eight, no verb, argument, field or cause added, no file of the driver states either fixed limit — **met**: 50 unit tests; the `grep -r -n -E "scrolled out of a scrolling box|shifted by its own scroll offset" packages/escher-driver/src` entry green at `exit 1`, `no output`.
- (security) the engine's added lines hold no `unwrap`, `expect`, `panic!`, `unreachable!`, no panicking accessor, no print, no ungated `tracing` — **met**: the engine-lines gate green at `exit 1`, `last line 0`. The diff does add a read of `self.final_layout().border` and of `primary_styles()` in `hit_inner`; both are accessors the function already called before this chunk.
- (arch) escher-driver's logic unchanged; nothing outside the write set moves — **met**: the `execute.rs` gate and the preservation gate green. `tests/blitz-tests/tests/stand_actionable_keys.rs` is outside the plan's write set and is not among the paths the preservation gate names; it is recorded (Deviations).
- (obs) the command span keeps its eight fields; the host's stderr at `trace` holds 0 sentinel occurrences, 0 id needles, 0 name needles under both builds — **met**: `stand_act_spans`, `host_log` per package and for the workspace.
- (design) every answer line carrying the restated `covered` sentence is one line of JSON with no ESC byte; the hosted scroll and the click after it answer as before — **met**: `cli_commands`, `cli_flow`.
- (tests) each regression case red on the engine as built and green after, under both builds; each mutation control red on its named test and green on the restored tree, the restore proven by sha256 — **met**: `evidence/controls.md` §1 to §5 (2 + 5 regression cases; six controls; four files' sha256 equal after the last).
- (tests) the second reader measured, not fixed — **met**: `scrolled_box_absolute_position` green under both builds; no line of `Node::absolute_position` or of a caller of it is in the diff (`node.rs` new text ends at 1472; the reader stands below it, unedited).
- (tests) the script-facing readers hold — **met**: `cargo test -p blitz-vibey-script --locked` green.
- (tests) `fast` and `doc` exit 0, the stand run ends passed with 0 failed, the fork's CI reads `verdict: green` on the pushed sha, its run id named — **met**: CI#38062991988 on `5cc38cba`.

No criterion is contradicted by the diff, and none rests on an input that reads drifted, vanished or broken.

**Gates**, by `run`, in block order — implement's second full pass (run dir `.andromeda/runs/2026-10-10T14-33-38-implement/`), on the tree the pre-CI commit then carried:

- `cargo test -p escher-driver --locked` — green · exit 0, `lacks FAILED`
- `cargo test -p blitz-tests --locked --test scrolled_box_client_rect --test hit_clipped_at_scrolling_box --test scrolled_box_absolute_position` — green · exit 0
- `cargo test --workspace --locked --test scrolled_box_client_rect --test hit_clipped_at_scrolling_box --test scrolled_box_absolute_position --test stand_act_scroll --test stand_act_obstructed` — green · exit 0
- `cargo test -p blitz-tests --locked --test stand_act_disabled --test stand_act_obstructed --test stand_act_scroll` — green · exit 0
- `cargo test -p blitz-tests --locked --test stand_act_ids --test stand_act_diff --test stand_act_timer --test stand_act_refused --test stand_act_keys --test stand_act_range` — green · exit 0
- `cargo test -p blitz-tests --locked --test stand_act_spans` — green · exit 0
- `cargo test -p blitz-tests --locked --test scroll_into_view_nested --test inline_fragment_rects --test scrollbars --test scrollbar_drag --test fragment_navigation --test paint_order --test touch_action --test text_selection_anonymous_block --test stale_dirty_descendants --test stand_snapshot --test stand_snapshot_state --test stand_accessibility_ids` — green · exit 0
- `cargo test -p blitz-vibey-script --locked` — green · exit 0
- `cargo test -p seven_guis --locked --test cli_commands` — green · exit 0
- `cargo test -p seven_guis --locked --test cli_flow` — green · exit 0
- `cargo test -p seven_guis --locked --test host_log` — green · exit 0
- `cargo test --workspace --locked --test host_log` — green · exit 0
- `grep -r -n -E "scrolled out of a scrolling box|shifted by its own scroll offset" packages/escher-driver/src` — green · exit 1, `no output`
- `git diff -U0 1b195c9a… -- packages/escher-driver/src/execute.rs | …` — green · exit 1, `last line 0`
- `git diff 1b195c9a… -- packages/blitz-dom/src | …` — green · exit 1, `last line 0`
- `git diff --quiet 1b195c9a… -- Cargo.toml Cargo.lock deny.toml …` — green · exit 0
- `bash scripts/agent-run.sh boot` — green · exit 0
- `bash scripts/agent-run.sh run stand` — green · exit 0 — at the first full pass `red · exit 0 ✗ (exit 1)`, the one failing test `stand_actionable_keys::the_non_lean_tasks_are_measured`; green after the recorded edit
- `bash .github/scripts/ci-leg.sh fast` — green · exit 0 — at the first full pass `red · exit 0 ✗ (exit 101)`, the same test; green after the same edit
- `bash .github/scripts/ci-leg.sh doc` — green · exit 0
- `bash .github/scripts/ci-leg.sh a11y` — green · exit 0
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`, fired as written in the operator pass: first reading `hygiene: refused 1 files` (red), then, after the one rewrite the operator named, `hygiene: clean` — green · exit 0, `contains hygiene: clean`, on the tree the pre-CI commit carries
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` — `leg = 'operator'`, fired once through `gate.py run … --operator 23`: green · exit 0 · `history moved: refs/remotes/origin/build/escher-0.1.0 1b195c9a→5cc38cba`, the one ref the push was meant to move
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — `leg = 'operator'`, fired as written: green · exit 0, `contains verdict: green` — CI#38062991988, 16 of 16

No entry carries `defer`; none was skipped. Smoke: skipped — no boot path edited and the project has no headful self-verify; the `escher-session` binary ran as a process under the four `seven_guis` and `host_log` entries.

**Watches:** a fork CI job hanging in its package-install step · 1 green run [CI#38062991988] — the run's log holds 0 lines of `apt-install: attempt` among 1,182 holding `apt-install` (`gh run view 38062991988 -R Turbolet85/escher --log`, 17,545 lines, read once in the operator pass); no job was held.

**Outcome basis:** the operator pass ran, so the verdicts above rest on its final state: the one pass commit (`5cc38cba`, Setup's list) and the final HEAD's CI run as recorded in `evidence/operator-pass.md` — entry 23's `fast` leg re-ran the workspace on the committed tree (165 result lines · 776 passed · 0 failed · 11 ignored). Implement's P4 report, given in this session's conversation, is the basis for what only it holds: the red-first readings, the mutation controls, the conformance reading and the deviations (kept in `evidence/controls.md`, `conformance.md`, `readings.md`). Post-implement artifacts: `evidence/operator-pass.md`, and the one-token rewrite of the phase's `relay-3.md`. No fix commit followed the pre-CI commit. The tree at this report holds one uncommitted file beside this wrap's own: `evidence/operator-pass.md`, extended after the push.

**Process hygiene** (implement's census, re-measured at this wrap — `ps -eo comm=`, 0 rows matching `escher-session`, `seven_guis`, `wpt`, `cargo` or `rustc`):

| Process | Started by | Final state |
|---|---|---|
| the WPT fetch, the runner's release build and its two runs | implement | terminated — exited by themselves |
| the mutation-control batch (cargo children) | implement | terminated — exited by itself |
| a log-polling loop | implement | terminated — stopped by task id |
| the gate tool's three block runs and their cargo children | implement | terminated — exited by themselves |
| the operator entries (the `fast` leg and the push; the CI wait) | the operator pass | terminated — exited by themselves |
| the code-graph refresh | this wrap | terminated — exited by itself (`tree-refresh[rust]: 7316 nodes / 41707 edges`) |
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 1b195c9a (the parent of the oldest pre-CI commit 5cc38cba) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### packages/blitz-dom/src/document.rs — added 24 line(s) in 6 range(s)
added: 2256-2261 · 2271-2277 · 2348-2350 · 2354-2356 · 2392-2393 · 2395-2397
### packages/blitz-dom/src/layout/writing_mode.rs — added 15 line(s) in 3 range(s)
added: 375-380 · 391-397 · 409-410
### packages/blitz-dom/src/node/node.rs — added 40 line(s) in 6 range(s)
added: 1359 · 1377-1382 · 1385-1386 · 1389-1391 · 1395-1396 · 1447-1472
### packages/blitz-dom/src/scrolling.rs — added 1 line(s) in 1 range(s)
added: 774
### packages/escher-driver/src/execute.rs — added 1 line(s) in 1 range(s)
added: 323
### packages/escher-driver/src/json.rs — added 2 line(s) in 1 range(s)
added: 574-575
### packages/escher-driver/src/lib.rs — added 6 line(s) in 1 range(s)
added: 51-56
### packages/escher-driver/src/refusal.rs — added 4 line(s) in 2 range(s)
added: 73-74 · 254-255
### packages/escher-driver/src/schema.rs — added 4 line(s) in 4 range(s)
added: 200 · 227 · 421 · 426
### tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs — new file · 396 line(s)
- 21-24 «use blitz_traits::events::{»
- 35-50 @38 «fn rows_page(box_style: &str) -> String {»
  - 39-41 «let rows: String = (0..10)»
  - 42-43 «format!(»
  - 44-47 «<div id="stage" style="width:400px; height:500px">»
- 52-64 «fn page(html: &str, incremental: bool) -> HtmlDocument {»
  - 53-60 «let mut doc = HtmlDocument::from_html(»
- 66-70 «fn node(doc: &HtmlDocument, selector: &str) -> NodeId {»
  - 67-69 «doc.query_selector(selector)»
- 72-78 @73 «fn edges(doc: &HtmlDocument, selector: &str) -> (f64, f64, f64, f64) {»
  - 74-76 «let rect = doc»
- 80-82 «fn inside(point: (f64, f64), (left, top, right, bottom): (f64, f64, f64, f64)) -> bool {»
- 84-91 @86 «struct Points {»
- 93-103 «fn points(doc: &HtmlDocument) -> Points {»
  - 97-102 «Points {»
- 105-109 @106 «fn answer(doc: &HtmlDocument, point: (f64, f64)) -> Option<NodeId> {»
  - 107-108 «doc.hit(point.0 as f32, point.1 as f32)»
- 111-117 @112 «fn offset(doc: &HtmlDocument) -> f64 {»
  - 113-116 «doc.get_node(node(doc, "#box"))»
- 119-122 «fn scroll_the_box(doc: &mut HtmlDocument) {»
- 124-129 @125 «fn overflow(doc: &HtmlDocument) -> Option<(Overflow, Overflow)> {»
  - 126-128 «doc.get_node(node(doc, "#box"))»
- 131-152 «fn move_the_pointer(doc: &mut HtmlDocument, point: (f64, f64)) {»
  - 134-151 «driver.handle_ui_event(UiEvent::PointerMove(BlitzPointerEvent {»
- 154-168 @157 «fn assert_a_scrolled_out_row_extends_over_the_sibling(doc: &HtmlDocument, at: &Points, mode: &str) {»
  - 158-161 «assert!(»
  - 162-167 «assert!(»
- 170-181 @173 «fn assert_an_overflowing_row_extends_below_the_box(doc: &HtmlDocument, at: &Points, mode: &str) {»
  - 175-180 «assert!(»
- 183-203 @184 «fn a_point_above_a_scrolled_box_answers_the_sibling_there() {»
  - 185-202 «for incremental in [false, true] {»
- 205-218 @206 «fn a_point_below_an_unscrolled_box_answers_what_shows_there() {»
  - 207-217 «for incremental in [false, true] {»
- 220-246 @221 «fn a_point_inside_the_box_answers_the_row_showing_there() {»
  - 222-245 «for incremental in [false, true] {»
- 248-266 @249 «fn a_hidden_overflow_box_scrolled_by_a_program_reads_the_same() {»
  - 250-265 «for incremental in [false, true] {»
- 268-285 @269 «fn an_overflow_clip_box_keeps_its_clipped_out_content_from_a_hit() {»
  - 270-284 «for incremental in [false, true] {»
- 287-310 @291 «fn a_box_clipped_on_one_axis_is_stopped_at_its_whole_padding_box() {»
  - 292-309 «for incremental in [false, true] {»
- 312-329 @313 «fn an_overflow_visible_box_is_hit_through_to_its_overflowing_content() {»
  - 314-328 «for incremental in [false, true] {»
- 331-368 @332 «fn a_scrolled_box_overlay_scrollbar_thumb_is_still_resolved() {»
  - 333-367 «for incremental in [false, true] {»
- 370-396 @375 «fn a_contain_paint_box_with_visible_overflow_is_still_hit_through() {»
  - 376-395 «for incremental in [false, true] {»
### tests/blitz-tests/tests/scrolled_box_absolute_position.rs — new file · 78 line(s)
- 17-23 @18 «const ONE_BOX: &str = r#"<html><body style="margin:0">»
  - 20-22 «<div id="box" style="margin-left:40px; width:300px; height:200px; overflow:auto">»
- 25-37 «fn page(html: &str, incremental: bool) -> HtmlDocument {»
  - 26-33 «let mut doc = HtmlDocument::from_html(»
- 39-46 @40 «fn absolute_position(doc: &HtmlDocument, node: NodeId) -> (f32, f32) {»
  - 41-44 «let position = doc»
- 48-78 @49 «fn a_scrolled_box_reads_its_absolute_position_less_its_own_scroll_offset() {»
  - 50-77 «for incremental in [false, true] {»
### tests/blitz-tests/tests/scrolled_box_client_rect.rs — new file · 331 line(s)
- 22-28 @23 «const ONE_BOX: &str = r#"<html><body style="margin:0">»
  - 25-27 «<div id="box" style="margin-left:40px; width:300px; height:200px; overflow:auto">»
- 30-39 @31 «const TWO_BOXES: &str = r#"<html><body style="margin:0">»
  - 32-38 «<div id="outer" style="width:400px; height:300px; overflow:auto">»
- 41-45 @42 «const SCROLLING_INLINE_ROOT: &str = r#"<html><body style="margin:0">»
- 47-51 @48 «const POSITIONED_INLINE_ROOT: &str = r#"<html><body style="margin:0">»
- 53-62 @55 «const POSITIONED_BOX_AROUND_A_PARAGRAPH: &str = r#"<html><body style="margin:0">»
  - 57-61 «<div id="scroller" style="position:relative; width:300px; height:100px; overflow:auto; border:5px solid">»
- 64-73 @66 «const BOX_PAST_THE_VIEWPORT_EDGE: &str = r#"<html><body style="margin:0">»
  - 68-72 «<div id="box" style="box-sizing:border-box; margin-left:600px; width:300px; height:200px; overflow:auto; bo…»
- 75-87 «fn page(html: &str, incremental: bool) -> HtmlDocument {»
  - 76-83 «let mut doc = HtmlDocument::from_html(»
- 89-93 «fn node(doc: &HtmlDocument, selector: &str) -> NodeId {»
  - 90-92 «doc.query_selector(selector)»
- 95-101 @96 «fn rect(doc: &HtmlDocument, selector: &str) -> (f64, f64, f64, f64) {»
  - 97-99 «let rect = doc»
- 103-110 @104 «fn offset(doc: &HtmlDocument, selector: &str) -> (f64, f64) {»
  - 105-108 «let offset = doc»
- 112-115 «fn scroll(doc: &mut HtmlDocument, selector: &str, x: f64, y: f64) {»
- 117-123 @118 «fn fragments(doc: &HtmlDocument, selector: &str) -> Vec<(f64, f64, f64, f64)> {»
  - 119-122 «doc.node_client_rects(node(doc, selector))»
- 125-152 @126 «fn a_scrolled_box_reads_the_client_rect_it_read_unscrolled() {»
  - 130-151 «for incremental in [false, true] {»
- 154-182 @155 «fn a_scrolled_box_inside_a_scrolled_box_moves_by_the_outer_offset_alone() {»
  - 156-181 «for incremental in [false, true] {»
- 184-212 @185 «fn an_element_inside_a_scrolled_box_moves_by_the_box_offset() {»
  - 186-211 «for incremental in [false, true] {»
- 214-248 @215 «fn an_inline_element_moves_with_its_scrolling_inline_root() {»
  - 216-247 «for incremental in [false, true] {»
- 250-296 @251 «fn an_inline_element_offset_rect_ignores_its_offset_parent_scroll() {»
  - 252-258 «let pages = [»
  - 260-295 «for incremental in [false, true] {»
- 298-331 @299 «fn the_visible_region_inside_a_scrolled_box_is_its_padding_box_cut_by_the_viewport() {»
  - 300-330 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_act_obstructed.rs — added 33 line(s) in 10 range(s)
added: 12-15 · 27-29 · 359-361 · 363 · 393 · 399-402 · 413-414 · 418-420 · 422 · 424-434
- 27-29 «use session_common::{»
### tests/blitz-tests/tests/stand_act_scroll.rs — added 53 line(s) in 18 range(s)
added: 11-14 · 174-175 · 356-358 · 360-363 · 366 · 396-397 · 399-400 · 403-404 · 408 · 415 · 422-423 · 429-431 · 433
       464-465 · 469 · 472-485 · 487-493 · 498
### tests/blitz-tests/tests/stand_actionable_keys.rs — added 4 line(s) in 1 range(s)
added: 161-164
