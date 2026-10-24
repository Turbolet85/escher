# Task — security · 2026-10-24-stand-requirement-sweep

## The chunk
A test-and-gate sweep: every 0.2.0 requirement v020-01…v020-09 gets a recorded witness, a headless check on the 7GUIs stand or a minimal fixture beside it, and any requirement with no witness gets a new check (one file and one test target per behaviour, both layout modes).
Every witness it names is brought inside the `stand-repeat` CI leg, either by widening that leg's selection or by a `stand_*` check carrying the proof. The per-crate coverage floors are re-read, and may be raised or extended to blitz-dom.
It measures stderr from the `escher-session` host and its `mcp` role at `trace` under both builds, covering the 0.2.0 verb surface (`lint`, `snapshot`'s `part`, `--record`): no element id, accessible name or typed text may appear.
It adds checks for the visual lint's untested readings (a clipped or off-screen text node also reads `truncated`) and for `text_extent`'s stated limits (RTL and vertical text, a transformed box). Engine or lint behaviour changes only if the founder rules a reading a defect; otherwise the chunk adds tests and changes CI only.

## Reached files
- tests/blitz-tests/tests/stand_requirement_sweep.rs · merge surface
- tests/blitz-tests/tests/common/mod.rs
- tests/blitz-tests/tests/session_common/mod.rs
- tests/blitz-tests/tests/fixtures/mod.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/text_extent.rs
- tests/blitz-tests/Cargo.toml · merge surface
- .github/scripts/test_blitz_tests_targets.py
- examples/seven_guis/tests/host_log.rs
- examples/seven_guis/tests/mcp_log.rs
- examples/seven_guis/tests/common/mod.rs
- .github/scripts/ci-leg.sh · conflict surface
- .github/scripts/test_ci_workflows.py
- packages/escher-driver/src/lint.rs
- packages/blitz-dom/src/text_extent.rs

## Needs
- The escher-telemetry sink's target allowlist and scrub sets. The stderr measurement asserts what they let through: engine targets keep their safe fields, `escher_*` targets have content-named fields redacted, other targets are dropped at every level, and a closed span's fields follow the same rule.
- The driver's two log spans, `command` (eight fields of fixed words, bools and counts) and `tool_call` (`tool` and `ending`). The measurement depends on `lint`, a paged `snapshot` and a recorded session adding no span field: no findings or their count, no part number or part count, no screen text or its length.
- The `escher-session` binary's stdout/stderr split. stdout carries the command's answer (ids, names, values, findings, part text) and is no log line. Usage and error lines go to stderr and write back nothing the caller typed. The measurement must count needles on stderr alone.
- The recording directory rules under `--record` on `start`, the host role and the `mcp` role: directory `0700`, a group- or other-open directory refused `record-dir-not-private`, each check `0600` and `create_new`, and the directory never logged. A recorded run in the log measurement must keep all of these, and stay inside the test-owned target directory.
- Kept-check data at rest: a masked or unresolved `type` is kept as `MASKED_STAND_IN`, answers are masked as on every carrier, and a shot is kept without its pixels. A check that records a session must not commit its file. The two committed Counter checks under `tests/blitz-tests/checks/` stay the one owner-only exception and are not rewritten.
- The strict `Check::from_bytes` reader and `Check::replay` (through `Session::run` alone). They are reached because `stand_replay` and the seven_guis `recording` check run inside `stand-repeat`.
- Masked values: a password's or file input's value reads `MASKED_VALUE` in the snapshot, its text, diffs and every carrier, and a password paints as bullets. The typed-text needle depends on this, and so do fixture witnesses such as `password_caret_click` brought into the sweep.
- The driver session socket and state directory as the session checks use them: state directory `0700`, socket `0600`, the socket path's length bound (`state-dir-too-long`), the idle expiry, and one connection at a time. The host-log measurement drives a hosted session over it.
- The MCP stdio server's bounds, as `mcp_log` drives the `mcp` role: request line and result size limits, fixed protocol errors with no `data`, and the request `id` echoed only in its two admitted forms.
- The driver command schema and refusals. `lint` takes no argument and `part` is bounded 1 to `MAX_PART`. A `Refusal` holds nothing of the call, and no id, name, value or call-supplied argument name may appear in a new check's test name or assertion message.
- The visual lint's findings: ids, closed defect words and numbers only, with no `NodeId`, page text, colour or control value. They cross on every carrier as the `lint` answer and are kept whole in a recorded check. Changing what `Session::lint` returns would move a driver answer on every carrier, which is a boundary-widening fork for the founder.
- `BaseDocument::text_extent` stays a read-only reader returning geometry only (Layout values). Changing it is an engine change for every document, a founder fork.
- The widening rule: a new field of a closed value set inside a ratified carrier is no widening. Authored text, an open value set, a new carrier or a new exit still is. This governs any reading a new check might push into an answer.
- Markup attribute reads in fixture witnesses brought inside `stand-repeat`: `inert` by presence, ARIA states through closed parses carrying no attribute text, `disabled` on form controls, range and select rules including the select's private typeahead buffer, and Enter/Space activation. They are read only, with no new log site, print or carrier.
- The stable element id, which leaves only by the ratified carriers, never through a log. A fixture's author ids act as the id needles in the stderr measurement.
- Environment values a check reads: `CARGO_BIN_EXE_escher-session` and `CARGO_TARGET_TMPDIR` at build time in the session checks, and `RUST_LOG`, which the log checks set for escher-telemetry's filter. Any new env read or test-only listener that a new check makes must be registered as test-only before it lands.
- `LOAD_WAIT` and settle never waiting on a load in flight. Reached through `stand_load_wait` and the fixture boot's held network stand-in running under the leg's three rounds.
- The `coverage_floor.py` argument rules, applied when floors in `ci-leg.sh` change or blitz-dom is added: each floor `CRATE_DIR=PERCENT`, at most 100, no crate twice, an `SF` record for every crate, else exit 2. It prints only crate directories and counts.
- CI leg integrity: every cargo command in `ci-leg.sh`, including `stand-repeat` and `coverage`, passes `--locked`. `cargo-llvm-cov` stays pinned in ci.yml, and `test_ci_workflows.py`'s pin invariants are kept when the leg's selection or floors change.
- `scripts/agent-run.sh`'s closed selection (`stand`, `all` or an existing test-file stem). A new `stand_*` check joins the `stand` selection there, and its events keep carrying no content-named field.
- No new dependency. blitz-tests' dev-dependencies (including `png`) stay workspace-inherited, and no package enters `Cargo.lock`.

## Not needed
- Remote web content and outbound networking: blitz-net's `file:`/`data:` fetching, request limits, TLS, the HTTP cache, cookies, `@font-face` sniffing, and SVG, CSS, image and font input checks. The fixtures run offline.
- Script execution: blitz-vibey-script, Boa, the ScriptFetcher, `isTrusted`, and JS API argument checks.
- The browser and rdme apps: urlbar, external-browser navigation, sqlite history, error pages, clipboard, the file dialog, and comrak's raw HTML.
- The WPT runner's file reads, quarantine and `env_logger`, and the upstream apps' unscrubbed `fmt::init()` subscribers. The measurement covers escher's sink alone.
- The cold-agent pipe: `cold-agent.sh`, the agent-written check, the model provider and the operator's Claude Code login. That is v020-11, owned by the later «Cold-agent bar» entry.
- CI secrets and signing: GitHub Actions secrets, the publish-browser, wpt and wpt-post-results workflows, and their repository guards. No secret-bearing job is touched.
- The dependency audit's per-ID ignores and the licence gate: `deny.toml`, paste, zip, the zbus family and memmap2. No crate is added or bumped.
- The example CLIs (`screenshot`, `paint_bench`, `preact_script`) and `multi_doc`'s `MULTI_DOC_*` env reads, plus `apt-install.sh`'s argument rules.
- Unsafe code in stylo_taffy and layout, and the engine's listed panic paths (URL resolve, network setup, parse drivers, table layout, shell and runtime setup). The chunk adds tests, not engine code paths, unless a founder fork rules otherwise.
- The driver session label rules and the session socket's wire grammar and `v3` reply forms. The chunk drives existing verbs and adds no request or reply form.
- The dioxus-native form-data validity gap and Dioxus mutation handling (`dangerous_inner_html`).
- Authentication: none exists and none is added. Socket access stays the filesystem's.
