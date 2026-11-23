# security-plan · the rows that bind 2026-11-23-circle-drawer-on-a-canvas — picked 91 of 577 (63 by every draw) · rule any · pin 3410

## threat-model-summary · ## Threat Model Summary
decision/actions-answer-crossing · Actions answer crossing — An actions answer carries what each actionable node can take now on the socket, the command line, MCP and a kept check: ids, owner ids, keyed booleans, verb-table words and closed cause words, never a control value, typed text or NodeId. · why: A boundary widening, ratified.
decision/canvas-content-crosses-as-shot-pixels · Canvas content crosses only as shot pixels — A canvas's app-painted content, a widget painted from Rust, crosses only as pixels in a screenshot on every ratified carrier, the model provider on a cold-agent live run included, with no new carrier, field, cause, session error, exit, log field or span, and a kept check keeps no pixels. · why: A boundary widening, ratified. · guard: stand_canvas_shot
decision/graph-answer-crossing · Graph answer crossing — A graph answer carries the screen's UI graph on the socket, the command line, MCP and a kept check: ids, owner ids, component names, registered state names, handler event names, closed flag words, booleans and a part's part and parts numbers, never a control value, typed text or NodeId. · why: A boundary widening, ratified.
decision/lint-findings-crossing · Lint findings cross every ratified carrier — A lint's findings cross every ratified carrier as stable ids, closed defect words and numbers, never page text, colour or value.
contract/actions-answer/id · The actions answer · id — The node's id.
contract/actions-answer/verbs · The actions answer · verbs — A list of entries each holding verb, a word of the verb table, and cause, a closed cause word left out when absent; every listed node carries layout with no cause.
decision/driver-session-unix-socket-only · Driver session on one Unix socket — The driver session socket binds one Unix-domain socket in a caller-named state directory, never a TCP or UDP port, and exists on unix only, while an in-process session runs on linux, windows and macOS alike.
contract/actions-answer/owner · The actions answer · owner — The node's owner id.

## input-validation · ## Input Validation
decision/accessibility-adapter-content-classes · Content classes on the accessibility adapter crossing — The platform accessibility adapter crossing carries four author-derived content classes: element ids, accessible names, control states and descriptions. · why: Each widening ratified; the states by ruling, the names' and descriptions' upstream additions ratified as upstream's behaviour.
decision/accessible-name-exits · Ratified exits of accessible names — Accessible names leave the process only through the platform accessibility adapter, a hosted call's answer in the snapshot text and diff JSON over the session socket and the client's stdout, and MCP tool results, and reach no log. · why: Each carrier ratified.
decision/accessible-names-not-logged · Accessible names kept out of logs — Escher's own code logs no accessible name, and a host that installs escher's sink writes none to stderr at any `RUST_LOG` level, the third-party record that carried one being dropped whole. · why: That third-party target is outside the sink's allowlist. · guard: mcp_log
decision/actions-keyed-field · Keyed flag in the actions answer — The `actions` verb's answer carries, per actionable node, its id, owner id, a `keyed` boolean and closed verb and cause words, no NodeId. · why: A ratified boundary widening.
decision/actions-read-changes-nothing · Actions read runs nothing and moves no session state — The actions read runs nothing and changes no snapshot, focus reading or diff, so it can be called without moving the session. · why: The stand_actions test the_read_changes_nothing_and_holds_no_typed_text_or_mask pins it; agents call the actions verb between steps over socket and MCP
decision/actions-read-shares-target-check · Actions read and verbs share one target check — Every id-taking verb and the actions read meet one shared target check, so what the actions read lists agrees with what a call would be refused. · why: The stand_actions test every_listed_cause_on_the_lean_tasks_is_what_the_call_returns pins that each listed cause equals the refusal the call returns
decision/aria-label-trimmed-once · One trimmed aria-label read — An `aria-label` non-empty after trimming becomes the node's label and a whitespace-only one names nothing; there is one trimmed, non-empty `aria-label` read. · why: Follows accname-1.2; upstream's unfiltered read was dropped in the merge. · guard: accessibility_label
decision/attribute-readers-no-log-or-panic · Markup attribute readers without log or panic — Markup attributes, remote pages included, are read from parsed content by in-process readers with no log site or panic path, ARIA states and closed values through closed parses carrying no attribute text.
decision/attribute-text-leaves-by-ratified-carriers · Attribute text leaves only by ratified carriers — Markup attribute values, names and descriptions leave escher only by the ratified carriers. · except: The page's own form submission and events.
decision/cancelled-pointerup-still-clicks · A cancelled pointerup still clicks — A pointerup whose handler cancels it still clicks and still ends a scrollbar drag. · why: An engine change for every document by upstream merge, ratified. · guard: pointerup_cancelled
decision/closed-driver-verb-schema · Closed driver verb schema — Every driver call is checked against a closed table of verbs and bounded argument kinds and answered with one typed command or one refusal, the first failure in a fixed order; a refused call changes nothing. · why: Each widening of the verb table is a widening of a validated surface, ratified.
decision/custom-widget-content-box-frame · Custom widget painted and pointed in its content box — Behind the custom-widget feature, a custom widget's paint size is its content box times scale, floored at 0, its scene sits at the content origin inside the element's transform, unclipped, and its pointer events arrive in its content-box frame while a sub-document's stay in its border-box frame. · why: Ratified; answers upstream issue #1578, and paint size, placement and pointer frame are layout values, not input. · guard: stand_canvas_placement, stand_canvas_pointer_frame
decision/custom-widget-document-font-context · Custom widget handed the document's font context — Behind the custom-widget feature, a custom widget is handed the document's own shared, mutable font context before each paint, and the base document exposes that context publicly, so app widget code holds a live handle to the engine's font context; no input crosses and the default ignores it. · why: Ratified; no input crosses, the handle being the engine's own. · guard: stand_canvas_text
decision/custom-widget-initial-attribute-replay · Custom widget initial attribute replay — Behind the custom-widget feature, setting a custom widget replays each existing attribute, parsed page content included, to the widget with no old value, so the widget receives them at set-up and checking each value stays the widget's own job. · why: An engine change for every document behind the feature, ratified. · guard: custom_widget_replays_initial_attributes
decision/event-is-trusted-by-source · isTrusted by dispatch source — A script-dispatched event reads isTrusted false, and an event the user agent dispatches reads isTrusted true. · why: A boundary widening by upstream merge, ratified.
decision/lint-findings-closed-content · Lint findings hold no page content — A lint finding holds ids, closed defect words and numbers only, never page text, colour or a control value, and no log site, print, span or field carries a finding. · why: A ratified boundary widening.
decision/no-agent-urls-through-font-path · No agent URL through font sources — escher routes no agent- or user-supplied URL through the font-face source path.
decision/one-actionable-rule · One actionable rule for both readings — The unkeyed-actionable reading and the actionable-nodes reading both read one actionable rule: focusable, an interactive role, or current listeners holding an event other than `mounted`. · why: The actionable test a_listener_removed_on_a_re_render_is_read_by_neither_reader pins it; the driver's actions verb and the unkeyed lint read these readers
decision/range-keyboard-actions · Range input key actions — On a focused range input with no `disabled` attribute, `ArrowRight` or `ArrowUp` steps up, `ArrowLeft` or `ArrowDown` steps down, `Home` and `End` go to the minimum and maximum, and every other key does nothing. · why: Ratified.
decision/range-pointer-actions · Range input pointer actions — A main-button pointerdown on an enabled range input sets the value from the hit's fraction of the content box, and a click focuses it and changes no value. · why: Ratified.
decision/range-value-exits · Range current value exits — A range input's current value leaves only as a form submission entry, the `input` event's value and the snapshot's control value, and no accessibility node value or action changes. · why: An engine change for every document and a boundary widening, ratified.
decision/range-value-sanitized · Range value always finite and on grid — Every range seed, attribute write and move is clamped to its bounds and moved to the nearest grid value, the greater on a tie, so the value stays finite, bounded and on its grid on degenerate markup; unparsable numbers read as absent. · why: An engine change for every document and a boundary widening, ratified.
decision/screen-pixels-in-band · Screen pixels in band — A screenshot's PNG travels inside the answer itself as base64 text, on the command line and as an MCP image block. · why: Ratified as the in-band crossing.
decision/screenshot-in-band · Screenshot carried in band — A screenshot's PNG crosses in band, as base64 inside its JSON answer line or as an MCP image block, the whole screen or one element's visible part, a password input painted masked, under the unchanged answer bound. · why: The in-band image was the ratified choice, a boundary widening.
decision/socket-carries-screen-reading · Session socket carries what the screen reads — An answer carries what the screen reads: ids, accessible names, control values, password and file input masked, snapshot text, diff, lint findings, UI graph, layout readings and an acting step's caused text scrubbed by its own field names, error kept, never a NodeId or an attribute value as typed. · why: The crossing was ratified.
decision/space-keyup-activates-focused-control · Space release activates the focused control — A released Space on the focused node dispatches a click for a `button`, a `summary`, or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` that is not disabled; a link, a text input and a range input take no Space. · why: An engine change for every document and a boundary widening, ratified.
decision/target-refusal-order · Fixed order of target refusals — A click or type on an element that cannot take it is refused with nothing run, the first of disabled, off-screen, covered that holds, leaving the snapshot and both focus readings as they were; scroll reports in_view instead of refusing. · why: The detection of refusal causes by code was ratified.
decision/text-extent-read-only-none-when-unresolved · Text extent reader read-only with none answers — The text extent reader is read-only and answers none for an id that does not resolve or a node with no inline layout. · why: The text_extent test an_element_with_no_inline_content_reads_none pins the none answers; the escher-driver lint and stand_lint read it for truncation
contract/actions-answer/keyed · The actions answer · keyed — A boolean on each listed node.
contract/actions-answer/nodes · The actions answer · nodes — `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}`, one entry per actionable node, holding ids, owner ids, `keyed` booleans, verb words and closed cause words, never a control value, typed text or NodeId.
contract/driver-session-reply/no-attribute-text · driver session socket replies · attribute text — A reply never carries an attribute's text, except what an author wrote into a record's message or into the `error` field of a `caused` item.
contract/driver-verbs/actions · Driver verbs · actions — Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent, never a control value, typed text or NodeId.
contract/driver-verbs/graph · Driver verbs · graph — Takes an optional part from 1 to 1000 (anything else refused malformed, nothing run), runs no step, and answers {"nodes":[…],"states":[…]}, a part adding part and parts, of ids, owner ids, component, state and handler event names, flag words and booleans, never a control value, typed text or NodeId.
contract/graph-answer/nodes-states · Graph answer · nodes and states — `{"nodes":[…],"states":[…]}` holding ids, component names, state names, handler event names, the flag words `undeclared` and `unconfirmed` and booleans, never a control value, typed text or NodeId; a part adds `part` and `parts`, its states empty past part 1.
contract/screenshot-answer/png · Screenshot answer · png — `{"width":…,"height":…,"png":"<base64>"}`, a PNG of the whole screen or of one element's visible part.
contract/snapshot-text/clickable · Snapshot text state words · clickable — Written after `focused` only when the element's current listeners hold `click` and it is no disabled form control.
contract/snapshot-text/name-field · Snapshot text · name field — Each line's accessible name is written as a quoted string in the string's Debug form, so a name holding a line break or a `"` stays inside its one field.
contract/stable-element-id/author-key · Stable element id · author key — An element's HTML `id` is its author key only when it is non-empty, holds no `/` and is the first element in document pre-order to claim it; an empty, `/`-bearing or later-duplicate value gives no key and anchors nothing.
contract/stable-element-id/derivation · The stable element id · derivation — The HTML `id` is the author key when non-empty, `/`-free and first in document order; otherwise an anchored, component or document path is used, so no `id` can make two ids equal.
contract/stable-element-id/no-collision · Stable element id · uniqueness — An author key holds no `/`, an anchored path is the only id holding `//`, and a component or document path has no empty segment, so no `id` attribute can make two ids equal, a key spelled like a component name included.
decision/advance-moves-virtual-clock-exactly · Advance moves the virtual clock exactly — The advance verb, accepted on every session, moves the session's virtual clock by exactly the milliseconds asked inside one settled step and reports exactly that time. · why: Ratified.
decision/aria-labelledby-outranks-label · aria-labelledby outranks label and aria-label — An element's `aria-labelledby` targets' text, gathered cycle-guarded, is its computed label, outranking a `<label>` and an `aria-label`; an id naming no element is ignored and `aria-label` is the fallback when the targets hold no text. · why: Upstream's behaviour, ratified at the upstream sync. · guard: accessibility_label
decision/author-id-on-accessibility-nodes · Stable id as accessibility author id — Under the accessibility feature each element node of the accessibility tree carries its stable id as its AccessKit author id, never on a text run, the document root or the synthetic window node, so it leaves the process only through the platform adapter. · why: Ratified; the crossing text runs and labels already take.
decision/boundary-widening-rule · Boundary widening rule — A new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
decision/closed-value-field-no-widening · A closed-value field in a ratified carrier is no widening — A new field of a closed value set inside an already ratified carrier is not a widening and needs no ratification; a field holding authored text, an open value set, a new carrier or a new exit still does. · why: Ratified as the standing rule.
decision/dispatch-recorded-only-with-signal-registry · Dispatch recorded only where a signal registry stands — The Dioxus event handler records a dispatch only where a signal registry is registered; on a page with none, the event's handlers run and nothing is recorded. · why: The dioxus_document tests a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is and the_reached_record_holds_on_a_page_with_no_registered_state pin it
decision/id-lookup-anew-each-call · Caller-supplied id looked up anew — An acting command that takes a caller-supplied id looks it up on the screen as it reads now, on every call, never by index and keeping no node handle between calls; an id that names nothing is refused with nothing run and no panic.
decision/no-node-id-between-calls · No NodeId kept between calls — No NodeId is kept between calls.
decision/no-process-handle-crosses · No process-local handle crosses with an id — Neither the stable id, the snapshot, the diff nor any driver session, answer or refusal carries a `NodeId`, `ElementId`, `ScopeId` or pointer; only the stable id text crosses.
decision/range-current-value-apart · Range input current value apart from its attribute — A range input holds a current value apart from its `value` attribute; a `value` write re-seeds it and a `min`, `max` or `step` write or removal re-sanitizes it. · why: A departure from the spec's dirty-value flag, the cost the ratified ruling took on.
decision/stable-id-exits · Ratified exits of the stable id — The stable id leaves the process only through the platform accessibility adapter, the driver's answers over the session socket and the client's stdout, MCP tool results on the `mcp` role's stdout, and kept check files. · why: Each carrier ratified as a boundary widening.
decision/unknown-target-refused · Call on an unknown or unfit element refused — A call whose id names no element, or an element that cannot take the call, is refused with nothing run.
contract/driver-argument-kinds/key · Driver argument kinds · key — One name of a closed twelve-name list.
contract/snapshot-text/id-field · Snapshot text · id field — Each node's id is written as a quoted `id=` field in the string's Debug form, so the field never holds a line break or a bare `"`; the snapshot has one text form and no JSON form, Display form or parser of the text.
contract/stable-element-id/carriers · The stable element id · carriers — The stable element id leaves by the platform adapter, the driver's answers, MCP results and kept checks, and is written by escher's code to no log.
contract/stable-element-id/stability · Stable element id · stability — A stable id is never a list index, pointer, hash, clock or process-local value, so a row reads the same id across a re-render, a remount and a second process.

## api-security · ## API Security
decision/actions-answer-unpaged · The actions answer is unpaged — The actions verb takes no argument and a screen's whole answer is one answer under the 1,048,576-byte answer bound. · guard: stand_actions the_whole_cells_answer_is_within_the_answer_bound
decision/bounded-load-wait · Bounded wait on loads in flight — A harness settle never waits on a load in flight, and the driver's acting step waits on one for at most `LOAD_WAIT`. · why: So the absent request timeout cannot hang a settle or a command.
decision/load-wait-bound · Acting commands bounded by LOAD_WAIT — An acting driver command waits on a load in flight for at most LOAD_WAIT, 5 s of wall time, moving no time on the held instance, then returns the load still in flight as settled false with its busy class. · why: The absent request timeout cannot hang a command.
decision/scrollbar-fade-on-document-clock · Scrollbar fade reads the document clock — The overlay scrollbar fade reads the document's clock, so on a harness-built document two shots of one state read the same pixels until an advance moves the fade; a document built with no clock still fades by wall time. · why: Ratified.
decision/time-moves-only-on-advance · Time moves only on advance — Time on a held instance moves only when a caller asks for it with advance, 1 to 60000 ms a call; no driver verb has an argument or result field of a wait, and snapshot, screenshot, lint, graph and actions wait on no load.
decision/graph-answer-paged · The graph answer is paged — The graph verb takes an optional part from 1 to 1000; a graph within the 10,000-byte snapshot budget with no part asked is one whole answer, else parts of whole nodes each within the budget, a node over it a part alone, the states in part 1 alone and a part past the last empty. · why: A boundary widening, ratified. · guard: stand_graph cells_reads_in_parts_each_within_the_budget_and_together_the_whole_graph
decision/no-wait-verb-fields · No wait argument or result on verbs — No driver verb has an argument or a result field of a wait.
contract/driver-answer/settled · Driver command answer · settled — A step that ran and did not go quiet is a result, settled false with its busy class, not a refusal; the busy reading names a class only, never a URL, request, node, id, name or value.

## dependency-security · ## Dependency Security
decision/engine-dependency-pins · Engine dependency pins — The pins stay: Stylo 0.24, Taffy the git pin of upstream DioxusLabs/taffy at commit a3997d55, Dioxus 0.7.3 and winit =0.31.0-beta.4.
decision/harness-blitz-paint-custom-widget · Harness turns on blitz-paint's custom-widget feature — blitz-test-harness turns on blitz-paint's custom-widget feature, so blitz-paint's widget paint pass is compiled into the harness, the driver and the escher-session binary in every build selection that reaches the harness, adding no package to Cargo.lock. · why: So a shot paints an app's widget in every build selection. · guard: stand_canvas_shot
decision/locked-builds · Locked builds — Every build, the flake, the bundle, every CI cargo leg and every cargo run of the build-directory sweep passes --locked so the lockfile is verified. · except: examples/wasm_hello, a standalone workspace with no Cargo.lock
decision/no-git-dependencies · No git dependencies in the root manifest — The root manifest holds no git dependency; parley and every other crate are crates.io registry versions with checksums in the lockfile. · except: The taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy still passes the lock, audit and MSRV legs.
decision/no-git-dependency · No git dependency — The workspace takes no git dependency. · except: Taffy alone takes a git source: upstream pinned at a full commit rev, no patch section, integrity resting on the rev as its lock entry has no checksum; it still passes the lock, audit and MSRV legs.
decision/png-encodes-screenshots · png encodes a screenshot — The png crate is what encodes a screenshot in escher-driver, and blitz-tests takes it only as a dev-dependency.
decision/png-shot-encoder · png as the shot encoder — The png crate is the encoder of a shot in escher-driver.
decision/workspace-inherited-dependency-versions · Workspace-inherited dependency versions — Dependency versions are inherited from the workspace with workspace = true rather than declared inline in each manifest.
decision/dependency-audit-cargo-deny · Dependency audit with cargo-deny — cargo-deny checks advisories and licenses, locked, over the whole workspace's resolved graph with all features for the six platforms CI builds, configured by the root `deny.toml`, ignores per ID only, each with a written reason, re-read on every push. · why: Ratified: audit the whole workspace and check licences. · except: `unsound` stays at its default, which keeps memmap2's unsound advisory out of the gate, a wait on a cacache release as cacache 13.1.0 uses no advise_range.
decision/licence-gate · Licence gate on every push — The audit leg runs upstream's licences table, held byte for byte in the root deny.toml, on every push over the whole workspace's graph, so the fork gates on licences. · why: An open question, decided as the fork's chosen option. · guard: the audit leg · except: upstream's own licenses job stays reverted
decision/no-base64-crate · No base64 crate — No base64 crate is taken as a dependency; a shot's base64 is written by hand.

## error-handling · ## Error Handling
decision/stale-node-ids-never-panic · Stale node ids never panic — A stale node id is skipped in layout children and resolves to none on lookup rather than panicking; a stale animation entry for a removed node is skipped on the next resolve. · guard: stale_interaction_state, stale_node_mapping, animations and custom_widget_layout tests
decision/try-doc-none-while-borrowed · try_doc answers none while the document is borrowed — Event handling may ask for the document while it is mutably borrowed and gets None instead of a panic. · why: The browser status bar reads the document through try_doc and returns early instead of panicking while event handling holds the borrow
contract/refusal-causes/covered · Driver refusal causes · covered — The target is covered at its hit point; its fixed meaning states the hit's limits, among them that an option of a closed select reads covered, remedied by clicking the select first.
decision/dioxus-render-into-buffer · Dioxus renders into a buffer before borrowing — A Dioxus document renders mutations into a buffer with the document unborrowed, then borrows it once to replay, and flushes queued mounted events after the borrow is dropped. · why: So a task polled during render can reach the document through its mounted handle with no `RefCell already borrowed` panic. · guard: dioxus_runtime_probe mounted_data_in_a_task_a_key_handler_spawned_acts tests
contract/refusal-causes/fonts-unavailable · Driver refusal causes · fonts-unavailable — One of the eight closed causes, returned by the executor, with a fixed name, meaning and remedy holding no path.
contract/refusal-causes/off-screen · Driver refusal causes · off-screen — The element lies outside the visible part, including outside a scrolling box that holds it; its remedy names the `scroll` verb.

## logging-monitoring · ## Logging & Monitoring
decision/no-screen-content-in-logs · Screen content stays out of logs — No driver call or in-process read puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer, a layout answer, a coverage check and a step's caused text reach no log line at any level. · guard: stand_act_spans, host_log and mcp_log, read at RUST_LOG=trace under both builds
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
