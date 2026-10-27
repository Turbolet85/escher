# Task — a11y · 2026-10-26-codebase-health

## The chunk
A refactor with no behaviour change. It brings three escher-written functions from over cognitive complexity 15 to at or under it, using named helpers or table walks.
The three functions are the visual lint's per-node walk `lint.rs::findings` (16), the MCP listing test `the_listing_is_the_twelve_verbs_of_the_table` (18) and the requirement sweep `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` (18).
The lint's output must not change: the same findings in the same order, the six defects, the `fonts-unavailable` refusal first, and nothing changed on the held instance. Each test asserts the same rows. Test functions stay in the count, and the engine function `clear_interaction_state_for_removed_node` is not touched.
It re-runs `stand_lint` and the lint unit tests unchanged, the two touched tests and the `fast` and `doc` legs. It re-measures the audit's complexity reading (at most 85 over the ceiling, with no new entrant).

## Reached files
- packages/escher-driver/src/lint.rs · edits `findings` (its branch ladder moved into named helpers or a table walk)
- packages/escher-driver/src/mcp_tests.rs · edits `the_listing_is_the_twelve_verbs_of_the_table`
- tests/blitz-tests/tests/stand_requirement_sweep.rs · edits `every_0_2_0_requirement_names_its_witness_inside_stand_repeat`
- tests/blitz-tests/tests/stand_lint.rs · re-reads stand_lint
- .github/scripts/ci-leg.sh · re-reads the `fast` and `doc` legs
- .github/scripts/test_blitz_tests_targets.py · re-reads the pin holding `stand_requirement_sweep` as a blitz-tests target
- .andromeda/runs/2026-10-25T21-43-01-code-audit/over_ceiling.py · re-reads the audit's over-ceiling complexity reading

## Needs
- The visual lint's walk `lint.rs::findings` is factored into helpers or a table walk, and every new unit reads at or under 15. Its findings stay identical: document order, then `Defect::ALL` order, covering `overlap`, `clipped`, `truncated`, `off-screen`, `zero-size` and `contrast`.
- Indirect, through the contrast harness: the lint's `contrast` defect is still read through `Harness::text_contrast` alone. It keeps `stand_contrast`'s exempt rule (a node is exempt exactly where the snapshot's `enabled` reads `Some(false)`). No helper gains a second contrast formula, threshold or required ratio (3.0 for large text, else 4.5).
- Indirect, through the snapshot model: the lint takes each node's role, name, bounds and `enabled` from the `Snapshot`, which reads them from the accessibility tree. A helper reads none of them from the element and adds no second role or name mapping.
- Hidden and inert nodes (`display: none`, `visibility: hidden`, `inert`) are absent from the tree and the snapshot, so they stay outside the lint's walk. A select's options stay in, despite `display: none`, and read their select's client rect as bounds.
- Because of that bounds reading, the flight booker's one `overlap` at boot (`flight-type-one-way` × `flight-type-return`) still reads after the refactor. That residual is kept as it is, not fixed here.
- A finding holds stable ids, closed defect words and numbers only. No helper or table row carries page text, a colour, a control value or a masked value into a finding.
- `stand_lint` re-runs unchanged in both layout modes. The counter, the timer, CRUD and the load page read 0 findings at boot and the flight booker reads one `overlap`. After twelve Creates the last CRUD row reads `clipped` inside `crud-list` and also `truncated`.
- The MCP listing test still asserts all twelve tools in `VERBS` order, with `lint` after `screenshot` and `help` last. Each tool is still described by its help and its result fields, the `lint` tool's `findings` included.
- The requirement sweep reads the `stand-repeat` leg's list, which holds eight a11y files (the four `stand_*` files, `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread` and `select_interaction`). Its table walk asserts the same witness rows over that list, and the list is not edited.
- The `a11y` leg runs its fourteen files green and unchanged, and so does `stand-repeat`'s three-round run of its eight a11y files. The chunk adds, renames or edits none of them.
- Dropping a row from the table a refactored test reads (from `VERBS`, or from the requirement witness list) still turns that test red. The rows guarded before the refactor stay guarded.

## Standing conventions
- S1 · A check that asserts a stated table states it once and asserts its row count beside the row-by-row walk · body §3 A11y Assertion Harness Contract
- S2 · A check reads roles and names from the shared module's tables or the snapshot model, never from a second role or name mapping · .andromeda/a11y-plan-detail/screen-reader-support.md
- S3 · A check that boots a document runs in both layout modes (`for incremental in [false, true]`) · body §3 A11y Assertion Harness Contract

## Not needed
- The engine's role mapping and ARIA state reads in the accessibility builder (`aria-selected`, `aria-pressed`, `aria-checked`, `aria-invalid`, `aria-labelledby`, `aria-describedby`, `aria-expanded`, `aria-disabled`, `aria-level`). The builder is not touched.
- Focus order, focusability, the disabled-presence rule, and Enter and Space activation (§5). No engine or stand behaviour changes, and the Tab-order tables of `stand_keyboard` are not edited.
- The range input's and the single-choice select's key and pointer models, and select typeahead. The chunk changes no engine code.
- Focus restoration and interaction state on node removal. The engine function `clear_interaction_state_for_removed_node` is left untouched by the founder's word.
- The accessibility tree lifecycle, the changed set and its drain, and the shell's poll-driven refresh (§2). These are not reached.
- The AccessKit platform adapter, `accesskit_xplat`, `author_id` wiring and the windowed assistive-technology witness residual.
- The actionable-key rule and `DioxusDocument::unkeyed_actionable`. No element, key or stand markup changes.
- The password and file-input masking in snapshot and paint, the driver's `disabled` refusal on a `click` or `type`, and the stand's attribute-stated `aria-selected` and `aria-invalid` states. The chunk does not reach them.
- `stand_contrast`'s control rows, `TEXT_ROWS` and `STATE_ROWS`, and the stand accent colour. That check and the stand's colours are not edited.
- The flight booker's invalid-date cue (cognitive error recovery, §8), and the open gap on the served pages' keyboard and contrast coverage (`load-fetch`). Neither is in this chunk's scope.
- The focus ring, motion, colour scheme, zoom, language, text alternatives and IME sections. All are not yet measured or absent, and the chunk touches none.
- §10 SLO budgets, §11 anti-patterns and §12 decisions log. All three hold no recorded intent.
