@history_item n=1 marker=2026-11-15-taffy-from-upstream-git
**2026-11-15-taffy-from-upstream-git** (the three row entries and the take's ratification) — ~ decision/taffy-layout (Taffy 0.15 from upstream's git source pinned to one revision, part of blitz-dom's API through its signatures, no re-export), ~ decision/no-git-dependencies, ~ decision/deferred-upstream-dependency-bumps (Taffy from #1372 on, with its reversed-scroll-origin change, waits for its own take), and the eight git-Taffy layout commits at `e90751fd` ratified for that take alone. This is the Taffy the read explains, and the scope's «taken ahead of Taffy past #1372» rests on it: a scrolled-box reading built now must be re-read at that take.

@history_item n=2 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** — ~ decision/deferred-upstream-dependency-bumps, ~ decision/no-git-dependencies (Taffy-only git exception), ~ decision/font-face-css-descriptors (`font-stretch` reaches matching). This is the sync that ran ahead of this epoch (Errors and explanation). It sets the dependency freeze the read must keep (Stylo 0.22, Dioxus 0.7.3, no new crate), and it shows that font inputs in the computed style can move sizes.

@history_item n=3 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (Taffy's tree traits on `LayoutPassState`; the bounds reader renamed; `writing-mode` opt-in) — + contract/layout/taffy-traits: the crate-private `LayoutPassState` implements Taffy's traversal and layout traits for one pass, and `BaseDocument` implements none. + contract/layout/stylo-taffy-exports: `to_taffy_style`, `TaffyStyloStyle`, `WritingMode`. Bounds go through `physical_unrounded_geometry`, which has two bodies, one with the `writing-mode` feature and one without. A read of the layout chain must work outside a layout pass, from `final_layout` and `to_taffy_style`, and must respect the writing-mode split.

@history_item n=4 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — + decision/normal-self-alignment-taffy: align/justify `normal` map to Taffy's NORMAL keyword, resolved by the aligned box's layout mode. decision/block-content-alignment-safe: the `safe` default reaches table cells. These alignment inputs are inputs the read reports «from the parent's alignment», and they are approximations the explanation must state as the engine applies them, not as CSS reads.

@history_item n=5 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the dropped-node hit skip, the replaced element's natural height and the recorded style facts) — ~ decision/node-holds-structure: `Node::out_of_flow`, written by the style traversal and read by `LayoutPassState::is_out_of_flow` alone, while `Node::is_out_of_flow` keeps its style reading. [Layout]: a replaced element's natural height follows its `aspect-ratio`. The read's «absolute or fixed positioning» kind and a replaced box's size come from these facts. Trap: the recorded flag and the style reading are two separate readers.

@history_item n=6 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the inline box's paint skip; the resolved `width`/`height`; a table box's layout damage; the key-press panic's fix; the orphan comment gone) — ~ decision/node-holds-structure gains the box-construction fact `IS_INLINE_WITH_ONLY_BLOCK_CHILDREN`. `resolved_style_value`'s width and height read `unrounded_layout()`, sub-pixel where `final_layout` rounds. + decision/anonymous-block-dealloc-detaches and decision/anonymous-block-drop-clears-layout-parent: a stale `layout_parent` no longer panics. Taffy has no `pub use` in blitz-dom. A read walking layout parents and anonymous blocks rests on these. Pick rounded or unrounded layout deliberately.

@history_item n=7 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (taffy is no re-export of blitz-dom; the bridge's select tests; an open list's exceptions to scrolling and the hit) — Taffy is in blitz-dom's API through signatures (`taffy_node_id`, `dom_node_id`) with no re-export. escher-driver has no blitz-dom or taffy dependency, so the read's layout facts must cross through dioxus-native-dom readers, with `is_masked_input` and `closed_select_of` as the precedents. The snapshot's `bounds` read `select_list_bounds` first, so an open list's row is placed by the engine's own overlay, not by Taffy. That is a placement kind the read must not misattribute.

@history_item n=8 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`::before`/`::after` in flow classification; six upstream behaviour changes ratified for the take) — + decision/pseudo-elements-vote-in-flow-classification: every box-generating pseudo votes in `classify_flow_children`, so a pseudo-only inline-block exports its line baseline. This is a layout-mode input of the parent's chain the explanation inherits. The take also set decision/upstream-behaviour-changes-ratified-per-take, the pattern any engine change met here falls under.

@history_item n=9 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (upstream's both-modes exception ended; visible_region through transforms; a non-invertible transform takes no hit) — the both-layout-modes convention lost its exception. `visible_region` reads clipping boxes through transforms, exactly under translate and scale, as a bounding box under rotation. `hit_inner` answers `None` under a non-invertible transform. The read's «any scroll or transform that moves it» clause must agree with these readers and their stated rotation limit.

@history_item n=10 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — + decision/client-rect-own-box and decision/client-rect-ignores-own-scroll: the shared bounds reader subtracts the scroll offsets of the containing-block chain above a node, never the node's own. These were ratified as a direct engine change for every document. They are the precedent for how scroll enters a box's place, which the read must explain the same way the snapshot's `bounds` read it.

@history_item n=11 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the geometry readers: the hit at `contain: paint`, `absolute_position`, the client rect under transforms; the PROVISIONAL batch ratified) — + decision/absolute-position-ancestor-scroll-only (guard `scrolled_box_absolute_position`). + decision/client-rect-under-transforms (guard `transformed_client_rect`): the border box is mapped through its own and every containing block's transform and read as an axis-aligned box. + decision/transformed-element-bounds and decision/hit-stops-at-paint-clip. These are the second bounds reader and the transform mapping the read's «where layout put it vs. where it shows» rests on.

@history_item n=12 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (the hit's own-box, one-axis and transformed-and-scrolled readings measured) — + decision/one-axis-clip-readings-kept (guard `stand_act_obstructed`): the hit stops at the whole padding box while `visible_region` narrows the clipped axis only, and neither reader moves. This is a measured divergence an explanation of placement and visibility must not paper over, and readers are not changed to agree.

@history_item n=13 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot's `bounds` come from `get_client_bounding_rect`. The snapshot derives no reading of its own, so it cannot disagree with its source. The read reports «the bounds the snapshot already reads», so it must take them from the same reader. Trap: the snapshot is a node only where the accessibility tree keeps it (contract/snapshot/membership).

@history_item n=14 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the in-process lint and blitz-dom's text extent reader; the gated part: `lint` the twelfth verb) — + contract/basedocument-readers/text-extent: a read-only `BaseDocument` reader on every document. The lint ran in process first and then became a verb on every carrier and recorded, a ratified widening of a closed defect set. This is the observed gap's own precedent: the scope's «lint names a finding but not its cause» link to this read, and the read-only engine reader is the shape the layout read's engine side can take.

@history_item n=15 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — `Harness::text_contrast` reads computed colour, size and weight, with the background image read through `resolved_style_value` and no `style` crate in the harness. It is a precedent for reading computed Stylo values from outside blitz-dom without adding a dependency, which the read's «inputs from the computed style» needs.

@history_item n=16 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the one verb table: a later entry adds its verbs to `VERBS`, never a second table. `validate` answers once per call in a fixed order (contract/validation-order/first-failure). `Command`/`Call` print ids under `Debug`, so nothing fields them. If the read is a verb, it joins this table.

@history_item n=17 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — + decision/validated-commands-only: `Session::run` is the one run entry, and nothing public runs a `Command` that did not come from `validate`. Nothing resolved outlives a call, and no `NodeId` is kept. A new read resolves its id through this path.

@history_item n=18 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — + decision/visibility-from-geometry-not-hit. A verb joins `VERBS` as a ratified widening of the validated verb set (decision/validated-verb-set-widened-by-ratification; the `scroll` precedent). An id resolves `stale` vs `not-found` through the record of seen ids. These are the read's refusal and visibility readings.

@history_item n=19 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the `command` span (target `escher_driver`, INFO) is built by hand (decision/driver-hand-built-spans), and a read records `verb` alone, or `cause` when refused. It never fields screen content or its size. This is the scope's span rule for the read.

@history_item n=20 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session socket carries a call and its answer; nine verbs in two levels, the written forms, `command_line`) — + decision/socket-carries-driver-calls. Answers are hand-written JSON keyed by the schema's words (decision/hand-written-answers, decision/answers-written-by-own-json-writer). The command line's endings and exit statuses 0 · 1 · 2 · 3 are fixed. A new read rides these carriers and writers.

@history_item n=21 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the MCP server and its `mcp` role; `serde_json` a reader only; the `tool_call` span) — + decision/mcp-stdio-crossing, + decision/serde-json-reader-only, contract/mcp-server/result-bound (1,048,576 bytes), contract/tool-call-span/tool. A read's answer reaches MCP through `tools/list` and `tools/call` with no `structuredContent`, and `serde_json` writes nothing.

@history_item n=22 marker=2026-10-12-self-description
**2026-10-12-self-description** (the self-description: `help`, `describe`, the tables; fixed program text on the ratified carriers, ratified) — + decision/fixed-program-text-on-carriers and decision/one-self-description-for-cli-and-mcp. `describe()` counts verbs and `FieldKind`s from the tables. A new verb's help, field help and limits are the program's fixed text, inside the carrier, and must state the read's measured limits (decision/measured-limits-stated-in-the-tool).

@history_item n=23 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the `screenshot` verb, `fonts-unavailable`, `Outcome::Shot` and the MCP image block) — this is the precedent of an optional-`id` read verb that resolves its id as `click` does. It adds a `FieldKind` and `Outcome` variant and states its limits in its help. It also set + decision/screenshot-in-band within the 1,048,576-byte bounds.

@history_item n=24 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — + decision/large-screen-in-parts: a screen over the budget crosses in parts with no new verb, carrier, exit, cause, session error, span field or dependency. This is the precedent if the read's answer can grow past a bound, beside the unpaged precedents.

@history_item n=25 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph crosses the ratified carriers; the `graph` verb; the reserved virtual-node id form; the unpaged graph) — `graph` is a no-argument read verb that runs no step. It added `FieldKind`s, `Command::Graph` and `Outcome::Graphed`, and was ratified as a boundary widening (decision/ui-graph-crossing). + contract/dioxus-bridge/virtual-node-id and contract/stable-element-id/virtual-node-id: the reserved `{owner}///{key}` form is spelled and produced by no tier. This is the exact precedent the scope cites for reserving «placed by the app inside canvas X» without a 0.4.0 contract change. The unpaged-answer limit is owned by `res-7cc5bi4k`.

@history_item n=26 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer crosses the ratified carriers; fourteen verbs; the actions read and the shared per-verb target check; the unpaged actions answer) — + decision/actions-answer-crossing. `VERBS` holds fourteen, ten of them instance verbs, and `FieldKind` thirteen kinds. The actions read changes nothing, makes no pump, tick or dispatch, opens no span of its own, and is recorded whole. This is the latest read-verb precedent, with the counts a new verb moves (`VERBS`, `describe`, `tools/list`, the recording's instance verbs, the README and the capability statement).

@history_item n=27 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role's kept-check files on argv; the coverage reader and its stated limits) — + decision/coverage-role-reads-named-checks and decision/coverage-reads-named-checks. Its stated limit «handlers inside a canvas not counted, deferred to 0.4.0» is the same canvas boundary the scope keeps open. A new instance verb also changes what `handler_coverage` replays.

@history_item n=28 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the coverage role's kept-check files on argv and the `escher-session` coverage role, with no PROVISIONAL left. The coverage-role rows the picked rows hold stand ratified.

@history_item n=29 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — + decision/recorded-session-checks and decision/recorded-session-kept-check: answered calls are kept at rest as an owner-only replayable check, and a masked `type` is kept as its stand-in. Wire `v3`. A new verb's answers are kept in every recording.

@history_item n=30 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer is held whole at rest, beside its digest (contract/kept-check/answer, contract/kept-check/digest). The replay compares live lines through `serde_json` and stops at the first difference (contract/check-replay/first-difference), and the two committed checks are the owner-only exception. Trap: + decision/no-screenshot-in-cross-os-byte-check, since bytes differ across OSes. A read whose numbers differ across OS or Taffy pins makes kept checks of it fragile.

@history_item n=31 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — ~ contract/kept-check/opening-record carries `format`, + contract/driver-explore/explored-verbs, + absence/exploration-on-carriers. A new instance verb must be weighed against the explorer's verb selection and the check format, and a read verb dispatches nothing to explore.

@history_item n=32 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — + decision/capability-statement (guard `test_capability_statement.py`) and contract/root-readme/mcp-tool-count: `docs/capabilities.md` and README's tool count are held to `VERBS`, `Cause`, `SESSION_ERRORS`, `NODE_FIELDS` and `EXIT_STATUSES` by the pin. A new verb or node key moves these counts, as the scope names.

@history_item n=33 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (a node's component and a held signal's readers through Dioxus public API) — + decision/owning-component-via-public-api: a node's owning component is read through Dioxus 0.7.10's public API with no fork change. The bridge's listener attributes over-report. This is the route if the answer names an owner, as for the reserved canvas case, by stable id.

@history_item n=34 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the stable id grammar (author key, component path, document path; the anchored tier came later). No `NodeId`, `ElementId`, `ScopeId` or pointer enters an id (contract/stable-element-id/no-process-local-values). The read is addressed «by its stable id», and every id in its answer, including the containing block's, must be one.

@history_item n=35 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the bridge drops a removed node; the default sheet from the config) — + decision/bridge-removal-drops-subtree and contract/stable-element-id/removed-node: a removed node no longer resolves. This history also moved Taffy to the registry, which the git pin of n=1 later reversed. The read's id resolution and a stale containing-block reference rest on this.

@history_item n=36 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — + decision/default-font-fallback (the default font for text and `ch`/`ex` metrics), ~ decision/both-layout-modes (escher's own targets run both modes; only upstream-taken targets are excepted), ~ decision/changed-set-write-sites (layout writes no changed-set entry). Sizes the read explains depend on font fallback, and the read's checks must run both modes.

@history_item n=37 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — ~ decision/stand-typography-dejavu-sans (guard `stand_bundled_font_families`) and + decision/stand-readings-same-on-every-ci-os (guard `stand_cross_os_readings`): the six served tasks read the same digests on linux, windows and macOS. A layout answer on the stand inherits this determinism and should read identically across CI OSes.

@history_item n=38 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the headless stand serves Cells and a load page; the served-page checks and the scrollbar hazard) — contract/headless-stand/boot and offline-boots: six served tasks in a pinned 800x600 viewport, with + decision/stand-load-page-in-process-url. There is a measured overlay-scrollbar hazard right after a `scroll`. The stand cases a placement read is proven on («a fixture beside it where the stand lacks a case») sit on this boot.

@history_item n=39 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the stand's pinned 800x600, scale 1.0, Light viewport (decision/stand-pinned-environment), with one bundled font and no env read. This is the environment every placement number the read reports on the stand is measured in.

@history_item n=40 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the acting verbs wait on a load, and `snapshot`, `screenshot`, `lint` (and later `graph` and `actions`) wait on nothing (contract/load-wait/verbs). A new read verb joins the no-wait list.

@history_item n=41 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — + decision/settle-never-waits-on-time-or-loads, + decision/input-helpers-do-not-settle. A settle verb on the socket is a crossing question (absence/settle-verb-on-socket). The read runs no step and settles nothing, so it reads layout as the last resolve left it.

@history_item n=42 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — + decision/document-clock and decision/one-virtual-clock: harness readers such as `screenshot`, `text_contrast` and `settle` never move the clock. A read-only layout reader keeps that rule, and a read taken mid-animation reads the layout at the harness's time.

@history_item n=43 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — + decision/per-crate-coverage-floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and decision/stand-repeat-zero-failure-budget, both legs added here. New code in the driver or bridge must hold the floors, and a new `stand_*` check enters `stand-repeat`'s zero-failure rounds.

@history_item n=44 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — + decision/no-seed-on-carriers, ratified later at 2026-11-14-remote-pages-local-files. A precedent of the widening rule applied as an absence: a value that would widen the boundary is kept off every carrier.
