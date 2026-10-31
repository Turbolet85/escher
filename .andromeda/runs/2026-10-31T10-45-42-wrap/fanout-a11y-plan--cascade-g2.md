proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 8
    section: "## 5. Keyboard Navigation"
    change: Replace the closing clause, from «a known limit stands» to the end of the line with its residual owner, with «HTML's focus fixup is built since 2026-10-31-engine-debts, in every document, so a node that holds focus when it becomes inert gives focus up at the flush, no DOM `blur` or `focusout` firing — ratified by the founder (2026-10-31, «Снимать фокус без blur (Рекомендую)», relayed verbatim by the operator)»; the rest of the line stands.
    sidecar: keyboard-navigation#1.1 — the focus-fixup limit and its residual owner retired, the fixup built (2026-10-31-engine-debts).
    rationale: Report Symbols / APIs F3 builds the fixup and fires no blur; Retired names `inert_subtree`'s focus-fixup limit lines; Counts / qualifiers moved has `focus-fixup` changed at `keyboard-navigation.md:8`; the P5 card's row 5 absorbs the residual lines into `v030-04`.
    basis: packages/blitz-dom/src/mutator.rs:1051-1063
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 8
    section: "## 5. Keyboard Navigation"
    change: Add after the line, in the group's shape → - Since 2026-10-31-engine-debts, in every document, `DocumentMutator::flush` calls `BaseDocument::clear_focus` where `focus_node_id` resolves to a node reading `is_inert()`, with no `inert` arm joining the attribute writes, so a subtree moved under an inert box is covered too; no DOM `blur` or `focusout` fires, and `disabled` and `hidden` read as before (packages/blitz-dom/src/mutator.rs:1051-1063; tests/blitz-tests/tests/inert_subtree.rs:229-310) — ratified by the founder (2026-10-31, «Снимать фокус без blur (Рекомендую)», relayed verbatim by the operator) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
    sidecar: keyboard-navigation — the focus fixup's rule added beside the focusability entity (2026-10-31-engine-debts).
    rationale: Report Symbols / APIs F3 states the rule and Outcome (F3, a11y) meets it with `inert_subtree` 6; the entity beside it holds the limit this rule ends.
    basis: packages/blitz-dom/src/mutator.rs:1051-1063
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 12
    section: "## 5. Keyboard Navigation"
    change: Add after the line, in the group's shape → - Since 2026-10-31-engine-debts, in every document, a select's options include its `optgroup` children's in tree order, reachable by keys and text, and an option's disabled-ness is one rule, `option_disabled`, which `readings` and the form's entries share, so a disabled group's rows read disabled (packages/blitz-dom/src/node/select.rs:175-190; packages/blitz-dom/src/form.rs:281; tests/blitz-tests/tests/select_interaction.rs:857-894) — ratified by the founder (2026-10-31, «Построить сейчас», relayed verbatim by the operator) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
    sidecar: keyboard-navigation — the options under a group and their one disabled rule added beside the disabled-ness entity (2026-10-31-engine-debts).
    rationale: Report Symbols / APIs F7 states `options`, `owner_select` and `option_disabled`, the one rule `readings` and `form.rs:281` share; the entity beside it keys the disabled-ness of form controls and holds no option's.
    basis: packages/blitz-dom/src/node/select.rs:175-190
    fact: F6
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 25
    section: "## 5. Keyboard Navigation"
    change: Narrow «while an option is not clicked» to an option of a closed single select, under a group too, the driver's `covered` refusal and the flight booker's two options standing with it, and state beside it that a `multiple` select's row takes a click through `handle_click`'s `option` arm; «a click on any other element is unchanged» and the rest of the line stand.
    sidecar: keyboard-navigation#2.3 — the option-click limit narrowed to a closed select's option, a `multiple` select's row taking a click (2026-10-31-engine-debts).
    rationale: Report Symbols / APIs F6 adds `click_select_row` behind a new `option` arm and F8 keeps a closed single select's option refused `covered`; `option-click` narrows `keyboard-navigation.md:25`; the row is a new interactive element the line now denies.
    basis: packages/blitz-dom/src/events/pointer.rs:1013-1054
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 25
    section: "## 5. Keyboard Navigation"
    change: Add after the line, in the group's shape → - Since 2026-10-31-engine-debts, in every document, `click_select_row` runs behind `handle_click`'s `option` arm on a `multiple` select's row — a disabled select stops the click; otherwise it focuses the select and clears the typeahead and the text selection; a plain click chooses the row alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing (packages/blitz-dom/src/events/pointer.rs:1013-1054; tests/blitz-tests/tests/select_interaction.rs:601-664; tests/blitz-tests/tests/stand_act_select.rs:354-379) — ratified by the founder (2026-10-31, «Построить сейчас», relayed verbatim by the operator) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
    sidecar: keyboard-navigation — `click_select_row`'s rule added beside the pointer click entity (2026-10-31-engine-debts).
    rationale: Report Symbols / APIs F6, `events/pointer.rs`, states the rule; Coverage of new surfaces lists `select_interaction` and `stand_act_select` for it.
    basis: packages/blitz-dom/src/events/pointer.rs:1013-1054
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 25
    section: "## 5. Keyboard Navigation"
    change: Add after the line, in the group's shape → - Since 2026-10-31-engine-debts, Enter and Space open nothing on a single select, and an option of a closed single select has no click arm and stays refused `covered`, under a group too, no popup being built (tests/blitz-tests/tests/select_interaction.rs:355-380; tests/blitz-tests/tests/stand_act_select.rs:448-457) — ratified by the founder (2026-10-31, «Давай нарисовать в отдельном чанке» and «Только строки multiple (Рекомендую)», relayed verbatim by the operator) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
    sidecar: keyboard-navigation — the closed select's standing limits added beside the pointer click entity (2026-10-31-engine-debts).
    rationale: Report Symbols / APIs F8 states both limits; the entity beside it holds the option-click reading this fact restates.
    basis: tests/blitz-tests/tests/stand_act_select.rs:448-457
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 28
    section: "## 5. Keyboard Navigation"
    change: Replace «no test asserts `:open`» with «`open_state`, 4 tests in both layout modes, asserts the `open` attribute read by presence at construction, a write and a removal through the mutator moving it, and a summary's click and Enter flipping its `details` — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»; the rest of the line stands.
    sidecar: keyboard-navigation#2.6 — the `:open` gap closed by `open_state` (2026-10-31-engine-debts).
    rationale: Report Counts / qualifiers moved, gaps CARRY 2 closes, has `no-assert-gap` changed at `keyboard-navigation.md:28`; New targets list `open_state` 4; Outcome (CARRY 2) meets it.
    basis: tests/blitz-tests/tests/open_state.rs:54-143
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 29
    section: "## 5. Keyboard Navigation"
    change: Add after the line, in the group's shape → - Since 2026-10-31-engine-debts, in every document, after `input` `handle_click`'s checkbox and radio arms dispatch a `DomEventData::Change` with the same value, a checkbox always, a radio only where it was not checked before the click, and the script runtime's `change` synthesis is gone, so a page's script hears one `change`, from the engine (packages/blitz-dom/src/events/pointer.rs:793-799; packages/blitz-dom/src/events/pointer.rs:811; packages/blitz-dom/src/events/pointer.rs:827-836; tests/blitz-tests/tests/checkable_change_event.rs:68-119) — ratified by the founder (2026-10-31, «Шлёт движок (Рекомендую)», relayed verbatim by the operator) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
    sidecar: keyboard-navigation — the engine's `change` on a checkbox and a radio added beside the checkable click entity (2026-10-31-engine-debts).
    rationale: Report Symbols / APIs F10 states the dispatch and the loss of the runtime's synthesis, Retired names that synthesis, and `checkable_change_event` 3 asserts it; the entity beside it names `input` alone.
    basis: packages/blitz-dom/src/events/pointer.rs:793-799
    fact: F10
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 107
    section: "## 5. Keyboard Navigation"
    change: Replace the clause from «HTML's focus fixup is not built» to the end of the line with «HTML's focus fixup is built, so a node focused as it becomes inert gives focus up at the flush, no DOM `blur` or `focusout` firing.»; the rest of the line stands.
    sidecar: a11y-plan §5 — the broad line's focus-fixup limit and residual owner retired (2026-10-31-engine-debts).
    rationale: Report Symbols / APIs F3 and Retired; `focus-fixup` changed at `a11y-plan.md:107`; the broad line of the entity at `keyboard-navigation.md:8`.
    basis: packages/blitz-dom/src/mutator.rs:1051-1063
    dependent-of: D-retired
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 113
    section: "## 5. Keyboard Navigation"
    change: After «a click on a `select` focuses it» add «, a click on a `multiple` select's row focuses the select and acts on the row, and an option of a closed single select takes no click»; the rest of the line stands.
    sidecar: a11y-plan §5 — the broad click rule gains a `multiple` select's row and a closed select's option (2026-10-31-engine-debts).
    rationale: Report Symbols / APIs F6, `events/pointer.rs`, and F8; the broad line of the entity at `keyboard-navigation.md:25`, whose «any other element is unchanged» would otherwise read over the new `option` arm.
    basis: packages/blitz-dom/src/events/pointer.rs:1013-1054
    dependent-of: D-a11y-surface
note: This listing ends at `keyboard-navigation.md` line 37, so the report's changes at `keyboard-navigation.md:38` and `:102` and at `a11y-plan.md:123`, `:142` are not proposed here and F1 to F4, F8 and F11 to F16 stay with another group or the new-detail pass, F6 and F7 here overlapping any carrier at line 38; D-a11y-obs-schema does not fire, the report moving no log site or schema.
