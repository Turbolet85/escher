@history_item n=1 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** (Established Decisions: 2 rows) — added `decision/canvas-crosses-as-pixels`: a canvas's app-painted content crosses the driver's carriers only as pixels in a screenshot, with no new carrier, verb, field, exit, cause, session error, log field or span, ratified by the founder («Ратифицировать для всех каналов»). It also added `decision/harness-paints-custom-widgets`, which names blitz-paint's custom-widget feature in blitz-test-harness. Why it matters: a draw-command verb carries canvas content as data, so it directly reverses this decision. The decision has to be amended or replaced as a boundary widening on the founder's own word, which is the carrier fork the scope names. The harness's feature edge is what makes a widget's `Scene` reachable from a harness read beside `screenshot`.

@history_item n=2 marker=2026-11-23-canvas-placement-and-text
**2026-11-23-canvas-placement-and-text** (Established Decisions and Standard Contracts) — added `decision/custom-widget-content-box-paint`, now ratified: the paint size is the content box times the scale, floored at 0, and the scene is appended at the content origin inside the element's transform, unclipped, guarded by `stand_canvas_placement`. It also added `contract/nodes-widgets/widget-fonts` (`Widget::fonts` gets the document's shared Parley font context before each paint) and `contract/base-document/font-ctx`. Why it matters: the dump's coordinates and the canvas size it reports come from this placement rule. A glyph run's font data comes from the document's shared font context, which bears on the premise about referencing fonts by digest or index and on the authored-text fork.

@history_item n=3 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** (Existing Scopes and Occupied Resources) — gave `decision/no-screenshot-in-cross-os-byte-check` a canvas-only exception: one committed digest table holds a canvas-only shot byte-equal on linux x86_64, windows x86_64 and macOS aarch64, while a shot holding document text stays out. It also restated `decision/committed-stand-checks`' reason as "a shot holding document text differs by OS". This rests on the founder's F1 «A: требовать побайтовое равенство». Why it matters: it is the precedent and the measured base for the chunk's claim of a hardware-independent dump across the three CI hosts. The same text-versus-canvas split is the premise to measure for glyph positions (the WATCH on `stand_canvas_cross_os`).

@history_item n=4 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** (Established Decisions: 3 rows) — added `decision/widget-click-synthesis` and `decision/widget-click-point-content-box` (read a click point against the content box, not the client rect). It also added to `decision/click-focuses-button` the exception that a custom-widget click runs no click walk. Why it matters: these rules shape the steps whose before/after dumps must diff, for example adding circles on the stand's one canvas. The boundaries say click synthesis stands as built.

@history_item n=5 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** (Standard Contracts: 2 rows) — changed `contract/headless-stand/boot` to add Circle Drawer's boot, which runs in process only and is served by no escher-session role («A: не сейчас»). It also added `decision/canvas-placement-at-click-centre`. Why it matters: this is the premise that a verb on the command line, MCP or a kept check reaches no stand canvas until Circle Drawer is served or a fixture serves one. The carrier fork depends on it. The size measurement of Circle Drawer's canvas also runs on this in-process boot.

@history_item n=6 marker=2026-11-24T06-11-11-wrap
**2026-11-24T06-11-11-wrap** — `decision/canvas-placement-at-click-centre` went from provisional to standing on the founder's «Подтвердить». Why it matters: the picked row is now ratified, and it is a measured limit stated in the tool's help. A canvas verb's help and the capability statement must keep it consistent.

@history_item n=7 marker=2026-11-24-upstream-sync-ahead-of-inside-the-canvas
**2026-11-24-upstream-sync-ahead-of-inside-the-canvas** — the last sync before this chunk:
- `decision/taffy-layout` now reads Taffy 0.16 from crates.io, and the Taffy exception was dropped from `decision/no-git-dependencies`.
- The rule in `decision/one-axis-clip-readings-kept` now covers paint as well.
- Added `contract/nodes-widgets/widget-drop-acceptance` and the OS drop input, both PROVISIONAL.
- `decision/default-action-event-order` gained a clause.

Why it matters: the `Widget` trait this chunk reads (`paint -> Scene`) changed again with drop acceptance, which is still provisional, and the merge surface of blitz-paint's widget paint path moved. No git dependency may come back with a new crate.

@history_item n=8 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** (Stack and Technologies) — changed `decision/anyrender-rendering-abstraction` to the AnyRender 0.15 set, where vello-gpu replaced vello-hybrid. Two skrifa versions now stand (0.44 with parley, 0.49 with the renderers), under `decision/skrifa-matches-parley`. Why it matters: the widget `Scene`, its `RenderCommand`s and `GlyphRunCommand` are AnyRender 0.15 types, and the `anyrender_serialize` option belongs to this set. A glyph run's font, resolved through skrifa, sits on this version split.

@history_item n=9 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the `screenshot` verb, `fonts-unavailable`, `Outcome::Shot` and the MCP image block) — added the eleventh verb. It takes one optional `id`, dispatches nothing, is no step and changes nothing. A `fonts-unavailable` refusal is checked before anything is painted, and the shot's line, its base64 and the MCP image block are hand-written with no base64 crate. Why it matters: this is the closest precedent for a no-step canvas read named by id: how it resolves an id, refuses `off-screen` and `fonts-unavailable`, writes its answer by hand, and records only `verb` on the `command` span.

@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the harness's screenshot and the image in band) — added `Harness::screenshot` and `screenshot_rect`, which paint through blitz-paint into `VelloCpuImageRenderer` and composite over white. This gave the harness its `blitz-paint`, `anyrender` and `anyrender_vello_cpu` dependencies. The image in band was ratified as the founder's fork 1. Why it matters: the scope wants "a read of one canvas's scene, beside `screenshot`" in `packages/blitz-test-harness/src/`. The harness already reaches blitz-paint and anyrender here, and the screenshot's `FontsUnavailable` and `EmptyRect` errors are the pattern a scene read would follow.

@history_item n=11 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph crosses the ratified carriers, and the `graph` verb) — added the thirteenth verb as a ratified widening. The changes ran through `VERBS`, `FieldKind`, `describe()` counts, `Command` and `Outcome`, the hand-written answer form, `tools/list`, kept-check verbs, the README tool count and the cold-agent tool list. The `command` span records `verb` alone. Why it matters: it lists every site a new verb must touch, and it records a new content class crossing the carriers only on the founder's own word.

@history_item n=12 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (fourteen verbs, and the actions read with its shared per-verb target check) — added `actions`. `contract/actions-verb/target-verbs` covers each instance verb that takes an id argument, and `check_target` is the single per-verb target check shared by every id-taking arm. Why it matters: trap — if the canvas read takes an `id`, the actions answer and its acceptance check automatically list it for every node. The read must go through the shared `check_target`, and its no-cause or cause behaviour must be stated, as `layout` was.

@history_item n=13 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — added `contract/driver-session/layout` as an in-process read that opens no span, and narrowed `decision/driver-spans-only` to "per call its run path runs, an in-process read outside the run path opening none". It also landed `decision/layout-read-in-process-only` as PROVISIONAL, on the delegate's answer, while the founder's carrier word was owed. Why it matters: it is the precedent for building a read in process first and leaving its carrier crossing to the founder's fork. It also states the span rule a canvas read must meet in either form.

@history_item n=14 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Standard Contracts, all four parts) — added the fifteenth verb `layout`. It changed `contract/verb-table/order`, `driver-session/command-schema`, `mcp-server/tools-list`, `kept-check/kept-verbs` and `kept-check-reader/strict-read` (eleven instance verbs), and `actions-verb/target-verbs` (`layout` fifth, with no cause). `driver-explore/explored-verbs` now leaves out a verb whose result holds an image or a layout-reading field kind. `decision/layout-read-in-process-only` was replaced by `decision/layout-answer-crossing`, and closed-word answer-JSON rows with fallback words were added. Why it matters: it is the most recent full template for a sixteenth verb, including its order, every count site, the explore rule that selects by field kind (a draw-command field kind needs a decision there), and closed-set fallback words for non-exhaustive engine enums such as `RenderCommand`.

@history_item n=15 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Established Decisions: 2 rows) — widened the closed list in `decision/no-node-id-across-session` to "… layout readings, closed words, booleans, counts and numbers". It also widened `decision/select-list-crossing`'s "only as pixels" to admit the layout placing words. Why it matters: a draw-command dump adds a new class to the closed list of what crosses a session, so that list must name it. The select-list precedent shows that an "only as pixels" crossing is narrowed by an explicit, ratified amendment and never silently.

@history_item n=16 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — added `absence/component-alone-on-carriers` and put the one-component boot in `contract/headless-stand/boot`. Its delegate-confirmed cards were then ratified. Why it matters: like a fixture state, a stand surface reached in process only is recorded as a stated absence on the carriers. If the canvas verb stays in process or Circle Drawer stays unserved, the same absence form applies.

@history_item n=17 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — added `absence/fixture-state-on-carriers`, `contract/headless-stand/start-in` and `contract/driver-session/reach`. Why it matters: the scope's premise says a canvas reaches a carrier only once it is served "or a fixture serves one". This row says a fixture state is on no carrier, so a fixture route needs its own ratified widening.

@history_item n=18 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Standard Contracts, both parts) — moved the served-task set from six to seven, adding `order`. The changes reached `decision/stand-readings-same-on-every-ci-os`, `contract/escher-session-cli/usage`, `escher-session/invocation`, `escher-session/mcp`, `headless-stand/boot` and the cold-agent pass bar. Why it matters: serving Circle Drawer on the carriers would follow this path, through the served-task table, every count site, the cross-OS digest table and the cold-agent prompt and verdict. It is a scope question for the carrier fork.

@history_item n=19 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — restated `decision/stand-typography-dejavu-sans`: one bundled DejaVu Sans, with every generic family and any unavailable family falling back to it, guarded by `stand_bundled_font_families`. It also added `decision/stand-readings-same-on-every-ci-os`, so served answer digests match on the three CI OSes, compared with no screenshot. Why it matters: a dump's glyph runs are shaped from this one font. Its cross-host determinism claim extends the cross-OS digest guarantee, which until now covers answers but not pixels or scenes.

@history_item n=20 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — keeps each answer whole at rest beside its digest, masked as on every carrier, with a shot kept without its pixels. Replay compares the live line against the kept answer. It also limited `stand_screenshot_repeat`'s byte identity to one host, so a check compared across OSes holds no `screenshot` step. Why it matters: a canvas verb's answer would be kept whole and replayed byte for byte. Font and image data must be heavy-by-reference and deterministic, or it would carry the same cross-OS hazard and the same at-rest content as a shot's pixels.

@history_item n=21 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — added `contract/driver-explore/explored-verbs` (target verbs, less those with an image-kind result field, selected by kind and never by name). It changed `contract/kept-check/opening-record` to carry `format` / `CHECK_FORMAT` 1, and added `absence/exploration-on-carriers`. Why it matters: a new id-taking verb would join exploration unless its result field kind is excluded. A kept-check shape change would raise the questions the `CHECK_FORMAT` rule answers.

@history_item n=22 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — built the single schema table. Its standing rule: a later entry adds its verbs to `VERBS`, never to a second table, and `Command`, `Call` and `ArgValue` print ids and text under `Debug`, so nothing fields them. Why it matters: the sixteenth verb, or an argument on an existing verb, goes into the one `VERBS` table in `schema/spec.rs`. This rule stands behind `decision/validated-verb-set-widened-by-ratification`.

@history_item n=23 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (`serde_json` as a reader only, and the `tool_call` span) — took `serde_json` as a reader only, keeping the written forms hand-written. It added the `tool_call` span (`tool` and `ending` only) under the standing rule that no span field holds what a call supplied or a screen reads. Why it matters: the premise about `anyrender_serialize` and serde derives touches this rule. If the dump is written through serde, it breaks `decision/serde-json-reader-only` and `decision/answers-written-by-own-json-writer` unless the founder ratifies that.

@history_item n=24 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `help` and `describe()`, served from the fixed table text of the verb, node-key and cause tables. That program text on the ratified carriers was ratified. Why it matters: the new verb's help, result fields and stated limits ship as fixed table text. `describe()`'s counts and the MCP tool descriptions move with them.

@history_item n=25 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — added `docs/capabilities.md` and the capability-statement pin (`CapabilityStatementTest` over `VERBS`, causes, `SESSION_ERRORS`, `NODE_FIELDS` and `EXIT_STATUSES`). It also added README's count of "the {word} driver commands as tools", which the pin reads. Why it matters: a new verb must update the statement's names and counts and the README's tool word, or `.github/scripts/test_capability_statement.py` goes red. The scope names these files.

@history_item n=26 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — restated `decision/capability-statement` so the pin also holds the a11y leg's file count and stand-repeat's named-target count against the CI leg runner. Why it matters: a new stand check named in `stand-repeat` or the a11y leg moves a count the capability statement is pinned to.

@history_item n=27 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — added `SNAPSHOT_TEXT_BUDGET` = 10,000 bytes, a ceiling callers compare against and `to_text` never enforces. It also made the snapshot text the screen's one serialized form. Why it matters: the scope measures the dump's size at each level against this budget, as `decision/snapshot-text-budget` states (the agent client's token limits).

@history_item n=28 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a screen over the budget crosses in parts through the `part` argument (`ArgKind::Part`, 1 to 1000), the empty part past the last, and the paged JSON line, with no new verb or carrier. This is ratified. Why it matters: if a full-level canvas dump can exceed the snapshot budget, paging by `part` is the ratified precedent.

@history_item n=29 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (Established Decisions and Standard Contracts) — extended `decision/large-screen-in-parts` and the `graph` verb so the UI graph pages at the snapshot budget, with the states in part 1 alone, retiring the unpaged-graph limit. Why it matters: it is the latest precedent for bringing a second answer kind under the budget-paged form, including `contract/argument-kinds/part` widened to name the new reader. A canvas dump over budget would follow it.

@history_item n=30 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `lint` as the twelfth verb, on every carrier and recorded) — the lint first shipped in process. The founder's «Новая команда lint» then made it a verb on every carrier and recorded, with no new carrier, exit, cause, field or dependency, and `describe` counts were re-measured. Why it matters: «Canvas lint» is the next entry and builds on this chunk's read. This is the precedent for moving a read from in process to a verb, and for keeping the lint out of this chunk.

@history_item n=31 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage reader `handler_coverage` and its stated limits) — records the limit that handlers inside a canvas are not counted, deferred to 0.4.0. Why it matters: it is an owned canvas-related limit of this version that later canvas entries (virtual nodes and grouping) may resolve. This chunk does not, and its boundaries keep virtual nodes out.

@history_item n=32 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's paint, and the list on the carriers) — `paint_scene` paints an open select list last. The carve-out from "paint assumes resolved layout" (row text laid out at paint) became the except clause of `decision/paint-to-paint-scene`. The list's option text crossing as painted pixels in a shot was named a boundary widening and ratified. Why it matters: it is the precedent for engine-painted text reaching the carriers and for paint-time exceptions to `paint_scene`. Any change to `paint_scene` or `build_custom_widget_scenes` to keep a widget's `Scene` for a read must respect `contract/paint/paint-scene`, where a shell reaches painting only through `paint_scene`.

@history_item n=33 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`CustomWidget`'s attribute replay, and blending painted) — `ElementData::set_custom_widget` replays existing attributes to `Widget::attribute_changed`. `render_element` pushes a viewport-wide blend layer for `mix-blend-mode`. Why it matters: these are prior changes to the `Widget` trait's lifecycle and to blitz-paint's render path, around where the widget scene is built and appended. A read of the recorded `Scene` must not depend on paint-order layers the document adds.

@history_item n=34 marker=2026-11-22-epoch-2-corrective
**2026-11-22-epoch-2-corrective** — added `decision/fork-additions-in-own-functions`: a fork addition to an upstream function over the cognitive-complexity ceiling goes into a function of escher's own, with one call in upstream's body. Why it matters: keeping a widget's `Scene` past `paint_scene` touches upstream's `build_custom_widget_scenes` and `process_custom_widget_node` in `packages/blitz-paint/src/lib.rs`, so the addition belongs in escher's own helper.

@history_item n=35 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (Occupied Resources) — widened `decision/cold-agent-run-carries-stand-screen` to include graph and actions answers sent to the model provider, on the founder's «Утвердить graph и actions». Why it matters: what a canvas dump may carry to a model provider is the founder's fork («authored text in glyph runs»). Each new answer kind that reaches the provider on the live run has needed his own word in this row.

@history_item n=36 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Occupied Resources) — widened the same row to the acting answers' caused text, the order page's authored messages included, on «В 0.3.0, до прогона». Why it matters: it is the precedent for authored text crossing to the model provider, which is what writing a glyph run's text (or glyph ids that encode it) would do.

@history_item n=37 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Established Decisions, and the first part of Standard Contracts) — gave `decision/no-echo-of-typed-input` an exception (caused carries an app's own text as written). It added "a step's caused text" to `decision/no-node-id-across-session`, and stated in `decision/driver-library-side-effect-free` the exception for the capture's panic hook. Why it matters: if a dump writes glyph-run text, it may echo typed text an app paints on its canvas. That needs the same explicit exception and an extended closed list of what crosses.

@history_item n=38 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — added `stand_masks_on_new_answers` as the guard of `decision/masked-input-values`: masked password and file values stay masked on the graph, actions and layout answers, kept checks, explored checks and the MCP role. Why it matters: each new answer kind has been added to this masking guard, so a canvas dump answer should join it.

@history_item n=39 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Driver session and CLIs) — closed `CAUSES` at eight and put a shot's fade on the session's clock, so two shots of one state with no `advance` between them read the same pixels. Why it matters: the dump's determinism leans on the same clock, and the dump must add no refusal cause (`decision/refusal-causes-closed-at-eight`). It reuses existing causes such as `not-found`, `stale` and `fonts-unavailable`.

@history_item n=40 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — added the `command` span, a fixed field set with the standing rule that no screen text, image, argument or outcome is ever fielded. The rule was later extended so each new read verb records `verb` alone. Why it matters: a canvas verb's span records `verb` (and `cause` when refused) and never a command, count or text of the dump (`contract/command-span/never-fielded`, `decision/driver-logs-no-content`).

@history_item n=41 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set per-crate coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %), the zero-failure `stand-repeat` leg, and the workspace audit. Why it matters: the new verb's code in escher-driver and the harness read must hold these floors under `decision/ci-quality-gates`. Its stand checks run in `stand-repeat` by prefix.

@history_item n=42 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Established Decisions: 2 rows) — restated `decision/driver-library-side-effect-free` and `decision/driver-spans-only` as "installs no global subscriber", with the scoped capture default set only around an acting step. Why it matters: a no-step canvas read opens no capture, and the driver library's side-effect rules stand as written for it.
