# security-plan · the rows that bind 2026-11-16-taffy-past-1372 — picked 78 of 553 (57 by every draw) · rule any · pin 1390

## threat-model-summary · ## Threat Model Summary
decision/open-list-shot-pixels · Open select list shot carries labels as pixels — A shot of an open select list carries the page's option labels and group labels as pixels, on the socket and MCP carriers alike, the shot's fields unmoved. · why: A boundary widening, ratified. · guard: cli_commands, pinning a hosted shot equal to an in-process one
decision/select-list-crossing · Select list crossing limited to bounds and pixels — The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff and as pixels in a shot; node state holds no field of it and no expanded node key stands.
decision/skip-snapshot-of-unstyled-node · Never-styled nodes not snapshotted — Snapshotting a never-styled node is skipped. · why: Stylo invalidation would otherwise panic.

## input-validation · ## Input Validation
decision/advance-moves-virtual-clock-exactly · Advance moves the virtual clock exactly — The advance verb, accepted on every session, moves the session's virtual clock by exactly the milliseconds asked inside one settled step and reports exactly that time. · why: Ratified.
decision/button-click-focuses · A button click focuses the button — A click on a `button` of any type or an `input` of type `submit`, `reset` or `button` focuses it and submits its form owner when it is a submit button, a disabled form control stopping the walk first. · why: An engine change for every document and a boundary widening, ratified.
decision/dangerous-inner-html-parsed-unsanitized · Dioxus dangerous_inner_html set as raw inner HTML — Dioxus `dangerous_inner_html` is applied as inner HTML of the element, unsanitized. · why: The stand_keyboard and stand_snapshot_state fixtures build their button rows through dangerous_inner_html and depend on the markup being parsed as given
decision/disabled-read-by-presence · disabled read by presence on form controls — The disabled attribute is read by its presence, whatever its value, and only on a button, input, select or textarea, for focusability, pointer and keyboard activation and the snapshot's enabled and clickable readings. · why: The presence reading was ratified.
decision/engine-change-event-checkable · Engine fires change on checkbox and radio — A click on a checkbox always, and on a radio only where it was not checked before, sends `change` after `input` with the same value; the page script runtime synthesizes no `change`, so a script hears one, from the engine. · why: An engine change for every document and a boundary widening, ratified. · guard: checkable_change_event
decision/inert-subtree · Inert subtree — An element carrying `inert`, or under an ancestor that does, is not focusable, takes no hit, leaves the accessibility tree and the snapshot, and a node holding focus when it comes under `inert` gives focus up; presence only, no value read. · why: An engine change for every document, ratified. · guard: inert_subtree
decision/inline-box-heights-finite · Inline-box heights kept finite — Inline-box heights are kept finite. · why: Huge author lengths cannot stall the line breaker.
decision/multiple-select-size-rows · Multiple select size sets its rows — A multiple select's size, a whole number above zero capped at 65534, makes it that many rows tall and a key scrolls the next row in; zero or a non-numeric value gives no hint, and a single select's size is unread. · why: A boundary widening, ratified. · guard: size_reads_a_whole_number_above_zero_capped
decision/non-invertible-transform-no-hit · Non-invertible transform takes no hit — In every document, the whole subtree of a transform whose determinant is 0 or not finite takes no hit. · why: An engine change for every document, ratified. · guard: hit_non_invertible_transform
decision/programmatic-scroll-clamped · Programmatic scroll clamped and missing node ignored — Scroll targets are clamped to the scroll range, and a programmatic scroll to a missing node does nothing. · why: The fragment_navigation tests scroll_to_sets_element_offset_and_clamps and scroll_by_uses_relative_offsets_and_clamps pin the clamp; script scrollTo and scrollBy call it
decision/select-list-closing · What closes an open select list — An open select list closes on a press outside it, a page scroll or a move of its box, while a list a click just opened survives a focus rule's restyle. · why: An engine change for every document and a boundary widening, ratified. · guard: escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event
decision/select-list-hostile-markup-finite · Hostile markup under an open select list — A hostile size, thousands of options and very long labels under an open select list keep every box finite and panic nowhere. · why: An engine change for every document and a boundary widening, ratified. · guard: hostile_markup_keeps_every_box_finite_and_panics_nowhere
decision/select-list-pointer-release-chooses · Pointer in an open select list — In an open select list a press starts no text drag and the release chooses the row under it. · why: An engine change for every document and a boundary widening, ratified. · guard: a_press_on_a_row_selects_no_text_and_a_drag_across_rows_chooses_where_it_ends
decision/select-list-scroll-own-offset · Select list row scrolled into view by its own offset — Scrolling an open select list's row into view writes the list's own offset at once, moving no scroll chain or viewport, the offset staying within its rows. · why: An engine change for every document and a boundary widening, ratified. · guard: a_scroll_stays_within_the_rows_and_brings_a_row_in_by_the_least_movement
decision/select-list-twenty-rows · Open select list shows at most twenty rows — An open select list shows at most 20 rows, those past that scrolled. · why: An engine change for every document and a boundary widening, ratified. · guard: a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel
decision/select-list-wheel · Wheel over and outside an open select list — A wheel over an open select list scrolls the list, and a wheel elsewhere closes the list and then scrolls. · why: An engine change for every document and a boundary widening, ratified. · guard: a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel
decision/target-refusal-order · Fixed order of target refusals — A click or type on an element that cannot take it is refused with nothing run, the first of disabled, off-screen, covered that holds, leaving the snapshot and both focus readings as they were; scroll reports in_view instead of refusing. · why: The detection of refusal causes by code was ratified.
decision/visible-region-open-list-row · Visible region of an open select list row — For a row or a group of an open select list, the visible region answers the list's scrollport. · why: An engine change for every document, ratified.
decision/visible-region-through-transforms · Visible region read through transforms — A clipping box's visible region is read through its transforms, exact under translate and scale, a rotation reading the rotated box's bounding box, and a mapped box with a non-finite edge answers none. · why: An engine change for every document, ratified. · guard: visible_region_transformed, stand_act_transformed, transformed_client_rect
decision/wpt-runner-js-script-type-detection · WPT runner treats only JavaScript script types as scripts — The WPT runner runs only JavaScript script types and escapes its wrapper HTML. · why: The WPT ref test runner calls document_has_scripts to decide whether to run the script runtime, skipping JSON data blocks; the wrapper HTML is built with html_escape
contract/recorded-check/accepted · Recorded check file · accepted — One of the closed record keys, whether the call was accepted.
contract/recorded-check/answer · Recorded check file · answer — Required and a JSON object; key order is not enforced.
contract/recorded-check/arguments · Recorded check file · arguments — Argument names drawn from that verb's schema.
contract/recorded-check/digest · Recorded check file · digest — 16 lowercase hex characters.
contract/recorded-check/start-record · Recorded check file · start record — The array opens with `{"tool":"start","arguments":{"app":…},"format":1}`; a check without format reads as format 1, and any other format value, of whatever JSON type, is refused malformed, holding nothing of the bytes.
contract/recorded-check/tool · Recorded check file · tool — One of the ten instance verbs.
contract/stable-element-id/unresolved-node · Stable element id · non-element, stale or detached node — A non-element, stale or detached node reads no id, and no path computation panics.
decision/actions-read-shares-target-check · Actions read and verbs share one target check — Every id-taking verb and the actions read meet one shared target check, so what the actions read lists agrees with what a call would be refused. · why: The stand_actions test every_listed_cause_on_the_lean_tasks_is_what_the_call_returns pins that each listed cause equals the refusal the call returns
decision/id-lookup-anew-each-call · Caller-supplied id looked up anew — An acting command that takes a caller-supplied id looks it up on the screen as it reads now, on every call, never by index and keeping no node handle between calls; an id that names nothing is refused with nothing run and no panic.
decision/multiple-select-keys-and-clicks · Multiple select keys and clicks — In a multiple select, a plain arrow, Home, End or click chooses that row alone, Shift spans from the anchor, Ctrl or Meta moves the current row by key and flips a row by click, and disabled rows are never chosen. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select · except: Ctrl+A is not modelled.
decision/no-panic-on-call-input · No panic on call-supplied input — Neither the driver command schema's validation nor the executor has unwrap, expect, panic or unreachable on anything a call supplies.
decision/password-caret-placed-by-bullets · Password caret placed by the bullets — A click inside a password field places the caret by the painted bullets, so no width of the real characters reaches the caret. · why: Ratified.
decision/password-painted-masked · Password input painted masked — A password input is painted as one bullet per character in every document and through every renderer that paints with the engine's painter, so a typed password never leaves as pixels in a screenshot. · why: An engine change for every document, ratified. · guard: password_painted_masked, stand_act_screenshot · except: The frame depends on the value's length; a file input's paint is unchanged.
decision/range-pointer-actions · Range input pointer actions — A main-button pointerdown on an enabled range input sets the value from the hit's fraction of the content box, and a click focuses it and changes no value. · why: Ratified.
decision/select-list-live-reader · Open select list read through one live reader — The open select list is one document field, read through one live-list reader everywhere but reconciliation, the removal arm and row hover, which read the field directly.
decision/select-list-open-gate · Which select opens a list — A disabled, inert or hidden select, a select with no row to show, and one under a non-invertible transform open no list. · why: An engine change for every document and a boundary widening, ratified. · guard: a_disabled_inert_hidden_singular_or_empty_select_opens_nothing
decision/select-readers-resolve-node-closed-select-none · Select readers resolve the node and skip an open list — Each public select reader resolves its node so a stale id reads none, and the closed-select reader answers none for a select whose list is open. · why: The escher-driver lint and the snapshot's option bounds call closed_select_of and select_list_bounds, relying on none for a stale id or an open list
decision/single-select-selectedness · Single-choice select selectedness — In every document, remote pages included, a select without multiple holds its choice as its options' checked state and settles every recorded select by HTML's selectedness setting algorithm: the last chosen, else the first option not disabled. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select · except: An option of a closed single select has no click arm and stays refused covered; a single select's size is not read.
decision/unknown-target-refused · Call on an unknown or unfit element refused — A call whose id names no element, or an element that cannot take the call, is refused with nothing run.
contract/screenshot-answer/png · Screenshot answer · png — `{"width":…,"height":…,"png":"<base64>"}`, a PNG of the whole screen or of one element's visible part.

## data-protection · ## Data Protection
decision/committed-counter-checks · Committed stand-Counter checks — The only committed kept checks are the two of the repository's scripted stand-Counter sessions, holding no typed text, host path, name or secret and no `screenshot` step, read only through the strict `Check::from_bytes`. · why: The one exception to a kept check's owner-only medium.
decision/kept-check-answer-whole · Kept check keeps each answer whole — A kept check keeps each answer as the line the driver wrote, masked as on every carrier, with a shot kept as its width and height and never its pixels. · why: Ratified, a shot's pixels excepted.
decision/kept-check-call-content · Kept check call-content class — A kept check holds call content: target ids, press keys, advance ms and text typed into unmasked controls, a masked or unresolved type's text kept as MASKED_STAND_IN. · why: Ratified.
decision/kept-check-owner-only · Kept checks are owner-only — A recorded check is written 0600 into a 0700 directory the launcher named, is never logged, and is never removed by escher but for its own check when a recording ends. · why: Ratified. · guard: recording::the_committed_checks_are_what_the_binary_records · except: Two scripted stand-Counter checks, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step.
contract/kept-check-file/contents · Kept check files · contents — A kept check holds per call its verb, arguments, acceptance, digest and masked answer, never a pixel.
contract/kept-check-file/entry · Kept check file · entry — A check is a JSON array: an opening record of the app label and the format, then per answered call its verb, its arguments, its acceptance, a FNV-1a 64 digest of its written answer and the answer itself as answer.
decision/kept-check-actions-answer · Kept check holds the actions answer — A kept actions answer holds ids, owner ids, keyed booleans, the table's verb words and closed cause words, never a control value, typed text or NodeId. · why: A boundary widening, ratified. · guard: stand_replay a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes
decision/kept-checks-owner-only · Kept checks owner-only and uncommitted — A kept check stays in an owner-only medium, is never logged and is never committed. · except: The two committed checks of the repository's own scripted stand-Counter sessions.

## api-security · ## API Security
decision/scrollbar-fade-on-document-clock · Scrollbar fade reads the document clock — The overlay scrollbar fade reads the document's clock, so on a harness-built document two shots of one state read the same pixels until an advance moves the fade; a document built with no clock still fades by wall time. · why: Ratified.
decision/time-moves-only-on-advance · Time moves only on advance — Time on a held instance moves only when a caller asks for it with advance, 1 to 60000 ms a call; no driver verb has an argument or result field of a wait, and snapshot, screenshot, lint, graph and actions wait on no load.
decision/actions-answer-unpaged · The actions answer is unpaged — The actions verb takes no argument and a screen's whole answer is one answer under the 1,048,576-byte answer bound. · guard: stand_actions the_whole_cells_answer_is_within_the_answer_bound
decision/graph-answer-unpaged · The graph answer is unpaged — The graph verb takes no argument and a screen's whole graph is one answer under the 1,048,576-byte answer bound. · why: A part argument would widen the crossing. · guard: stand_graph the_whole_cells_answer_is_within_the_answer_bound

## dependency-security · ## Dependency Security
decision/advisory-ignores-by-id · Advisory ignores by ID only — An advisory is ignored only by its ID, each with a written reason, with no blanket allow and unmaintained and unsound at their defaults.
decision/audit-ignores-per-id-with-reason · Audit ignores per ID with a written reason — Audit ignores are per advisory ID only, each with a written reason, no blanket allow, and each ignore is re-read on every push.
decision/cargo-deny-audit-tool · cargo-deny as the audit tool — cargo-deny 0.20.2, with the root deny.toml, is the dependency audit tool.
decision/dependency-audit-cargo-deny · Dependency audit with cargo-deny — cargo-deny checks advisories and licenses, locked, over the whole workspace's resolved graph with all features for the six platforms CI builds, configured by the root `deny.toml`, ignores per ID only, each with a written reason, re-read on every push. · why: Ratified: audit the whole workspace and check licences. · except: `unsound` stays at its default, which keeps memmap2's unsound advisory out of the gate.
decision/dependency-audit-ci-gate · Dependency audit CI gate — The CI audit job runs the dependency audit on every pull request and push, in the slow tier behind the fast jobs, uncached, its log uploaded on failure.
decision/engine-dependency-pins · Engine dependency pins — The pins stay: Stylo 0.22.0, Taffy the git pin of upstream DioxusLabs/taffy at commit e90751fd, Dioxus 0.7.3 and winit =0.31.0-beta.3.
decision/licence-gate · Licence gate on every push — The audit leg runs upstream's licences table, held byte for byte in the root deny.toml, on every push over the whole workspace's graph, so the fork gates on licences. · why: An open question, decided as the fork's chosen option. · guard: the audit leg · except: upstream's own licenses job stays reverted
decision/local-audit-before-push · Run the audit leg locally before a push — The local fast leg runs no audit, so the audit leg is run locally before a push. · why: A CI audit can go red on advisories published after a green audit with the lock and deny config unchanged.
decision/locked-builds · Locked builds — Every build, the flake, the bundle and every CI cargo leg, passes --locked so the lockfile is verified. · except: examples/wasm_hello, a standalone workspace with no Cargo.lock
decision/no-git-dependencies · No git dependencies in the root manifest — The root manifest holds no git dependency; parley and every other crate are crates.io registry versions with checksums in the lockfile. · except: The taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy still passes the lock, audit and MSRV legs.
decision/no-git-dependency · No git dependency — The workspace takes no git dependency. · except: Taffy alone takes a git source: upstream pinned at a full commit rev, no patch section, integrity resting on the rev as its lock entry has no checksum; it still passes the lock, audit and MSRV legs.
decision/upstream-sync-dependency-set · Upstream sync takes the dependency set whole — Each upstream sync takes upstream's dependency set whole or, by cherry-pick, enters no package and takes git-pinned content only by a lock-only, semver-compatible update of a pinned registry crate or Taffy's own git pin; its lock is read name by name, the audit leg green after it.
decision/color-crate-luminance-source · color crate as luminance source — The test harness's contrast reader takes its luminance from the color crate already in the graph through blitz-dom, adding no package to Cargo.lock.

## error-handling · ## Error Handling
decision/select-list-closes-when-select-unfit · Open select list closes when its select becomes unfit — A select removed, disabled, hidden or made inert with its list open closes the list with no panic, and a dropped option is no row. · why: The select_popup test a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes pins the close after each layout
decision/select-row-layout-none-without-styles · Select row layout reads none for an unstyled row — The select list's public readers and writer read a row with no styles as None and add no panic path. · why: The blitz-paint select list painter skips a row whose select_row_layout answers None instead of failing
decision/stale-node-ids-never-panic · Stale node ids never panic — A stale node id is skipped in layout children and resolves to none on lookup rather than panicking; a stale animation entry for a removed node is skipped on the next resolve. · guard: stale_interaction_state, stale_node_mapping, animations and custom_widget_layout tests
contract/refusal-causes/covered · Driver refusal causes · covered — The target is covered at its hit point; its fixed meaning states the hit's limits, among them that an option of a closed select reads covered, remedied by clicking the select first.
contract/refusal-causes/off-screen · Driver refusal causes · off-screen — The element lies outside the visible part, including outside a scrolling box that holds it; its remedy names the `scroll` verb.
contract/wpt-results/crash · WPT runner results · CRASH — A test that panics is caught and reported as a CRASH result.
contract/wpt-results/fail-uncaught-js-error · WPT runner results · FAIL with Uncaught JS error — A harness test that never reports but had uncaught JS errors yields FAIL with an "Uncaught JS error" subtest.
contract/wpt-results/timeout · WPT runner results · TIMEOUT — A harness test that never reports and had no uncaught JS errors yields TIMEOUT.
decision/dropped-anonymous-block-clears-parent · A dropped anonymous block clears its children's parent — Dropping an anonymous block clears the layout parent of every child still naming it, so a key press on a control whose block was dropped does not panic and still acts. · why: So the id does not go stale, curing every reader of a stale layout parent. · guard: key_press_after_container_lost_its_box
contract/fragment-navigation/unknown-target · Fragment navigation · unknown fragment target — An unknown fragment target returns None or false and leaves scroll at 0.

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
