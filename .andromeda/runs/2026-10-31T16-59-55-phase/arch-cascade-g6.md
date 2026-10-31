# arch · cascade read · group 6 of 9

## Carried
- (.andromeda/architecture-detail/standard-contracts.md:194) The chunk ends the closed-option refusal, so this measured count and its two options' reading must be re-measured and restated at the chunk's close on the stand's own `select#flight-type`, open and closed.
  > On the stand a driver `click` is accepted on 68 of the 77 snapshot nodes of the four lean tasks at boot, in each layout mode, and refused on 9: the three disabled controls (`flight-return-date`, `crud-update`, `crud-delete`) as `disabled`, `task-header-spacer` in each task as `covered` — a box 0 high, at whose centre the hit answers the header that holds it — and the flight booker's two options `flight-type-one-way` · `flight-type-return` as `covered`, each read at its select's client rect (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md).
  (.andromeda/architecture.md:183)
  > its options are its children in the snapshot, a `multiple` select's rows and groups reading client rects of their own.
- (.andromeda/architecture-detail/standard-contracts.md:178) The chunk restates `covered`'s reading and schema text, so a restatement must reach every place this text is written and served, the refusal module and the JSON test it cites among them, the remedy, the eight causes and their names standing.
  > `covered`'s meaning reads "another element is hit at the point the action would land; a box clipped on one axis stops a hit at its whole padding box, so an element lying outside it along the axis it does not clip can read `covered` though it shows; a box with no width or no height is hit at its centre on the element that holds it, so it can read `covered` though nothing covers it" — the remedy, the eight causes and their names unchanged — and is in every `covered` refusal's JSON line and in `describe()`'s causes on the command line's and the MCP server's `help` (packages/escher-driver/src/refusal.rs:74-79; packages/escher-driver/src/refusal.rs:20-24; packages/escher-driver/src/json.rs:868-871)
  (.andromeda/architecture-detail/standard-contracts.md:179)
  > the crate's docs state that an element's `bounds` are where it stands, that a hit stops where a box clips by `overflow`, three readings as off
  (.andromeda/architecture-detail/standard-contracts.md:198)
  > `tools/list` answers the twelve verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`), `help` last, each a tool named by its verb with a `description` — its help and, since 2026-10-12-self-description, then `. Returns: ` and each result field as `{name} ({kind}): {help}` joined by `; `
- (.andromeda/architecture-detail/standard-contracts.md:180) An open list is a new box under the driver's hit, so `covered` must keep these readings for what lies under and beside it, `stand_act_obstructed` still holding both off readings.
  > because the engine's hit stops at the padding box of a box paint clips (the Scrolling, selection, tree contract above); two readings are off — a box with no width or no height, hit at its centre on the element that holds it, which reads `covered` though nothing covers it (the stand's `task-header-spacer`, 0 high; tests/blitz-tests/tests/stand_act_ids.rs:55), and a target lying beside a box clipped on one axis, along the axis it does not clip, which reads in view and is refused `covered` with the instance unchanged, in both layout modes (tests/blitz-tests/tests/stand_act_obstructed.rs:560-605; as measured at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md); `stand_act_obstructed` holds both readings
  (.andromeda/architecture.md:143)
  > `visible_region` is the read-only reader of what can be seen, and seeing is never read from a hit; a hit stops at the padding box of a box paint clips, skips a node dropped since the last resolve, and answers nothing for the whole subtree of a transform whose determinant is 0 or not finite
- (.andromeda/architecture-detail/standard-contracts.md:188) An option of an open list must be acted on through this one check order, its raw hit reaching the option, a disabled option reading `disabled` first and a refused call moving nothing, the list's open state included.
  > A `click` or a `type` on an element that cannot take it is then refused with nothing run, with one answer, the first that holds in this fixed order: `disabled` — the snapshot node's `enabled` is `Some(false)`; `off-screen` — the centre of the node's `bounds` lies outside `BaseDocument::visible_region` of the node, or that reads `None`; `covered` — the raw hit at the page point, taken to its nearest non-anonymous ancestor and then up the `parent` chain, never reaches the target, a point over no box reading as the root element. `press` and `advance` are refused for no screen-level cause, and a refused call leaves the snapshot and both focus readings as they were.
  (.andromeda/architecture.md:151)
  > an option's is the one rule `option_disabled`, its own attribute or its group's; the select's rules live in the crate-private `node::select`, with no public type.
- (.andromeda/architecture-detail/standard-contracts.md:189) `click` and `type` on a single select both land a click on it, so once a click opens the list the plan must state what each verb leaves open: `type` must still choose by its text and must not leave a list open by accident.
  > Then `click` clicks the centre of the snapshot's `bounds` for the node plus the viewport's scroll — a page point, where until 2026-10-07-refusal-detection it was the centre of the layout border box — through `Harness::click_at`, and `type`, on every target but a `multiple` select, which it focuses with no click (below), clicks it the same way, which focuses a text input, and types through `Harness::type_text`
  (.andromeda/architecture-detail/standard-contracts.md:179)
  > `type`'s help still says it replaces what the element holds and that an empty text clears it, and says that on a select the text chooses the first option whose label starts with it and, on a `multiple` select, that the text chooses its match alone (packages/escher-driver/src/schema.rs:237-239)
- (.andromeda/architecture-detail/standard-contracts.md:190) A `multiple` select's driver path must stay as built, no click and a cleared typeahead, its reading adding nothing to the closed sets.
  > `type` on a `multiple` select: `Target` carries `multiple`, and `type` does not click such a target, a click landing on the row at the centre: the crate-private `focus_multiple_select` focuses it with no click and clears the typeahead, without which a second `type` joins the first's text; a text opening on a space types nothing, since no label can match and the Space would flip a row; the reading holds no verb, argument, key name, result field, cause, session error or exit of its own (packages/escher-driver/src/execute.rs:235-239; packages/escher-driver/src/execute.rs:557-563).
  (.andromeda/architecture.md:135)
  > public `clear_select_typeahead` clears the typed text, its one non-test caller the driver's `focus_multiple_select`; the choice's writers are crate-private.
- (.andromeda/architecture-detail/standard-contracts.md:191) Choosing from the list by key and closing it without a choice must work for an agent through `press` on what holds focus, within the twelve existing key names, so the key contract's fork must rest on that focus.
  > `press` presses its key on what holds focus through `Harness::press_with`, with the shift modifier when `shift` is true
  (.andromeda/architecture-detail/standard-contracts.md:172)
  > `Key` (one name of `KEY_NAMES`: `tab` · `enter` · `escape` · `backspace` · `delete` · `space` · `arrow-up` · `arrow-down` · `arrow-left` · `arrow-right` · `home` · `end`)
  (.andromeda/architecture-detail/standard-contracts.md:192)
  > `press` carries no Ctrl, so a set of rows of a `multiple` select that are not adjacent cannot be built through the driver, and its help states it: "a set of rows that are not adjacent needs Ctrl, which this verb does not carry" (packages/escher-driver/src/schema.rs:261-263).
- (.andromeda/architecture-detail/standard-contracts.md:191) An option past the list's visible part must answer `scroll` by this reading, accepted whatever covers it, and an option `scroll` cannot bring in must answer `in_view` false rather than a refusal.
  > `scroll` applies none of the three checks that stand in front of a click — it is accepted on a disabled or a covered element: it calls the harness's `scroll_into_view` input helper on the resolved node, an instant scroll of every scrolling box that holds the element and then of the viewport, as one settled step, and answers `in_view` by making the `off-screen` reading again on the snapshot taken after the step — false when no scroll could bring the element into view, and when the step removed it; it is never refused for failing to reach its target, its result says so.
- (.andromeda/architecture-detail/standard-contracts.md:175) A driver test or pointer gate that opens, closes or chooses by two clicks on one point must advance at least 500 ms between them or meet the double click this reads as.
  > two `click`s on one point with no `advance` of at least 500 ms between them read as a double click, as `click`'s help says (ratified by the founder, 2026-10-18)
- (.andromeda/architecture-detail/standard-contracts.md:187) A list's options must stay in the snapshot and in `element_ids` under their author keys so that an option's id resolves on the screen as it reads now, open or closed.
  > the id names an element when that snapshot lists it and the first pair of `DioxusDocument::element_ids()` whose id equals the argument has a node that resolves through `get_node`; otherwise the call is refused with nothing run — `stale` when the session's record holds the id, `not-found` when it does not.
  (.andromeda/architecture.md:177)
  > under `accessibility` the snapshot, its text form and its diff read the screen read-only, holding no `NodeId`.
  (.andromeda/architecture.md:181)
  > Every element an agent can act on reads an author key: `DioxusDocument::unkeyed_actionable` lists, read-only, each actionable element whose stable id holds a `/`, with a remedy naming the fix.
- (.andromeda/architecture-detail/standard-contracts.md:173) The open state must reach an agent through these thirteen keys alone unless a fork names a new one, and an acting verb that opens or closes a list must report its options' moved boxes in this diff form.
  > written on a changed node only: the keys whose reading differs from before the step, in this order, a node that only moved reading `bounds` alone
  (.andromeda/architecture-detail/standard-contracts.md:193)
  > Each acting verb is one settled step: the snapshot before, the step, the load wait over `Harness::settle`, the snapshot after and `before.diff(&after)`; settling moves no time
- (.andromeda/architecture-detail/standard-contracts.md:171) The scope adds no verb, node key, cause, session error, exit status or carrier, so each of these counts and closed sets must hold, and anything a fork needs must be named on that fork as a further widening he ratifies.
  > `VERBS` lists twelve verbs in three levels
  (.andromeda/architecture-detail/standard-contracts.md:174)
  > 12 verbs, 13 node keys, 12 key names, 3 busy classes, 8 causes, 4 exit statuses, 15 session errors
  (.andromeda/architecture-detail/standard-contracts.md:176)
  > `Cause` is closed at eight, listed in `CAUSES` (`[Cause; 8]`, ratified by the founder, 2026-10-18)
  (.andromeda/architecture-detail/standard-contracts.md:176)
  > none of the three has a `String` field, so a refusal holds nothing a call supplied.
  (.andromeda/architecture-detail/standard-contracts.md:169)
  > The wire is crate-private and versioned `v3`
  (.andromeda/architecture-detail/standard-contracts.md:164)
  > the crate re-exports 57 names under `#![deny(missing_docs)]`
  (.andromeda/architecture.md:167)
  > the command and refusal schema stated once as `'static` data — twelve verbs, eight closed causes, no refusal holding what a call supplied; the `v3` wire, the command line, the MCP server and the `command` and `tool_call` spans carry it; no `NodeId` crosses, and nothing logs an id, a name or a value.
- (.andromeda/architecture-detail/standard-contracts.md:186) Acting on an option must enter through this one path, the `execute` edit adding no route around `validate`.
  > `Session::run(&mut self, call: &Call) -> Result<Outcome, Refusal>` is the one entry point — it calls `validate(call)` first and returns its refusal before anything else runs, and nothing public runs a `Command` that did not come from `validate`.
  (.andromeda/architecture.md:167)
  > a `Session` (one booted app) runs every call through `Session::run`, which validates first; nothing public runs a `Command` that did not come from `validate`
- (.andromeda/architecture-detail/standard-contracts.md:182) Whatever the `execute` edit answers must be written by hand in this module, keyed by the schema's own words, with no `NodeId` in a call or an outcome.
  > What a caller is answered with has one written form each, a line of JSON keyed by the schema's own words, written by hand in the crate-private `json` module and output only
  (.andromeda/architecture-detail/standard-contracts.md:195)
  > and no `NodeId` is in either
  (.andromeda/architecture.md:169)
  > Every answer is written by hand in the crate-private `json` module — no serialization crate writes one, `serde_json` only reading MCP requests and kept checks — and no command's ending writes back anything the caller typed.
- (.andromeda/architecture-detail/standard-contracts.md:193) A driver `screenshot` must show the opened list and an option's element shot must crop to what its own box shows, the shot changing nothing, the list's state included.
  > with an `id` it crops the frame to the part of the element's bounds that can be seen, rounded outward to whole pixels (private `crop`, packages/escher-driver/src/execute.rs:473-494), a crop of no pixel refused `off-screen`
  (.andromeda/architecture-detail/standard-contracts.md:193)
  > a shot changes nothing — `has_changes` stays false, focus and the next snapshot are unchanged — and on the stand a whole shot is an 800 × 600 PNG equal to the harness's raster and a shot of `crud-list` 369 × 420, that element's rectangle.
- (.andromeda/architecture-detail/standard-contracts.md:171) Reduced motion's fork must keep this reading: any motion the list paints must read the session's clock, so two shots of one state with no `advance` between them read the same pixels.
  > the overlay scrollbar fade reading the session's clock, so two shots of one state with no `advance` between them read the same pixels (ratified by the founder, 2026-10-18)
  (.andromeda/architecture-detail/standard-contracts.md:175)
  > `advance` moves the session's clock forward by `ms` milliseconds on every session — the engine's animations, the overlay scrollbar fade and, through the step a caller hands over with `Session::with_time` where there is one, the app's own time together — then settles
- (.andromeda/architecture-detail/standard-contracts.md:208) A `snapshot` or `lint` step's answer is kept whole, so the two committed checks must be re-read against the snapshot's new option boxes, and any re-recording must come through this one recording path.
  > a call no record can spell (a verb outside the eight instance verbs, a `lint` step kept like every other with its answer whole, an argument its schema does not name or names twice) is not kept
  (.andromeda/architecture-detail/standard-contracts.md:208)
  > each answer is kept whole, a shot's pixels excepted.
  (.andromeda/architecture.md:171)
  > The recording keeps a session's answered calls as a kept check, a masked `type` as its stand-in and a shot without its pixels; `Check::replay` runs them again through `Session::run`
- (.andromeda/architecture-detail/standard-contracts.md:210) `stand_replay` must still hold after the change, and a check whose answer the open list moves must be restated on the evidence of this comparison, never by loosening it.
  > otherwise it runs each step's `call()` through `Session::run`, so every step is validated again, and stops at the first step that differs — its acceptance differs, or the live answer's kept line, read by `serde_json::from_str`, is not equal to the kept `answer` (private `change_of`).
- (.andromeda/architecture-detail/standard-contracts.md:213) The exclusion must be restated for an open list's own boxes, a closed list's options staying out, on the element and never the role, and `closed_select_of`'s answer once a select can be open decides what this entry reads.
  > The `overlap` reading leaves out a closed select's options: `Entry.in_closed_select` reads through `closed_select_of`, by the element and never the role, so CRUD's `role="option"` rows are not read as a select's, and `overlaps` skips such an entry on either side; a `multiple` select's rows are linted as usual (packages/escher-driver/src/lint.rs:246-247; packages/escher-driver/src/lint.rs:337-338; packages/escher-driver/src/lint.rs:347; packages/escher-driver/src/lint.rs:364; packages/escher-driver/src/lint.rs:368).
  (.andromeda/architecture.md:183)
  > the public `closed_select_of` and `is_multiple_select` are the select tests the driver reads, naming the element and never the role
- (.andromeda/architecture-detail/standard-contracts.md:212) An open list's options read real boxes under these readings, so the plan must state what they answer: `overlap` against the page beneath the list, and `clipped` and `off-screen` against a clip region built from DOM ancestors the overlay may paint past.
  > Each node's clip region is `visible_region` cut by the padding box of every DOM ancestor below the root whose computed `contain` includes `paint`
  (.andromeda/architecture-detail/standard-contracts.md:212)
  > `overlap`, two nodes neither an ancestor of the other whose visible parts intersect, a closed select's options excepted (below), reported once at the highest level, on the earlier node — a part scrolled out of view overlaps nothing; `clipped` and `off-screen`, read against that clip region;
  (.andromeda/architecture-detail/standard-contracts.md:212)
  > `contrast`, a node with direct non-whitespace text or a text control, read through `Harness::text_contrast` and reported below `required_ratio()`, exempt at `enabled == Some(false)`, `None` not measured.
- (.andromeda/architecture-detail/standard-contracts.md:211) Any restatement must leave the lint a pure reading of the screen, opening and closing nothing, its new exclusion read from geometry and the element with no hit taken.
  > it reads `harness.doc.snapshot()`, recording each node's id in the session's record of ids as `snapshot` does, and runs no step, settle, pump or time move, so it changes nothing.
  (.andromeda/architecture-detail/standard-contracts.md:171)
  > whose help states the six defect words, each reason's keys and its limits — geometry only, no hit, right-to-left and vertical text read as left-to-right lines
  (.andromeda/architecture.md:171)
  > `Session::lint` reads the screen and changes nothing

## Bindings
- **Driver's `covered` and acting checks ↔ the engine's hit and overlay (Scrolling, selection, tree)**: the option's reachability is read from the raw hit, so the overlay's hit target and the driver's check must agree on what an open list's option answers, and the off-screen half must read `visible_region`, never a hit (.andromeda/architecture-detail/standard-contracts.md:188) (.andromeda/architecture.md:143)
- **`closed_select_of` ↔ `execute` and `lint`**: both driver readings rest on this one public select test, so what it answers once a select can be open must be decided once and read by both (.andromeda/architecture-detail/standard-contracts.md:213) (.andromeda/architecture.md:183)
- **Snapshot's option boxes ↔ the recording, the replay and the stand tables**: the option boxes move a kept `snapshot` and `lint` answer, so `snapshot`, `stand_replay` and the `stand_lint` row must move together (.andromeda/architecture-detail/standard-contracts.md:208) (.andromeda/architecture-detail/standard-contracts.md:210)
- **Closed driver sets ↔ Established Decisions' validated surface and the Playbook Boundary widening**: the twelve verbs are a validated set widened only on the founder's ratification, so a fork needing a verb, key or cause must say so on the fork, apart from the new input and pixels the widening admits (.andromeda/architecture-detail/standard-contracts.md:171)
- **Reduced motion and new default colours ↔ the driver's shot and lint**: the fork's answers must reach the driver as a clock-read motion and as a `contrast` reading through `Harness::text_contrast` (.andromeda/architecture-detail/standard-contracts.md:171) (.andromeda/architecture-detail/standard-contracts.md:212)
