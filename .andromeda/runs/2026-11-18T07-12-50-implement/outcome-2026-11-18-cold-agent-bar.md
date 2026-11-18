# Outcome — 2026-11-18-cold-agent-bar · green

**Run:** 2026-11-18T07-12-50-implement · mode: pending (not gated) · base 764b4ba63 (no pre-CI commit)

## Files changed
- `scripts/cold-agent.sh`: steps 1–5.
  - The stand prompt widened: the counter's UI graph, the flight booker's actions and its list opening, the CRUD
    people list's layout, and the Cells guard against graph and actions.
  - `COUNTED` widened to six verbs.
  - New: `CAUSES`, `CAUSE_FIELD` and `list_opens()`.
  - `span_counts()` now returns `by_cause`.
  - The verdict gains `graph_calls`, `actions_calls`, `layout_calls`, `list_opens` and `refused_by_cause`.
  - `capability-unused` reads eight fields.
  - The bar's comment is restated, `WRONG_CALL_BAR_PCT` unchanged at 5.
- `.github/scripts/test_cold_agent.py`: steps 6–8.
  - The fake writes `command` span lines (with `cause` when refused) for instance verbs.
  - New knobs `causes` and `lists`.
  - `ACTS` and `EXTRA` widened; `FLIGHT_TYPED` added.
  - Four cases changed and three new (41 cases; the ci-scripts leg runs 124).
- `README.md` and `docs/capabilities.md`: untouched. Step 10 is gated on F2.

## Deviations (with justifications)
- **Order.** Steps 6–8 (tests) were written before steps 1–5 (pipe). This let step 8's red-before-green be read
  against HEAD's untouched pipe in the working tree. Red reading: 41 tests, failures=5 and errors=5, all seven
  new or changed cases red. Recorded in `.andromeda/cache/chunks/2026-11-18-cold-agent-bar/evidence/red-before-green.md`
  (its raw capture moved to the session scratchpad under §Hygiene's capture rule).
- **The prompt test's goal words.** Step 8 named `UI graph` and `what can be done`. The test pins `read its UI graph`
  and `ask what can be done on its screen` instead. Control (d) read DOES NOT HOLD with the plan's words, because
  both phrases also stand in the Cells sentence. The longer words each stand once, and control (d) holds with them.
- **The fake's acting answer.** It now carries `added`, `removed` and `changed`, the driver's own acting-answer shape
  (`packages/escher-driver/src/json/diff.rs`). The fake's `{"settled":true}` answer had no `changed` list.
- **No `command` line on a fake `error` ending.** An instance verb called with no session writes no `command` line.
  The reason: the real host answers it before `Session::run`. This is assumed, not measured; research measured only
  the session-verb case.
- **The `screenshot_calls` sub-case** keeps the flight booker's `actions` call. Its single reason is then the
  screenshot alone.
- Scope record: none. `gate.py scope`: `scope: clean — changed 2 · listed 2 · recorded 0`.

## Gates (run 2, the final tree; run 1 identical in verdicts before the control (d) fix)
- `bash -n scripts/cold-agent.sh` — green · exit 0
- `python3 -m unittest discover -s .github/scripts -p test_cold_agent.py` — green · exit 0
- `bash .github/scripts/ci-leg.sh ci-scripts` — green · exit 0
- `git grep -n -i -E "/home/|api[_-]?key|authorization:|bearer " -- README.md docs/capabilities.md` — green · exit 1 · no output
- `gate.py guard … --base 764b4ba63…` — green · last line `guard: clean`. Pass line, copied whole:
  `guard: allowed 2 — .github/scripts/test_cold_agent.py, scripts/cold-agent.sh · companions excluded 0 · changed 192 · excluded 190`
  (the 190 excluded are the pipeline's own artifacts under `.andromeda/` and the route files, outside the pathspec's
  product reach)
- `bash .github/scripts/ci-leg.sh fast` — green · exit 0 · 497.88 s
- `cargo build --locked -p seven_guis --bin escher-session` — not run, leg operator. Gated on F3/F4.
- `bash scripts/cold-agent.sh run stand` — not run, leg operator. Gated on F3/F4.
- `gate.py hygiene` · the push · `ci.py conclusion` — not run, leg operator (the operator pass).
- Summary: `entries 11 · green 6 · red 0 · recorded 0 · timeout 0 · not-run 5`.

## Controls (`gate.py control`, entry ci-scripts, --timeout 300)
- `.andromeda/cache/chunks/2026-11-18-cold-agent-bar/evidence/control-1.txt` holds (a), (b), (c) and the first (d).
  - The tool reads (a), (b) and (c) INDETERMINATE "no result line": its case reader does not know unittest's
    `FAIL:` form.
  - By hand, from the removed logs: (a) `test_each_unused_capability_fails (field='graph_calls')` failed (1 failure).
    (b) `test_refusals_are_counted_by_their_cause_from_the_command_lines` failed (1 failure).
    (c) `test_a_list_opening_is_read_from_the_acting_answers` failed (among 34 failures).
  - The first (d) read DOES NOT HOLD.
- `.andromeda/cache/chunks/2026-11-18-cold-agent-bar/evidence/control-2.txt` holds the re-fired (d), after the fix.
  By hand: `test_the_prompt_names_no_stand_id` failed (1 failure).
- All four controls hold. Each was read green in place in the clone.

## Matrix
- v030-15 is `planned, gated`, on the founder's F1, F3 and F4. No ref was written: a gated cap takes none until its
  block clears.

## Surfaced / stuck
- none

## Next
The OPERATOR PASS:
- entry 9, `gate.py hygiene`;
- entry 10, the push;
- entry 11, `ci.py conclusion`.

Then `/andromeda-wrap-session`. Entries 7 and 8 (the build and the live run) wait on the founder's F3 and F4, and
steps 10 and 11 on F2 and F1 (`.andromeda/runs/2026-11-18T06-33-34-phase/forks-founder.md`).

## Process hygiene
| process | started by | final state |
|---|---|---|
| fake `escher-session` / `claude` shim children of the contract tests | a leg (entries 2, 3, 6 and the controls) | terminated — `gate.py census --comm escher-session --comm claude --args cold-agent.sh` reads no `escher-session` and no `cold-agent.sh` process |
| `claude` sessions (9 rows in the census) | NOT DERIVED — the host's own sessions (overseers, builders, this session) | left running: not this run's |

## Decisions & corrections
- A control's goal word that also stands elsewhere in the prompt makes a deletion control vacuous. Pin each goal by
  a phrase standing once. Evidence: control (d) DOES NOT HOLD → holds (control-1.txt, control-2.txt).
- `gate.py control` reads no Python unittest case line, so a control on the `ci-scripts` leg reads INDETERMINATE
  "no result line". Read its case from the removed log by hand. Evidence: control-1.txt.
- Guard refusals: the Bash guard refused twice a `cd` that would move the session cwd. Each was re-issued once, as
  absolute paths or a Read.
- Hygiene at P4:
  - It refused `evidence/red-before-green.txt` (a unittest capture, ×14 `in-root`). The capture moved to the
    scratchpad, and its figures are kept in `evidence/red-before-green.md`.
  - It refused `.andromeda/runs/2026-11-18T06-33-34-phase/p5-controls.md:10` (a planted home path with a made-up user). It was
    rewritten to `/home/<user>/x` on the operator's word, inputs#I4. That word is standing: such a path is written in
    the `<user>` form, and a met one is rewritten and named without asking.
  - Hygiene then read `clean`.
