# Records — 2026-10-27-readme-and-records (v030-16, by construction)

Written by /andromeda-implement (run `.andromeda/runs/2026-10-27T05-19-41-implement/`), read from the tree at this
chunk's base `316c6d25b59a` with the working tree's uncommitted chunk edits (none of them touches a test file or a
design-system file). The wrap writes (a) through `matrix.py refine` (F1) and moves (b) through design-system's door
(F2, the founder's delegate answer, PROVISIONAL, inputs#I3). Nothing here was written into a ledger or a master by
implement.

## (a) The two verified ledger refs

Every test name the two refs hold, looked up as `fn {name}(` in its file (a scratchpad lookup over the 13 names; the
research sweep counted 14 · 3 stale · 11 standing, this lookup reads 13 · 3 stale · 10 standing — the stale three are
the same):

| Cap | File | Name in the ref | Reads now | `fn` line |
|---|---|---|---|---|
| v010-17 | `tests/blitz-tests/tests/stand_snapshot_state.rs` | `exactly_the_active_flight_mode_reads_pressed` | renamed → `exactly_the_chosen_flight_type_reads_selected` | `stand_snapshot_state.rs:368` |
| v010-17 | `tests/blitz-tests/tests/stand_snapshot_state.rs` | `exactly_the_selected_row_reads_selected` | stands | `stand_snapshot_state.rs:410` |
| v010-17 | `tests/blitz-tests/tests/stand_snapshot_state.rs` | `a_date_that_is_no_date_reads_invalid` | stands | `stand_snapshot_state.rs:462` |
| v010-17 | `tests/blitz-tests/tests/stand_act_states.rs` | `a_flight_mode_click_returns_both_mode_buttons_with_their_pressed_reading` | renamed → `a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading` | `stand_act_states.rs:80` |
| v010-17 | `tests/blitz-tests/tests/stand_act_states.rs` | `a_row_click_returns_the_row_selected_and_the_row_it_leaves_not_selected` | stands | `stand_act_states.rs:118` |
| v010-17 | `tests/blitz-tests/tests/stand_act_states.rs` | `a_typed_start_date_returns_the_field_invalid_or_not` | stands | `stand_act_states.rs:149` |
| v010-17 | `tests/blitz-tests/tests/stand_act_diff.rs` | `a_choice_on_the_flight_booker_returns_the_control_it_enabled` | stands | `stand_act_diff.rs:124` |
| v010-17 | `tests/blitz-tests/tests/stand_act_diff.rs` | `create_returns_the_new_row_added_and_delete_returns_it_removed` | stands | `stand_act_diff.rs:197` |
| v020-05 | `tests/blitz-tests/tests/stand_lint.rs` | `the_planted_defects_are_named_each_by_its_id_and_reason` | stands | `stand_lint.rs:175` |
| v020-05 | `tests/blitz-tests/tests/stand_lint.rs` | `the_stand_at_boot_reads_no_finding` | renamed → `the_stand_at_boot_reads_its_measured_findings` | `stand_lint.rs:288` |
| v020-05 | `tests/blitz-tests/tests/stand_lint.rs` | `the_lint_verb_answers_what_the_reader_reads` | stands | `stand_lint.rs:416` |
| v020-05 | `examples/seven_guis/tests/cli_commands.rs` | `lint_answers_one_line_of_findings` | stands | `cli_commands.rs:420` |
| v020-05 | `tests/blitz-tests/tests/stand_mcp_tools.rs` | `a_lint_tool_call_answers_the_readers_findings_and_parents_its_command` | stands | `stand_mcp_tools.rs:707` |

The three old names read no `fn` line anywhere in their files.

### The re-pointed ref texts (today's text, the three names swapped, nothing else)

v010-17 (`escher-0.1.0/verification-matrix.json`, chunk `2026-10-10-snapshot-states`):

```text
tests/blitz-tests/tests/stand_snapshot_state.rs::exactly_the_chosen_flight_type_reads_selected · ::exactly_the_selected_row_reads_selected · ::a_date_that_is_no_date_reads_invalid · tests/blitz-tests/tests/stand_act_states.rs::a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading · ::a_row_click_returns_the_row_selected_and_the_row_it_leaves_not_selected · ::a_typed_start_date_returns_the_field_invalid_or_not · tests/blitz-tests/tests/stand_act_diff.rs::a_choice_on_the_flight_booker_returns_the_control_it_enabled · ::create_returns_the_new_row_added_and_delete_returns_it_removed
```

v020-05 (`escher-0.2.0/verification-matrix.json`, chunk `2026-10-19-visual-lint`):

```text
tests/blitz-tests/tests/stand_lint.rs::the_planted_defects_are_named_each_by_its_id_and_reason + ::the_stand_at_boot_reads_its_measured_findings (Session::lint, both layout modes) + ::the_lint_verb_answers_what_the_reader_reads (the `lint` verb through Session::run answers Session::lint's findings on the planted page and the four lean tasks, both modes) + examples/seven_guis/tests/cli_commands.rs::lint_answers_one_line_of_findings (the escher-session binary) + tests/blitz-tests/tests/stand_mcp_tools.rs::a_lint_tool_call_answers_the_readers_findings_and_parents_its_command (MCP) — the carriage the founder's own word on F1 set, option 1 (inputs#I7)
```

v010-17's `acceptance` still describes the retired pressed pair (`flight-one-way` · `flight-return`); F1 («This wrap,
refs only (Recommended)») leaves it as worded. v020-05's `acceptance` still reads «on each of the four lean stand tasks
at boot it names 0 findings», while the renamed test asserts the flight booker's one `overlap`; F1 leaves that as
worded too.

## (b) The design-system stand rows (F2, «Every stand row (Recommended)», PROVISIONAL)

The move set, each with its values as they stand and where it lands. Landing: colour, shadow, spacing and radius go to
layout-templates §Surface: desktop-native → Primary screens' `values · desktop-native-screens` block
(`.andromeda/layout-templates.md:12`); the font registration goes to architecture §Standard Contracts → Headless stand
(`.andromeda/architecture-detail/standard-contracts.md:120`), beside `font_ctx()`. A fact the landing already states is
folded, not repeated.

| # | Row | `file:line` | Values | Lands in | Already stated there (fold) |
|---|---|---|---|---|---|
| 1 | seven_guis accent (body sentence) | `.andromeda/design-system.md:16` | `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`); white button text passes SC 1.4.3's 4.5 at rest, hover, active | layout-templates `values · desktop-native-screens` | the three shades, on the load page's `load-fetch` (`layout-templates.md`, the "served screens" bullet) |
| 2 | seven_guis accent (row) | `.andromeda/design-system-detail/color-palette.md:27` | `#3a5ce5` / `#2a4cd3` / `#1f3fc1`; page `#f5f5f5`, text `#1a1a1a`; white on accent 5.47, hover 6.83, active 8.27; the old `#4a6cf7` 4.39; three `rgba(58, 92, 229, …)` uses | layout-templates `values · desktop-native-screens` | as row 1 |
| 3 | seven_guis load page (stand) | `.andromeda/design-system-detail/color-palette.md:28` | `load-fetch` white on `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`); `load-done` `#1a1a1a` on `#f5f5f5`; no contrast row | layout-templates `values · desktop-native-screens` | `load-fetch` white on the three accent shades (the "served screens" bullet); `load-done`'s colours and the no-contrast-row fact are new there |
| 4 | seven_guis success | `.andromeda/design-system-detail/color-palette.md:73` | `#ebf8ee` / `#68d391` / `#276749`, the success message | layout-templates `values · desktop-native-screens` | none |
| 5 | Field — seven_guis flight-type select (Border Progression) | `.andromeda/design-system-detail/color-palette.md:101` | `1px solid #c0c0c0`; focused `#3a5ce5`; `#1a1a1a` on `#ffffff` (17.40, 15px 400) | layout-templates `values · desktop-native-screens` | the flight booker's mode-region bullet names the select and its options, not its border |
| 6 | DejaVu Sans registration | `.andromeda/design-system-detail/typography.md:31` | `build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx` for every generic, system fonts off, native; every boot, `boot_cells` and `boot_load` included; seven_guis' native `woff` feature | architecture §Standard Contracts → Headless stand | `font_ctx()` = `build_single_font_ctx(DEJAVU_SANS)`, system fonts off; "for every generic", the Cells and load-page boots and the `woff` feature are new there |
| 7 | seven_guis cards' shadow | `.andromeda/design-system-detail/depth-strategy.md:10` | cards `0 2px 8px rgba(0, 0, 0, 0.08)`; circle dialog 0.10 | layout-templates `values · desktop-native-screens` | none |
| 8 | seven_guis cards' padding and gap | `.andromeda/design-system-detail/spacing.md:19` | cards pad 24px 32px, gap 16px; home pads 48px 32px 64px | layout-templates `values · desktop-native-screens` | none |
| 9 | seven_guis radius (untokenized) | `.andromeda/design-system-detail/border-radius.md:12` | cards 8px, buttons 6px, inputs 4px, task cards 6px, tags 3px, the `flight-type` select 6px | layout-templates `values · desktop-native-screens` | none |

### Further seven_guis stand lines in design-system, outside the plan's list

A fixed-string search for `seven_guis` and `stand` over `.andromeda/design-system.md` and its detail files reads these
stand lines beyond the nine above. The plan's step 6(b) does not name them; whether «Every stand row» (F2) moves them
too is the wrap's to read with the operator at its door. Listed, not decided:

- `.andromeda/design-system-detail/color-palette.md:45` — `seven_guis / todomvc page` `#f5f5f5` (shared with todomvc)
- `.andromeda/design-system-detail/color-palette.md:60` — `seven_guis text` `#1a1a1a`
- `.andromeda/design-system-detail/color-palette.md:72` — `seven_guis invalid` `#e53e3e` / `#fff5f5` / `#c53030`
- `.andromeda/design-system-detail/typography.md:18` — `seven_guis` sans-serif, home title 700, base 14px, title 36px
- `.andromeda/design-system-detail/typography.md:19` — the `flight-type` select, 400, 15px
- `.andromeda/design-system-detail/spacing.md:20` — the `flight-type` select pads 8px vertical, 10px horizontal
- `.andromeda/design-system-detail/surface.md:10` — the stand's `select#flight-type` overrides (15px, colours, border,
  radius 6, padding 8/10, focus border)
- `.andromeda/design-system-detail/surface.md:38-40` — the `values · desktop-native-stand-load-page` block (the load
  page's two keyed elements)
- `.andromeda/design-system-detail/motion.md:34` — the served load page's `LoadNet` answer after 200 ms
- `.andromeda/design-system.md:220` — seven_guis and todomvc wasm pages on `#f5f5f5` (shared with todomvc)

## (c) The incubator's ignore (step 5)

Before (with `.git/info/exclude`'s last two lines present), `git check-ignore -v escher-0.3.0-incubator/intent.md`,
exit 0:

```text
.gitignore:39:escher-0.3.0-incubator/	escher-0.3.0-incubator/intent.md
```

The two lines removed from `.git/info/exclude` (its exact tail, nothing else of the file; 659 → 541 bytes):

```text
# escher 0.3.0 incubator (the founder, 2026-10-16), local until a .gitignore line replaces it
escher-0.3.0-incubator/
```

After, the same call, exit 0:

```text
.gitignore:39:escher-0.3.0-incubator/	escher-0.3.0-incubator/intent.md
```

`.git/info/exclude` is local and untracked; no commit carries the removal.
