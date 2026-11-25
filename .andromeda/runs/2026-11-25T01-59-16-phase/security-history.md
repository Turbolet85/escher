@history_item n=1 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the reserved virtual-node id form) — added the row that the id grammar reserves a fifth form `{owner}///{key}` (`VIRTUAL_KEY_SEPARATOR`), with public `virtual_node_id(owner, key)` returning `None` for an empty or `/`-bearing key. No walk calls it, so no served id holds `///` and the anchored path stays the only served id holding `//`. Ratified as a stable-id contract change by the founder's P4 fork 2. This is exactly the form (contract/stable-element-id/virtual-node-form) this chunk turns from reserved into produced. The "no served id holds `///`" clause goes false the moment a declared node is read, so the row needs restating and probably ratifying again.

@history_item n=2 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (UI graph crossing, the signal registry) — the graph answer (ids, owner ids, component, state and handler event names, closed flag words, booleans, never a value or NodeId) was ratified onto the socket, command line, MCP and kept checks. `use_graph_state`/`SignalRegistry` was added as an in-process app API with no log site, no value held and a `Debug` of counts only. This is the precedent for an in-process app-declared registry (a declaration surface) staying inert and closed until a later entry carries it onto the carriers. contract/graph-answer/nodes-states is the row "Virtual nodes in every read" later extends.

@history_item n=3 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — set the author-key rule for the `id` reader: only non-empty, `/`-free and first in document pre-order. An empty, `/`-bearing or later-duplicate value gives no key. No `id` makes two ids equal, nothing panics, and no engine id or pointer is carried. This is the same validation shape this chunk's virtual-node author key copies (non-empty, no `/`, duplicate refused). Not a widening, because it reads an already-admitted input in process.

@history_item n=4 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — stated the anchored `{key}//{segment}` tier (ratified by the founder) and the three-way uniqueness argument: a key holds no `/`, an anchored path is the only id holding `//`, a component or document path has no empty segment. It added `unkeyed_actionable` as an in-process reader and scoped "no NodeId/ElementId/ScopeId/pointer" to the id and snapshot. Trap recorded: any wire form must leave `node` behind. The uniqueness argument (contract/stable-element-id/no-collision) must be extended to cover `///` so a virtual id cannot equal any served id.

@history_item n=5 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — added a keyed list row segment `{tag}[{key}]` and the stability rule: a key is never a list index, pointer, hash, clock or process-local value, so an id reads the same across re-render, remount and a second process. A dropped NodeId reads `None` with no panic. This binds a virtual node's key (contract/stable-element-id/stability): the app's declared key must not be positional or process-local if the id is to persist.

@history_item n=6 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — stated the refusal of an empty or `/`-bearing Dioxus key (positional segment instead), with one named unit-test witness per condition. The Epoch 2 audit found the filter's `&&` could flip to `||` with no test failing. This is a precedent trap for this chunk's declaration validation: each refusal condition (empty key, `/` in key, empty name, region outside the content box, duplicate key) needs its own witness.

@history_item n=7 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the stable id leaves through AccessKit `author_id` on element nodes only (never a text run, the document root or Window). The operator ratified this as a new author-derived class on an existing crossing. Standing rule: any further consumer of the id beyond the platform accessibility API re-opens the row. Relevant to the scope's reserved premise (AccessKit nodes with no DOM behind them) and to the fork over where declared nodes enter the tree, which «Virtual nodes as elements» owns.

@history_item n=8 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the closed-field rule; the adapter crossing's third class) — the founder ruled that a new field of a closed value set inside a ratified carrier is no widening, while authored text, an open value set, a new carrier or a new exit still is (decision/closed-value-field-no-widening, decision/boundary-widening-rule). The control states were ratified as a new author-derived class on the adapter crossing. This rule decides the scope's two forks: a closed role set rides free, but a declared name is authored text and is a widening on any carrier.

@history_item n=9 marker=2026-10-12-self-description
**2026-10-12-self-description** — ratified program-fixed `'static` text crossing out, and made it a playbook rule that text from a page, a call or a screen, a new carrier or a new exit still widens. This confirms that a virtual node's app-authored name cannot ride as "program text". Crossing it is a founder fork, as the scope's names'-crossing fork states.

@history_item n=10 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the merged ARIA reads) — `aria-labelledby` names and `aria-description` text reached the adapter: a fourth content class (descriptions), the platform adapter its only exit, and the snapshot reading no description. `aria-expanded`/`aria-disabled`/`aria-level` were added as closed values. This is the precedent for authored text reaching the adapter alone with no driver carrier (decision/accessibility-adapter-content-classes, decision/merged-aria-states-closed-parse). It is the conservative option if a virtual node's name enters only the tree.

@history_item n=11 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — the tree handed to the adapter is asserted headlessly equal to the document's tree, with every `author_id` its node's element id, none empty, all digits or `0x`. What an assistive technology reads stays unread (residual). This is the existing guard any virtual node entering the tree must keep passing (the id form must not look like a NodeId).

@history_item n=12 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the tree's `focus`, a non-invertible transform takes no hit) — `build_accessibility_tree` names focus only for a built node, else the window (decision/accessibility-focus-built-nodes-only). A non-invertible transform's subtree takes no hit (decision/non-invertible-transform-no-hit). Both are engine changes for every document, ratified. Virtual nodes with no DOM behind them must not break "the tree never names a node it did not build". A region under a non-invertible transform is the hit case a later click entry inherits.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's row bounds, a group's label painted) — the select list's rows are engine-drawn sub-items. They cross only as a row's own `bounds` (no widening under the closed-field rule) and as pixels. The highlighted row crosses no structured carrier, and a group's `label` became both a tree name and a painted header, a ratified widening (decision/optgroup-options-and-label, decision/select-choice-exits). This is the nearest precedent for sub-element regions carrying authored labels: bounds are closed, while a label as a name needed the founder's ratification.

@history_item n=14 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a custom widget's attributes replayed at set-up) — behind `custom-widget`, `set_custom_widget` replays parsed attributes, page content included, to `Widget::attribute_changed`, and leaves the value's check to the widget's own code. This was ratified as an engine change. The `Widget` trait is a candidate home for the declaration (scope fork "where the declaration lives"). It is a precedent that the engine leaves widget-side validation to the widget, which this chunk must not do for declarations: a bad declaration must be refused by escher with no panic.

@history_item n=15 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** — canvas content was ratified to cross only as shot pixels on every carrier (later replaced). blitz-test-harness turns on blitz-paint's `custom-widget` feature with no package added (decision/harness-blitz-paint-custom-widget). This is the feature gate this chunk's `Widget`-side declaration compiles under.

@history_item n=16 marker=2026-11-23-canvas-placement-and-text
**2026-11-23-canvas-placement-and-text** — a custom widget's paint size, scene placement and pointer events moved to its content box (decision/custom-widget-content-box-frame), with a sub-document staying in its border box. The widget was also handed the document's font context. This fixes the frame the scope infers for a declared region: the content box, floored at 0, times scale. A region check "inside the content box" must use this frame and guard non-finite or negative sizes.

@history_item n=17 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added engine click synthesis on a custom-widget target, so an app's click and double-click handlers on the widget fire. This is a ratified engine change and the existing pointer path that «Virtual nodes as elements» (`click`) and «Circle Drawer's circles as virtual nodes» will route through. It is not this chunk's, but its frame and target are what declared regions must line up with.

@history_item n=18 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — replaced the pixels-only canvas row with decision/canvas-content-crossing: the canvas verb carries draw commands as closed words, numbers and 64-bit digests, never a glyph id, text, font bytes or family name. It also added decision/kept-check-canvas-answer, name/command/canvas, the canvas answer in no-screen-content-in-logs, and the cold-agent canvas row. The canvas crossing explicitly excludes text. A virtual node's name crossing through any canvas-adjacent carrier would contradict it unless a new ratified row says so. "Draw commands grouped by node" (owned later) extends this answer.

@history_item n=19 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** — added decision/canvas-label-lint-crossing (PROVISIONAL, delegate): a canvas label's ink box crosses in a truncated finding. decision/lint-findings-closed-content was restated as "never page text, a canvas label's text…". skrifa was taken as a driver dependency (PROVISIONAL). This is the most recent precedent for canvas-authored text: its geometry crosses but its text never does. It is also the trap that a row already "at its bound" needed a new row for a new canvas content path. The two PROVISIONAL marks are still open going into this chunk.

@history_item n=20 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part) — the `lint` verb's findings (ids, closed defect words, numbers) were ratified onto every carrier, with no log site, span or field carrying a finding. The record of ids is fed by `Session::lint`. This is the origin of decision/lint-findings-closed-content, which a virtual node's id in a finding would extend.

@history_item n=21 marker=2026-11-24-upstream-sync-ahead-of-inside-the-canvas
**2026-11-24-upstream-sync-ahead-of-inside-the-canvas** — added decision/os-drag-and-drop-untrusted and decision/dropped-file-read-unrestricted (PROVISIONAL on a delegate). These are the two rows the scope carries as CARRY cry-tjzqz3hz: never written by this chunk's code, only by its wrap's leaf pass into `.claude/docs/security-summary.md`.

@history_item n=22 marker=2026-11-24T21-15-59-wrap
**2026-11-24T21-15-59-wrap** — the founder ratified the ten drag-and-drop rows («Подтвердить все 10»), setting decision/dropped-file-read-unrestricted and decision/os-drag-and-drop-untrusted to standing. Both rows are now standing. Only their entries in `.claude/docs/security-summary.md` remain owed, as the scope's carry states.

@history_item n=23 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session socket carries calls and answers; ids, names and values leave in an answer) — the founder ratified the socket carrying an id and typed text in, and ids, names, masked values, snapshot text and diff out, never a NodeId (decision/session-socket-carries-calls, contract/driver-session-reply/no-node-id). Accessible names left the process in answers while still reaching no log. This is the carrier set a declared node's id or name would join. The scope's "no new reading on the socket, CLI, MCP or kept check" keeps it untouched.

@history_item n=24 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — ratified the MCP crossing (decision/mcp-result-crossing): ids, names, masked values and snapshot text to the client, whose onward use, a model provider included, is the client's. "The next carrier is a new question" is the standing rule if any declared-node content reaches MCP.

@history_item n=25 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — the snapshot's one text form writes an id as a quoted `id=` field and a name as a quoted string, both in `str`'s Debug form, so a field never holds a line break or bare `"` (contract/snapshot-text/id-field, contract/snapshot-text/name-field). Standing rule: the first command exposing the text is a new crossing. A virtual id holding `///` and an app-authored name must stay inside these quoted forms if they later reach the snapshot.

@history_item n=26 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — escher's sink drops every record from a target outside its allowlist at every level, so a sink-installing host logs no id and no name at any level (decision/stable-id-not-logged, decision/accessible-names-not-logged, decision/telemetry-sink-target-allowlist). Trap from the 2026-10-07-driver-session entry: a "nothing in the logs" check over a process that installs no sink passes vacuously. Any no-log claim for declared keys or names needs `host_log`/`mcp_log`-style reads.

@history_item n=27 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — `password` joined the content-named field set (twelve names; decision/escher-target-content-scrub, decision/escher-target-content-fields-withheld). decision/no-screen-content-in-logs gained a step's caused text. This is the current form of the log-scrub rows a declared name must stay out of.

@history_item n=28 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — contract/driver-session-reply/no-attribute-text gained its one exception: an author's text in a record's message or a caused item's `error` field. A declared node's name is author text that is not an attribute. Whether it rides an answer is the names'-crossing fork, which this exception does not cover.

@history_item n=29 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (Input Validation: decision/socket-carries-screen-reading) — the socket row now lists "layout and canvas readings". This is the latest form of the carried-set row any declared-node reading would have to join by an explicit amendment.

@history_item n=30 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — ratified the `layout --id` verb on every carrier (decision/session-layout-read-in-process restated; decision/kept-check-layout-answer). The select-list row gained the layout answer's placing word. `Placer::App { canvas }` (which «Virtual nodes in every read» owns) lives on this answer. This is a precedent that an in-process read became a carrier verb only by a founder fork, the path this chunk's in-process declared-nodes read keeps closed.

@history_item n=31 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — added the actions answer crossing (ids, owner ids, `keyed`, verb and cause words; decision/kept-check-actions-answer) and the one actionable rule `Readings` with `actionable_nodes` returning `{id, owner, keyed}` and no NodeId. A virtual node's `owner` (the canvas's id) and `keyed` reading are what «Virtual nodes in every read» extends. The answer is unpaged under the 1 MiB bound (`res-7cc5bi4k`).

@history_item n=32 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — the founder ratified the actions answer's `keyed` field, so decision/kept-check-actions-answer is now standing.

@history_item n=33 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage line's content and its silence in the logs) — the coverage line holds ids, owner ids, event names and counts only (decision/coverage-line-no-user-data, contract/escher-session-coverage-line/content). It was ratified at 2026-11-15T05-21-42-wrap. Handler coverage over virtual nodes is a later entry's work, but these are the closed-content rows it must keep.

@history_item n=34 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — the graph answer became paged (`part`/`parts`; contract/graph-answer/nodes-states, decision/kept-check-graph-answer). decision/kept-check-call-content gained the ctrl flag and graph parts. These are the current forms of the picked kept-check and graph rows that declared nodes would later appear in.

@history_item n=35 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — added decision/exploration-report-closed-content (closed endings, indices, calls, booleans, never a NodeId or screen text) and kept each explored path as an owner-only kept check. Exploration over virtual nodes is owned by «Virtual nodes in every read». This row is what it must hold.

@history_item n=36 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — a Dioxus document renders into a `MutationBuffer` unborrowed, then borrows once to replay and flushes mounted events after (decision/dioxus-render-into-buffer), with no panic path or log site in the recorder. The stated limit is that a task polled during render sees the previous render's document. If the declaration lives in the Dioxus renderer, it must not borrow the document during render, and a declaration read mid-render sees the prior state.

@history_item n=37 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (the driver session socket) — the session label is 1 to 32 bytes of `a-z0-9-`, checked before boot, never document text, and the one caller string a reply carries (decision/session-label-form, decision/session-label-only-caller-string). The standing note is that a command carrying an id, name, value or snapshot text reopens the crossing question.

@history_item n=38 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — added the select row: the choice's exits (form entries, event values, snapshot value, the options' tree nodes) stay inside the ratified name and control-value crossings, with no carrier, field or log site added. This is the origin of decision/select-choice-exits, a precedent for engine-modelled sub-items exiting only through already-ratified crossings.

@history_item n=39 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent pipe carried the stand's screen (ids, names, values) to the model provider. Its transcript stays raw, unprinted and gitignored, and committed copies are host-path-masked (decision/cold-agent-record-unprinted, decision/cold-agent-transcript-raw). This underlies the operator condition in the scope that no raw log with a temp or home path goes into evidence.

@history_item n=40 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — the agent-run raw cargo/libtest log `target/agent-run/run.log` is unscrubbed, never printed and gitignored (decision/agent-run-raw-log-unprinted). This bears on how this chunk's probes and evidence are captured.

@history_item n=41 marker=2026-11-18-cold-agent-live-run
**2026-11-18-cold-agent-live-run** — added decision/credential-probe-reads-valued-keys: the credential probe reads a key name carrying a value, after `apiKeySource` hit a field-name pattern. This holds decision/live-evidence-holds-no-credentials for any evidence this chunk commits.
