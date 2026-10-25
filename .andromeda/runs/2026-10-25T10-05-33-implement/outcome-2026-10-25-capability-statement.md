# /implement outcome — 2026-10-25-capability-statement

**Outcome: green** — smoke skipped (no boot-path or UI-surface change; the plan lists no smoke or self-verify entry).
Next → the OPERATOR PASS, entries 12-16 (hygiene · the pre-CI commit then the push entry 13 · the CI read 14 · the
pointer's commit line 15 · the operator's confirmation, step 6, read by 16), then `/andromeda-wrap-session`.

## Files changed
- `docs/capabilities.md` — new: «What Escher can do», five sections in the plan's order (step 1).
- `README.md` — line 48: «the eleven» → «the twelve driver commands as tools», and one sentence linking
  `docs/capabilities.md` on the same line (step 2; numstat `1 1`).
- `.github/scripts/test_capability_statement.py` — new: the drift pin, four cases (step 3).
- Outside this repository: `../viola/viola-0.2.0-incubator/research/escher-capabilities.md` — the untracked pointer,
  its `commit:` line the placeholder step 4 names (step 4; ignored by Viola's `.gitignore:6`).
- Evidence: `chunks/2026-10-25-capability-statement/evidence/{counts,pin-controls,viola-pointer}.md`.

## Gates
- Whole block: `entries 16 · green 11 · red 0 · recorded 0 · timeout 0 · not-run 5` (the five `operator` entries).
- Re-fired after the a11y wording edit: `--only 1,4,6,9,10` green 5 · red 0; after its last tightening `--only 1,4,6`
  green 3 · red 0. Trail: `.andromeda/runs/2026-10-25T10-05-33-implement/gate-2026-10-25-capability-statement.json`.
- Workspace tally (entry 10): `lines 206 passed 1105 failed 0 ignored 12`, as predicted. `ci-scripts`: `Ran 104
  tests` · `OK`. `ci-leg.sh doc`: exit 0.
- Guard (entry 6), its pass lines copied whole from `gate.py show --n 6`:
  `guard: allowed 3 — .github/scripts/test_capability_statement.py, README.md, docs/capabilities.md · companions excluded 0 · changed 3 · excluded 0`
  `guard: clean`
  No file excluded.
- No source-delta deferral: every listed gate ran, the Rust workspace legs included.
- The pin's red-before-green: two scratch-copy controls read each of the four cases red on its own drift
  (`evidence/pin-controls.md`).

## Matrix
`v020-10` (`manual`): no ref written. Its acceptance needs the pushed commit, the pointer's `commit:` line naming it and
the operator's confirmation in `evidence/operator-confirm.md` — none existed at /implement, so the pin's green did not
satisfy it as written.

Written after the operator pass (entries 12-16 green, `evidence/operator-pass.md`; the founder's «Подтверждаю
(Рекомендую)» in `evidence/operator-confirm.md`): `v020-10 · status planned → implemented`, ref
`manual: escher-0.2.0/chunks/2026-10-25-capability-statement/evidence/operator-confirm.md (confirmed: e7e71221dfdcd29c6df603d222104855f43c07f4 · 2026-10-25 · the founder, «Подтверждаю (Рекомендую)») · docs/capabilities.md at e7e71221d, pinned by .github/scripts/test_capability_statement.py (4 tests, green on CI#40184674449, 17/17) · README.md:48 links it · Viola pointer viola-0.2.0-incubator/research/escher-capabilities.md names docs/capabilities.md at that commit (evidence/viola-pointer.md; operator entries 15, 16 green)`.

## Deviations from plan, with justifications
1. **The pin's count case reads five tables, not three.** Beside the verb, cause and session-error counts it reads
   `NODE_FIELDS` (thirteen node keys) and `EXIT_STATUSES` (four exit statuses) from `schema.rs`, since the (arch)
   acceptance names those two counts as «read from the schema's tables and none hand-copied». Same file, same case,
   still four tests.
2. **The pin folds the markdown's whitespace** before matching a phrase: its first run read «thirteen node keys» red
   because the phrase wrapped across two source lines. It asserts `phrase in text` with the phrase as the message
   rather than `assertIn`, so a red names the missing phrase instead of printing the whole statement.
3. **The statement's phrasing** of the counts is `{word} verbs` · `{word} refusal causes` · `{word} session errors` ·
   `{word} node keys` · `{word} exit statuses`, the forms the pin reads.
4. No out-of-list file in the repository: `scope: clean — changed 3 · listed 3 · recorded 0`; `scope-record.md`
   absent. The Viola pointer is the plan's one listed write outside the repository.

## Surfaced
Nothing. The plan's `Expected amendments (wrap)` stand as written; no new spec↔reality gap met.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc (the `fast` and `doc` legs) | a leg (entries 9, 11) | terminated — `census: none running` (comm cargo, rustc, escher-session, seven_guis_native) |
| escher-session / seven_guis_native | none started by this run | none running |

## Decisions & corrections
- Guard refusal: the Bash guard refused a `cd` into the chunk's `inputs/` dir («moves the session cwd for every later
  call»); re-issued once with absolute paths and ran.
- No operator correction or ruling met in this run.
