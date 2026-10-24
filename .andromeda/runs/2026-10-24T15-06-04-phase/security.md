# security extract

## Relevance
partial — the sweep adds checks and may move CI gates, but builds no new surface. Its security load is the "new verbs' stderr id-free" measurement (the `lint` verb, `snapshot`'s `part` and `--record`) and the gates and artifacts a sweep touches: coverage floors, fixtures, evidence and kept checks.

## Constraints
- Security plan §Logging & Monitoring requires that escher's sink, installed by `escher-session` (host role and `mcp` role), print no element id, accessible name or typed text at any `RUST_LOG` level: an engine target prints only its seven allowlisted fields, an `escher_` target has its eleven content-named fields redacted, and any other target is dropped whole. The sweep's stderr measurement of `lint`, `part` and `--record` runs against this rule, at `trace`, under both builds. Whether `host_log`/`mcp_log` already drive a `--record` session is research's question: the plan records `lint` and a part read as measured there, but records no stderr reading under `--record`.
- Security plan §Input Validation (Driver command schema) requires the `command` span to hold its eight fields of fixed words, bools and counts only. It never fields a lint's findings or their count, a part's number or the part count, the screen text or its length, or a shot's image or dimensions. The `tool_call` span holds `tool` and `ending` only. Any new check that reads span lines asserts this field set as it stands and adds no field.
- Security plan §Input Validation (Driver recording directory) requires that no tool call, verb, argument or result field names the recording directory, and that nothing of it is logged. A `--record` stderr reading therefore also counts the recording directory's path, beside the id, name and typed-text needles.
- Security plan §Input Validation (Driver session socket, the widening rule) treats a field holding authored text, an open value set, a new carrier or a new exit as a boundary widening. Under C1, a fix that changes `Session::lint`'s findings changes a driver answer on every ratified carrier. That fix, and any `text_extent` change, is the founder's fork, not a lean. Security plan §Input Validation (Layout values) holds `text_extent` as a read-only reader that returns geometry only.
- Security plan §Data Protection (Files, committed checks) makes `tests/blitz-tests/checks/counter-{cli,mcp}.json` the only committed kept checks. A check recorded by any other session (a sweep run, a fixture flow) stays owner-only (`0600` in a `0700` directory) and is never committed.
- Security plan §Input Validation (CLI arguments, coverage_floor.py) requires each floor to match `CRATE_DIR=PERCENT`, with a percent of at most 100 and no crate named twice. The script prints only crate directories and counts, never a source or lcov path. A raised floor or an added crate (blitz-dom) goes through this reader, and `test_coverage_floor.py` pins it.
- Security plan §Secret Management (Environment values read) lists every env read. The session checks read `CARGO_BIN_EXE_escher-session` and `CARGO_TARGET_TMPDIR` at build time, and the binary reads `RUST_LOG` alone. A new check or fixture adds no env read outside that list unless the read is registered first.

## Patterns to follow
- Use the needle-count instrument of `examples/seven_guis/tests/{host_log,mcp_log}.rs`. Plant a typed sentinel, take every id and name the flow reads as needles, and count occurrences in stderr at `trace` under `-p seven_guis` and `--workspace`. Also count the first 64 base64 characters of any shot (per security plan §Logging & Monitoring).
- Record the reading as counts by build and level, as 0.1.0's `evidence/log-counts.md` did. Never copy raw stderr into committed evidence: per security plan §Logging & Monitoring, an allowlisted `log.file` field can still carry a host path.
- New fixture checks read refusals and findings as closed words and counts. Per security plan §Error Handling, a `Refusal` has no `String` field and only a cause's fixed name reaches a log.
- Read a kept check only through the strict `Check::from_bytes` and replay it through `Session::run` alone, as `stand_replay` does (per security plan §Input Validation, Recorded check).

## Anti-patterns to avoid
- Adding a log site, print, span field or `Debug` dump that carries an `Outcome`, a `Call`, a finding, a part's text or a recording path, even in a sweep or diagnostic helper. Per security plan §Input Validation (Driver command schema), the crate fields none of them.
- Committing a sweep-recorded kept check, a raw stderr capture or an evidence file that holds host paths, typed text or screen content (per security plan §Data Protection).
- Changing a CI gate (a coverage floor or `stand-repeat`'s selection) without its `test_coverage_floor.py`/`test_ci_workflows.py` invariants moving with it (per security plan §Input Validation, CLI arguments (coverage_floor.py)).

## Contract bindings
- security §Logging & Monitoring ↔ obs-plan (the sink's target allowlist and scrub sets in `packages/escher-telemetry/src/format.rs`): the stderr needle readings are the obs pipeline's redaction witness.
- security §Input Validation (CLI arguments, coverage_floor.py) ↔ test-plan CI integration: the `coverage` leg's floors and the `stand-repeat` leg's selection (`.github/scripts/ci-leg.sh`) are pinned by `.github/scripts/test_ci_workflows.py` and `test_coverage_floor.py`.
- security §Data Protection (committed checks) ↔ test-plan: `stand_replay` and seven_guis `recording` keep the two committed Counter checks fresh, and no other kept check joins them.

## Acceptance criteria contributions
- Under both builds at `RUST_LOG=trace`, `escher-session`'s host role and its `mcp` role, each driving `lint`, a `snapshot` part read and a `--record` session, show 0 typed-sentinel occurrences, 0 id needles and 0 name needles on stderr. The recording directory's path occurs 0 times (per security plan §Logging & Monitoring; §Input Validation (Driver recording directory)).
- Every `command` span line read by the sweep's checks holds only the fields `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed` and `changed`. Every `tool_call` line holds only `tool` and `ending` (per security plan §Input Validation (Driver command schema)).
- `git ls-files tests/blitz-tests/checks/` lists exactly `counter-cli.json` and `counter-mcp.json` at the chunk's end (per security plan §Data Protection).
- If a coverage floor or crate changes, `python3 .github/scripts/test_coverage_floor.py` and the leg's own floor read pass. `coverage_floor.py` output still holds only crate directories and counts (per security plan §Input Validation (CLI arguments, coverage_floor.py)).
