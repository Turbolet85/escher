# security-plan · the rows that bind 2026-11-25-virtual-nodes-declared — picked 62 of 586 (29 by every draw) · rule any · pin 3727

## threat-model-summary · ## Threat Model Summary
decision/os-drag-and-drop-untrusted · OS drag-and-drop is an untrusted input source — The local desktop's drag-and-drop is an untrusted input source to the engine and to a Dioxus app's handlers, carrying strings and file paths chosen by the user's OS session.
decision/canvas-content-crossing · Canvas content crossing — A canvas's app-painted content crosses as pixels in a shot and, on the socket, the command line, MCP and a kept check, as a canvas answer of closed words, numbers and 64-bit digests, fonts and images as indexes with digests, never a glyph id, glyph position, text, font bytes or family name. · why: A boundary widening, ratified. · guard: stand_canvas_carriers the_fixtures_answers_hold_no_glyph_id_and_no_text
decision/mcp-result-crossing · MCP result crossing — An MCP result carries ids, accessible names, control values with a password's or file input's as the mask, the snapshot text and an acting call's diff to the client; what a client does onward, a model provider included, is the client's. · why: A boundary widening, ratified.
decision/session-socket-carries-calls · Session socket carries calls and answers — The session socket carries the lifecycle and calls with their answers: an element id and typed text in; ids, accessible names, control values with a password's or file input's as the fixed mask, the snapshot text and the diff out; no NodeId crosses. · why: A boundary widening, ratified.
contract/driver-session-reply/no-node-id · driver session socket replies · NodeId — No reply carries a NodeId.

## input-validation · ## Input Validation
decision/accessibility-adapter-content-classes · Content classes on the accessibility adapter crossing — The platform accessibility adapter crossing carries four author-derived content classes: element ids, accessible names, control states and descriptions. · why: Each widening ratified; the states by ruling, the names' and descriptions' upstream additions ratified as upstream's behaviour.
decision/accessible-name-exits · Ratified exits of accessible names — Accessible names leave the process only through the platform accessibility adapter, a hosted call's answer in the snapshot text and diff JSON over the session socket and the client's stdout, and MCP tool results, and reach no log. · why: Each carrier ratified.
decision/accessible-names-not-logged · Accessible names kept out of logs — Escher's own code logs no accessible name, and a host that installs escher's sink writes none to stderr at any `RUST_LOG` level, the third-party record that carried one being dropped whole. · why: That third-party target is outside the sink's allowlist. · guard: mcp_log
decision/anchored-id-tier · Anchored stable-id tier — An unkeyed element's stable id is anchored under the nearest author-keyed element of its own component instance before falling back to its component path or document path. · why: The anchored tier was ratified.
decision/attribute-text-leaves-by-ratified-carriers · Attribute text leaves only by ratified carriers — Markup attribute values, names and descriptions leave escher only by the ratified carriers. · except: The page's own form submission and events.
decision/author-id-on-accessibility-nodes · Stable id as accessibility author id — Under the accessibility feature each element node of the accessibility tree carries its stable id as its AccessKit author id, never on a text run, the document root or the synthetic window node, so it leaves the process only through the platform adapter. · why: Ratified; the crossing text runs and labels already take.
decision/boundary-widening-rule · Boundary widening rule — A new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
decision/closed-value-field-no-widening · A closed-value field in a ratified carrier is no widening — A new field of a closed value set inside an already ratified carrier is not a widening and needs no ratification; a field holding authored text, an open value set, a new carrier or a new exit still does. · why: Ratified as the standing rule.
decision/custom-widget-content-box-frame · Custom widget painted and pointed in its content box — Behind the custom-widget feature, a custom widget's paint size is its content box times scale, floored at 0, its scene sits at the content origin inside the element's transform, unclipped, and its pointer events arrive in its content-box frame while a sub-document's stay in its border-box frame. · why: Ratified; answers upstream issue #1578, and paint size, placement and pointer frame are layout values, not input. · guard: stand_canvas_placement, stand_canvas_pointer_frame
decision/dropped-file-read-unrestricted · Dropped-file reader without path restriction — The Dioxus native bridge's dropped-file reader reads a dropped file's metadata and bytes through std::fs with no path restriction. · why: Upstream's design, ratified.
decision/no-process-handle-crosses · No process-local handle crosses with an id — Neither the stable id, the snapshot, the diff nor any driver session, answer or refusal carries a `NodeId`, `ElementId`, `ScopeId` or pointer; only the stable id text crosses.
decision/socket-carries-screen-reading · Session socket carries what the screen reads — An answer carries what the screen reads: ids, accessible names, control values, password and file input masked, snapshot text, diff, lint findings, UI graph, layout and canvas readings and a step's caused text scrubbed by its field names, error kept, never a NodeId or an attribute value as typed. · why: The crossing was ratified.
decision/stable-id-exits · Ratified exits of the stable id — The stable id leaves the process only through the platform accessibility adapter, the driver's answers over the session socket and the client's stdout, MCP tool results on the `mcp` role's stdout, and kept check files. · why: Each carrier ratified as a boundary widening.
decision/stable-id-not-logged · Stable ids kept out of logs — Escher's own code writes the stable id to no log, and a host that installs escher's sink drops third-party style records carrying an element's HTML id whole, printing that value at no `RUST_LOG` level. · why: Those third-party targets are outside the sink's allowlist.
contract/driver-session-reply/no-attribute-text · driver session socket replies · attribute text — A reply never carries an attribute's text, except what an author wrote into a record's message or into the `error` field of a `caused` item.
contract/stable-element-id/anchored-path · Stable element id · anchored path — An element with no author key reads `{key}//{segment}` (then `/{segment}` per further DOM level) under the nearest keyed element of its own component instance, else its component path, else its document path; an anchor acts inside one component only.
contract/stable-element-id/author-key · Stable element id · author key — An element's HTML `id` is its author key only when it is non-empty, holds no `/` and is the first element in document pre-order to claim it; an empty, `/`-bearing or later-duplicate value gives no key and anchors nothing.
contract/stable-element-id/carriers · The stable element id · carriers — The stable element id leaves by the platform adapter, the driver's answers, MCP results and kept checks, and is written by escher's code to no log.
contract/stable-element-id/derivation · The stable element id · derivation — The HTML `id` is the author key when non-empty, `/`-free and first in document order; otherwise an anchored, component or document path is used, so no `id` can make two ids equal.
contract/stable-element-id/no-collision · Stable element id · uniqueness — An author key holds no `/`, an anchored path is the only id holding `//`, and a component or document path has no empty segment, so no `id` attribute can make two ids equal, a key spelled like a component name included.
contract/stable-element-id/no-node-id · The stable element id · NodeId — No `NodeId` crosses any carrier of the stable element id.
contract/stable-element-id/stability · Stable element id · stability — A stable id is never a list index, pointer, hash, clock or process-local value, so a row reads the same id across a re-render, a remount and a second process.
contract/stable-element-id/unresolved-node · Stable element id · non-element, stale or detached node — A non-element, stale or detached node reads no id, and no path computation panics.
contract/stable-element-id/virtual-node-form · Stable element id · virtual-node form — The id grammar reserves `{owner}///{key}` for a virtual node, spelled for a non-empty key with no `/`; no served id holds `///`, and the anchored path stays the only served id holding `//`.
decision/accessibility-focus-built-nodes-only · Accessibility focus names a built node — The accessibility tree's focus names the focused node only when that node is among the built nodes, else the window, so the tree never names a node it did not build. · why: An engine change for every document, ratified. · guard: inert_subtree
decision/lint-findings-closed-content · Lint findings hold no page content — A lint finding holds ids, closed defect words and numbers only, never page text, a canvas label's text, colour or a control value, and no log site, print, span or field carries a finding. · why: A ratified boundary widening.
decision/merged-aria-states-closed-parse · Closed parse of expanded, disabled and level — `aria-expanded`, `aria-disabled` and `aria-level` are read as closed values carrying none of the attribute's text and exit only on the accessibility adapter; `aria-disabled` moves no snapshot reading. · why: As upstream wrote them, inside the ratified states carrier. · guard: accessibility_merged_states
decision/non-invertible-transform-no-hit · Non-invertible transform takes no hit — In every document, the whole subtree of a transform whose determinant is 0 or not finite takes no hit. · why: An engine change for every document, ratified. · guard: hit_non_invertible_transform
decision/optgroup-options-and-label · Optgroup options and label — A select's options include its optgroup children's options in tree order, and a group's non-empty label is its name in the accessibility tree and a header painted in an open list. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select, select_popup, select_popup_painted
decision/select-choice-exits · Where a select's choice leaves — A select's choice leaves only as form submission's entries, the input and change events' value, the snapshot's value and the options' accessibility-tree nodes, inside the ratified name and control-value crossings. · why: A boundary widening, ratified.
decision/session-label-form · Session label form — A session label is 1 to 32 bytes of a-z, 0-9 and -, checked before the caller's boot runs; it is never document text, and is the one caller string a reply carries.
decision/session-label-only-caller-string · Session label is the one echoed caller string — The session label is never text read from the document and is the one caller string a reply carries.
contract/graph-answer/nodes-states · Graph answer · nodes and states — `{"nodes":[…],"states":[…]}` holding ids, component names, state names, handler event names, the flag words `undeclared` and `unconfirmed` and booleans, never a control value, typed text or NodeId; a part adds `part` and `parts`, its states empty past part 1.
contract/snapshot-text/id-field · Snapshot text · id field — Each node's id is written as a quoted `id=` field in the string's Debug form, so the field never holds a line break or a bare `"`; the snapshot has one text form and no JSON form, Display form or parser of the text.
contract/snapshot-text/name-field · Snapshot text · name field — Each line's accessible name is written as a quoted string in the string's Debug form, so a name holding a line break or a `"` stays inside its one field.
contract/stable-element-id/keyed-list-segment · Stable element id · keyed list row segment — A keyed list row's segment carries its Dioxus key as `{tag}[{key}]`; a Dioxus key that is empty or holds `/` is refused and the row reads the positional `{tag}:{n}` segment instead.

## data-protection · ## Data Protection
decision/coverage-line-no-user-data · Coverage line carries no user data — The coverage answer line holds ids, owner ids, event names and counts only, never a control value, typed text, path or NodeId. · guard: stand_handler_coverage a_typed_sentinel_never_reaches_the_coverage_line
decision/exploration-report-closed-content · Exploration report holds closed content only — An exploration's report holds only closed endings, indices, calls and booleans, never a panic payload, NodeId, screen text or screen digest, and it is never logged or printed.
decision/kept-check-actions-answer · Kept check holds the actions answer — A kept actions answer holds ids, owner ids, keyed booleans, the table's verb words and closed cause words, never a control value, typed text or NodeId. · why: A boundary widening, ratified. · guard: stand_replay a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes
decision/kept-check-call-content · Kept check call-content class — A kept check holds call content: target ids, press keys and the ctrl flag, graph parts, advance ms and text typed into unmasked controls, a masked or unresolved type's text kept as MASKED_STAND_IN. · why: Ratified.
decision/kept-check-canvas-answer · Kept check holds the canvas answer — A kept check holds a canvas step and its whole answer of closed words, numbers and 64-bit digests, never a glyph id, text or NodeId, and its replay stops naming digest when a canvas digest differs. · why: A boundary widening, ratified. · guard: stand_canvas_carriers a_kept_canvas_check_replays_green_and_a_flipped_digest_stops_it
decision/kept-check-graph-answer · Kept check holds the graph answer — A kept graph answer holds ids, owner ids, component names, registered state names, handler event names, closed flag words, booleans and a part's part and parts numbers, never a control value or typed text. · why: A boundary widening, ratified.
decision/kept-check-layout-answer · Kept check holds the layout answer — A kept check holds a layout step and its whole answer of ids, closed words and numbers, never a control value, typed text or NodeId, and its replay stops on a changed placed value. · guard: stand_replay a_kept_layout_step_replays_and_a_changed_one_stops_naming_placed

## dependency-security · ## Dependency Security
decision/harness-blitz-paint-custom-widget · Harness turns on blitz-paint's custom-widget feature — blitz-test-harness turns on blitz-paint's custom-widget feature, so blitz-paint's widget paint pass is compiled into the harness, the driver and the escher-session binary in every build selection that reaches the harness, adding no package to Cargo.lock. · why: So a shot paints an app's widget in every build selection. · guard: stand_canvas_shot

## bootstrap-phases-derive-for-route-setup-project · ## Bootstrap phases (derive for route / setup-project)
decision/telemetry-sink-target-allowlist · Telemetry sink target allowlist — escher's own log sink redacts engine-target messages and fields, redacts the content-named fields of escher targets, and drops every record from a target outside its two prefix sets at every level. · except: the upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed

## secret-management · ## Secret Management
decision/live-evidence-holds-no-credentials · Live evidence holds no credentials — No committed live evidence, artifact, event or verdict carries a key, auth header, credential-file name, token field, e-mail address or the operator's login.

## error-handling · ## Error Handling
decision/stale-node-ids-never-panic · Stale node ids never panic — A stale node id is skipped in layout children and resolves to none on lookup rather than panicking; a stale animation entry for a removed node is skipped on the next resolve. · guard: stale_interaction_state, stale_node_mapping, animations and custom_widget_layout tests
decision/dioxus-render-into-buffer · Dioxus renders into a buffer before borrowing — A Dioxus document renders mutations into a buffer with the document unborrowed, then borrows it once to replay, and flushes queued mounted events after the borrow is dropped. · why: So a task polled during render can reach the document through its mounted handle with no `RefCell already borrowed` panic. · guard: dioxus_runtime_probe mounted_data_in_a_task_a_key_handler_spawned_acts tests

## logging-monitoring · ## Logging & Monitoring
decision/no-screen-content-in-logs · Screen content stays out of logs — No driver call or in-process read puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer, a layout answer, a canvas answer, a coverage check and a step's caused text reach no log line at any level. · guard: stand_act_spans, host_log and mcp_log, read at RUST_LOG=trace under both builds
decision/agent-run-raw-log-unprinted · Agent-run raw log kept unprinted — The agent-run raw cargo and libtest output log is unscrubbed, never printed by logs, and gitignored.
decision/cold-agent-record-unprinted · Cold-agent record kept unprinted — The cold-agent server log, check, raw transcript and the live child's kept checks are never printed; the transcript stays unscrubbed and gitignored, and each committed copy has its host paths masked.
decision/cold-agent-transcript-raw · Cold-agent raw transcript kept unscrubbed — The cold-agent session's raw transcript stays unscrubbed by design, is never printed and is gitignored; each committed copy has its host paths, the session dir and the home prefix, masked.
decision/escher-target-content-fields-withheld · escher_ targets withhold content-named fields — In escher's telemetry sink an escher_ target keeps all fields but twelve content-named ones.
decision/escher-target-content-scrub · Escher targets scrub content-named fields — An event whose target starts with escher_ prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which print as [redacted].
decision/session-output-holds-no-caller-input · Session output holds no caller input — No escher-session output line holds anything the caller typed. · except: The mcp role's echoed request id.
contract/escher-session-coverage-line/content · The escher-session coverage role's stdout line · content — The coverage line holds ids, owner ids, event names and two counts, and nothing of a named path.
contract/escher-session-output/caller-text · escher-session output · caller-typed text — No output line echoes a caller's argument but the admitted request id the mcp role echoes; an answer carries each node's current value, so text typed into an unmasked control returns, a masked input reading only the mask.

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
name/command/canvas — canvas

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
