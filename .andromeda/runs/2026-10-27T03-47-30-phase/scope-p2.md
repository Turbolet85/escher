# Scope — 2026-10-27-readme-and-records

**Working entry (verbatim, escher-0.3.0/working-route.md:13):** README and records — README shows 0.2.0 done, 0.3.0
planned; capabilities statement on the 2026-10-25 run; ledger refs, design stand rows, incubator ignore (v030-02,
v030-16). No freight block stands on it (`route.py pins`: one freight block on the tail, at line 54, none on line 13).

**Caps:**
- v030-02 «The version on the GitHub page» (`method: manual`). Requirement: the README shows 0.2.0 done and verified,
  with what it can do and its cold-agent result, and 0.3.0 planned with its stages; the capabilities statement
  describes the 2026-10-25 run. Acceptance (as routed): the project's front page on GitHub shows 0.2.0 done and
  verified, with what it can do and its cold-agent result, and 0.3.0 planned with its stages; the capabilities
  statement describes the 2026-10-25 run of six apps, 66 calls and 0 wrong, and the statement's pin stays green.
- v030-16 «Records kept true» (`method: manual`). Requirement: the two verified ledger refs name the tests' current
  names, the design-system stand rows (the accent, the success row, the cards item, the stand's DejaVu Sans item) move
  out of the product surface into architecture and layout-templates, and the incubator is ignored by `.gitignore` in
  place of a local exclude. Acceptance (as routed): the two ledger references name the tests' current names, the
  design-system's stand rows stand in architecture and layout-templates instead, and the incubator is ignored by the
  repository's own ignore file.

**The founder's word:**
- 2026-10-26, his 0.3.0 scope (inputs#I1): «…плюс первыми чанками надо будет сделать улучшения кодовой базы на основе
  аудита, обновить ридми под новую версию и синхронизироваться с апстримом». The README entry is the second of the
  first three, after the codebase improvement, which is complete (2026-10-26-codebase-health).
- 2026-10-16, §The README (inputs#I2): «…обновление ридми чтоб на гите было видно текущее состояние а то сейчас там
  описание 0.1.0 и еще хотел попросить чтоб эшер там с большой буквы писался». The overseer read it as follows: the
  GitHub page shows the current state, and the project's name is written «Escher», capital E, in the README's prose.
  Crate, binary and path names such as `escher-driver` and `escher-session` stay as they are.
- Neither is a delegate answer, so neither is PROVISIONAL.

## What this chunk builds

### The README (v030-02)
- **`README.md` §Status** shows 0.1.0 done and verified, 0.2.0 done and verified, and 0.3.0 planned with its stages.
  Today it reads "0.2.0 — planned, in five stages" (`README.md:66`).
  - 0.2.0 done and verified: its stages as built, what it can do, and its cold-agent result. [inferred] The facts are
    the 0.2.0 records': its master-route section, its verification matrix, its last chunk's report
    (2026-10-25-cold-agent-bar) and its version-end audit. P3 reads each number from its record, never from this
    scope.
  - 0.3.0 planned with its stages: the working route's six epochs (`escher-0.3.0/working-route.md`, `### Epoch 1` to
    `### Epoch 6`). [inferred] They are written as plans, in the README's own register, and name no marker.
  - The "Later:" line (`README.md:74`) is restated to what is past 0.3.0. [inferred] Its source is
    `escher-0.3.0/intent.md` §Out of this version.
- **The README's other sections** describe the current state where they describe 0.1.0 alone. [inferred] "What Escher
  is", the driver paragraph and "Try it" name what 0.2.0 added: the visual lint, a session recorded as a replayable
  check, a large screen read in parts, a `select` driven by id, the wait for a page load, and the virtual clock. Each
  is named only as the capabilities statement already states it.
- **The name.** «Escher», capital E, in every prose mention in `README.md` (inputs#I2). Crate, binary, path and command
  names stay lowercase as spelled in code (`escher-session`, `escher-driver`, `escher-0.3.0/`). [inferred] On the
  first read at P1, every prose mention already reads «Escher». The rule binds every line this chunk writes.
- **The README's tool count** stays the driver schema's: "the twelve driver commands as tools" (`README.md:56`).
  `.github/scripts/test_capability_statement.py` holds it to `VERBS`, and that pin stays green.

### The capabilities statement (v030-02)
- **`docs/capabilities.md:288-291`** is restated against the 2026-10-25 cold-agent live run: six apps, twelve tools,
  66 calls, 0 wrong, with a screenshot and a lint called (`.andromeda/residuals.md:41`, absorbed:v030-02). Today it
  reads the 2026-10-15 run: one run, eleven tools, 43 calls, «It has not been re-fired since `lint` made the tools
  twelve». [inferred] What crossed to the model provider on that run (ids, names and values in snapshot texts, paged
  parts and diffs, a lint's findings and one screenshot) is stated as the security plan records the run's crossing.
  P3 reads it from `.andromeda/security-plan.md` and the run's report.
- **The statement's pin stays green.** `.github/scripts/test_capability_statement.py` holds its verb and cause names
  and its five counts to the driver schema's tables.
- [inferred] Any other line of `docs/capabilities.md` that names the 2026-10-15 run as the latest is restated the same
  way. P3 sweeps the file.

### The records (v030-16)
- **Two verified ledger refs** (`.andromeda/residuals.md:42`, absorbed:v030-16) name the tests' current names. They are
  `escher-0.1.0/verification-matrix.json` v010-17 (`stand_snapshot_state::exactly_the_active_flight_mode_reads_pressed`,
  now `exactly_the_chosen_flight_type_reads_selected`) and `escher-0.2.0/verification-matrix.json` v020-05
  (`stand_lint::the_stand_at_boot_reads_no_finding`, now `the_stand_at_boot_reads_its_measured_findings`). The intent
  calls it «a factual correction with no word needed». [inferred] Each new name stands in its test file, and the write
  goes through the ledger tool on a verified cap of a closed version. P3 reads whether `matrix.py` has a verb that
  re-points a verified ref, and whose step owns that write.
- **The design-system stand rows** move out of `.andromeda/design-system.md`'s product surface into
  `.andromeda/architecture.md` and `.andromeda/layout-templates.md`. The rows are the accent row, the success row, the
  cards item and the stand's DejaVu Sans item, on the founder's «В 0.3.0, через инкубатор (Рекомендую)»
  (`escher-0.3.0/intent.md` §Also carried; incubator `deferred-from-0.2.0.md`, end: «a 0.3.0 wrap or door moves
  them»). [inferred] The move is a master amendment. Phase never edits a spec source, so the plan names it as an
  expected wrap amendment. P3 reads the four rows where they stand, the masters' section each one lands in, and
  whether `design-system.md` is an extracted master with a door (`detail.py check`).
- **The incubator ignore.** [inferred, measured at P1] `.gitignore:39` already reads `escher-0.3.0-incubator/`, added by
  `143b60132` (2026-10-16), so the repository's own ignore file already ignores the incubator. What is left is the local
  copy, two lines at the end of `.git/info/exclude` («# escher 0.3.0 incubator (the founder, 2026-10-16), local until a
  .gitignore line replaces it» and `escher-0.3.0-incubator/`). It is an untracked, local file that no commit carries.
  P3 closes whether removing those two lines is this chunk's work, and how the acceptance is witnessed (`git
  check-ignore -v` naming `.gitignore:39`).

## Boundaries
- No code, test, engine, driver, CI or schema change: no verb, field, carrier, log site, span or count changes. The
  README's and the statement's counts follow the schema and are not moved by this chunk.
- The README states only what the repository's records prove (verified caps, measured runs). It makes no claim the
  capabilities statement or a record does not hold.
- The upstream sync, the engine debts and every theme entry are other chunks. The README names them as 0.3.0's
  planned stages and does nothing toward them.
- The README's license, author and contact sections are kept as they stand.
- The `escher-0.2.0-incubator/` ignore line (`.gitignore:37`) is not touched.

## CI read at take-up (Setup 5a)
- `316c6d25b59a` (HEAD, the 2026-10-26-codebase-health wrap commit): verdict green, checks 17/17, wall 1266 s,
  CI#40444036930 push completed/success (trail `.andromeda/runs/2026-10-27T03-47-30-phase/ci-no-marker.json`). Nothing
  to fold.
