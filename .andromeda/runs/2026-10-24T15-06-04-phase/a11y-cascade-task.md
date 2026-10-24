# Task — a11y · 2026-10-24-stand-requirement-sweep

## The chunk
A sweep that re-proves, headless on the 7GUIs stand or a minimal fixture beside it, every 0.2.0 requirement v020-01 to v020-09, and records per requirement the task or fixture holding its case and the check that proves it; a requirement with no such check gets one.
The record extends or sits beside 0.1.0's stated witness table in the stand requirement sweep check; every named witness runs inside the stand-repeat leg at the chunk's end, by widening that leg's selection or by carrying the proof in a stand check.
v020-03's witnesses are the engine debts: inert, aria-checked toggles, the unread aria-activedescendant, disabled focusability, the password caret click, contain: paint, the transformed client rect and the absolute position reader.
It holds the coverage floors, measures the lint verb, the snapshot part argument and the record option for id-free, name-free and typed-text-free stderr on the escher-session host and its mcp role, and adds fixture checks, in both layout modes, for the visual lint's untested readings (a clipped or off-screen text node also reading truncated; text_extent's RTL, vertical and transformed-box limits).
It changes no engine or lint behaviour unless a founder's fork answers change; the stand's Tab order, Enter and Space activation and contrast re-assertion belong to the later A11y re-read entry.

## Reached files
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- tests/blitz-tests/tests/common/mod.rs
- tests/blitz-tests/tests/session_common/mod.rs
- tests/blitz-tests/tests/fixtures/mod.rs
- examples/seven_guis/tests/host_log.rs
- examples/seven_guis/tests/mcp_log.rs
- examples/seven_guis/tests/common/mod.rs
- .github/scripts/ci-leg.sh · merge surface
- .github/scripts/test_ci_workflows.py
- .github/scripts/test_blitz_tests_targets.py
- .github/scripts/coverage_floor.py
- packages/blitz-dom/src/text_extent.rs · conflict surface
- packages/escher-driver/src/lint.rs · conflict surface

## Needs
- The CI integration of the a11y checks: the a11y leg's six files live in ci-leg.sh beside the stand-repeat leg this chunk may widen, and the three stand files that run in both legs; any change to ci-leg.sh's selection must keep the a11y leg's set intact.
- The assertion harness contract: accessibility checks read the built tree in both layout modes with stated tables and asserted row counts; a new fixture check or a check moved into stand-repeat follows that shape.
- The inert rule as a witness of v020-03: an inert subtree skipped by Tab and Shift+Tab, passed through by a hit, out of the tree and the snapshot, restored on removal; the focus fixup not built, owned by a residual.
- The builder's state reads as v020-03 witnesses: aria-checked writing toggled on the five checkable roles alone, aria-pressed elsewhere, a native checkbox or radio reading neither, role switch mapping to Switch; aria-selected and aria-invalid.
- The unread aria-activedescendant on the CRUD list: focus on the list, never the option, pinned by its own check, which the sweep names as a witness.
- Focusability by disabled presence on a form control alone, and no inert node focusable, as v020-03 witnesses.
- Focus on click and the double click read on the document's virtual clock (500 ms, 2 px), reached through v020-06's one virtual clock and the password caret click witness.
- The password mask: a password input's value read as the fixed mask in the snapshot and painted as bullets, the caret placed by the bullets on a click; the password caret check is a witness the sweep names.
- The select's keyboard model (arrows, Home, End, typeahead on the document's clock, one input then one change) and its options in the tree carrying their chosen state, as v020-08's witnesses.
- The range input's keyboard model only as far as an existing witness the sweep names rests on it.
- Visibility and hit-testing: display none, visibility hidden and inert excluding nodes from the tree and the snapshot; a select's option kept under display none; a hit stopping at a clipping box including contain paint.
- The snapshot model consuming the tree (role, name, focus, selected, pressed, invalid from the tree; enabled and value from the element), reached through the lint's findings, the select's snapshot value and the part-reading witnesses.
- The visual lint's contrast reading through the harness's text contrast reader with stand_contrast's exempt rule and no second formula, reached through v020-05's lint witnesses and any lint fixture this chunk adds.
- The contrast harness itself (the text contrast reader, the WCAG ratio, 3.0 or 4.5 by text size) as the lint's contrast witness, read not re-asserted.
- The stable author id on every tree node and the actionable-key rule, reached through the lint findings' ids and the stderr measurement that must carry no id or accessible name.
- The obs log format for escher's own sink (the scrubbed target allowlist, the two record classes), reached through the stderr id-free measurement on host_log and mcp_log.
- The flight booker's aria-invalid date inputs and aria-pressed mode buttons, and the CRUD list's aria-selected rows, as stand states a witness may read.

## Not needed
- The platform accessibility adapter and its windowed assistive-technology witness: owned by a residual, no stand binary builds an adapter, and the chunk works headless only.
- The adapter's creation-before-window-shown rule: no windowed run is in this chunk.
- Language and direction, text alternatives and IME: no requirement in reach and no check this chunk adds reads them.
- Motion, focus ring, colour scheme, zoom and user-preference media: unmeasured areas the sweep's requirements do not reach.
- Cognitive orientation and status cues, and the flight booker's error recovery as a colour cue: no 0.2.0 requirement in reach rests on them.
- The landmark and HTML-to-AccessKit role mapping: the chunk adds no role and changes no mapping.
- The upstream name and description reads (aria-labelledby, aria-describedby, aria-description, aria-expanded, aria-disabled, aria-level): they arrived by upstream sync and are no v020 requirement's witness.
- The Dioxus bridge's falsy boolean attribute rule: no stand or fixture markup this chunk adds turns on it.
- The stand's Tab-order tables, Enter and Space on each stand button and the stand's contrast tables re-asserted after 0.2.0's engine changes: owned by the later A11y re-read entry.
- The CRUD list's keyboard region (third Tab stop, arrow keys moving the selection): a 0.1.0 capability, re-asserted by the A11y re-read entry, not by this sweep.
- The tree's lifecycle and refresh on a changed set at the shell: a 0.1.0 capability with its own check, not a 0.2.0 requirement.
- SLO budgets, anti-patterns and the decisions log: no recorded intent.
