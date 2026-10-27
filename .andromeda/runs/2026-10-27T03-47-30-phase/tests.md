# tests extract

## Relevance
partial — no test, fixture or CI leg is built; the chunk's two edited documents (`README.md`, `docs/capabilities.md`) are read by the CI-script pin `test_capability_statement.py`, the capabilities restatement must equal the cold-agent live run's recorded proof, and the two re-pointed ledger refs must name test functions as they stand.

## Constraints
- test-plan §4 Capability statement requires `README.md` to hold «the {word} driver commands as tools», the word read from `VERBS` through `NUMBER_WORDS`. The README rewrite keeps that phrase (it may wrap a source line, since `prose` folds whitespace) and does not hand-copy a different count (per test-plan §4 Capability statement).
- test-plan §4 Capability statement requires every verb of `VERBS` and every cause of `Cause::name` to stand as a code span of `docs/capabilities.md`, and the statement to state five counts in words: `{word} verbs`, `{word} refusal causes`, `{word} session errors`, `{word} node keys` and `{word} exit statuses`. A restated paragraph must not drop or reword any of those phrases (per test-plan §4 Capability statement).
- test-plan §3 Cold-agent run pipe (Proof) records the one live run as the proof of the cold-agent gate: the operator host, never part of CI, fired once. The statement's restated run (`docs/capabilities.md:288-291`) states only what that record holds: the outcome, `isolated` on the twelve tools, all six apps in `tasks_met`, the tool-call and wrong-call counts against the 5 % bar, and the lint and screenshot call counts. The source is the record's evidence (`escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-verdict.json`, `evidence/live-run.md`), not the scope (per test-plan §3 Cold-agent run pipe).
- test-plan §2 Claims mapped to checks requires `docs/capabilities.md`'s `## Where each claim is proven` to name the check behind each claim, and to state each limit as pinned by its check or as asserted by no check. Only names and counts are pinned, so a renamed or moved check changes that section by hand. Whether any check name the section holds is stale, such as the renamed `stand_lint` test, is research's question for P3's file sweep (per test-plan §2 Claims mapped to checks).
- test-plan §1 Coverage scope (the `stand_requirement_sweep` row) requires `WITNESSES_020`, the 0.2.0 record, to name each witness test that is found as `fn {test}(` in its file. Whether `WITNESSES_020`'s v020-05 row already names `the_stand_at_boot_reads_its_measured_findings` is research's question. The chunk changes no test, so its ledger re-point must agree with what that table and the test files hold (per test-plan §1 Coverage scope).
- test-plan §9 Local pre-push gate requires `bash .github/scripts/ci-leg.sh fast`, which is fmt → clippy → test → ci-scripts, stopping at the first red. The ci-scripts leg (`python3 -m unittest discover -s .github/scripts`) is the leg that reads the edited documents (per test-plan §9 Local pre-push gate; test-plan §1 Coverage scope, CI Python scripts).

## Patterns to follow
- The pin has no fixture file. `test_capability_statement.py` reads `docs/capabilities.md`, `README.md` and escher-driver's `schema.rs`, `refusal.rs` and `error.rs` by path from the repository root and writes nothing. Editing the two documents is the whole test input, so no fixture moves (per test-plan §7, the Repository text row).
- Witness each ledger ref and each check name in the statement against the tree: look for `fn {name}(` in the named test file, the same lookup `stand_requirement_sweep` makes. Never take a name from the scope or a relay (per test-plan §1 Coverage scope).
- Read a gate by its own printed verdict. Redirect the leg's output to a file and read the file, since the leg's log lands in `target/ci-logs/{leg}.log` (per test-plan §9 Failure logs).

## Anti-patterns to avoid
- A count hand-copied into the README or the statement in place of the schema's table word, or a pinned phrase reworded so that the pin reads it no longer (per test-plan §4 Capability statement: "none hand-copied").
- Stating the cold-agent run as a CI gate, as run more than once, or with any figure the §3 Proof record does not hold (per test-plan §3 Cold-agent run pipe).

## Contract bindings
- tests §4 capability pin ↔ the driver schema's tables in arch (`VERBS`, `Cause::name`, `SESSION_ERRORS`, `NODE_FIELDS`, `EXIT_STATUSES`): the README's and the statement's counts follow those tables, and this chunk moves none of them (per test-plan §4 Capability statement).
- tests §3 Cold-agent run pipe ↔ obs §3: the restated run's `counts_agree` and `wrong_calls` come from the server's `tool_call` span lines in `server.log`. The statement's count of 66 calls is that span count read against the transcript (per test-plan §3 Cold-agent run pipe).
- tests §3 Cold-agent run pipe ↔ security plan: what the live run carried to the model provider is stated as the security plan records that crossing, not from the test record (per test-plan §3 Cold-agent run pipe).
- tests §1 `stand_requirement_sweep` (`WITNESSES_020`) ↔ the 0.2.0 verification matrix: the v020-05 ledger ref and the sweep's witness row name the same current test (per test-plan §1 Coverage scope).

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh fast` reads green on the edited tree. Its ci-scripts leg runs `CapabilityStatementTest`'s four tests green over the new `README.md` and `docs/capabilities.md`, and no test file is added or changed (per test-plan §9 Local pre-push gate; test-plan §4 Capability statement).
- `docs/capabilities.md`'s cold-agent paragraph states the 2026-10-25 run with the figures of the test-plan §3 Proof record: `passed`, the twelve tools, all six apps, 66 tool calls, 0 wrong against the 5 % bar, `lint_calls` 1 and `screenshot_calls` 1. No line of the file still names the 2026-10-15 run as the latest (per test-plan §3 Cold-agent run pipe).
- Each re-pointed ledger ref (v010-17 in `stand_snapshot_state`, v020-05 in `stand_lint`) and each check named under `## Where each claim is proven` resolves as `fn {name}(` in its named file on the chunk's tree (per test-plan §2 Claims mapped to checks; test-plan §1 Coverage scope).
