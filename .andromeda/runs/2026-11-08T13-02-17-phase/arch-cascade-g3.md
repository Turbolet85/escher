@carried n=1 label=architecture/established-decisions fact=fct-qnjfjj37
why: The read's actionable set must leave out an inert element exactly as unkeyed_actionable does, and its covered reading must keep a hit passing through an inert element, so the list names no node the snapshot omits and no cause a call would not return.
quote: `Node::is_focussable` also reads false for an inert node through the crate-private `Node::is_inert`, an ancestor walk, so the cached `ElementData::is_focussable` stays per element and no `inert` arm joins the attribute writes, and the Tab walk `focus_next_node` / `focus_prev_node`, autofocus (`process_added_subtree`) and `DioxusDocument::unkeyed_actionable` skip it; `hit_inner` answers no hit at an element carrying `inert`, before any other test, so a hit passes through it; the accessibility builder's `is_hidden_from_accessibility_tree` reads true for it, so the subtree leaves the tree and the snapshot
@carried n=1 run=2 label=architecture fact=fct-ay4qkh7m
quote: `inert` is honoured by its presence on the element or an ancestor — not focusable, not hit, out of the accessibility tree and the snapshot;
@carried n=2 label=architecture/established-decisions fact=fct-r7sg7jcz
why: The owner-served nodes the read lists under a select must be exactly its options and its groups' options in tree order, each a snapshot node though it computes display none, and an option of a closed single select must read refused covered.
quote: a select's options are its `option` children and its `optgroup` children's, in tree order;
@carried n=2 run=2 label=architecture/established-decisions fact=fct-r7sg7jcz
quote: The accessibility builder keeps an option whose parent is a `select`, or an `optgroup` under one, although it computes `display: none` — `hidden`, `inert` and `visibility: hidden` still exclude it —
@carried n=2 run=3 label=architecture fact=fct-hnlfyl4f
quote: an option of a closed single select, under a group too, takes no click and stays refused `covered`;
@carried n=3 label=architecture/established-decisions fact=fct-qcw5degj
why: The read's disabled cause for an option must come from option_disabled, its own attribute or its group's, and for a form control from the one disabled reading, through the shared path the executor calls.
quote: `option_disabled` is the one rule `readings` and `form.rs` share
@carried n=3 run=2 label=architecture fact=fct-fe5adstg
quote: `disabled` is read by its presence on a form control through one reading — focusability, the pointer and click gates and the Enter and Space activation read it — and an option's through `option_disabled`, its own attribute or its group's;
@carried n=4 label=architecture/established-decisions fact=fct-wau2zivr
why: An owner-served group under an open list chooses nothing, and an open list answers a hit first, so the read's per-verb answer for a group and a row must agree with what a real call on them does.
quote: a header chooses nothing, and a group with no label reads the whole list's box
@carried n=4 run=2 label=architecture fact=fct-6ketzni4
quote: a hit stops at the padding box of every box paint clips; where a select list stands open a hit answers the list first, with no scrollbar, so a row lies past any box that clips its select,
@carried n=5 label=architecture/established-decisions fact=fct-kukffiv3
why: The new read's answer on the socket, the command line and MCP must cross as ids, closed words and booleans with no NodeId, page text or value, its help and tool text as fixed static text of the one table, serde_json staying a reader, and any new verb, carrier, exit or field brought as a boundary widening for the founder.
quote: a lint's findings cross on those carriers and rest in a kept check as the `lint` verb's answer — stable ids, the closed defect words and numbers, no page text, colour, control value or `NodeId` — with no new carrier, exit, path argument, cause, session error, span field or dependency: a ratified boundary widening;
@carried n=5 run=2 label=architecture/established-decisions fact=fct-kukffiv3
quote: since 2026-10-12-self-description the command line's stdout and the `mcp` role's stdio also carry the program's own fixed text, held as `'static` data of the one table — the `help` self-description, a session error's remedy and each tool's description with its result fields — riding those ratified carriers, while text from a page, a call or a screen, a new carrier or a new exit still widens;
@carried n=5 run=3 label=architecture/established-decisions fact=fct-kukffiv3
quote: `serde_json` (`{ workspace = true }`, no feature and no `[features]` table of its own, a reader of MCP requests, of kept checks and of the live answer line a replay compares, writing nothing
@carried n=5 run=4 label=architecture/established-decisions fact=fct-kukffiv3
quote: it opens one `tracing` span per call handed to `Session::run` and, since 2026-10-10-mcp-surface, one `tool_call` span per MCP `tools/call` request, holding `tool` and `ending` only
@carried n=5 run=5 label=architecture fact=fct-nb3g6bqn
quote: ids, names, masked values, snapshot text, diffs, shots, lint findings and a UI graph's names, closed flag words and booleans cross, never a `NodeId`; the library depends on no app, emits no event, installs no subscriber and reads no env var; a hosted session expires when idle; a new carrier or exit widens the boundary.
@carried n=6 label=architecture/established-decisions fact=fct-ttqlo35r
why: The new read stands beside graph as its ratified precedent and reads the snapshot's closed clickable bool, so its answer must hold only stable ids, owner ids, verb names, closed cause words and booleans, never a value, typed text or NodeId, and its argument rule must refuse malformed with nothing run.
quote: The UI graph crosses the ratified carriers — the session socket's `call`, the command line and MCP, as a text block — and rests in a kept check as the `graph` verb's answer: `graph` takes no argument, one refused `malformed` with nothing run, and its answer holds stable ids, owner ids, component names, registered state names, handler event names, the declared writes the app authors, the closed flag words `undeclared` and `unconfirmed` and booleans, never a value, typed text or `NodeId`; each snapshot node carries the closed bool `clickable`; no new carrier, exit, cause, session error, span field or dependency: a ratified boundary widening
@carried n=7 label=architecture/established-decisions fact=fct-7p2g4lnp
why: The new read's answer must rest in a kept check like every answered call, owner-only and never logged, and replay through Session::run alone, the two committed stand checks staying the one exception.
quote: a kept check holds the screens' ids, names, values, snapshot text, diffs, a lint's findings and a UI graph's names, closed flag words and booleans at rest, never logged and never named by a tool call; a kept check replays through `Session::run` alone (`Check::replay`), and the repository's own two scripted stand checks are committed in git, the one exception to owner-only.
@carried n=7 run=2 label=architecture fact=fct-2okzb4io
quote: A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, never logged and never named by a tool call; the repository's two committed stand checks are the one exception to owner-only.
@binding n=1
**established-decisions (driver session crossing) ↔ standard-contracts (driver session schema)**: the new read, a verb added to the one `'static` command and refusal schema (architecture fct-ksgwc2yv), must cross only as the ratified decisions allow, ids, closed words and booleans with no `NodeId` (architecture/established-decisions fct-kukffiv3) (architecture/established-decisions fct-ttqlo35r); a new verb on every carrier, a new field or a VerbSpec change is a boundary widening brought to the founder, not taken.
@binding n=2
**established-decisions (disabled and inert readings) ↔ standard-contracts (nodes and widgets, scrolling)**: the shared refusal path must read disabled through `ElementData::is_disabled` and `option_disabled` (architecture fct-bedjim2b) (architecture/established-decisions fct-qcw5degj), seeing through `visible_region` never from a hit (architecture fct-mjzuvlbq), and a hit passing through `inert` (architecture/established-decisions fct-qnjfjj37), so the read and a real call return one cause.
@binding n=3
**established-decisions (select model) ↔ standard-contracts (Dioxus DOM bridge)**: the owner-served node of a select must be its options and groups as the snapshot holds them, its children (architecture fct-6gp3hxh7), by the select's own rules (architecture/established-decisions fct-r7sg7jcz), a closed single select's option reading covered (architecture fct-hnlfyl4f).
@binding n=4
**established-decisions (inert) ↔ standard-contracts (actionable-key rule)**: the read's actionable set and its folded actionable-key answer must agree with `unkeyed_actionable` (architecture fct-d5zuqqfb), which skips an inert element (architecture/established-decisions fct-qnjfjj37).
@binding n=5
**established-decisions (kept check) ↔ occupied-resources (recording directory)**: the new read's answer rests in a kept check in the launcher-named owner-only directory (architecture fct-kjd5y7c4) and replays through `Session::run` alone (architecture/established-decisions fct-7p2g4lnp).
