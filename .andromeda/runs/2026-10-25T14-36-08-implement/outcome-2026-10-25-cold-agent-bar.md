# /implement outcome — 2026-10-25-cold-agent-bar

**Outcome: green + smoke.** Every plan entry this skill fires is green, the founder's one live run included
(`passed`). v020-11 is `implemented` in `escher-0.2.0/verification-matrix.json`. Nothing is committed.
Next → the OPERATOR PASS: entries 25 (`gate.py hygiene`, before the pre-CI commit), 26 (fast, clean-tree guard, push,
after the pre-CI commit `chore(2026-10-25-cold-agent-bar): operator pre-CI commit, for the run this chunk's verdict
reads`) and 27 (`ci.py conclusion`), then `/andromeda-wrap-session`.

## The live run (step 12 — entry 23, fired once on «Запускать сейчас (Рекомендую)», inputs#I6)
- `passed`, reasons none: isolated (the twelve `mcp__escher__*` tools, one server `escher`), `counts_agree`.
- **Wrong calls 0 of 66 tool calls** (refused 0 · errors 0 · foreign 0 · denials 0) against the founder's 5 % bar.
- **Goals met: all six** — counter · flight-booker · timer · crud · cells · load.
- Capabilities: lint 1 · screenshot 1 · advance 1 · part reads 2; recorded sessions 6, `recording_kept` true.
- The agent's check: 59 steps, 16 expects, `check_passed` and `check_can_fail`.
- **Cost $0.8244** of the $3 cap; 67 turns; 102,704 ms; Claude Code 2.1.308, `claude-opus-5-5`.
- Calls by verb: advance 1 · click 14 · lint 1 · press 13 · screenshot 1 · scroll 1 · snapshot 13 · start 6 · stop 6 ·
  type 10. 0.1.0's baseline was 4 tasks, 43 calls, 0 wrong, $0.32.
- Evidence: `escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/` — `live-verdict.json`, `live-events.jsonl`,
  `live-check.json`, `live-transcript.jsonl` (session dir masked as `$TMPDIR/session` ×2, home as `~` ×5; 0 host
  spellings left), `live-run.md` (the entry lines verbatim and the counts). The six kept checks stay under
  `target/cold-agent/record/` (`0700`/`0600`), none committed or printed.

## Gates (run dir `.andromeda/runs/2026-10-25T14-36-08-implement/`, trail `gate-2026-10-25-cold-agent-bar.json`)
Second block firing: `entries 27 · green 21 · red 1 (24) · recorded 0 · timeout 0 · not-run 5`. Entry 24 read
`absent input` then and green after step 12 (`24.3.log`). Entries 22 and 23 green through `--operator`.
- 1 stand unit: 5 passed (2 → 5: `the_stand_options_stay_offline`, `cells_boots_in_the_shell`,
  `the_load_page_click_leaves_a_load_in_flight`).
- 2 the new checks: `stand_cells_served` 4, `stand_load_page` 2 (red-before-green: no target at the base).
- 3 the re-pinned stand checks: 20 targets green. 4 seven_guis process checks: `cli_commands` 14, `host_log` 4 (3 → 4).
- 5 workspace `host_log` + `mcp_log` green. 6 windows-gnu `cargo check --tests` green.
- 7 contract tests: Ran 37, OK (33 → 37); against HEAD's pipe 34 of the 37 fail (red-before-green).
- 8 `bash -n` · 9 pipe census 0 · 10 silence census 0 · 11 `git diff` probe 0 · 12 no platform adapter — green.
- 13 guard, its pass line whole:
  `guard: allowed 24 — .github/scripts/test_cold_agent.py, examples/seven_guis/src/session_host.rs,
  examples/seven_guis/src/stand.rs, examples/seven_guis/src/tasks/flight_booker.rs,
  examples/seven_guis/tests/cli_commands.rs, examples/seven_guis/tests/common/mod.rs,
  examples/seven_guis/tests/host_log.rs, scripts/cold-agent.sh, tests/blitz-tests/tests/common/mod.rs,
  tests/blitz-tests/tests/session_common/mod.rs, tests/blitz-tests/tests/stand_act_diff.rs,
  tests/blitz-tests/tests/stand_act_disabled.rs, tests/blitz-tests/tests/stand_act_ids.rs,
  tests/blitz-tests/tests/stand_act_states.rs, tests/blitz-tests/tests/stand_cells_served.rs,
  tests/blitz-tests/tests/stand_contrast.rs, tests/blitz-tests/tests/stand_element_ids.rs,
  tests/blitz-tests/tests/stand_id_persistence.rs, tests/blitz-tests/tests/stand_keyboard.rs,
  tests/blitz-tests/tests/stand_load_page.rs, tests/blitz-tests/tests/stand_requirement_sweep.rs,
  tests/blitz-tests/tests/stand_session_recording.rs, tests/blitz-tests/tests/stand_snapshot.rs,
  tests/blitz-tests/tests/stand_snapshot_state.rs · companions excluded 2 —
  tests/blitz-tests/tests/stand_accessibility_ids.rs (serves step 5), tests/blitz-tests/tests/stand_lint.rs (serves
  step 5) · changed 26 · excluded 0` — then `guard: clean`. The two excluded companions are test files that pinned the
  flight booker's old data (its 15 controls; 0 lint findings), each re-pinned to the measured reading.
- 14–15 agent-run: boot ready; `run stand` 50 files, passed 205, failed 0, ignored 7 (48 → 50 files).
- 16 a11y: 14 files, 85 passed, 0 failed (84 → 85) — accessibility_active_descendant_unread 1 · description 3 ·
  hidden 6 · label 13 · merged_states 3 · roles 6 · states 5 · focusability_updates 4 · inert_subtree 4 ·
  select_interaction 7 · stand_accessibility_ids 8 · stand_act_select 5 · stand_contrast 5 · stand_keyboard 15.
- 17 stand-repeat: `stand-repeat: 3 of 3 rounds green`.
- 18 coverage: escher-driver 4918/5019 = 97.99 %, dioxus-native-dom 2741/3202 = 85.60 %, blitz-test-harness
  952/1008 = 94.44 %, `coverage floor: 3 of 3 crates at or above their floors`.
- 19 fast green; 20 its tally `lines 208 passed 1117 failed 0 ignored 12` (206 → 208 lines); ci-scripts Ran 108
  (104 → 108). 21 doc green.
- No deferral: every language changed and every gate ran.

## Files changed (26)
New: `tests/blitz-tests/tests/stand_cells_served.rs`, `tests/blitz-tests/tests/stand_load_page.rs`, and
`evidence/` (live-*). Modified: `examples/seven_guis/src/{tasks/flight_booker,stand,session_host}.rs`,
`examples/seven_guis/tests/{cli_commands,host_log,common/mod}.rs`, `scripts/cold-agent.sh`,
`.github/scripts/test_cold_agent.py`, `tests/blitz-tests/tests/{common/mod,session_common/mod}.rs` and the stand checks
`stand_{keyboard,contrast,act_states,snapshot_state,act_disabled,act_ids,act_diff,element_ids,id_persistence,snapshot,
session_recording,requirement_sweep}.rs`, plus the two companions. `stand_diff.rs` needed no change (Book still moves
eight nodes, seven by bounds alone). The ledger: `escher-0.2.0/verification-matrix.json` (v020-11).

## Deviations from the plan, with justifications
- **Scope record** (`chunks/2026-10-25-cold-agent-bar/scope-record.md`):
  - `tests/blitz-tests/tests/stand_lint.rs` · companion · serves step 5 · self
  - `tests/blitz-tests/tests/stand_accessibility_ids.rs` · companion · serves step 5 · self
- **The lean tasks' lint no longer reads 0** (plan, carried sampler spacing#2.4). The driver's lint reads the flight
  booker's two type options as overlapping, since the snapshot gives each option its select's client rect. Surfaced
  to the founder: `stand_lint` re-pinned as measured — the flight booker reads exactly one `overlap`
  (`flight-type-one-way` × `flight-type-return`), the other three 0 — its test renamed
  `the_stand_at_boot_reads_its_measured_findings` (inputs#I5, «Перезакрепить как измерено (Рекомендую)»). A driver
  false positive for the wrap to own: any select with two options reads one `overlap`.
- **Step 4's tables hold the select alone, not its options.** Options take no focus, and the a11y acceptance needs
  every `controls(task)` id in `tab_order` ∪ `ENABLED_TAB_ORDER`. So `controls`, `rendered`, `tab_order` and
  `ENABLED_TAB_ORDER` hold `flight-type`, and `stand_snapshot` adds `the_flight_type_select_holds_its_two_options` (the
  ComboBox "Flight type", value `one-way`, two named ListBoxOptions, the first selected, each at the select's rect). The
  stand's controls are 14 (from 15), its contrast control rows 17 (from 18, tally 13 · 0 · 3 · 1), its state rows 18
  (from 19), the `stand_act_ids` census 77 nodes, 68 accepted and 9 refused (the two options `covered`), and the
  activation table 16 rows (from 20).
- **`stand_keyboard`** gains `the_flight_type_select_moves_its_choice_by_its_arrows` (ArrowDown, ArrowDown, Enter,
  Space, ArrowUp, ArrowUp: the value, the return date's enabled reading and the options' selected readings, focus
  staying on the select).
- **`stand_id_persistence`** chooses and books in one re-render: a choice on the select moves no class or text the DOM
  string shows.
- **`stand_cells_served`**: a click at Z25 right after `scroll` lands on the root's shown overlay scrollbar (a thumb
  drag) and opens nothing, and a pointer left on the thumb keeps it shown, so the walk takes `advance 5000` before the
  click, and `a_click_right_after_the_scroll_lands_on_the_shown_scrollbar` pins the early click on its own session.
  The prompt carries no hint (inputs#I5, «Без подсказки (Рекомендую)»); the live agent met Cells anyway.
- **`host_log`'s Cells read commits a sentinel first.** Cells' names in part 2 at boot are empty cells and row numbers,
  no needle a hit could be attributed by. The test commits the typed sentinel into the middle column-A cell part 2 holds
  (`common::cells_needles`) and reads part 2: 0 id needles and 0 name needles under both builds (entries 4 and 5).
- **`stand_requirement_sweep`**: a `Served` subject (Cells or the load page) beside `AllFour` · `Task` · `Fixture` ·
  `Unit`; the two new files carried in `source`; the v010-17 witness moved to the select test.
- **The pipe** also decodes a paged answer in the replay's expect match (`screen_text`), the same reader as
  `tasks_met`: a paged JSON line escapes its quotes, so an agent's expect written as the screen reads would never
  match. Its reasons, in order: … `task-unmet`, `capability-unused`, `recording-not-kept`, `wrong-calls-over-bar`,
  then the check's. The contract test `test_wrong_calls_are_counted_from_the_span_lines_and_agree_with_the_transcript`
  now fails `wrong-calls-over-bar` (3 wrong of 32), as the bar requires.

## Surfaced (for the wrap)
- The driver lint's `overlap` on a select's options (above) — a driver change for every document.
- The flight booker's buttons were the stand's only `aria-pressed` control: the pressed reading is no longer exercised
  on the stand (v010-17's stand witness now reads `selected`; `accessibility_states` still pins `aria-pressed` on a
  fixture). CLAUDE.md, a11y and design-system prose naming the mode buttons or `aria-pressed` on the flight booker move
  with the wrap.
- Stale test names in the ledgers: the 0.1.0 matrix ref names
  `stand_snapshot_state.rs::exactly_the_active_flight_mode_reads_pressed` and
  `stand_act_states.rs::a_flight_mode_click_returns_both_mode_buttons_with_their_pressed_reading`; the 0.2.0 v020-05 ref
  names `stand_lint.rs::the_stand_at_boot_reads_no_finding`. They are now `exactly_the_chosen_flight_type_reads_selected`,
  `a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading` and
  `the_stand_at_boot_reads_its_measured_findings`.
- `docs/capabilities.md`:288-289 (the 2026-10-15 run's line) is the wrap's to re-read against this run (scope).
- Residual `.andromeda/residuals.md:38` (a paged part read through the host's sink) is now measured — `host_log`
  `a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace`, under both builds.

## Process hygiene
`gate.py census --comm escher-session --comm claude --comm seven_guis_native --args 'escher-session mcp' --args
'escher-session serve'` (trail `census-2026-10-25-cold-agent-bar.json`): `census: 11 running`.

| process | started by | final state |
|---|---|---|
| `escher-session` (hosts, `mcp` children of the checks and the live run) | this run · a leg | terminated — 0 running |
| `claude -p` of the live run | a leg (entry 23) | terminated — no `claude -p` among the 11 |
| `seven_guis_native` | none started | — |
| 11 `claude` sessions (the overseers, builders, worker and this session) | NOT DERIVED — not started by this run | left running: the operator's own sessions |

Left on disk (a recursive `rm` is denied on this host): `target/tmp/recprobe-1` (the recording-size probe's kept checks,
owner-only) and the scratchpad's `rbg-1` (HEAD's pipe beside the new contract tests), both outside every commit.

## Decisions & corrections
- The founder's word, relayed, on two P1 questions: re-pin `stand_lint` as measured; no prompt hint (inputs#I5).
- The founder's word, relayed verbatim, on step 12: «Запускать сейчас (Рекомендую)» — fired once (inputs#I6).
- The first ask for the live run timed out after 58 min with no answer (held by the driver's hook); the run stopped with
  an interim outcome and nothing fired until the word came.
- Guard refusals: a `cat` heredoc to a file (re-issued through Edit) and a recursive `rm` (re-issued without it, fresh
  directories used).
