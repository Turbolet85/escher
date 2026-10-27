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
  - 0.2.0 done and verified: its stages as built, what it can do, and its cold-agent result. The facts are the 0.2.0
    records': its five epochs (`escher-0.2.0/working-route.md`), its matrix (verified 11/11, `matrix.py coverage`), the
    run's verdict (`escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-verdict.json`: six apps, 66 calls, 0
    wrong, the 5 % bar) and the version-end audit as `escher-0.3.0/intent.md` §Findings 1 records it. Each number is
    read from its record, never from this scope. [verified at P3: research.md §Files inspected]
  - 0.3.0 planned with its stages: the working route's six epochs (`escher-0.3.0/working-route.md`, `### Epoch 1` to
    `### Epoch 6`), written as plans in the README's own register, naming no marker. [verified at P3: six `### Epoch`
    headers, lines 10-51]
  - The "Later:" line (`README.md:74`) is restated to what is past 0.3.0, from `escher-0.3.0/intent.md` §Out of this
    version. [verified at P3: that section names L8, L13, L14, L24 and the advisories]
- **The README's other sections** describe the current state where they describe 0.1.0 alone. "What Escher is", the
  driver paragraph and "Try it" name what 0.2.0 added: the visual lint, a session recorded as a replayable check, a
  large screen read in parts, a `select` driven by id, the wait for a page load, and the virtual clock. Each is named
  only as the capabilities statement already states it. [verified at P3: each stands in `docs/capabilities.md` §What it
  can do]
- **The name.** «Escher», capital E, in every prose mention in `README.md` (inputs#I2). Crate, binary, path and command
  names stay lowercase as spelled in code (`escher-session`, `escher-driver`, `escher-0.3.0/`). Every prose mention
  already reads «Escher», and the rule binds every line this chunk writes. [verified at P3: 0 lowercase prose hits,
  research.md §Sweeps]
- **What the GitHub front page renders.** The page renders the default branch's README, and the default branch is
  `build/escher-0.1.0` (`gh repo view Turbolet85/escher --json defaultBranchRef`; `origin/build/escher-0.1.0` at
  `1a34eb2bb`, 2026-10-16). That README holds no `## Status`: it is the 0.1.0 description the founder saw. An edit on
  `build/escher-0.3.0` alone does not reach the page. [premise-corrected: the entry assumed the README edit shows on
  GitHub; measured at P4]
- **gate:** the founder's own word on F3, whether the front page comes to render `build/escher-0.3.0`'s README (the
  operator's «Ask the founder first», inputs#I4) — it makes the front page's reading of 0.2.0 done and 0.3.0 planned
  impossible tonight. Everything else in this chunk completes now: the README and the statement on this branch, the
  pin green, the records.
- **The README's tool count** stays the driver schema's: "the twelve driver commands as tools" (`README.md:56`).
  `.github/scripts/test_capability_statement.py` holds it to `VERBS`, and that pin stays green.

### The capabilities statement (v030-02)
- **`docs/capabilities.md:288-291`** is restated against the 2026-10-25 cold-agent live run: six apps, twelve tools,
  66 calls, 0 wrong, with a screenshot and a lint called (`.andromeda/residuals.md:41`, absorbed:v030-02). Today it
  reads the 2026-10-15 run: one run, eleven tools, 43 calls, «It has not been re-fired since `lint` made the tools
  twelve». What crossed to the model provider on that run is stated as the security plan records the run's crossing:
  the six apps' screens (ids, names and values in snapshot texts, paged parts and diffs), Cells' contents, the
  `flight-type` select's options and value, a lint's findings and one screenshot. [verified at P3:
  `.andromeda/security-plan-detail/threat-model-summary.md:37`, `.andromeda/architecture-detail/occupied-resources.md:23`]
- **The statement's pin stays green.** `.github/scripts/test_capability_statement.py` holds its verb and cause names
  and its five counts to the driver schema's tables.
- Any other line of `docs/capabilities.md` the 2026-10-25 run's chunk made false is restated from its record. No other
  line names the 2026-10-15 run. One line does state as open what that chunk measured: `:224`, «A paged `part` read
  through the host's sink is unmeasured, owned by an open `.andromeda/residuals.md` line». It is restated to the
  measured reading (0 id and 0 name needles under both builds, seven_guis `host_log`'s fourth test), its residual line
  dropped. [premise-corrected: the sweep found one stale line beyond the run paragraph, measured at
  escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md F13 and residuals line 38 dropped; research.md §Sweeps]
- `:3` («current with Escher 0.2.0, the `build/escher-0.2.0` branch») and `:150` (its version, 0.2.0) stay. The
  statement describes 0.2.0's capabilities until 0.3.0 adds one, and the Epoch 6 «Capability statement» entry restates
  it for 0.3.0 (`escher-0.3.0/working-route.md:58`). Leaned per that entry.

### The records (v030-16)
- **Two verified ledger refs** (`.andromeda/residuals.md:42`, absorbed:v030-16) name the tests' current names. They are
  `escher-0.1.0/verification-matrix.json` v010-17 (`stand_snapshot_state::exactly_the_active_flight_mode_reads_pressed`,
  now `exactly_the_chosen_flight_type_reads_selected`) and `escher-0.2.0/verification-matrix.json` v020-05
  (`stand_lint::the_stand_at_boot_reads_no_finding`, now `the_stand_at_boot_reads_its_measured_findings`). The intent
  calls it «a factual correction with no word needed». Each new name stands in its test file. v010-17 holds a second
  stale name, `stand_act_states::a_flight_mode_click_returns_both_mode_buttons_with_their_pressed_reading`, now
  `a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading` (`stand_act_states.rs:80`), so
  three names are re-pointed in all. [premise-corrected: 14 names checked by `fn {name}(`, 3 stale, not 2;
  research.md §Sweeps] The only verb that re-points a verified cap's `ref` is `matrix.py refine --chunk {M}`, which
  keeps the status and refuses any `M` but the cap's own `chunk`; the contract binds that arm to the owning chunk's
  wrap. Whose step fires it is P4's fork F1. [verified at P3: `matrix.py:622-656`; verification-matrix-contract.md §Tool]
- v010-17's `acceptance` still describes the retired pressed pair (`flight-one-way` · `flight-return`). Only the `ref`
  is in this cap's letter, so the acceptance of a verified cap is not re-worded here; F1 asks it beside the refs.
- **The design-system stand rows** move out of `.andromeda/design-system.md`'s product surface into
  `.andromeda/architecture.md` and `.andromeda/layout-templates.md`. The rows are the accent row, the success row, the
  cards item and the stand's DejaVu Sans item, on the founder's «В 0.3.0, через инкубатор (Рекомендую)»
  (`escher-0.3.0/intent.md` §Also carried; incubator `deferred-from-0.2.0.md`, end: «a 0.3.0 wrap or door moves
  them»). The move is a master amendment. Phase never edits a spec source, so the plan names it as an expected wrap
  amendment, through design-system's door (an extracted master, 8 detail files). The rows stand at
  `.andromeda/design-system.md:16` and `.andromeda/design-system-detail/color-palette.md:27` (accent), `color-palette.md:73`
  (success) and `typography.md:31` (DejaVu Sans). "The cards item" names no single row: the seven_guis cards stand in
  `depth-strategy.md:10` (shadow), `spacing.md:19` (padding and gap) and `border-radius.md:12` (a row that also holds
  buttons, inputs, task cards, tags and the select). Two more stand rows sit beside them under the same founder rule:
  `color-palette.md:28` (the load page, repeating the accent's values) and the Border Progression's `flight-type` select
  row. Which of them move is P4's fork F2. [premise-corrected: the cards item is three rows, not one; research.md
  §Patterns detected]
- **The incubator ignore.** `.gitignore:39` already reads `escher-0.3.0-incubator/`, added by `143b60132` (2026-10-16),
  and it is the line that ignores the incubator: `git check-ignore -v` names `.gitignore:39` for the directory and a
  file in it, since `.gitignore` outranks `.git/info/exclude`. So the cap's acceptance for it already holds at HEAD.
  What is left is the local copy, the last two lines of `.git/info/exclude` («# escher 0.3.0 incubator (the founder,
  2026-10-16), local until a .gitignore line replaces it» and `escher-0.3.0-incubator/`). It is an untracked, local file
  that no commit carries, and the requirement says «in place of a local exclude», so implement removes those two lines.
  The witness is `git check-ignore -v` still naming `.gitignore:39`. Leaned per the requirement's own words.
  [verified at P3: research.md §Patterns detected]

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
