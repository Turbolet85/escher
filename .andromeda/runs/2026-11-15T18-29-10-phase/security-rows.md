# security-plan · the rows that bind 2026-11-15-taffy-from-upstream-git — picked 44 of 550 (19 by every draw) · rule any · pin 1162

## input-validation · ## Input Validation
decision/inline-box-heights-finite · Inline-box heights kept finite — Inline-box heights are kept finite. · why: Huge author lengths cannot stall the line breaker.
decision/non-invertible-transform-no-hit · Non-invertible transform takes no hit — In every document, the whole subtree of a transform whose determinant is 0 or not finite takes no hit. · why: An engine change for every document, ratified. · guard: hit_non_invertible_transform
decision/visible-region-through-transforms · Visible region read through transforms — A clipping box's visible region is read through its transforms, exact under translate and scale, a rotation reading the rotated box's bounding box, and a mapped box with a non-finite edge answers none. · why: An engine change for every document, ratified. · guard: visible_region_transformed, stand_act_transformed, transformed_client_rect
decision/actions-read-changes-nothing · Actions read runs nothing and moves no session state — The actions read runs nothing and changes no snapshot, focus reading or diff, so it can be called without moving the session. · why: The stand_actions test the_read_changes_nothing_and_holds_no_typed_text_or_mask pins it; agents call the actions verb between steps over socket and MCP
decision/actions-read-shares-target-check · Actions read and verbs share one target check — Every id-taking verb and the actions read meet one shared target check, so what the actions read lists agrees with what a call would be refused. · why: The stand_actions test every_listed_cause_on_the_lean_tasks_is_what_the_call_returns pins that each listed cause equals the refusal the call returns
decision/boundary-widening-rule · Boundary widening rule — A new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
decision/closed-value-field-no-widening · A closed-value field in a ratified carrier is no widening — A new field of a closed value set inside an already ratified carrier is not a widening and needs no ratification; a field holding authored text, an open value set, a new carrier or a new exit still does. · why: Ratified as the standing rule.
decision/dangerous-inner-html-parsed-unsanitized · Dioxus dangerous_inner_html set as raw inner HTML — Dioxus `dangerous_inner_html` is applied as inner HTML of the element, unsanitized. · why: The stand_keyboard and stand_snapshot_state fixtures build their button rows through dangerous_inner_html and depend on the markup being parsed as given
decision/mask-holds-on-every-carrier · Value mask holds on every carrier — A masked value stays masked on every carrier the snapshot reaches: its text form, a diff, the driver's return, the session socket and MCP tool results. · why: Ratified. · guard: stand_act_filled, stand_mcp_masked
decision/multiple-select-keys-and-clicks · Multiple select keys and clicks — In a multiple select, a plain arrow, Home, End or click chooses that row alone, Shift spans from the anchor, Ctrl or Meta moves the current row by key and flips a row by click, and disabled rows are never chosen. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select · except: Ctrl+A is not modelled.
decision/one-stamping-request-constructor · One stamping constructor for sub-resource requests — blitz-dom builds a sub-resource request only through the one constructor that stamps its loading document as the request's initiator. · why: Upstream's blitz-dom has no such lint config, so the rule is a new upstream merge surface. · guard: blitz-dom's clippy disallowed-methods rule on Request::get · except: The stamping constructor itself, under a single expect.
decision/optgroup-options-and-label · Optgroup options and label — A select's options include its optgroup children's options in tree order, and a group's non-empty label is its name in the accessibility tree and a header painted in an open list. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select, select_popup, select_popup_painted
decision/password-file-value-masked · Password and file input values masked — The snapshot reads a password or file input's non-empty value as one fixed marker whatever was typed or chosen, an empty one as empty, never the text, its length or a file's host path. · why: Ratified. · guard: stand_act_filled, stand_mcp_masked · except: The DOM is untouched: a file input's value attribute still holds the host path and form submission still carries it.
decision/password-painted-masked · Password input painted masked — A password input is painted as one bullet per character in every document and through every renderer that paints with the engine's painter, so a typed password never leaves as pixels in a screenshot. · why: An engine change for every document, ratified. · guard: password_painted_masked, stand_act_screenshot · except: The frame depends on the value's length; a file input's paint is unchanged.
decision/select-choice-one-write-path · Every select choice through one write path — Every select choice, by key, click or open list, goes through one write path that marks the changed set and dispatches input then a non-cancelable, bubbling change, a choice of the already chosen option firing nothing. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select, select_popup
decision/select-keys-closed-match · Select keys pass a closed match — A focused select's keys, an open list's keys and a multiple select's keys each pass one closed key match, and every key outside the match does nothing. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select, select_popup
decision/select-list-hostile-markup-finite · Hostile markup under an open select list — A hostile size, thousands of options and very long labels under an open select list keep every box finite and panic nowhere. · why: An engine change for every document and a boundary widening, ratified. · guard: hostile_markup_keeps_every_box_finite_and_panics_nowhere
decision/select-typeahead-private · Select typeahead kept private and short-lived — Select typeahead keeps the typed text in the document's private buffer, starts afresh for another select or after 1 s of the document's clock, and the text is compared only, never kept past 1 s and never carried out. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select
decision/single-select-selectedness · Single-choice select selectedness — In every document, remote pages included, a select without multiple holds its choice as its options' checked state and settles every recorded select by HTML's selectedness setting algorithm: the last chosen, else the first option not disabled. · why: An engine change for every document and a boundary widening, ratified. · guard: select_interaction, stand_act_select · except: An option of a closed single select has no click arm and stays refused covered; a single select's size is not read.
decision/text-extent-read-only-none-when-unresolved · Text extent reader read-only with none answers — The text extent reader is read-only and answers none for an id that does not resolve or a node with no inline layout. · why: The text_extent test an_element_with_no_inline_content_reads_none pins the none answers; the escher-driver lint and stand_lint read it for truncation

## data-protection · ## Data Protection
decision/committed-counter-checks · Committed stand-Counter checks — The only committed kept checks are the two of the repository's scripted stand-Counter sessions, holding no typed text, host path, name or secret and no `screenshot` step, read only through the strict `Check::from_bytes`. · why: The one exception to a kept check's owner-only medium.
decision/coverage-line-no-user-data · Coverage line carries no user data — The coverage answer line holds ids, owner ids, event names and counts only, never a control value, typed text, path or NodeId. · guard: stand_handler_coverage a_typed_sentinel_never_reaches_the_coverage_line
decision/kept-check-actions-answer · Kept check holds the actions answer — A kept actions answer holds ids, owner ids, keyed booleans, the table's verb words and closed cause words, never a control value, typed text or NodeId. · why: A boundary widening, ratified. · guard: stand_replay a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes
decision/kept-check-owner-only · Kept checks are owner-only — A recorded check is written 0600 into a 0700 directory the launcher named, is never logged, and is never removed by escher but for its own check when a recording ends. · why: Ratified. · guard: recording::the_committed_checks_are_what_the_binary_records · except: Two scripted stand-Counter checks, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step.
contract/coverage-line/checks · Coverage answer line · checks — The count of checks replayed.
contract/coverage-line/held · Coverage answer line · held — The count of checks that held.
contract/coverage-line/unexercised · Coverage answer line · unexercised — An array of id, owner and event per unexercised handler, owner left out when absent.

## dependency-security · ## Dependency Security
decision/advisory-ignores-by-id · Advisory ignores by ID only — An advisory is ignored only by its ID, each with a written reason, with no blanket allow and unmaintained and unsound at their defaults.
decision/audit-ignores-per-id-with-reason · Audit ignores per ID with a written reason — Audit ignores are per advisory ID only, each with a written reason, no blanket allow, and each ignore is re-read on every push.
decision/cargo-deny-audit-tool · cargo-deny as the audit tool — cargo-deny 0.20.2, with the root deny.toml, is the dependency audit tool.
decision/dependency-audit-cargo-deny · Dependency audit with cargo-deny — cargo-deny checks advisories and licenses, locked, over the whole workspace's resolved graph with all features for the six platforms CI builds, configured by the root `deny.toml`, ignores per ID only, each with a written reason, re-read on every push. · why: Ratified: audit the whole workspace and check licences. · except: `unsound` stays at its default, which keeps memmap2's unsound advisory out of the gate.
decision/dependency-audit-ci-gate · Dependency audit CI gate — The CI audit job runs the dependency audit on every pull request and push, in the slow tier behind the fast jobs, uncached, its log uploaded on failure.
decision/engine-dependency-pins · Engine dependency pins — The pins stay: Stylo 0.22.0, Taffy the registry 0.15, Dioxus 0.7.3 and winit =0.31.0-beta.3.
decision/licence-gate · Licence gate on every push — The audit leg runs upstream's licences table, held byte for byte in the root deny.toml, on every push over the whole workspace's graph, so the fork gates on licences. · why: An open question, decided as the fork's chosen option. · guard: the audit leg · except: upstream's own licenses job stays reverted
decision/local-audit-before-push · Run the audit leg locally before a push — The local fast leg runs no audit, so the audit leg is run locally before a push. · why: A CI audit can go red on advisories published after a green audit with the lock and deny config unchanged.
decision/locked-builds · Locked builds — Every build, the flake, the bundle and every CI cargo leg, passes --locked so the lockfile is verified. · except: examples/wasm_hello, a standalone workspace with no Cargo.lock
decision/no-git-dependencies · No git dependencies in the root manifest — The root manifest holds no git dependency; taffy and parley are crates.io registry versions with checksums in the lockfile. · except: The taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy still passes the lock, audit and MSRV legs.
decision/no-git-dependency · No git dependency — The workspace takes no git dependency. · except: The taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy still passes the lock, audit and MSRV legs.
decision/upstream-sync-dependency-set · Upstream sync takes the dependency set whole — Each upstream sync takes upstream's dependency set whole or, by cherry-pick, enters no package and takes git-pinned content only by a lock-only, semver-compatible update of a pinned registry crate; its lock is read name by name against upstream's or the prior tree, the audit leg green after it.
decision/workspace-inherited-dependency-versions · Workspace-inherited dependency versions — Dependency versions are inherited from the workspace with workspace = true rather than declared inline in each manifest.
decision/embolden-locked-fast-leg · font-embolden tested by a locked fast leg — The blitz-paint font-embolden feature is tested by a locked cargo leg among the fast legs, with no CI job of its own.

## error-handling · ## Error Handling
decision/ci-matrix-runs-every-job · CI matrix runs every job — CI matrix jobs run with fail-fast off, so one failing matrix job does not cancel its siblings.
decision/stale-node-ids-never-panic · Stale node ids never panic — A stale node id is skipped in layout children and resolves to none on lookup rather than panicking; a stale animation entry for a removed node is skipped on the next resolve. · guard: stale_interaction_state, stale_node_mapping, animations and custom_widget_layout tests

## logging-monitoring · ## Logging & Monitoring
decision/no-screen-content-in-logs · Screen content stays out of logs — No driver call puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer and a coverage check reach no log line at any level. · guard: stand_act_spans, host_log and mcp_log, read at RUST_LOG=trace under both builds

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
