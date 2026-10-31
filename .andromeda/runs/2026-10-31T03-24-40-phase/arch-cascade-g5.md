# arch · cascade read · group 5 of 8

## Carried
- (.andromeda/architecture-detail/standard-contracts.md:141) The Tab-order, keyboard-activation and contrast rows on the load page must read this page and this Button, the row's contrast measured on its white-on-accent text, and a red row is a further fork that moves these colours, never a silent re-pin.
  > `boot_load(HarnessOptions) -> Harness<DioxusDocument>` mounts a load page in `TaskShell` titled "Load" (the private `load_root` and `LoadPage`, its CSS `.load-root` · `.load-body` · `.load-btn` with `:hover` · `:active` · `.load-done`) holding two keyed elements — `load-fetch`, a Button "Load" in white on the stand accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`), whose click links the page's one stylesheet, and `load-done`, a Paragraph "Loaded" at `display: none` until that sheet lands, no CSS selecting by id — its options' `net_provider` replaced by `Some(Arc::new(LoadNet))` through `..options`, every other field kept
  (.andromeda/architecture.md:163)
  > `seven_guis::stand` boots the four lean tasks and, beside them, Cells (`boot_cells`, no `LeanTask`) and a load page (`boot_load`) in a pinned 800x600 Light viewport with one bundled font, every boot offline but the load page's, whose one stylesheet fetch the in-process stand-in `LoadNet` answers after `LOAD_ANSWER_DELAY` (200 ms)
  (.andromeda/architecture.md:159)
  > `text_contrast` reads a WCAG contrast and changes nothing
- (.andromeda/architecture-detail/standard-contracts.md:142) A keyboard-activation row that witnesses `load-fetch` by `load-done` must wait out this answer, which arrives from a thread of its own and is counted as a load, so the row's wait is real time the shared clock does not move.
  > each `fetch` answers the fixed `LOADED_CSS` (`.load-body .load-done { display: block; }`) for the fixed `LOAD_URL` `http://stand.test/load.css` from a thread of its own after `pub const LOAD_ANSWER_DELAY: Duration` (200 ms); it overrides no `is_noop`, so the harness counts its load, and holds no `blitz_net`, print, env read, socket or listener
  (.andromeda/architecture.md:159)
  > `settle` drives render and layout to quiet, reports a load in flight and never waits on one or moves time.
  (.andromeda/architecture.md:161)
  > The harness holds one `VirtualClock` its documents share; `tick` and `advance_time` move it, and `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never do.
- (.andromeda/architecture-detail/standard-contracts.md:169) A fix to the transformed scrolling box must restate this meaning and every text that repeats its second clause, on a recorded word as the first reword was, and a kept limit must keep it as it stands.
  > its meaning reads "the element lies outside the viewport, or outside the visible part of a scrolling box that holds it; a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows" — its first clause the operator's (2026-10-07, at that chunk's plan review), its second a measured limit (the Scrolling, selection, tree contract) (packages/escher-driver/src/refusal.rs:25-28; packages/escher-driver/src/refusal.rs:81-86) and its remedy "the element is out of view: bring it into view with `scroll`, then act again" (the founder, 2026-10-07, relayed verbatim by the overseer).
  (.andromeda/architecture-detail/standard-contracts.md:171)
  > and a scrolling box under a transform read where it stands untransformed, so an element inside it can read `off-screen` though it shows (packages/escher-driver/src/lib.rs:72-78)
  (.andromeda/architecture.md:145)
  > `get_client_bounding_rect` reads an element where its transforms move it, while `visible_region` reads a scrolling box under a transform where it stands untransformed (a measured limit)
- (.andromeda/architecture-detail/standard-contracts.md:180) This fixed order is the route by which `visible_region` reaches a driver call, so the pan/zoom proof must read a target through it: past `off-screen` a target in a transformed box meets the `covered` hit next, and the order itself must hold.
  > A `click` or a `type` on an element that cannot take it is then refused with nothing run, with one answer, the first that holds in this fixed order: `disabled` — the snapshot node's `enabled` is `Some(false)`; `off-screen` — the centre of the node's `bounds` lies outside `BaseDocument::visible_region` of the node, or that reads `None`; `covered` — the raw hit at the page point, taken to its nearest non-anonymous ancestor and then up the `parent` chain, never reaches the target, a point over no box reading as the root element.
  (.andromeda/architecture-detail/standard-contracts.md:181)
  > Then `click` clicks the centre of the snapshot's `bounds` for the node plus the viewport's scroll — a page point, where until 2026-10-07-refusal-detection it was the centre of the layout border box — through `Harness::click_at`
  (.andromeda/architecture-detail/standard-contracts.md:182)
  > answers `in_view` by making the `off-screen` reading again on the snapshot taken after the step — false when no scroll could bring the element into view, and when the step removed it
  (.andromeda/architecture-detail/standard-contracts.md:183)
  > with an `id` it crops the frame to the part of the element's bounds that can be seen, rounded outward to whole pixels (private `crop`, packages/escher-driver/src/execute.rs:460-481), a crop of no pixel refused `off-screen`
  (.andromeda/architecture.md:143)
  > `visible_region` is the read-only reader of what can be seen, and seeing is never read from a hit
- (.andromeda/architecture-detail/standard-contracts.md:170) A fork that moves a select's option or a hit must restate `covered`'s meaning on its one table row, so that every carrier reads it, the cause set and order and every other text standing.
  > `covered`'s meaning reads "another element is hit at the point the action would land; a box clipped on one axis stops a hit at its whole padding box, so an element lying outside it along the axis it does not clip can read `covered` though it shows; a box with no width or no height is hit at its centre on the element that holds it, so it can read `covered` though nothing covers it" — the remedy, the eight causes and their names unchanged — and is in every `covered` refusal's JSON line and in `describe()`'s causes on the command line's and the MCP server's `help`
  (.andromeda/architecture-detail/standard-contracts.md:171)
  > three readings as off — a box clipped on one axis stopping a hit at its whole padding box, so an element beside it along its unclipped axis can read `covered` though it shows; a box with no width or no height hit at its centre on the element that holds it, so it can read `covered` though nothing covers it
  (.andromeda/architecture-detail/standard-contracts.md:172)
  > two readings are off — a box with no width or no height, hit at its centre on the element that holds it, which reads `covered` though nothing covers it (the stand's `task-header-spacer`, 0 high; tests/blitz-tests/tests/stand_act_ids.rs:55), and a target lying beside a box clipped on one axis, along the axis it does not clip, which reads in view and is refused `covered` with the instance unchanged, in both layout modes (tests/blitz-tests/tests/stand_act_obstructed.rs:560-605; as measured at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md); `stand_act_obstructed` holds both readings
- (.andromeda/architecture-detail/standard-contracts.md:184) The select debts must be measured against this count and its two refused options, both forks over a clicked option or a painted popup moving it, and the plan must re-measure it in each layout mode where a fix lands.
  > On the stand a driver `click` is accepted on 68 of the 77 snapshot nodes of the four lean tasks at boot, in each layout mode, and refused on 9: the three disabled controls (`flight-return-date`, `crud-update`, `crud-delete`) as `disabled`, `task-header-spacer` in each task as `covered` — a box 0 high, at whose centre the hit answers the header that holds it — and the flight booker's two options `flight-type-one-way` · `flight-type-return` as `covered`, each read at its select's client rect (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md).
  (.andromeda/architecture-detail/standard-contracts.md:171)
  > and says that on a select the text chooses the first option whose label starts with it (packages/escher-driver/src/schema.rs:238-239)
  (.andromeda/architecture.md:183)
  > a select's `input` and `change` reach the app as form data, and its options are its children in the snapshot.
  (.andromeda/architecture.md:141)
  > the engine dispatches `change` only after `input` on a select's choice.
- (.andromeda/architecture-detail/standard-contracts.md:165) The native-checkedness fix must be measured against these readings, the snapshot's `checked` already reading the element and `pressed` left out for the five checkable roles, so that writing `toggled` moves no reading silently.
  > `checked` a native checkbox's or radio's own state, else the toggle of the five checkable roles (`aria-checked`), a mixed one reading false, `selected` · `pressed` · `invalid` read by `aria-selected` · `aria-pressed` (left out for those five roles) · `aria-invalid` with the boolean model's two limits (a mixed toggle reads `pressed` false, a grammar or spelling error reads `invalid` true), `bounds` the same however far a box's content is scrolled
  (.andromeda/architecture.md:139)
  > the accessibility tree states no native checkbox's or radio's checked state, its fix owned by a `.andromeda/residuals.md` line
- (.andromeda/architecture-detail/standard-contracts.md:165) The node keys must stay these thirteen: a debt whose fix would add one is a boundary question brought as a fork.
  > `added` and `changed` (nodes, each keyed by the thirteen keys `NODE_FIELDS` names: `id` · `parent` · `role` · `name` · `enabled` · `checked` · `value` · `selected` · `pressed` · `invalid` · `focused` · `bounds` · `differs`
- (.andromeda/architecture-detail/standard-contracts.md:163) The verbs must stay these twelve: a fix that would add a verb is a boundary question brought as a fork.
  > `VERBS` lists twelve verbs in three levels
- (.andromeda/architecture-detail/standard-contracts.md:168) The causes must stay this closed eight with each text a fixed string of the `'static` schema, written by hand and holding no `NodeId`, so a restated `off-screen` or `covered` text changes a table row and nothing else, and a fix that would add a cause is brought as a fork.
  > `Cause` is closed at eight, listed in `CAUSES` (`[Cause; 8]`, ratified by the founder, 2026-10-18) — `unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `fonts-unavailable`, the eighth (meaning "the app this session holds has no font it can paint text with", remedy "boot the app with a font it can decode") — each with a fixed `name()`, `meaning()` and `remedy()`; `Fault` is `Missing` · `Unnamed` · `Repeated` · `WrongKind` · `OutOfBound`, a variant that names an argument holding the schema's own name for it; none of the three has a `String` field, so a refusal holds nothing a call supplied.
  (.andromeda/architecture-detail/standard-contracts.md:162)
  > The command and refusal schema is stated once, as `'static` data;
  (.andromeda/architecture-detail/standard-contracts.md:166)
  > its keys are `HELP`'s seven result fields in order, every key name read from `HELP.fields` and every text from the tables — `VERBS`, `NODE_FIELDS`, `KEY_NAMES`, `BUSY_CLASSES`, `CAUSES` with their remedies, `EXIT_STATUSES` and the session errors with their remedies: 12 verbs, 13 node keys, 12 key names, 3 busy classes, 8 causes, 4 exit statuses, 15 session errors
  (.andromeda/architecture-detail/standard-contracts.md:174)
  > What a caller is answered with has one written form each, a line of JSON keyed by the schema's own words, written by hand in the crate-private `json` module and output only
  (.andromeda/architecture-detail/standard-contracts.md:185)
  > and no `NodeId` is in either
- (.andromeda/architecture-detail/standard-contracts.md:183) A served-page row driven through the driver must read `load-fetch`'s activation through this settled step and stay inside this wait, its keyboard route being the one `press` carries.
  > Each acting verb is one settled step: the snapshot before, the step, the load wait over `Harness::settle`, the snapshot after and `before.diff(&after)`; settling moves no time, and a load in flight is waited for, for at most `LOAD_WAIT` (the load wait, below).
  (.andromeda/architecture-detail/standard-contracts.md:182)
  > `press` presses its key on what holds focus through `Harness::press_with`, with the shift modifier when `shift` is true
  (.andromeda/architecture.md:171)
  > `LOAD_WAIT` (5 s) bounds an acting step's wait for a load, moving no time.
  (.andromeda/architecture.md:141)
  > Enter and a released Space dispatch a Click the driver queues, so the app's handlers run before the default action
- (.andromeda/architecture-detail/standard-contracts.md:202) The recorded fix to the option overlap edits `overlaps`, and it must keep this reading for every other pair of nodes, in every document, or state what it moves.
  > `overlap`, two nodes neither an ancestor of the other whose visible parts intersect, reported once at the highest level, on the earlier node — a part scrolled out of view overlaps nothing; `clipped` and `off-screen`, read against that clip region;
- (.andromeda/architecture-detail/standard-contracts.md:202) A fix to `visible_region` moves each lint reading built on it, so the plan must re-measure `stand_lint` under that fix as well as under the overlap's.
  > Each node's clip region is `visible_region` cut by the padding box of every DOM ancestor below the root whose computed `contain` includes `paint` — the lint's own walk, `visible_region` unchanged
- (.andromeda/architecture-detail/standard-contracts.md:202) `stand_lint`'s pinned reading is the chunk's to re-pin only as the founder's word says, in both layout modes and with the `lint` verb answering the same.
  > a node both clipped or off-screen and holding text also reads `truncated`, pinned by `stand_lint` in both layout modes, the `lint` verb answering the same
- (.andromeda/architecture-detail/standard-contracts.md:202) A new contrast row on a served page must read the same measure the lint reads, through `Harness::text_contrast` against the same required ratio, so the row and the lint cannot disagree over one node.
  > `contrast`, a node with direct non-whitespace text or a text control, read through `Harness::text_contrast` and reported below `required_ratio()`, exempt at `enabled == Some(false)`, `None` not measured.
- (.andromeda/architecture-detail/standard-contracts.md:201) An overlap fix must leave the lint read-only and its closed defect set and reason shapes standing, its findings in the same order and the same written form on every carrier.
  > `Session::lint(&mut self) -> Result<Vec<Finding>, Refusal>` reads the held instance's current screen and returns one `Finding { id, defect, bounds, reason }` per defect it reads, in document order, then `Defect::ALL` order
  (.andromeda/architecture-detail/standard-contracts.md:201)
  > and runs no step, settle, pump or time move, so it changes nothing. `Defect` is closed at six
  (.andromeda/architecture-detail/standard-contracts.md:163)
  > whose help states the six defect words, each reason's keys and its limits — geometry only, no hit, right-to-left and vertical text read as left-to-right lines
  (.andromeda/architecture-detail/standard-contracts.md:175)
  > a `lint`'s outcome `{"findings":[…]}`, each finding an object keyed `id` · `defect` · `bounds` (`[x, y, width, height]`, as a node's bounds), then its reason's keys — `other` and `area` for an overlap
  (.andromeda/architecture.md:171)
  > `Session::lint` reads the screen and changes nothing
- (.andromeda/architecture-detail/standard-contracts.md:198) A restated cause text, a moved reading or a fixed overlap changes answers a kept check holds whole, so the plan must read which committed steps hold them.
  > then appends one record per answered call, `tool` · `arguments` in the verb's schema order · `masked` (`true`, on a `type` alone) · `accepted` · `digest` · `answer` — the answer's line as the driver wrote it (`Outcome::to_json` or `Refusal::to_json`, private `written`) inlined as its own JSON object, masked as on every carrier
  (.andromeda/architecture-detail/standard-contracts.md:198)
  > a `lint` step kept like every other with its answer whole
  (.andromeda/architecture.md:171)
  > The recording keeps a session's answered calls as a kept check, a masked `type` as its stand-in and a shot without its pixels; `Check::replay` runs them again through `Session::run`
- (.andromeda/architecture-detail/standard-contracts.md:200) The two committed checks must still replay after any fix, since a live answer that differs from a kept one stops the replay at that step, and a moved answer in them is the chunk's to re-record on the founder's word.
  > otherwise it runs each step's `call()` through `Session::run`, so every step is validated again, and stops at the first step that differs — its acceptance differs, or the live answer's kept line, read by `serde_json::from_str`, is not equal to the kept `answer` (private `change_of`).
  (.andromeda/architecture.md:238)
  > checks/ — the two committed kept checks `stand_replay` replays

## Bindings
- **The driver's `off-screen` text ↔ the scrolling contract in the body**: a fix moves the limit in `visible_region` and its statement in the driver's refusal text and crate docs together, and neither stands alone, (.andromeda/architecture-detail/standard-contracts.md:169) with (.andromeda/architecture.md:145).
- **The driver's refusal texts ↔ the capability statement**: `off-screen` and `covered` are stated in the schema text and in `docs/capabilities.md`, whose pin a moved text moves with it, (.andromeda/architecture-detail/standard-contracts.md:170).
- **The lint's `overlap` ↔ the select model in Established Decisions**: an option's bounds read at its select's client rect are what the overlap reads, and a painted popup changes what they should read, so the overlap's fork is asked with the popup's, (.andromeda/architecture-detail/standard-contracts.md:184) with (.andromeda/architecture.md:62).
- **The load page's rows ↔ the driver's load wait**: `LOAD_ANSWER_DELAY` must stay inside `LOAD_WAIT` for a driver step on `load-fetch` to read `load-done`, and a harness row must wait in real time because `settle` does not, (.andromeda/architecture-detail/standard-contracts.md:142) with (.andromeda/architecture.md:171).
- **The driver's answers ↔ the committed kept checks**: any moved cause text or reading reaches `stand_replay` through the answer kept whole, (.andromeda/architecture-detail/standard-contracts.md:198) with (.andromeda/architecture-detail/standard-contracts.md:200).
- **Native checkedness ↔ the snapshot's readings**: the tree's `toggled` write and the snapshot's `checked` and `pressed` must agree after a fix, (.andromeda/architecture-detail/standard-contracts.md:165) with (.andromeda/architecture.md:139).
