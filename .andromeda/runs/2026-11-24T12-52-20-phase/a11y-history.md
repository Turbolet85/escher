@history_item n=1 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added `+ decision/circle-drawer-canvas-pixels-only`: Circle Drawer's drawing area is one canvas node, role group, named Drawing area, keyed and clickable, with its circles and diameter labels reaching no DOM, snapshot, accessibility tree, UI graph or lint and leaving only as shot pixels (delegate's choice, PROVISIONAL at the time). This chunk's draw-command read takes those circles and labels out as data for the first time, so it widens the row's "only as shot pixels" clause and must amend it or rule a new crossing; guard `stand_circle_drawer_canvas`.

@history_item n=2 marker=2026-11-24T06-11-11-wrap
**2026-11-24T06-11-11-wrap** — `~ decision/circle-drawer-canvas-pixels-only` state provisional → standing: the founder ratified the canvas as one named node with circles and labels leaving only as pixels («Подтвердить (Recommended)», 2026-11-24). The row this chunk widens is now the founder's own word, so changing what leaves the canvas (glyph-run text, coordinates) needs his fork, not a delegate's.

@history_item n=3 marker=2026-11-24-upstream-sync-ahead-of-inside-the-canvas
**2026-11-24-upstream-sync-ahead-of-inside-the-canvas** — `+ absence/keyboard-drag-and-drop`: drag and drop reaches a document only as OS pointer drop input, with no keyboard or AT route. This epoch's previous entry; the chunk's boundaries exclude drag, pan and zoom and no canvas keyboard path, so the absence stands unchanged.

@history_item n=4 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (how an open list's labels leave) — new row (now `decision/option-labels-leave-only-as-pixels`): an option's and a group's label leave an open list only as painted pixels in a screenshot and the snapshot's name, with no log site, ratified by the founder as a widening. This is the precedent for authored text leaving the engine as pixels only. A draw-command dump that writes glyph-run text or glyph ids is the same kind of crossing and goes to the founder's fork.

@history_item n=5 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select; the changed set where a list stands) — `type` on any select focuses it with no click and closes any list. This is the origin of `decision/driver-type-focuses-select-no-click`, a change that added nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, errors or exits. A new verb or argument for the canvas read grows exactly those closed sets, so the row's "adds nothing" precedent and the closed-set enumerations must be checked. The same marker qualified `decision/changed-set-marks` for list open/close.

@history_item n=6 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the driver on a `multiple` select) — first stated that `type` on a select adds no verb, argument, key name, result field, cause, session error or exit to the driver's closed sets, the wording `decision/driver-type-focuses-select-no-click` keeps. These are the closed sets a new canvas verb or argument enlarges.

@history_item n=7 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — made the driver's `screenshot` one more reader of the model's bounds: an element shot by stable id is its bounds rounded outward and cropped (`contract/driver-screenshot/element-shot`), and a shot changes nothing, with no focus move, `has_changes` false and the next diff empty (`decision/screenshot-changes-nothing`). It also put a password's typed text in no painted frame (`decision/password-never-revealed`). The canvas read sits beside `screenshot` in blitz-test-harness and must meet the same changes-nothing bar. It must also carry no masked text through a scene (a password painted as bullets in blitz-paint).

@history_item n=8 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — added the `actions` verb as a read-only reader: it changes nothing (`decision/actions-verb-changes-nothing`), and its answer writes ids, owner ids, verb and cause words and booleans only, never a value, typed text, MASKED_VALUE or NodeId (`contract/actions-answer/fields`, `contract/actions-answer/causes`). It also measured the unpaged Cells answer at 106,088 bytes as a stated limit (residual `res-7cc5bi4k`), and landed the `keyed` field PROVISIONAL (`decision/keyed-wire-form`). This is the nearest precedent for a new read-only driver verb on every carrier: what it may write, its size measured against a budget, and its crossing ruled by the founder.

@history_item n=9 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/keyed-wire-form` provisional → standing on the founder's «Утвердить keyed». A new field on the carriers lands PROVISIONAL under a delegate until the founder's own word. A canvas verb's result fields on the carriers follow the same path.

@history_item n=10 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — added the read-only UI graph `DioxusDocument::graph` (`decision/ui-graph-carries-no-values`: ids, names, closed words and booleans only, never a value) and reserved the `{owner}///{key}` form (`contract/node-id/reserved-separator`, `virtual_node_id`, which the walk never calls). It built `NodeState.clickable`, so Circle Drawer reads 1 clickable. This chunk must not serve a `///` id: grouping commands by virtual node belongs to «Circle Drawer's circles as virtual nodes». A canvas dump carrying coordinates or text is a values-bearing reader, unlike the graph.

@history_item n=11 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — made the snapshot model consume the accessibility tree with no second role or name mapping (`decision/snapshot-model-single-source`, `decision/readers-of-one-snapshot-model`). A canvas command dump is not a reader of the snapshot model. Its relation to the text form, diff, MCP results, help and screenshot list must be stated, so the "nothing else" readers rule is not breached silently.

@history_item n=12 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — made `Snapshot::to_text` a reader of the model, gated by `accessibility`, and masked a file input's value (`contract/snapshot/file-input-value`). A deterministic text form of the snapshot is the precedent for the dump's line form. The masks bind any new dump that could carry painted control text.

@history_item n=13 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — added `Snapshot::diff` as a reader of the model, compared by stable id with a text change reported on its element and never as a text-run entry (`contract/snapshot-diff/entries`). It also made the changed set truthful and the tree refresh on change. This is the precedent for the chunk's "diffable" requirement, where an unchanged step reads an empty diff. The diff's `accessibility` gating is a precedent for where a canvas diff lives.

@history_item n=14 marker=2026-10-12-self-description
**2026-10-12-self-description** — added the node keys' help and `DiffNode::differs` (`contract/snapshot-diff/differs`, readings named in node-key order) as model readers with closed words only. It measured a password typed over MCP reading MASKED_VALUE, its sentinel 0 times. The new verb's `help` entry and any changed-lines naming follow this closed-word form, and the sentinel-absence check is the pattern for proving no masked text crosses.

@history_item n=15 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — added `Snapshot::text_parts` and paged an over-budget screen by `SNAPSHOT_TEXT_BUDGET` (`contract/snapshot-text/parts`), with an MCP result the text for a whole screen and a JSON line `{text, part, parts}` for a paged one (`contract/mcp-result/snapshot`), on the founder's F1. The chunk must measure the dump's size against this 10,000-byte budget, and a dump that can exceed it raises the same paging question.

@history_item n=16 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — made MCP tool results a reader of the snapshot model, on the crossing the founder ratified (2026-10-10). Whatever the new verb carries over MCP to a model provider is a new crossing and the founder's fork (scope: «authored text in glyph runs»).

@history_item n=17 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — recorded that the driver's schema defines no role or name set of its own, with `role` and `name` only field names of its result nodes (`decision/driver-schema-has-no-role-set`). A canvas verb's schema and result shape in `schema.rs` / `schema/spec.rs` must not introduce a role or name mapping of its own.

@history_item n=18 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on dioxus-native-dom unconditionally, with no `[features]` table, so the feature reaches every binary depending on it (`decision/driver-requires-accessibility`, `decision/dioxus-accessibility-feature-by-name`). Neither stand binary builds a platform adapter. A canvas verb added to the driver compiles under that feature on every carrier. Any new dependency (e.g. `anyrender_serialize`) must keep the adapter out of the stand graph.

@history_item n=19 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — first placed `escher-driver` between the stand and the Dioxus bridge with no accessibility feature, deferring the feature to the first snapshot reader. It also ratified the tree's refresh on change and the changed-set contract. This is the history behind `decision/driver-requires-accessibility` and `decision/a11y-tree-refresh-on-change`.

@history_item n=20 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — made every element of a Dioxus document carry its stable id as `author_id` (`contract/accessibility-tree/author-id`). It wired the Dioxus `accessibility` feature by name only and routed the tree through `Document::accessibility_tree`. The canvas read addresses one canvas element by that stable id.

@history_item n=21 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added the actionable-key rule and `unkeyed_actionable` (`decision/actionable-key-rule`), on the founder's word. The trap is to key an element and never satisfy the check with a `tabindex` or a role. The stand canvas is keyed and clickable, and any stand edit for the read keeps it keyed with the Tab order unmoved.

@history_item n=22 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — made `unkeyed_actionable` read 0 on Circle Drawer, its dialog open included. It pinned the click-without-role count (Circle Drawer 1). These are the stand's Circle Drawer a11y readings that the chunk's stand edits must leave unmoved.

@history_item n=23 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — made a password input read MASKED_VALUE in the snapshot (`contract/snapshot/password-value`, `decision/password-value-masked`). A canvas dump of glyph runs must not become a path by which typed or masked text leaves.

@history_item n=24 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — set the obs sink's target allowlist: `text`, `value`, `html` and `attrs` are redacted for `escher_*` targets and an outside target is dropped whole. A canvas verb's logging must write no glyph text or command payload outside this rule, keeping the "no log site" bar of the pixels-only rows.

@history_item n=25 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — added the second record class (one line per closed span of an admitted target, ratified 2026-10-09). A new driver verb gets a command span whose recorded fields fall under the same allowlist.

@history_item n=26 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set, and a freshly booted document holds a non-empty set (`decision/harness-settle-leaves-changed-set`, guard `stand_settle`). A changes-nothing check of the canvas read that uses `has_changes` must drain first.

@history_item n=27 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** — `~ decision/changed-set-marks` to its present rule (post-layout focus loss marks too; node creation, hover, active, other scrolls and layout alone do not). The canvas read runs no step and must mark nothing in this set.

@history_item n=28 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — `~ decision/changed-set-marks` (a highlight moving rows marks the set), with the snapshot's highlighted reading added from the tree. This is an earlier step of the row this chunk rests on for its changes-nothing reading.

@history_item n=29 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — `~ decision/changed-set-marks` (a scroll that closes a list marks its select). This is an earlier step of the picked row.

@history_item n=30 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** — `~ decision/a11y-tree-refresh-on-change` and `~ decision/accessibility-tree-through-document-method`: the tree is pushed in a redraw after style resolve only when the changed set is non-empty, and a poll pushes no tree. A read that changes nothing triggers no tree push.

@history_item n=31 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — made blending paint: `render_element` pushes and pops viewport-wide blend layers in blitz-paint, with no frame test (a CARRY on «Upstream sync ahead of determinism and exploration»). `text_contrast` reads None past a blend. The chunk's dump records push/pop layer commands from widget scenes, and this record says what layers paint_scene adds around them that are not the widget's.

@history_item n=32 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes the accessibility reach under `## Accessibility reach` (tier 0, SC 2.1.1 · 1.4.3 · 2.4.3, AT reading unmeasured; `decision/accessibility-reach-tier-0`), changed by hand. The AT witness is owned by residual `res-llkgqygv` (re-carried). The chunk edits `docs/capabilities.md` for a new verb and must leave the reach section true. A canvas read is no AT route (`absence/assistive-technology-reach` stands).

@history_item n=33 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — the capability statement's pin now reads the a11y leg's file count among the section's claims, and no check reads the rest. A README tool count or capability edit for the new verb is pinned by `.github/scripts/test_capability_statement.py`, while the reach claims stay hand-held.

@history_item n=34 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the stand's graph re-read) — the stand's graph is re-read at each lock move, with no `accesskit_xplat` / `accesskit_winit`. The adapter and its windowed AT witness are owned by residual `res-llkgqygv`. If the chunk takes `anyrender_serialize` or another crate into the driver or harness, the adapter probe is re-read.

@history_item n=35 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — Enter and Space activation build a synthetic Click, and a click focuses a button. This is the source of `decision/synthetic-click-at-node-centre` (a primary click at the node's centre), which the canvas chunks' click synthesis rests on. The scope keeps click synthesis unchanged.
