# Fan-out results — the wrap of 2026-10-10-snapshot-states

Seven doc-agents, one batch, at this wrap's resume. 15 detectors over the seven prompts (architecture 2 · security-plan
3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2) = the 15 `doc:` names of the drift
base. Each doc: its verdict line, then its parsed list. Dispositions are §Validate's, written beside each proposal.

## design-system — 0 proposals

Verdict: `proposals: []`. The return carried three YAML comment lines above the list, no other text: every Coverage row
reads `tokens n/a`, none `hardcoded✗`; the three stand rows are attribute-only with no style line; the report's own
expected amendment 13 is not carried as an amendment — the shifted citations are the citation sweep's.

## layout-templates — 1 proposal

```yaml
proposals:
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: desktop-native → Primary screens (the `seven_guis Home and TaskShell` bullet)"
    change: >-
      Add to the lean-task clause, beside the existing "named by attributes alone" sentence: since 2026-10-10-snapshot-states two lean tasks state role and state by attributes alone — the flight booker's mode buttons `flight-one-way` and `flight-return` are toggle buttons stating `aria-pressed` `"true"` | `"false"` (examples/seven_guis/src/tasks/flight_booker.rs:69; examples/seven_guis/src/tasks/flight_booker.rs:76), its date inputs `flight-start` and `flight-return-date` state `aria-invalid` (examples/seven_guis/src/tasks/flight_booker.rs:85; examples/seven_guis/src/tasks/flight_booker.rs:93), `crud-list` takes `role="listbox"` (examples/seven_guis/src/tasks/crud.rs:58) and each CRUD row `role="option"` with `aria-selected` (examples/seven_guis/src/tasks/crud.rs:71-72) — adding no element, class, style, key, id or order, the three classes that style those states staying; the windowed stand not booted.
    sidecar: >-
      2026-10-10-snapshot-states — Primary screens, seven_guis stand: recorded the flight booker's `aria-pressed` mode buttons and `aria-invalid` date inputs and the CRUD list's `listbox` / `option` / `aria-selected` roles as attribute-only markup (no element, class, style, key, id or order changed).
    rationale: >-
      The report's Changes → Symbols / APIs → "Stand markup, attributes only" bullet adds role and state attributes to two regions this section already describes at attribute level (ids, `for`, `aria-label`), and its Expected amendments item 12 names this exact section as carried. The body at line 10 states none of them. No new element or region was added, so this is a re-characterised existing region rather than a wholly new surface — warning, one additive sentence, no wireframe entry. Nothing is retired and there is no dependent occurrence: the stand's markup is restated only at line 45 (IA notes, the `crud-list` nested scroller), which stays true. The `escher-session` entry of §Surface: cli is untouched. The existing citations on line 10 are the citation sweep's.
    basis: "examples/seven_guis/src/tasks/flight_booker.rs:69 · :76 · :85 · :93; examples/seven_guis/src/tasks/crud.rs:58 · :71-72 (report, New text by line)"
```

## obs-plan — 4 proposals

Verdict: 4 proposals, all `D-obs-stack`, all on the one bullet at obs-plan:70. The return carried three YAML comment
lines above the list: `D-obs-instrumentation` clean (the builder's reads carry no log site by design; the command span
unchanged at eight fields); `D-obs-pii` clean (booleans and a closed set of bare words; `host_log` at 0 id needles, 0
name needles, 0 typed text; expected amendment 11 not carried — 742 B and 860 B did not move); `D-obs-stack` finds no
off-spec logger, only the stale census.

```yaml
proposals:
  - detector: D-obs-stack          # O1
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (subscriber installation) — the headless-stand bullet (obs-plan:70)"
    change: "In the no-subscriber census, read 'for the twelve driver-action checks `stand_act_*` (the two children of `stand_act_spans` aside)' in place of 'the eleven driver-action checks', and add to the bullet's closing provenance list 'the twelfth `stand_act_*` check, `stand_act_states`, at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md'."
    sidecar: "2026-10-10-snapshot-states: §3 no-subscriber census — driver-action checks `stand_act_*` eleven → twelve (`stand_act_states` added; no subscriber, env read or print in it)."
    rationale: "Report Counts row 'Driver-action checks `stand_act_*` | 11 → 12 files' names obs-plan:70 as a master site that says 'eleven'; Expected amendment 10 is carried; the Outcome's (obs) criterion reads no subscriber, env read or print in a new check — met, 0 hits of eight patterns over the two new files."
    basis: "the report's Counts row 'Driver-action checks'; tests/blitz-tests/tests/stand_act_states.rs (new file, 158 lines); obs-plan:70"
  - detector: D-obs-stack          # O2
    severity: warning
    section: "the same bullet (obs-plan:70)"
    change: "The shared-module clause reads 'the module seventeen of the eighteen checks that hold a session read — four of the five `stand_session_*`, `stand_settle` and the twelve `stand_act_*` — `tests/blitz-tests/tests/session_common/mod.rs`' in place of 'sixteen of the seventeen … and the eleven `stand_act_*`'."
    sidecar: "2026-10-10-snapshot-states: §3 no-subscriber census — readers of `session_common` sixteen → seventeen, checks holding a session seventeen → eighteen, the `stand_act_*` term eleven → twelve."
    rationale: "Second occurrence of the retired 'eleven `stand_act_*`' claim, in the same bullet, with the two counts built on it. Report Counts row 'Readers of `mod session_common;` | 16 → 17'. 'Eighteen' is not a figure the report states: it is the doc's own sum. That the seventeenth reader is `stand_act_states` is inferred, not stated — confirm with one grep before applying."
    basis: "the report's Counts rows 'Readers of `mod session_common;`' and 'Driver-action checks'; obs-plan:70"
    dependent-of: D-obs-stack
  - detector: D-obs-stack          # O3
    severity: warning
    section: "the same bullet (obs-plan:70)"
    change: "After the three scrolled-box checks, the census also names, since 2026-10-10-snapshot-states, the engine's accessibility-state check `accessibility_states` — no subscriber, no env read and no print in it — and the closing provenance list gains '`accessibility_states` at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md'."
    sidecar: "2026-10-10-snapshot-states: §3 no-subscriber census gains the engine check `accessibility_states` (no subscriber, env read or print)."
    rationale: "Expected amendment 10 — the two new check files in the no-subscriber census — carried. The census lists engine checks by name and the new one is absent from it. The Outcome's (obs) criterion is met: 0 hits of eight patterns over the two new files. The report does not say whether the file declares a shared module, so the change asserts nothing on that."
    basis: "the report's Expected amendments item 10 and the Outcome's (obs) criterion; tests/blitz-tests/tests/accessibility_states.rs (new file, 220 lines)"
  - detector: D-obs-stack          # O4
    severity: warning
    section: "the same bullet (obs-plan:70)"
    change: "The opening parenthesis reads 'driven in-process by the `stand_*` checks, 17 of which read the shared module `tests/blitz-tests/tests/common/mod.rs`' in place of '15 of which'."
    sidecar: "2026-10-10-snapshot-states: §3 — `stand_*` readers of `common/mod.rs` corrected 15 → 17 (disproved by measurement; the count predates this chunk, which added no reader)."
    rationale: "Report 'Spec claims disproved by measurement' item 3 names obs-plan:70: 17 `stand_*.rs` files declare `mod common;`, at the base and now. Counts row 'Readers of `mod common;` | 17 → 17'. A separate claim from the 'eleven' one, so its own primary. Swept: obs-plan has no other occurrence of it — the other '15 of 15' hits (obs-plan:294) are id needles."
    basis: "the report's Spec claims disproved item 3; obs-plan:70"
```

## a11y-plan — 8 proposals

Verdict: 8 proposals, all `D-a11y-surface` (one primary, seven `dependent-of`). The return carried three YAML comment
lines above the list: `D-a11y-obs-schema` no drift (no violation schema and no log schema moved); `D-a11y-surface`
drift — no element was added, but attributes turned the CRUD list and its rows into a listbox and its options, an
interactive role surface the report's Coverage marks keyboard ✗ and name ✗, and the plan states none of it; the keyed
contract (bootstrap phases) is untouched by any Change.

```yaml
proposals:
  - detector: D-a11y-surface          # A1 (primary)
    severity: warning
    section: "§7 Screen Reader Support → Accessibility tree output"
    change: "Add a bullet: on the stand the CRUD list states `role=\"listbox\"` and each person row `role=\"option\"` with `aria-selected`, reading `ListBox` / `ListBoxOption` in the tree, with the actionable-key check still returning nothing; two gaps stand open — a row is not focusable, so the stand shows a listbox option no keyboard reaches (owner: route entry \"Stand keyboard harness\"), and the list reads the accessible name `\"\"` (owner: route entry \"Stand a11y assertions\"); the flight booker's two mode buttons state `aria-pressed` with role Button and the focus order unchanged, and its two date inputs state `aria-invalid` (examples/seven_guis/src/tasks/crud.rs:58; examples/seven_guis/src/tasks/crud.rs:71-72; examples/seven_guis/src/tasks/flight_booker.rs:69; examples/seven_guis/src/tasks/flight_booker.rs:76; examples/seven_guis/src/tasks/flight_booker.rs:85; examples/seven_guis/src/tasks/flight_booker.rs:93; tests/blitz-tests/tests/stand_snapshot_state.rs:377-397; tests/blitz-tests/tests/stand_snapshot_state.rs:399-449; tests/blitz-tests/tests/stand_snapshot_state.rs:451-486) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md"
    sidecar: "2026-10-10-snapshot-states: §7 records the stand's listbox/option, toggle-button and aria-invalid markup, with the two open gaps (an option no keyboard reaches; the listbox unnamed) and their route owners."
    rationale: "Report Coverage of new surfaces, last row: the CRUD list as `listbox` and rows as `option` with `aria-selected` — roles ✓, actionable-key check ✓, keyboard ✗ (a row is not focusable), name ✗ (the list reads the accessible name \"\"). An interactive role surface without the §5 keyboard and §7 name coverage is the detector's drift; Expected amendment 7 names this site as carried."
    basis: "the report's Coverage last row and its Stand markup bullet; a11y-plan §7 — 0 hits for `no keyboard reaches`"
  - detector: D-a11y-surface          # A2
    severity: warning
    section: "§5 Keyboard Navigation → Per-surface keyboard shortcuts"
    change: "Add a bullet: seven_guis CRUD — the person rows are `option`s of a `listbox` by `role` attribute and are not focusable, so no keyboard reaches an option; the gap is recorded, not closed, and is owned by the route entry \"Stand keyboard harness\" (examples/seven_guis/src/tasks/crud.rs:58; examples/seven_guis/src/tasks/crud.rs:71-72) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md"
    sidecar: "2026-10-10-snapshot-states: §5 records the CRUD listbox option as unreachable by keyboard, owner \"Stand keyboard harness\"."
    rationale: "Same Coverage row: keyboard ✗ on the listbox option, decided at P4 (`inputs#I3`), owner the route entry \"Stand keyboard harness\"; Decisions & corrections lists the CARRY on that entry as owed. §5 is where the plan states per-surface keyboard reach and it is silent on it."
    basis: "the report's Coverage last row; its route-items line"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface          # A3
    severity: warning
    section: "§7 Screen Reader Support → Accessibility tree output"
    change: "In the bullet 'In a Dioxus document every element node carries its stable element id…', after 'the 15 interactive controls carry an HTML-AAM role and a non-empty accessible name' add: those 15 read as before (`stand_accessibility_ids` unedited), and beside them the CRUD list now carries an interactive role by its `role` attribute — `ListBox` — with the accessible name `\"\"`, and its rows `ListBoxOption`; so not every interactive-role element on the stand is named (examples/seven_guis/src/tasks/crud.rs:58; examples/seven_guis/src/tasks/crud.rs:71-72) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md"
    sidecar: "2026-10-10-snapshot-states: §7 qualifies the '15 interactive controls … non-empty accessible name' claim — the CRUD listbox is an interactive-role element outside the 15 and reads no name."
    rationale: "The bullet's claim reads as 'the stand's interactive elements are named'. Report Coverage: the list reads the accessible name \"\" (measured through the binary) while the roles/names checks pass on unedited files, so the list sits outside the 15 and the claim needs its bound stated."
    basis: "a11y-plan.md:270; the report's Coverage last row and its (a11y) Outcome criterion"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface          # A4
    severity: warning
    section: "§7 Screen Reader Support → Accessibility tree output"
    change: "In the actionable-key bullet, replace 'CRUD's person rows reading `crud-person-{person.id}` and Home's seven cards `task-card-{slug}`, keyed by an `id` attribute alone with no `tabindex`, role, focusability or Tab-order change' so that it holds only of Home's cards and of the rows' keying: since 2026-10-10-snapshot-states each CRUD row also carries `role=\"option\"` and `aria-selected` and the list `role=\"listbox\"` — a row now reads `ListBoxOption`, still with no `tabindex`, no focusability and no Tab-order change, and the check still returns nothing (`stand_actionable_keys` unedited) (examples/seven_guis/src/tasks/crud.rs:58; examples/seven_guis/src/tasks/crud.rs:71-72) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md"
    sidecar: "2026-10-10-snapshot-states: §7 actionable-key bullet no longer says the CRUD rows carry no role — they read `ListBoxOption`; the check's result is unchanged."
    rationale: "The bullet asserts the CRUD rows were keyed 'with no … role … change'. Report Symbols / APIs: each row takes `role=\"option\"` and `aria-selected`, `crud-list` takes `role=\"listbox\"`; Coverage: the actionable-key check still returns nothing; Decisions: a row now reads `ListBoxOption`. The no-role wording is retired for the rows."
    basis: "a11y-plan.md:272; the report's Stand markup bullet and Coverage last row"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface          # A5
    severity: warning
    section: "§7 Screen Reader Support → Accessibility tree output"
    change: "In the snapshot-model bullet, beside 'the tree gives an element node no value, checked or disabled property, so the snapshot reads those three from the element', state the other half: the tree does give an element node `selected`, `toggled` and `invalid` (the §2 bullet), and the snapshot takes those three from the tree, never from the element — `NodeState` gains `selected: Option<bool>`, `pressed: Option<bool>` and `invalid: bool`, seven fields in the order `enabled` · `checked` · `value` · `selected` · `pressed` · `invalid` · `focused`, with two limits of the boolean model: a mixed toggle reads `pressed` false and a grammar or spelling error reads invalid; `Snapshot::to_text` writes `selected` | `unselected`, `pressed` | `unpressed` and `invalid` after `value=` and before `focused`, and `Snapshot::diff` names a node whose one such reading moved as `changed`; on the stand each reading is asserted equal to its accessibility node's, in both layout modes — exactly the active flight mode reads pressed, exactly the selected row reads selected, a date that is no date reads invalid (packages/dioxus-native-dom/src/snapshot.rs:59-69; packages/dioxus-native-dom/src/snapshot.rs:159-161; packages/dioxus-native-dom/src/snapshot_text.rs:71-83; tests/blitz-tests/tests/stand_snapshot_state.rs:80-107; tests/blitz-tests/tests/stand_snapshot_state.rs:377-397; tests/blitz-tests/tests/stand_snapshot_state.rs:399-449; tests/blitz-tests/tests/stand_snapshot_state.rs:451-486) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md"
    sidecar: "2026-10-10-snapshot-states: §7 snapshot bullet records that the snapshot takes selected / pressed / invalid from the accessibility tree, with the two boolean-model limits and the text tokens."
    rationale: "The bullet enumerates what the tree does and does not give an element node and what the snapshot reads from where; it now understates the tree. Report Symbols / APIs: `NodeState` gains three public fields read from the accessibility node at the one construction site, `to_text` writes three more state tokens, `diff` unchanged in code; Coverage row 3: each reading asserted equal to the accessibility node's; Outcome v010-17 gives the test ranges. Expected amendment 7 — carried."
    basis: "a11y-plan.md:271; the report's Symbols / APIs bullets on `NodeState`, `to_text` and `diff`, its Coverage row 3 and its v010-17 Outcome"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface          # A6
    severity: warning
    section: "§2 A11y Strategy → Role derivation (semantic HTML and ARIA)"
    change: "Add a bullet: the builder writes three states on an element node from its ARIA state attributes, on every element whatever its role and with no role consulted — the value trimmed of ASCII whitespace and matched exactly in lower case: `aria-selected` `true` / `false` → `selected`, anything else nothing; `aria-pressed` `true` / `false` / `mixed` → `toggled` True / False / Mixed, anything else nothing; `aria-invalid` `true` / `grammar` / `spelling` → `invalid` True / Grammar / Spelling, absent, empty or `false` nothing, any other value `Invalid::True` (WAI-ARIA 1.2's rule; the lower-case match is a choice, so `TRUE` reads unrecognized); `aria-checked` is not read (packages/blitz-dom/src/accessibility.rs:102-129; tests/blitz-tests/tests/accessibility_states.rs:128-145; tests/blitz-tests/tests/accessibility_states.rs:147-165; tests/blitz-tests/tests/accessibility_states.rs:167-186; tests/blitz-tests/tests/accessibility_states.rs:188-220) — ratified by the founder (2026-10-10, relayed by the overseer): the builder writes the three states on the node; what an assistive technology reads of them is unmeasured — no stand binary builds a platform adapter and the dev host's AT-SPI bus is off — and the windowed witness is owed on the route's \"Stand a11y assertions\" — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md"
    sidecar: "2026-10-10-snapshot-states: §2 records the builder's three ARIA state reads (aria-selected / aria-pressed / aria-invalid by value; aria-checked unread), the founder's ratification and the owed windowed witness."
    rationale: "§2 states how the tree is derived (role from `role`, else HTML-AAM) and is silent on state. Report Symbols / APIs: `build_accessibility_node` reads three state attributes with the value grammar quoted; Coverage row 2: the adapter crossing is tests ✗ / a11y `unrunnable-here`; Cross-project: fork 2 answered option A (`inputs#I6`); the WAI-ARIA line states the case choice. Expected amendment 7 names §2 as carried."
    basis: "the report's builder bullet, its WAI-ARIA and founder's-word lines, its Coverage row 2"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface          # A7
    severity: warning
    section: "§4 ARIA Patterns & Roles → Per-component pattern catalog (engine mapping)"
    change: "Add a bullet after 'ARIA `role` attribute': **ARIA state attributes** — `aria-selected` → `selected`, `aria-pressed` → `toggled` (a toggle button; `mixed` kept as Mixed), `aria-invalid` → `invalid` (True / Grammar / Spelling), read by value on any element; `aria-checked` sets none of the three, so an ARIA checkbox or switch states no checked reading through it (packages/blitz-dom/src/accessibility.rs:102-129; tests/blitz-tests/tests/accessibility_states.rs:128-145; tests/blitz-tests/tests/accessibility_states.rs:147-165; tests/blitz-tests/tests/accessibility_states.rs:167-186; tests/blitz-tests/tests/accessibility_states.rs:188-220) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md"
    sidecar: "2026-10-10-snapshot-states: §4 pattern catalog gains the three ARIA state attributes and the unread `aria-checked`."
    rationale: "The catalog lists what the engine maps (role attribute, hidden content) and holds 0 hits for `aria-(selected|pressed|invalid|checked)`. Report Symbols / APIs gives the mapping and that `aria-checked` is not read; Outcome (a11y): `accessibility_states.rs` asserts every value of each grammar and `aria-checked` setting nothing; Decisions lists the ARIA checkbox/switch reading as a CARRY on \"Stand requirement sweep\". Expected amendment 8 — carried."
    basis: "the report's builder bullet, its expected amendment 8 and its (a11y) Outcome criterion"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface          # A8
    severity: warning
    section: "§8 Cognitive Accessibility → Error recovery"
    change: "In the bullet 'Invalid dates get an `invalid` class with red styling and the Book button disables', add that since 2026-10-10-snapshot-states the two date inputs `flight-start` and `flight-return-date` also state `aria-invalid`, so the invalid state is in the accessibility tree and the snapshot and not carried by the class alone; the class and its styling stay, and the non-colour cue check passes unedited beside it (examples/seven_guis/src/tasks/flight_booker.rs:85; examples/seven_guis/src/tasks/flight_booker.rs:93; tests/blitz-tests/tests/stand_snapshot_state.rs:451-486) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md"
    sidecar: "2026-10-10-snapshot-states: §8 error-recovery bullet records `aria-invalid` on the two flight-booker date inputs beside the `invalid` class."
    rationale: "The bullet names the mechanism by which an invalid date is conveyed (a class, red styling, a disabled button) and omits the one this chunk added. Report Symbols / APIs: `flight-start` and `flight-return-date` state `aria-invalid`, the three classes that style the states stay; Coverage: the non-colour invalid cue check passes beside it (`stand_flight_booker` unedited)."
    basis: "a11y-plan.md:315; the report's Stand markup bullet and its date-inputs Coverage row"
    dependent-of: D-a11y-surface
```

Note on the record: each proposal's `basis` and `rationale` above name the report's bullets where the return gave
`report.md:{n}` line numbers; the return's numbers pointed at the report as it read after the resume's edits and are
not coordinates of the tree. `change` and `sidecar` are the return's text.

## security-plan — 7 proposals

Verdict: 7 proposals, all `D-security-input` (one primary, six `dependent-of`), each graded `escalate` by the
detector's own severity. The return carried three YAML comment lines above the list: `D-security-auth` clean (no
identity, session, token or key code touched; `wire.rs`, `host.rs`, `client.rs`, `session.rs` unedited; no socket,
port or env var added); `D-security-deps` clean (Dependencies: none added or bumped); `D-security-input` — the new
markup boundary is validated in code (closed-set match, no panic path) but §Input Validation records neither it nor
the two crossings it reaches.

```yaml
proposals:
  - detector: D-security-input          # S1 (primary)
    severity: escalate
    section: "§Input Validation → Markup attributes, a new row `aria-selected` · `aria-pressed` · `aria-invalid` (control states), beside the `aria-label` row"
    change: >-
      New row — Markup attributes | `aria-selected` · `aria-pressed` · `aria-invalid` (control states) | Read from parsed content — the browser app's remote HTML included — into the accessibility tree by the in-process builder `BaseDocument::build_accessibility_node`, on every element whatever its role; the parser already admits the attributes, so no new input class enters; each value is trimmed of ASCII whitespace and compared exactly, in lower case, against a closed set: `aria-selected` `true` / `false` → AccessKit `selected` true / false, anything else nothing; `aria-pressed` `true` · `false` · `mixed` → `toggled` `True` · `False` · `Mixed`, anything else nothing; `aria-invalid` `true` · `grammar` · `spelling` → `invalid` `True` · `Grammar` · `Spelling`, absent, empty after trimming or `false` → nothing, and any other value → `Invalid::True` (WAI-ARIA 1.2's rule); lower-case exact match is escher's choice — the ARIA page states no case rule — so `TRUE` reads unrecognized (nothing for the first two, invalid for the third); `aria-checked` is not read and no role is consulted; the added lines hold no log site and no panic path, a value of 5,500 characters builds the tree, and neither an unrecognized word nor the long value is written anywhere (the tree's `Debug` holds neither); two exits, both closed enums or booleans and never the attribute's text: (1) the platform accessibility adapter, whose nodes now carry `selected` / `toggled` / `invalid` — ratified by the founder (2026-10-10, by question dialog in the overseer session, relayed by the overseer: the builder writes the three states on the node); what an assistive technology reads is unmeasured until the windowed check (no stand binary builds an adapter and the dev host's AT-SPI bus is off); (2) the snapshot — `NodeState.selected: Option<bool>`, `.pressed: Option<bool>` (only `Toggled::True` reads `Some(true)`; `False` and `Mixed` read `Some(false)`) and `.invalid: bool` (a grammar or spelling error reads invalid) — which `Snapshot::to_text` writes as the bare words `selected` | `unselected`, `pressed` | `unpressed`, `invalid` and `Outcome::to_json` as booleans, crossing the session socket and the client's stdout inside the already ratified carriers (the Driver session socket row); the password and file mask is untouched (packages/blitz-dom/src/accessibility.rs:102-129; packages/dioxus-native-dom/src/snapshot.rs:59-69; packages/dioxus-native-dom/src/snapshot.rs:159-161; packages/dioxus-native-dom/src/snapshot_text.rs:71-83; packages/escher-driver/src/json.rs:129-135; tests/blitz-tests/tests/accessibility_states.rs:128-220) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md
    sidecar: "2026-10-10-snapshot-states — §Input Validation gains the Markup attributes row for `aria-selected` · `aria-pressed` · `aria-invalid`: closed-set match by value, no panic path, two exits (platform adapter, snapshot), the adapter exit ratified by the founder 2026-10-10"
    rationale: >-
      Report Changes → Symbols / APIs: the accessibility builder reads three state attributes on every element, markup from any document; Coverage of new surfaces, first bullet: validation closed-set match by value, no panic path. The mechanism §Input Validation asks of a markup attribute is present, but the section has no row for the boundary — the report measures `aria-(selected|pressed|invalid|checked)` at 0 hits in the seven masters and lists this as Expected amendment 6, carried. The plan asked for it as PROVISIONAL; at the wrap's resume the founder's answers are in (Cross-project, inputs#I6: fork 2 = ratify), and the report leaves the record to Validate on a halt and the operator's one confirm (inputs#I7) — so the row is worded as ratified and belongs on that halt card.
    basis: packages/blitz-dom/src/accessibility.rs:102-129
  - detector: D-security-input          # S2
    severity: escalate
    section: "§Input Validation → Driver session socket (escher-driver) row"
    change: >-
      After "carries what the screen reads: ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff" add: since 2026-10-10-snapshot-states those two carriers also hold three state readings — the snapshot's text up to five more bare words of a closed set (`selected` | `unselected`, `pressed` | `unpressed`, `invalid`), a diff node up to three more keys (`selected` and `pressed` written only where present, `invalid` always written, each a boolean; twelve node keys in the schema's order) — never an attribute's text; the wire stays `v2`, with no verb, argument kind, refusal cause, socket or port added and the request grammar and bounds as they were; inside the crossing ratified at 2026-10-10-driver-cli and not a further widening (the founder, 2026-10-10, by question dialog in the overseer session, relayed by the overseer) (packages/escher-driver/src/schema.rs:32-37; packages/escher-driver/src/json.rs:129-135) — the states as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md
    sidecar: "2026-10-10-snapshot-states — Driver session socket row: an answer's text and diff now carry three state readings (closed words / booleans), inside the ratified crossing per the founder 2026-10-10; wire stays v2"
    rationale: >-
      The row at security-plan:74 enumerates what an answer carries and stops at names and control values. Report Changes → Symbols / APIs ("Not added"): what a `v2` answer carries grew inside two existing carriers — a diff node up to three more keys, a snapshot's text up to five more words; Expected amendment 5 names this row, carried. The founder's fork 1 answer (inputs#I6) = inside the ratified crossing; the record is Validate's on the halt (inputs#I7).
    basis: packages/escher-driver/src/json.rs:129-135
    dependent-of: D-security-input
  - detector: D-security-input          # S3
    severity: escalate
    section: "§Input Validation → Markup attributes, the `id` (stable element id) row"
    change: >-
      Where the row says the 2026-10-07 refresh is "the same crossing and the same content classes (ids as `author_id`, accessible names), more often", add: since 2026-10-10-snapshot-states the platform-adapter crossing carries a third content class, control states — `selected`, `toggled` and `invalid` on each element node whose markup states them, closed enums and bools, never an attribute's text (the `aria-selected` · `aria-pressed` · `aria-invalid` row) — ratified by the founder (2026-10-10, by question dialog in the overseer session, relayed by the overseer); its windowed witness is owed like the refresh's (no test at the adapter on the dev host).
    sidecar: "2026-10-10-snapshot-states — `id` row: the platform-adapter crossing's content classes are now three (ids, accessible names, control states), ratified by the founder 2026-10-10"
    rationale: >-
      security-plan:118 names the adapter crossing's content classes as two. Report Coverage: `selected` / `toggled` / `invalid` on accessibility nodes handed to the platform adapter — PII closed enums and bools, tests ✗ at the adapter (`unrunnable-here`); Symbols / APIs: the output of the public `build_accessibility_tree` changes for every document and feeds the shell's `update_tree` and `handle_blitz_shell_event`. Expected amendment 6 names the adapter's content classes at security-plan:118. Founder's fork 2 = ratify (inputs#I6).
    basis: packages/blitz-dom/src/accessibility.rs:102-129
    dependent-of: D-security-input
  - detector: D-security-input          # S4
    severity: escalate
    section: "§Threat Model Summary → Attack surface, the vector `local IPC — the driver session's socket`, its Trust boundary bullet"
    change: >-
      After "an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out" add: — the text and the diff holding, since 2026-10-10-snapshot-states, three state readings per control (selected, pressed, invalid) as closed words and booleans, inside the ratified crossing (the founder, 2026-10-10, relayed by the overseer; as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md).
    sidecar: "2026-10-10-snapshot-states — Threat Model, local IPC trust boundary: the answer's text and diff now hold three state readings, inside the ratified crossing"
    rationale: >-
      The Threat Model restates the socket row's content list word for word, so it would survive a single-site apply. Same basis as S2. The report gives no line for this site; it is found by its wording "an answer takes ids, accessible names, control values".
    basis: packages/escher-driver/src/json.rs:129-135
    dependent-of: D-security-input
  - detector: D-security-input          # S5
    severity: escalate
    section: "§Input Validation → CLI arguments (escher-session) row"
    change: >-
      "an answer on stdout holds ids, names and values by the ratified crossing (the Driver session socket row)" becomes "an answer on stdout holds ids, names, values and, since 2026-10-10-snapshot-states, the three state readings (closed words in the snapshot's text, booleans in a diff node) by the ratified crossing (the Driver session socket row)"; the argv reader itself is unchanged — no verb, flag or argument kind added.
    sidecar: "2026-10-10-snapshot-states — CLI arguments (escher-session) row: stdout answers now also hold the three state readings"
    rationale: >-
      Third statement of the same content list. Report Coverage: the diff node's keys `selected` `pressed` `invalid`, crossing the session socket and the command line (tests `cli_commands`, `cli_flow`, and by hand through the binary); `cli.rs` and `command.rs` are unedited per "Not added". Found by its wording "holds ids, names and values".
    basis: packages/escher-driver/src/json.rs:129-135
    dependent-of: D-security-input
  - detector: D-security-input          # S6
    severity: escalate
    section: "§Input Validation → Driver command schema (escher-driver) row"
    change: >-
      "`Outcome` derives it too, printing the ids, accessible names and control values of its diff (a password's or a file input's value as the mask)" gains "and, since 2026-10-10-snapshot-states, each diff node's three state readings (`selected`, `pressed`, `invalid` — two optional booleans and a boolean)"; and beside the span's eight fields: the span is unchanged at eight fields with no field for a state (`stand_act_spans`; `host_log` under both builds). The verb table stays nine verbs, six argument kinds and eight causes.
    sidecar: "2026-10-10-snapshot-states — Driver command schema row: `Outcome`'s `Debug` and written form now include the three state readings; the command span stays at eight fields, none for a state"
    rationale: >-
      Fourth statement of what an outcome's diff holds — in process (`Debug`), no boundary of its own, listed so the enumeration does not stay at three classes in one row and four in the next. Report Symbols / APIs: `NodeState` gains three public fields (still `Debug`); Coverage: the command span unchanged at eight fields, no field for a state; Outcome (arch): nine verbs, six argument kinds, eight causes — met. Found by its wording "printing the ids, accessible names and control values of its diff".
    basis: packages/dioxus-native-dom/src/snapshot.rs:59-69
    dependent-of: D-security-input
  - detector: D-security-input          # S7
    severity: escalate
    section: "§Error Handling → Error format (typed errors), the escher-driver command refusal item"
    change: >-
      "the help of `changed` and of `snapshot`'s `text` read as they did before 2026-10-10-driver-cli and state no limit" becomes: at 2026-10-10-snapshot-states those two help texts were restated — each now names the state words and says the two limits of the boolean model (a mixed toggle reads unpressed / `pressed` false; a grammar or spelling error reads invalid) — each still a fixed literal of the verb table holding nothing a call supplies, the cause set staying eight in its order; the item's two `schema.rs` citations for them move to packages/escher-driver/src/schema.rs:203-205 (`changed`) and packages/escher-driver/src/schema.rs:232-236 (`text`) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md
    sidecar: "2026-10-10-snapshot-states — refusal item: the help of `changed` and of `snapshot`'s `text` restated to name the three states and the boolean model's two limits; still fixed strings"
    rationale: >-
      Not a boundary: a served description of the same two carriers, and the one place security-plan says those help texts "state no limit" — false after this chunk. Report Symbols / APIs: two help texts of `VERBS` restated — `snapshot`'s `text` (`schema.rs:232-236`) and the acting verbs' `changed` (`:203-205`). Found by its wording "state no limit". If the orchestrator holds this outside D-security-input's invariant it can be applied at warning weight — the founder's two fork answers do not bear on it.
    basis: packages/escher-driver/src/schema.rs:203-205
    dependent-of: D-security-input
```

## test-plan — 16 proposals

Verdict: 16 proposals, all `D-tests-coverage` (T1 the primary of a seven-member `dependent-of` group on the retired
"eleven `stand_act_*`" claim; T9 to T16 each its own primary). The return carried comment lines above the list:
`D-tests-framework` no drift (the gates run `cargo test … --locked`, `scripts/agent-run.sh` and
`.github/scripts/ci-leg.sh` only; no dependency; the two new targets auto-discovered); `D-tests-obs-harness` no drift
(harness and gate surface unchanged; the host-log figures did not move) with a cross-doc note that obs-plan:70 states
the same "eleven" census and must move in the same pass; `D-tests-coverage` — every new code path carries a test at
the tier the plan's pattern uses, the drift being the plan's own stated counts and census plus one surface with no
test (the platform adapter). `change`, `sidecar` and `basis` below are the return's text; each `rationale` is cut to
its facts.

```yaml
proposals:
  - detector: D-tests-coverage          # T1 (primary of the "eleven" group)
    severity: warning
    section: "§3 → Session lifecycle"
    change: >-
      In the `session-proof` row, "the eleven driver-action files, `Session::run` in each, both layout modes" becomes "the twelve driver-action files"; the per-file list gains "· `stand_act_states` 3" after "`stand_act_filled` 4"; the total "(37; 29 over ten files until 2026-10-10-driver-cli)" becomes "(40; 37 over eleven files until 2026-10-10-snapshot-states, 29 over ten until 2026-10-10-driver-cli)"; the provenance gains "the twelfth driver-action file at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md"; the CI witness list gains "and its run 38072258834 on `2fc32829` (16 of 16 checks green, wall 767 s, read by conclusion) of the twelfth driver-action file on them". "escher-driver's 50 unit tests" and its per-file split stay as they read.
    sidecar: "2026-10-10-snapshot-states — session-proof: driver-action files eleven → twelve (`stand_act_states` 3; total 37 → 40), fork CI#38072258834 on `2fc32829` added as witness."
    rationale: "Counts row 'Driver-action checks 11 → 12 files' names this key file; Expected amendment 9: the twelfth, `stand_act_states`, holds 3 tests, `Session::run` in each, both layout modes. 2+6+4+3+2+1+2+5+6+2+4 = 37 as the row reads, +3 = 40. CI from Cross-project. escher-driver unit tests 50 → 50."
    basis: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
  - detector: D-tests-coverage          # T2
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: >-
      "`tests/session_common/mod.rs`, read through `mod session_common;` by 16 checks — four `stand_session_*.rs`, `stand_settle.rs`, which declares it and not `mod common;`, and the eleven `stand_act_*.rs`, six of which (…) declare both" becomes "by 17 checks — four `stand_session_*.rs`, `stand_settle.rs`, …, and the twelve `stand_act_*.rs`, six of which (`stand_act_ids`, `stand_act_diff`, `stand_act_keys`, `stand_act_spans`, `stand_act_obstructed`, `stand_act_filled`) declare both". "read through `mod common;` by 17 `stand_*.rs` checks" stays as it reads.
    sidecar: "2026-10-10-snapshot-states — Directory pattern: `mod session_common;` readers 16 → 17, `stand_act_*.rs` eleven → twelve (`stand_act_states`, which declares `session_common` and not `common`)."
    rationale: "Counts rows 'Readers of `mod session_common;` 16 → 17' and 'Driver-action checks 11 → 12', both naming test-plan:66; 'Readers of `mod common;` 17 → 17', so the list of six holds."
    basis: .andromeda/test-plan.md:66
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage          # T3
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (stand checks)"
    change: >-
      "the five `stand_session_*` checks and six of the eleven driver-action checks — … — do, 17 in all" becomes "six of the twelve driver-action checks — … — do, 17 in all" (the six names and the total of 17 unchanged; the twelfth, `stand_act_states`, does not declare `mod common;`).
    sidecar: "2026-10-10-snapshot-states — stand checks helper census: \"six of the eleven\" → \"six of the twelve\" driver-action checks; `mod common;` readers stay 17."
    rationale: "Counts row 'Driver-action checks' lists test-plan:100; 'Readers of `mod common;` 17 → 17' and Spec claims disproved item 3 confirm 17."
    basis: .andromeda/test-plan.md:100
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage          # T4
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (session checks)"
    change: >-
      "so do the eleven driver-action checks `stand_act_*` (16 readers)" becomes "so do the twelve driver-action checks `stand_act_*` (17 readers)"; the trailing provenance gains ", the seventeenth, `stand_act_states`, at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md". The historical clauses ("the eleven readers at …act-by-id", "the fourteen readers", "the fifteenth", "the sixteenth, `stand_act_filled`") stay as they read.
    sidecar: "2026-10-10-snapshot-states — session checks helper census: driver-action checks eleven → twelve, `mod session_common;` readers 16 → 17 (`stand_act_states`)."
    rationale: "Counts rows on test-plan:66, :100, :101. The new file's use of the shared module shows in New text: `stand_act_states.rs` holds a `Held` session and steps a `Call` through it."
    basis: .andromeda/test-plan.md:101
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage          # T5
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → escher-driver"
    change: >-
      "the executor's verbs and its refusals are covered by the eleven `stand_act_*` integration files, not by a unit test" becomes "by the twelve `stand_act_*` integration files". The count "50 inline unit tests in ten files" and the per-file split stay; optionally the schema.rs clause may say the node fields now number twelve and json.rs's that `invalid` is always written — both tests were restated, none added.
    sidecar: "2026-10-10-snapshot-states — §1 escher-driver: covering `stand_act_*` files eleven → twelve; unit-test count unchanged at 50."
    rationale: "Same retired claim; an occurrence on test-plan:21 the report's site list did not name. Counts row 'Driver-action checks 11 → 12'."
    basis: .andromeda/test-plan.md:21
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage          # T6
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → escher-driver"
    change: >-
      "while the eleven `stand_act_*` files drive it in process through a `Session` (§5)" becomes "while the twelve `stand_act_*` files drive it in process through a `Session` (§5)".
    sidecar: "2026-10-10-snapshot-states — §4 escher-driver: `stand_act_*` files that drive the schema in process eleven → twelve."
    rationale: "An occurrence of the retired claim the report's site list does not name."
    basis: ".andromeda/test-plan.md — §4, the escher-driver bullet (no line stated by the report)"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage          # T7
    severity: warning
    section: "§5 Integration Test Strategy → Boundaries covered → Session ↔ held instance"
    change: >-
      "(the eleven `stand_act_*` files, §1)" becomes "(the twelve `stand_act_*` files, §1)", and before it the bullet gains: "and since 2026-10-10-snapshot-states an acting call returns the node it changed with its `selected`, `pressed` or `invalid` reading as it reads after the step — a flight-mode click both mode buttons with their pressed reading, a row click the row selected and the row it leaves not selected, a typed start date the field invalid or not — each diff equal across the two layout modes (`stand_act_states`)"; the provenance gains "the three state readings at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md".
    sidecar: "2026-10-10-snapshot-states — §5 Session ↔ held instance: `stand_act_*` files eleven → twelve; the three state readings returned by an acting call recorded (`stand_act_states`)."
    rationale: "An occurrence the report's site list does not name. The new boundary reading is the report's Outcome for v010-17 and its Coverage row for the diff node's keys."
    basis: tests/blitz-tests/tests/stand_act_states.rs:79-158
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage          # T8
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: >-
      In the list of the driver's calls run through `Session::run`, after "the eleventh, since 2026-10-10-driver-cli, `stand_act_filled` 4 (…)" add "the twelfth, since 2026-10-10-snapshot-states, `stand_act_states` 3 (a flight-mode click returns both mode buttons with their pressed reading; a row click returns the row selected and the row it leaves not selected; a typed start date returns the field invalid or not — each in both layout modes, the two modes' diffs equal)"; add its citation `tests/blitz-tests/tests/stand_act_states.rs:79-158` to the bullet's citation list and "`stand_act_states` at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md" to its provenance. The ordinals "the tenth" and "the eleventh" stay.
    sidecar: "2026-10-10-snapshot-states — §1 tests/blitz-tests: the twelfth driver-action file `stand_act_states` (3) added to the per-file list."
    rationale: "The bullet enumerates every driver-action file with its count and stops at 'the eleventh'. Test names and ranges from New text, by line (:79-103, :105-134, :136-158)."
    basis: tests/blitz-tests/tests/stand_act_states.rs:79-158
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage          # T9
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: >-
      "106 `.rs` files, 105 test targets and the unbuilt `all.rs` (103 and 102 until 2026-10-10-scrolling-box-bounds-and-hit added …" becomes "108 `.rs` files, 107 test targets and the unbuilt `all.rs` (106 and 105 until 2026-10-10-snapshot-states added `accessibility_states.rs` and `stand_act_states.rs`, both auto-discovered with no manifest line; 103 and 102 until 2026-10-10-scrolling-box-bounds-and-hit added …". The later clause "no `all` executable among 107" is an executable count measured at the upstream-sync chunk; leave it as it reads — this chunk did not re-measure it.
    sidecar: "2026-10-10-snapshot-states — Directory pattern: `.rs` files directly under tests/blitz-tests/tests 106 → 108, test targets 105 → 107 (`accessibility_states`, `stand_act_states`)."
    rationale: "Counts row '`.rs` files directly under `tests/blitz-tests/tests/` 106 → 108'; Crates / modules: two new test targets, auto-discovered. The same row says the executable count was not re-measured, so no figure is proposed for it."
    basis: .andromeda/test-plan.md:66
  - detector: D-tests-coverage          # T10
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → dioxus-native-dom"
    change: >-
      "55 unit tests in eight files" becomes "60 unit tests in eight files"; "fourteen on the snapshot builder" becomes "seventeen on the snapshot builder", its list gaining "an `aria-selected` value reading selected, an `aria-pressed` value reading pressed while a checked one does not, an `aria-invalid` value reading invalid — each over a stated table of seven rows"; "seven on the snapshot's text form" becomes "eight", its "the state tokens and their fixed order" now covering `selected` | `unselected`, `pressed` | `unpressed` and `invalid` after `value=` and before `focused`; "eight on the snapshot's diff" becomes "nine", gaining "a `selected`, `pressed` or `invalid` reading that alone moved naming that node"; the provenance gains "the count of 60 and the five state tests at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md". The two boolean-model limits (a mixed toggle reads unpressed; a grammar or spelling error reads invalid) are pinned here and at the engine only — no stand screen shows either.
    sidecar: "2026-10-10-snapshot-states — §1 dioxus-native-dom: unit tests 55 → 60 (snapshot.rs 14 → 17, snapshot_text.rs 7 → 8, snapshot_diff.rs 8 → 9) for the `selected` · `pressed` · `invalid` readings."
    rationale: "Counts row 'dioxus-native-dom unit tests 55 → 60' with the per-file split, naming test-plan:24. Test names from New text, by line. Limits from Decisions ('no stand screen shows a mixed toggle')."
    basis: .andromeda/test-plan.md:24
  - detector: D-tests-coverage          # T11
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: >-
      "under `SNAPSHOT_TEXT_BUDGET` (10,000 bytes; the largest, Crud, reads 2034)" becomes "(10,000 bytes; the largest, Crud, reads 2049 since 2026-10-10-snapshot-states — 2034 before it)".
    sidecar: "2026-10-10-snapshot-states — §1 tests/blitz-tests: the largest snapshot text at boot, Crud, 2034 → 2049 bytes."
    rationale: "Counts row 'Text at boot' naming test-plan:26; Outcome: 2049 of 2304."
    basis: .andromeda/test-plan.md:26
  - detector: D-tests-coverage          # T12
    severity: warning
    section: "§6 E2E Test Strategy → Drivers per surface → Command line (`escher-session`)"
    change: >-
      "(832 · 1,375 · 1,368 · 2,187 bytes on the four tasks, line break included, by hand smoke)" becomes "(832 · 1,393 · 1,368 · 2,202 bytes on the four tasks, line break included, by hand smoke at 2026-10-10-snapshot-states; 832 · 1,375 · 1,368 · 2,187 before it)".
    sidecar: "2026-10-10-snapshot-states — §6 Command line: hosted `snapshot` line sizes 832 · 1,375 · 1,368 · 2,187 → 832 · 1,393 · 1,368 · 2,202 bytes."
    rationale: "Counts row 'Hosted `snapshot` line, four tasks' naming test-plan:200."
    basis: .andromeda/test-plan.md:200
  - detector: D-tests-coverage          # T13
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Proof"
    change: >-
      Append to the re-count history: "; re-counted at 2026-10-10-snapshot-states, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 125 · failed 0 · ignored 6 over 32 files — +6 passed (`stand_act_states` 3, a new file picked up by its `stand_` prefix with no script change, and `stand_snapshot_state` +3) — `accessibility_states` carries no `stand_` prefix and is not selected, and `run all` was not re-read — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md".
    sidecar: "2026-10-10-snapshot-states — Agent-run Proof: `run stand` 119 passed · 6 ignored · 31 files → 125 · 6 · 32, 0 failed."
    rationale: "Counts row '`run stand`' naming test-plan:116; the Gates table's `run.end`; Harness / gate surface: the script unedited. The +6 split is the report's own rows. A proof re-count, not a harness change."
    basis: .andromeda/test-plan.md:116
  - detector: D-tests-coverage          # T14
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Local baseline → `cargo test --workspace`"
    change: >-
      Append to the re-count history: "; re-counted at 2026-10-10-snapshot-states: 167 result lines, 791 passed · 0 failed · 11 ignored (+15 passed: two new result lines — blitz-tests' `accessibility_states` 4 and `stand_act_states` 3 — `dioxus-native-dom` +5 in its lib line (`snapshot` 3, `snapshot_text` 1, `snapshot_diff` 1) and `stand_snapshot_state` +3 in its existing line; escher-driver's lib line unmoved at 50), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run at /implement and in its operator pass on the tree its pre-CI commit 2fc32829 carries (escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md)".
    sidecar: "2026-10-10-snapshot-states — Local baseline: workspace test leg 165 result lines · 776 passed → 167 · 791, 0 failed, 11 ignored."
    rationale: "Counts row 'Workspace test leg' naming test-plan:329; the Gates rows for `ci-leg.sh fast`. The split sums the report's rows: 4 + 3 + 5 + 3 = +15."
    basis: .andromeda/test-plan.md:329
  - detector: D-tests-coverage          # T15
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: >-
      In the stand-snapshot-state passage ("each control's state proven through real input in both layout modes — …") add: "and, since 2026-10-10-snapshot-states, exactly the active flight mode reading pressed and the reading swapping with the mode, exactly the selected CRUD row reading selected across a move, a Delete and a filter edit, and a date that is no date reading invalid on both date inputs — no other node reading pressed, selected or invalid, and every node's three readings asserted equal to its accessibility node's (the file's `stated` helper; tests/blitz-tests/tests/stand_snapshot_state.rs:80-107, :377-486)"; and restate "(`a_textarea_reads_back_typed_text`, the file's eighth test, …" as "the eighth test the file gained, of eleven now" — the three new tests stand above it in the file.
    sidecar: "2026-10-10-snapshot-states — §1 tests/blitz-tests: `stand_snapshot_state` 8 → 11 tests (pressed, selected, invalid readings on the stand, each equal to the accessibility node's)."
    rationale: "Counts row '`stand_snapshot_state` tests 8 → 11'; Outcome v010-17 gives the ranges. The bullet describes the file's content test by test and holds none of the three."
    basis: tests/blitz-tests/tests/stand_snapshot_state.rs:377-486
  - detector: D-tests-coverage          # T16
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: >-
      Beside the `accessibility_names` clause add: "and, since 2026-10-10-snapshot-states, the accessibility builder's three state attributes read by value on a parsed page, in both layout modes — `accessibility_states` 4: `aria-selected` reads `true` and `false` and nothing else (9 rows), `aria-pressed` its three values and nothing else (10), `aria-invalid` its values and an unrecognized one as true (11), each on a role that supports the attribute and one that does not, row counts asserted, and `aria-checked` sets none of the three (tests/blitz-tests/tests/accessibility_states.rs:128-220) — run by the workspace `test` leg and per package, not by the `a11y` leg, which stays three files; blitz-dom holds no unit test of it; no check reads the three states as the platform adapter hands them to an assistive technology — no stand binary builds an adapter and the dev host's AT-SPI bus is off — a gap owned by the route entry \"Stand a11y assertions\" — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md".
    sidecar: "2026-10-10-snapshot-states — §1 tests/blitz-tests: new engine check `accessibility_states` (4) for `aria-selected` · `aria-pressed` · `aria-invalid`; the platform-adapter reading of the three states recorded as untested here."
    rationale: "The one new surface the report marks without a test: Coverage — tests ✗ at the adapter (`unrunnable-here`); Decisions lists the CARRY on 'Stand a11y assertions' for the windowed witness. The builder itself is covered (`accessibility_states`, 4 tests, both layout modes, both builds; row counts 9, 10 and 11 asserted). The `a11y` CI leg stays three named files."
    basis: tests/blitz-tests/tests/accessibility_states.rs:128-220
```

## architecture — 16 proposals

Verdict: 16 proposals — 15 `D-arch-resources` (R1 and R10 primaries, the rest `dependent-of`), 1 `D-arch-decisions`
(R16); R11 and R16 graded `escalate` by the detector because each writes the founder's word into the master. The
return carried no text outside the list. `change`, `sidecar` and `basis` below are the return's text; each
`rationale` is cut to its facts.

```yaml
proposals:
  - detector: D-arch-resources          # R1 (primary)
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      In the snapshot-model clause (architecture:136), replace "`NodeState` carries `enabled: Option<bool>`, `checked: Option<bool>`, `value: Option<String>` and `focused: bool`" with the seven fields in their order — `enabled: Option<bool>` · `checked: Option<bool>` · `value: Option<String>` · `selected: Option<bool>` · `pressed: Option<bool>` · `invalid: bool` · `focused: bool` — the three middle ones since 2026-10-10-snapshot-states (packages/dioxus-native-dom/src/snapshot.rs:59-69), no new public type and the crate-root re-export unedited.
    sidecar: "Dioxus DOM bridge: `NodeState` restated at seven fields (adds `selected`, `pressed`, `invalid`) — 2026-10-10-snapshot-states."
    rationale: "Symbols / APIs: `NodeState` gains three public fields, seven now in the stated order; Counts row '`NodeState` fields 4 → 7 … architecture:136 (names the four)'."
    basis: "packages/dioxus-native-dom/src/snapshot.rs:59-69"
  - detector: D-arch-resources          # R2
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      In the same clause's "every field is an existing reader's" list (architecture:136), between the `value` reading and "`focused` the `TreeUpdate`'s `focus`", add the three new readings, each read from the accessibility node at the one construction site: `selected` its `is_selected()`, `pressed` its `toggled()` with only `Toggled::True` reading `Some(true)` (`False` and `Mixed` read `Some(false)` — a mixed toggle reads not pressed), `invalid` its `invalid().is_some()` (a grammar or spelling error reads invalid) (packages/dioxus-native-dom/src/snapshot.rs:159-161).
    sidecar: "Dioxus DOM bridge: the per-field reader list gains `selected`, `pressed`, `invalid` and the two limits of the boolean model — 2026-10-10-snapshot-states."
    rationale: "Same retired claim (a node's state is four readings), restated in the reader enumeration; Symbols / APIs gives the three readers at `:159-161`."
    basis: "packages/dioxus-native-dom/src/snapshot.rs:159-161"
    dependent-of: D-arch-resources
  - detector: D-arch-resources          # R3
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      In the `Snapshot::to_text` clause (architecture:136), restate the fixed token order as `enabled` / `disabled` · `checked` / `unchecked` · `value=` with the value quoted · `selected` / `unselected` · `pressed` / `unpressed` · `invalid` · `focused` — `selected`/`unselected` and `pressed`/`unpressed` each only where the model holds a reading, `invalid` only when true (packages/dioxus-native-dom/src/snapshot_text.rs:71-83); still a pure function of the snapshot.
    sidecar: "Dioxus DOM bridge: `to_text` token order gains `selected`/`unselected`, `pressed`/`unpressed`, `invalid` after `value=` and before `focused` — 2026-10-10-snapshot-states."
    rationale: "Symbols / APIs: `Snapshot::to_text` writes three more state tokens after `value=` and before `focused`; Schema / config names the token grammar as changed."
    basis: "packages/dioxus-native-dom/src/snapshot_text.rs:71-83"
    dependent-of: D-arch-resources
  - detector: D-arch-resources          # R4
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      In the `to_text` clause (architecture:136), replace "on the stand each lean task's screen at boot reads Counter 755 · FlightBooker 1269 · Timer 1259 · Crud 2034 bytes" with Counter 755 · FlightBooker 1287 · Timer 1259 · Crud 2049 bytes, the same in both layout modes, on the dev host (Linux) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md (the earlier figures at the compact-snapshot-serialization report).
    sidecar: "Dioxus DOM bridge: boot text sizes FlightBooker 1269 → 1287, Crud 2034 → 2049 (Counter 755, Timer 1259 unchanged) — 2026-10-10-snapshot-states."
    rationale: "Counts row 'Text at boot' naming architecture:136; Outcome: 1287 of 1536, 2049 of 2304."
    basis: "escher-0.1.0/chunks/2026-10-10-snapshot-states/evidence/readings.md (§Sizes)"
    dependent-of: D-arch-resources
  - detector: D-arch-resources          # R5
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: >-
      In the `to_text` clause (architecture:136), replace "the hosted `snapshot` line, JSON and line break included, measures 832 · 1,375 · 1,368 · 2,187 bytes on the four lean tasks at boot" with 832 · 1,393 · 1,368 · 2,202 bytes, inside the budget — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md.
    sidecar: "Dioxus DOM bridge: hosted `snapshot` line 832 · 1,375 · 1,368 · 2,187 → 832 · 1,393 · 1,368 · 2,202 bytes — 2026-10-10-snapshot-states."
    rationale: "Counts row 'Hosted `snapshot` line, four tasks' naming architecture:136."
    basis: "escher-0.1.0/chunks/2026-10-10-snapshot-states/evidence/readings.md (§Sizes)"
    dependent-of: D-arch-resources
  - detector: D-arch-resources          # R6
    severity: warning
    section: "§Standard Contracts → Driver session"
    change: >-
      In the acting verbs' result-fields sentence (architecture:133), restate the fields `NODE_FIELDS` names as twelve — `id` · `parent` · `role` · `name` · `enabled` · `checked` · `value` · `selected` · `pressed` · `invalid` · `focused` · `bounds` — the constant now `[&str; 12]` where it was `[&str; 9]` (packages/escher-driver/src/schema.rs:32-37); no verb, argument kind, field kind, refusal cause or `SessionError` added and the wire stays `v2`.
    sidecar: "Driver session: `NODE_FIELDS` restated at twelve keys (adds `selected`, `pressed`, `invalid` between `value` and `focused`) — 2026-10-10-snapshot-states."
    rationale: "Symbols / APIs: `NODE_FIELDS` `[&str; 9]` → `[&str; 12]`; Counts row naming architecture:133 (lists the nine)."
    basis: "packages/escher-driver/src/schema.rs:32-37"
    dependent-of: D-arch-resources
  - detector: D-arch-resources          # R7
    severity: warning
    section: "§Standard Contracts → Driver session"
    change: >-
      In the `Outcome::to_json` sentence (architecture:133), replace "a node an object keyed by `NODE_FIELDS` in order with `parent`, `enabled`, `checked` and `value` left out when absent" with: `parent`, `enabled`, `checked`, `value`, `selected` and `pressed` left out when absent, and `id`, `role`, `name`, `invalid`, `focused` and `bounds` always written — `invalid` a boolean on every node (packages/escher-driver/src/json.rs:102-104; packages/escher-driver/src/json.rs:129-135; packages/escher-driver/src/json.rs:174-176).
    sidecar: "Driver session: a node's JSON form — six keys optional (adds `selected`, `pressed`), `invalid` always written — 2026-10-10-snapshot-states."
    rationale: "Symbols / APIs: `Outcome::to_json`, a node's written form; Counts row 'keys left out when absent 4 → 6'."
    basis: "packages/escher-driver/src/json.rs:129-135"
    dependent-of: D-arch-resources
  - detector: D-arch-resources          # R8
    severity: warning
    section: "§Standard Contracts → Driver session"
    change: >-
      In the help-text sentence (architecture:133), retire "the help of the result field `changed` reads \"the elements on the screen before and after the step whose role, name, state, bounds or parent differ, as they read after it\" and the help of `snapshot`'s `text` \"the screen as one text: one line per element, nested by indent\" — both the words that stood before 2026-10-10-driver-cli" and its two citations: both help texts were restated at 2026-10-10-snapshot-states and now name the state words and the two limits of the boolean model — a mixed toggle reads unpressed / `pressed` false, a grammar or spelling error reads invalid — quoted from the source as they now read (`changed` at packages/escher-driver/src/schema.rs:203-205, `snapshot`'s `text` at packages/escher-driver/src/schema.rs:232-236).
    sidecar: "Driver session: the two served help texts (`changed`, `snapshot`'s `text`) restated with the state words and the two boolean-model limits; the pre-driver-cli wording and its citations retired — 2026-10-10-snapshot-states."
    rationale: "Symbols / APIs: two help texts of `VERBS` restated. Architecture quotes both verbatim as unchanged words, so the quotation is now false. The report gives the lengths but not the wording — the orchestrator reads the two ranges for the exact text."
    basis: "packages/escher-driver/src/schema.rs:203-205"
    dependent-of: D-arch-resources
  - detector: D-arch-resources          # R9
    severity: warning
    section: "§Standard Contracts → Driver session"
    change: >-
      In the crate-docs clause (architecture:133: "the crate's docs state that an element's `bounds` are where it stands, that a hit stops where a box clips by `overflow`, and one reading as off, the `contain: paint` one"), add that since 2026-10-10-snapshot-states they also state, in one sentence, the three states a node reads — selected, pressed, invalid — and the two limits of the boolean model (packages/escher-driver/src/lib.rs:53-56).
    sidecar: "Driver session: crate docs gain one sentence on the three states and the two limits — 2026-10-10-snapshot-states."
    rationale: "Symbols / APIs: Crate docs — one sentence on the three states and the two limits (`lib.rs:53-56`)."
    basis: "packages/escher-driver/src/lib.rs:53-56"
    dependent-of: D-arch-resources
  - detector: D-arch-resources          # R10 (primary)
    severity: warning
    section: "§Standard Contracts → Mutation, query and CSSOM"
    change: >-
      After the `build_accessibility_tree` naming-sources clause (architecture:122), add: since 2026-10-10-snapshot-states the builder (`BaseDocument::build_accessibility_node`, private) also reads three state attributes on every element, whatever its role, each value trimmed of ASCII whitespace and matched exactly in lower case — `aria-selected` `true` / `false` → AccessKit `selected` true / false, anything else nothing; `aria-pressed` `true` · `false` · `mixed` → `toggled` `True` · `False` · `Mixed`, anything else nothing; `aria-invalid` `true` · `grammar` · `spelling` → `invalid` `True` · `Grammar` · `Spelling`, absent, empty after trimming or `false` nothing, and any other value `Invalid::True` (WAI-ARIA 1.2's rule); `aria-checked` is not read, no role is consulted, no log site and no panic path — so the tree changes for every document (packages/blitz-dom/src/accessibility.rs:102-129).
    sidecar: "Mutation, query and CSSOM: the accessibility builder reads `aria-selected`, `aria-pressed`, `aria-invalid` by value on every element — 2026-10-10-snapshot-states."
    rationale: "Symbols / APIs: the accessibility builder reads three state attributes, with the three value grammars; expected amendment 3: the fact is new to every master; the builder's contract at architecture:122 names only its three name sources."
    basis: "packages/blitz-dom/src/accessibility.rs:102-129"
  - detector: D-arch-resources          # R11
    severity: escalate
    section: "§Standard Contracts → Mutation, query and CSSOM"
    change: >-
      In the shell clause of the same contract (architecture:122), where the platform accessibility adapter is said to receive "the same crossing and the same content classes (ids as `author_id`, accessible names)", add that since 2026-10-10-snapshot-states the nodes it is handed also carry a third class, states — `selected`, `toggled` and `invalid`, closed enums and booleans, no attribute value as typed — ratified by the founder (2026-10-10, by question dialog in the overseer session, relayed by the overseer: the builder writes the three states on the node); what an assistive technology reads of them is unmeasured until the windowed check — no stand binary builds a platform adapter and the dev host's AT-SPI bus is off.
    sidecar: "Mutation, query and CSSOM: the platform-adapter crossing gains the content class \"states\" (`selected`, `toggled`, `invalid`), founder-ratified 2026-10-10, windowed witness owed — 2026-10-10-snapshot-states."
    rationale: "Same engine fact as R10, restated where architecture enumerates what crosses to the platform adapter. Coverage row 2; Cross-project (inputs#I6): fork 2 = ratify, its price the unmeasured assistive-technology reading. Marked escalate because it writes the founder's word into a master and the report owes a halt at Validate (inputs#I7)."
    basis: "packages/blitz-dom/src/accessibility.rs:102-129"
    dependent-of: D-arch-resources
  - detector: D-arch-resources          # R12
    severity: warning
    section: "§Existing Scopes → blitz-tests"
    change: >-
      In the driver-calls clause (architecture:263), replace "the eleven files `stand_act_ids` … and, since 2026-10-10-driver-cli, `stand_act_filled`, 37 tests and two ignored children" with the twelve files, adding "and, since 2026-10-10-snapshot-states, `stand_act_states`" (3 tests, each through `Session::run`, both layout modes), the total 40 tests and two ignored children; add to the covered behaviours "a step's diff returning a node with its pressed, selected or invalid reading as it reads after the step", and add `tests/blitz-tests/tests/stand_act_states.rs` to the clause's citation list.
    sidecar: "blitz-tests scope: driver-action files eleven → twelve (`stand_act_states`, 3 tests), 37 → 40 tests — 2026-10-10-snapshot-states."
    rationale: "Files: new `stand_act_states.rs` (158 lines); Counts row 'Driver-action checks 11 → 12 files … architecture:115, :263'; expected amendment 9. The total 40 is architecture's 37 plus the twelfth's 3; the report states no total, so the orchestrator re-derives it."
    basis: "tests/blitz-tests/tests/stand_act_states.rs:79-158"
  - detector: D-arch-resources          # R13
    severity: warning
    section: "§Existing Scopes → blitz-tests"
    change: >-
      In the session-common clause of the same row (architecture:263), replace "read through `mod session_common;` by four of the session checks, by `stand_settle` and by the eleven `stand_act_*`, sixteen readers" with "by the twelve `stand_act_*`, seventeen readers"; the neighbouring "read through `mod common;` by 17 stand checks" stays as it reads.
    sidecar: "blitz-tests scope: `mod session_common;` readers sixteen → seventeen (twelve `stand_act_*`) — 2026-10-10-snapshot-states."
    rationale: "Second occurrence of the 'eleven driver-action files' claim in the same row; Counts rows on the two shared modules."
    basis: "tests/blitz-tests/tests/stand_act_states.rs"
    dependent-of: D-arch-resources
  - detector: D-arch-resources          # R14
    severity: warning
    section: "§Conventions → Tests"
    change: >-
      In the shared-module sentence (architecture:115), replace "the settle check `stand_settle` and the eleven driver-action checks `stand_act_*`" with "the twelve driver-action checks `stand_act_*`".
    sidecar: "Conventions → Tests: driver-action checks eleven → twelve — 2026-10-10-snapshot-states."
    rationale: "Third occurrence of the 'eleven' claim; the Counts row names architecture:115."
    basis: "tests/blitz-tests/tests/stand_act_states.rs"
    dependent-of: D-arch-resources
  - detector: D-arch-resources          # R15
    severity: warning
    section: "§Existing Scopes → blitz-tests"
    change: >-
      In the row's opening coverage list (architecture:263: "covering accessibility (…accessibility_hidden.rs…; …accessibility_roles.rs…)"), add the third accessibility check, `tests/blitz-tests/tests/accessibility_states.rs` — the three state attributes on a parsed page, every value of each grammar, on a role that supports the attribute and one that does not, `aria-checked` setting none of the three, 4 tests in both layout modes; it rides the workspace test leg, the `a11y` CI leg staying its three named files.
    sidecar: "blitz-tests scope: accessibility coverage gains `accessibility_states` (4 tests; not in the `a11y` leg) — 2026-10-10-snapshot-states."
    rationale: "Files: new `accessibility_states.rs` (220 lines); Coverage: 4 tests, both layout modes, both builds; Harness / gate surface: the `a11y` CI leg stays three named files. §Standard Contracts → CI contracts' three-file leg needs no edit."
    basis: "tests/blitz-tests/tests/accessibility_states.rs:128-220"
  - detector: D-arch-decisions          # R16
    severity: escalate
    section: "§Established Decisions → Scrolling and input"
    change: >-
      After "both are flagged upstreamable, cut against upstream `main` at 7832c177, and no upstream pull request is open" (architecture:87), record the third direct engine change for every document: since 2026-10-10-snapshot-states the accessibility builder writes `selected`, `toggled` and `invalid` from `aria-selected`, `aria-pressed` and `aria-invalid` (packages/blitz-dom/src/accessibility.rs:102-129) — ratified by the founder (2026-10-10, by question dialog in the overseer session, relayed by the overseer: the builder writes the three states on the node; what an assistive technology reads stays unmeasured until the windowed check), flagged upstreamable against the same 7832c177 — `accessibility.rs` reads +57 −1 against it where it read +26 −0, the ref not fetched — and no upstream pull request is open.
    sidecar: "Established Decisions → Scrolling and input: a third direct engine change (the accessibility builder's three states) recorded as founder-ratified and upstreamable against 7832c177 — 2026-10-10-snapshot-states."
    rationale: "Architecture:87 records direct engine changes as a founder-gated pattern and accounts for two. The report lands a third (Files: `accessibility.rs` +31 −1; the tree changes for every document) and expected amendment 4 names this site. The decision is not contradicted but unrecorded; escalate because it writes his word into a master under the halt owed at Validate (inputs#I7). No other `D-arch-decisions` drift: no dependency, crate, feature, socket, port or env var; the wire stays `v2`; the §Established Decisions → Driver session and §Occupied Resources → Network ports and listeners carrier lists stand as written, the three readings sitting inside those two ratified carriers per the founder's fork-1 answer."
    basis: "packages/blitz-dom/src/accessibility.rs:102-129"
```

## Totals

52 proposals over 7 docs: architecture 16 · security-plan 7 · design-system 0 · layout-templates 1 · test-plan 16 ·
obs-plan 4 · a11y-plan 8. Entity probe over this file after the last return was recorded: see §Validate.

## Validate

Probe: 0 HTML entities in the recorded returns; 52 proposal heads parsed. No raw twin is owed: no return failed the
parse or the probe, and the one `proposals: []` return (design-system) was valid YAML as it arrived.

**The opening rule (a coordinate the report does not carry).** 0 rejected. Every source coordinate a proposal writes
was checked against the report's bullets and its last section: the stand's seven attribute lines; `accessibility.rs`
102-129; `snapshot.rs` 59-69 and 159-161; `snapshot_text.rs` 71-83 (an added range); `json.rs` 102-104, 129-135 and
174-176; `schema.rs` 32-37, 203-205 and 232-236; `lib.rs` 53-56; `accessibility_states.rs` 128-145, 147-165, 167-186,
188-220 and the span 128-220; `stand_act_states.rs` the span 79-158; `stand_snapshot_state.rs` 80-107, 377-397,
399-449, 451-486 and the span 377-486. The a11y return's `report.md:{n}` numbers are coordinates of the report, not
of the tree. Two figures no return could take from the report were measured by the orchestrator: `stand_act_states`
declares `mod session_common;` and not `mod common;`, `accessibility_states` declares neither (`grep -c`, 1 · 0 and
0 · 0); the twelve `stand_act_*` files hold 42 `#[test]` lines, 40 and the two ignored children of `stand_act_spans`.

**Check 1 — playbook.**
- *Accurate this-chunk addition* (routine → apply): L1 · O1 O2 O3 · A1 A2 A3 A4 A5 A7 A8 · T1 to T16 · R1 to R10 ·
  R12 R13 R14 R15 · S6. Each names a symbol, count or file the report's Changes carry as this chunk's, landing in an
  existing section. S6 is the in-process `Debug` enumeration of a diff's content — no exit of its own.
- O4 (15 → 17 readers of `common/mod.rs`): the count predates the chunk and the chunk added no reader, so it is not a
  this-chunk addition; it is the report's disproved claim 3, measured at the base and now, and check 6 owes it a
  disposition. Applied: the body takes the measured figure. No fix is owed in code, so the measured-disproof rule's
  second half (a CARRY for the fix) has nothing to pin.
- S7 (the two help texts "state no limit"): graded `escalate` by the detector's own severity; its subject is no
  boundary — a served description, fixed literals — and it is the amendment the citation sweep's two `claim false`
  rows of security-plan raise. Applied as routine whatever the card's answer.
- *Boundary widening* (never routine, judged by subject → escalate): S1 · S2 · S3 · S4 · S5 · R11 · R16 · A6. Each
  records what crosses a hardened boundary, or writes the founder's ratification into a master. S2, S4 and S5 are
  fork 1's subject (three readings inside the socket's and the command line's two ratified carriers); S1, S3, R11,
  R16 and A6 are fork 2's (three state properties on the nodes the platform adapter reads; the third direct engine
  change). The plan's reviewed list recorded both PROVISIONAL on the operator's reading (`inputs#I4`); the founder's
  own answers are now in (`inputs#I6`) and by the authority rule his word supersedes the provisional reading. No
  PROVISIONAL mark stands in any master for either (0 hits), so nothing is discharged out of batch — the record is
  written once, ratified. Staged to the halt the operator asked for (`inputs#I7`).
- A rule proposal rides the same card: the relay carries fork 1's stated price as a rule of the founder's from now
  on — a new field of a closed value set inside an already ratified carrier is not a widening and does not come to
  him. It narrows what the never-routine class covers, so it is the operator's to approve, never the wrap's to mint.

**Check 2 — cross-contradiction.** None. The census figures agree across the three masters that state them
(twelve driver-action files; 17 readers of each shared module; 40 tests and two ignored children: R12 · R13 · T1 ·
T2 · T3 · T4 · O1 · O2), and the three records of the builder's grammar (R10 · S1 · A6) say one thing.

**Check 3 — intent-consistency.** The report's seven deviations each carry their justification and none leaves the
entry's intent or v010-17's acceptance; the scope record is empty (`scope: clean`, 0 recorded). No escalation.

**Check 4 — absence needs evidence.** The report's "0 hits in the seven masters and the registries" for the three
attributes names its search; the returns' "no other occurrence" claims are re-derived by the cascade sweep, whose
dispositions are written after it runs.

**Check 5 — expected amendments.** 1 → R1 to R5 · 2 → R6 to R9 · 3 → R10 (R11) · 4 → R16 · 5 → S2 (S4, S5) · 6 → S1,
S3 · 7 → A1 to A6 · 8 → A7 · 9 → T1 to T16 and the key file · 10 → O1 to O3 · 11 → no amendment, the figures did not
move (the report's reading) · 12 → L1 · 13 → no amendment, coordinates only, the citation sweep's. The four `claim
false` rows of `citation-dispositions.md`: architecture ×2 → R8; security-plan ×2 → S7. Nothing is raised by the
orchestrator beyond the proposal set at this point; sites the cascade sweep finds are folded in at Apply.

**Check 6 — disproved claims.** 1 (a plan and research claim on `NodeState` literals) and 2 (a plan claim on where
step 11's readings come from) and 4 (research's per-file count): no master states them → P3 curation reads them
with the report's sweep hazards. 3 (obs-plan:70) → O4.

**Staged:** apply 44 · escalate 8 (+ the rule proposal) · reject 0.

## Escalate — resolved

One card, one question, at this wrap's Validate (2026-10-10): the founder's two recorded answers (`inputs#I6`), the
eight proposals they cover by document and fork (S2 · S4 · S5 fork 1; S1 · S3 · R11 · R16 · A6 fork 2), the note that
no PROVISIONAL mark stands in a master, and the playbook rule in full with its preconditions.

Answer — word: "Confirm all (Recommended)" — the operator, 2026-10-10, by question dialog at this wrap, with the note
that the confirm is the founder's own: the card was put to him whole by question dialog in the overseer session on
2026-10-10 and he chose the option to confirm all, the overseer's label for it «Подтвердить всё», relayed by the
overseer, no words of his own added. The answer and its note are kept verbatim in `relay-2.md` beside this file.

Dispositions: S1 · S2 · S3 · S4 · S5 · R11 · R16 · A6 → apply, each recorded as ratified by the founder (2026-10-10,
by question dialog in the overseer session, relayed by the overseer), the sidecar entries naming the ratification.
The playbook rule → appended as drafted on the card, its note naming both of his words. Open escalations: 0.

**Final:** apply 52 · reject 0 · escalations resolved 1 (eight proposals and one rule).
