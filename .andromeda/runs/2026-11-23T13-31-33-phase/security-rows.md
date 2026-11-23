# security-plan · the rows that bind 2026-11-23-canvas-shot-across-hosts — picked 71 of 577 (35 by every draw) · rule any · pin 3354

## threat-model-summary · ## Threat Model Summary
decision/canvas-content-crosses-as-shot-pixels · Canvas content crosses only as shot pixels — A canvas's app-painted content, a widget painted from Rust, crosses only as pixels in a screenshot on every ratified carrier, the model provider on a cold-agent live run included, with no new carrier, field, cause, session error, exit, log field or span, and a kept check keeps no pixels. · why: A boundary widening, ratified. · guard: stand_canvas_shot
decision/driver-session-unix-socket-only · Driver session on one Unix socket — The driver session socket binds one Unix-domain socket in a caller-named state directory, never a TCP or UDP port, and exists on unix only, while an in-process session runs on linux, windows and macOS alike.

## authentication-authorization · ## Authentication & Authorization
decision/ci-token-read-only · Read-only CI tokens — CI tokens are scoped by GitHub Actions permissions per workflow: ci.yml declares a workflow-level contents read permission and no job-level grant, so every ci.yml job's GITHUB_TOKEN is read-only. · guard: test_ci_workflows.py
decision/publish-job-repository-guard · Publish job repository guard — The publish job is repository-guarded so no fork ref reaches its "Signed Builds" environment or its secrets.
decision/publish-secrets-upstream-main-only · Publish secrets reachable only upstream — The publish job runs only in DioxusLabs/blitz and uses the Signed Builds environment only on main or ci-test branches, so no fork ref reaches the environment or its secrets.

## input-validation · ## Input Validation
decision/boundary-widening-rule · Boundary widening rule — A new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
decision/custom-widget-content-box-frame · Custom widget painted and pointed in its content box — Behind the custom-widget feature, a custom widget's paint size is its content box times scale, floored at 0, its scene sits at the content origin inside the element's transform, unclipped, and its pointer events arrive in its content-box frame while a sub-document's stay in its border-box frame. · why: Ratified; answers upstream issue #1578, and paint size, placement and pointer frame are layout values, not input. · guard: stand_canvas_placement, stand_canvas_pointer_frame
decision/custom-widget-document-font-context · Custom widget handed the document's font context — Behind the custom-widget feature, a custom widget is handed the document's own shared, mutable font context before each paint, and the base document exposes that context publicly, so app widget code holds a live handle to the engine's font context; no input crosses and the default ignores it. · why: Ratified; no input crosses, the handle being the engine's own. · guard: stand_canvas_text
decision/fixture-name-closed-table · Fixture state name checked against a closed table — A fixture state name is compared whole against the app's closed table of named states before any session starts; any other name is refused as an unknown state and nothing is booted.
decision/screen-pixels-in-band · Screen pixels in band — A screenshot's PNG travels inside the answer itself as base64 text, on the command line and as an MCP image block. · why: Ratified as the in-band crossing.
decision/screenshot-in-band · Screenshot carried in band — A screenshot's PNG crosses in band, as base64 inside its JSON answer line or as an MCP image block, the whole screen or one element's visible part, a password input painted masked, under the unchanged answer bound. · why: The in-band image was the ratified choice, a boundary widening.
decision/session-socket-bounds · Session socket bounds — One connection at a time; each read and write under 2 s, a client's read of an answer under 30 s, a request line at most 16,384 bytes, a lifecycle reply 128 bytes, an answer 1,048,576 bytes. · guard: an_answer_read_past_its_bound_is_a_timeout
contract/screenshot-answer/png · Screenshot answer · png — `{"width":…,"height":…,"png":"<base64>"}`, a PNG of the whole screen or of one element's visible part.
decision/closed-driver-verb-schema · Closed driver verb schema — Every driver call is checked against a closed table of verbs and bounded argument kinds and answered with one typed command or one refusal, the first failure in a fixed order; a refused call changes nothing. · why: Each widening of the verb table is a widening of a validated surface, ratified.
decision/closed-value-field-no-widening · A closed-value field in a ratified carrier is no widening — A new field of a closed value set inside an already ratified carrier is not a widening and needs no ratification; a field holding authored text, an open value set, a new carrier or a new exit still does. · why: Ratified as the standing rule.
decision/custom-widget-initial-attribute-replay · Custom widget initial attribute replay — Behind the custom-widget feature, setting a custom widget replays each existing attribute, parsed page content included, to the widget with no old value, so the widget receives them at set-up and checking each value stays the widget's own job. · why: An engine change for every document behind the feature, ratified. · guard: custom_widget_replays_initial_attributes
decision/driver-outcomes-never-logged · Driver outcomes never logged — The driver crate logs and fields no command, call, argument, outcome, refusal, screen text, image, finding, session label or id record; an outcome is written only as a command-line answer, an MCP tool result, a session-socket reply or a recording file the caller asked for. · why: Outcomes leave the process only by the ratified crossing.
decision/hintless-font-face-source-sniffed · Hint-less font-face source fetched and sniffed — An @font-face source whose URL cannot be resolved is skipped; a source with no format hint and no extension is fetched and its format sniffed from its bytes. · why: Ratified as upstream's behaviour, with a check asked for. · guard: font_face_sniffed_source
decision/one-validation-path-for-every-carrier · One validation path for every carrier — The session socket, the command line and MCP each pass a call through the schema's validation before anything runs, and nothing public runs a command that did not come from that validation. · why: The crossing to external input surfaces was ratified with this check as the one they all pass through.
decision/session-label-form · Session label form — A session label is 1 to 32 bytes of a-z, 0-9 and -, checked before the caller's boot runs; it is never document text, and is the one caller string a reply carries.
decision/session-socket-unix-only · Session socket is Unix only — The driver session socket exists on Unix only; elsewhere a session is `Unsupported`.
decision/state-dir-path-is-parameter · Session state path is a parameter — The session state directory path is a caller parameter, never read from the environment or defaulted relative to the working directory.
decision/state-directory-path-is-parameter · State directory path is a parameter — The state directory path is a parameter, never an environment read or a working-directory-relative default.
contract/cli-exit-codes/0 · CLI exit codes · 0 — Accepted: the answer as one line of JSON on stdout.
contract/driver-session-label/format · driver session label · format — A label is 1 to 32 bytes of a-z, 0-9 and -, checked before the caller's boot runs.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The call was accepted: one uncoloured JSON line on stdout holds the answer, or for the coverage role its one coverage line.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — The call was refused: one uncoloured JSON line on stdout holds the refusal, or for the coverage role a malformed refusal.
contract/session-errors/state-dir-too-long · Session errors · state-dir-too-long — The state directory path is too long for a socket address; the message states the bound, 94 bytes on Linux and 90 elsewhere, holds no path, and the remedy names a shorter or relative path; status 3.
contract/session-socket-reply/ok-accepted · Session socket reply · ok v3 accepted — The call ran and was accepted; its answer JSON follows on the line.
contract/session-socket-reply/ok-oversize · Session socket reply · ok v3 oversize — The call ran but its answer is over 1,048,576 bytes.
contract/session-socket-reply/ok-refused · Session socket reply · ok v3 refused — The call was refused; the refusal JSON follows on the line.
contract/session-socket-request/call · Session socket request · call — `call v3 <verb> [<name>=<kind>:<value>]…` runs a validated verb, kinds `t` text, `n` whole number, `f` flag, verb, names and text percent-escaped.

## data-protection · ## Data Protection
decision/committed-counter-checks · Committed stand-Counter checks — The only committed kept checks are the two of the repository's scripted stand-Counter sessions, holding no typed text, host path, name or secret and no `screenshot` step, read only through the strict `Check::from_bytes`. · why: The one exception to a kept check's owner-only medium.
decision/kept-check-answer-whole · Kept check keeps each answer whole — A kept check keeps each answer as the line the driver wrote, masked as on every carrier, with a shot kept as its width and height and never its pixels. · why: Ratified, a shot's pixels excepted.
contract/kept-check-file/contents · Kept check files · contents — A kept check holds per call its verb, arguments, acceptance, digest and masked answer, never a pixel.
decision/kept-check-owner-only · Kept checks are owner-only — A recorded check is written 0600 into a 0700 directory the launcher named, is never logged, and is never removed by escher but for its own check when a recording ends. · why: Ratified. · guard: recording::the_committed_checks_are_what_the_binary_records · except: Two scripted stand-Counter checks, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step.
contract/kept-check-file/entry · Kept check file · entry — A check is a JSON array: an opening record of the app label and the format, then per answered call its verb, its arguments, its acceptance, a FNV-1a 64 digest of its written answer and the answer itself as answer.

## api-security · ## API Security
decision/ci-api-via-gh-cli · CI API calls through the gh CLI — CI API calls go through the `gh api` CLI with the workflow token.
decision/ci-github-api-through-gh · CI GitHub API calls through gh — CI calls the GitHub API through the gh CLI, with GH_TOKEN taken from the workflow token secret.
decision/scrollbar-fade-on-document-clock · Scrollbar fade reads the document clock — The overlay scrollbar fade reads the document's clock, so on a harness-built document two shots of one state read the same pixels until an advance moves the fade; a document built with no clock still fades by wall time. · why: Ratified.
decision/time-moves-only-on-advance · Time moves only on advance — Time on a held instance moves only when a caller asks for it with advance, 1 to 60000 ms a call; no driver verb has an argument or result field of a wait, and snapshot, screenshot, lint, graph and actions wait on no load.
contract/session-socket/oversize · Driver session socket · oversize — An answer is at most 1,048,576 bytes on its JSON line; over it the reply is oversize, read by the client as AnswerTooLarge.
decision/no-wait-verb-fields · No wait argument or result on verbs — No driver verb has an argument or a result field of a wait.

## dependency-security · ## Dependency Security
decision/harness-blitz-paint-custom-widget · Harness turns on blitz-paint's custom-widget feature — blitz-test-harness turns on blitz-paint's custom-widget feature, so blitz-paint's widget paint pass is compiled into the harness, the driver and the escher-session binary in every build selection that reaches the harness, adding no package to Cargo.lock. · why: So a shot paints an app's widget in every build selection. · guard: stand_canvas_shot
decision/local-audit-before-push · Run the audit leg locally before a push — The local fast leg runs no audit, so the audit leg is run locally before a push. · why: A CI audit can go red on advisories published after a green audit with the lock and deny config unchanged.
decision/no-base64-crate · No base64 crate — No base64 crate is taken as a dependency; a shot's base64 is written by hand.
decision/png-encodes-screenshots · png encodes a screenshot — The png crate is what encodes a screenshot in escher-driver, and blitz-tests takes it only as a dev-dependency.
decision/png-shot-encoder · png as the shot encoder — The png crate is the encoder of a shot in escher-driver.
decision/locked-builds · Locked builds — Every build, the flake, the bundle, every CI cargo leg and every cargo run of the build-directory sweep passes --locked so the lockfile is verified. · except: examples/wasm_hello, a standalone workspace with no Cargo.lock
decision/serde-json-read-only · serde_json, read only — serde_json, with no features and no preserve_order, reads the MCP server's untrusted stdin, a kept check's bytes and the driver's own answer line for replay, and writes nothing. · why: Chosen as read-only use.
decision/serde-json-untrusted-input-reader · serde_json as the untrusted input reader — serde_json, a workspace dependency with no preserve_order feature, is the reader of the MCP server's untrusted stdin, of a kept check's bytes and of the driver's answer line the replay compares.

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project)
decision/telemetry-sink-target-allowlist · Telemetry sink target allowlist — escher's own log sink redacts engine-target messages and fields, redacts the content-named fields of escher targets, and drops every record from a target outside its two prefix sets at every level. · except: the upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed

## secret-management · ## Secret Management
decision/ci-secrets-in-actions · CI secrets in GitHub Actions secrets and vars — CI secrets live in GitHub Actions secrets and vars: signing keys, the keystore and its passwords, the Apple certificate, the cross-repo dispatch token and the workflow token are read from them.
decision/ci-secrets-in-github-actions · CI secrets in GitHub Actions — Production secrets live in GitHub Actions secrets and vars and are passed only to the steps that use them.
decision/live-evidence-holds-no-credentials · Live evidence holds no credentials — No committed live evidence, artifact, event or verdict carries a key, auth header, credential-file name, token field, e-mail address or the operator's login.

## error-handling · ## Error Handling
decision/ci-matrix-runs-every-job · CI matrix runs every job — CI matrix jobs run with fail-fast off, so one failing matrix job does not cancel its siblings.
decision/closed-refusal-cause-set · Closed refusal cause set — A driver command refusal is a typed value whose cause comes from a closed set of eight, with a fault only beside `malformed`; a new verb or argument shape is refused within the set, never by adding a cause. · why: The closed set of eight was ratified.
decision/refusal-closed-cause-set · Command refusal as a closed set of causes — An escher-driver command refusal is a typed value from a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and with no free-text field. · why: So a refusal cannot hold anything a call supplied. · except: A Fault is carried only beside the malformed cause.
decision/refusal-holds-no-call-input · A refusal holds nothing a call supplied — Every refusal's name, meaning and remedy is a fixed static text holding no path, and a fault carries at most the schema's own argument name, so a refusal cannot hold anything a call supplied.
decision/refusal-log-cause-name-only · Only a refusal's cause name is logged — Exactly one thing of a refusal reaches a log: its cause's fixed name, as the `cause` field of the call's driver span; no fault, meaning, remedy or display text is logged.
decision/refusal-logged-by-cause-name · Refusal logged by cause name only — Of an escher-driver command refusal, only its cause's fixed name reaches a log.
contract/refusal-causes/fonts-unavailable · Driver refusal causes · fonts-unavailable — One of the eight closed causes, returned by the executor, with a fixed name, meaning and remedy holding no path.
contract/refusal-json/cause · Refusal JSON · cause — The refusal's cause name, inside `{"refused":{...}}`, built from the cause's fixed strings.
decision/net-handler-error-default-noop · NetHandler error default does nothing — NetHandler::error is a default that does nothing, so a handler sees no failure unless it overrides it. · why: The image, font-face and document-src handlers take the no-op default, while the stylesheet handler and the test harness CountedHandler override error to act on a failure
decision/reach-error-closed-set · Reach errors as a closed set with fixed texts — A reach error is one of a closed set: unknown state, refused at a step with the step's index and its closed refusal cause, or not settled at a step with the busy reading; each reads a fixed text holding nothing a caller supplied. · guard: a_reach_error_reads_fixed_text_naming_no_name_tried
contract/mcp-tool-result/is-error · MCP tool result · isError — A refusal or session error is a tool result `{content:[{type:"text",text}], isError: true}`, its text the same JSON as on the command line, a refused `screenshot` included.

## logging-monitoring · ## Logging & Monitoring
decision/escher-target-content-fields-withheld · escher_ targets withhold content-named fields — In escher's telemetry sink an escher_ target keeps all fields but twelve content-named ones.
decision/escher-target-content-scrub · Escher targets scrub content-named fields — An event whose target starts with escher_ prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which print as [redacted].
decision/no-screen-content-in-logs · Screen content stays out of logs — No driver call or in-process read puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer, a layout answer, a coverage check and a step's caused text reach no log line at any level. · guard: stand_act_spans, host_log and mcp_log, read at RUST_LOG=trace under both builds
contract/command-span/cause · The command span line · cause — Recorded when the call is refused, as a closed cause word such as fonts-unavailable.
contract/escher-session-stdout/command-line · escher-session stdout · command result line — escher-session writes one uncoloured JSON line per command on stdout.

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
