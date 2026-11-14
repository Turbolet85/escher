# security-plan · the rows that bind 2026-11-14-remote-pages-local-files — picked 13 of 550 (8 by every draw) · rule any · pin 984

## threat-model-summary · ## Threat Model Summary
decision/file-url-only-for-file-documents · File URLs load only for file documents — A file: URL loads only for a file: document or a top-level load; for any other document, remote pages included, JS fetch, module import and script src refuse it with a FetchError, and blitz-net refuses the engine's stylesheet, @import, font, image and iframe loads before any disk read.

## input-validation · ## Input Validation
decision/attribute-readers-no-log-or-panic · Markup attribute readers without log or panic — Markup attributes, remote pages included, are read from parsed content by in-process readers with no log site or panic path, ARIA states and closed values through closed parses carrying no attribute text.
decision/button-click-focuses · A button click focuses the button — A click on a `button` of any type or an `input` of type `submit`, `reset` or `button` focuses it and submits its form owner when it is a submit button, a disabled form control stopping the walk first. · why: An engine change for every document and a boundary widening, ratified.
decision/enter-activates-focused-control · Enter activates the focused control — A pressed `Enter` on the focused node dispatches a click on it when it is no disabled form control and is a `button`, a `summary`, an `input` of type `submit`, `reset` or `button`, or an `a` with `href`. · why: An engine change for every document and a boundary widening, ratified.
decision/file-urls-only-from-file-documents · file URLs load only from file documents — A script fetch, module import or script src, and every engine stylesheet, @import, font, image or iframe load, may read a file: URL only for a file: document or a top-level load, in every document, remote pages included; blitz-net refuses any other before a disk read. · why: A narrowing, not a boundary widening. · guard: remote_document_cannot_load_file_urls

## api-security · ## API Security
decision/external-targets-to-system-browser · External targets open in the system browser — Navigation to external targets goes to the system browser.
decision/form-dialog-method-rejected · Form submission by scheme and method — Form submission goes by scheme and method, and the `dialog` method is rejected.
decision/outbound-http-through-blitz-net · Outbound HTTP through blitz-net — The apps' outbound HTTP goes through blitz-net with http2, and in rdme through reqwest.
decision/outbound-http-via-reqwest-or-blitz-net · Outbound HTTP through reqwest or blitz-net — Outbound HTTP in the examples and apps goes through reqwest or blitz-net.
decision/subresource-requests-get-only · Sub-resource requests are GETs — Sub-resource requests are issued as GETs only.

## error-handling · ## Error Handling
decision/error-pages-skip-history-and-favicon · Error pages skip history and favicon probing — Error pages are flagged is_error and skip history recording and favicon probing. · why: The browser tab's load handler reads is_error to skip record_visit and the favicon probe for synthesized 404 and error pages
decision/failed-fetch-error-page-shows-debug-text · Failed fetch shows the error page with Debug text — A failed fetch shows the user error.html with the error's Debug text in its error paragraph. · why: The browser app's DocumentLoader shows this page to the user on every failed navigation, the error's Debug text written into the page's error paragraph
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
