# Task — a11y · 2026-10-27-readme-and-records

## The chunk
A records-only chunk with no code, test, engine, driver, CI or schema change. No verb, field, carrier, log site, span or count moves.
The README's §Status shows 0.1.0 and 0.2.0 done and verified, the latter with what it can do and its cold-agent result, and 0.3.0 planned in six stages. Its other sections ("What Escher is", the driver paragraph, "Try it") name what 0.2.0 added, and «Escher» takes a capital E in prose.
The capabilities statement is restated against the 2026-10-25 cold-agent live run: six apps, twelve tools, 66 calls, 0 wrong, a screenshot and a lint called. The chunk re-reads the statement's pin, which stays green.
Two verified ledger refs are re-pointed to the tests' current names. The design-system's four stand rows (the accent, the success row, the cards item, the stand's DejaVu Sans item) move into architecture and layout-templates as an expected wrap amendment. The incubator ignore is witnessed at `.gitignore:39`.

## Reached files
- README.md · edits §Status, the "Later:" line, "What Escher is", the driver paragraph and "Try it"
- docs/capabilities.md · edits the cold-agent run paragraph (lines 288-291) and any other line naming the 2026-10-15 run as the latest
- .github/scripts/test_capability_statement.py · re-reads the statement's pin (verb and cause names, five counts, README's tool count)
- escher-0.1.0/verification-matrix.json · edits v010-17's verified ref
- escher-0.2.0/verification-matrix.json · edits v020-05's verified ref
- tests/blitz-tests/tests/stand_snapshot_state.rs · re-reads the test name `exactly_the_chosen_flight_type_reads_selected`
- tests/blitz-tests/tests/stand_lint.rs · re-reads the test name `the_stand_at_boot_reads_its_measured_findings`
- .andromeda/design-system.md · edits the accent row, the success row, the cards item and the stand's DejaVu Sans item (moved out, an expected wrap amendment)
- .andromeda/architecture.md · edits the section the stand rows land in (an expected wrap amendment)
- .andromeda/layout-templates.md · edits the section the stand rows land in (an expected wrap amendment)
- .gitignore · re-reads the incubator ignore line 39 (`git check-ignore -v`)
- .git/info/exclude · edits its two trailing incubator lines
- .andromeda/residuals.md · re-reads lines 41 and 42 (absorbed:v030-02, absorbed:v030-16)
- .andromeda/security-plan.md · re-reads the 2026-10-25 run's recorded crossing

## Needs
- The stand's accent row moves out of design-system into architecture and layout-templates with its colour and measured contrast unchanged (white on `#3a5ce5`, 5.47; the old `#4a6cf7` read 4.39). No stand colour changes, so `stand_contrast`'s control, text and state tables do not move.
- The stand's success row moves the same way: its colour and the contrast record it carries for SC 1.4.3 are restated, never changed.
- The stand's DejaVu Sans item moves with its role intact: the stand's bundled font, which the contrast reader's font size and weight and the screenshot's and lint's `fonts-unavailable` gate depend on.
- Every accessibility claim the README or the capabilities statement writes stays inside the measured tier-0 target, SC 2.1.1 · 1.4.3 · 2.4.3. SC 1.4.3 is met on every measured non-exempt stand row, and no wider claim is made.
- The pages the stand serves beside the lean four (Cells, the load page) hold no Tab-order, keyboard-activation or contrast row. SC 2.1.1 and 1.4.3 are unasserted on the load page's `load-fetch` Button. A README or statement line about the 2026-10-25 run's six apps claims no keyboard or contrast coverage of them.
- The windowed assistive-technology witness is not reached: no stand binary builds a platform adapter, and the residual line reads `re-carried:0.2.0`. The README states no screen-reader or platform-adapter result.
- The visual lint as the README names it: its `contrast` defect reads through `Harness::text_contrast` under `stand_contrast`'s exempt rule. A finding holds ids, closed words and numbers only, and the prose states no second formula or threshold.
- A `select` driven by id as the README names it: the engine's single-choice select model (arrow, `Home` · `End` and typeahead keys, its `input` then `change`, no popup, no `multiple`), named only as the capabilities statement states it.
- The snapshot's readings as the README names them: role and name from the accessibility tree, a password's or file input's value as `MASKED_VALUE`, and `selected` · `pressed` · `invalid` from ARIA state attributes. The prose names no second role or name mapping.
- Ledger ref v010-17 is re-pointed to `stand_snapshot_state::exactly_the_chosen_flight_type_reads_selected`. It reflects the flight booker's mode as a `select` whose options state the choice by `selected`, the stand stating no `aria-pressed`.
- Ledger ref v020-05 is re-pointed to `stand_lint::the_stand_at_boot_reads_its_measured_findings`. At boot the stand reads the flight booker's one `overlap` (a select's options given its client rect), not zero findings.
- The README's 0.3.0 stages and "Later:" line name the open accessibility debts as plans, never as done: HTML's focus fixup for a node made inert, a native checkbox's or radio's checked state in the tree's `toggled`, the missing click-reaction reading, and the `load-fetch` coverage gap.

## Standing conventions
- (none)

## Not needed
- The assertion harness contract (§3): the `accessibility_*` test files, `assert_role` and `unknown_tags`, and the state, label and description tables. The chunk adds and changes no test.
- The `a11y` CI leg's fourteen files and the `stand-repeat` membership (§9): no test file or CI job is added or renamed.
- The engine's focus order and focusability rules, and the stand's Tab-order tables (`tab_order`, `ENABLED_TAB_ORDER`): no engine or stand markup change.
- Focus restoration on click, the double-click window and the range input's pointer and key model: no engine change, and the README names no range behaviour.
- The accessibility tree lifecycle and the changed set's refresh (§2): no shell or engine change.
- Role derivation, the strong-native-semantics TODO and the landmark mapping (§2, §4): no markup or builder change.
- Hidden-content exclusion and `aria-hidden` handling (§4), and visibility and hit-testing (§6): unchanged.
- The Dioxus bridge's falsy boolean attribute removal (§5 Script and framework exposure): no bridge change.
- The CRUD list's keyboard shortcuts and the unread `aria-activedescendant` (§5): no stand change.
- Name computation (`aria-label`, `aria-labelledby`, `<label>` binding) and the stable `author_id` on tree nodes (§7): no builder or id change.
- The actionable-key rule and `unkeyed_actionable` (§7): no new actionable element.
- The platform adapter's creation-before-visible rule (§7): no shell code.
- Language and direction, text alternatives and IME (§7): no page or fixture change.
- The flight booker's invalid-date error recovery cue (§8): no stand change.
- Focus ring, motion, state colour and user-preference media (§6): no stand style change.
