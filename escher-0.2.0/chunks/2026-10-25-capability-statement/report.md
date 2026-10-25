# Report — 2026-10-25-capability-statement

**Chunk:** what escher 0.2.0 can do, its stated limits, a11y reach and what an MCP client carries onward, stated where Viola's planning reads it (v020-10)
**Date:** 2026-10-25
**Commits:** since `last_wrap` (2026-10-25T01:36:47Z): `e2eb01b44 chore(setup-project): upgrade escher — U56` (setup's, before the take-up — not this chunk's) · `e7e71221d chore(2026-10-25-capability-statement): operator pre-CI commit, for the run this chunk's verdict reads` (the operator pass; `git log --format='%h %s' e2eb01b44..HEAD`: that one commit). Base: `e2eb01b44`, the parent of the oldest pre-CI commit.

## Changes (structured — detectors read this)
- **Files:** new `docs/capabilities.md` (304 lines, «What Escher can do»: five sections `## What it can do` · `## Stated limits` · `## Accessibility reach` · `## What an MCP client carries onward` · `## Where each claim is proven`); new `.github/scripts/test_capability_statement.py` (108 lines); `README.md:48` (one line replaced: «the eleven driver commands as tools» → «the twelve driver commands as tools», and one sentence on the same line linking `docs/capabilities.md` as the statement of what Escher can do and its limits; `git diff --numstat e2eb01b44 -- README.md` `1 1`). Outside this repository: `../viola/viola-0.2.0-incubator/research/escher-capabilities.md`, untracked and ignored in Viola (inputs#I6). Chunk evidence: `evidence/{counts,pin-controls,viola-pointer,operator-pass,operator-confirm}.md`. Coordinates of new text: the `## New text, by line` section below.
- **Symbols / APIs:** no Rust symbol, verb, argument, field, cause, session error, exit, carrier, log field or span added or changed (the guard entry: `guard: clean`, below). The pin module `test_capability_statement.py` holds `NUMBER_WORDS` (one … twenty), helpers `read`, `prose` (whitespace folded, a phrase may wrap a markdown line), `number_word`, `verb_names` (the `VERBS` identifiers in `packages/escher-driver/src/schema.rs`, lower-cased), `cause_names` (`Cause::name`'s match arms in `refusal.rs`), `table_length` (the `N` of `SESSION_ERRORS: [ErrorSpec; N]` in `error.rs`, `NODE_FIELDS: [NodeFieldSpec; N]` and `EXIT_STATUSES: [ExitStatus; N]` in `schema.rs`), and `CapabilityStatementTest`'s four cases: every verb a code span in the statement · every cause a code span · `{word} verbs` · `{word} refusal causes` · `{word} session errors` · `{word} node keys` · `{word} exit statuses` stated · README.md holds «the {word} driver commands as tools». It reads by path from the repository root, writes nothing, reads no env var; no helper carries a `test_` prefix.
- **Crates / modules:** none.
- **Dependencies:** none.
- **Schema / config:** none.
- **Spec-master edits:** none.
- **Retired:** none.
- **Counts / qualifiers moved:**
  - README.md's MCP sentence: the driver commands an MCP client gets as tools, eleven → twelve (`VERBS` holds twelve since 2026-10-19-visual-lint), and README now links `docs/capabilities.md` — pattern `readme12`: 1 standing · `.andromeda/architecture-detail/project-intent.md:21` («the page saying eleven since 2026-10-13-upstream-sync-polish-and-ship») · changed.
  - The `ci-scripts` leg: 100 → 104 tests, seven → eight test files (`test_capability_statement.py`'s 4; the fast leg's `ci-scripts` part read `Ran 104 tests` · `OK`, `evidence/counts.md`) — pattern `ciscripts`: 3 standing rows · `.andromeda/test-plan-detail/test-scope-summary.md:8` («100 tests in seven test files») changed · `.andromeda/test-plan-detail/unit-test-strategy.md:29` ×2: @1002/@1017 («the CI-scripts leg runs 100 tests over seven test files», the per-file list) changed, @1303 («as measured at 2026-10-15-cold-agent-test (`ci-leg.sh ci-scripts`: `Ran 100 tests`»…) no-change (a dated measurement) · `.andromeda/architecture-detail/standard-contracts.md:261` («which discovers seven test files since 2026-10-14-quality-gates») changed.
  - `docs/` now holds escher's own `capabilities.md` beside upstream's three files — pattern `docsdir`: control unfired (`cascade.py sweep --pre-pass`: «control: title, docsdir never fired over the seven masters' pre-pass text, bodies and sets (e7e71221)») — no master names a file under `docs/`; the arch Directory structure names no `docs/` (research.md, Files inspected).
  - The workspace tally unchanged: `lines 206 passed 1105 failed 0 ignored 12` (entry 10, `evidence/counts.md`) — no Rust test added.
  - Sweep reach: 6 patterns · 8 moved facts, 8 with a pattern (the report's own count: `readme12`, `ciscripts` ×2, `docsdir`, `mcpcarry`, `causes9`, `atowner`, the README link riding `readme12`) · owner pattern control unfired · by line 0 lines · 0 offsets.
- **Dev-tool versions:** none.
- **Harness / gate surface:** `.github/scripts/test_capability_statement.py` joins the `ci-scripts` leg (`python3 -m unittest discover -s .github/scripts`, `.github/scripts/ci-leg.sh:27`) — no leg, job, workflow or `ci-leg.sh` line changed; CI's «Test CI scripts» job runs it (CI#40184674449 green).
- **Cross-project / external claims:**
  - Viola (`../viola`, branch `build/viola-0.1.0`, commit `e4aeb25b` as snapped): the pointer `viola-0.2.0-incubator/research/escher-capabilities.md` — a title, one sentence, then `repository: Turbolet85/escher` · `path: docs/capabilities.md` · `commit: e7e71221dfdcd29c6df603d222104855f43c07f4` (entry 15 wrote the commit line); ignored by Viola's `.gitignore:6` (`git -C ../viola check-ignore -v`: `.gitignore:6:viola-0.2.0-incubator/`), so no Viola commit carries it.
  - CI: CI#40184674449 on `e7e71221dfdc` (the pre-CI commit) — `verdict: green · checks 17/17 · wall 1191 s` (ci.py, operator entry 14, `evidence/operator-pass.md`). Per-job tallies not read.
  - Inputs (`inputs.py verify`, this wrap's run dir): I1 · ../viola:viola-0.2.0-incubator/README.md · copy untracked@1862fcfb · unchanged (cited) · I2 · the founder's F1/F2 relayed by the operator · copy message · n/a (cited) · I3 · inputs#I3, the operator pass directive (entries 12-15 by hand, the pre-CI commit's subject, stop after 15 or on any red, operator-confirm.md the operator's own) · copy message · n/a · I4 · inputs#I4, the operator's option 1 after entry 12 read red (respell `p5-controls.md:15`'s cell in words, nothing else in that run dir, then continue) · copy message · n/a · I5 · inputs#I5, step 6 done: the founder's «Подтверждаю (Рекомендую)», operator-confirm.md written by the operator; run entry 16, then the v020-10 ref · copy message · n/a · I6 · inputs#I6, the Viola pointer as entry 15 left it · copy untracked@e4aeb25b · unchanged. `inputs: 6 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 4 · uncited 4 · unparsed 0` (fired before this report; I3-I6 are cited here). No DRIFT.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - «the cause set (nine)» at `.andromeda/security-plan-detail/error-handling.md:9` and «the nine causes and their names unchanged» at `.andromeda/architecture-detail/standard-contracts.md:149` (the same sentence there opening «the cause set — eight, in its order») — `CAUSES: [Cause; 8]` (`packages/escher-driver/src/refusal.rs:35`), the pin's `cause_names()` reading eight — pattern `causes9`: 2 standing · both changed.
  - The windowed assistive-technology witness owned by a residuals line «(target: next)» — the line reads `re-carried:0.2.0` (`.andromeda/residuals.md:14`, «the stand's platform accessibility adapter and its windowed witness»), and `escher-0.2.0/requirements.md:42-43` owes it «at an active assistive technology, which this version does not reach» — pattern `atowner`: 12 standing rows · changed (each names the owner «(target: next)»): `.andromeda/a11y-plan.md:9` · `:27` · `:35` · `.andromeda/a11y-plan-detail/a11y-scope-summary.md:17` · `.andromeda/a11y-plan-detail/a11y-strategy.md:9` (its owner clause «and is owned by a `.andromeda/residuals.md` line (target: next)») · `.andromeda/a11y-plan-detail/a11y-strategy.md:21`; no-change (a true «still owed», no owner disposition named): `.andromeda/architecture.md:139` · `.andromeda/architecture-detail/cross-cutting-patterns.md:35` · `.andromeda/architecture-detail/standard-contracts.md:28` · `.andromeda/security-plan-detail/input-validation.md:193` · `:195` ×2 · `:210` («owned by its `.andromeda/residuals.md` line», no target). Three sites worded otherwise, outside the pattern's reach, found by `git grep -n -F '(target: next)'` over the arch, security and test masters filtered to a clause naming an adapter, a windowed check or an assistive technology: `.andromeda/architecture-detail/standard-contracts.md:27` @1413 · `.andromeda/security-plan-detail/input-validation.md:217` @1129 · `.andromeda/test-plan.md:48` @458 — each changed. The leaves hold 5 `atowner` rows (`.claude/rules/a11y.md:25`, `:27`, `.claude/docs/a11y-summary.md:21`, `:26`, `.claude/docs/services/blitz-shell.md:18`) for P2's cascade.
  - The MCP stdio row's carried list (`.andromeda/security-plan-detail/threat-model-summary.md:34`, under the `boundary-mcp-stdio` label at `:30`) names ids, accessible names, control values and the snapshot text, the diffs only inside its cold-agent sentence, and no lint finding — while a result carries snapshot text and diffs and, since 2026-10-19-visual-lint, a lint's findings (CLAUDE.md escher-driver row; `docs/capabilities.md` «What an MCP client carries onward») — pattern `mcpcarry`: 2 standing rows, both labels (`.andromeda/security-plan.md:423`, the category index comment · `threat-model-summary.md:30`, the detail comment) · the site is `:34`, read under that label · changed.
  - Not this chunk's fact, carried for Validate: the generic owner phrase stands once — `.andromeda/security-plan-detail/input-validation.md:197` «no driver, CLI or MCP command yet (whether it becomes one is owned by a route entry)», of the actionable-key check as a driver command, whose `.andromeda/residuals.md` line reads `2026-10-12-self-description · re-carried:0.2.0` — no-change here; `docs/capabilities.md` states only that the check «has no driver verb».
- **Expected amendments (from plan):**
  - security-plan §Threat Model Summary — `boundary-mcp-stdio`'s carried list names a lint's findings and the diffs → carried: **Spec claims disproved** (`mcpcarry`, 2 label rows; the site `threat-model-summary.md:34`).
  - security-plan §Error Handling — «the cause set (nine)» → carried: **Spec claims disproved** (`causes9`, 2 standing: `error-handling.md:9` and `architecture-detail/standard-contracts.md:149`).
  - a11y-plan §1 / §2, test-plan §1, architecture §Standard Contracts → Mutation, query and CSSOM, security-plan §Input Validation — the AT witness's residuals line «(target: next)» → `re-carried:0.2.0` → carried: **Spec claims disproved** (`atowner`, 12 standing, 6 changed + 3 sites outside its reach changed; a11y-plan 3 body + 3 category rows, test-plan.md:48, standard-contracts.md:27, input-validation.md:217).
  - architecture §Project Intent → Front page — README says twelve and links `docs/capabilities.md` → carried: **Counts / qualifiers moved** (`readme12`, 1 standing, `project-intent.md:21`).
  - architecture §Infrastructure Patterns → Directory structure — `docs/` holds escher's `capabilities.md` beside upstream's three → carried: **Counts / qualifiers moved** (`docsdir`, control unfired — no master names `docs/`; the amendment adds the row).
  - test-plan §4 — `test_capability_statement.py` joins the CI scripts → carried: **Counts / qualifiers moved** and **Harness / gate surface** (`ciscripts`, 3 standing rows: test-scope-summary.md:8, unit-test-strategy.md:29, standard-contracts.md:261).
  - `matrix#v020-10`: no master site — a ledger entry, written at the operator pass (Outcome).
- **Coverage of new surfaces:**
  - `docs/capabilities.md` (a document, no runtime surface) → validation n/a · instrumentation n/a · PII n/a (secret search entry: 0 hits over it and the pin) · tests unit (`test_capability_statement.py`, 4) · a11y n/a · tokens n/a.
  - `.github/scripts/test_capability_statement.py` (CI-script test, no runtime surface) → validation n/a · instrumentation n/a · PII n/a · tests itself, controlled red-before-green (`evidence/pin-controls.md`) · a11y n/a · tokens n/a.

## Deviations from intent
- **The pin's count case reads five tables, not three** (plan step 3 lists the verb, cause and session-error counts): it also reads `NODE_FIELDS` (thirteen node keys) and `EXIT_STATUSES` (four exit statuses) from `schema.rs`, since the (arch) acceptance names those counts as «read from the schema's tables and none hand-copied». Same case, still four tests (`Ran 4 tests`).
- **The pin folds the markdown's whitespace and asserts `phrase in text`** with the phrase as its message: its first run read «thirteen node keys» red, the phrase wrapped across two source lines; `assertIn` printed the whole statement on a red.
- **The statement's count phrases** read `{word} verbs` · `{word} refusal causes` · `{word} session errors` · `{word} node keys` · `{word} exit statuses` — the forms the pin reads.
- **A file of the phase run dir respelled at the operator pass:** `.andromeda/runs/2026-10-25T02-58-45-phase/p5-controls.md:15`'s must-fail cell named the planted fake home path the P5 control put into a scratch copy of `docs/capabilities.md`; operator entry 12 (`gate.py hygiene`) read it `P1 … ×1 · home` and refused; on the operator's word (inputs#I4) the cell names it in words («a home-directory path under the made-up user someone») and nothing else in that run dir was touched; entry 12 re-read `hygiene: clean`. A run dir is outside the scope read (`excluded`).
- scope record: none — `gate.py scope` (this wrap): `scope: clean — changed 3 · listed 3 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 496`; `scope-record.md` absent.

## Decisions & corrections
- A phase P5 control record spelled a planted host-shaped path verbatim (`p5-controls.md:15`, a home path under the made-up user `someone`) and the operator pass's hygiene read refused the commit on it (`P1 .andromeda/runs/2026-10-25T02-58-45-phase/p5-controls.md:15 ×1 · home`); the operator chose option 1, the cell respelled in words, touching nothing else in that run dir, «the cascade files there stay intact for overseer1» (inputs#I4). Evidence: `evidence/operator-pass.md`, entry 12.
- A markdown-reading pin must match across a line wrap: the first green run of `test_capability_statement.py` read «thirteen node keys» red because the phrase wrapped two source lines of `docs/capabilities.md` (`evidence/pin-controls.md`); the pin folds whitespace since.
- The first draft of the statement named `ArgKind::Name` a session label; reading `schema.rs:401-402` showed it is `start`'s `app` argument (and `ArgKind::Flag` is `press`'s `shift`) — corrected before any gate ran.
- A `manual` cap's ref is not written at /implement when its acceptance needs the pushed commit and the operator's confirmation: `v020-10`'s ref was withheld in /implement's outcome (`.andromeda/runs/2026-10-25T10-05-33-implement/outcome-2026-10-25-capability-statement.md`, Matrix) and written after operator entry 16 on the operator's direction (inputs#I5): `matrix.py implement` → `v020-10 · status planned → implemented`.
- The operator pass ran by hand in plan order on the operator's directive (inputs#I3): `evidence/operator-confirm.md` is the operator's own (step 6) and this session wrote no part of it; the founder confirmed by dialog, «Подтверждаю (Рекомендую)» (inputs#I5).
- Guard refusals: the Bash guard refused a `cd` twice — into the chunk's `inputs/` at /implement Setup and into `.andromeda/` at this P1 («moves the session cwd for every later call») — each re-issued once with absolute paths and ran.
- Site-sweep hazard: the chunk title «Capability statement» and the `docs/` file names fire nowhere in the masters (`cascade.py sweep --pre-pass` exit 3, unfired controls), and the AT-owner fact is worded three ways the `atowner` pattern does not reach (`standard-contracts.md:27`, `input-validation.md:217`, `test-plan.md:48`, found by `(target: next)` filtered to a clause naming an adapter, a windowed check or an assistive technology).

## Outcome
- (arch) `docs/capabilities.md` stands at the pushed commit `e7e71221d` with its five sections in order; it names the twelve verbs in three levels, the eight causes, fifteen session errors, thirteen node keys and four exit statuses, each count pinned to the schema's tables by the pin — **met** (entry 1 `Ran 4 tests` · `OK`; CI#40184674449 «Test CI scripts» green).
- (arch) README.md says «the twelve driver commands as tools», no line reads «the eleven driver commands», it links `docs/capabilities.md`, one line changed — **met** (entries 2, 3, 5).
- (security) The MCP section names the boundary as the spawning client holding stdio with no authentication, lists the carried set (ids, names, masked values, snapshot text and diffs, a shot as an `image` block painted masked, a lint's findings), names no description, and states the client's onward use is the client's — **met** (`docs/capabilities.md` «What an MCP client carries onward»).
- (security) No authentication claimed beyond the `0700` directory and `0600` socket and holding the stdio; no credential, token, auth header or host home path in the committed text — **met** (entry 4: 0).
- (security) No carrier, verb, field, exit, log field or span added — **met** (entry 6 `guard: clean`; the diff adds markdown, one README line and a python test).
- (design) Carriers described as one JSON line or JSON-RPC 2.0 lines on stdout, no colour or ESC byte, exit `0` · `1` · `2` · `3`; contrast `#3a5ce5` 5.47 · 6.83 · 8.27 on the accent controls — **met**.
- (layouts) Stand reach headless at 800 × 600, scale 1.0, Light, DejaVu Sans; windowed end-to-end runs unmeasured; lint 0 findings at boot per lean task and `crud-person-14` `clipped` inside `crud-list` after twelve Creates — **met**.
- (tests) Every «pinned» limit names its check; the select's limits «asserted by no check»; the boolean model's two limits «pinned at unit and engine level only»; the cross-OS shot limit cites CI#39225375577; the 2026-10-15 cold-agent run stated at eleven tools, not re-fired since `lint` — **met**.
- (tests) `ci-leg.sh fast` and `ci-leg.sh doc` exit 0 on the tree with the statement, the README edit and the pin; `ci.py conclusion` reads `verdict: green` on the pushed sha — **met** (entries 9, 11, 13; entry 14 CI#40184674449).
- (obs) The two spans `command` and `tool_call` with closed-word, bool and count fields, stderr only, no driver line at the default `warn`; the scrub's limits named (the raw panic message, `log.file` on a bridged `js_console` record, the dropped third-party WARN or ERROR, the unscrubbed upstream apps and WPT runner) — **met**.
- (a11y) Tier 0 SC 2.1.1 · 1.4.3 · 2.4.3 and no other criterion; the `a11y` leg's fourteen files at 14 · 84 · 0 · 0 on CI#40134371519, no required status check; the AT reading unmeasured, owned by its residuals line `re-carried:0.2.0`, which this version does not reach; the focus fixup, `multiple` · `optgroup`, `aria-activedescendant` and the native checked state among the limits — **met**.
- (v020-10) operator-confirmed (`manual`): the statement at `e7e71221d`, its pin green, README linking it, Viola's pointer naming `docs/capabilities.md` and that commit, the founder's confirmation in `evidence/operator-confirm.md` (`confirmed: e7e71221dfdcd29c6df603d222104855f43c07f4 · 2026-10-25 · the founder …`) — **met**; `verification-matrix.json#v020-10` `implemented`, its ref `manual: …/evidence/operator-confirm.md …` (written after entry 16).
- Gates (the outcome basis: the operator pass's final state, HEAD `e7e71221d`; entries 1-11 from /implement's run `.andromeda/runs/2026-10-25T10-05-33-implement/`, its last firing of each on the tree the commit carries):
  - `python3 -m unittest discover -s .github/scripts -p test_capability_statement.py 2>&1` — green (exit 0 · `Ran 4 tests` · last line `OK`)
  - `grep -c "the eleven driver commands" README.md` — green (exit 1 · last line 0)
  - `grep -c "docs/capabilities.md" README.md` — green (exit 0, 1)
  - `cat docs/capabilities.md .github/scripts/test_capability_statement.py | grep -c -i -E '/home/|…'` — green (exit 1 · last line 0)
  - `git diff --numstat e2eb01b4… -- README.md | awk …` — green (exit 0 · last line `1 1`)
  - `gate.py guard … --allow '(README\.md|docs/capabilities\.md|\.github/scripts/test_capability_statement\.py)' -- …` — green (`guard: clean`; `guard: allowed 3 — .github/scripts/test_capability_statement.py, README.md, docs/capabilities.md · companions excluded 0 · changed 3 · excluded 0`)
  - `grep -c -E '^(repository|path|commit): ' ../viola/…/escher-capabilities.md` — green (exit 0 · last line 3)
  - `git -C ../viola check-ignore -q viola-0.2.0-incubator/research/escher-capabilities.md` — green (exit 0)
  - `bash .github/scripts/ci-leg.sh fast` — green (exit 0; artifact `target/ci-logs/test.log` written inside the firing)
  - `grep -E '^test result:' target/ci-logs/test.log | awk …` — green (`lines 206 passed 1105 failed 0 ignored 12`)
  - `bash .github/scripts/ci-leg.sh doc` — green (exit 0)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (leg operator) — red · `refused 1 files`, then green after the respell (inputs#I4): `hygiene: clean — read 486 …` (`evidence/operator-pass.md`)
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.2.0` (leg operator) — green (exit 0 · 103.15 s · `history moved: refs/remotes/origin/build/escher-0.2.0 e2eb01b4→e7e71221`, the intended push)
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` (leg operator) — green (`e7e71221dfdc verdict: green · checks 17/17 · wall 1191 s · runs CI#40184674449 completed/success`)
  - `sha=$(git rev-parse HEAD) && sed -i … escher-capabilities.md && grep -c "^commit: $sha$" …` (leg operator) — green (exit 0 · last line 1 · `history: unmoved`)
  - `grep -c -E "^confirmed: " …/evidence/operator-confirm.md` (leg operator) — green (exit 0 · last line 1 · `history: unmoved`)
  - No `defer`, no source-delta deferral; smoke skipped — no boot-path or UI-surface change and no smoke or self-verify entry.
- Watches: none.
- Outcome basis: the operator pass ran (pre-CI commit `e7e71221d`, the only commit from the base); the final HEAD's CI run CI#40184674449 is recorded in `evidence/operator-pass.md`; /implement's P4 report is this window's own run and its persisted `outcome-2026-10-25-capability-statement.md` (`.andromeda/runs/2026-10-25T10-05-33-implement/`, its Matrix section rewritten after entry 16); post-implement artifacts: `evidence/operator-pass.md`, `evidence/operator-confirm.md` (the operator's), `evidence/viola-pointer.md`'s commit line, the respelled `p5-controls.md:15`, `verification-matrix.json`.
- Process hygiene: /implement's census `census: none running` (cargo, rustc, escher-session, seven_guis_native — the fast and doc legs' processes terminated); re-measured at this wrap (`gate.py census`, the same names, this run dir): `census: none running`.
## Addendum — resumed 2026-10-25
- inputs#I7 · message · copy message · n/a — the operator's word for this resume's P2, the founder's order relayed by overseer1: «forced sample» (`detail.py sample --side wrap --force`), every P2 batch one Workflow, the run dir kept intact
  - no fact above rests on it
- inputs.py verify, re-fired: inputs: 7 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 5 · uncited 1 · unparsed 0; moved since the report: entries 6 → 7 (I7), uncited 4 → 1 (I3-I6 cited by the report; I7, uncited at that read, cited by this addendum)
## New text, by line
Generated by `cites.py added` (cites v1.5); pasted by `splice.py`. No line of this section is typed or edited.
The diff: e2eb01b4 (the parent of the oldest pre-CI commit e7e71221) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/scripts/test_capability_statement.py — new file · 108 line(s)
- 13-34 «NUMBER_WORDS = {»
- 37-38 «def read(path):»
- 41-43 «def prose(path):»
- 46-49 «def number_word(count):»
  - 47-48 «if count not in NUMBER_WORDS:»
- 52-57 «def verb_names():»
  - 55-56 «if body is None:»
- 60-65 «def cause_names():»
  - 63-64 «if body is None:»
- 68-72 «def table_length(path, pattern):»
  - 70-71 «if found is None:»
- 75-104 «class CapabilityStatementTest(unittest.TestCase):»
  - 76-81 «def test_every_verb_is_named_as_a_code_span(self):»
  - 83-88 «def test_every_cause_is_named_as_a_code_span(self):»
  - 90-100 «def test_the_table_counts_are_stated_in_words(self):»
  - 102-104 «def test_the_readme_counts_the_mcp_tools(self):»
- 107-108 «if __name__ == "__main__":»
### README.md — added 1 line(s) in 1 range(s)
added: 48
### docs/capabilities.md — new file · 304 line(s)
- 25-26 «- **over a local unix-domain socket**: `start` · `attach` · `stop` · `call`, wire `v3`, the host ending at `s…»
- 27-28 «- **by the `escher-session` command line**: one command per run, one line of JSON on stdout, no colour and no…»
- 29-30 «- **by the `escher-session mcp` role**: an MCP server on its own stdin and stdout, newline-delimited JSON-RPC…»
- 127-128 «- **a range input model**: arrows, `Home` and `End` step it, a pointerdown sets it, each change fires one `…»
- 132-133 «- **`inert`** is honoured: an inert subtree is skipped by Tab, passed through by a hit and left out of the»
- 161-163 «- **`covered` beside a box clipped on one axis.** A box clipped on one axis stops a hit at its whole paddin…»
- 164-165 «- **`covered` on a box with no width or height.** Such a box is hit at its centre on the element that holds…»
- 166-168 «- **`off-screen` inside a transformed scrolling box.** A scrolling box under a transform is read where it s…»
- 172-174 «- **The boolean model's two limits.** A mixed toggle reads unpressed, and a grammar or spelling error reads…»
- 175-177 «- **`text_extent`, which the lint reads.** RTL and vertical text read as left-to-right horizontal lines, an…»
- 178-179 «- **HTML's focus fixup is not built.** A node that holds focus when it becomes inert keeps focus until focu…»
- 180-181 «- **A shot's PNG bytes.** They repeat byte for byte on one host and differ between linux x86_64 and macOS a…»
- 185-187 «- **Unpaged diffs and findings.** An acting verb's diff and a lint's findings are not paged: on a large scr…»
- 188-192 «- **The `select` model's limits.** A `multiple` select and an `optgroup`'s options are not modelled, no pop…»
- 193-194 «- **The bounded record of ids.** The session keeps the last 4,096 ids its screens read, to tell `stale` from»
- 196-197 «- **The Dioxus bridge.** Focus and blur events are not queued. IME events are not handled. Animation and tr…»
- 198-200 «- **Loading pages.** No verb takes a URL. The engine reads `file:` URLs with no path restriction and has no…»
- 204-205 «- **Unix only.** The socket host, its client and session recording are unix only and answer `Unsupported` e…»
- 206-207 «- **The session state directory.** It is named by the caller, owner-only, holds the socket alone, and is re…»
- 208-209 «- **WebAssembly.** The wasm builds of the examples are browser-target builds through Trunk. The driver does…»
- 238-239 «- The engine's Web Platform Tests conformance. One report-only reading exists: `css/cssom-view` moved from …»
- 256-261 «- **What an assistive technology reads is unmeasured.** No stand binary builds a platform accessibility ada…»
- 265-266 «- A `role` attribute reaches the tree through the engine's one role-name mapping. The role, name and state …»
- 300-301 «- **`stand-repeat`**: the 48 `stand_*` files, ten named blitz-tests targets and seven_guis' eight process c…»
