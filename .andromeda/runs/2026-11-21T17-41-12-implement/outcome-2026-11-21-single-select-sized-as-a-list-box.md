# Outcome — 2026-11-21-single-select-sized-as-a-list-box

**Run:** 2026-11-21T17-41-12-implement
**Outcome:** green. Smoke: skipped — no boot-path change; the stand's headless checks ran three rounds green as P2 gate 8.

## Files changed (16 · 0 new; `gate.py scope`: clean — changed 16 · listed 16 · recorded 0)
- packages/blitz-dom/src/node/select.rs · node/select_list.rs · stylo/presentational_hints.rs · mutator.rs · document.rs ·
  events/keyboard.rs · events/pointer.rs · accessibility.rs · layout_reason.rs
- packages/dioxus-native-dom/src/snapshot.rs
- packages/escher-driver/src/schema.rs · schema_tests.rs
- docs/capabilities.md
- tests/blitz-tests/tests/select_interaction.rs · select_popup.rs · stand_act_select.rs

## Gates (final firing, `.andromeda/runs/2026-11-21T17-41-12-implement/gate-2026-11-21-single-select-sized-as-a-list-box.json`)
- `cargo test -p blitz-dom -p dioxus-native-dom -p escher-driver --locked --lib` — green
- `cargo test -p blitz-tests --locked --no-fail-fast --test select_interaction --test select_popup --test stand_act_select` — green
- `cargo test -p blitz-tests --locked --no-fail-fast --test select_popup_painted … --test incremental_oracle` (13 targets) — green
- `python3 .github/scripts/test_capability_statement.py` — green
- `bash .github/scripts/ci-leg.sh a11y` — green
- `bash .github/scripts/ci-leg.sh fast` — green (red twice earlier on clippy: `some_filter` in presentational_hints.rs, then `type_complexity` in select.rs's unit test; each fixed)
- `bash .github/scripts/ci-leg.sh doc` — green
- `bash .github/scripts/ci-leg.sh stand-repeat` — green
- `gate.py hygiene`, the push, `ci.py conclusion` — `not run — leg operator`: the operator pass's entries 9, 10, 11

Summary line: `entries 11 · green 8 · red 0 · recorded 0 · timeout 0 · not-run 3`

## Controls (`{work_dir}/evidence/control-1.txt`, `control-2.txt`)
- selectedness · row-hints · bridge-closed: holds (control-1)
- popup-gate: DOES NOT HOLD on case `a_size_write_turns_a_drop_down_into_a_list_box_and_back` (control-1), because a size write moves the select's box and `live_select_list` drops the list without reading the gate; held (control-2) on the plan's named case `a_sized_single_select_opens_no_list`, once that case gained a fixed-size drop-down whose box a `size` write does not move.

## Deviations (with justification)
1. Step 11: the `press` help reads «the arrows, `home` and `end` choose a row and nothing opens». The plan's words add «and typed text», but `press` carries only the twelve `KEY_NAMES` and no character, so it cannot type text. `type` covers typed text, and its help is unchanged and still true.
2. Step 15: the boot lint of the list-box page reads 4 findings, not the plan's 0: `fx-s-four` clipped and truncated, `fx-s-five` truncated and off-screen, as measured. The page's own check (c) needs two rows scrolled out, and the lint reads those as it reads any scrolling box's hidden rows (cf. CRUD's `clipped` row). The check pins those four and no `overlap`, and is named `a_sized_single_selects_page_lints_only_its_rows_scrolled_out_at_boot`.
3. Step 15 (b): the click targets the row off the select's centre (`fx-s-three`), not the middle row. With the bridge control's edit, the centre row's hit lands on itself and is accepted, so its control could not read red. Off the centre, the click is refused `covered` under the control, as the plan's prose predicts.
4. Step 15 (e): «no stray click» is asserted as no `selected` change on the centre row. The type scrolls the matched row into view, so every row's `bounds` moves in the diff.
5. Step 14: `a_sized_single_select_opens_no_list` also holds a drop-down fixed by the page at `width: 80px; height: 60px; vertical-align: top`, written `size="3"` with its list open. Its box does not move, and the list closes. That is the one reading of the popup gate's list-box term (controls above).
6. Step 13: «lays out nothing» reads the computed `display` beside the client rect. Measured: a node turned `display: none` by a restyle keeps the client rect it last laid out at. This is existing engine behaviour, not changed here.
7. Step 13 (d): the `scroll_top` pins are the measured 17 · 36 · 94 · 2 · 74. A row brought in from below sits on the box's bottom edge, and Home's first row sits within the 2px padding.
8. Step 13: `fx-sized` is a controlled select (the app writes `selected` from its `input` value). A `selected: true` literal is re-written on each re-render and re-seeded the choice.
9. Step 3: a `multiple` write on a select also restyles its descendants, as a `size` write does. Both decide whether the rows take the list box's hints.
10. Step 1: besides `is_list_box`, two crate-private readings, `is_single_list_box` and `is_drop_down`. The popup gate, `click_select`, the key dispatch and the tree's `expanded` arm read `is_drop_down`.
11. Step 8: a single list box, like a drop-down, reads no author `aria-expanded`.
12. Step 7: a Space with typed text standing joins the typeahead, as on a drop-down and a `multiple` select; with none standing it does nothing.

scope record: none — `gate.py scope` clean, 0 recorded.

## Surfaced
none (the deviations above are measurements the wrap dispositions; no spec↔reality gap was left unfixed in the impl).

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo · rustc · test binaries (`select_interaction`, `select_popup`, `stand_act_select`, blitz-tests targets) · `escher-session` | this run's gates and controls | terminated — `gate.py census --comm cargo --comm rustc --comm escher-session --comm select_interaction --comm select_popup --comm stand_act_select --args blitz-tests`: `census: none running` |
| control clones `~/.cache/andromeda-control/33ae8b60`, `a79f50fc` | this run's controls | removed (`clone removed` on each summary) |

## Decisions & corrections
- Bash guard refusal: a `cat` heredoc append to a test file was refused («a cat/tee heredoc with a file target»), re-issued through the Edit tool.
- Bash guard refusal: a `cd` into the skill's references dir was refused (it moves the session cwd); re-issued with absolute paths.
- Recurrence of the 2026-11-19 learning «a check written over a path just measured pins the measured value»: the scroll offsets and the lint findings were measured, then pinned as literals.
- Measured: `fast` stops at its first clippy error, so two lints in new code took two firings; `ci-leg.sh clippy` alone before the third firing read clean.
- Measured: a node turned `display: none` keeps its last client rect (`get_client_bounding_rect` reads stale layout).
- Measured: a Dioxus `selected: true` literal is re-written on a re-render triggered elsewhere in the component, re-seeding a single select's choice.

## Next
The operator pass: entries 9 (`gate.py hygiene`), 10 (the push) and 11 (`ci.py conclusion`), then `/andromeda-wrap-session`.
