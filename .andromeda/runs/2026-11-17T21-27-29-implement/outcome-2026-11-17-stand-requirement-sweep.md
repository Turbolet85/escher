# Outcome — 2026-11-17-stand-requirement-sweep (implement run 2026-11-17T21-27-29-implement)

**Outcome: green** — every gate of the chunk's own work green; one entry (4, the guard) red by the plan's own wording,
listed below. Smoke: skipped — no boot-path or UI-surface change. Next → the OPERATOR PASS: entries 12 (`gate.py
hygiene`), 13 (the push) and 14 (`ci.py conclusion`), then `/andromeda-wrap-session`.

## Files changed
- `tests/blitz-tests/tests/stand_requirement_sweep.rs` — modified (1,109 → 1,708 lines, `wc -l`): `WITNESSES_030`,
  `NO_SCREEN_WITNESS`, `STAND_REPEAT_NAMED` 10 → 17, the arm reader `stand_repeat_arm` with `runs_test`,
  `seven_guis_line`, `assert_the_arm_runs_every_named_target`, `assert_0_3_0_row`, the test
  `every_0_3_0_requirement_names_its_witness_inside_stand_repeat`, `inside_stand_repeat` reading the arm, 19
  `source()` entries.
- `tests/blitz-tests/tests/stand_masks_on_new_answers.rs` — new (269 lines), four tests, both layout modes.
- `apps/browser/src/document_loader.rs` — modified: the test module's `Probe` widened, `probe_config_with`, five field
  tests.

## Gates (trail `.andromeda/runs/2026-11-17T21-27-29-implement/gate-2026-11-17-stand-requirement-sweep.json`)
- 1 `cargo test -p browser --locked --bin blitz document_loader::tests` — green (8 passed)
- 2 `cargo test -p blitz-tests --locked --test stand_requirement_sweep` — green (4 passed)
- 3 `cargo test -p blitz-tests --locked --test stand_masks_on_new_answers` — green (4 passed)
- 4 `gate.py guard --chunk … --base 66fbe33be…` — **red · last line 'guard: clean' ✗ (last: 'guard: outside 3 files')**
  — red by the plan's own wording: the three files outside are the plan's own `### New files` / `### Files to
  modify`; the entry carries no `--allow`, and the tool excuses only scope-record companions. The same guard fired
  by hand with `--allow` over the three touchpoints reads:
  `guard: allowed 3 — apps/browser/src/document_loader.rs, tests/blitz-tests/tests/stand_masks_on_new_answers.rs, tests/blitz-tests/tests/stand_requirement_sweep.rs · companions excluded 0 · changed 183 · excluded 180` /
  `guard: clean` (`{work_dir}/evidence/guard.md`).
- 5 `bash .github/scripts/ci-leg.sh fast` — green (395 s)
- 6 `bash .github/scripts/ci-leg.sh stand-repeat` — green, `stand-repeat: 3 of 3 rounds green` (1086 s)
- 7 `bash .github/scripts/ci-leg.sh coverage` — green, `coverage floor: 3 of 3 crates at or above their floors`
  (escher-driver 97.73 % · dioxus-native-dom 90.53 % · blitz-test-harness 95.99 %)
- 8 `bash scripts/agent-run.sh boot` — green (ready)
- 9 `run stand` — green, run.end 332 · 0 · 10
- 10 `run session` — green, outcome passed (2 · 0 · 0)
- 11 `run all` — green, run.end 879 · 0 · 15
- 12–14 — not run, leg operator.

Controls (`gate.py control`, `{work_dir}/evidence/control-1.txt`): `control: 9 holds · 0 timeout · 0 does not hold ·
0 unreproduced · clone removed` — leg-arm, named-17, row-030, mask, shell-provider, html-parser, font-ctx,
abort-signal, style-threading.

## Deviations, with justification
- **Three witness names differ from the ledger refs the plan says to copy:** the verified refs of v030-01
  (`the_listing_is_the_twelve_verbs_of_the_table`), v030-09
  (`a_panicking_paths_kept_check_holds_the_steps_before_it_and_replays_green`) and v030-10
  (`a_kept_check_replays_its_caused_and_a_flipped_count_stops_naming_caused`) name tests since renamed. The table
  names the current functions (`the_listing_is_the_fifteen_verbs_of_the_table`,
  `a_panicking_paths_kept_check_holds_the_panicked_step_and_replays_green`,
  `a_kept_check_replays_its_caused_and_a_changed_message_stops_naming_caused`) — the table's own `fn {test}(`
  assert read each stale name red. The ledger refs stay as written (implement writes no ref to a cap it does not
  own); surfaced for the wrap.
- **Witness tests chosen where a ref names a file alone** (v030-04's `visible_region_transformed.rs`,
  `select_interaction.rs`, `stand_act_select.rs`, `form.rs::tests`, `checkable_change_event.rs`,
  `text_range_no_change_event.rs`, `stand_keyboard.rs`, `stand_contrast.rs`, `stand_load_page.rs`,
  `stand_cells_served.rs`; v030-14's `stand_cross_os_readings.rs`; v030-17's `select_popup.rs`,
  `select_popup_painted.rs`): one or more tests per file named by what the debt or acceptance states.
- **Subjects beyond `AllFour`, `Unit`, `Served`:** step 3 lists three; the table also uses `Fixture` and `Task(…)`,
  as the 0.2.0 table does, where a witness runs on a fixture page or one lean task.
- **`inside_stand_repeat` reads the arm itself** (step 4): a named target counts only when the arm runs it, a
  `stand_` file only when the arm holds the glob, a seven_guis target only on the arm's seven_guis line. The arm's
  words are read whole (`runs_test`: `--test` then the name, `(` and `)` breaking words), so `select_popup` is not
  read inside `select_popup_painted`.
- **The mask check's explored path** boots each fresh session with the sentinel already typed (`hold_typed`), so
  every path's screens hold the typed password; the plan states the explore over "the fixture".
- scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 3 · listed 3 · recorded 0`).

## Surfaced
- Entry 4's form (plan defect, red by its own wording) — for the wrap's light gate.
- Three verified 0.3.0 ledger refs name renamed tests (v030-01, v030-09, v030-10) — premise disproof of the refs'
  test names, the caps' acceptance unmoved.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / test binaries / `escher-session` children of entries 1–11 | a gate entry (this run) | terminated — `gate.py census --comm escher-session --comm blitz --comm cargo --comm seven_guis_native --args escher-session --args stand_masks_on_new_answers`: `census: none running` |
| the control clone's firings | `gate.py control` (this run) | terminated, `clone removed` |

## Decisions & corrections
- Guard refusal: two Bash calls with a bare `cd` into a directory below the root were refused by bash-guard's cwd
  arm; each re-issued once as an absolute path or a subshell.
- Sweep hazard: `git grep -l` counts tracked files alone — `^mod session_common;` read 42 tracked, 43 with
  `--untracked` (this chunk's new target); a count taken before the pre-CI commit names which.
- Sweep hazard: a verified ledger ref's test name is no proof the function still bears it — three of the 0.3.0 refs
  name renamed tests; a table pinned by `fn {test}(` finds them.
