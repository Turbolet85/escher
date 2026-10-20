# Outcome — 2026-10-19-visual-lint (gated mode)

**Outcome: green + smoke.** Plan step 12 is built against the real carriage. The premise is cleared by the founder's own
word on F1, «Новая команда lint (Рекомендую)», option 1 «New verb, all carriers, recorded», and on F2, «Без поля
(Рекомендую)», option 1, by dialog on 2026-10-20 at about 04:35Z, relayed verbatim by the overseer. It is snapped as
inputs#I7 and recorded in `evidence/founder-word.md` (gate 20 reads it, 1 row). `lint` is the eighth instance verb and
the twelfth verb, on `Session::run`, the socket, the command line, MCP and `help`, and is recorded through the table.
Its span records `verb` and, when refused, `cause` only (F2). v020-05 is `implemented`, its ref written with
`--cleared-file .andromeda/runs/2026-10-20T07-11-30-implement/cleared-v020-05.md`.

## Gates
One firing of the block, trail `.andromeda/runs/2026-10-20T07-11-30-implement/gate-2026-10-19-visual-lint.json`:
`entries 22 · green 15 · red 3 (7,10,14) · recorded 0 · timeout 0 · not-run 4 · voided 3 (20,21,22)`. Entries 20–22 were
voided `block cleared — the founder's own word on F1 «Новая команда lint (Рекомендую)» = option 1 (inputs#I7 ·
evidence/founder-word.md)`, all three green: 20 reads 1; 21 is `stand_lint` 6 passed, with
`the_lint_verb_answers_what_the_reader_reads ... ok`; 22 is `cli_commands` 13 passed, with
`lint_answers_one_line_of_findings ... ok`. `ci-leg.sh fast` (13), `doc` (15) and `a11y` (12) are green.

The three reds are atoms the block froze at the first take-up, not failures of this chunk's code. Each is measured
against HEAD's own control (`escher-0.2.0/chunks/2026-10-20-select-by-id/evidence/counts.md`):
- **10** `run stand` reads `192 · 0 · 7` against the atom 183. Control on HEAD: 190 · 0 · 7. +2 by name:
  `stand_lint::the_lint_verb_answers_what_the_reader_reads` and
  `stand_mcp_tools::a_lint_tool_call_answers_the_readers_findings_and_parents_its_command`.
- **14** the workspace tally reads `lines 204 passed 1087 failed 0 ignored 12` against the atom 201 · 1055. Control on
  HEAD: 204 · 1083. +4: the two above, plus `cli_commands::lint_answers_one_line_of_findings` and
  `mcp_server::a_lint_tool_call_answers_its_line_of_findings`.
- **7** the guard «no tracked file outside the in-process change moves before the founder word» reads 40. 23 files
  were committed by 2026-10-20-load-wait and 2026-10-20-select-by-id (`git diff e97a7f41b HEAD`). 20 are this tree's
  step 12: the 19 gated files plus `stand_act_scroll.rs`. 3 files are on both sides. His word, option 1, is the crossing
  this guard forbade, so the guard's premise has fallen and the plan offers no `defer` to lift it. The wrap's light gate
  decides how it reads 7, 10 and 14.
- Entry 8 (no new untracked code file) stays green: step 12 adds no file in the code tree.

Iterations: 1, with no fix needed. Deferrals: none.

## Smoke
Fired: the `escher-session` command line gained a verb. Run by hand from `target/debug/escher-session` (built
07:25Z, after the edits) in `target/tmp/vl-smoke`:
- `start counter`: exit 0.
- `lint`: `{"findings":[]}`, exit 0.
- `help`: its verbs are `snapshot … screenshot, lint, start, status, stop, help`, twelve.
- `stop`: exit 0.

## Files changed
- escher-driver:
  - `schema.rs`: the `LINT` row and its `findings` field (`Entries`). `VERBS` now holds twelve.
  - `command.rs`: `Command::Lint`, its validation and `spec`. Test rows 30 → 31 and 39 → 40; the table pins move to
    12 and (8, 3, 1).
  - `execute.rs`: `Outcome::Linted { findings }`, the `lint` arm calling `Session::lint`, and the early return in
    `record_result`.
  - `json.rs`: the finding's written form — `id` · `defect` · `bounds`, then the reason's keys; boxes as node
    bounds; ratios in their `f32` form; `visible` left out when none can be seen. Two outcome rows added (8 rows).
  - `describe.rs` and `mcp.rs`: the counts and docs.
  - `lib.rs`: the docs.
  - `schema_tests.rs`: `the_eleven_verbs…` is renamed `the_twelve_verbs…`; the lint row and its field kind are added.
  - `mcp_tests.rs`: `the_listing_is_the_eleven…` is renamed `…twelve…`; a lint with no session, and a lint with an
    argument, are added.
  - `cli_tests.rs`: the lint line rows (96) and the usage line.
  - `recording_tests.rs`: a lint step kept whole (10 calls).
- blitz-tests:
  - `session_common/mod.rs`: `lint()`, `linted()`, and the `act` arm.
  - `stand_lint.rs`: `the_lint_verb_answers_what_the_reader_reads`.
  - `stand_act_spans.rs`: `Expect::Linted`; a lint and a refused no-font lint (34 calls, eight verbs read back).
  - `stand_mcp_tools.rs`: `a_lint_tool_call_answers_the_readers_findings_and_parents_its_command`.
  - `stand_act_scroll.rs`: `VERBS.len()` 12.
- seven_guis:
  - `cli_commands.rs`: `lint_answers_one_line_of_findings`; `VERBS` 12.
  - `mcp_server.rs`: `a_lint_tool_call_answers_its_line_of_findings`; the listing is twelve, with `lint` before
    `start`.
  - `host_log.rs` and `mcp_log.rs`: one `lint` each on their flows. Their checks for no leaked ids, names, typed
    text or image bytes still read 0, inside the fast leg.
- `scripts/cold-agent.sh` and `.github/scripts/test_cold_agent.py`: the twelve verb names, and the fake answering
  `lint`. The 33 contract tests pass.
- Chunk records: `evidence/founder-word.md` (new), `evidence/counts.md` (the gated-pass section), `scope-record.md`
  (new), `inputs/I7-morning-founder-answers-2026-10-20.md.txt` (snapped), and the `v020-05` entry in
  `escher-0.2.0/verification-matrix.json`.

## Deviations from the plan
- `tests/blitz-tests/tests/stand_act_scroll.rs` is outside research's lists. Its line in `scope-record.md`:
  `` `tests/blitz-tests/tests/stand_act_scroll.rs` · companion · serves `packages/escher-driver/src/schema.rs` · self ``
  Research's sweep read it as table-reading with no count pin, but it asserts `VERBS.len() == 11`.
- Two test names renamed eleven → twelve, as the 2026-10-12-headless-screenshot chunk renamed ten → eleven:
  `schema::tests::the_twelve_verbs_are_named_in_order_each_at_its_level` and
  `mcp::tests::the_listing_is_the_twelve_verbs_of_the_table`. Records that cite the old names are the wrap's
  amendments.
- Two tests added beyond the plan's three named ones: `stand_mcp_tools::a_lint_tool_call_…` and
  `mcp_server::a_lint_tool_call_…`, in files research lists for the MCP carriage.
- The plan's smoke section says there is no boot path to smoke. The binary gained a verb, so it was smoked by hand
  (above).

## Surfaced
Nothing is a spec↔reality gap. For the wrap: gate entries 7, 10 and 14 cannot read green as written in this gated pass
(see Gates).

## Scope read
`scope: clean — changed 22 · listed 21 · recorded 1 (companion 1 · mechanical 0 · in-intent 0 · widening 0) ·
absorbed 0 · excluded 15`

## Process hygiene
| process | started by | final state |
|---|---|---|
| `escher-session` host (counter, `target/tmp/vl-smoke`) | this run (P3 smoke) | terminated (`stop` answered `{"stopped":true}`) |
| gate entries' cargo, test and agent-run children | this run (the gate tool) | terminated |

Census: `census · ps · comm escher-session, seven_guis_native · args vl-smoke, agent-run · own tree excluded: 8` →
`census: none running`.

## Decisions & corrections
- A guard refusal: one `cd` into the package dir was refused by the Bash guard («moves the session cwd»), re-issued
  with absolute paths.
- A standing learning held: 2026-10-10 «search `tests/` and `examples/*/tests/` for the table's name before calling a
  check unaffected». A `git grep` for `VERBS` found the `stand_act_scroll.rs` pin, which research's sweep had missed.
- The founder's word, as given: F1 «Новая команда lint (Рекомендую)», F2 «Без поля (Рекомендую)». Both are recorded
  verbatim in `evidence/founder-word.md`.

## Next
The operator pass: the plan's `operator` entries 16 (hygiene), 17 (the pre-CI commit — subject exactly
`chore(2026-10-19-visual-lint): operator pre-CI commit, for the run this chunk's verdict reads` — then the push), 18
(`ci.py conclusion`) and 19 (the per-OS tallies into `evidence/ci.md`). Then this chunk's gated wrap
(`/andromeda-wrap-session`).
