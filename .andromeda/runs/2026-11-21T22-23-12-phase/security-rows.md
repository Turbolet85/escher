# security-plan · the rows that bind 2026-11-21-dependency-and-ci-debts — picked 28 of 570 (24 by every draw) · rule any · pin 2957

## authentication-authorization · ## Authentication & Authorization
decision/ci-token-read-only · Read-only CI tokens — CI tokens are scoped by GitHub Actions permissions per workflow: ci.yml declares a workflow-level contents read permission and no job-level grant, so every ci.yml job's GITHUB_TOKEN is read-only. · guard: test_ci_workflows.py

## input-validation · ## Input Validation
decision/sql-values-bound-by-placeholders · SQL values bound by placeholders — Persistence statements bind every value through placeholders.
decision/sqlite-bound-parameters · SQLite values bound through placeholders — Values written to the sqlite persistence are bound through `params!` placeholders, never spliced into SQL text.
decision/seeded-source-in-process-seed-only · Seeded source takes its seed from in-process code only — The seeded source of randomness takes its seed only from in-process code: a root context the virtual DOM's builder gives else seed 0, or a script document's building code, whose page Math.random it replaces; outside any run it seeds from the host's random keys, reading no env var, file or carrier.

## data-protection · ## Data Protection
decision/history-in-sqlite · Browser history stored in sqlite — Browser history is written to a sqlite file; on mobile it is in-memory only and does not persist across launches.
decision/clipboard-via-winit · Clipboard through winit — Clipboard text is read and written through winit; arboard is no dependency and the Dioxus native default feature set carries no clipboard feature.

## api-security · ## API Security
decision/ci-api-via-gh-cli · CI API calls through the gh CLI — CI API calls go through the `gh api` CLI with the workflow token.
decision/ci-github-api-through-gh · CI GitHub API calls through gh — CI calls the GitHub API through the gh CLI, with GH_TOKEN taken from the workflow token secret.
decision/session-socket-sole-listener · Session socket as the one listener — The driver session's Unix-domain socket is the one listener of every default build, serving the session lifecycle and driver calls and their answers.

## dependency-security · ## Dependency Security
decision/advisory-ignores-by-id · Advisory ignores by ID only — An advisory is ignored only by its ID, each with a written reason that names its owner as a residual row by its id, with no blanket allow and unmaintained and unsound at their defaults; no product file names the residuals file.
decision/audit-ignores-per-id-with-reason · Audit ignores per ID with a written reason — Audit ignores are per advisory ID only, each with a written reason, no blanket allow, and each ignore is re-read on every push.
decision/cargo-deny-audit-tool · cargo-deny as the audit tool — cargo-deny 0.20.2, with the root deny.toml, is the dependency audit tool.
decision/ci-actions-pinned-to-sha · CI actions pinned to commit SHAs — Every action a CI workflow uses is pinned to a 40-hex commit SHA, with its ref kept as a trailing comment. · guard: WorkflowPinTest in test_ci_workflows.py, reading every workflow file
decision/dependency-audit-cargo-deny · Dependency audit with cargo-deny — cargo-deny checks advisories and licenses, locked, over the whole workspace's resolved graph with all features for the six platforms CI builds, configured by the root `deny.toml`, ignores per ID only, each with a written reason, re-read on every push. · why: Ratified: audit the whole workspace and check licences. · except: `unsound` stays at its default, which keeps memmap2's unsound advisory out of the gate.
decision/dependency-audit-ci-gate · Dependency audit CI gate — The CI audit job runs the dependency audit on every pull request and push, in the slow tier behind the fast jobs, uncached, its log uploaded on failure.
decision/engine-dependency-pins · Engine dependency pins — The pins stay: Stylo 0.24, Taffy the git pin of upstream DioxusLabs/taffy at commit a3997d55, Dioxus 0.7.3 and winit =0.31.0-beta.4.
decision/licence-gate · Licence gate on every push — The audit leg runs upstream's licences table, held byte for byte in the root deny.toml, on every push over the whole workspace's graph, so the fork gates on licences. · why: An open question, decided as the fork's chosen option. · guard: the audit leg · except: upstream's own licenses job stays reverted
decision/local-audit-before-push · Run the audit leg locally before a push — The local fast leg runs no audit, so the audit leg is run locally before a push. · why: A CI audit can go red on advisories published after a green audit with the lock and deny config unchanged.
decision/locked-builds · Locked builds — Every build, the flake, the bundle, every CI cargo leg and every cargo run of the build-directory sweep passes --locked so the lockfile is verified. · except: examples/wasm_hello, a standalone workspace with no Cargo.lock
decision/no-git-dependencies · No git dependencies in the root manifest — The root manifest holds no git dependency; parley and every other crate are crates.io registry versions with checksums in the lockfile. · except: The taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy still passes the lock, audit and MSRV legs.
decision/no-git-dependency · No git dependency — The workspace takes no git dependency. · except: Taffy alone takes a git source: upstream pinned at a full commit rev, no patch section, integrity resting on the rev as its lock entry has no checksum; it still passes the lock, audit and MSRV legs.
decision/skia-default-build-fetch · Skia built by default with a build-time fetch — Skia is built by default in the browser app and the WPT runner, its build script fetching prebuilt binaries from GitHub releases at build time with no content check. · why: A supply-chain widening by upstream merge, ratified.
decision/upstream-sync-dependency-set · Upstream sync takes the dependency set whole — Each upstream sync takes upstream's dependency set whole or, by cherry-pick, a coupled registry set upstream moved, packages entering with it, or a lock-only semver-compatible update; git content only by Taffy's own pin; its lock read name by name, the audit leg green after it.
decision/workspace-inherited-dependency-versions · Workspace-inherited dependency versions — Dependency versions are inherited from the workspace with workspace = true rather than declared inline in each manifest.

## secret-management · ## Secret Management
contract/environment-variables/ci · Environment variables · CI — A flag with no content, read only by the CI leg runner's fast arm and tested only for empty or unset; set, the fast arm runs no build-directory sweep.
decision/ci-secrets-in-actions · CI secrets in GitHub Actions secrets and vars — CI secrets live in GitHub Actions secrets and vars: signing keys, the keystore and its passwords, the Apple certificate, the cross-repo dispatch token and the workflow token are read from them.
decision/ci-secrets-in-github-actions · CI secrets in GitHub Actions — Production secrets live in GitHub Actions secrets and vars and are passed only to the steps that use them.

## error-handling · ## Error Handling
decision/ci-matrix-runs-every-job · CI matrix runs every job — CI matrix jobs run with fail-fast off, so one failing matrix job does not cancel its siblings.

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
absence/served-network-api — A served network API in a default build: no default feature set serves a TCP or UDP port
overview/api-security — No served network API exists in a default build: no default feature set serves a TCP or UDP port. The one default listener is local, the driver session's Unix-domain socket, and the `escher-session mcp` role serves the driver's verbs as MCP tools over its own stdin and stdout, binding nothing, under bounded requests and results. The opt-in devtools feature builds a CDP server listening on 127.0.0.1 with no authentication. The remaining controls govern outbound requests and in-document navigation.

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
