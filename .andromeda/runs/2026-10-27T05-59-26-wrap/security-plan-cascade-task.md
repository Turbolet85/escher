# Task — security-plan · 2026-10-27-readme-and-records

## The session
A documents-and-records chunk: no source, test, CI script, manifest, lock file, deny.toml or spec master changed (preservation guard clean, 2 files allowed and changed).
README.md now shows 0.1.0 and 0.2.0 done and verified and 0.3.0 planned in six stages, names 0.2.0's additions, the six task slugs, a `lint` try-it line and a six-item list of what the driver does since 0.2.0.
docs/capabilities.md restated three passages: the lint at boot (the flight booker reads one `overlap`), the host-sink part read (now measured: `serve cells` at `trace`, 0 id and 0 name needles under both builds) and the run paragraph, now the 2026-10-25 cold-agent live run (six apps, twelve tools, 66 calls, 0 wrong against a 5 % bar, client `claude` 2.1.308, through the operator's own login).
Records: the 0.3.0 ledger and route stamped (v030-02 gated on the founder's word on F3, v030-16 by-construction); the local incubator exclude lines removed from `.git/info/exclude` (`.gitignore:39` ignores the incubator before and after).
The operator pass respelled a planted control literal, a home-directory path of a made-up user `someone`, to the placeholder `<user>` in a committed run-dir script after hygiene P1 refused it; CI green 17/17 on the pushed head.
Measured: the front page renders the default branch `build/escher-0.1.0`, not this branch's README (a project-intent fact, not a security one).

## Reached files
- README.md
- docs/capabilities.md
- escher-0.3.0/verification-matrix.json
- escher-0.3.0/working-route.md
- .andromeda/master-route.md
- .andromeda/friction-log.ndjson
- escher-0.3.0/chunks/2026-10-27-readme-and-records/scope.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/research.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/plan.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/inputs/
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/records.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/counts.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/operator-pass.md
- .andromeda/runs/2026-10-27T03-47-30-phase/p5-controls.sh
- .andromeda/runs/2026-10-27T05-19-41-implement/
- .andromeda/runs/2026-10-27T05-59-26-wrap/
- .git/info/exclude

## Touches
- threat-model-summary, boundary-mcp-stdio: the standing line that the cold-agent live runs carried the screen to the model provider (a screenshot and a lint's findings among it). The statement now names the 2026-10-25 run as the measured one: six apps' ids, names and values in snapshot texts, paged parts and diffs, Cells' contents, the `flight-type` select's options and value, a lint's findings and one screenshot. Re-read it against that; the carriage itself is unchanged.
- secret-management, development storage on the operator host: the statement says the 2026-10-25 run reached the model provider through the operator's own login. Re-read the claims that the login is held by the `claude` CLI and that no key or credential is in committed evidence. Security probe entry 7 found 0 hits of `/home/`, an api key, `authorization:` or `bearer ` over README.md, docs/capabilities.md and their pin `test_capability_statement.py`.
- Any security-plan line that names a cold-agent live run by date, its tool count or its client version: the statement now names the 2026-10-25 run (twelve tools, client `claude` 2.1.308) as the measured instance and names the 2026-10-15 run once, dated.
- logging-monitoring, the escher-session host's sink when a part is read: the statement changed from unmeasured to measured (`serve cells` at `trace`, 0 id and 0 name needles under both builds). The report's sweep found `logging-monitoring.md:14` already stating the measured reading, so no change is expected.
- logging-monitoring, the cold-agent pipe's verdict, which holds only counts, booleans and closed words: every run figure in the README and the statement is a `live-verdict.json` field (tool calls 66, wrong 0, bar 5 %, six tasks met, 1 lint, 1 screenshot, client version, outcome passed). No line quotes transcript text, a tool answer or a kept check's content.
- input-validation and logging, the rule that committed evidence and transcripts carry no host path (`gate.py hygiene` P1): hygiene refused the planted home-directory literal (a made-up user `someone`) in the committed `.andromeda/runs/2026-10-27T03-47-30-phase/p5-controls.sh:52`. Its user was respelled `<user>`, keeping the control's meaning, and is named in the pre-CI commit body. This is the second time a P5 control has tripped P1; the rule itself is unmoved.
- input-validation, wherever `select#flight-type` is named as an input path or contract (the `ds-select` sweep hits in security-plan): this chunk's design-system stand-row move leaves those mentions as they are.
- Counts: twelve driver verbs as tools, eight refusal causes and the capability statement's five counts are unmoved (pin `test_capability_statement.py` ran 4 tests, OK). Any security-plan count of verbs or tools stands.
- The README now publicly lists the 0.2.0 driver capabilities whose security rules security-plan holds: a large screen in parts, the `select`, `lint` and its six defects, the one virtual clock, the bounded load wait (`LOAD_WAIT`), and a session recorded as a check that replays. Each is worded as docs/capabilities.md states it. Re-read for consistency only; no rule moved.
- The README's Later line names the platform accessibility adapter witnessed with a real assistive technology. It agrees with the standing line that the adapter's state crossing is unmeasured at an assistive technology and re-carried.
- Repository ignore: the incubator `escher-0.3.0-incubator/` stays ignored by `.gitignore:39`; only the local, uncommitted `.git/info/exclude` lines were removed. The `incubator` pattern did not fire over the masters, so no security-plan text describes it.

## New facts
- (none)

## Untouched
- dependency-security and its supply-chain group: no manifest, `Cargo.lock`, `deny.toml`, advisory ignore, CI pin or audit leg changed.
- api-security: no listener, socket, port, bound, idle expiry or `LOAD_WAIT` value changed; the README and statement only describe them.
- authentication-authorization: no handshake, token, peer check or state-directory mode changed.
- error-handling: the eight refusal causes, the session errors and their remedies, the MCP protocol errors and every panic path are unchanged; no source changed.
- input-validation, the driver schema, the wire `v3`, the request and answer bounds, the MCP server's bounds, `Check::from_bytes` and the CLI argument readers: none changed. The README's new `lint` try-it line runs an existing verb.
- data-protection, kept checks and the two committed Counter checks (`tests/blitz-tests/checks/`): not regenerated or changed; the README only mentions that a recorded session replays.
- secret-management, the list of environment values read: no env read was added; the chunk touched no source.
- logging-monitoring, escher-telemetry's allowlist (the seven engine fields, the eleven content-named fields, the drop of other targets): unchanged; only the statement's record of a past measurement moved.
- threat-model-summary, the WPT post-results workflow's checkout of scripts from the default branch: it shares the words default branch with this chunk's front-page finding, but it concerns upstream's guarded workflow and stays true.
