# Codebase Research — 2026-10-27-readme-and-records

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 11
- **Harness rules consulted:** none — no live leg in this chunk (the done gate is `ci-leg.sh fast` and `ci-leg.sh doc`; no scenario or harness run against a live process)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg (Setup 5a read HEAD green, 17/17)
- **External inputs:** `inputs#I1` — the founder's 0.3.0 scope, «обновить ридми под новую версию» (2026-10-26); `inputs#I2` — the founder's README word of 2026-10-16, «…чтоб на гите было видно текущее состояние… чтоб эшер там с большой буквы писался», with the overseer's reading (crate, binary and path names stay lowercase)
- **Code graph:** not queried — the modify set holds no code symbol (two markdown documents and two ledger files); no caller, signature or crate edge moves

## Files inspected
- `README.md` (full) — 85 lines. `## Status` holds "0.1.0 — done and verified" (six epochs, three facts) and "0.2.0 — planned, in five stages" (`README.md:66`), then "Later:" (`README.md:74`). The pinned phrase «the twelve driver commands as tools» and the `docs/capabilities.md` link both stand on `README.md:48`. Every prose mention reads «Escher» (10 occurrences; 0 lowercase prose hits, `grep -o '[^-`/a-z_]escher[^-`/_.0-9a-z]' README.md`).
- `docs/capabilities.md` (lines 1-16, 145-152, 218-226, 240-304) — 18,760 B, the five sections in order. `:3` "current with Escher 0.2.0, the `build/escher-0.2.0` branch"; `:150` "Its version, 0.2.0, is kept apart from the crates' shared workspace version, 0.3.0-beta.4"; `:224` "A paged `part` read through the host's sink is unmeasured, owned by an open `.andromeda/residuals.md` line"; `:248-249` the `a11y` leg's 14 · 84 · 0 · 0 on CI#40134371519; `:288-291` the 2026-10-15 run paragraph (one run, eleven tools, 43 calls, «It has not been re-fired since `lint` made the tools twelve»).
- `.github/scripts/test_capability_statement.py:40-110` — four `CapabilityStatementTest` cases: every `VERBS` verb and every `Cause::name` a code span of the statement, five counts in words (`{word} verbs` · `{word} refusal causes` · `{word} session errors` · `{word} node keys` · `{word} exit statuses`), and README holding `the {word} driver commands as tools`; `prose()` folds whitespace, so a phrase may wrap a line.
- `escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-verdict.json` (full) — `outcome` `passed`, `isolated` true on twelve `mcp__escher__*` tools, `tool_calls` 66 = `server_tool_calls` 66, `wrong_calls` 0, `wrong_call_bar_pct` 5, `lint_calls` 1, `screenshot_calls` 1, `advance_calls` 1, `part_reads` 2, `recorded_sessions` 6, `recording_kept` true, `tasks_met` the six apps, `client_version` 2.1.308, `cost_usd` 0.8244, `ts` 2026-10-25T16:55:07Z.
- `escher-0.3.0/intent.md:1-39, §Out of this version` — 0.2.0 closed verified 11/11 at `5e15b78d6`; the cold-agent bar run drove six apps in 66 calls with 0 wrong, for $0.82, under the founder's 5 % bar; the Epoch 5 audit's version-end mutation (blitz-dom 53 of 53 caught; seven_guis unscored, 12 of 15 unviable; wpt 0 of 5 caught, all in upstream's `attr_test.rs`). §Out of this version: OTel export (L8), MCP revision `2026-07-28` (L13), the platform adapter and its AT witness (L14), Cells' selected-cell state (L24), the advisories.
- `escher-0.3.0/working-route.md` (`### Epoch` headers, lines 10-62) — six epochs: Foundation · Engine debts · The UI graph · Determinism and exploration · Errors and explanation · Polish & ship.
- `escher-0.2.0/working-route.md` (`### Epoch` headers and stamped lines) — five epochs, 21 stamped entries; `matrix.py coverage --dir escher-0.2.0`: verified 11/11, done-test YES.
- `escher-0.1.0/verification-matrix.json` v010-17 and `escher-0.2.0/verification-matrix.json` v020-05 (`matrix.py show --id`) — both `verified`; v010-17 chunk `2026-10-10-snapshot-states`, v020-05 chunk `2026-10-19-visual-lint`.
- `andromeda-tools/scripts/matrix.py:622-656` (`cmd_refine`) — the `--chunk` arm re-words `acceptance` · `ref` · `method` with status kept, refusing only when `--chunk` differs from the cap's own `chunk`; it appends a dated note. `verification-matrix-contract.md` §Tool: "`--chunk M` is the owning chunk's wrap (P7.3)"; `cmd_implement` refuses a verified cap ("no verified → implemented edge").
- `.andromeda/design-system-detail/{color-palette,typography,depth-strategy,spacing,border-radius}.md` (grep `card`, the cascade's carried lines) — the four named rows and their neighbours (below).
- `.gitignore:33-39`, `.git/info/exclude` (full) — the ignore line and the local copy.
- `.andromeda/playbook.md` (every `- pattern` line, the three `verdict: escalate` entries) — boundary widening, provisional discharge, boundary widening by upstream merge, engine behaviour change; none holds for this chunk's forks.

## Graph impact
- not queried — no code symbol in the modify set (see Scope)

## Patterns detected
- **The README's Status shape** (`README.md:56-74`; arch §Project Intent → Front page, `.andromeda/architecture-detail/project-intent.md:21`): a done version lists its stages, one line per epoch of its working route, then its recorded facts in one paragraph; a planned version lists one line per epoch. 0.2.0 done takes the five-epoch list in the same form, and 0.3.0 the six epochs.
- **Every 0.2.0 capability the README may name already stands in the statement** (`docs/capabilities.md` §What it can do): the visual lint, a session recorded as a replayable check, a large screen read in parts, a `select` driven by id, the load wait, the virtual clock. The README names each in a phrase and points to the statement for the limits, as `README.md:48` does today.
- **The run's figures are the verdict's** (`live-verdict.json`): every number the README and the statement write is a verdict field — counts, booleans and closed words only.
- **The incubator is already ignored by the repository's file** (`git check-ignore -v escher-0.3.0-incubator/ escher-0.3.0-incubator/intent.md` → `.gitignore:39:escher-0.3.0-incubator/` for both, exit 0, at HEAD `316c6d25b`): `.gitignore` takes precedence over `.git/info/exclude`, so the line committed by `143b60132` (2026-10-16) is what ignores it now. The local copy, the last two lines of `.git/info/exclude`, is redundant. It is no tracked file and no commit carries it.
- **The design-system stand rows where they stand** (design-system is an extracted master: `detail.py check` → `design-system: extracted · 24 pointers · 8 files`; the move goes through its door's tooling at the wrap):
  - accent: the body sentence `.andromeda/design-system.md:16` and the row `.andromeda/design-system-detail/color-palette.md:27`;
  - success: `.andromeda/design-system-detail/color-palette.md:73`;
  - DejaVu Sans: `.andromeda/design-system-detail/typography.md:31`;
  - cards: no single row. The seven_guis cards stand in three: `.andromeda/design-system-detail/depth-strategy.md:10` (the cards' shadow), `.andromeda/design-system-detail/spacing.md:19` (the cards' padding and gap, with home's padding) and `.andromeda/design-system-detail/border-radius.md:12` (the "seven_guis (untokenized)" row: cards 8px with buttons, inputs, task cards, tags and the `flight-type` select);
  - neighbours under the same founder rule (stand rows not named by the cap): `.andromeda/design-system-detail/color-palette.md:28` (the load page, repeating the accent's values) and the Border Progression's `flight-type` select field row (2026-10-25-cold-agent-bar).
- **The GitHub front page renders the default branch, `build/escher-0.1.0`** (read at P4: `gh repo view
  Turbolet85/escher --json defaultBranchRef` → `build/escher-0.1.0`; `git show origin/build/escher-0.1.0:README.md`
  at `1a34eb2bb`, 2026-10-16, holds no `## Status`). The README this chunk writes on `build/escher-0.3.0` reaches the
  page only if the page comes to render this branch, which is F3 (inputs#I4), the chunk's gated criterion.
- **Masters cite the README by line**: `README.md:48` in `.andromeda/architecture-detail/project-intent.md` (×2) and `.andromeda/architecture-detail/standard-contracts.md` (×1) (`git grep -n -o 'README\.md:[0-9-]*' -- .andromeda/`). A rewrite that moves the pinned phrase's line moves those citations, which the wrap's citation sweep re-points.

## Conventions to follow
- **The pin's phrases**: the statement keeps every verb and cause as a code span and the five counts in words, and the README keeps «the twelve driver commands as tools»; the counts are the schema's, never hand-moved (`.github/scripts/test_capability_statement.py:75-104`).
- **The name**: «Escher» in prose and title; `escher-session`, `escher-driver`, `escher-0.3.0/`, URLs and the e-mail as spelled (arch §Project Intent → Front page; inputs#I2).
- **A measured claim names its witness** (`docs/capabilities.md:13-15`): a check by file name, a CI run or the chunk report that measured it; a limit says pinned, asserted by no check, or unmeasured with its residuals line.
- **Run facts are counts, booleans and closed words only**, cited from the verdict and the report, never from `target/cold-agent/` (security-plan §Logging & Monitoring; obs-plan §3).
- **Committed prose holds no host path, key, token or e-mail beyond the kept contact line**: the recorded hygiene search `/home/|api[_-]?key|authorization:|bearer ` plus the two e-mail domains, case-insensitive (security-plan §Secret Management).

## New files to create
- none

## Files to modify
- `README.md` — §Status to 0.1.0 done · 0.2.0 done and verified with what it added and its cold-agent result · 0.3.0 planned in six stages; "Later:" restated past 0.3.0; "What Escher is" and the driver paragraph name 0.2.0's additions; the pinned phrase kept
- `docs/capabilities.md` — `:288-291` restated to the 2026-10-25 run; `:224` restated to the measured host-sink part read
- `escher-0.1.0/verification-matrix.json` — v010-17's `ref`: two stale names re-pointed, through `matrix.py` (P4 fork F1 decides the step)
- `escher-0.2.0/verification-matrix.json` — v020-05's `ref`: one stale name re-pointed, through `matrix.py` (P4 fork F1 decides the step)

## Sweeps
- `2026-10-15` over `docs/capabilities.md`: 1 hit (`:288`) · 1 changed · 0 no-change (`grep -n`)
- `unmeasured` over `docs/capabilities.md`: 4 hits (`:14`, `:192`, `:224`, `:256`) · 1 changed (`:224`, measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md F13, its residual line dropped) · 3 no-change (`:14` the page's own rule; `:192` the select's contrast pair, no record measures it; `:256` what an assistive technology reads, unmeasured per a11y-plan §1)
- the two refs' test names, `fn {name}(` in each named file: 14 names · 3 stale · 11 standing — v010-17 `stand_snapshot_state::exactly_the_active_flight_mode_reads_pressed` → `exactly_the_chosen_flight_type_reads_selected` (`stand_snapshot_state.rs:368`) and `stand_act_states::a_flight_mode_click_returns_both_mode_buttons_with_their_pressed_reading` → `a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading` (`stand_act_states.rs:80`); v020-05 `stand_lint::the_stand_at_boot_reads_no_finding` → `the_stand_at_boot_reads_its_measured_findings` (`stand_lint.rs:288`); the rest, `cli_commands::lint_answers_one_line_of_findings` included, stand (a `grep -c "fn {name}("` loop over the 14)
- `README.md:` line citations over `.andromeda/` masters: 3 hits, all `README.md:48` · 0 changed by implement · 3 no-change at implement (the wrap's citation sweep re-points them if the pinned line moves)
- lowercase `escher` in `README.md` prose: 0 hits (see Files inspected)

## Open questions
- Who writes the two verified refs: no verb but `matrix.py refine --chunk {the owning marker}` re-points a verified cap's `ref`, and the contract binds that arm to the owning chunk's wrap; a dated `note` alone leaves the acceptance «the two ledger references name the tests' current names» unmet → blocks: plan-decision (P4 F1)
- What "the cards item" moves, and whether the stand's neighbouring rows (the load page colour row, the `flight-type` select border row, the whole seven_guis radius row) move under the founder's rule with the four named → blocks: plan-decision (P4 F2)
