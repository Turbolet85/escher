# Report — 2026-10-20-select-by-id

**Chunk:** Select by id — a select driven by id, read in the snapshot, fixture-proven (v020-08)
**Date:** 2026-10-20
**Commits:** `14818317c chore(2026-10-20-select-by-id): operator pre-CI commit, for the run this chunk's verdict reads`
(the one commit since `last_wrap` 2026-10-20T03:34:30Z; base `2f099735`, its parent — `git log --format='%h %s'
2f0997354..HEAD`)

## Changes (structured — detectors read this)
- **Files:** 20 code files (`gate.py scope`: changed 20 · listed 19 · recorded 1; trail
  `.andromeda/runs/2026-10-20T05-49-12-wrap/gate-2026-10-20-select-by-id.json`):
  - new: `packages/blitz-dom/src/node/select.rs` · `tests/blitz-tests/tests/select_interaction.rs` ·
    `tests/blitz-tests/tests/stand_act_select.rs`;
  - modified: `packages/blitz-dom/assets/default.css` · `packages/blitz-dom/src/{accessibility,document,form,mutator}.rs`
    · `packages/blitz-dom/src/events/{keyboard,pointer,mod}.rs` · `packages/blitz-dom/src/node/{element,mod}.rs` ·
    `packages/blitz-traits/src/events.rs` · `packages/blitz-vibey-script/src/runtime.rs` ·
    `packages/dioxus-native-dom/src/{dioxus_document,snapshot}.rs` · `packages/escher-driver/src/{schema,schema_tests}.rs`
    · `tests/blitz-tests/tests/accessibility_label.rs`;
  - ledger and chunk records: `escher-0.2.0/verification-matrix.json` (v020-08), `chunks/2026-10-20-select-by-id/`
    {`plan.md` (gate entry 12 amended on the operator's word), `scope-record.md`, `evidence/{checked-mechanism,counts,
    gate-12-amend,ci}.md`}.
- **Symbols / APIs** (coordinates from the `## New text, by line` listing below):
  - **The select's rules** — new module `blitz-dom::node::select` (`pub(crate) mod select`, node/mod.rs:10), cited to
    the WHATWG `select` element, unit-tested inline: `OptionReading` (17-26), `SelectStep` (`Next` · `Previous` ·
    `First` · `Last`, 28-39), `Typeahead` {select, text, at} (41-50), `TYPEAHEAD_TIMEOUT` = 1 s, `is_single_select`
    (55-58), `is_select_option` (60-68, an `option` whose parent is a `select`, read through `Node::with`),
    `options` (70-86, the `option` children in tree order — an `optgroup`'s options not listed), `is_chosen` (88-92,
    the `CHECKED` element state), `chosen_option` (94-100, the last chosen), `readings` (102-115), `option_label`
    (117-127, `label` attribute when not empty, else the text), `option_value` (129-139, `value` attribute when present,
    else the text), `option_text` / `strip_and_collapse` (141-153), `settled` (155-164, HTML's selectedness setting
    algorithm for a single-choice select: the last chosen, else the first option not disabled), `stepped` (166-181,
    next / previous / first / last not disabled; none chosen reads `Next` as `First`, `Previous` as `Last`), `matched`
    (183-193, the first enabled option whose label starts with the text, lowercase-compared; empty text matches
    nothing); `mod tests` (195-301, 4 table tests).
  - **`BaseDocument`** (document.rs): new **public** `select_value(&self, select: NodeId) -> Option<String>` (717-728;
    HTML's `select.value` — the chosen option's value, `None` with none chosen or for a non-select; its non-test caller
    dioxus-native-dom's snapshot `value`, and `apply_select_choice`); `pub(crate)` `settle_select_choice` (730-743),
    `set_select_choice` (745-753, reads whether any option changed), `set_option_chosen` (755-775, the `CHECKED` write
    through `snapshot_node_and` for an in-document option, else a direct state write); new field
    `select_typeahead: Option<select::Typeahead>` (338-339, `None` at construction, 555); new private free fn
    `is_labelable` (3126-3140). **Changed:** `label_bound_input_element` — name and signature unchanged, both arms now
    match HTML's labelable elements (`button`, `input` whose `type` is not `hidden` ASCII-case-insensitively, `meter`,
    `output`, `progress`, `select`, `textarea`) in place of `input` alone; its two callers unchanged in code and moved
    in behaviour: the accessibility tree's label loop (a `<label>` now names a select, a textarea, a button…) and
    `handle_click`'s label arm (a click on a select's label re-dispatches to the select, focusing it).
  - **Keyboard** (events/keyboard.rs): a select arm in `handle_key_or_input_event` (108-113) — focused element
    `select::is_single_select`, `!ElementData::is_disabled()`, a pressed `KeyPress` — calling the private
    `press_select_key` (165-200): `ArrowDown`/`ArrowUp` next/previous enabled option, `Home`/`End` first/last, a
    `Key::Character` with no Ctrl, Alt or Meta → typeahead; every other key nothing (Enter and Space open nothing; a
    released Space activates no select — `activates_on_space` unchanged). `BaseDocument::type_into_select` (private,
    248-269: the buffer starts afresh for another select or when `Clock::now() − at ≥ 1 s`) and
    `pub(crate) apply_select_choice(select, option, dispatch_event)` (271-297: the one write · mark · events · redraw
    path — `set_select_choice`; where a choice changed, the select joins `changed_nodes`, `Input` then `Change` are
    dispatched, each carrying `select_value`, and a redraw is requested; a choice of the chosen option fires nothing).
  - **Pointer** (events/pointer.rs:857-870): a `select` arm in `handle_click`'s walk — clears `select_typeahead` and
    the document's `text_selection`, focuses the select through `generate_focus_events` / `set_focus_to`; the walk's
    `is_disabled` gate stops a disabled select first, so a click neither focuses nor acts on it.
  - **Mutator** (mutator.rs): new field `select_nodes: HashSet<NodeId>` (58-60, 90); `maybe_record_node` gains a
    `select` arm (1211-1215) — every tree entry point that calls it on the parent (`append_children`,
    `insert_nodes_before`, `insert_nodes_after`, `replace_node_with`, `remove_node`, `remove_and_drop_node`,
    `remove_and_drop_all_children`, `reparent_children`) records the select; `process_added_subtree` records a
    `select` entering the document (1097-1101); `set_attribute` / `clear_attribute` of `selected` call the private
    `set_option_selectedness` (321-324, 473-481; 1220-1241): an option's `selected` written sets it chosen and its
    single-choice select's other options not (exclusive), cleared sets it not chosen, its select recorded; `flush`
    settles every recorded select (1017-1020). Fires no event.
  - **Seeding** (node/element.rs:455-461): `ElementData::new` inserts `CHECKED` on an `option` created with a
    `selected` attribute (HTML: a selected option matches `:checked`).
  - **Events** (blitz-traits events.rs): new `DomEventKind::Change` (after `Input`, so every later kind's discriminant
    moves up by one — the Dioxus bridge's 64-slot `event_handler_counts` is indexed by it, data and kind kept aligned),
    `from_str("change")`, new `DomEventData::Change(BlitzInputEvent)` with its `name` ("change"), `kind`,
    `cancelable` (false) and `bubbles` (true) arms (150 · 202 · 250-251 · 307 · 354 · 402 · 449). Companion arms in
    blitz-dom events/mod.rs (93, 246-248): no UI event mapped, no default action.
  - **Dioxus bridge** (dioxus_document.rs:377-382): `Input | Change` handed to the app as `NativeFormData { value }` —
    an `onchange` handler now fires.
  - **vibey-script** (runtime.rs:1672-1673): comment only — a real `Change` is dispatched by name like any event (a
    plain `Event`, `dom/event.rs`'s `_` arm); its own synthesized `change` stays keyed to a checkbox or radio target.
  - **Accessibility** (accessibility.rs): `is_hidden_from_accessibility_tree` keeps an option whose parent is a
    `select` although it computes `display: none`; `hidden`, `inert` and `visibility: hidden` still exclude it
    (257-258, 260-273); such an option writes `selected` from `is_chosen`, true or false, and reads no `aria-selected`
    (171-181); its non-empty `label` attribute is its `label` when it has no `aria-label` (157-162). Role mapping
    unchanged (`ComboBox`, `ListBox` for `multiple`, `ListBoxOption`).
  - **Form** (form.rs:276-290): a named `select`'s entries — every chosen option that is not disabled, its
    `option_value` (HTML's step as written; a single-choice select holds one).
  - **Snapshot** (snapshot.rs): `value` takes `(doc, node_id, element)` (253-259) — a `select` reads
    `BaseDocument::select_value`; new private `select_parent` (241-250) — an option whose parent is a `select` reads
    its select's client rect as its `bounds` (144-152, 155); unit tests `SELECT_VALUES` (936-953),
    `a_select_reads_its_chosen_options_value` (955-975), `a_selects_options_are_its_children_named_by_their_labels`
    (977-1014). No role or name mapped there.
  - **Driver** (escher-driver schema.rs:208-209): `type`'s help gains one fixed sentence — "on a select, the text
    chooses the first option whose label starts with it"; its pin (schema_tests.rs:77-78). No verb, argument, key name,
    result field, refusal cause, session error or exit added; `describe` count row `[11, 13, 12, 3, 8, 4, 15]` unmoved
    (escher-driver unit gate green).
  - **No port, socket, env var or listener** added.
- **Crates / modules:** module added — `blitz-dom::node::select` (`pub(crate)`); no crate added or removed.
- **Dependencies:** none (gate entry 11, `git diff --name-only 2f099735 -- Cargo.toml Cargo.lock packages/*/Cargo.toml …`
  reads 0).
- **Schema / config:** the user-agent stylesheet (`packages/blitz-dom/assets/default.css`): `select` joins `input,
  textarea`'s border (`1px solid #999`), padding (`2px`) and background (`white`) rule (77-78) and takes
  `display: inline-block` (84-86); new rule `select:not([multiple]) > option:checked { display: inline; }` (155-159,
  beside `option { display: none; }`). The added ranges 77-78 · 84-87 · 155-160 (new-text listing) move every later
  line down by 2, 6 and 12 in turn — the masters' `default.css:N` citations past line 76 (design-system §Color Palette
  / §Typography rows, a11y-plan rows) are the citation sweep's (P2).
- **Spec-master edits:** none (no master edited by this chunk).
- **Retired:** none.
- **Counts / qualifiers moved** (sweep: `cascade.py sweep --pre-pass`, trail
  `.andromeda/runs/2026-10-20T05-49-12-wrap/cascade-2026-10-20-select-by-id-pre-pass.json`; patterns
  `{run_dir}/report-patterns.toml`):
  - blitz-tests `.rs` files 141 → 143 and test targets 140 → 142 (`ls tests/blitz-tests/tests/*.rs | wc -l`) — pattern
    `bt-files`: 1 standing · test-plan.md:68 @c3507,3524 stale (141 → 143, 140 → 142), @c3567 its history clause
    (`140 and 139 until 2026-10-20-load-wait`) gains this chunk's step.
  - `run stand` files 46 → 47 and passed 185 → 190 (ignored 7, failed 0; gate entry 15, `run.end`
    2026-10-20T04:37:20Z) — patterns `stand-files`: 3 standing · test-plan.md:324 stale · test-plan.md:353 stale ·
    test-plan.md:119 @c10555 re-derived (its re-count clause gains this chunk's); `stand-pass`: 1 standing ·
    test-plan.md:119 re-derived.
  - `stand_act_*` files fourteen → fifteen (`stand_act_select` the fifteenth) — pattern `stand-act`: 13 standing ·
    10 stale (architecture.md:137, :354; test-plan.md:22, :68 «the fourteen `stand_act_*.rs`», :102 «six of the
    fourteen driver-action checks» → six of the fifteen — `stand_act_select` declares no `mod common;`, :103, :146,
    :185; obs-plan.md:70 @c5360, @c6267; registries/contracts/test-plan/session-lifecycle.md:11) · 3 no-change, history
    (test-plan.md:27 @c36504 «the fourteenth» is `stand_act_transformed`, @c48358; test-plan.md:119 @c3200;
    test-plan.md:340 @c7259, @c7816; obs-plan.md:70 @c13741, @c13917 — history offsets on stale lines too).
  - `tests/fixtures/mod.rs` readers 21 → 23 (20 → 21 `stand_*` files, `font_face_sniffed_source`, and
    `select_interaction`, a non-`stand_*` reader) — pattern `fixtures`: 6 standing · stale: architecture.md:363 («by 21
    (20 `stand_*` files and `font_face_sniffed_source`)» — the same line's «`mod common;` by 21 checks» is the
    `tests/common/mod.rs` count, unmoved: neither new check declares `mod common;`), test-plan.md:68 @c2542,
    test-plan.md:104, test-plan.md:132 («`run stand` runs the 20 `stand_*` readers» → 21) · no-change:
    architecture.md:142 (no count), test-plan.md:27 @c50507, @c51587 (per-check history).
  - `tests/session_common/mod.rs` readers 27 → 28 (`stand_act_select`) — pattern `session-common`: 7 standing ·
    stale: architecture.md:363 («by 27»), test-plan.md:68 @c1328 («by 27 checks»), test-plan.md:103 @c2926 («27
    readers») · no-change: architecture.md:137 @c4187 and :354 (no count — :354's «fourteen `stand_act_*`» is the
    `stand-act` row), test-plan.md:27 (per-check entries; this chunk's two checks are additions), obs-plan.md:70
    @c6844.
  - workspace tally 202 · 1062 · 0 · 12 → 204 · 1083 · 0 · 12 (gate entry 18 over `target/ci-logs/test.log`) —
    pattern `tally`: 2 standing · test-plan.md:334 and :340 re-derived (each gains this chunk's measured clause; the
    load-wait clauses stay as history). +21 by name: `node::select::tests` 4, `snapshot::tests` 2,
    `select_interaction` 7, `stand_act_select` 5, `accessibility_label` 3 (`evidence/counts.md`).
  - `accessibility_label` tests 10 → 13 — pattern `acc-label`: 7 standing · stale: security-plan.md:126 («10
    tests»), test-plan.md:27 @c13334 («`accessibility_label` 10»), a11y-plan.md:70 («10 tests») · no-change:
    test-plan.md:83, :85 (no count), test-plan.md:119 @c9164 and :340 @c16843 (history).
  - Per-OS CI tallies (CI#39371468367, `evidence/ci.md`): linux 204 · 1083 · 0 · 12; macos and coverage 185 · 1080 · 0 ·
    12; windows 185 · 1038 · 0 · 9; a11y 6 · 43 · 0 · 0; stand-repeat 165 · 660 · 0 · 21 — pattern `tally`'s
    test-plan.md:334 row (re-derived, a new clause).
  - A `<label>`'s target: an `input` alone → every labelable element — pattern `label`: 2 standing · stale:
    architecture.md:159, security-plan.md:126 (both describe `label_bound_input_element` as for-target, else first
    nested `input`).
  - The form event kinds: `Input` the one → `Input` and `Change` — pattern `change-event`: 2 standing ·
    architecture.md:160 re-derived (`DomEventKind::from_str`'s list gains `change`), architecture.md:234 no-change (the
    64-slot array indexed by `DomEventKind` still true; its discriminants moved by one, aligned).
  - The accessibility tree's exclusion: `hidden` / `display: none` / `visibility: hidden` exclude a node → except a
    select's option under `display: none` — pattern `hidden-tree`: 3 standing · a11y-plan.md:112 stale (its exclusion
    sentence gains the exception) · architecture.md:102, security-plan.md:130 no-change (they state `inert`'s read,
    still true).
  - A click's focus: a `button` / submit-reset-button `input` / range input / checkbox / radio / `summary` focuses,
    any other element clears focus → `select` added — pattern `click-focus`: 4 standing · stale: architecture.md:86
    @c9361,9394, security-plan.md:132, a11y-plan.md:161 («a click on any other element is unchanged») ·
    a11y-plan.md:160 no-change (a non-interactive area still clears focus).
  - `type`'s help: «replaces what the element holds … an empty text clears it» → + «on a select, the text chooses the
    first option whose label starts with it» — pattern `type-help`: 3 standing · re-derived: architecture.md:192
    @c21636, security-plan.md:363 @c3254 (each «`type`'s help still says …» gains the sentence) · no-change:
    architecture.md:214 (the CLI's usage line, unchanged).
  - The select's role list and the disabled-control list — pattern `select`: 12 standing, all no-change (true claims
    sharing the token: architecture.md:98, :175, security-plan.md:121, a11y-plan.md:147, :150, :151 — `select` among
    the form controls `is_disabled` reads; architecture.md:205 @c29020,29039 and a11y-plan.md:108 the role map,
    unchanged; a11y-plan.md:200, :285, :287, :288 the CRUD list's `listbox` / `option` roles by attribute). No master
    states a select's interaction model, value, box or options: each is an addition (Expected amendments).
  - The form entry — pattern `form-entry`: 27 standing, all no-change (form-owner, submission and range-input sites
    and citations; the select step is an addition beside security-plan.md:131's range-input row).
  - The user-agent stylesheet — pattern `ua-css`: 19 standing · no-change in claim (architecture.md:129, :250;
    design-system.md:103, :340; test-plan.md:27, :63, :244; a11y-plan.md:133) · citations into `default.css` past line
    76 move (design-system.md:25, :72, :116, :117, :118, :179, :211, :245; a11y-plan.md:245, :265, :303) — the P2
    citation sweep's.
  - The silence census — pattern `silence`: 8 standing · obs-plan.md:70 re-derived (its census gains
    `select_interaction` and `stand_act_select`, gate entry 9 reading 0 over them) · obs-plan.md:74-80 no-change
    (earlier chunks' silent-check statements; this chunk's is an addition).
  - Typeahead on the document's clock: none — verified (pattern `typeahead`'s control unfired over the seven masters:
    «typeahead · control UNFIRED over the seven masters at 14818317»).
  - Owners: pattern `owner` 2 standing — architecture.md:90 («merge surface, by file, is owned by a CARRY on
    «Upstream sync ahead of polish…»», the precedent for this chunk's merge surface), security-plan.md:125 @c9272
    (unrelated id rule, no-change). The chunk title: `owner pattern: control unfired` («title · control UNFIRED over
    the seven masters at 14818317»).
  - Sweep reach: 18 patterns · 15 moved facts, 15 with a pattern (the report's own count) · owner pattern control
    unfired · by line 12 lines · 33 offsets.
  - The residual «no `select` interaction model» (`.andromeda/residuals.md` line 11, `absorbed:v020-08`) closes on
    this chunk — no master site (residuals.md is no master; its line is P5's).
- **Dev-tool versions:** none.
- **Harness / gate surface:** none changed — two new blitz-tests targets, `select_interaction` (workspace `test` leg)
  and `stand_act_select` (picked up by `run stand` and `stand-repeat` by its `stand_` prefix, no script change); plan
  gate entry 12's `run` amended on the operator's word (`events/(keyboard|pointer)` → `events/(keyboard|pointer|mod)`,
  `evidence/gate-12-amend.md`).
- **Cross-project / external claims:**
  - CI#39371468367 on `14818317c397` (the pre-CI commit, pushed to `origin/build/escher-0.2.0` by operator entry 21):
    `verdict: green · checks 17/17 · wall 1022 s` (`ci.py conclusion`, `evidence/ci.md`).
  - Inputs (`inputs.py verify`, trail `.andromeda/runs/2026-10-20T05-49-12-wrap/inputs-2026-10-20-select-by-id.json`):
    `I1 · message: the operator, with /andromeda-phase, 2026-10-20T03:46Z · copy message · n/a` (cited scope.md:36 ·
    research.md:9; the forks-placement directive); `I2 · message: the operator as delegate, by question dialog,
    2026-10-20T04:00Z · copy message · n/a` (cited scope.md:46 · research.md:90 · research.md:163 · plan.md:17 ·
    plan.md:423; F1 keys + typeahead, F2 the engine shows the label, F3 options as children, F4 `change` and the
    labelable label — delegate, PROVISIONAL for the founder's word); `inputs: 2 entries — … n/a 2 · uncited 0 ·
    unparsed 0`. No entry names a P5 review card; `.andromeda/runs/2026-10-20T03-46-58-phase/p5-card-1.md`'s `## Beyond
    the plan` reads `none`.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - The plan's gate entry 12 (plan.md, `## Test Commands`, «no tracked file outside the plan write set») excluded
    `packages/blitz-dom/src/events/mod.rs`, which step 8's own «every exhaustive match the new variant breaks gains
    its arm» requires — measured: both implement firings read count 1, that file alone. DISPOSED: amended on the
    operator's word (`evidence/gate-12-amend.md`), firing 3 green. No master site — a plan probe.
  - Research's measured «`select` lays out as nothing … left out of the snapshot» (research.md §Measured) is the
    before; this chunk's `evidence/checked-mechanism.md` and `select_interaction` measure the after. No master states
    the before (pattern `select`: 0 sites stating it — no-change rows only).
- **Expected amendments (from plan):**
  - architecture §Established Decisions → [Scrolling and input] and [DOM semantics] — the select model — carried:
    **Symbols / APIs** (select rules, keyboard, pointer, mutator, seeding, accessibility, form); sites: patterns
    `select` (architecture 4 rows, all no-change: no master site states the model — an addition), `label` (architecture
    .md:159), `click-focus` (architecture.md:86), `change-event` (architecture.md:160).
  - architecture §Standard Contracts → Events (`Change`), Dioxus DOM bridge (a select's value, an option's bounds),
    Nodes and widgets — carried: **Symbols / APIs** (Events, Dioxus bridge, Snapshot); sites: `change-event`
    (architecture.md:160, :234), `select` (architecture.md:205, the snapshot role list — no-change).
  - security-plan §Input Validation — a select row, the labelable label, the `change` event, option labels as names —
    carried: **Symbols / APIs**, **Coverage of new surfaces**; sites: `label` (security-plan.md:126), `click-focus`
    (security-plan.md:132), `type-help` (security-plan.md:363), `select` (security-plan.md:121, no-change), `form-entry`
    (security-plan.md:131, the range row the select row joins).
  - a11y-plan §2, §4, §5, §7 — options in the tree with `selected` from their state, the keys and typeahead, a click
    focuses a select, a `<label>` binds every labelable element — carried: **Symbols / APIs** (Keyboard, Pointer,
    Accessibility); sites: `hidden-tree` (a11y-plan.md:112), `click-focus` (a11y-plan.md:161), `acc-label`
    (a11y-plan.md:70), `select` (a11y-plan.md:108, the role map).
  - design-system §Surface: desktop-native → Tokens — the user-agent `select` box and the shown option — carried:
    **Schema / config**; sites: `ua-css` (design-system.md:116-118, the `input, select, button` font rows citing
    `default.css`; 8 design-system rows, claims no-change, citations moved).
  - layout-templates §Surface: cli → Primary screens — `type`'s help sentence — carried: **Counts / qualifiers moved**
    (`type`'s help); pattern `type-help` holds 0 layout-templates rows (sites architecture.md:192, :214,
    security-plan.md:363) — no layout-templates owner by this pattern; the detector's own sweep is the control.
  - obs-plan §3 — the two new checks in the silence census — carried: **Counts / qualifiers moved** (the silence
    census); site: `silence` (obs-plan.md:70).
  - test-plan §1, §3, §4, §9, §10 — the two checks, the `accessibility_label` rows, the counts — carried: **Counts /
    qualifiers moved**; sites: `bt-files`, `stand-files`, `stand-pass`, `stand-act`, `fixtures`, `session-common`,
    `tally`, `acc-label` (test-plan rows above, and registries/contracts/test-plan/session-lifecycle.md:11).
  - Pattern «An engine behaviour change for every document», answered by the delegate operator — PROVISIONAL
    («Provisional discharge») — carried: **Cross-project / external claims** (I2) and **Decisions & corrections**.
  - The merge surface (upstreamable) — the wrap pins a `CARRY:` on the Epoch 5 upstream-sync entry naming them by file:
    blitz-dom `node/select.rs`, `document.rs`, `events/{keyboard,pointer,mod}.rs`, `mutator.rs`, `accessibility.rs`,
    `form.rs`, `node/element.rs`, `assets/default.css`; blitz-traits `events.rs`; blitz-vibey-script `runtime.rs` —
    for P5 (precedent: pattern `owner`, architecture.md:90).
  - Residuals: `.andromeda/residuals.md` line 11 closes; the step-17 limits (below, Outcome) are new residual lines or
    stated limits — for P5. No master site.
- **Coverage of new surfaces:**
  - `select` keys + typeahead (every document, remote pages included) → validation {closed key match; typed text
    compared, never stored past 1 s or carried out; `is_disabled` gate ✓} · instrumentation {n/a — no log site, span or
    field (gate entry 8: select.rs 0 print/log/env/panic tokens; entry 10: two `info_span!`)} · PII {n/a — option
    labels leave only as their nodes' `name`, the value as `value`, inside the ratified crossings} · tests {unit
    select.rs 4 · integ `select_interaction` 7, `stand_act_select` 5} · a11y {kbd ✓ arrows/Home/End/typeahead, focus
    ✓ click and Tab, Tab walk unchanged — a disabled select skipped} · tokens {n/a}
  - the `change` event (`DomEventKind::Change`) → validation {n/a — engine-emitted} · instrumentation {n/a} · PII {the
    select's value, as the `input` event carries} · tests {`select_interaction` counts one `input` and one `change` per
    choice, none for no change} · a11y {n/a} · tokens {n/a}
  - a `<label>` naming every labelable element → validation {closed element-name match; `type="hidden"` excluded} ·
    instrumentation {n/a} · PII {n/a — the label's text is the ratified name crossing} · tests {`accessibility_label`
    +3, both modes} · a11y {SC 1.3.1/4.1.2 names ✓} · tokens {n/a}
  - the user-agent select box (`default.css`) → tokens {UA defaults reused from `input`: `1px solid #999`, `2px`,
    `white` — engine defaults, not design tokens; n/a for the stand, which holds no select} · a11y {contrast not
    measured — no stand select; `stand_contrast` unmoved}
  - the snapshot's select value and option nodes → validation {n/a} · PII {the value is the chosen option's `value`
    attribute or text, inside the 2026-10-10 control-value crossing; no mask applies (a select is no masked input)} ·
    tests {snapshot unit tests 2; `stand_act_select`}

## Deviations from intent
1. **Re-seed hook (step 4):** one site, `maybe_record_node`, covers the six listed tree entry points (each already
   calls it on the parent), and `process_added_subtree` also records a `select` entering the document — beyond the
   plan's list: a Dioxus static template is built once and deep-cloned, so its options never pass through an append,
   and a select with no `selected` option read no value until the hook was added (measured: `select_interaction`'s
   first run, 6 of 7 red). Both in `mutator.rs`, a listed file.
2. **A `selected` write is exclusive:** it sets that option chosen and clears its single-choice select's other options
   at once (HTML: an option's selectedness set true on a select without `multiple` sets every other false), then the
   flush settles; with «two or more → the last» alone, a write on an option before the chosen one would lose to it.
3. **An option's `label` attribute names its tree node** (no `aria-label`), so «named by their labels» holds where a
   label differs from the text — asserted in the snapshot unit test (`Cherries`).
4. **Form submission writes every chosen, enabled option** (HTML's step as written) — no second path for `multiple`.
5. **runtime.rs takes a comment change only** — a real `Change` already dispatches by name, and the synthesis stays
   keyed to a checkbox or radio.
6. **`select_interaction`'s fixture app is controlled** — its `oninput` writes `selected` from the event's value.
   Measured: Dioxus re-writes a dynamic `selected` attribute on each re-render, so an app re-rendering without updating
   its state reverts the user's choice (the app keeps control, as the plan says of a range `value` write).
   `stand_act_select`'s page writes `selected` statically, with no handler.
7. **A click on a select clears the document's text selection** (`text_selection.clear()`) — the plan's «any text
   selection the click made inside it».
8. **Two clippy `type_complexity` aliases** in the unit-test tables (`type Row`, `type SelectOption`) — implement's
   firing 1 red on entry 17, firing 2 green.
9. **Plan gate entry 12 amended on the operator's word** — «On my word, an operator edit of plan.md: add events/mod to
   gate entry 12's whitelist (the Change event's exhaustive matches, the companion scope-record.md records).» (the
   operator, 2026-10-20, in this session); `evidence/gate-12-amend.md`.

Scope record (`gate.py scope`: `scope: clean — changed 20 · listed 19 · recorded 1 (companion 1 · mechanical 0 ·
in-intent 0 · widening 0) · absorbed 0 · excluded 61`):
- companion: ``- `packages/blitz-dom/src/events/mod.rs` · companion · serves step 8 · self`` — two exhaustive
  `DomEventData` matches gain a `Change` arm.

## Decisions & corrections
- Measured: Dioxus re-writes a dynamic `selected` attribute on every re-render of its component, whether or not the
  value changed — so a controlled select's app must write `selected` from its `input` event's value, or the user's
  choice reverts at the next render (evidence: a throwaway probe, `value after down Some("b")` with an `oninput`
  counter, `Some("c")` without; `select_interaction`'s fixture).
- Measured: Dioxus builds a static template once and deep-clones it, so a per-element hook on `append_children` never
  sees the template's children; state that depends on an element's children is settled where the element enters the
  document (`DocumentMutator::process_added_subtree`) — `select_interaction` firing 0: 6 of 7 red until then.
- A write-set probe in a plan (an alternation of allowed paths) missed the companion its own step required — the
  exhaustive matches a new enum variant breaks, here `events/mod.rs`; its P5 must-fail control used `lib.rs`, never
  that companion. Corrected on the operator's word (Deviation 9).
- The operator directed the operator pass in plan order with «Stop if anything is red»: entries 20 → the pre-CI
  commit → 21 → 22 → 23, each green or recorded (Outcome).
- Guard refusal: the Bash guard refused one `cat` heredoc writing a throwaway test target («a new document goes through
  the Write tool»); re-issued once through the Write tool.
- `ElementState` is not re-exported from blitz-dom, so a blitz-tests probe moving `CHECKED` swapped two options'
  `element_state()` copies through `snapshot_node_and` (`evidence/checked-mechanism.md`).
- Sweep hazard: in a site sweep, a bare count (`141`, `140`, `46`) matches citation line numbers across the masters —
  each count was swept by its wording (`141 \`.rs\` files`, `over 46 files`), never as a bare number.
- Implement's outcome basis carries no further operator word; its persisted report is
  `.andromeda/runs/2026-10-20T04-14-31-implement/outcome-2026-10-20-select-by-id.md` (moved to green + operator pass).

## Outcome
- (tests) v020-08 — met: `stand_act_select` 5 passed in both layout modes (gate `cargo test -p blitz-tests --locked
  --test stand_act_select 2>&1` green, `0 failed`): at boot `fx-pick` reads `ComboBox`, `Fruit`, value `b`, its three
  options under it named Apple · Banana · Cherry, Banana selected; `type --id fx-pick --text Cherry` accepted and
  settled, its diff naming `fx-pick` with `value` in `differs` and `fx-banana` / `fx-cherry` with `selected`, the next
  snapshot value `c`; the Tab walk `fx-before` → `fx-pick` → `fx-named` → `fx-after` → `fx-before`, the disabled
  `fx-off` skipped, equal before and after the choices. Matrix: `implemented`, ref
  `tests/blitz-tests/tests/stand_act_select.rs::at_boot_the_snapshot_reads_the_select_its_name_value_and_options ·
  ::type_with_a_label_chooses_that_option · ::the_tab_walk_skips_the_disabled_select_before_and_after_the_choices`.
- (arch) choice held as `CHECKED`, shown as the select's content — met: `evidence/checked-mechanism.md` (both modes:
  `fx-pick` 62 × 24 holding Banana 55.17 wide → 55 × 24 holding Cherry 48.02 after `CHECKED` moved) and
  `select_interaction`'s box test; a choice marks the changed set and a no-change key marks nothing (its key and
  typeahead tables assert `has_changes` per row).
- (a11y) keyboard operable — met: `select_interaction` (arrows, Home, End, typeahead; one `input` and one `change` per
  change, none per no change; a disabled select takes no key and no click); Tab skip in `stand_act_select`.
- (a11y) a `<label>` names a `select` and a `textarea`, a hidden input nothing — met: `accessibility_label` 13 passed
  (+3, both modes).
- (security) nothing crosses the driver's surface that did not before — met against the diff: the escher-driver change
  is help text alone (schema.rs:208-209), the describe count row unmoved, no verb / argument / key / field / cause /
  session error / exit in the diff; option labels reach a client only as node `name`, the value as `value`.
- (security) select rules hold no print, log, env read or panic path; gates read `is_disabled` — met: gate entry 8
  reads 0 over select.rs; the key arm and the click walk read `ElementData::is_disabled`.
- (obs) two span sites, silent checks, no wall read — met: entries 10 (2), 9 (0), 7 (0).
- (layouts) no stand file, shared module, manifest or lock — met: entry 11 reads 0.
- (tests) `run stand` 0 failed, 47 files; CI green with `stand-repeat` holding the new check — met: passed 190 · failed
  0 · ignored 7 over 47 files; CI#39371468367 green 17/17, `Stand checks repeated` 165 · 660 · 0 · 21 (+3 lines, +15
  passed — `stand_act_select` × 3 rounds); the coverage job green.
- (a11y) the a11y leg unmoved — met: 6 · 6 · 4 · 8 · 5 · 14 = 43.
- (tests) `ci-leg.sh fast` and `doc` pass — met (implement firing 2: entries 17, 19 green; operator entry 21 re-ran
  `fast` green before the push).

Gates (implement's run `.andromeda/runs/2026-10-20T04-14-31-implement/`, firing 2, with entry 12's firing 3 and the
operator entries):
- `cargo test -p blitz-dom --locked --lib 2>&1` green · `cargo test -p dioxus-native-dom --locked --lib 2>&1` green ·
  `cargo test -p escher-driver --locked 2>&1` green · `cargo test -p blitz-tests --locked --test select_interaction
  2>&1` green (contains 0 failed) · `cargo test -p blitz-tests --locked --test stand_act_select 2>&1` green ·
  `cargo test -p blitz-tests --locked --test accessibility_label --test accessibility_roles 2>&1` green (13 · 6);
- the `Instant::now|SystemTime::now` probe green (exit 1, 0) · the select.rs silence/panic probe green (exit 1, 0) ·
  the two checks' silence probe green (exit 1, 0) · the `info_span!` probe green (2) · the manifests / shared-modules /
  stand `git diff` probe green (0) · the write-set `git diff` probe — red in firings 1 and 2 (count 1,
  `events/mod.rs`), its `run` amended on the operator's word, firing 3 green (exit 1, 0) · the untracked-file probe
  green (exit 1, 0);
- `bash scripts/agent-run.sh boot` green · `bash scripts/agent-run.sh run stand` green (`"failed": 0`, passed 190 ·
  ignored 7, 47 files) · `bash .github/scripts/ci-leg.sh a11y` green (43) · `bash .github/scripts/ci-leg.sh fast`
  green (firing 1 red, clippy `type_complexity`; fixed) · the `test.log` tally probe green (`lines 204 passed 1083
  failed 0 ignored 12`) · `bash .github/scripts/ci-leg.sh doc` green;
- `leg = 'operator'` (results in `evidence/ci.md`): `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py
  hygiene` — `hygiene: clean`; `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet &&
  git push origin build/escher-0.2.0` — green, `history moved: refs/remotes/origin/build/escher-0.2.0
  2f099735→14818317` (the intended push); `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion
  --sha HEAD --wait 1800` — `verdict: green · checks 17/17`, CI#39371468367; `gh run view <id> … --log | …` (`--id
  39371468367`) — `recorded`, its output the per-OS tallies above, every moved count explained by name in
  `evidence/ci.md` (disposed: green, this chunk's).
- smoke: skipped — no boot-path change (no binary or entry point; the UA select rules have no windowed self-verify
  target — the stand holds no select); the stand's headless boot ran as `agent-run.sh run stand`.
- No deferral; walk-class 0 marked.

Stated limits (plan step 17), for P5 and the masters: a `multiple` select and an `optgroup`'s options are not modelled
(a `multiple` select lays out as an empty bordered box and takes no key; its options still read in the tree); no popup
is painted and an option is not clicked (`click --id fx-apple` refused `covered`, measured); a checkbox, radio or text
input fires no engine `change` (vibey-script's synthesis stays); an option with no author key reading in
`unkeyed_actionable` is the plan's statement, not measured; a controlled app must write `selected` from its `input`
event (Deviation 6).

Watches: none folded.

Outcome basis: the operator pass ran — `git log --format='%h %s' 2f0997354..HEAD` holds `14818317c` (the pre-CI commit,
no fix commit); the final HEAD's CI run CI#39371468367 is recorded in `chunks/2026-10-20-select-by-id/evidence/ci.md`.
This window holds implement's run whole; its persisted report
`.andromeda/runs/2026-10-20T04-14-31-implement/outcome-2026-10-20-select-by-id.md` (the only `*-implement` dir for this
marker) agrees, moved from `surfaced` to green + operator pass by the operator's word. Post-implement artifacts:
`evidence/gate-12-amend.md`, `evidence/ci.md`.

Process hygiene: implement P4 census `census: none running` (escher-session, cargo, seven_guis_native, agent-run.sh,
`target/debug/deps/stand_*`); re-measured here, `gate.py census` (this run dir): `census: none running`. Two throwaway
probe targets (`zz_checked_probe`, `zz_debug`) ran to exit under `cargo test`, their sources removed.
## Addendum — resumed 2026-10-20
- inputs#I3 · ../additional/escher-overseer/relays/morning-founder-answers-2026-10-20.md · copy no-repo · unchanged —
  the founder's own word by question dialog in the overseer session (2026-10-20, about 04:35Z), relayed by the overseer
  with his chosen labels verbatim: «Select by id» F1–F4 «Утвердить все четыре (Рекомендую)» (F1 «Keys + typeahead» ·
  F2 «Engine shows its label» · F3 «Options as children» · F4 «`change` event» + «`<label>` names it» ratified);
  the upstream sync's `:focus-within` take «Утвердить (Рекомендую)» (ratified); «Visual lint» F2 «Без поля
  (Рекомендую)» (ratified), its P2 escalation «Record as built» confirmed with it; «Visual lint» F1 «Новая команда lint
  (Рекомендую)» (option 1, «New verb, all carriers, recorded»), replacing the delegate's «In process only (Most
  conservative)» — the block on `2026-10-19-visual-lint` clears and its gated part is built by `/andromeda-implement`'s
  gated mode, then its gated wrap.
  - bears on **Cross-project / external claims**: supersedes «delegate, PROVISIONAL for the founder's word» (I2's
    four answers) — the founder ratified them (inputs#I3).
  - bears on **Expected amendments (from plan)**: supersedes «answered by the delegate operator — PROVISIONAL
    («Provisional discharge»)» — the pattern «An engine behaviour change for every document» is answered by the
    founder (2026-10-20, inputs#I3); this chunk's amendments write the select model ratified, not PROVISIONAL.
  - bears on **Decisions & corrections**: adds to it — the founder's word on the three carried PROVISIONAL takes
    (`:focus-within`, visual lint F2 with «Record as built», visual lint F1) is a recorded ratification this wrap acts
    on: the masters' PROVISIONAL marks on the first two drop as ratifications (P2 apply), and F1 clears the visual
    lint's gated block (P5's gated re-check).
- inputs.py verify, re-fired before this addendum: `inputs: 3 entries — unchanged 1 · drifted 0 · vanished 0 ·
  broken 0 · altered 0 · unreachable 0 · n/a 2 · uncited 1 · unparsed 0`; moved since the report: entries 2 → 3, the
  one uncited row I3, cited by this addendum.

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 2f099735 (the parent of the oldest pre-CI commit 14818317) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### packages/blitz-dom/assets/default.css — added 12 line(s) in 3 range(s)
added: 77-78 · 84-87 · 155-160
- 84-86 «select {»
- 155-159 @157 «select:not([multiple]) > option:checked {»
### packages/blitz-dom/src/accessibility.rs — added 35 line(s) in 5 range(s)
added: 2-3 · 157-162 · 171-181 · 257-258 · 260-273
### packages/blitz-dom/src/document.rs — added 91 line(s) in 8 range(s)
added: 9-11 · 338-339 · 555 · 677-683 · 691 · 707 · 717-776 · 3126-3141
- 9-11 «use crate::node::{»
  - 717-728 @721 «pub fn select_value(&self, select: NodeId) -> Option<String> {»
  - 730-743 @731 «pub(crate) fn settle_select_choice(&mut self, select: NodeId) {»
  - 745-753 @747 «pub(crate) fn set_select_choice(&mut self, options: &[NodeId], target: Option<NodeId>) -> bool {»
  - 755-775 @757 «pub(crate) fn set_option_chosen(&mut self, option: NodeId, chosen: bool) -> bool {»
- 3126-3140 @3127 «fn is_labelable(element: &ElementData) -> bool {»
  - 3128-3139 «match element.name.local {»
### packages/blitz-dom/src/events/keyboard.rs — added 98 line(s) in 4 range(s)
added: 4-7 · 108-113 · 165-201 · 248-298
  - 4-7 «node::{»
- 165-200 @170 «fn press_select_key(»
  - 178-184 «let step = match &event.key {»
  - 185-196 «let target = match (&event.key, step) {»
  - 197-199 «if let Some(option) = target.and_then(|index| options.get(index).copied()) {»
  - 248-269 @249 «fn type_into_select(&mut self, select: NodeId, text: &str) -> String {»
  - 271-297 @274 «pub(crate) fn apply_select_choice(»
### packages/blitz-dom/src/events/mod.rs — added 4 line(s) in 2 range(s)
added: 93 · 246-248
### packages/blitz-dom/src/events/pointer.rs — added 14 line(s) in 1 range(s)
added: 857-870
### packages/blitz-dom/src/form.rs — added 17 line(s) in 3 range(s)
added: 6 · 271 · 276-290
### packages/blitz-dom/src/mutator.rs — added 54 line(s) in 8 range(s)
added: 58-60 · 90 · 321-324 · 473-481 · 1017-1020 · 1097-1101 · 1211-1215 · 1220-1242
  - 1220-1241 @1223 «fn set_option_selectedness(&mut self, node_id: NodeId, selected: bool) {»
### packages/blitz-dom/src/node/element.rs — added 7 line(s) in 1 range(s)
added: 455-461
### packages/blitz-dom/src/node/mod.rs — added 1 line(s) in 1 range(s)
added: 10
### packages/blitz-dom/src/node/select.rs — new file · 301 line(s)
- 17-26 @19 «pub(crate) struct OptionReading {»
- 28-39 @30 «pub(crate) enum SelectStep {»
- 41-50 @43 «pub(crate) struct Typeahead {»
- 55-58 @56 «pub(crate) fn is_single_select(element: &ElementData) -> bool {»
- 60-68 @61 «pub(crate) fn is_select_option(node: &Node) -> bool {»
  - 62-67 «node.data.is_element_with_tag_name(&local_name!("option"))»
- 70-86 @72 «pub(crate) fn options(doc: &BaseDocument, select: NodeId) -> Vec<NodeId> {»
  - 73-85 «doc.get_node(select)»
- 88-92 @89 «pub(crate) fn is_chosen(node: &Node) -> bool {»
  - 90-91 «node.element_data()»
- 94-100 @95 «pub(crate) fn chosen_option(doc: &BaseDocument, select: NodeId) -> Option<NodeId> {»
  - 96-99 «options(doc, select)»
- 102-115 @103 «pub(crate) fn readings(doc: &BaseDocument, options: &[NodeId]) -> Vec<OptionReading> {»
  - 104-114 «options»
- 117-127 @119 «pub(crate) fn option_label(node: &Node) -> String {»
  - 120-126 «match node»
- 129-139 @131 «pub(crate) fn option_value(node: &Node) -> String {»
  - 132-138 «match node»
- 141-145 @143 «fn option_text(node: &Node) -> String {»
- 147-153 @148 «fn strip_and_collapse(text: &str) -> String {»
  - 149-152 «text.split(|c: char| c.is_ascii_whitespace())»
- 155-164 @159 «pub(crate) fn settled(options: &[OptionReading]) -> Option<usize> {»
  - 160-163 «options»
- 166-181 @169 «pub(crate) fn stepped(options: &[OptionReading], step: SelectStep) -> Option<usize> {»
  - 174-179 «let found = match (step, current) {»
- 183-193 @185 «pub(crate) fn matched(options: &[OptionReading], text: &str) -> Option<usize> {»
  - 186-188 «if text.is_empty() {»
  - 190-192 «options»
- 195-301 @196 «mod tests {»
  - 202-211 @203 «fn options(rows: &[Row]) -> Vec<OptionReading> {»
  - 213-234 @214 «fn the_selectedness_setting_algorithm_keeps_one_chosen_option() {»
  - 236-263 @237 «fn a_step_skips_disabled_options_and_stops_at_the_ends() {»
  - 265-287 @266 «fn typed_text_matches_the_first_enabled_label_it_starts() {»
  - 289-300 @290 «fn text_is_stripped_and_collapsed() {»
### packages/blitz-traits/src/events.rs — added 8 line(s) in 7 range(s)
added: 150 · 202 · 250-251 · 307 · 354 · 402 · 449
### packages/blitz-vibey-script/src/runtime.rs — added 2 line(s) in 1 range(s)
added: 1672-1673
### packages/dioxus-native-dom/src/dioxus_document.rs — added 6 line(s) in 1 range(s)
added: 377-382
### packages/dioxus-native-dom/src/snapshot.rs — added 113 line(s) in 7 range(s)
added: 7 · 144-152 · 155 · 176 · 241-251 · 253-259 · 932-1014
- 241-250 @242 «fn select_parent(doc: &BaseDocument, node: &Node) -> Option<NodeId> {»
  - 244-248 «(node.data.is_element_with_tag_name(&local_name!("option"))»
  - 257-259 «if &*element.name.local == "select" {»
  - 936-953 @939 «const SELECT_VALUES: [(&[SelectOption], Option<&str>); 4] = [»
  - 955-975 @956 «fn a_select_reads_its_chosen_options_value() {»
  - 977-1014 @978 «fn a_selects_options_are_its_children_named_by_their_labels() {»
### packages/escher-driver/src/schema.rs — added 2 line(s) in 1 range(s)
added: 208-209
### packages/escher-driver/src/schema_tests.rs — added 2 line(s) in 1 range(s)
added: 77-78
### tests/blitz-tests/tests/accessibility_label.rs — added 143 line(s) in 2 range(s)
added: 1 · 299-440
- 299-307 @300 «fn tree_id(document: &HtmlDocument, selector: &str) -> NodeId {»
  - 301-305 «let node = document»
- 309-348 @310 «fn a_label_for_names_a_select_and_a_textarea() -> TestResult<()> {»
  - 311-346 «for incremental in [false, true] {»
- 350-388 @351 «fn a_wrapping_label_names_a_select_and_a_textarea() -> TestResult<()> {»
  - 352-386 «for incremental in [false, true] {»
- 390-432 @391 «fn a_label_names_no_hidden_input() -> TestResult<()> {»
  - 392-430 «for incremental in [false, true] {»
- 434-439 «fn config_in(incremental: bool) -> DocumentConfig {»
  - 435-438 «DocumentConfig {»
### tests/blitz-tests/tests/select_interaction.rs — new file · 343 line(s)
- 23-65 @28 «fn page() -> Element {»
  - 32-64 «rsx! {»
- 69-73 @70 «fn value(harness: &Page, selector: &str) -> Option<String> {»
- 75-85 @76 «fn counted(harness: &Page) -> (u32, u32) {»
  - 77-83 «let read = |selector: &str| {»
- 87-98 @89 «fn measured(harness: &mut Page, act: impl FnOnce(&mut Page)) -> ((u32, u32), bool) {»
  - 94-97 «(»
- 100-111 @101 «fn focused_pick(incremental: bool) -> Page {»
  - 105-109 «assert_eq!(»
- 113-147 @114 «fn a_select_lays_out_as_a_box_holding_its_chosen_option() {»
  - 115-146 «for incremental in [false, true] {»
- 149-168 @150 «fn the_choice_is_seeded_from_the_markup() {»
  - 151-157 @152 «const SEEDED: [(&str, &str); 4] = [»
  - 158-167 «for incremental in [false, true] {»
- 170-187 @172 «const KEYS: [(Key, Modifiers, &str, u32); 14] = [»
- 189-214 @190 «fn the_arrows_home_and_end_choose_an_enabled_option() {»
  - 192-213 «for incremental in [false, true] {»
- 216-226 @218 «const TYPED: [(&str, &str, u32); 7] = [»
- 228-249 @229 «fn typed_text_chooses_the_first_enabled_option_its_label_starts() {»
  - 231-248 «for incremental in [false, true] {»
- 251-280 @252 «fn typed_text_is_kept_for_one_second_of_the_documents_clock() {»
  - 253-279 «for incremental in [false, true] {»
- 282-312 @283 «fn a_disabled_select_takes_no_key_and_no_click() {»
  - 284-311 «for incremental in [false, true] {»
- 314-343 @315 «fn an_app_writing_selected_on_another_option_re_seeds_the_choice() {»
  - 316-342 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_act_select.rs — new file · 256 line(s)
- 21-43 @23 «fn page() -> Element {»
  - 24-42 «rsx! {»
- 48-50 «fn hold(incremental: bool) -> Session {»
- 52-60 @53 «fn value(session: &Session, id: &str) -> Option<String> {»
  - 54-59 «session»
- 62-77 @63 «fn tab_walk(session: &mut Session, mode: &str) -> Vec<String> {»
  - 66-75 «for row in 0..TAB_WALK.len() {»
- 79-132 @80 «fn at_boot_the_snapshot_reads_the_select_its_name_value_and_options() {»
  - 81-131 «for incremental in [false, true] {»
- 134-167 @135 «fn type_with_a_label_chooses_that_option() {»
  - 136-166 «for incremental in [false, true] {»
- 172-198 @173 «fn a_click_then_a_key_chooses_an_option() {»
  - 174-197 «for incremental in [false, true] {»
- 200-233 @201 «fn the_tab_walk_skips_the_disabled_select_before_and_after_the_choices() {»
  - 202-232 «for incremental in [false, true] {»
- 235-256 @236 «fn a_disabled_select_and_an_unchosen_option_are_refused() {»
  - 237-255 «for incremental in [false, true] {»
