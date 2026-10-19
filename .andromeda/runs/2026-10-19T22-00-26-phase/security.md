# security extract

## Relevance
partial — test-only infrastructure (shared fixtures, one in-process held `NetProvider`, an agent-run selection), no new crossing; security binds where the moved checks witness plan rows and where `scripts/agent-run.sh` validates and logs.

## Constraints
- The held network stand-in opens no listener, port or socket: security-plan §API Security requires the driver session's Unix-domain socket to stay the one listener, and a test-only bind is registered as such before it lands (the `TMPDIR` precedent, per security-plan §Secret Management "Environment values read"). Whether the merged stand-in's code already holds no bind is research's question.
- The stand-in and the shared fixture modules read no new environment variable: security-plan §Secret Management "Environment values read" lists every env read by site (`CARGO_BIN_EXE_escher-session` and `CARGO_TARGET_TMPDIR` in `tests/blitz-tests/tests/session_common/mod.rs` among them). A site that moves from one file to a shared module moves its citation there. A new read is registered as test-only before it lands.
- The stand-in fetches nothing. It answers held requests with bytes the check supplies and never routes a URL through blitz-net. Security-plan §Input Validation (`file:` URLs (net provider) row) records that blitz-net reads `file:` paths with no restriction. Security-plan §Input Validation (`@font-face` source row) states that escher routes no agent- or user-supplied URL through the sniffed-font path.
- `font_face_sniffed_source` keeps its witness unchanged in both layout modes. Security-plan §Input Validation (`@font-face` source row, and the Fetched fonts row) records that the hint-less `data:` source is requested exactly once, that a `wOF2` body registers (text width 0 before the answer, above 0 after, no further request) and that a body no sniff matches registers nothing.
- `stand_settle` keeps its reading of a held load. Security-plan §API Security (Settle and loads in flight row) requires that a load in flight is reported at once as `NotSettled` naming `Loads` and is never waited on. The outcome names a class only, with no URL, request, node, id, name or value.
- A shared password fixture keeps the mask witness. Security-plan §Input Validation (Markup attributes, password and file `input` value row) requires the snapshot, its text and its diff to read `MASKED_VALUE`, never the typed text or its length. Which of the seven copies' markup a mask check rests on is research's question.
- An agent-run change for a fixture selection keeps the closed argv. Security-plan §Input Validation (CLI arguments (agent-run.sh) row) requires a verb outside the five, or a selection other than `stand`, `all` or an existing `^[a-z0-9_]+$` test-file stem, to print usage and exit 2 before any cargo call.

## Patterns to follow
- A hand provider that holds every request offline, with the check answering each one, as in `tests/blitz-tests/tests/font_face_sniffed_source.rs` (per security-plan §Input Validation `@font-face` source row).
- A test-only resource registered by its site, with the note that no binary serves it and no caller reaches it (per security-plan §Secret Management, the `TMPDIR` row).
- Assertions on the masked readings compare against the public `MASKED_VALUE` constant and count occurrences of the typed text (0), never print it (per security-plan §Input Validation password and file `input` value row).
- agent-run's selection reaches cargo only as an existing test-file stem. Its events carry closed metadata keys only (per security-plan §Input Validation CLI arguments (agent-run.sh) row, and §Logging & Monitoring agent-run row).

## Anti-patterns to avoid
- A shared stand-in that reaches the network or the file system, or reads `HOME`/`TMPDIR`. That would be a crossing, and an env read that security-plan §Secret Management and §API Security have not registered.
- A new agent-run event field that holds test output, panic text, a fixture's markup or a URL. It would break the "no content-named field" reading in security-plan §Logging & Monitoring (agent-run row), and `run.log` must stay unprinted by `logs`.
- An assertion message or test name in a moved check that carries an element id, accessible name or typed text. A refusal holds nothing a call supplied (per security-plan §Error Handling). The test-name and assertion-message wording lives in `.claude/rules/security.md` §Surfaces. Whether the plan itself states it is unconfirmed here.

## Contract bindings
- tests §8 (Network stand-in rows) ↔ security-plan §Input Validation (`@font-face` source and Fetched fonts rows) and §API Security (Settle and loads in flight row): the shared stand-in must keep both witnesses that those rows cite by file and line, and a moved check re-points the citation.
- tests §3 (agent-run · the bootstrap line) ↔ security-plan §Input Validation (CLI arguments (agent-run.sh) row) and §Logging & Monitoring (agent-run row): any selection change is pinned in `.github/scripts/test_agent_run.py`.
- security-plan §Secret Management (Environment values read) ↔ arch §Occupied Resources: an env read or bind moved into a shared module re-points its citation, and a new one is registered test-only in both.

## Acceptance criteria contributions
- The shared stand-in and fixture modules hold no `std::env`/`env!` read beyond those already registered, and no `UnixListener`/`TcpListener`/`bind(`/`fs::` call (a grep over the shared modules reads 0 new sites) (per security-plan §Secret Management "Environment values read" and §API Security).
- `cargo test -p blitz-tests --test font_face_sniffed_source` and `--test stand_settle` pass, asserting what they asserted before the move, both layout modes (per security-plan §Input Validation `@font-face` source row and §API Security Settle and loads in flight row).
- Every password-fixture check that moved still asserts `MASKED_VALUE` and 0 occurrences of the typed text in the snapshot text, the diff and what the driver returns (per security-plan §Input Validation password and file `input` value row).
- If `scripts/agent-run.sh` changes: a selection outside its closed set still exits 2 before cargo, `events.jsonl` adds no content-named key, and `test_agent_run.py` passes (per security-plan §Input Validation CLI arguments (agent-run.sh) row and §Logging & Monitoring agent-run row).
