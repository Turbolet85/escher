# tests extract

## Relevance
partial — the chunk writes a document and edits README.md, adding no code and no test. The tests domain binds only two things here. First, the statement's "proven by / pinned by / not measured" claims have to match what test-plan records as checked. Second, the tree with the document in it still passes the standing gates.

## Constraints
- A stated limit can be called "pinned" only where test-plan records a check that pins it. Per test-plan §1 Test Scope Summary (blitz-dom covered behaviours): `text_extent`'s stated limits (RTL and vertical text read as LTR lines) are asserted by blitz-tests' `text_extent`, while the select's (`multiple`, `optgroup`, no popup) are asserted by no check. The statement has to keep these two kinds apart.
- Per test-plan §1 Test Scope Summary (dioxus-native-dom): the boolean model's two limits (a mixed toggle reads unpressed, a grammar or spelling error reads invalid) are pinned in dioxus-native-dom unit tests and at the engine only. The statement must not credit them to a stand or driver check.
- Per test-plan §1 Test Scope Summary (tests/blitz-tests): no check reads the accessibility states as the platform adapter hands them to an assistive technology. That check is owed, and its `.andromeda/residuals.md` line (target: next) owns it. The a11y-reach section must state it as unmeasured.
- Per test-plan §2 Test Strategy (Pixel tests): a shot's PNG equals the harness's raster and is byte-identical across runs on one host, but its bytes differ between OSes, so a check compared byte for byte across OSes holds no shot. This is the source for the statement's cross-OS screenshot limit.
- Per test-plan §3 Test Harness Contract (Cold-agent run pipe, Proof): the one live run (2026-10-15-cold-agent-test) passed with the eleven `mcp__escher__*` tools. The pipe now requires twelve tools with `lint`, and the live run has not been re-fired. A statement that uses the cold-agent pipe as the measured MCP-client instance must say that `lint` was not part of that measured run.
- Per test-plan §6 E2E Test Strategy (Drivers per surface / Observed absent): the MCP role is driven in CI as a child through `Speaker` and in process. A real client drove it once, in the operator pass and not in CI. No test in the apps launches a window. The statement's carrier and reach claims must not go past these drivers.
- Per test-plan §9 CI Integration (Pipeline facts, Local pre-push gate): `bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → ci-scripts) is the gate the tree has to pass after the README edit. Whether any CI script test reads README.md's text, so that the verb-count edit could move it, is research's question.

## Patterns to follow
- Give every measured claim its source the way test-plan does ("as measured at {chunk}/report.md", or a CI run id). Example: the stand's reach as `run stand` 197 · 0 · 7 over 48 files, as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md (per test-plan §3 Test Harness Contract, Test-data bootstrap of the 5-command contract).
- State a proof surface by its named check targets and layout modes ("in both layout modes", "both builds"), as test-plan §1 Test Scope Summary and §6 E2E Test Strategy name them. Avoid general wording such as "tested".
- State the a11y reach from the `a11y` leg's recorded reading. Per test-plan §9 CI Integration (detail `ci-integration.md` line 65), that is fourteen files: 14 result lines · 84 passed · 0 failed · 0 ignored.

## Anti-patterns to avoid
- Do not present a limit that no check asserts as checked. Per test-plan §1 Test Scope Summary (blitz-dom covered behaviours), the select's limits are asserted by no check, and neither is the process-wide `Parallel` traversal lock.
- Do not present the live cold-agent run as evidence for the twelve-verb surface or for `lint` over MCP with a real client (per test-plan §3 Test Harness Contract, Cold-agent run pipe, Proof).

## Contract bindings
- tests ↔ a11y: the statement's a11y reach rests on the `a11y` leg's fourteen files (test-plan §9 CI Integration) and on the residual of the assistive-technology reading (test-plan §1 Test Scope Summary). Each claim is closed against a11y-plan §1 / §3.
- tests ↔ security: the cold-agent pipe's crossings and the MCP role's carriage (test-plan §3 Test Harness Contract, Cold-agent run pipe) are the measured instance for "what an MCP client carries onward". The claims are closed against security-plan §Surfaces.

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh fast` and `bash .github/scripts/ci-leg.sh doc` exit 0 on the tree that holds the statement and the README verb-count edit (per test-plan §9 CI Integration).
- Every limit the statement calls "pinned" or "asserted" names a check that test-plan records. The select's limits and the boolean model's two limits are worded exactly as their recorded coverage stands: no check for the select, unit-level and engine-level for the boolean model (per test-plan §1 Test Scope Summary).
- The statement's MCP-client section says the 2026-10-15 live cold-agent run measured eleven tools and has not been re-fired since `lint` made twelve (per test-plan §3 Test Harness Contract, Cold-agent run pipe).
- The statement's screenshot limit says a shot's PNG bytes differ between OSes (linux x86_64 vs macOS aarch64) and are byte-identical across runs on one host (per test-plan §2 Test Strategy, Pixel tests).
