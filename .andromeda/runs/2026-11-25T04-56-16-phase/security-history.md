@history_item n=1 marker=2026-11-25-virtual-nodes-declared
**2026-11-25-virtual-nodes-declared** — (Input Validation: 4 rows) `~ contract/stable-element-id/virtual-node-form` (the `{owner}///{key}` form now "produced in process only for a canvas's taken declared nodes", no served id holding `///`), `~ contract/stable-element-id/no-collision` (the anchored path the only *served* id holding `//`), `+ absence/virtual-nodes-on-carriers` (no snapshot, graph, canvas answer, accessibility tree or MCP result holds a declared node; the F3 name/key crossing left a deferred fork) and `+ decision/virtual-node-declarations-validated` (closed refusal set, index and reason alone, no panic). Both additions are PROVISIONAL on the escher overseer's delegate word. Why it matters: this chunk is the planned heir. F3 A, the founder's word, puts virtual nodes into the tree, snapshot, diff and every carrier. So the absence row has to be retired or inverted, and both id contracts have to change from "in process only, served by no carrier" to served on the stable-id exits, with `///` now held by served ids. The no-collision argument has to be restated over served virtual ids. The validated reader is the input this chunk carries out, and its provisional state stays open for the founder's batch.

@history_item n=2 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the reserved virtual-node id form) The id grammar gained the fifth form `{owner}///{key}` (`VIRTUAL_KEY_SEPARATOR`). `virtual_node_id` returns `None` for an empty or `/`-bearing key, and no walk served it. This was a stable-id contract change ratified by the founder's P4 fork 2. Why it matters: this chunk is the first to serve that form on carriers, so the ratified grammar (non-empty, `/`-free key) is the constraint the snapshot and click arms must keep.

@history_item n=3 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the snapshot's clickable reading; the graph verb) `NodeState.clickable` was added as a closed bool inside the ratified carrier: true where listeners hold `click` and the element is no disabled form control. `differs` gained `Reading::Clickable`, and `NODE_FIELDS` went to 14. `graph` became the ninth instance verb. It reads the snapshot (`DioxusDocument::graph`) and crosses ids and names on every carrier. Why it matters: F12 A has a virtual node read `clickable` false, a stated limit against this contract. Because the graph reads `DioxusDocument::snapshot`, F9 A's exclusion code in `graph.rs` is what keeps virtual nodes out of `decision/graph-answer-crossing` until «Virtual nodes in every read».

@history_item n=4 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — Under `accessibility`, `DioxusDocument::accessibility_tree` carries each element node's stable id as its AccessKit `author_id`. It is never on a TextRun, the document root or the Window, and never found by indexing the slab with a tree id. The operator ratified this crossing, with the standing rule "any further consumer of the id beyond the platform accessibility API re-opens this row". Why it matters: `decision/author-id-on-accessibility-nodes` comes from this entry. Virtual tree nodes are a new kind of node carrying an author id (and F7 A says the author-id loop must check the virtual map first), so the row's "each element node" scope has to be restated for virtual children of a canvas node.

@history_item n=5 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — The tree handed to the adapter is asserted headlessly in both layout modes. It must equal the document's own tree node for node, with every `author_id` its node's `element_id`, none empty, and none all digits or holding `0x`. What an assistive technology reads stays unmeasured, owned by a residual. Why it matters: these assertions will meet virtual tree nodes whose author id is `{owner}///{key}`, not an `element_id`. The proof has to be extended rather than broken, and the windowed AT witness for the new names stays owned by that residual.

@history_item n=6 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — The snapshot was added as an in-process reader of ids and names. Each tree id is resolved through `BaseDocument::get_node` (never an index), and an unresolved node is omitted. Trap recorded: the first wire form of the snapshot is a crossing for ids, names and values. Why it matters: this is the builder rule the premise correction rests on. An even-versioned virtual tree id decoded by `NodeId::from_u64` would alias a live DOM node, so F7 A's virtual-map-first check in the snapshot builder is what keeps this "never by index, unresolved omitted" guarantee true.

@history_item n=7 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — `Snapshot::diff` was added as a reader of ids and names through the snapshot model only, with no NodeId or AccessKit id. The platform tree was rebuilt on change, a boundary widening later ratified by the founder (2026-10-07). Why it matters: the diff will now name a virtual node added, removed or changed by its id, and it must keep holding no AccessKit id (the even-versioned virtual tree id included). Under F11 A a declaration change writes no changed-set mark, so the refresh-on-change crossing gains no new trigger.

@history_item n=8 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** — (Input Validation: 5 rows) `~ decision/accessibility-tree-refresh-on-change` (the rebuild moved to the redraw after style resolution, which reads and clears the changed set) and `+ decision/programmatic-focus-events`. Both were ratified on 2026-11-22. Why it matters: both rows are picked. Virtual nodes enter the platform tree only when that rebuild runs, and F11 A (no invalidation signal, no changed-set mark) leaves the platform adapter possibly stale on a declaration-only change. Driver reads stay current because the snapshot rebuilds the tree on every read.

@history_item n=9 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — (three state readings; the closed-field rule) The founder's rule: a new field of a closed value set inside an already ratified carrier is not a widening, while a field holding authored text, an open value set, a new carrier or a new exit still is. Why it matters: this is the source of `decision/boundary-widening-rule` and `decision/closed-value-field-no-widening`. It is the test the scope applies: a virtual node's role, bounds and state readings are closed values with no ratification needed, while its authored name and key are the widening F3 A answers.

@history_item n=10 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — (a Markup attributes row for the three control states) The adapter crossing gained a third author-derived class, control states. This was ratified as a widening of the adapter crossing by a new author-derived class, and its windowed witness is owed. Why it matters: `decision/accessibility-adapter-content-classes` grew one class at a time by ratification. Virtual node names and ids are canvas-side author text reaching the adapter, the "first author-written canvas-side text on a carrier" the founder answered.

@history_item n=11 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (the merged ARIA reads) The adapter crossing gained a fourth content class, descriptions, and its names include `aria-labelledby`'s text. This was ratified as upstream's behaviour. Why it matters: this is the current four-class list of `decision/accessibility-adapter-content-classes`. This chunk has to state whether virtual names ride the existing accessible-name class or extend the list.

@history_item n=12 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (the session socket carries calls and answers; ids, names and values leave in an answer) The trust boundary was widened: an id and typed text come in; ids, names, masked values, snapshot text and the diff go out; no NodeId crosses. The id gained its second exit, the driver's answers. An id longer than 1024 bytes is not recorded. All were ratified by the founder (2026-10-10). Why it matters: this is the source of `decision/session-socket-carries-calls`, `decision/stable-id-exits` and the id record bound. A virtual id and name ride these ratified exits, and a virtual click takes its id through the same 1-to-1024-byte `validate` path.

@history_item n=13 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — The MCP crossing was ratified: the id gained a third exit (MCP results), and names, control states and masked values gained MCP results as an exit. "The next carrier is a new question." Why it matters: this is the source of `decision/mcp-result-crossing` and the MCP leg of `decision/stable-id-exits` and `decision/accessible-name-exits`. Virtual nodes cross MCP under F3 A with no new carrier. Any carrier beyond these would re-open the question.

@history_item n=14 marker=2026-10-12-self-description
**2026-10-12-self-description** — The founder ratified that the program's own fixed `'static` text crossing on ratified carriers is inside the crossing, and made it a playbook rule: "Text from a page, a call or a screen, a new carrier or exit still widens". Why it matters: the help texts restating `click`'s and `covered`'s limits for virtual nodes (F10 B, F12 A) are fixed text inside the carriers. A virtual node's authored name is screen text, which is why F3 A needed the founder's own word.

@history_item n=15 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — The executor's id-taking verbs look the id up by a linear search of `element_ids()`, read through `get_node`, never by index, anew on every call, and an id naming nothing is refused. Why it matters: this is the origin of `decision/id-lookup-anew-each-call` and `decision/no-node-id-between-calls`. `click`'s virtual arm has to resolve `{owner}///{key}` against the screen as it reads now, keeping no handle between calls.

@history_item n=16 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — The executor refuses with nothing run: an unknown id as `stale` or `not-found` (stale when the session's record holds it), then `disabled`, `off-screen`, `covered` in that order. The snapshot and both focus readings are equal after each refusal. The session's record holds at most 4096 id texts fed from its own snapshots. Why it matters: this is the source of `decision/target-refusal-order`, `decision/unknown-id-refused-stale-or-not-found` and `decision/session-id-record-bounded`. A virtual node must meet the same order, and virtual ids now enter the record through the snapshot, which is what lets a vanished virtual node read `stale`.

@history_item n=17 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — Every call is validated against a closed table before anything runs. A refusal is a closed cause set with fixed texts holding no path, and a fault carries at most the schema's argument name. Rule: keep every refusal text fixed and free of call content. Why it matters: the virtual `click` arm and the not-found reading for other verbs (F10 B) must add no cause and must never echo the virtual id or key in a refusal (`decision/refusal-holds-no-call-input`, `decision/closed-refusal-cause-set`).

@history_item n=18 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — The cause set went from nine to eight (`time-unavailable` retired), with two causes from `validate` and six from the executor. Ratified 2026-10-18. Why it matters: this is the current closed set of eight in `decision/closed-refusal-cause-set` and `decision/refusal-closed-cause-set`. Virtual refusals reuse `stale`, `not-found`, `disabled`, `off-screen` and `covered` without growing it.

@history_item n=19 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the shared target check and the actions read) One crate-private `check_target` is called by the four id-taking arms and by `Session::actions`, no arm keeping a check of its own. `target_of` is shared by `locate` and `actions`. Why it matters: this is the source of `decision/actions-read-shares-target-check`. If the virtual click arm gets its own covered/off-screen/disabled reading outside `check_target`, the actions read and the call drift apart, even though actions stays out of scope until «Virtual nodes in every read».

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (`visible_region` through transforms, and `off-screen`'s meaning) `visible_region` reads each clipping box through its transforms: exact under translate and scale, a rotation read as a bounding box, a non-finite edge answering `None`. `off-screen`'s meaning was restated. Founder's F1. Why it matters: this is the source of `decision/visible-region-through-transforms`. A virtual node's `off-screen` is its region's centre against the canvas's `visible_region`, and its bounds are its region moved to the page by the content-box origin and transforms, so the transformed reading and its limits apply unchanged.

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the focus an inert subtree loses, and the tree's `focus`) `build_accessibility_tree` writes the tree's `focus` only where `focus_node_id` is among the built nodes, else the window. The inert focus clear fires no blur or focusout. Founder's F3/F4. Why it matters: this is the source of `decision/accessibility-focus-built-nodes-only`. The focus writer is one of the three tree-id decoders F7 A says must check the virtual map first, so an even-versioned virtual id must never be decoded as an aliased DOM node there.

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (a non-invertible transform takes no hit) In every document the whole subtree of a transform whose determinant is 0 or not finite takes no hit. Founder's F2. Why it matters: this is the source of `decision/non-invertible-transform-no-hit`. A virtual click lands as a pointer at the region's centre through `hit`, so under such a transform the canvas takes no hit and the virtual node reads `covered`.

@history_item n=23 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the `inert` read; `disabled` on form controls) An inert node leaves the tree the shell hands the adapter, and leaves the snapshot. `disabled` is read by presence on a form control alone through `ElementData::is_disabled`. Ratified 2026-10-18. Why it matters: `decision/inert-subtree` rests here, so the virtual children of a canvas under `inert` must leave the tree and snapshot with it. F8 A gives virtual nodes a declared `disabled: bool` read as the snapshot's `enabled`, a separate source from the presence reading, which the rows have to name.

@history_item n=24 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — The snapshot masks a password's value as `MASKED_VALUE`. The snapshot's `enabled` follows the `disabled` presence reading. Why it matters: this is the origin of `decision/mask-holds-on-every-carrier` and of the snapshot `enabled` source. A virtual node carries no value, and its `enabled` now comes from the declaration (F8 A), which the disabled-related rows must account for.

@history_item n=25 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — `~ decision/mask-holds-on-every-carrier` guard gained `stand_masks_on_new_answers`. Why it matters: that is the current guard set of the picked mask row. A new answer content class, virtual nodes in the snapshot and diff, should stay under it.

@history_item n=26 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — `~ decision/mask-holds-on-every-carrier` except (a caused item's message and error field hold what their author wrote) and `~ decision/socket-carries-screen-reading`. Why it matters: these give the current wording of two picked rows. `decision/socket-carries-screen-reading` lists what an answer carries and must name virtual nodes' ids and names if the reading list is restated, and a virtual click's answer carries `caused` like any acting step.

@history_item n=27 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — (Input Validation; Threat Model Summary; Logging; Data Protection) `+ contract/driver-verbs/canvas` (PROVISIONAL), `+ name/command/canvas`, `+ decision/canvas-content-crossing`, `+ decision/cold-agent-canvas-to-provider`, `~ decision/socket-carries-screen-reading` (layout and canvas readings), `~ decision/no-screen-content-in-logs` (a canvas answer), `~ contract/command-span/verb`. Why it matters: `canvas --id` on a virtual id must read `not-found` (F10 B), adding no cause, argument kind or exit as the canvas contract promises. The canvas crossing row "already at its bound" is why virtual names need their own ratified crossing, not a stretch of canvas content.

@history_item n=28 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** — `+ decision/canvas-label-lint-crossing` (a canvas label crosses only as numbers inside a truncated finding, never its text) and `~ decision/lint-findings-closed-content`. Both are PROVISIONAL on the escher overseer's delegate confirm. Why it matters: F9 A holds virtual nodes out of the lint (`lint.rs` reads the snapshot), so a virtual node's authored name must not reach a finding. Both rows' provisional state stays open.

@history_item n=29 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (the gated part: the `lint` verb's findings on every carrier) The lint verb's findings (ids, closed defect words, numbers) cross every ratified carrier, and the session's record of ids is fed by `Session::lint`. Why it matters: the lint reads the snapshot, so a virtual node in the snapshot model would reach findings and the id record through it. That is the exclusion F9 A codes in `lint.rs`, and `decision/lint-findings-crossing` must hold no virtual id.

@history_item n=30 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `~ decision/graph-answer-crossing` (a part's part and parts numbers), `~ contract/driver-verbs/graph`, `~ contract/graph-answer/nodes-states`, `~ decision/kept-check-graph-answer`. Why it matters: these give the current wording of the picked graph rows, whose carried set must stay free of virtual nodes under F9 A until «Virtual nodes in every read». CARRY `cry-qprhfz5u`'s graph half stands on them.

@history_item n=31 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — `+ decision/exploration-report-closed-content`, and `~ contract/kept-check-file/medium` (each explored path its own owner-only check). Why it matters: exploration's digest reads the snapshot (`explore.rs:192`), so F9 A's exclusion is what keeps virtual nodes out of the exploration report and its kept checks.

@history_item n=32 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the coverage role; the reached record and `handler_coverage` as readers of the stable id) A kept check named on the command line is read as untrusted, and `handler_coverage` is a further reader of the id crossing only on the coverage line. Ratified 2026-11-15. Why it matters: these are the origin of `decision/coverage-check-files-untrusted`. Handler coverage is out of this chunk («Virtual nodes in every read»), so a kept check holding a virtual click step should not make the coverage line name a virtual id.

@history_item n=33 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — The kept check at rest: an owner-only 0700/0600 directory, a strict bounded reader (`Check::from_bytes`), and each step re-validated. The id's exits gained a fourth, a kept check's call targets. Ratified. Why it matters: this is the source of `decision/recording-directory-owner-only`, `decision/recorded-check-strict-reader` and the kept-check rows. A virtual click's target id `{owner}///{key}` is now kept at rest and re-validated on replay.

@history_item n=34 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — Each kept answer is kept whole at rest (ids, names, values, snapshot text, diffs). Replay runs each step through `Session::run`, nothing on another app's session. Two committed stand-Counter checks are the one exception to owner-only. Ratified. Why it matters: this is the source of `decision/kept-check-answer-whole`, `decision/replay-through-session-run` and `decision/kept-check-owner-only`. Virtual names and ids in snapshots and diffs now sit at rest in kept checks, the "kept checks at rest" exit F3 A lists, and they must not enter the committed checks.

@history_item n=35 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — The cold-agent pipe carries the stand's screen to the model provider, and the agent-written check is validated before replay on fresh servers. A boundary widening ratified by the founder. Why it matters: this is the origin of `decision/cold-agent-screen-to-provider` and `decision/cold-agent-check-validation`. F3 A names a cold-agent live run's model provider as an exit for virtual ids and names.

@history_item n=36 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — `~ decision/cold-agent-screen-to-provider` (adds acting answers' caused text) and `~ decision/cold-agent-events-metadata-only`. Why it matters: these give the current rule of the picked provider row. Virtual nodes in snapshot texts and diffs reach the provider inside its existing "snapshot texts and diffs" clause, which may need naming as the new canvas-side text.

@history_item n=37 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — The carried sets (socket, MCP result, command line stdout, kept check) were reconciled. The committed statement `docs/capabilities.md` states the MCP crossing's carried set, adding no carrier. Why it matters: this is the origin of `decision/capability-statement-mcp-crossing`. The scope lists `docs/capabilities.md` as a surface F3 A reaches, so its carried-onward set and stated limits (only `click` resolves a virtual id; `covered` reads the centre; `clickable` false) must be restated.

@history_item n=38 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (what else of the list crosses the carriers; `covered`'s closed-option clause) An open list's row reads its own `bounds` in the snapshot and the diff as a closed value, no widening. `covered`'s meaning gained the closed-option clause, the program's own fixed text. Why it matters: this is the precedent for a non-element-like reading entering the snapshot with its own bounds inside the ratified carriers. `covered`'s fixed meaning text is where F12 A's centre-only limit for a virtual node would be stated.

@history_item n=39 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — `~ decision/snapshot-state-readings` (a highlighted reading as a closed bool) with `+ contract/snapshot-text/highlighted` and `+ contract/diff-node/highlighted`. Why it matters: these give the current wording of the picked state-readings row and the latest precedent for a closed state field joining the snapshot without ratification. A virtual node's state readings ride the same rule.

@history_item n=40 marker=2026-11-23-canvas-placement-and-text
**2026-11-23-canvas-placement-and-text** — `+ decision/custom-widget-content-box-frame` (paint size, scene placement and pointer frame in the content box) and `+ decision/custom-widget-document-font-context`. Why it matters: a virtual node's region is in content-box coordinates. The virtual click's pointer at the region's centre is heard by the widget in this frame, and the virtual node's bounds are the region moved by the content-box origin.

@history_item n=41 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — `+ decision/custom-widget-click-synthesis`: after a custom widget hears a pointer event, the engine synthesizes the UA click on it. Why it matters: a driver `click` on a virtual node lands as a pointer event on the canvas, so this synthesis is the path that delivers it. Which virtual node was meant is the widget's own reading of the point.

@history_item n=42 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (a custom widget's attributes replayed at set-up) Behind `custom-widget`, `set_custom_widget` replays each existing attribute to `Widget::attribute_changed`, the value's check staying the widget's own. Ratified as an engine change. Why it matters: under F2 A the declaration stays on the engine's `Widget` trait, so app-authored widget input, now including declared names and the `disabled` bool, is validated by its reader, not by the engine.

@history_item n=43 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `+ contract/driver-verbs/layout` and `~ decision/session-layout-read-in-process` (through the layout verb on any carrier, resolving its id through the run path's resolution). Why it matters: under F10 B, `layout --id` on a virtual id must read `not-found` through the shared `resolve`, keeping these rows' promise that layout answers ids, closed words and numbers only.

@history_item n=44 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot` takes an optional `id` looked up as a `click`'s is, and a crop of no pixel is `off-screen`. Ratified as a boundary widening. Why it matters: `screenshot --id` shares `click`'s resolution, so F10 B's virtual arm has to be `click`'s alone, with `screenshot --id` on a virtual id refused `not-found`.

@history_item n=45 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — The HTML `id` is an author key only when non-empty, `/`-free and first in pre-order, so no `id` makes two ids equal, and no path panics. Why it matters: this is the origin of `contract/stable-element-id/author-key`. A virtual key's reader mirrors its refusals (empty key, key holding a slash, later duplicate), and the served virtual id must not collide with an author-keyed id.

@history_item n=46 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — The anchored tier `{key}//{segment}` was ratified, with a three-way uniqueness argument (a key holds no `/`, only an anchored path holds `//`, a component or document path has no empty segment). Trap: a wire form must leave `node` behind. Why it matters: `decision/anchored-id-tier` and `contract/stable-element-id/no-collision` rest on this argument. Serving `{owner}///{key}` adds a fourth served shape holding `//`, so the argument has to be re-proved over served ids.

@history_item n=47 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — A sink-installing host logs no id and no name at any level: records from outside targets are dropped whole, and needles read 0 at debug and trace. Why it matters: this is the source of `decision/stable-id-not-logged`, `decision/accessible-names-not-logged` and `decision/telemetry-sink-target-allowlist`. Virtual ids and names must reach no log, proved by the same needle checks (`host_log`, `mcp_log`).

@history_item n=48 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — A diff's content never reaches a log, while its three sizes may be recorded on the command span. Standing rule: a field holding any node, id, name or value of a diff is a new crossing question. Ratified. Why it matters: this is the source of `decision/diff-counts-only-in-logs` and the span's `added`/`removed`/`changed` contracts. Virtual nodes counted in a diff may appear in those counts, but never as ids or names in a span.
