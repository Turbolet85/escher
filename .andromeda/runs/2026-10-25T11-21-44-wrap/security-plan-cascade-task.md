# Task — security-plan · 2026-10-25-capability-statement

## The session
The session added docs/capabilities.md, a 304-line public document in five sections: what escher 0.2.0 can do, its stated limits, its accessibility reach, what an MCP client carries onward, and where each claim is proven.
It added .github/scripts/test_capability_statement.py, a four-case pin. The pin reads the schema's tables (twelve verbs, eight refusal causes, fifteen session errors, thirteen node keys, four exit statuses) and checks that the statement and README state those counts. It joins the ci-scripts leg through unittest discover, taking that leg from 100 to 104 tests and from seven to eight test files.
It replaced one line of README.md (line 48): the eleven driver commands as MCP tools became twelve, and the line now links docs/capabilities.md.
It added, changed or retired no Rust symbol, verb, argument, field, cause, session error, exit, carrier, log field, span, dependency, crate or config (guard clean). Outside the repository, an ignored Viola pointer names only the repository, the path and a commit.
It measured three standing claims as false. The MCP stdio boundary's carried list leaves out diffs (outside the cold-agent sentence) and a lint's findings. The cause set is said to be nine where the code has eight. The assistive-technology witness's owner is named «(target: next)» where its residuals line now reads re-carried:0.2.0.

## Reached files
- docs/capabilities.md
- .github/scripts/test_capability_statement.py
- README.md

## Touches
- threat-model-summary, group boundary-mcp-stdio (site at line 34, label at line 30): the carried list names ids, accessible names, control values and snapshot text, with diffs only inside its cold-agent sentence and no lint finding. An MCP result carries snapshot text, diffs and, since 2026-10-19-visual-lint, a lint's findings, so the list must name diffs and findings.
- The body's §Threat Model Summary local-stdio vector (the broad view over boundary-mcp-stdio) lists the same carried set without diffs or lint findings. It follows the detail correction above.
- The category index's subjects comment at the foot of security-plan.md names boundary-mcp-stdio, one of the two mcpcarry label rows. The label stays; the site under it changes.
- error-handling, group errors (line 9): «the cause set (nine)» is false. `CAUSES: [Cause; 8]` in packages/escher-driver/src/refusal.rs:35, and the pin's cause_names reads eight.
- input-validation line 217: the windowed assistive-technology witness is owned by a residuals line «(target: next)». That line now reads re-carried:0.2.0 (.andromeda/residuals.md:14), and escher-0.2.0/requirements.md:42-43 says this version does not reach the witness, so the owner disposition changes.
- Indirect, through the pin: every count security-plan states for the driver's tables is now pinned against the schema by test_capability_statement.py. A stated count of verbs (twelve), refusal causes (eight), session errors (fifteen), node keys (thirteen) or exit statuses (four) that disagrees is contradicted.
- Indirect, through README.md: any security-plan text that cites README's count of MCP tools moves from eleven to twelve.
- Indirect, through the ci-scripts leg: any CI-scripts test count or file inventory in security-plan moves from 100 to 104 tests and from seven to eight files. No leg, job, workflow or ci-leg.sh line changed, so the cited ci-leg.sh line ranges stay valid.
- secret-management absence group: a re-read found the claim unmoved. The new committed statement and its pin hold no credential, token, auth header or host home path (the secret search read 0 hits).
- secret-management environment values: a re-read found the list unmoved. The pin reads by path from the repository root, writes nothing and reads no env var, so the list of env reads gains nothing.
- authentication-authorization values: a re-read found the claim unmoved. The statement claims no authentication beyond the 0700 state directory, the 0600 socket and holding the MCP server's stdio.
- input-validation masking line: a re-read found it unmoved. The statement says the MCP client receives masked values and a shot painted masked, which matches the standing MASKED_VALUE rule.
- api-security and input-validation net-provider limits: a re-read found them unmoved. The statement restates that no verb takes a URL, that file: URLs are read with no path restriction, and that there is no response-size cap or request timeout.
- logging-monitoring escher sink and spans: a re-read found them unmoved. The statement names the command and tool_call spans with closed-word, bool and count fields on stderr only, and names the scrub's limits (the raw panic message, log.file on a bridged js_console record, the dropped third-party WARN or ERROR, the unscrubbed upstream apps and WPT runner).

## New facts
- F1 · threat-model-summary · A committed public document, docs/capabilities.md (section «What an MCP client carries onward»), now states the MCP crossing. It names the boundary as the spawning client holding stdio with no authentication. It lists the carried set (ids, names, masked values, snapshot text and diffs, a shot as an image block painted masked, a lint's findings) and says no description is carried. It says what the client does onward is the client's. It adds no carrier.
- F2 · secret-management · The committed capability statement and its pin, .github/scripts/test_capability_statement.py, hold no credential, token, auth header or host home path (the secret search read 0 hits), and the pin reads no environment variable.

## Untouched
- input-validation lines 193, 195 (twice) and 210 name the assistive-technology witness as still owed, with no owner target. They are true as worded and do not change. Only line 217's «(target: next)» owner changes.
- input-validation line 197, the actionable-key check's owner phrase («no driver, CLI or MCP command yet … owned by a route entry»), does not change. The statement says only that the check has no driver verb.
- dependency-security as a whole: no dependency, lockfile entry, audit ignore, licence or CI tool pin changed.
- data-protection's kept checks and the two committed Counter checks: no recording, check file or replay rule changed, and the committed checks are still the same two.
- logging-monitoring's sink allowlist, scrub sets and span fields: no log field, span, subscriber or stdout form was added.
- input-validation's driver command schema, session socket grammar, state and recording directories, Check::from_bytes and MCP server bounds: no verb, argument kind, bound, refusal order or wire changed.
- error-handling's Refusal typing, SessionError table, MCP protocol errors and panic paths: only the cause count's word is wrong, and no error shape changed.
- threat-model-summary's remote-content, script-execution and session-socket boundary groups: no change reached them.
- api-security's outbound request controls, LOAD_WAIT and session-socket reach: unchanged.
- secret-management's CI secrets and the operator-login storage: no secret source was added or used.
- The cold-agent pipe's input rules and carried screen: the statement says the 2026-10-15 run had eleven tools. That is a dated reading, not re-fired, and no pipe rule changed.
