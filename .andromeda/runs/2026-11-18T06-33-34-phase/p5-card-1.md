# P5 review — 2026-11-18-cold-agent-bar («Cold-agent bar», escher-0.3.0, Epoch 6's last entry)

**Plan:** `.andromeda/cache/docs/2026-11-18-cold-agent-bar/plan.md`, rendered from `doc:2026-11-18-cold-agent-bar/plan`
rev 3 (317 lines). Open it in an editor. No plan stands in a chunk's folder. Writing in that file changes no record
until a `put`, and `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-cold-agent-bar`
is a diff of the chunk's records.

**The founder's forks:** `.andromeda/runs/2026-11-18T06-33-34-phase/forks-founder.md`. Five forks are written
undecided, each with options, measured basis and what depends on it, and none marked as recommended (inputs#I1, inputs#I2):
- F1, the bar's number (beside it: 0.3.0's prompt is not the one 0.2.0's 66 calls were measured on);
- F2, what README and the capability page may claim;
- F3, the live run's word;
- F4, `graph`/`actions` answers to the model provider, a boundary widening;
- F5, a served error page so a cold agent meets `caused`.

## Goal
The pipe can judge 0.3.0's run. One `stand` prompt asks for 0.3.0's carried reads (`graph` on the counter,
`actions` on the flight booker, `layout` on CRUD's people list, none on Cells) and the flight type chosen through its
painted list. The verdict counts and requires each (`graph_calls`, `actions_calls`, `layout_calls`, `list_opens`) and
reads refusals by cause (`refused_by_cause`, the eight closed words). The bar stays 5 until F1. The live run, the bar's
number and the docs' claims wait on the founder. `caused` is named out of reach (measured).

## Top acceptance criteria
- The cold-agent contract tests and the `ci-scripts` leg pass. Each new or changed case reads red against HEAD's pipe
  first, and controls (a)–(d) read their cases failed.
- The verdict's new fields are counts or closed words; the scrub-key and planted-marker cases stay green.
- A run leaving any of the four new reads unused fails `capability-unused`.
- The prompt names no stand id or slug, and the secret search over the two docs reads 0.
- Nothing outside the four touchpoints changes (guard, base `764b4ba6`).
- **Gated:** `matrix#v030-15` — the one live run, on the founder's word, reads `"outcome": "passed"`.

## Touchpoints
- `scripts/cold-agent.sh` — steps 1–5 (step 11 gated)
- `.github/scripts/test_cold_agent.py` — steps 6–8 (step 11 gated)
- `README.md`, `docs/capabilities.md` — step 10, gated on F2

## Test commands (11 entries)
1. `bash -n` on the pipe.
2. The cold-agent unittest file: baseline green, 38 tests.
3. The `ci-scripts` leg: baseline green, 11.34 s.
4. The secret search over the two docs: baseline green, its known positive red in a clone.
5. The guard: baseline green, the clone pair reading clean and outside.
6. `fast`: seen green at the last wrap.
7. Build, operator, **gated**.
8. Live run, operator, **gated** (F3, F4).
9. Hygiene, operator.
10. Push, operator.
11. `ci.py`, operator.

Controls: `.andromeda/runs/2026-11-18T06-33-34-phase/p5-controls.md`.

## Validation
- **Mechanical checks 1–12:** pass (dry-run parse, planlint 0 hits, lists parsed, citations resolved, baselines
  written).
- Entries 2 and 4 read red on their first baseline for an entry defect, not the subject: `unittest` refuses a
  `.github` path, and the pipe and tests spell `apiKeySource` by design. Both were re-spelt and re-baselined green.
- **Validation-1:** aligned with the scope as P3 corrected it.
- **Graph:** not applicable (bash and python only).

## Concretization preview — `matrix#v030-15` (gated claim)
- **Current:** «A cold agent's run over 0.3.0's new capabilities passes the bar the founder sets from 0.2.0's baseline
  of six apps, 66 calls and 0 wrong.»
- **Proposed:** «The one live `bash scripts/cold-agent.sh run stand`, fired on the founder's word, reads
  `"outcome": "passed"` in `target/cold-agent/verdict.json`. All six apps are met. `graph_calls`, `actions_calls`,
  `layout_calls` and `list_opens` are each ≥ 1, and lint, screenshot, advance and a part read are each used. The
  recordings are kept. The agent's check replays green and can fail. `wrong_calls × 100 ≤ B × tool_calls`, where B is
  the founder's F1 number. The run's reach statement names `caused`, handler coverage, a fixture state, exploration, a
  component alone, the seed and the bundled fonts as outside a cold agent's MCP carrier, each with its headless
  witness.»
- **Gated on:** `BLOCKED-ON: the founder's word on F1 (the bar), F3 (the live run) and F4 (graph and actions answers
  to the model provider) — .andromeda/runs/2026-11-18T06-33-34-phase/forks-founder.md`. Stamped on the chunk's line at
  step 4 after the word.
- The outcome is not weakened: the bar is still the founder's, judged on a real cold agent. The concretization adds
  four required reads and names what the run cannot reach.

## Beyond the plan
- The five founder forks (F1–F5) are answered outside this plan, relayed by the operator. F5's B or C would open new
  work: a re-plan of this chunk, or a 0.4.0 route entry. Owner: the founder, through the operator.
- A stale figure is named for the report: residual `res-7cc5bi4k` and the 2026-11-08 record state the Cells `actions`
  answer as 106,088 B, while it measures 118,292 B at HEAD (each node now lists `layout`). Owner: the wrap's route
  adaptation (Carry-forward).

Apply? (yes / review / cancel)
