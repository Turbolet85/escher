# security extract

## Relevance
partial — the coverage floor and flakiness budget are test-plan matters; security reaches this chunk through folded CARRYs 1 (audit reach), 5 (apt-install), 6 (licence gate), 9 (hint-less `@font-face` check) and 10 (command-line closed pipe and stderr by level), and through every CI job or tool the gates add.

## Constraints
- Closing the audit's reach (CARRY 1) must keep the audit discipline: advisories ignored by ID with a written reason only, no blanket allow, `unmaintained` and `unsound` kept at their defaults. A scan that reaches RUSTSEC-2024-0436 (paste) and RUSTSEC-2026-0186 (memmap2) must settle each by a semver-compatible update or a reasoned per-ID ignore (per security-plan §Dependency Security, Audit tool). Whether cargo-deny still prunes `http-cache` and `ravif` from its graph at HEAD is research's question.
- Whether the fork gates on licences, and over which graph, is recorded as open and owned by this route entry. The plan records one reading and no intent, so no extract here may present a licence gate as already required (per security-plan §Dependency Security, Supply chain integrity — Licence compliance).
- A check of ours on the hint-less, extension-less `@font-face` source (a `data:` URL whose format is sniffed from its bytes) is owed before 0.1.0 ships, and this entry owns it. The plan also requires that escher route no agent- or user-supplied URL through this path (per security-plan §Input Validation, `@font-face` source row).
- Any tool a gate adds to ci.yml (a coverage threshold reader, a retry or repeat runner, a lockfile scanner) must be pinned the way ci.yml already pins its tools: every `uses:` at a 40-hex commit SHA with its ref kept as a comment, tools installed at exact versions through `taiki-e/install-action`, and every cargo leg run with `--locked` (per security-plan §Dependency Security, Pinning — CI tooling; Lockfile verification).
- ci.yml's `GITHUB_TOKEN` must stay read-only. A new coverage, flake or audit job takes the workflow-level `permissions: contents: read` and adds no job-level grant (per security-plan §Authentication & Authorization, RBAC row for ci.yml).
- The command line's two unnamed paths (CARRY 10) are mandated behaviour that a named check must hold: a write to a closed pipe is dropped, not a panic, and no ending writes back anything the caller typed (per security-plan §Input Validation, CLI arguments (escher-session) row). Whether the code already passes both under a check is research's question.
- Any stderr reading by level (CARRY 10's client stderr, the host on more flows, the windowed binary under a workspace build) and any line carrying `busy` (CARRY 4) must stay inside the sink's allowlist. Non-allowlisted targets are dropped whole, and the command span keeps its eight fixed-word and count fields, `busy` being the schema's class word (per security-plan §Logging & Monitoring, Log format and backends).

## Patterns to follow
- A per-ID `deny.toml` ignore with a written reason, as the one standing RUSTSEC-2026-0192 ignore is recorded, re-read by the audit leg on every push (per security-plan §Dependency Security, Audit tool).
- A hand review of any dependency or tool a gate brings in: lock delta, licence, repository, build script, proc-macro flag, the audit leg green with no ignore added (per security-plan §Dependency Security, Pinning).
- A stderr census for a new by-level reading: 0 id needles, 0 name needles and 0 typed-sentinel occurrences at `RUST_LOG=trace` under both builds, as `host_log` and `mcp_log` take it (per security-plan §Input Validation, `aria-label` · `<label for>` row).
- An apt-install.sh change for an interrupted dpkg (CARRY 5) must keep three properties: a bad call exits 2 before `sudo`, `timeout` or `apt-get` runs; the script reads no env var of its own and writes no file; and it names no secret. All three stay pinned by `InstallScriptTest.test_a_bad_call_exits_2_before_anything_runs` (per security-plan §Input Validation, CLI arguments (apt-install.sh) row).

## Anti-patterns to avoid
- Turning a widened audit green by a blanket allow, by setting `unmaintained` or `unsound` to none, or by an ignore with no written reason (per security-plan §Dependency Security, Audit tool).
- A font-face check, or any new fixture, that fetches a remote or caller-supplied URL through blitz-net. The source should be an inline `data:` URL, offline, and must not add a driver verb that takes a URL (per security-plan §Input Validation, `@font-face` source row).
- Putting call-supplied or screen content (an id, a name, typed text, a shot's image or size) in a span field, a log line, a refusal, a test name or an assertion message. This applies equally to a flake or repeat runner's own output (per security-plan §Input Validation, Driver command schema row; §Error Handling, Error format).

## Contract bindings
- CI security gate ↔ test-plan §9 CI legs and `.github/scripts/test_ci_workflows.py`. The SHA-pin and read-only-permissions assertions there must cover any job or leg the gates add.
- Log redaction ↔ obs-plan §3, the sink allowlist and the command span's fields. CARRY 4's `busy` reading and CARRY 10's stderr readings are taken through it.
- `@font-face` check ↔ test-plan §3, one integration-test file per target, the blitz-tests target pin `.github/scripts/test_blitz_tests_targets.py`.
- Licence gate, audit-reach widening and any new CI gate ↔ the founder dialog the scope requires (inputs#I1). The security plan supplies no lean.

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh audit` passes. `deny.toml` holds per-ID ignores only, each with a reason, with `unmaintained` and `unsound` at defaults. The closed reach demonstrably evaluates paste and memmap2: a recorded run shows each advisory either reported or settled (per security-plan §Dependency Security, Audit tool).
- A named check of ours loads an `@font-face` source with no format hint and no URL extension from an inline `data:` URL, offline, and asserts it is fetched and its format sniffed (per security-plan §Input Validation, `@font-face` source row).
- `.github/scripts/test_ci_workflows.py` passes with every added `uses:` pinned to a 40-hex SHA, every added tool installed at an exact version, every added cargo command `--locked`, and no job-level permission grant in ci.yml (per security-plan §Dependency Security, Pinning; §Authentication & Authorization, RBAC row for ci.yml).
- A named check runs `escher-session` with stdout closed early and asserts no panic. The same check asserts that no stdout or stderr line holds a caller-typed sentinel (per security-plan §Input Validation, CLI arguments (escher-session) row).
