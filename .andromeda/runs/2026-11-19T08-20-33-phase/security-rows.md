# security-plan · the rows that bind 2026-11-19-build-directory-held — picked 41 of 556 (29 by every draw) · rule any · pin 2275

## threat-model-summary · ## Threat Model Summary
decision/browser-javascript-opt-in · Browser JavaScript opt-in — In the browser app JavaScript execution is opt-in, behind the non-default javascript feature.
decision/javascript-opt-in · JavaScript opt-in in the browser app — In the browser app JavaScript runs only behind the non-default `javascript` feature.

## authentication-authorization · ## Authentication & Authorization
decision/ci-token-read-only · Read-only CI tokens — CI tokens are scoped by GitHub Actions permissions per workflow: ci.yml declares a workflow-level contents read permission and no job-level grant, so every ci.yml job's GITHUB_TOKEN is read-only. · guard: test_ci_workflows.py

## input-validation · ## Input Validation
decision/agent-run-selection-allowlist · agent-run.sh selection reaches cargo only as a test stem — agent-run.sh exits 2 with usage before any cargo call on an unknown verb, a wrong argument count or a selection other than the fixed words `stand`, `all` or `session`, matched before the file-stem arm, or an existing test-file stem matching `^[a-z0-9_]+$`.
decision/cold-agent-check-reader-counts-only · Cold-agent recording reader keeps counts only — The cold-agent pipe's reader of its live child's kept checks holds bools and counts only, reads an unreadable directory as no check kept, and never prints or copies a check's content.
decision/cold-agent-checks-never-printed · Cold-agent checks never printed — The cold-agent check file is never printed, and the live child's kept checks are read for booleans and counts only and never printed.
decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg holds three fixed line coverage floors: escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90. · guard: coverage leg
decision/recording-dir-never-named · Recording directory is never exposed — No tool call, verb, argument or result field names the recording directory, and nothing of it is logged. · why: Ratified.
decision/recording-directory-never-named · Recording directory never named or logged — No tool call, verb, argument or result field names the recording directory, and nothing of it is logged.
contract/agent-run-cli/2 · agent-run.sh exit codes · 2 — A verb outside boot run status cleanup logs, a wrong argument count, or a selection other than stand, all, session or an existing test-file stem prints usage and exits 2 before any cargo call.
decision/cold-agent-call-validation · cold-agent.sh call and task allowlist — cold-agent.sh exits 2 with usage on stderr and empty stdout before any precondition or `claude` call on an unknown verb, a wrong argument count or a task outside the allowlist `stand`.
decision/cold-agent-check-validation · Agent-written check validated before replay — Only the reply's last fenced json block is taken, at most 65,536 bytes and 400 steps, each step of closed keys and a known verb; a check outside these is never replayed, and the check is never printed. · why: A boundary widening to a new input class, ratified.
decision/coverage-floor-input-validation · Coverage floor checker validates and prints no paths — coverage_floor.py, stdlib only, refuses a malformed or repeated floor or a crate with no record with exit 2, and prints only crate directories and counts, never a source or lcov path. · guard: CoverageFloorTest
decision/recording-dir-owner-only · Recording directory is owner-only — The recording directory is created mode 0700 and a present one open to group or others is refused `record-dir-not-private` with status 3, nothing written and nothing spawned. · why: Ratified.
decision/recording-directory-owner-only · Owner-only recording directory — The recording directory is created 0700, a group- or other-open one is refused with nothing written or spawned, each check is created 0600 and never replaced, and recording is unix only.

## data-protection · ## Data Protection
decision/cold-agent-checks-read-for-counts-only · Cold-agent checks read for bools and counts only — The cold-agent pipe's live child keeps its checks in an owner-only record directory, read by the pipe for bools and counts only, never printed, copied or committed, and removed by its cleanup.
decision/cold-agent-records-private · Cold-agent recordings stay private — The cold-agent pipe records into an owner-only directory, never prints, copies or commits the checks, reads them for bools and counts only and removes them with its state on cleanup.
decision/kept-check-owner-only · Kept checks are owner-only — A recorded check is written 0600 into a 0700 directory the launcher named, is never logged, and is never removed by escher but for its own check when a recording ends. · why: Ratified. · guard: recording::the_committed_checks_are_what_the_binary_records · except: Two scripted stand-Counter checks, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step.
decision/kept-checks-owner-only · Kept checks owner-only and uncommitted — A kept check stays in an owner-only medium, is never logged and is never committed. · except: The two committed checks of the repository's own scripted stand-Counter sessions.
contract/kept-check-file/lifecycle · Kept check files · lifecycle — A kept check is never logged and never removed by escher, but for its own check when a recording ends.
contract/kept-check-file/medium · Kept check files · medium — Under `--record <dir>`, `Session::recording_into` or an exploration's record directory, one `check-{n}.json` with mode `0600` per recorded session, each explored path its own, is written in a `0700` directory the launcher named.

## api-security · ## API Security
decision/session-socket-sole-listener · Session socket as the one listener — The driver session's Unix-domain socket is the project's one listener, serving the session lifecycle and driver calls and their answers.

## dependency-security · ## Dependency Security
decision/blitz-tests-not-published · blitz-tests not published — The blitz-tests crate is never published (publish = false).
decision/cargo-llvm-cov-coverage-tool · cargo-llvm-cov as the coverage tool — cargo-llvm-cov 0.9.1 is the coverage tool installed in CI.
decision/ci-actions-pinned-to-sha · CI actions pinned to commit SHAs — Every action a CI workflow uses is pinned to a 40-hex commit SHA, with its ref kept as a trailing comment. · guard: WorkflowPinTest in test_ci_workflows.py, reading every workflow file
decision/embolden-locked-fast-leg · font-embolden tested by a locked fast leg — The blitz-paint font-embolden feature is tested by a locked cargo leg among the fast legs, with no CI job of its own.
decision/locked-builds · Locked builds — Every build, the flake, the bundle and every CI cargo leg, passes --locked so the lockfile is verified. · except: examples/wasm_hello, a standalone workspace with no Cargo.lock
decision/no-git-dependency · No git dependency — The workspace takes no git dependency. · except: Taffy alone takes a git source: upstream pinned at a full commit rev, no patch section, integrity resting on the rev as its lock entry has no checksum; it still passes the lock, audit and MSRV legs.
decision/test-harness-no-tracing-no-style · Test harness takes no tracing and no style — blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
decision/workspace-inherited-dependency-versions · Workspace-inherited dependency versions — Dependency versions are inherited from the workspace with workspace = true rather than declared inline in each manifest.
decision/dependency-audit-cargo-deny · Dependency audit with cargo-deny — cargo-deny checks advisories and licenses, locked, over the whole workspace's resolved graph with all features for the six platforms CI builds, configured by the root `deny.toml`, ignores per ID only, each with a written reason, re-read on every push. · why: Ratified: audit the whole workspace and check licences. · except: `unsound` stays at its default, which keeps memmap2's unsound advisory out of the gate.

## secret-management · ## Secret Management
decision/ci-secrets-in-github-actions · CI secrets in GitHub Actions — Production secrets live in GitHub Actions secrets and vars and are passed only to the steps that use them.
decision/live-evidence-holds-no-credentials · Live evidence holds no credentials — No committed live evidence, artifact, event or verdict carries a key, auth header, credential-file name, token field, e-mail address or the operator's login.
decision/ci-secrets-in-actions · CI secrets in GitHub Actions secrets and vars — CI secrets live in GitHub Actions secrets and vars: signing keys, the keystore and its passwords, the Apple certificate, the cross-repo dispatch token and the workflow token are read from them.
decision/live-runs-use-operator-login · Live runs use the operator's own login — The cold-agent live run and the MCP live trial reach the model provider only through the operator's own Claude Code login held outside the repository: no API key, no env read by the pipe, no CI secret, no new secret source and no auth library. · why: Ratified.

## error-handling · ## Error Handling
decision/ci-matrix-runs-every-job · CI matrix runs every job — CI matrix jobs run with fail-fast off, so one failing matrix job does not cancel its siblings.

## logging-monitoring · ## Logging & Monitoring
decision/agent-run-raw-log-unprinted · Agent-run raw log kept unprinted — The agent-run raw cargo and libtest output log is unscrubbed, never printed by logs, and gitignored.
decision/cold-agent-record-unprinted · Cold-agent record kept unprinted — The cold-agent server log, check, raw transcript and the live child's kept checks are never printed; the transcript stays unscrubbed and gitignored, and each committed copy has its host paths masked.
decision/cold-agent-recordings-owner-only · Cold-agent recordings owner-only — The cold-agent live server records its kept checks into an owner-only directory, read for booleans and counts only, never printed, copied or committed, and removed at cleanup; the replay children run without recording.
decision/cold-agent-server-output-unprinted · Cold-agent server output never printed — The cold-agent pipe reads its server's stderr log only by counting span lines and writes the agent's check as parsed, printing or copying neither; the replay children's stderr goes to /dev/null.
decision/cold-agent-transcript-raw · Cold-agent raw transcript kept unscrubbed — The cold-agent session's raw transcript stays unscrubbed by design, is never printed and is gitignored; each committed copy has its host paths, the session dir and the home prefix, masked.

# security-plan · its absences, names and overviews, whole

## threat-model-summary · ## Threat Model Summary
> NOT YET MEASURED — no slice states an application-level threat model; data classification, auth model, infrastructure and compliance triggers were not gathered for this summary (the stored-data and auth facts the reading did reach are in the sections below).
absence/same-origin-cors-checks — Same-origin and CORS checks on script fetches

## authentication-authorization · ## Authentication & Authorization
absence/application-authentication — Application authentication or authorization
absence/authentication-authorization — Authentication or authorization code

## input-validation · ## Input Validation
absence/dioxus-native-form-validation — Form data validation in the Dioxus native bridge: form data always reads valid
absence/same-origin-and-cors-checks — Same-origin and CORS checks on script fetches
absence/url-taking-driver-verb — A driver verb that takes a URL

## data-protection · ## Data Protection
> NOT YET MEASURED — HSTS, TLS version requirements and certificate pinning: no slice gathered them. ¶ > NOT YET MEASURED — key management, backups, retention periods and anonymization: no slice gathered them.
absence/database-encryption — Encryption of the history database

## api-security · ## API Security
absence/content-security-policy — CSP enforcement
absence/cors-checks — CORS checks on outbound requests
absence/cors-same-origin-policy — A CORS or same-origin policy
absence/request-timeout — A request timeout on outbound requests
absence/response-body-size-limit — A response body size limit on outbound requests
absence/response-size-limit — A response size limit on outbound requests
absence/same-origin-checks — Same-origin checks on outbound requests
absence/served-network-api — A served network API: no binary serves a TCP or UDP port
overview/api-security — No served network API exists: no binary serves a TCP or UDP port. The one listener is local, the driver session's Unix-domain socket, and the `escher-session mcp` role serves the driver's verbs as MCP tools over its own stdin and stdout, binding nothing, under bounded requests and results. The remaining controls govern outbound requests and in-document navigation.

## dependency-security · ## Dependency Security
> NOT YET MEASURED — a critical-CVE response SLA: none is stated. The lockfile's advisory state is read on every push by the audit leg, within cargo-deny's resolved graph (Audit tool above). ¶ Supply chain integrity › > NOT YET MEASURED — SBOM generation and base image scanning: no slice gathered them.
absence/automated-dependency-updates — Automated dependency update tooling, such as dependabot
absence/signed-artifacts — Signed release artifacts produced by the fork

## secret-management · ## Secret Management
> NOT YET MEASURED — development secret storage (beyond the fact that the cold-agent run's login is held by the `claude` CLI; its at-rest location is unmeasured), secret scanning in CI, rotation cadence and access auditing: no slice gathered them.

## error-handling · ## Error Handling
absence/error-reporting-integration — An error reporting integration

## logging-monitoring · ## Logging & Monitoring
> NOT YET MEASURED — security-event logging (auth, authorization or validation failures), log retention, access controls and tamper evidence: no slice gathered them.
absence/monitoring-and-alerts — Monitoring beyond the stderr log: no metrics, no alerting and no telemetry export

## compliance-controls · ## Compliance Controls
> NO RECORDED INTENT

## security-anti-patterns-never-do-these · ## Security Anti-Patterns (NEVER do these)
> NO RECORDED INTENT

## security-decisions-log · ## Security Decisions Log
> NO RECORDED INTENT
