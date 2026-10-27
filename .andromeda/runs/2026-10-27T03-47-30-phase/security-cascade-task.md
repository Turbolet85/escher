# Task — security · 2026-10-27-readme-and-records

## The chunk
Rewrites the README's prose so it shows 0.1.0 and 0.2.0 done and verified (what 0.2.0 can do, its cold-agent result) and 0.3.0 planned in its six stages, writing «Escher» with a capital E in prose.
Restates the capabilities statement's cold-agent paragraph against the 2026-10-25 live run (six apps, twelve tools, 66 calls, 0 wrong, a screenshot and a lint called), with what crossed to the model provider stated as the security plan records it.
Re-points two verified ledger refs to the tests' current names, and moves four design-system stand rows into architecture and layout-templates as an expected wrap amendment.
Re-reads the incubator ignore in `.gitignore` (witnessed by `git check-ignore -v`) and the capability statement's pin, which stays green; no code, test, engine, driver, CI or schema change.

## Reached files
- README.md · edits §Status, the "Later:" line, "What Escher is", the driver paragraph, "Try it"
- docs/capabilities.md · edits the cold-agent run paragraph (lines 288-291) and any other line naming the 2026-10-15 run as the latest
- .github/scripts/test_capability_statement.py · re-reads the statement's pin (verb and cause names, five counts, the README's tool count)
- escher-0.1.0/verification-matrix.json · edits v010-17's verified ref
- escher-0.2.0/verification-matrix.json · edits v020-05's verified ref
- tests/blitz-tests/tests/stand_snapshot_state.rs · re-reads the test name `exactly_the_chosen_flight_type_reads_selected`
- tests/blitz-tests/tests/stand_lint.rs · re-reads the test name `the_stand_at_boot_reads_its_measured_findings`
- .andromeda/design-system.md · edits the accent row, the success row, the cards item, the stand's DejaVu Sans item
- .andromeda/architecture.md · edits the section that receives the stand rows
- .andromeda/layout-templates.md · edits the section that receives the stand rows
- .andromeda/residuals.md · edits the two absorbed lines (41, 42)
- .gitignore · re-reads the incubator ignore line (line 39, `git check-ignore -v`)
- .git/info/exclude · edits the two incubator lines at its end

## Needs
- The capabilities statement's account of the 2026-10-25 cold-agent live run: what crossed to the model provider (ids, names and values in snapshot texts, paged parts and diffs, a lint's findings and one screenshot) must match the security plan's record of that run's crossing, neither wider nor narrower.
- The cold-agent pipe's registered crossings (spawned `claude` client, the `escher-session mcp` server as its tools, the model provider on live runs only), as the statement and the README name them.
- The MCP stdio boundary as the README's driver paragraph describes it: a stdio child of the spawning client, no listener, port or socket, results carrying ids, names, masked control values, snapshot text, diffs, screenshots and lint findings.
- The driver session socket as the README describes it: a local Unix-domain socket, owner-only by file mode (directory `0700`, socket `0600`), no other authentication, no TCP or UDP port.
- The masking rule wherever README or statement prose describes a value or a shot: a password's or file input's value reads as `MASKED_VALUE`, a password paints as bullets, a recorded masked `type` is kept as `MASKED_STAND_IN`.
- Kept checks at rest as the README names "a session recorded as a replayable check": owner-only `0600` files in a `0700` directory, never logged, the two committed Counter checks under `tests/blitz-tests/checks/` the one exception.
- The `--record <dir>` flag as "Try it" may show it: a group- or other-open directory is refused `record-dir-not-private`.
- The driver schema's closed tables the README and statement count (twelve verbs, eight refusal causes, seven argument kinds), held to `VERBS` by the statement's pin; the prose follows them and moves none.
- The visual lint's findings as the README names them: stable ids, closed defect words and numbers only, no page text, colour or value.
- The wait for a page load as the README names it: an acting command waits at most `LOAD_WAIT` (5 s), `snapshot`, `screenshot` and `lint` wait on none.
- The `escher-session` command line as "Try it" shows it: one JSON line on stdout, exit statuses 0 · 1 · 2 · 3, nothing the caller typed written back.
- The operator's Claude Code login the live run used, held by the `claude` CLI: the statement may say the run used it, and names no key, credential file, token or e-mail address.
- The README's claims stay inside what the repository's records prove: no carrier, field, exit or widening stated that the security plan has not recorded as ratified.
- The incubator stays out of every commit: `.gitignore`'s line ignores it, and no incubator file is tracked.

## Standing conventions
- S1 · committed text holds no secret, credential, auth header, token or e-mail address · .andromeda/security-plan-detail/secret-management.md
- S2 · any statement of what a carrier holds stays inside the ratified crossings; a new field of an open value set, authored text, a new carrier or a new exit is a widening for the founder · .andromeda/security-plan-detail/input-validation.md
- S3 · committed prose and evidence hold no host path · .andromeda/security-plan-detail/logging-monitoring.md

## Not needed
- Dependency security (cargo-deny, `deny.toml`, the advisory ignores, `serde_json`, `png`, `color`, supply-chain pins): no manifest or `Cargo.lock` change.
- Remote web content handling (net provider `file:` and `data:` URLs, fetched fonts, `@font-face` sniffing, SVG, CSS and HTML parsing, subdocument caps): no engine change.
- Script execution on Boa and the default `ScriptFetcher`: untouched.
- rdme's comrak markdown renderer and its raw-HTML pass-through: no app code changes.
- The WPT runner's local file reads, its net provider and its `env_logger`: untouched.
- The CI workflows' post-results guard, publish-browser signing material and environment: no workflow change.
- Browser persistence (sqlite history), the HTTP cache, the clipboard and the native file dialog: untouched.
- Engine panic paths, typed engine errors and graceful-degradation readings: no code change.
- escher-telemetry's sink allowlist, scrub sets and the `command` and `tool_call` span fields: no log site or span changes.
- The example binaries' argv handling (`screenshot`, `paint_bench`, `preact_script`, `multi_doc`) and the `agent-run.sh`, `apt-install.sh` and `coverage_floor.py` argument checks: untouched.
- The cold-agent pipe's argv allowlist and its agent-written check bounds: the script is neither changed nor re-run.
- `Check::from_bytes`'s strict reader and `Check::replay`'s internals: no driver code changes.
- Outbound request controls (per-host concurrency, absent CORS, CSP, size limit and timeout) and in-document navigation rules: untouched.
- The unsafe calc-pointer blocks in `stylo_taffy` and blitz-dom layout: untouched.
