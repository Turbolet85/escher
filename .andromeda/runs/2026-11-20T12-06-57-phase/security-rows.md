# security-plan · the rows that bind 2026-11-20-browser-s-surviving-mutants — picked 14 of 567 (8 by every draw) · rule any · pin 2634

## threat-model-summary · ## Threat Model Summary
decision/default-script-fetcher-local-only · Default script fetcher takes local URLs only — The default script fetcher takes only `file:` and `data:` URLs; a `file:` URL loads only for a `file:` document, and fetch(), a module import and a script src are each refused otherwise with a fetch error. · why: A narrowing of what a document's scripts may fetch.
decision/file-url-only-for-file-documents · File URLs load only for file documents — A file: URL loads only for a file: document or a top-level load; for any other document, remote pages included, script fetch, module import and script src fail, blitz-net refuses engine loads before a disk read, and the browser app names no file: favicon and drops a file: navigation silently.
decision/script-fetcher-file-data-only · Script fetcher takes file and data URLs only — The default script fetcher supports only file: and data: URLs; any other scheme returns UnsupportedScheme.
decision/browser-javascript-opt-in · Browser JavaScript opt-in — In the browser app JavaScript execution is opt-in, behind the non-default javascript feature.
decision/javascript-opt-in · JavaScript opt-in in the browser app — In the browser app JavaScript runs only behind the non-default `javascript` feature.

## input-validation · ## Input Validation
decision/default-fetcher-file-and-data-only · Default script fetcher takes file and data only — The default script fetcher takes only file: and data: URLs.
decision/driver-installs-no-subscriber · Driver installs no subscriber — The driver crate and its command line install no tracing subscriber and read no environment variable; clocks are read only by the host's idle expiry, the MCP server on a request, the session's bounded load wait, and the client's bounded stop and ready waits. · except: An acting step runs under escher-telemetry's scoped log capture, which prints nothing.
decision/driver-no-subscriber-no-env · Driver installs no subscriber and reads no env — The driver crate and its session command line install no tracing subscriber and read no environment variable. · except: An acting step runs under escher-telemetry's scoped log capture, which prints nothing.
decision/file-urls-only-from-file-documents · file URLs load only from file documents — A file: URL is read only for a file: document or a top-level load: a script fetch, module import or script src, every engine stylesheet, @import, font, image or iframe load, and in the browser app a favicon and a link navigation; blitz-net refuses before a disk read, a navigation drops silently. · why: A narrowing, not a boundary widening. · guard: remote_document_cannot_load_file_urls

## data-protection · ## Data Protection
decision/cookies-and-cache-default-features · Cookies and HTTP cache on by default — Cookies and the HTTP cache are default features of the browser.

## api-security · ## API Security
decision/session-socket-sole-listener · Session socket as the one listener — The driver session's Unix-domain socket is the one listener of every default build, serving the session lifecycle and driver calls and their answers.

## dependency-security · ## Dependency Security
decision/locked-builds · Locked builds — Every build, the flake, the bundle, every CI cargo leg and every cargo run of the build-directory sweep passes --locked so the lockfile is verified. · except: examples/wasm_hello, a standalone workspace with no Cargo.lock

## error-handling · ## Error Handling
decision/parallel-style-traversal-serialised · Parallel style traversals serialised — Every parallel style traversal takes one process-wide lock, its poison recovered, so two documents resolving parallel at once are serialised. · why: Two concurrent parallel traversals otherwise panic `already mutably borrowed`; the lock adds no panic path.

## logging-monitoring · ## Logging & Monitoring
decision/no-screen-content-in-logs · Screen content stays out of logs — No driver call or in-process read puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer, a layout answer, a coverage check and a step's caused text reach no log line at any level. · guard: stand_act_spans, host_log and mcp_log, read at RUST_LOG=trace under both builds

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
