# security-plan · the rows that bind 2026-11-15-browser-corrective — picked 5 of 550 (3 by every draw) · rule any · pin 1111

## threat-model-summary · ## Threat Model Summary
decision/file-url-only-for-file-documents · File URLs load only for file documents — A file: URL loads only for a file: document or a top-level load; for any other document, remote pages included, script fetch, module import and script src fail, blitz-net refuses engine loads before a disk read, and the browser app names no file: favicon and drops a file: navigation silently.
decision/default-script-fetcher-local-only · Default script fetcher takes local URLs only — The default script fetcher takes only `file:` and `data:` URLs; a `file:` URL loads only for a `file:` document, and fetch(), a module import and a script src are each refused otherwise with a fetch error. · why: A narrowing of what a document's scripts may fetch.

## input-validation · ## Input Validation
decision/file-urls-only-from-file-documents · file URLs load only from file documents — A file: URL is read only for a file: document or a top-level load: a script fetch, module import or script src, every engine stylesheet, @import, font, image or iframe load, and in the browser app a favicon and a link navigation; blitz-net refuses before a disk read, a navigation drops silently. · why: A narrowing, not a boundary widening. · guard: remote_document_cannot_load_file_urls

## api-security · ## API Security
decision/external-targets-to-system-browser · External targets open in the system browser — Navigation to external targets goes to the system browser.

## error-handling · ## Error Handling
contract/blitz-net-provider-error/file-forbidden · blitz-net provider errors · FileForbidden — The refusal of a file: load for a non-file document has the fixed display text «file: load refused for a non-file document», holding no URL or path, and reaches the handler only through the existing error path with no log site of its own.

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
