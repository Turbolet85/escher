# security-plan · the rows that bind 2026-11-13-local-files-for-local-documents — picked 33 of 544 (17 by every draw) · rule any · pin 687

## threat-model-summary · ## Threat Model Summary
decision/default-script-fetcher-local-only · Default script fetcher takes local URLs only — The default script fetcher takes only `file:` and `data:` URLs; a `file:` URL loads only for a `file:` document, and fetch(), a module import and a script src are each refused otherwise with a fetch error. · why: A narrowing of what a document's scripts may fetch.
decision/file-url-only-for-file-documents · File URLs load only for file documents — A file: URL loads only for a file: document; for any other document, remote pages included, JS fetch, module import and script src alike refuse it with a FetchError.
decision/script-fetcher-file-data-only · Script fetcher takes file and data URLs only — The default script fetcher supports only file: and data: URLs; any other scheme returns UnsupportedScheme.
decision/iframe-nesting-depth-limit · Iframe nesting depth limit — Iframe nesting depth is limited. · why: Guards against infinitely recursive self-embedding pages.
decision/import-nesting-depth-limit · CSS @import nesting depth limit — CSS @import nesting depth is limited. · why: Prevents unbounded recursion, such as an import URL that grows geometrically per level.
decision/remote-content-nesting-limits · Remote content nesting limits — Remote web content is treated as an attack vector: iframe and @import nesting depths are limited, and stale iframe responses are discarded by request id.

## input-validation · ## Input Validation
decision/boundary-widening-rule · Boundary widening rule — A new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
decision/default-fetcher-file-and-data-only · Default script fetcher takes file and data only — The default script fetcher takes only file: and data: URLs.
decision/failed-stylesheet-blocks-no-render · Failed stylesheet blocks no render — In every document, remote pages included, a stylesheet that fails to load, is empty, is not CSS or is not valid UTF-8 blocks no render. · guard: failed_stylesheet_unblocks_rendering, empty_or_non_css_stylesheet_unblocks_rendering
decision/file-urls-only-from-file-documents · file URLs load only from file documents — A script fetch, a module import or a script src may load a file: URL only for a file: document, in every document, remote pages included. · why: A narrowing, not a boundary widening. · guard: remote_document_cannot_load_file_urls
decision/unresolvable-stylesheet-href-no-load · Unresolvable stylesheet href starts no load — In every document, a stylesheet href that cannot be resolved starts no load and blocks no render. · guard: invalid_stylesheet_url_does_not_block_rendering
decision/attribute-text-leaves-by-ratified-carriers · Attribute text leaves only by ratified carriers — Markup attribute values, names and descriptions leave escher only by the ratified carriers. · except: The page's own form submission and events.
decision/closed-value-field-no-widening · A closed-value field in a ratified carrier is no widening — A new field of a closed value set inside an already ratified carrier is not a widening and needs no ratification; a field holding authored text, an open value set, a new carrier or a new exit still does. · why: Ratified as the standing rule.
decision/hintless-font-face-source-sniffed · Hint-less font-face source fetched and sniffed — An @font-face source whose URL cannot be resolved is skipped; a source with no format hint and no extension is fetched and its format sniffed from its bytes. · why: Ratified as upstream's behaviour, with a check asked for. · guard: font_face_sniffed_source

## api-security · ## API Security
decision/bounded-load-wait · Bounded wait on loads in flight — A harness settle never waits on a load in flight, and the driver's acting step waits on one for at most `LOAD_WAIT`. · why: So the absent request timeout cannot hang a settle or a command.
decision/dioxus-scheme-local-files · Dioxus scheme served from local files — The `dioxus` scheme is served from the local file system.
decision/document-abort-signal-cancels-fetches · Document abort signal cancels its fetches — Aborting the configured AbortSignal cancels every in-flight fetch tied to the document. · why: The browser DocumentLoader aborts the previous page's controller on each navigation and in abort_current, and iframe reloads abort the prior generation
decision/failed-load-ends-wait · A failed load ends the wait — A failed load ends a settle's wait, so a failed stylesheet never leaves a document not settled on Loads for good.
decision/settle-never-waits-on-loads · Settle never waits on a load — A harness settle reports a load in flight at once as not settled with the class Loads, waits on no clock and moves none, and its pass loop is bounded at 64 passes. · why: The absent request timeout cannot hang a settle.
decision/load-wait-bound · Acting commands bounded by LOAD_WAIT — An acting driver command waits on a load in flight for at most LOAD_WAIT, 5 s of wall time, moving no time on the held instance, then returns the load still in flight as settled false with its busy class. · why: The absent request timeout cannot hang a command.
decision/outbound-per-host-limit · Outbound per-host concurrency limit — Concurrent outbound requests are limited to 6 per host.
decision/per-host-request-concurrency · Outbound requests limited per host — At most 6 outbound requests run concurrently per host.
decision/subresource-requests-get-only · Sub-resource requests are GETs — Sub-resource requests are issued as GETs only.

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project)
decision/telemetry-sink-target-allowlist · Telemetry sink target allowlist — escher's own log sink redacts engine-target messages and fields, redacts the content-named fields of escher targets, and drops every record from a target outside its two prefix sets at every level. · except: the upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed

## error-handling · ## Error Handling
decision/net-handler-error-default-noop · NetHandler error default does nothing — NetHandler::error is a default that does nothing, so a handler sees no failure unless it overrides it. · why: The image, font-face and document-src handlers take the no-op default, while the stylesheet handler and the test harness CountedHandler override error to act on a failure
contract/wpt-results/crash · WPT runner results · CRASH — A test that panics is caught and reported as a CRASH result.
contract/wpt-results/fail-uncaught-js-error · WPT runner results · FAIL with Uncaught JS error — A harness test that never reports but had uncaught JS errors yields FAIL with an "Uncaught JS error" subtest.
contract/wpt-results/timeout · WPT runner results · TIMEOUT — A harness test that never reports and had no uncaught JS errors yields TIMEOUT.

## logging-monitoring · ## Logging & Monitoring
contract/resource-load-failure-log/error · Resource load failure log · error — A resource load failure log carries structured field error.
contract/resource-load-failure-log/url · Resource load failure log · url — A resource load failure log carries structured field url.
contract/resource-load-failure-log/waiting-nodes · Resource load failure log · waiting_nodes — A resource load failure log carries structured field waiting_nodes.
contract/network-tracing-log/error · Network tracing log · error — A failed network request log records the failure as field error.
contract/network-tracing-log/url · Network tracing log · url — A network request log records the request URL as field url.

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
