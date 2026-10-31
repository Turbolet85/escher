proposals:
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 64
    section: "## Established Decisions"
    change: "In the third direct engine change, replace «and from neither on a native checkbox or radio (ratified by the founder, 2026-10-18)» with «and from neither attribute on a native checkbox or radio (ratified by the founder, 2026-10-18): a native control under a checkable role writes `Toggled::True` or `False` from its own `checkbox_input_checked()`, and under any other role writes nothing (ratified by the founder, 2026-10-31, «Писать на checkable-ролях (Рекомендую)», by dialog in the overseer session, relayed verbatim by the operator)»."
    sidecar: "dec-scrolling-and-input: a native checkable control's `toggled` now reads its checked state on a checkable role; neither author attribute is read on it."
    rationale: "Report, Symbols / APIs, fork F5: `build_accessibility_node` writes `Toggled` where `is_native_checkable && is_checkable(role)`, so «from neither» no longer reads whole; the founder's word moves the 2026-10-18 reading."
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 68
    section: "## Established Decisions"
    change: "Replace «a click on a `select` focuses it through its own arm of the walk (below), and a click on any other element still clears focus» with «a click on a `select` focuses it through its own arm of the walk (below), a click on a `multiple` select's row focuses that select through the walk's `option` arm (below), and a click on any other element still clears focus»."
    sidecar: "dec-scrolling-and-input: `handle_click`'s walk gains an `option` arm for a `multiple` select's row, which focuses the select."
    rationale: "Report, Symbols / APIs, fork F6, `events/pointer.rs`: `click_select_row` stands behind a new `option` arm of `handle_click` and focuses the select, so a row is no longer «any other element»."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 75
    section: "## Established Decisions"
    change: "- Eight more direct engine changes for every document stand beside those seven, each ratified by the founder's own word (2026-10-31, by dialog in the overseer session, relayed verbatim by the operator): `visible_region` reading a clipping box through its transforms and a non-invertible transform taking no hit (§Standard Contracts → Scrolling, selection, tree); the focus fixup ([DOM semantics]); the tree's focus falling back to the window and a native control's `toggled` on a checkable role (§Standard Contracts → Mutation, query and CSSOM); the `multiple` select and `optgroup` (below; [DOM semantics]); and the engine's `change` on a checkbox and a radio (§Standard Contracts → Events). Each is flagged upstreamable and nothing is sent; upstream `0980cce5f` has no hunk on `visible_region`, and its `hit_inner` hunk is a textual conflict with the non-invertible answer for Epoch 3's sync, whose line owns their merge surface (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/merge-surface.md)."
    sidecar: "dec-scrolling-and-input: records the eight direct engine changes of 2026-10-31, flagged upstreamable, nothing sent, and upstream's `hit_inner` conflict for Epoch 3's sync."
    rationale: "Report, Symbols / APIs (forks F1 to F7 and F10), Cross-project claims (Upstream `ebacd68bd`) and Expected amendments, the eight engine changes and their merge surface."
    fact: F25
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 60
    section: "## Established Decisions"
    change: "Replace «and the select model of keys, typeahead and click, each flagged upstreamable» with «the select model of keys, typeahead and click, and the eight of 2026-10-31 — `visible_region` through transforms, a non-invertible transform's no hit, the focus fixup, the tree's focus fallback, native `toggled`, the `multiple` select, `optgroup` and the engine's `change` on a checkbox and a radio — each flagged upstreamable»."
    sidecar: "Body, [Scrolling and input]: the broad tally of direct engine changes gains the eight of 2026-10-31."
    rationale: "Report, Symbols / APIs and Expected amendments: the eight engine changes the broad line's tally now has to hold, flagged upstreamable with nothing sent."
    dependent-of: D-new-detail
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 77
    section: "## Established Decisions"
    change: "Replace «Its limits: no popup is painted, an option is not clicked (a driver `click` on one is refused `covered`), and a `multiple` select or an `optgroup`'s options are not modelled.» with «Its limits: no popup is painted, and an option of a closed single select is not clicked (a driver `click` on one is refused `covered`, under a group too).»"
    sidecar: "dec-scrolling-and-input: the select model's `multiple` and `optgroup` limits end; the option-click limit narrows to a closed single select's option."
    rationale: "Report, Counts / qualifiers moved: `multiple-select`, `optgroup` and `option-click` each change `established-decisions.md:77`; forks F6, F7 and F8 under Symbols / APIs."
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 77
    section: "## Established Decisions"
    change: "Replace «Every choice that changes the chosen option goes through the one path `apply_select_choice`» with «Every choice that changes the chosen option or a `multiple` select's chosen set goes through the one path `apply_select_choice`, which takes the set»."
    sidecar: "dec-scrolling-and-input: `apply_select_choice` takes a set and stays the one write path."
    rationale: "Report, Symbols / APIs, fork F6, `events/keyboard.rs`: `apply_select_choice` takes a set and stays the one write path."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: "## Established Decisions"
    change: "- A `multiple` select's keys, ratified by the founder (2026-10-31, «Построить сейчас», by dialog in the overseer session, relayed verbatim by the operator): `ArrowDown`, `ArrowUp`, `Home` and `End` alone choose that row alone, with Shift span from the anchor, and with Ctrl or Meta move the current row and change nothing; a Space flips the current row with no typed text standing and joins the text otherwise; a character with no Ctrl, Alt or Meta chooses its match alone; disabled rows are never chosen; Ctrl+A is not modelled (packages/blitz-dom/src/events/keyboard.rs:213-244; packages/blitz-dom/src/events/keyboard.rs:326-332; packages/blitz-dom/src/events/keyboard.rs:334-349; packages/blitz-dom/src/events/keyboard.rs:351-380)."
    sidecar: "dec-scrolling-and-input: records a `multiple` select's keys."
    rationale: "Report, Symbols / APIs, fork F6, `events/keyboard.rs`."
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: "## Established Decisions"
    change: "- A `multiple` select's pointer: `click_select_row`, behind an `option` arm of `handle_click`, focuses the select and clears the typeahead and the text selection; a plain click chooses the row alone, Ctrl or Meta flips it, Shift spans, Shift with no cursor yet spans from the last chosen row, and a disabled row or a disabled select changes nothing (packages/blitz-dom/src/events/pointer.rs:1013-1054)."
    sidecar: "dec-scrolling-and-input: records a `multiple` select's pointer rules."
    rationale: "Report, Symbols / APIs, fork F6, `events/pointer.rs`, and Deviations 6b."
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: "## Established Decisions"
    change: "- No popup is built (the founder, 2026-10-31, «Давай нарисовать в отдельном чанке», «Только строки multiple (Рекомендую)» and, on `size`, «В чанк с попапом (Рекомендую)», relayed verbatim by the operator): Enter and Space open nothing on a single select, an option of a closed single select has no click arm and stays refused `covered`, under a group too, `size` is unread, and the popup is a later chunk's."
    sidecar: "dec-scrolling-and-input: records that no popup is built, what a closed select's option still reads, and that `size` waits for the popup's chunk."
    rationale: "Report, Symbols / APIs, fork F8; Reverted / negative API facts (no `size` read); Decisions & corrections (the founder's word on `size`)."
    fact: F14
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture.md
    near: 62
    section: "## Established Decisions"
    change: "This line reads: - The select model: every choice that changes the chosen option or a `multiple` select's chosen set goes through the one path `apply_select_choice`, and a choice of the chosen option fires nothing; a `multiple` select and an `optgroup`'s options are modelled in keys, pointer, form, tree, sheet, snapshot and bridge; no popup is painted, and an option of a closed single select is not clicked."
    sidecar: "Body, [Scrolling and input]: the select model's broad line drops the `multiple` and `optgroup` limits and narrows the option-click limit."
    rationale: "Report, Counts / qualifiers moved: `multiple-select`, `optgroup` and `option-click` each change `architecture.md:62`."
    dependent-of: D-arch-decisions
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 99
    section: "## Established Decisions"
    change: "Replace «its test file not asserting both layout modes (§Conventions → Tests)» with «its test file asserting both layout modes, each of its three tests running `for incremental in [false, true]` (§Conventions → Tests)»."
    sidecar: "dec-layout: upstream's both-modes exception ends for `pseudo_element_flow_classification`."
    rationale: "Report, Retired: `upstream's both-modes exception`, the file's three tests now run in both modes; the same fact as the `flow-class` rows, which this occurrence joins."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 113
    section: "## Established Decisions"
    change: "Remove the sentence «A known limit: HTML's focus fixup is not built — a node that holds focus when it becomes inert keeps it until focus moves (pinned by `inert_subtree::removing_inert_restores_focus_hit_and_tree`), its fix owned by a `.andromeda/residuals.md` line (target: next).»; what replaces it is the F3 line beside this one."
    sidecar: "dec-dom-semantics: the known focus-fixup limit and its residual owner end."
    rationale: "Report, Retired: the test's focus-fixup limit lines; Counts / qualifiers moved: `focus-fixup` changes `established-decisions.md:113`; fork F3."
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 113
    section: "## Established Decisions"
    change: "Replace «and no mutator hook is added» with «and no `inert` arm joins the attribute writes»."
    sidecar: "dec-dom-semantics: the mutator's flush now reads `is_inert`, so the inert rule's «no mutator hook» narrows to the attribute writes."
    rationale: "Report, Symbols / APIs, fork F3: `DocumentMutator::flush` reads `is_inert()` and no `inert` arm joins the attribute writes, so «no mutator hook» no longer reads true as written."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 113
    section: "## Established Decisions"
    change: "- HTML's focus fixup stands without its events, ratified by the founder (2026-10-31, «Снимать фокус без blur (Рекомендую)», by dialog in the overseer session, relayed verbatim by the operator): `DocumentMutator::flush` calls `BaseDocument::clear_focus` where `focus_node_id` resolves to a node reading `is_inert()`, firing no DOM `blur` or `focusout`, with no `inert` arm in the attribute writes, so a subtree moved under an inert box is covered too; `disabled` and `hidden` read as before (packages/blitz-dom/src/mutator.rs:1051-1063), held by `inert_subtree::a_node_made_inert_loses_focus_and_takes_no_key`."
    sidecar: "dec-dom-semantics: records the focus fixup, cleared at the mutator's flush with no `blur` or `focusout`."
    rationale: "Report, Symbols / APIs, fork F3; `inert_subtree`'s new test in New text, by line."
    fact: F3
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture.md
    near: 74
    section: "## Established Decisions"
    change: "This line reads: - The focus fixup: a node that holds focus when it comes under `inert` gives it up at the mutator's flush, with no `blur` or `focusout`."
    sidecar: "Body, [DOM semantics]: the focus-fixup limit's broad line becomes the fixup's own, its residual owner gone."
    rationale: "Report, Counts / qualifiers moved: `focus-fixup` changes `architecture.md:74`; Retired: the limit lines; fork F3."
    dependent-of: D-retired
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 115
    section: "## Established Decisions"
    change: "Replace «a select's options are its `option` children in tree order (an `optgroup`'s are not listed)» with «a select's options are its `option` children and its `optgroup` children's, in tree order»."
    sidecar: "dec-dom-semantics: a select's options now run through its groups."
    rationale: "Report, Symbols / APIs, fork F7: `options` walks a select's `option` children and its `optgroup` children's, in tree order; Counts / qualifiers moved: `optgroup` changes `established-decisions.md:115`."
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 115
    section: "## Established Decisions"
    change: "Replace «a `selected` write sets that option chosen and its select's other options not» with «a `selected` write sets that option chosen and, on a single-choice select, its select's other options not, leaving a `multiple` select's others alone»."
    sidecar: "dec-dom-semantics: a `selected` write leaves a `multiple` select's other options alone."
    rationale: "Report, Symbols / APIs, fork F6, `mutator.rs`: `set_option_selectedness` leaves a `multiple` select's other options alone."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 116
    section: "## Established Decisions"
    change: "Replace «keeps an option whose parent is a `select` although it computes `display: none`» with «keeps an option whose parent is a `select`, or an `optgroup` under one, although it computes `display: none`»."
    sidecar: "dec-dom-semantics: the accessibility builder keeps a select's option under a group too."
    rationale: "Report, Symbols / APIs, fork F7: `is_select_option` holds under a group; `stand_act_select`'s `an_option_under_a_closed_selects_group_is_refused_covered` reads such an option, so it stands in the tree."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 116
    section: "## Established Decisions"
    change: "Replace «so a select lays out as a box holding its chosen option» with «so a single-choice select lays out as a box holding its chosen option»."
    sidecar: "dec-dom-semantics: the box-holding-its-chosen-option reading narrows to a single-choice select."
    rationale: "Report, Schema / config: the default sheet now lays a `multiple` select's rows and groups out as `display: block`, so the sentence holds of a single-choice select alone."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 116
    section: "## Established Decisions"
    change: "- `optgroup`, ratified by the founder (2026-10-31, «Построить сейчас», relayed verbatim by the operator): `options` walks a select's `option` children and its groups' in tree order, `is_select_option` holds under a group, `option_disabled` is the one rule `readings` and `form.rs` share, `maybe_record_node`'s `optgroup` arm records its select, and the tree gives a group `Role::Group` named by its non-empty `label`, which stays unpainted (packages/blitz-dom/src/node/select.rs:175-190; packages/blitz-dom/src/form.rs:281; packages/blitz-dom/src/mutator.rs:1253-1257)."
    sidecar: "dec-dom-semantics: records the `optgroup` model."
    rationale: "Report, Symbols / APIs, fork F7."
    fact: F12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 116
    section: "## Established Decisions"
    change: "- The default sheet lays out `optgroup` and `multiple` rows: `select:not([multiple]) > optgroup:not([hidden]) { display: contents; }`, `select:not([multiple]) > optgroup > option:checked { display: inline; }`, `select[multiple] > option, select[multiple] > optgroup > option { display: block; padding: 0 2px; white-space: nowrap; }` and `select[multiple] > optgroup { display: block; }`, and adds the engine-default chosen-row pair `select[multiple] > option:checked, select[multiple] > optgroup > option:checked { background-color: #c8c8c8; color: #000000; }`, 12.56:1 as `select_interaction` measures it, Chromium's inactive selected pair, hardcoded since the engine holds no tokens (packages/blitz-dom/assets/default.css:161-188)."
    sidecar: "dec-dom-semantics: records the default sheet's `optgroup` and `multiple` rows and the chosen-row pair."
    rationale: "Report, Schema / config and Coverage of new surfaces (the first row's tokens)."
    fact: F19
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture.md
    near: 72
    section: "## Established Decisions"
    change: "Replace «a `select`'s choice is held as its options' `CHECKED` state, settled by HTML's selectedness algorithm, and a controlled app» with «a `select`'s choice, a `multiple` select's chosen set among them, is held as its options' `CHECKED` state, a single-choice select's settled by HTML's selectedness algorithm; its options include its `optgroup` children's; and a controlled app»."
    sidecar: "Body, [DOM semantics]: a select's choice line covers a chosen set and its groups' options."
    rationale: "Report, Symbols / APIs, forks F6 and F7: a `multiple` select's set and `options` through a group."
    dependent-of: D-arch-decisions
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 123
    section: "## Established Decisions"
    change: "Replace «with no test of its own,» with «held since 2026-10-31-engine-debts by its own `open_state` (4 tests: the attribute at construction, a write and a removal through the mutator, a `summary`'s click and its Enter),»."
    sidecar: "dec-dom-semantics: `:open` gains its own test target, `open_state`."
    rationale: "Report, Counts / qualifiers moved: new target `open_state` 4 and «Gaps CARRY 2 closes»; Outcome, CARRY 2; its four tests in New text, by line."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 141
    section: "## Established Decisions"
    change: "Replace «held by the windowed `seven_guis_native` smoke boot and `focus_redraw`» with «held by the windowed `seven_guis_native` smoke boot, `focus_redraw` and blitz-shell's first unit test, `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once`, which states what it cannot read»."
    sidecar: "dec-shell-and-windowing: the redraw route gains blitz-shell's first unit test."
    rationale: "Report, Counts / qualifiers moved: `redraw-route` changes `established-decisions.md:141`; Crates / modules: `blitz-shell/src/event.rs` (`:128-172`), its first unit test; Outcome, CARRY 2."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 153
    section: "## Established Decisions"
    change: "Replace the clause «`change` synthesised after `input` on checkbox/radio» with «a checkbox's or radio's `change` comes from the engine, the runtime synthesising none, so a page's script hears one `change`»."
    sidecar: "dec-javascript: the runtime's `change` synthesis is retired; a page's script hears the engine's one `change`."
    rationale: "Report, Retired: `runtime/events.rs`'s `change` synthesis and `target_is_checkbox_or_radio`; Symbols / APIs, fork F10; Counts / qualifiers moved: `change-event` changes `established-decisions.md:153`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 159
    section: "## Established Decisions"
    change: "- A measured hazard, no engine change: a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, and a `MountedData` call in it panics at `events.rs:162` (`RefCell already borrowed`); Cells calls its handles synchronously for that reason, and no engine line moved. Any app meets it; measured headlessly through the harness's `poll(None)`, the windowed shell's same lines read and not run (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/readings.md)."
    sidecar: "dec-dioxus-integration: records the `MountedData` hazard of a task spawned from a key handler."
    rationale: "Report, Reverted / negative API facts (step 15's async form); Decisions & corrections (a hazard outside this chunk's fixes); Spec claims disproved, `mounted-hazard`: no master states the form."
    fact: F21
note: "#15.1's rule on removed nodes is left as it reads because the report states nothing of what `select_cursor` does when its node is removed; the citation beside #22.1's retired clause is left to the citation sweep and the orchestrator; the report's fork numbers F1 to F13 are not the task's fact numbers F1 to F26."
