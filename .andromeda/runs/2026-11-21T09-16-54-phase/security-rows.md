# security-plan · the rows that bind 2026-11-21-engine-debts — picked 71 of 570 (37 by every draw) · rule any · pin 2822

## threat-model-summary · ## Threat Model Summary
decision/bootstrap-native-helper-removed · Bootstrap native helper hidden from pages — The URLSearchParams bootstrap's native search-params helper is deleted from globalThis before any page script runs.
decision/browser-javascript-opt-in · Browser JavaScript opt-in — In the browser app JavaScript execution is opt-in, behind the non-default javascript feature.
decision/caused-text-crossing · Caused text crossing — Each acting verb's answer carries caused on every ratified carrier when the step caused something: one item per panic, boundary error and kept record with its message, location, boundary, cause chain and fields, error as written within the text bound, every other content-named field redacted. · why: A boundary widening to the error text, ratified.
decision/default-script-fetcher-local-only · Default script fetcher takes local URLs only — The default script fetcher takes only `file:` and `data:` URLs; a `file:` URL loads only for a `file:` document, and fetch(), a module import and a script src are each refused otherwise with a fetch error. · why: A narrowing of what a document's scripts may fetch.
decision/file-url-only-for-file-documents · File URLs load only for file documents — A file: URL loads only for a file: document or a top-level load; for any other document, remote pages included, script fetch, module import and script src fail, blitz-net refuses engine loads before a disk read, and the browser app names no file: favicon and drops a file: navigation silently.
decision/javascript-opt-in · JavaScript opt-in in the browser app — In the browser app JavaScript runs only behind the non-default `javascript` feature.
decision/root-boundary-err-crossing · Root-boundary Err crossing — A handler Err the app's root error boundary takes crosses as screen content, through Dioxus's default boundary rendering of its Display, and in caused as one error item holding its message and cause chain under the root boundary.
decision/script-fetcher-file-data-only · Script fetcher takes file and data URLs only — The default script fetcher supports only file: and data: URLs; any other scheme returns UnsupportedScheme.
decision/skip-snapshot-of-unstyled-node · Never-styled nodes not snapshotted — Snapshotting a never-styled node is skipped. · why: Stylo invalidation would otherwise panic.

## input-validation · ## Input Validation
decision/accessibility-focus-built-nodes-only · Accessibility focus names a built node — The accessibility tree's focus names the focused node only when that node is among the built nodes, else the window, so the tree never names a node it did not build. · why: An engine change for every document, ratified. · guard: inert_subtree
decision/accessibility-tree-refresh-on-change · Accessibility tree refreshed on change — The platform accessibility adapter receives the tree on change as well as on its initial request: a redraw that finds a non-empty changed set rebuilds the platform tree after style resolution and clears the set. · why: Ratified: the same crossing and the same content classes, more often.
decision/attribute-readers-no-log-or-panic · Markup attribute readers without log or panic — Markup attributes, remote pages included, are read from parsed content by in-process readers with no log site or panic path, ARIA states and closed values through closed parses carrying no attribute text.
decision/boundary-widening-rule · Boundary widening rule — A new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
decision/closed-driver-verb-schema · Closed driver verb schema — Every driver call is checked against a closed table of verbs and bounded argument kinds and answered with one typed command or one refusal, the first failure in a fixed order; a refused call changes nothing. · why: Each widening of the verb table is a widening of a validated surface, ratified.
decision/closed-value-field-no-widening · A closed-value field in a ratified carrier is no widening — A new field of a closed value set inside an already ratified carrier is not a widening and needs no ratification; a field holding authored text, an open value set, a new carrier or a new exit still does. · why: Ratified as the standing rule.
decision/default-fetcher-file-and-data-only · Default script fetcher takes file and data only — The default script fetcher takes only file: and data: URLs.
decision/inert-focus-clear-silent · Inert focus clear fires no blur — A node that loses focus by coming under `inert` fires no DOM `blur` or `focusout`. · why: An engine change for every document, ratified. · guard: inert_subtree
decision/inert-subtree · Inert subtree — An element carrying `inert`, or under an ancestor that does, is not focusable, takes no hit, leaves the accessibility tree and the snapshot, and a node holding focus when it comes under `inert` gives focus up; presence only, no value read. · why: An engine change for every document, ratified. · guard: inert_subtree
decision/programmatic-focus-events · Programmatic focus changes fire focus events — A programmatic focus change fires blur, focusout, focus and focusin. · why: An engine change for every document by upstream merge, ratified. · guard: event_queue, inert_subtree · except: The inert fix-up's focus clear and the driver's select focus fire none.
decision/seeded-source-in-process-seed-only · Seeded source takes its seed from in-process code only — The seeded source of randomness takes its seed only from in-process code, a root context the virtual DOM's builder gives else the default seed 0, and outside any run seeds from the host's per-process random keys, reading no environment variable, file or carrier.
decision/space-keyup-activates-focused-control · Space release activates the focused control — A released Space on the focused node dispatches a click for a `button`, a `summary`, or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` that is not disabled; a link, a text input and a range input take no Space. · why: An engine change for every document and a boundary widening, ratified.
decision/stable-id-exits · Ratified exits of the stable id — The stable id leaves the process only through the platform accessibility adapter, the driver's answers over the session socket and the client's stdout, MCP tool results on the `mcp` role's stdout, and kept check files. · why: Each carrier ratified as a boundary widening.
decision/accessibility-adapter-content-classes · Content classes on the accessibility adapter crossing — The platform accessibility adapter crossing carries four author-derived content classes: element ids, accessible names, control states and descriptions. · why: Each widening ratified; the states by ruling, the names' and descriptions' upstream additions ratified as upstream's behaviour.
decision/actions-read-changes-nothing · Actions read runs nothing and moves no session state — The actions read runs nothing and changes no snapshot, focus reading or diff, so it can be called without moving the session. · why: The stand_actions test the_read_changes_nothing_and_holds_no_typed_text_or_mask pins it; agents call the actions verb between steps over socket and MCP
decision/actions-read-shares-target-check · Actions read and verbs share one target check — Every id-taking verb and the actions read meet one shared target check, so what the actions read lists agrees with what a call would be refused. · why: The stand_actions test every_listed_cause_on_the_lean_tasks_is_what_the_call_returns pins that each listed cause equals the refusal the call returns
decision/dangerous-inner-html-parsed-unsanitized · Dioxus dangerous_inner_html set as raw inner HTML — Dioxus `dangerous_inner_html` is applied as inner HTML of the element, unsanitized. · why: The stand_keyboard and stand_snapshot_state fixtures build their button rows through dangerous_inner_html and depend on the markup being parsed as given
decision/enter-activates-focused-control · Enter activates the focused control — A pressed `Enter` on the focused node dispatches a click on it when it is no disabled form control and is a `button`, a `summary`, an `input` of type `submit`, `reset` or `button`, or an `a` with `href`. · why: An engine change for every document and a boundary widening, ratified.
decision/event-is-trusted-by-source · isTrusted by dispatch source — A script-dispatched event reads isTrusted false, and an event the user agent dispatches reads isTrusted true. · why: A boundary widening by upstream merge, ratified.
decision/falsy-boolean-attribute-removed · Falsy boolean attribute removed — A falsy value of a listed boolean attribute clears the attribute rather than writing it. · why: The tests falsy_boolean_attributes_are_removed and every_listed_boolean_attribute_is_removed_when_falsy pin it; snapshot disabled readings depend on it
decision/file-urls-only-from-file-documents · file URLs load only from file documents — A file: URL is read only for a file: document or a top-level load: a script fetch, module import or script src, every engine stylesheet, @import, font, image or iframe load, and in the browser app a favicon and a link navigation; blitz-net refuses before a disk read, a navigation drops silently. · why: A narrowing, not a boundary widening. · guard: remote_document_cannot_load_file_urls
decision/fixture-name-closed-table · Fixture state name checked against a closed table — A fixture state name is compared whole against the app's closed table of named states before any session starts; any other name is refused as an unknown state and nothing is booted.
decision/session-layout-read-in-process · In-process layout read on a session — The session's layout read, in process or through the layout verb on any ratified carrier, resolves its id through the run path's id resolution refusing stale or not-found, runs no step, pump, tick or settle, and answers stable id strings, closed words and numbers, never a NodeId, text or value.
decision/signal-registry-holds-no-state-value · Signal registry holds no state value — The in-process signal registry takes dioxus-core types alone, holds no value of a state, and has no log site, print, env read or `unwrap` on app input.
decision/target-refusal-order · Fixed order of target refusals — A click or type on an element that cannot take it is refused with nothing run, the first of disabled, off-screen, covered that holds, leaving the snapshot and both focus readings as they were; scroll reports in_view instead of refusing. · why: The detection of refusal causes by code was ratified.
decision/unknown-target-refused · Call on an unknown or unfit element refused — A call whose id names no element, or an element that cannot take the call, is refused with nothing run.
decision/url-search-params-bootstrap · URLSearchParams through a bootstrap script — URLSearchParams and a URL's live searchParams, href and search reach page script through a bootstrap script, bad calls and receivers throwing TypeError and values passing USV conversion. · why: A realization on the registered script crossing, no boundary widening; ratified. · guard: url_search_params
contract/driver-verbs/layout · Driver verbs · layout — Takes one required id, refused malformed without it or with a second argument and not-found or stale for an unknown id; changes nothing, leaves delivered Timer ticks unapplied, and answers the element's placement, shifts and sizes as ids, closed words and numbers.
contract/mcp-errors/32602 · MCP protocol errors · -32602 — invalid params or unknown tool, with no data: params not an object with a string name and an optional object arguments, or a name outside the verb table.
contract/mcp-tool-call-span/tool · The MCP tool_call span · tool — The verb table's word for the called tool; the span parents the call's command span.
contract/session-socket-reply/ok-refused · Session socket reply · ok v3 refused — The call was refused; the refusal JSON follows on the line.
contract/session-socket-reply/refused-malformed · Session socket reply · refused malformed — A request line the grammar does not read or over 16,384 bytes; nothing runs.
contract/session-socket-reply/refused-version · Session socket reply · refused version — A known word with another wire version; nothing runs.
contract/session-socket-request/call · Session socket request · call — `call v3 <verb> [<name>=<kind>:<value>]…` runs a validated verb, kinds `t` text, `n` whole number, `f` flag, verb, names and text percent-escaped.
contract/stable-element-id/carriers · The stable element id · carriers — The stable element id leaves by the platform adapter, the driver's answers, MCP results and kept checks, and is written by escher's code to no log.
contract/stable-element-id/no-node-id · The stable element id · NodeId — No `NodeId` crosses any carrier of the stable element id.

## data-protection · ## Data Protection
decision/log-sink-redacts-and-allowlists · Log sink redacts and allowlists targets — Escher's own log sink redacts records and drops a record from a target outside escher-telemetry's target allowlist.

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project)
decision/telemetry-sink-target-allowlist · Telemetry sink target allowlist — escher's own log sink redacts engine-target messages and fields, redacts the content-named fields of escher targets, and drops every record from a target outside its two prefix sets at every level. · except: the upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed

## error-handling · ## Error Handling
decision/closed-refusal-cause-set · Closed refusal cause set — A driver command refusal is a typed value whose cause comes from a closed set of eight, with a fault only beside `malformed`; a new verb or argument shape is refused within the set, never by adding a cause. · why: The closed set of eight was ratified.
decision/dioxus-render-into-buffer · Dioxus renders into a buffer before borrowing — A Dioxus document renders mutations into a buffer with the document unborrowed, then borrows it once to replay, and flushes queued mounted events after the borrow is dropped. · why: So a task polled during render can reach the document through its mounted handle with no `RefCell already borrowed` panic. · guard: dioxus_runtime_probe mounted_data_in_a_task_a_key_handler_spawned_acts tests
decision/dropped-anonymous-block-clears-parent · A dropped anonymous block clears its children's parent — Dropping an anonymous block clears the layout parent of every child still naming it, so a key press on a control whose block was dropped does not panic and still acts. · why: So the id does not go stale, curing every reader of a stale layout parent. · guard: key_press_after_container_lost_its_box
decision/refusal-closed-cause-set · Command refusal as a closed set of causes — An escher-driver command refusal is a typed value from a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and with no free-text field. · why: So a refusal cannot hold anything a call supplied. · except: A Fault is carried only beside the malformed cause.
decision/script-errors-recorded-not-propagated · Script errors recorded and failed scripts skipped — Uncaught script errors are logged and not propagated, and a script that fails to resolve or fetch is skipped. · why: The WPT harness, ref and crash test runners and the browser's DocumentLoader drain take_js_errors; blitz-vibey-script dom and selection tests read it too
decision/skip-mounted-event-of-removed-element · Mounted event of a removed element skipped — Flushing queued mounted events skips an element removed since its event was queued, for every document. · why: Differs from upstream: upstream's test asserting no mounted event is left out, as it reads one on Dioxus 0.7.
decision/stale-node-ids-never-panic · Stale node ids never panic — A stale node id is skipped in layout children and resolves to none on lookup rather than panicking; a stale animation entry for a removed node is skipped on the next resolve. · guard: stale_interaction_state, stale_node_mapping, animations and custom_widget_layout tests
decision/try-doc-none-while-borrowed · try_doc answers none while the document is borrowed — Event handling may ask for the document while it is mutably borrowed and gets None instead of a panic. · why: The browser status bar reads the document through try_doc and returns early instead of panicking while event handling holds the borrow
contract/mounted-element-operations/missing-node · Dioxus mounted-element operations · missing node — An operation on a node that does not exist returns MountedError::OperationFailed wrapping NodeNotExistErr.
decision/select-list-closes-when-select-unfit · Open select list closes when its select becomes unfit — A select removed, disabled, hidden or made inert with its list open closes the list with no panic, and a dropped option is no row. · why: The select_popup test a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes pins the close after each layout
contract/refusal-causes/covered · Driver refusal causes · covered — The target is covered at its hit point; its fixed meaning states the hit's limits, among them that an option of a closed select reads covered, remedied by clicking the select first.
contract/refusal-causes/disabled · Driver refusal causes · disabled — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
contract/refusal-causes/fonts-unavailable · Driver refusal causes · fonts-unavailable — One of the eight closed causes, returned by the executor, with a fixed name, meaning and remedy holding no path.
contract/refusal-causes/malformed · Driver refusal causes · malformed — The one cause that carries a fault; a call breaking the schema, such as an `actions` call with an argument or a `graph` part outside 1 to 1000, is refused with it and nothing runs.
contract/refusal-causes/not-found · Driver refusal causes · not-found — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
contract/refusal-causes/off-screen · Driver refusal causes · off-screen — The element lies outside the visible part, including outside a scrolling box that holds it; its remedy names the `scroll` verb.
contract/refusal-causes/stale · Driver refusal causes · stale — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
contract/refusal-causes/unknown-verb · Driver refusal causes · unknown-verb — One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
contract/refusal-json/cause · Refusal JSON · cause — The refusal's cause name, inside `{"refused":{...}}`, built from the cause's fixed strings.

## logging-monitoring · ## Logging & Monitoring
decision/closed-span-lines · One sink line per closed span — The sink prints one line per closed span whose target the allowlist admits and nothing on creation, record, enter or exit; each span field is judged by the event rule, an outside-target span writes no byte, and a content-named span field's value is never stored. · why: A boundary widening of the sink, ratified as built.
decision/engine-target-field-allowlist · Engine log targets keep only allowlisted fields — In escher's telemetry sink an engine target keeps only seven allowlisted fields, and every other field is written as [redacted]. · except: An allowlisted log.file field can still carry a host path.
decision/engine-target-safe-fields · Engine targets print only safe fields — An event whose target starts with blitz, dioxus_native, stylo_taffy, accesskit_xplat, debug_timer or js_console prints only node_id, status, waiting_nodes, property, log.module_path, log.file and log.line; every other field, the message included, prints as [redacted].
decision/other-log-targets-dropped · Other log targets dropped whole — In escher's telemetry sink any target other than an engine or escher_ target is dropped whole at every level, and spans follow the same rule.
decision/outside-target-dropped · Outside targets dropped whole — An event or bridged log record from a target outside the allowlist is not printed at all, zero bytes, at every level including WARN and ERROR, whatever RUST_LOG names. · why: The named cost: a third-party WARN or ERROR no longer prints either.

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
