# arch extract

## Relevance
relevant — the chunk changes escher-driver's kept-check form and reader (a Standard Contract), adds replay targets to the workspace's test layout and CI legs, and may place kept checks at a new filesystem path.

## Constraints
- architecture §Established Decisions → [Driver session] (detail `dec-driver-session`) requires the kept check to hold the calls as they crossed, a masked or unresolved `type` as the one fixed stand-in, and each answer as a digest — and records that the answer kept whole, a shot's pixels excepted, is owed at «Recorded-check replay»; the new `answer` key is that owed form, and the master must be amended at the wrap to drop "each answer as a 64-bit digest alone and no screen reading".
- architecture §Established Decisions → [Driver session] requires escher-driver to take its app's boot from its caller and depend on no app (no manifest under `packages/` names an example; the one edge runs `seven_guis → escher-driver`), with `std` only for the recording and no dependency added; a replay runner that becomes public escher-driver API must keep both (whether it does is P4's).
- architecture §Standard Contracts → Driver session requires every call to reach the app only through `Session::run`, which validates first; a replayed step must go through it and nothing else, so "steps re-validated" holds by construction.
- architecture §Standard Contracts → Driver session requires `serde_json` to read MCP requests and kept checks and write nothing — every JSON line (`Outcome::to_json`, a refusal's, the kept check's records) written by the crate's hand-written writer; the `answer` key must be written that way, and `Check::from_bytes` must stay strict (closed keys, bounded at 1,048,576 bytes and 400 records, `Cause::Malformed` with nothing of the file held, no panic path) while admitting `answer`.
- architecture §Standard Contracts → Driver session (detail `contract-driver-session`) fixes the record's key order (`tool` · `arguments` · `masked` · `accepted` · `digest`) and `answer_digest` as FNV-1a 64 over the answer's `to_json()`; the scope places `answer` after `digest`, and the bounds `MAX_CHECK_STEPS` · `MAX_CHECK_BYTES` still end a recording — whether a whole answer per step moves a stand session past 1,048,576 bytes is research's question.
- architecture §Occupied Resources → Filesystem (detail `res-filesystem`) requires a kept check to live as `check-{n}.json` (`0600`, `create_new`) in a `0700` launcher-named directory, never an env read or a default; checks committed into the repository are a new resource and must be registered there at the wrap (and their at-rest mode reconciled), and §Occupied Resources → Environment variables admits no new env var to locate them.
- architecture §Infrastructure Patterns → CI/CD requires the matrix to test windows and macos `--all --tests`; the recording (`Session::recording_into`, the `recording` module) is unix only per §Standard Contracts → Driver session, so a replay target must compile and read green off unix — whether `Check::from_bytes` is reachable off unix is research's question.

## Patterns to follow
- One behaviour per integration-test file, opened by a `//!` doc, auto-discovered under `tests/blitz-tests/tests/` (no `autotests`, no new `[[test]]` table), scenarios run `for incremental in [false, true]`, and shared session helpers kept in `tests/session_common/mod.rs` (no test target) — per architecture §Conventions → Tests; the target list is pinned by `.github/scripts/test_blitz_tests_targets.py`.
- The stand's own replay round trip as built — `stand_session_recording` records the four lean tasks, reads them back and runs them again with each step's acceptance and digest equal in both layout modes — per architecture §Existing Scopes → blitz-tests; the binary's CLI and MCP recording checks sit in `examples/seven_guis/tests/recording.rs` per §Existing Scopes → seven_guis.
- Session checks' scratch state under `CARGO_TARGET_TMPDIR` (`target/tmp/`), short names so a socket path fits — per architecture §Occupied Resources → Filesystem.
- The `stand-repeat` leg holds the stand checks to a zero-failure budget over three rounds, restoring the test job's cache and never saving — per architecture §Infrastructure Patterns → CI/CD and §Stack and Technologies → Code quality.

## Anti-patterns to avoid
- A `Serialize` derive or any `serde_json` write in escher-driver for the `answer` key or a replay report (per architecture §Standard Contracts → Driver session).
- A dependency from escher-driver on `seven_guis`, a stand type or any app to run a replay (per architecture §Established Decisions → [Driver session]).
- A new env var, default path or log field that locates or carries a kept check or its answers — the kept check is never logged and named by no tool call, verb, argument or result field (per architecture §Occupied Resources → Filesystem and §Standard Contracts → Driver session).

## Contract bindings
- arch §Standard Contracts → Driver session ↔ security: the `answer` key puts ids, names, values, snapshot text and diffs at rest — the ratified F3 B widening (inputs#I4) on this chunk's wrap card; committed checks' at-rest mode against the owner-only kept-check rule is the security plan's question.
- arch §Occupied Resources → Filesystem ↔ tests: committed checks as test input and a new replay target bind to the test plan's §3 agent-run contract (`agent-run.sh run stand` picking up `stand_*`), the `stand-repeat` leg and `test_blitz_tests_targets.py`.
- arch §Standard Contracts → Driver session ↔ obs: the `command` span's eight fields and the never-logged rule stay unchanged under replay; no field holds an answer.

## Acceptance criteria contributions
- escher-driver's manifest gains no dependency and no `[features]` table, and the `answer` key is written by the crate's hand-written JSON writer with `serde_json` still a reader only (per architecture §Established Decisions → [Driver session]).
- Every replayed step runs through `Session::run` and nothing else, and `Check::from_bytes` admits `answer` after `digest` while still refusing any other unknown key `malformed` (per architecture §Standard Contracts → Driver session).
- Each new replay target is one file directly under `tests/blitz-tests/tests/` (or a seven_guis test file), auto-discovered, both layout modes, pinned by `test_blitz_tests_targets.py`, and the windows and macos matrix legs read green with it (per architecture §Conventions → Tests and §Infrastructure Patterns → CI/CD).
- Any committed kept check's path is registered in §Occupied Resources → Filesystem at the wrap, and no new env var is read to find it (per architecture §Occupied Resources → Filesystem).
