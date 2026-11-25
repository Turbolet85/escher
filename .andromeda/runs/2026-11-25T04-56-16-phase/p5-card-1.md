# P5 review — 2026-11-25-virtual-nodes-as-elements «Virtual nodes as elements» (v040-11)

**The plan to read:** `.andromeda/cache/docs/2026-11-25-virtual-nodes-as-elements/plan.md` (rev 2). No plan stands in
a chunk's folder. Writing in that file changes no record until a `put`. A diff of the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-25-virtual-nodes-as-elements`.

**What it rests on.** The owed row read was run again whole on the corrected scope (rev 4, `render --repin`). Its
result: 7 of 7 tasks, 117 of 117 picker draws and 7 of 7 history agents passed first try; 841 rows picked of 3,637;
no master at `picked 0`. The answers are `inputs#I2`. F3 A, F11 A, and F2 A with F8 A are the founder's own words.
F7 A, F9 A, F10 B and F12 A are the escher overseer's, so a row resting on them alone lands PROVISIONAL.

## Goal
A canvas's declared virtual nodes become screen nodes.
- **The tree.** They are children of the canvas's tree node, with even-versioned tree ids (F7 A).
- **The snapshot and diff.** Each is read with id, role, name, declared `enabled` (F8 A), `clickable` false (F12 A)
  and bounds. The bounds are the region moved by the canvas's content-box origin.
- **`click`.** It takes a virtual id and acts at the region's centre on the canvas, settled with the diff.
- **The refusals.** A virtual id is refused `disabled` · `off-screen` · `covered` · `not-found` · `stale` in the one
  order, through the one target check.

The name and id ride today's exits (F3 A, the founder's): the adapter, the snapshot text and diff on the socket, the
command line and MCP, kept checks, and the cold-agent provider. No carrier, cause, exit, log field or span is added.

## Top acceptance criteria
1. The tree holds each virtual node under its canvas: author id, role, name, disabled state, an even high half equal to
   no DOM id. A hidden canvas holds none.
2. The snapshot reads each one, and its answer line holds the id and the name. This inverts the snapshot half of CARRY
   `cry-qprhfz5u`.
3. `click` on one is accepted, and its diff names the node it changed. The canvas reads focused.
4. Five refusals, each in the one order, with nothing run.
5. `type`, `scroll`, `screenshot`, `layout` and `canvas` read a virtual id `not-found` (F10 B). The graph, actions and
   lint hold none (F9 A). The graph half of `cry-qprhfz5u` stands, controlled by V1.
6. MCP in process carries the id and name. A kept check holds the virtual click and replays.
7. Under escher's sink no log line holds a virtual id or name (an extended `stand_act_spans`).
8. Controls V1–V8 each read red with their edit removed. The operator pass's CI read is green.

## Touchpoints
- New: `tests/blitz-tests/tests/virtual_nodes_as_elements.rs`.
- Modified (16 files):
  - blitz-dom: `custom_widget.rs` (`disabled`, F8 A).
  - dioxus-native-dom: `virtual_node.rs`, `dioxus_document.rs`, `snapshot.rs`, `graph.rs`, `element_id.rs`.
  - escher-driver: `execute.rs`, `lint.rs`, `schema.rs`, `refusal.rs`. The last two carry the stated limits in the
    tool's own text.
  - tests: `virtual_nodes_declared.rs`, `fixtures/mod.rs`, `stand_act_spans.rs`.
  - CI: `ci-leg.sh` and `test_ci_workflows.py`. The new check joins the a11y leg, which goes from nineteen files to
    twenty.
  - docs: `docs/capabilities.md`, by hand.
- Unchanged by lean: `explore.rs`. Its candidates come from `actions()` → the graph.

## Test commands (14 entries)
- **The probe.** No manifest or lock changed from `739132d5f`. It reads green, and its scratch-clone control reads red
  on a manifest edit (`control-probes.md`).
- **build** and the two unit entries. dioxus-native-dom is seen green. escher-driver is baselined green at 140 passed.
- **The new target**, baselined red: no target named `virtual_nodes_as_elements` yet.
- **`virtual_nodes_declared`**, seen green.
- **Seventeen companion blitz-tests targets**, baselined green at 106 passed.
- **seven_guis** `cli_commands`, `mcp_server`, `host_log` and `mcp_log`, baselined green at 42 passed.
- **The `a11y` leg**, baselined green: 19 result lines, 190 passed.
- **`fast`** and **`doc`**, seen green.
- **Operator pass:** hygiene, push, then `ci.py conclusion --sha HEAD --wait 2700`.
- **Controls V1–V8** are implement's one-shot `gate.py control` runs, each entering the evidence with its red
  reading. No plan step makes or deletes a worktree, `target/` or a build directory.

## Capabilities: concretization preview
None claimed. v040-11's acceptance names the stand's Circle Drawer circles, every read and keyboard reach, each carried
by a later markerless entry. After the word, one dated note goes to `matrix#v040-11`
(`.andromeda/runs/2026-11-25T04-56-16-phase/note-v040-11.md`) saying what this chunk proves. The cap stays pooled.

## Expected amendments (wrap), by who answered
- **F3 A, the founder's own (`rul-izni57qv`): the wrap halts once, as the answer card.** The rows:
  - security-plan · decision/accessible-name-exits, decision/stable-id-exits, decision/author-id-on-accessibility-nodes;
  - security-plan · the virtual-node-form and no-collision contracts;
  - security-plan · absence/virtual-nodes-on-carriers, narrowed;
  - security-plan · kept-check, socket and cold-agent provider rows;
  - architecture · the id, snapshot, author-id, click and refusal contracts.
- **F8 A and F11 A, the founder's.**
  - contract/nodes-widgets/virtual-nodes gains `disabled`.
  - decision/virtual-node-declaration-marks-nothing is re-worded. Its PROVISIONAL mark, and F2 A's, stay for the
    epoch-boundary batch (`rul-3ndrhilc`): this is not Epoch 4's last chunk.
- **F9 A, F10 B and F12 A, a delegate's: PROVISIONAL.** The graph and lint exceptions, `not-found` on the other verbs,
  `covered` DOM-only, `clickable` false. The a11y-plan, test-plan and obs-plan rows are listed in the plan's notes.

## Checks
- **Mechanical: all pass.**
  - Sections are present and the modify paths exist.
  - The fence parses (14 entries). planlint reads 0 hits. Its six «unchanged» claims each read true: each names a site
    a step reads, never edits.
  - No placeholder. Inputs: 3, none drifted, none uncited.
  - Research lists parsed (15). Graph trace present. No live leg. Step references resolve.
- **Size: WARN.** About 465 lines of prose outside the fence, against about 150–400. The breadth is the founder's
  F3 A, which reaches the carriers, the a11y leg and the capability statement.
- **Validation-1: aligned** with `doc:2026-11-25-virtual-nodes-as-elements/scope` rev 5 and the working entry.
  - The scope premise on transformed bounds was corrected at P3. A rotated or scaled canvas is read unrotated and
    unscaled, as the lint reads canvas labels, with no engine change.

## Beyond the plan
1. **New CARRY on «Circle Drawer's circles as virtual nodes»** (an existing entry, verbatim title). The socket host's
   answer, the command line's stdout and a cold-agent live run carry a virtual node's id and name by construction
   (one `Session::run` line), but none is read on the binary: no served app declares a virtual node. That entry, the
   first served app to declare one, owes the binary-level reading.
