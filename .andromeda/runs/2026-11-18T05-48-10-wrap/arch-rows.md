# architecture · the rows 2026-11-18-capability-statement touches — picked 20 of 1527 · rule any · pin 2036

## standard-contracts · ## Standard Contracts · too large to hand whole: its other boundaries under `also here:`
decision/a11y-leg-widens-without-new-job · Accessibility leg widens in place — The a11y leg gains tests with no new job, no required status check and no workflow change. · guard: LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log · r1 · add-to · F1
decision/absolute-position-ancestor-scroll-only · Absolute position subtracts only ancestors' scroll — The absolute position of a node subtracts only its ancestors' scroll offsets, so a box reads at the same position at every scroll offset of its own, and every caller reads such a box where it stands. · why: Ratified. · guard: scrolled_box_absolute_position · r1
decision/accessibility-feature-gate · Snapshot readers behind the accessibility feature — A crate taking a Dioxus crate with default features off gets the snapshot, its text, its diff and the actionable-key check only by naming the accessibility feature or depending on a crate that names it. · r1
decision/accessibility-tree-refresh-on-change · Platform accessibility tree refreshed on change — On a poll that reported work the shell drains the document's changed set, with or without the accessibility feature, and hands the adapter a fresh tree when the set was non-empty. · why: ratified as refresh, then drain — the shell half of one item with the engine's changed-set contract · r1
decision/actionable-elements-keyed · Every actionable element reads an author key — Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /. · why: A ratified public contract addition. · guard: stand_actionable_keys · r1
decision/adapter-crossing-carries-descriptions · Adapter crossing carries descriptions — Nodes handed to the platform adapter carry descriptions, free page text, to the adapter only; the snapshot reads no description. · why: a widening by upstream merge, ratified as upstream's behaviour · r1
decision/adapter-crossing-carries-states · Adapter crossing carries states — Nodes handed to the platform adapter carry states as closed enums and booleans, never an attribute's value as typed. · why: a widening of the adapter crossing, ratified · r1
decision/advance-moves-session-clock · Advance moves the session's clock — An advance moves the session's clock by exactly the asked milliseconds on every session, and the app's own time through a caller's time step where one was handed; a session with no time step still takes advance, which moves its clock alone; advance reports the clock's move. · why: Ratified. · r1
decision/advanced-ms-is-the-clock-move · advanced_ms reports the clock's move — An advance's advanced_ms is exactly the milliseconds asked, the clock's move, and its diff can be non-empty on any animated page. · why: Ratified. · r1
decision/anchored-id-tier · Anchored id tier reaches one component — An author key anchors the ids of elements below it only inside its own component instance, spelled key//segment; a template root owned by another component restarts at that component's chain. · why: The anchored tier, its key//segment spelling and its one-component reach are ratified. · r1
decision/answers-written-by-own-json-writer · Answers written by the driver's own JSON writer — Every answer the driver gives has one written form, a line of JSON keyed by the schema's own words and written by the driver's own output-only writer; no serialization crate writes one, and serde_json only reads MCP requests and kept checks. · why: Ratified as serde_json for reading only. · r1
decision/aria-invalid-unknown-is-true · Unknown aria-invalid reads invalid — An aria-invalid value outside the allowed list reads as invalid true. · why: WAI-ARIA 1.2's rule for a value outside the allowed list · r1
decision/aria-state-values-exact-lowercase · ARIA state values matched exactly in lower case — ARIA state values are trimmed of ASCII whitespace and compared exactly in lower case, so an upper-case value reads unrecognized. · why: a choice: the specification as read states no case rule and no browser was measured · r1
decision/blitz-tests-single-test-target · blitz-tests manifest declares one test table — The blitz-tests package keeps test autodiscovery on and declares exactly one test table, `all`, with test set to false. · guard: BlitzTestsTargetsTest · r1
decision/boa-script-engine · Boa runs document scripts — A document's scripts run on Boa, JS listeners before Blitz default actions. · r1
decision/bounded-package-install · Bounded package install — Every fork CI package install goes through the one install script: up to 3 attempts, each update and install under the timeout bound BOUND=120 with the kill grace GRACE=10, pausing 10 s when another attempt remains. · guard: InstallScriptTest; test_o_package_installs_are_bounded · r1
decision/bounded-retried-package-install · Bounded, retried package install — Every CI package install goes through one bounded, retried install script; no ci.yml line calls apt-get. · r1
decision/capability-statement-matches-schema · Capability statement held to the driver schema — The capability statement and README name every verb and refusal cause and state in words the counts of verbs, refusal causes, session errors, node keys and exit statuses that the driver schema holds. · guard: CapabilityStatementTest · r1 · add-to · F1
decision/capture-dispatcher-pair · Capture's process-wide dispatcher pair — Where the held default has no capture layer, the log capture uses a process-wide pair of scoped dispatchers built once and never dropped, one used and a twin never used, so tracing-core never falls into its one-dispatcher interest mode. · why: Per-thread dispatchers die with their threads and leave tracing-core in its one-dispatcher mode; they were rejected. · r1
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of the CI leg script runs with `--locked`. · except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock` · r1
decision/cells-grid-single-tab-stop · Cells grid is one Tab stop with arrow and Enter editing — The cells-grid is one Tab stop whose arrows move the current cell and whose Enter opens the editor, an editor Enter committing and returning focus to the grid. · why: Pinned by the stand_keyboard Cells walk and stand_cells_served, which read focus back on cells-grid after an editor Enter · r1
decision/cells-not-a-lean-task · Cells is not a lean task — The Cells task boots through its own stand entry and is kept out of the lean task set. · why: Its screen passes the snapshot text's budget and reads in parts. · r1
decision/changed-set-refresh-then-drain · Changed set, refresh then drain — A document's changed set holds every tracked write until drained, node creation marking nothing; the windowed shell drains it, and a drained id is read as possibly dropped, never by indexing. · why: ratified; the engine half of one item with the shell's refresh on change, changing behaviour for every Blitz document · r1
decision/ci-contract-pinned · CI workflow contract pinned by tests — The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg. · guard: test_ci_workflows.py · r1 · add-to · F1
decision/class-atoms-synced-on-write · Class atoms kept in sync on class attribute writes — Stylo's class matching reads the element's class atoms, kept in sync on every class attribute write. · why: Stylo's has_class and each_class matching read the atoms, and every class-toggling Dioxus page writes class through the mutator · r1
decision/client-rect-under-transforms · Client rect read through transforms — The client bounding rect and client rects of an element read it where its transforms move it: the border box mapped through its own and every containing block's transform, read as the axis-aligned bounding box; with no transform on the chain the reading is the untransformed one. · why: Matches CSSOM View getBoundingClientRect; ratified. · guard: transformed_client_rect · r1
decision/cold-agent-isolated-session · Cold-agent run is one isolated session — A cold-agent run starts one isolated non-interactive Claude session from a fresh temporary directory with a fixed prompt and only the escher server's tools, under a budget and time cap. · why: ratified · guard: test_cold_agent.py · r1
decision/cold-agent-six-apps · Cold-agent prompt names six apps — The cold-agent prompt names six apps (counter, flight booker, timer, CRUD, cells and the load page) and their goals, naming no stand id. · r1
decision/cold-agent-wrong-call-bar · Cold-agent wrong-call bar — A cold-agent run passes only when wrong calls times 100 is at most 5 times the tool calls. · r1
decision/coverage-excludes-keyboard-compat · Coverage excludes the keyboard compat bridge — Both coverage measurement lines exclude dioxus-native-dom's crate-private keyboard compat module, the floors unchanged. · why: The module is upstream's temporary keyboard-types 0.7/0.8 bridge, excluded until escher takes upstream's retirement of it. · guard: test_coverage_leg_reaches_the_floor_on_each_crate · r1
decision/diff-content-never-logged · Diff content never logged — A diff's content never reaches a log or event; only its three counts of added, removed and changed may be recorded on the driver's command span. · why: Ratified. · r1
decision/diff-not-engine-changed-set · Diff separate from the engine's changed set — The diff compares two snapshots and is not derived from the engine's changed set; a set changed flag beside an empty diff is a designed case. · why: The changed set is the cheap signal that something was written; the diff is what a snapshot reads differently. · r1
decision/double-click-window · Two clicks without an advance read as a double click — Two clicks on one point with no advance of at least 500 ms between them read as a double click, as the click verb's help says. · why: Ratified. · r1
decision/driver-logs-no-content · Driver logs no content — Nothing in escher-driver logs an id, a name or a value. · r1
decision/each-aria-attribute-read-once · Each ARIA attribute read once — The accessibility builder reads each attribute once, keeping the trimmed aria-label and the closed-set state reads over upstream's. · why: differs from upstream, whose bool-parsed aria-selected and unfiltered aria-label reads were dropped in the merge · r1
decision/falsy-boolean-attribute-cleared · Falsy boolean attributes cleared — A falsy value of one of the 28 boolean attribute names of upstream's predicate clears the attribute on both the dynamic and the static template path; a truthy value is still written, and an unlisted attribute holding false is written as "false". · why: The project's own list of 27 names was retired for upstream's predicate, leaving the mutation writer identical to upstream's. · guard: dioxus_falsy_boolean_attrs · r1
decision/file-url-load-rule · File URLs only for file documents — A `file:` URL loads only for a `file:` document or as a top-level load — in the browser app a navigation a non-`file:` page starts is none, dropped silently; script loads read a refusal as Forbidden, and blitz-net refuses any other engine request with FileForbidden before reading the disk. · r3
decision/font-embolden-local-witness · Local witness for font-embolden paint — The font-embolden feature is built and tested locally by the embolden step of the fast gate, with no CI job; macOS exercises the feature in CI. · why: Running the painted select-popup test under the feature on linux reproduces macOS's two readings. · guard: test_fast_leg_ends_on_the_font_embolden_witness · r1
decision/fresh-list-survives-focus-restyle · A just-opened list survives the focus restyle — A select list opened by a click is held fresh against the restyle of a :focus rule, so a rule that moves the select's box cannot close the list the click opened. · why: A click focuses and opens in one step, before the focus rule's restyle. · guard: a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened · r1
decision/geometry-transform-parse-stylo · Stylo parses geometry transforms — The script geometry interfaces delegate transform parsing to the native side, which uses stylo's CSS parser. · r1
decision/graph-signal-hook-in-seven-guis · Graph signal hook lives in seven_guis — The Signal form of the graph-state hook lives in the stand application, not in the Dioxus DOM bridge. · why: dioxus-native-dom names no crate that defines Signal. · r1
decision/hand-written-answers · Driver answers written by hand — Every driver answer is written by hand in a crate-private module; no serialization crate writes one, serde_json only reading MCP requests and kept checks. · r1
decision/harness-counts-loads-at-net-provider · Harness counts loads at the net provider — A supplied non-noop net provider is wrapped in a counting provider that counts a request in flight until its handler is consumed or dropped and forwards errors; an offline harness or a wrapped document is read through its pending critical resources alone. · why: Loads are counted where every request crosses. · r1
decision/harness-reexports-key-types · Harness re-exports keyboard types — The test harness re-exports the keyboard types a key press is named with. · why: A crate depending on the harness names a key without a keyboard dependency of its own. · r1
decision/input-helpers-do-not-settle · Input helpers do not settle — Pump, tick, advance-time, dispatch and every input helper settle nothing; settling is asked for separately. · r1
decision/is-select-public-re-export · Select test public and re-exported — The engine's select test is public and re-exported so the driver can test for a select. · why: escher-driver has no blitz-dom dependency to name local_name! with; the masked-input test is the precedent · r1
decision/js-engine-boa · Boa as the JavaScript engine — Script documents run HTML with JavaScript enabled on a Boa-based engine. · r1
decision/keyed-wire-form · keyed as the actionable-key wire form — ActionableNode's keyed reading, true where the id holds no /, is the actionable-key check's first wire form, carried by the actions verb on every carrier. · r2
decision/layout-answer-crossing · Layout answer crosses the carriers — The layout answer crosses the session socket's call, the command line and MCP and rests in a kept check as the layout verb's answer, holding stable ids, closed words and numbers only, never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, node key, span field or dependency. · r1
decision/lint-own-bounds-type · Lint's own rectangle type — Every lint rectangle is the lint's own bounds type in the snapshot bounds' space, and lint types hold no engine node id, page text, colour or value. · why: the driver crate names neither blitz-dom nor stylo · r1
decision/listener-mark-outlives-listener · Bridge listener mark outlives its listener — The bridge's event attribute stays whether a listener leaves the attribute list or turns to None. · why: Pinned by a_listener_removed_on_a_re_render_is_read_by_neither_reader, which asserts the mark stays so readers must use the vdom walk · r1
decision/listeners-read-from-vdom · Listeners read from the vdom walk — Every reader of an element's listeners reads the vdom walk's current listeners, never the bridge's listener mark. · why: The listener mark over-reports: a listener a re-render removes leaves its attribute and mark standing, and a mounted listener writes none. · r1
decision/load-page-message-hidden-until-sheet · Load button links one sheet that reveals Loaded — The load page's Load button links one stylesheet, and the Loaded paragraph stays hidden until that sheet lands. · why: Pinned by the_load_page_click_waits_out_its_load_and_shows_the_message and the_load_page_enter_waits_out_its_load_and_shows_the_message · r1
decision/masked-input-values · Password and file input values masked — A password or file input's value reads MASKED_VALUE when non-empty and empty when empty, never the text, its length or a file's host path. · guard: stand_masks_on_new_answers · r2
decision/mcp-current-revision-not-taken · Current MCP revision not taken up — The MCP server does not take up the current MCP revision; a handshake asking it is answered with the latest of the four revisions the server speaks. · why: Deferred by decision, left among the remainders. · r1
decision/measured-limits-stated-in-the-tool · Measured limits stated where an agent reads the tool — A measured limit of the driver is stated where an agent reads the tool, in the fixed help texts of the verb table, the cause texts and the node keys' help. · why: Ratified. · r1 · stale
decision/mixed-toggle-reads-unchecked · Mixed toggle reads unchecked — A checkable-role node whose toggle is mixed reads checked as Some(false). · why: Ratified. · r1
decision/multiple-removal-settles-one-option · Removing multiple settles the chosen set to one — Removing multiple from a select through the mutator settles its chosen set to one option. · why: Pinned by multiple_removed_settles_the_set_to_one_option; the Dioxus bridge removes attributes through the mutator · r1
decision/nested-scroll-written-instantly · Nested scrolling boxes are written instantly — When scrolling into view, a nested scrolling box is written at once whatever behaviour was asked, and the requested behaviour applies only to the viewport, whose offset is computed after the nested writes. · why: The document holds one scroll animation at a time. · r1
decision/no-base64-crate · No base64 crate — A screenshot's PNG base64 is written by hand in the driver, RFC 4648 §4 padded; no base64 crate is taken. · r1
decision/no-echo-of-typed-input · No echo of typed input — No driver command's ending writes back anything the caller typed. · except: caused carries an app's own messages and fields as their author wrote them, error included, scrubbed by the answer's own hidden field names, so an app that writes typed text into a record echoes it. · r3
decision/no-node-id-crosses-the-socket · Calls and outcomes cross as lines, no NodeId — Across the socket a call travels as a request line and its outcome or refusal comes back as one line of JSON, and no engine NodeId is in either. · why: Ratified. · r1
decision/no-script-text-selection-readers · Script text readers unused — escher uses none of the script text-selection and innerText readers. · r1
decision/no-seed-on-carriers · No seed crosses the driver's carriers — No run seed crosses the driver's carriers: no verb, argument, command-line flag, MCP tool field, kept-check key or span field carries one, so every driver session and every kept check's replay reads the default seed; a seed reaches a run only from in-process code that builds its VirtualDom. · why: A seed on a carrier would widen the session boundary; the absence is ratified. · r3
decision/no-snapshot-in-logs · No snapshot content in logs — No log or event carries the snapshot, its text or a diff. · except: a diff's three counts, added, removed and changed, on escher-driver's command span. · r1
decision/no-wall-time-outside-clock · No direct time reads in blitz-dom — No Instant::now() or SystemTime::now() stands in blitz-dom outside its clock module. · r1
decision/one-actionable-rule · One actionable rule — Both actionable readers read one actionable rule: focusable, an interactive role, or current listeners holding an event other than mounted. · r1
decision/one-axis-clip-readings-kept · One-axis clip readings kept as measured — A box clipped on one axis stops the hit at its whole padding box while the visible region narrows only on the clipped axis, so a driver click on a target outside along the visible axis is refused as covered; neither reader is changed. · why: Ratified to measure and record the difference rather than move either reader. · guard: stand_act_obstructed · r1
decision/one-ci-leg-runner · One CI leg runner — Every linux CI job runs its leg through one bash leg runner, the same command the dev host runs. · except: The cross-platform test matrix, its linux android entry included, runs cargo or cross directly. · r1
decision/one-disabled-reading · One shared disabled reading — Focusability, the pointer gates and the activation keys all share one reading of disabled: a button, input, select or textarea carrying a disabled attribute. · r1
decision/one-self-description-for-cli-and-mcp · One self-description for command line and MCP — The command line's help and the MCP server's help tool serve the same bytes, the program's own fixed text read from the schema's tables, reading no environment and printing nothing. · why: Ratified as the one table riding the ratified carriers. · r1
decision/one-virtual-clock · One virtual clock per harness — The harness holds one virtual clock its built documents share; only tick and advance-time move it, a negative or non-finite step moving nothing, and screenshot, text contrast and settle never move it; a wrapped document keeps its own clock. · why: Ratified. · r1
decision/package-install-repair-before-retry · Repair before a package install retry — Every attempt after the first starts with a bounded `dpkg --configure -a` repair, and a failed repair fails that attempt. · why: A bound that fires inside an install can leave dpkg interrupted, which the next attempt's repair configures. · guard: InstallScriptTest · r1
decision/password-caret-placed-by-bullets · Password caret placed by bullets — A click inside a password field places the caret by the laid-out bullets, so no width of the real characters reaches the caret; a double click selects the whole value. · why: Ratified. · guard: password_caret_click · r1
decision/password-input-painted-masked · Password inputs paint masked — A password input paints a bullet per character in every document and through every renderer using blitz-paint, so two values of one length paint byte-identical frames, focused or not, with the caret and field scroll measured on the bullets. · why: Ratified. · r1
decision/per-crate-coverage-floor · Per-crate coverage floor — CI holds a coverage floor per crate. · guard: coverage_floor.py · r1
decision/reached-record-before-dispatch · Reached record written before dispatch — The reached record takes each node of the dispatch chain whose current listeners hold the event before the event runs, holding no borrow across the render. · why: So a handler whose own run removes its element still counts. · r1
decision/refusal-causes-closed-at-eight · Refusal causes closed at eight — The set of refusal causes is closed at eight, each with a fixed name, meaning and remedy, and a refusal holds nothing a call supplied. · why: Ratified. · r1
decision/refused-call-records-no-ids · A call validation refuses records no ids — A call the schema's validation refuses records nothing in the session's record of seen ids; every snapshot a validated call takes is recorded, on a refusal too. · why: Ratified. · r1
decision/render-with-document-unborrowed · Render with the document unborrowed — The vdom renders into a mutation buffer with the document unborrowed; one borrow then replays the buffer and is dropped before mounted events flush. · why: So user code reaching the document through MountedData during a render hits no RefCell already-borrowed panic. · r1
decision/replay-hook-after-each-step · Replay calls a hook after each replayed step — Handler coverage relies on the replay calling a hook after each replayed step, so the screen can be read between steps. · why: The driver's coverage role passes a hook to replay_each that reads the screen's reached handlers between steps · r1
decision/resolve-reconciles-select-list · Resolve reconciles the select list before hover — Resolve reconciles the select list after layout and before refreshing hover, and removing a select clears its list. · why: Pinned by a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes and a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened · r1
decision/screenshot-png-in-band · Screenshot image carried in band — A screenshot's PNG crosses inside the answer line as base64, and on MCP as an image block in the tool result. · why: Ratified as the chosen fork. · r1
decision/screenshot-reads-session-clock · Screenshots read the session's clock — The overlay scrollbar fade reads the session's clock, so two screenshots of one state with no advance between them read the same pixels. · why: Ratified. · r1
decision/scrolled-box-bounds · Scrolled box bounds — A scrolled box reads the rect it stands at, the same before and after its content is scrolled; only the scroll offsets of boxes above it on the containing-block chain move it. · why: Fixed on a ratified ruling. · guard: scrolled_box_client_rect · r1
decision/seen-ids-record-bounded · Session's record of seen ids bounded — Between calls a session keeps of its screens only id text, at most 4096 ids, never an id longer than a call can name, forgetting the one read longest ago past the bound; the record has no public reader and is never printed or logged. · why: Ratified for ids longer than a call can name. · guard: the_record_holds_no_id_longer_than_a_call_can_name · r1
decision/select-choice-single-write-path · Select choice has one write path — Every change of a select's chosen options goes through the one select-choice write path, which takes a set of options. · r1
decision/select-list-colours-engine-constants · Select list colours are engine constants — The open select list's highlight text, highlight background, disabled text and border colours are engine constants on the list view, not design tokens. · r1
decision/select-list-rows-and-placement · Select list shows at most 20 rows, placed where they fit — An open select list shows at most 20 rows, opens where its rows fit or on the roomier side, and a select with no row to show opens nothing. · why: Pinned by a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel, a_disabled_inert_hidden_singular_or_empty_select_opens_nothing and the place unit test · r1
decision/select-tests-name-element · Select tests name the element — The select tests the driver reads name the element and never the role. · r1
decision/session-apps-one-table · Session apps from one table — The escher-session usage line's apps and the MCP start tool's enum are written from one private table. · r1
decision/session-moves-clock-not-time-step · Session, not the time step, moves the clock — A stand's time step only delivers the whole timer ticks the asked milliseconds hold, carrying the remainder across calls; the driver session, not the step, moves the harness's clock. · why: Ratified. · r1
decision/settle-never-waits-on-time-or-loads · Settle never waits on loads or time — Settle drives render and layout to quiet, reports a load in flight and never waits on one, and neither waits on nor advances a source only time moves; it reads no clock but the harness's own, logs nothing and starts no thread. · r1
decision/snapshot-text-budget · Snapshot text budget — One screen's text stays under SNAPSHOT_TEXT_BUDGET, 10,000 bytes, a ceiling callers compare against and text_parts cuts within, never enforced by to_text. · why: Keeps a screen inside the agent client's documented 10,000-token warning, 25,000-token maximum and 50,000-character file threshold. · r1
decision/snapshot-text-never-logged · Snapshot text never logged — The snapshot text leaves the process only as a returned value or a hosted call's answer; no log or event carries it, and a kept check holds it only in an owner-only file never logged. · why: Ratified. · except: The two committed checks are the one exception to owner-only. · r1
decision/stale-mounted-event-skipped · Stale mounted event skipped — A mounted event queued for an element removed before the flush is skipped. · why: Differs from upstream: upstream's test for it is left out, as Dioxus 0.7 delivers one mounted event where the test asserts none. · r1
decision/stand-offline · Stand boots offline — Every stand boot is offline, with no net provider and no base URL. · guard: the_stand_options_stay_offline · except: The load page boot replaces the net provider with its own network stand-in. · r1
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable. · r1
decision/stand-readings-same-on-every-ci-os · Stand readings identical on every CI OS — The six served stand tasks read the same driver answer digests of snapshot, lint and actions on linux x86_64, windows x86_64 and macOS aarch64, in both layout modes, at boot and after one step, compared with no screenshot. · guard: stand_cross_os_readings · r1
decision/stand-typography-dejavu-sans · Stand typography is DejaVu Sans — Every stand boot loads one font, the bundled DejaVu Sans, with system fonts off, so every generic family and any unavailable named family lays out in DejaVu Sans by fallback; the typography is stated in the stand contract, not on the design-system product surface. · guard: stand_bundled_font_families · r3
decision/toggle-read-per-role · Toggle state read per role — aria-checked sets the toggle only on checkable roles and aria-pressed only on others, and a native checkbox or radio reads neither, its native checkedness winning. · why: ratified; native checkedness winning follows HTML-AAM · r1
decision/tool-call-span-tool-and-ending · MCP tool call span with tool and ending — Each MCP tool call leaves one span carrying only the tool's table word and its ending, the command span of a session run nested under it. · why: Ratified. · r1
decision/transformed-element-bounds · Transformed element bounds — An element under a transform reads its bounds where its transforms move it, so a driver click on a translated button is accepted and lands. · why: Ratified. · guard: stand_act_transformed · r1
decision/type-replaces-control-content · Type replaces what a control holds — The type verb replaces what the target control holds, selecting all and typing over it, and an empty text clears the control; a target that reads no value is typed into as is. · why: Chosen in place of appending the text after the old value. · r1
decision/upstream-only-workflows · Upstream-only workflows — The WPT, WPT post-results and publish workflows, and the Signed Builds and WPT environments, run only on DioxusLabs/blitz; the fork builds no bundle. · r1
decision/validated-commands-only · Driver commands run only after validation — Every driver call runs through the session's one run entry, which validates first; nothing public runs a command that did not come from validation. · r1
decision/validated-verb-set-widened-by-ratification · Validated verb set widened only by ratification — The driver's validated verb set is closed; a verb joins it only as a ratified widening of that set, at the instance, session or tool level. · why: Each verb beyond the first acting verbs joined as a ratified widening of the validated verb set. · r1
decision/visibility-from-geometry-not-hit · Visibility is read from geometry, never from a hit — Whether an element can be seen is read from the visible-region geometry of its clipping chain and never from a hit test. · r1
decision/windows-entries-pass-through · Windows script entries are pass-throughs — The Windows entries of the agent-run and cold-agent pipes are pass-throughs with no logic. · r1
contract/acting-verb-results/added · acting verb results · added — Nodes on the screen after the step that were not before, written without differs. · r1
contract/acting-verb-results/busy · acting verb results · busy — One of the busy classes, present only when settled is false. · r1
contract/acting-verb-results/caused · acting verb results · caused — Present only when the step caused something: an item per panic, error or record: kind, level, source, target, message cut at 1024 bytes, location, an error's boundary and chain, a log's fields, error kept, first reached handler; panics, then errors new since the boundary marks, then records; 16 kept · r3 · stale
contract/acting-verb-results/caused-past · acting verb results · caused_past — Written after caused only when non-empty on click, type, press, advance and scroll: one entry per kind and source counting the items past caused's 16, a panic or error entry writing no source. · r1
contract/acting-verb-results/changed · acting verb results · changed — Elements before and after the step whose role, name, state, bounds or parent differ, as they read after it, each carrying differs. · r1
contract/acting-verb-results/removed · acting verb results · removed — The ids of the elements the step removed. · r1
contract/acting-verb-results/settled · acting verb results · settled — A flag; a step that did not go quiet is a result with settled false and its busy class, never a refusal or a session error, and is not rolled back. · r1
contract/ci-leg-legs/a11y · ci-leg.sh legs · a11y — The `a11y` leg runs named accessibility integration tests on one `cargo test --workspace --locked` line; other focus and accessibility tests stay in the `test` leg. · r2 · add-to · F1
contract/ci-leg-legs/audit · ci-leg.sh legs · audit — The `audit` leg runs `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace's graph. · r1
contract/ci-leg-legs/build · ci-leg.sh legs · build — `build` is a leg of the CI leg script. · r1
contract/ci-leg-legs/ci-scripts · ci-leg.sh legs · ci-scripts — `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts. · r1 · add-to · F1
contract/ci-leg-legs/clippy · ci-leg.sh legs · clippy — The `clippy` leg runs the workspace clippy line and, only when that is green, upstream's own blitz-dom clippy line with `-D warnings`. · r1
contract/ci-leg-legs/counter · ci-leg.sh legs · counter — `counter` is a leg of the CI leg script. · r1
contract/ci-leg-legs/coverage · ci-leg.sh legs · coverage — The `coverage` leg runs `cargo llvm-cov` to an lcov file and its per-file report, then the coverage-floor script, and exits with the floor's status. · r1
contract/ci-leg-legs/doc · ci-leg.sh legs · doc — The `doc` leg runs `cargo doc --workspace --no-deps`. · r1
contract/ci-leg-legs/embolden · ci-leg.sh legs · embolden — The `embolden` leg runs `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, has no CI job, and is the last step of `fast`. · r1
contract/ci-leg-legs/fast · ci-leg.sh legs · fast — The `fast` leg runs fmt, clippy, test, ci-scripts and embolden in order, stopping at the first red: the local pre-push gate. · r1
contract/ci-leg-legs/fmt · ci-leg.sh legs · fmt — The `fmt` leg runs `cargo fmt --all --check`. · r1
contract/ci-leg-legs/msrv · ci-leg.sh legs · msrv — The `msrv` leg runs `cargo +1.91 build`. · r1
contract/ci-leg-legs/stand-repeat · ci-leg.sh legs · stand-repeat — The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`. · r1 · add-to · F1
contract/ci-leg-legs/test · ci-leg.sh legs · test — `test` is a leg of the CI leg script. · r1
contract/ci-leg-legs/wasm · ci-leg.sh legs · wasm — `wasm` is a leg of the CI leg script. · r1
contract/headless-stand/boot · seven_guis headless stand · seven_guis::stand — seven_guis::stand, native only, boots the four lean tasks, Cells, a load page and one component alone in a pinned 800x600 Light viewport with one bundled font. · r2 · stale
contract/headless-stand/no-telemetry · seven_guis headless stand · telemetry and environment — The stand installs no telemetry, reads no env var and names no blitz_net. · r1
contract/headless-stand/offline-boots · seven_guis headless stand · network — Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms). · r1
contract/headless-stand/start-in · seven_guis headless stand · start_in — A start in a state refuses a name outside its lean task's closed fixture table before anything boots; Crud holds fourteen-people and person-selected, Flight Booker return-flight, and Counter and Timer none. · r1
contract/headless-stand/timer-step · seven_guis headless stand · timer_step — timer_step hands the Timer its ticks for a driver session. · r1
contract/telemetry-scrub/content-span-fields · Telemetry scrub · content span fields — A span field named in the content fields is never stored; other span fields are held in memory until close and written only as the rule prints them. · r1 · stale
contract/telemetry-scrub/engine-targets · Telemetry scrub · engine targets — An event under an engine target prefix prints only the safe fields; every other field, message included, prints as [redacted]. · r1 · stale
contract/telemetry-scrub/escher-targets · Telemetry scrub · escher targets — An event under the escher_ prefix prints every field except content fields, which are redacted. · r1 · stale
contract/telemetry-scrub/other-targets · Telemetry scrub · other targets — An event or span of any other target writes zero bytes at every level, whatever RUST_LOG names. · r1 · stale
absence/component-alone-on-carriers — A component alone on the driver's carriers: no verb, argument, MCP tool, socket call or kept check reaches it · r1
absence/exploration-follows-navigation — Exploration that follows navigation to another document · r1
absence/exploration-on-carriers — Exploration on the driver's carriers: no verb, role, argument, MCP tool, session socket call or kept check reaches it · r1
absence/fixture-state-on-carriers — A fixture state on the driver's carriers: the command line, MCP, the session socket or a kept check · r1
absence/script-same-origin-cors — Same-origin and CORS checks on script loads · r1
absence/settle-verb-on-socket — A settle verb or a busy-source reply of its own on the session socket · r1

also here:
- Accessibility tree · 14 row(s)
- blitz-dom accessibility tree · 7 row(s)
- accesskit_xplat adapter · 3 row(s)
- Actionable element · 1 row(s)
- Actionable nodes · 1 row(s)
- Dioxus actionable readers · 2 row(s)
- The actions verb · 6 row(s)
- agent-run.sh JSON-line events · 6 row(s)
- agent-run.sh exit codes · 4 row(s)
- agent-run.sh state · 1 row(s)
- answer JSON · 13 row(s)
- App CLIs · 3 row(s)
- apt-install.sh arguments · 2 row(s)
- apt-install.sh exit codes · 3 row(s)
- apt-install.sh stderr · 1 row(s)
- argument kinds · 7 row(s)
- BaseDocument · 8 row(s)
- BaseDocument readers · 1 row(s)
- blitz-dom clock API · 3 row(s)
- blitz-dom crate root · 3 row(s)
- blitz-net Provider · 1 row(s)
- blitz-traits net types · 1 row(s)
- Browser about URLs · 4 row(s)
- Browser history · 3 row(s)
- Browser toolbar · 1 row(s)
- busy classes · 3 row(s)
- Check replay · 3 row(s)
- ci-leg.sh invocation · 3 row(s)
- fork CI log artifacts · 2 row(s)
- CI contracts · 1 row(s)
- Command line coverage role · 5 row(s)
- CLI exit statuses · 4 row(s)
- Command line usage line · 1 row(s)
- cold-agent.sh command line · 1 row(s)
- cold-agent.sh JSON-line events · 4 row(s)
- cold-agent.sh exit codes · 4 row(s)
- cold-agent recording · 1 row(s)
- cold-agent.sh state · 1 row(s)
- cold-agent verdict reasons · 3 row(s)
- cold-agent verdict.json · 5 row(s)
- cold-agent verdict · 3 row(s)
- command line · 6 row(s)
- command span · 9 row(s)
- coverage_floor.py exit codes · 3 row(s)
- coverage_floor.py output · 1 row(s)
- Script CSSStyleDeclaration · 1 row(s)
- CSSOM · 1 row(s)
- Default script fetcher · 1 row(s)
- Dioxus DOM bridge · 9 row(s)
- Dioxus form events · 2 row(s)
- dioxus-native Document context · 1 row(s)
- dioxus-native-dom public API · 6 row(s)
- dioxus-native-dom · 1 row(s)
- dioxus-native launch · 1 row(s)
- blitz-dom document clock · 3 row(s)
- DocumentConfig · 2 row(s)
- blitz-dom document core · 5 row(s)
- DocumentEvent messages · 2 row(s)
- Document trait · 3 row(s)
- DomEventKind · 1 row(s)
- DOM events · 2 row(s)
- DOM event dispatch · 9 row(s)
- escher-driver explore · 6 row(s)
- driver form-control readers · 6 row(s)
- Driver JSON answer · 1 row(s)
- Driver public API · 1 row(s)
- escher-driver recording · 3 row(s)
- escher-driver session · 9 row(s)
- driver verbs · 15 row(s)
- escher-session command line · 25 row(s)
- escher-session exit codes · 4 row(s)
- escher-session host role exit codes · 2 row(s)
- escher-session mcp role exit codes · 2 row(s)
- Event dispatch · 9 row(s)
- EventState · 1 row(s)
- Example CLIs · 6 row(s)
- blitz-dom form control readers · 4 row(s)
- Form submission · 1 row(s)
- Geometry · 1 row(s)
- blitz-dom geometry readers · 5 row(s)
- Graph registry · 3 row(s)
- The graph verb · 5 row(s)
- Handler coverage reader · 6 row(s)
- blitz-html and blitz-net crates · 2 row(s)
- HTML open attribute · 1 row(s)
- Kept check reader · 2 row(s)
- Kept check file · 9 row(s)
- key names · 12 row(s)
- high-level launch · 1 row(s)
- blitz-dom layout reason · 4 row(s)
- blitz-dom layout · 2 row(s)
- lint reason keys · 6 row(s)
- The lint verb · 11 row(s)
- Driver acting step · 3 row(s)
- MCP server · 12 row(s)
- MCP server tool result · 1 row(s)
- blitz-dom mutation, query and CSSOM · 3 row(s)
- NetHandler · 1 row(s)
- NetProvider · 1 row(s)
- Network resources · 1 row(s)
- node keys · 14 row(s)
- blitz-dom nodes and widgets · 4 row(s)
- blitz-paint · 3 row(s)
- PR body WPT results section · 2 row(s)
- blitz-traits provider traits · 3 row(s)
- Reached record · 5 row(s)
- Session recording · 2 row(s)
- refusal causes · 8 row(s)
- refusal faults · 5 row(s)
- refusal JSON · 1 row(s)
- Resolved style · 2 row(s)
- root crate features · 1 row(s)
- ScreenshotError · 2 row(s)
- Script document · 1 row(s)
- Script event handling · 1 row(s)
- Script fetcher · 1 row(s)
- Script globals · 4 row(s)
- blitz-vibey-script · 4 row(s)
- BaseDocument scroll and hit testing · 8 row(s)
- Select list default actions · 8 row(s)
- BaseDocument open select list · 6 row(s)
- BaseDocument select readers · 5 row(s)
- Selector API · 1 row(s)
- session client · 2 row(s)
- session error JSON · 1 row(s)
- session errors · 15 row(s)
- session host · 3 row(s)
- session wire · 11 row(s)
- ShellProvider · 1 row(s)
- Snapshot diff · 6 row(s)
- Snapshot node keys · 1 row(s)
- Snapshot text · 6 row(s)
- Snapshot · 16 row(s)
- Socket host reply · 1 row(s)
- Stable element ids · 12 row(s)
- blitz-dom StylesheetHandler · 1 row(s)
- Per-verb target check · 3 row(s)
- escher-telemetry capture · 1 row(s)
- Telemetry init · 3 row(s)
- Telemetry line · 2 row(s)
- Telemetry panic event · 1 row(s)
- escher-telemetry bootstrap · 6 row(s)
- Test harness · 3 row(s)
- blitz-test-harness · 5 row(s)
- the tool_call span · 1 row(s)
- UiEvent · 1 row(s)
- UI graph · 8 row(s)
- Actionable-key check · 2 row(s)
- call validation · 1 row(s)
- Driver verb table · 1 row(s)
- Viewport · 1 row(s)
- WPT attribute tests · 1 row(s)
- WPT CI artifacts · 2 row(s)
- wpt diff JSON entry · 8 row(s)
- wpt_diff_to_pr.py command line · 6 row(s)
- wpt_diff_to_pr.py output · 4 row(s)
- WPT harness status codes · 4 row(s)
- WPT runner network provider · 2 row(s)
- WPT runner outputs · 5 row(s)
- WPT page messages · 2 row(s)
- WPT reftests · 1 row(s)
- WPT repository dispatch · 1 row(s)
- WPT runner result flags · 1 row(s)
- WPT runner result kinds · 5 row(s)
- WPT runner result line · 1 row(s)
- WPT runner result statuses · 5 row(s)
- WPT script fetcher · 3 row(s)
- WPT subtest status codes · 5 row(s)
- WPT test documents · 1 row(s)

## occupied-resources · ## Occupied Resources
decision/agent-state-areas-local · Agent state areas stay local — The agent-run and cold-agent state areas under target are local only and uploaded by no CI job. · r1
decision/cold-agent-run-adds-no-listener · Cold-agent run adds no listener — The live cold-agent run spawns one claude client that reaches escher-session mcp over stdio only; no listener, port or socket is added, and the client's own messaging socket is the client's, opened by no escher code. · why: ratified · r1
decision/cold-agent-run-carries-stand-screen · Live cold-agent run carries the stand's screen — The live cold-agent run sends the stand's served screens to the model provider: element ids, accessible names, control values, snapshot texts and diffs, from the driver's own MCP server. · why: chosen as one run capped at $3, with a JSON check and a retry · r1
decision/committed-stand-checks · Committed stand-Counter checks — Two kept checks of the repository's own scripted stand-Counter sessions, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step. · why: A shot's PNG bytes differ by OS, so no screenshot step is kept. · guard: seven_guis recording, comparing a fresh recording byte for byte with the committed copies · except: The one exception to a kept check's owner-only medium. · r1
decision/coverage-role-in-process · Coverage role runs in process — The coverage role runs in one process with no listener, socket or port, boots one fresh session per named check through the same boot closure as the host, and opens no span of its own. · r1
decision/escher-crates-unpublished · escher crates unpublished — escher-telemetry and escher-driver are publish = false libraries. · r1
decision/headless-documents-sequential-style · Headless documents style sequentially — Sequential style threading stays the default: headless documents of the harness, the stand and the driver never set Parallel; the windowed launch, the browser and the multi-document bench example do. · r1
decision/kept-checks-owner-only · Kept checks on an owner-only medium — A kept check lives on an owner-only medium. · except: the two committed checks under the blitz-tests checks directory. · r1
decision/mcp-over-stdio · MCP role over stdio — The escher-session mcp role speaks over its own stdio, with no listener. · r1
decision/mcp-role-no-listener · MCP role holds one session and no listener — The escher-session mcp role is one process holding at most one session in process, with no listener and no thread of its own, started by the MCP client that spawns it. · r1
decision/mcp-role-over-stdio · MCP role over standard streams — The escher-session mcp role speaks newline-delimited JSON-RPC 2.0 over its own stdin and stdout to the MCP client that spawned it, opening no listener, socket or port. · why: A boundary widening, ratified. · r1
decision/model-provider-only-on-live-cold-agent-run · Model provider reached only on the live cold-agent run — The model provider is reached only by the spawned claude client through the operator's own login on the operator-driven live cold-agent run, never in CI, and no escher code makes the request. · why: ratified · r1
decision/model-provider-operator-only · Model provider reached only on operator runs — The model provider is reached only by the spawned claude client on the operator-driven live cold-agent run, never in CI; no escher code makes the request. · r1
decision/no-escher-allocator-change · No escher binary changes allocator — No escher binary changes the global allocator; mimalloc is the allocator only of the two upstream-only binaries, the WPT runner and the browser app. · r1
decision/one-local-unix-listener · One local Unix-domain listener — The one local listener is the driver session's Unix-domain socket, owner-only in an owner-only state directory, unix only; no second served listener stands. · r1
decision/raw-run-output-never-printed · Raw run output never printed — Raw run output kept in the state areas (merged logs, the session transcript, the server log, the parsed check and the recorded kept checks) stays unscrubbed and is never printed or copied. · r1
decision/record-dir-launcher-named-private · Recording directory launcher-named and owner-only — The recording directory is named only by the launcher's --record option, never an env read, a default or a tool call, verb, argument or result field; created 0700 when absent and refused RecordDirNotPrivate when open to its group or others. · r1
decision/recording-directory · Recording directory — The recording directory is a path the launcher names by --record, never an env read or a default and named by no tool call, created 0700 and refused when open to its group or others. · r1
decision/recording-removes-only-own-checks · Recording removes only its own checks — Escher removes a check only when its recording ends on a failed write or a passed bound, never a file it did not create and never the recording directory. · r1
decision/recording-removes-own-checks-only · Recording removes only its own checks — escher removes a check only when its recording ends on a failed write or a passed bound, never a file it did not create and never the directory. · r1
decision/run-state-areas-local-only · Agent-run and cold-agent state areas local only — The agent-run and cold-agent state areas under target/ are recreated by their run, removed by cleanup with nothing else under target/ touched, local only and uploaded by no CI job. · r1
decision/screen-pixels-in-band · Screen pixels in band — A screenshot leaves in band inside the answer on both carriers, open select labels as painted pixels included, adding no listener or port and leaving the wire's bounds unchanged. · why: A boundary widening, ratified. · r1
decision/session-host-process-model · Session host process model — The escher-session host is one process with one session and one listener; its mcp role holds at most one session in process and its coverage role one fresh session per check, with no listener. · r1
decision/session-host-single-process · Session host is one process with one listener — The escher-session host is one process holding one session and one listener on its main thread, spawning no thread of its own, ended by stop or by an idle expiry of 1,800 s with no answered request. · r1
decision/session-socket-carries-calls · Session socket carries calls — The session socket carries calls and their answers: an element id and typed text cross in; ids, accessible names, masked password and file-input values, snapshot text and diff cross out, never a NodeId. · why: A boundary widening, ratified. · r1
decision/session-socket-one-listener · Session socket the one listener — The driver session's Unix-domain socket is the only listener served: mode 0600 inside a caller-named state directory created 0700, with no second listener and no network port. · why: A boundary widening, ratified. · r1
decision/session-state-dir-caller-named-private · Session state directory caller-named and owner-only — The driver session's state directory is a path its caller names, never an env read or a cwd-relative default; created mode 0700 when absent and refused as StateDirNotPrivate when open to its group or to others. · r1
decision/session-state-dir-removal · Session state directory holds the socket alone — The session state directory holds the socket file session.sock alone and is removed at stop or idle expiry: the socket file, then the directory by a non-recursive remove, nothing else deleted. · r1
decision/session-state-directory · Driver session state directory — The driver session's state directory is a path its caller names, never an env read or a default, created 0700, holding the session socket alone, removed at stop or idle expiry. · r1
decision/stand-load-net-answers-after-delay · Load stand-in answers from its own thread after 200 ms — The stand's LoadNet answers each fetch from a thread of its own with the fixed CSS after 200 ms, with no socket or listener. · why: the_load_page_click_leaves_a_load_in_flight and the stand_load_page checks need a load in flight that the settle waits out · r1
decision/stand-load-page-in-process-url · Load page stylesheet URL under .test answered in process — The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network. · why: The stand_load_page driver checks and the session host's load app rely on the boot answering offline with no resolver reached · r1
decision/telemetry-one-install-site · One telemetry install site per binary — Every role of the seven_guis binaries runs under the one escher_telemetry::init install, which installs the global tracing subscriber, the log bridge and a chaining panic hook and spawns no thread. · r1
decision/test-cache-restore-only · Shared test cache restored, never saved — The a11y and stand-repeat jobs restore the test job's workspace-test cache and never save it. · r1
decision/upstream-ci-resources-upstream-only · Upstream CI resources reached only from upstream — Every job that uses the Signed Builds and WPT environments or the warpbuild WPT runner carries a DioxusLabs/blitz repository guard in its if, so no fork ref reaches them. · r1
contract/actions-answer/argument · The actions verb's answer · argument — The actions verb takes no argument; a call that gives one is refused malformed. · r1
contract/actions-answer/node-fields · The actions verb's answer · node fields — Each actionable node gives its stable id, owner id and keyed boolean, the table's verb words and the closed cause words; never a value, typed text or NodeId. · r1 · contradict
contract/actions-answer/paging · The actions verb's answer · paging — The whole screen is answered unpaged, within the 1,048,576-byte answer bound; the wire adds no listener or port. · r1
contract/command-span/actions-call · The command span · actions call — An actions call's command span records verb alone, and cause when refused, parented by its tool_call span under the mcp role. · r1
contract/command-span/graph-call · The command span · graph call — A graph call's command span records verb alone, parented by its tool_call span under the mcp role. · r1
contract/driver-diff/differs · the driver diff · differs — A changed node's differs names closed node-key words, bounds among them. · r1
contract/driver-graph/answer · the graph verb · answer — On both carriers and in a kept check, the answer holds stable ids, owner ids, component names, handler event names, registered state names, declared writes, the flags undeclared and unconfirmed and booleans; never a value, typed text or NodeId. · r1
contract/driver-graph/argument · the graph verb · argument — graph takes no argument; a call that gives one is refused malformed with nothing run. · r1
contract/driver-lint/answer · the lint verb · answer — On both carriers and in a kept check, the answer holds stable ids and the closed defect words and numbers, never a NodeId, page text, colour or control value. · r1
contract/driver-snapshot/clickable · the driver snapshot · clickable — A snapshot node's clickable is a closed bool, crossing in the snapshot and the diff. · r1
contract/driver-snapshot/row-bounds · the driver snapshot · row bounds — An open select list's row carries its own bounds in the snapshot and the diff. · r1
contract/environment-variables/rust-log · Environment variables · RUST_LOG — escher_telemetry::init reads RUST_LOG as tracing filter directives, defaulting to warn when unset or unparsable. · r1
contract/mcp-role/handshake-revision · the mcp role · handshake revision — The mcp role speaks four handshake revisions, the fourth, of 25 November 2025, its default. · r1
contract/mcp-role/help · the mcp role · help — The help tool is answered by the server with no session run. · r1
contract/mcp-role/screenshot-result · the mcp role · screenshot result — A screenshot result is an image block (image/png, base64) then a text block, bounded by its whole written form. · r1
contract/mcp-role/self-description · the mcp role · self-description and remedy — A result carries the tool's self-description and a session error's remedy, from the program's own fixed static table text. · r1
contract/mcp-role/tool-call-crossings · the mcp role · tool call crossings — A tool call brings an element id and typed text in; a result takes ids, accessible names, masked password and file-input values, snapshot text and diff out. · r1
contract/mcp-role/tool-listing · the mcp role · tool listing — The tool listing names each verb's result fields. · r1
contract/recording-directory/check-file · The recording directory · check-{n}.json — One check-{n}.json per recorded session, mode 0600, created new with n the smallest from 1 naming no file, so no file is replaced; rewritten whole after every call through a temporary file and a rename. · r1
contract/recording-directory/platform · The recording directory · platform — Recording is unix only; elsewhere it is refused Unsupported. · r1
contract/session-socket-wire/answer-bound · the session socket's wire · answer bound — An answer is at most 1,048,576 bytes. · r1
contract/session-socket-wire/call · the session socket's wire · call — The call request runs a driver call and returns its answer. · r1
contract/session-socket-wire/hello · the session socket's wire · hello — The hello request is served. · r1
contract/session-socket-wire/non-unix-targets · the session socket's wire · non-unix targets — On targets other than unix the lifecycle and a call return SessionError::Unsupported. · r1
contract/session-socket-wire/recording-failed · the session socket's wire · recording-failed — A call's replies include the fixed recording-failed reply. · r1
contract/session-socket-wire/request-line-bound · the session socket's wire · request line bound — A request line is at most 16,384 bytes. · r1
contract/session-socket-wire/screenshot-answer · the session socket's wire · screenshot answer — The answer to a screenshot call holds its PNG as base64 inside the JSON line. · r1
contract/session-socket-wire/stop · the session socket's wire · stop — The stop request is served. · r1
contract/session-socket-wire/version · the session socket's wire · version — The wire version is v3. · r1
contract/telemetry/no-thread · escher-telemetry bootstrap · threads — escher-telemetry's global subscriber, log bridge and chaining panic hook spawn no thread. · r1
contract/telemetry/rust-log · escher-telemetry bootstrap · RUST_LOG — RUST_LOG is the filter escher_telemetry::init reads, warn when unset or unparsable. · r1
absence/served-network-port — A served network port: no TCP or UDP port any binary serves · r1
name/binary/blitz — blitz · r1
name/binary/escher-session — escher-session · r1
name/binary/seven-guis-native — seven_guis_native · r1
name/binary/todomvc-native — todomvc_native · r1
name/crate/accesskit-xplat — accesskit_xplat · r1
name/crate/blitz — blitz · r1
name/crate/blitz-dom — blitz-dom · r1
name/crate/blitz-traits — blitz-traits · r1
name/crate/blitz-vibey-script — blitz-vibey-script · r1
name/crate/browser — browser · r1
name/crate/browser-persistence — browser-persistence · r1
name/crate/counter — counter · r1
name/crate/debug-timer — debug_timer · r1
name/crate/escher-driver — escher-driver · r1
name/crate/escher-telemetry — escher-telemetry · r1
name/crate/rdme — rdme · r1
name/crate/seven-guis — seven_guis · r1
name/crate/todomvc — todomvc · r1
name/crate/transparent — transparent · r1
name/crate/wasm-hello — wasm_hello · r1
name/crate/wgpu-texture — wgpu_texture · r1
name/other/blitz-target — blitz-target · r1
name/other/blitz-user-agent — Mozilla/5.0 (compatible; Blitz) · r1
name/other/cargo-target-tmpdir — CARGO_TARGET_TMPDIR · r1
name/other/cold-agent-ps1 — cold-agent.ps1 · r1
name/other/cold-agent-sh — cold-agent.sh · r1
name/other/com-dioxuslabs-blitz — com.dioxuslabs.blitz · r1
name/other/dioxus-scheme — dioxus://index.html · r1
name/other/mcp-escher — mcp__escher · r1
name/other/multi-doc-flat — MULTI_DOC_FLAT · r1
name/other/multi-doc-n — MULTI_DOC_N · r1
name/other/multi-doc-seq — MULTI_DOC_SEQ · r1
name/other/paint-tree-bench-html — PAINT_TREE_BENCH_HTML · r1
name/other/rust-log — RUST_LOG · r1
name/other/tmpdir — TMPDIR · r1
name/other/wpt-dir — WPT_DIR · r1
name/server/escher — escher · r1

## infrastructure-patterns · ## Infrastructure Patterns
decision/browser-load-aborts-previous · A browser document load aborts the previous load — Each browser document load aborts the previous one, as reload and drop do. · why: The browser's tab navigation calls load_document and the toolbar reload calls reload, so a superseded page's fetches stop; DocumentLoader's Drop aborts too · r1
decision/bundle-build-jobs · Two build jobs for browser bundles — Browser bundle builds run with two cargo build jobs. · why: to debug possible OOM · r1
decision/capabilities-doc-beside-upstream · Capability statement beside upstream docs — escher's capability statement sits in a file of its own under docs, beside upstream's three files. · why: an upstream change under docs never conflicts with it · r1
decision/cargo-deny-advisory-gate · cargo-deny advisory gate — A cargo-deny advisory gate, configured by the root deny.toml, holds the dependencies. · r1 · add-to · F2
decision/ci-cache-save-refs · CI cache save refs — ci.yml caches save only on main and build/* refs; publish-browser's and wpt's caches save only on main. · r1
decision/ci-concurrency · Per-ref CI concurrency — CI runs with per-ref concurrency and cancels a run in progress. · r1
decision/ci-failure-logs · CI logs uploaded on failure — Every leg job uploads its own log on failure, kept 7 days, and the coverage job also uploads its report on success, kept 7 days. · r1
decision/ci-fast-slow-split · CI fast/slow split — fmt, clippy, test-features-default and ci-scripts carry no needs, and every other CI job, the matrix included, needs all four. · r1
decision/ci-read-only-permissions · Read-only CI permissions — CI runs under a workflow-level contents: read permission with no job-level grant. · r1
decision/fmt-clippy-gates · fmt and clippy gates — CI gates every change on rustfmt and clippy. · r1
decision/js-entry-redraws-and-arms-timer · JS entry points request a redraw and arm the timer thread — Each JS entry point requests a redraw and re-arms the timer thread that wakes the event loop at the next JS timer deadline. · why: The windowed shell repaints after script work and wakes for due JS timers only through this; the WPT runner opts out with without_timer_thread · r1
decision/linux-tested-by-fast-leg · Linux tested by the fast leg — Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android. · r1
decision/locked-cargo-legs · Locked cargo legs — Every cargo leg of CI runs with --locked. · except: fmt, and examples/wasm_hello. · r1
decision/net-launch-enters-tokio-runtime · Net launch enters a tokio multi-thread runtime — Launch paths built with `net` off wasm32 enter a tokio multi-thread runtime before the event loop runs. · why: blitz-net's Provider spawns every fetch with tokio::spawn, so dioxus-native and blitz launches with net need the entered runtime for loads to run · r1
decision/nextest-mutants-profile · nextest mutants profile — cargo-nextest carries one profile, mutants, for the mutation tier; the default profile stays untouched. · r1
decision/nix-flake · Nix flake — A Nix flake exposes the browser package as default and a blitz-dev devShell, wrapping Linux packages so the dlopen'd graphics and windowing libraries are found. · r1
decision/objc2-encoding-assertions-off · objc2 encoding assertions disabled — blitz-dom depends on objc2 with disable-encoding-assertions on Apple targets. · why: a hack to stop debug builds panicking · r1
decision/pinned-actions · SHA-pinned actions — Every uses: of every workflow is pinned to a 40-hex commit SHA, its ref kept as a trailing comment, and every rust-toolchain step names its toolchain. · guard: test_ci_workflows.py · r2
decision/rustdoc-warnings-gate · Workspace rustdoc warnings gate — Rustdoc warnings are errors, and the docs job documents every workspace library crate under that setting, with no output-filename collision. · except: the browser binary blitz is not documented, so the lib blitz alone writes its docs · r1
decision/shell-polls-after-window-event · Shell polls its window after each window event — Every shell window event is followed by a Poll. · why: DioxusNativeApplication forwards window events to the shell and runs the vdom work handlers scheduled in the Poll that follows (CloseRequested excepted) · r1
decision/sites-outside-workspace · sites outside the workspace — The sites directory is excluded from the Cargo workspace. · r1
decision/stand-uses-bundled-font · Stand shares the bundled font — seven_guis' bundled DejaVuSans bytes compile on native too, where the headless stand registers the same font. · r1
decision/unrewritten-profiles · Build profiles unrewritten — CI jobs build the profiles unrewritten, with no opt-level rewrite step. · r1
decision/workspace-shared-dependencies · Shared workspace dependencies — The Cargo workspace declares its dependencies once in a shared workspace dependency table. · r1
decision/wpt-test-reset-and-panic-catch · WPT runner resets the net provider and catches test panics — The WPT runner resets the net provider before each test and runs each test inside a panic catch, a panic yielding Unknown or Crash. · why: The WPT run's per-test isolation and its CRASH count in the summary and wpt report depend on the reset and the caught panic mapped to Unknown kind and Crash status · r1

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/a11y-rebuild-on-nonempty-changed-set · Accessibility tree rebuilt on a non-empty changed set — Under accessibility, the shell rebuilds the platform accessibility tree on a poll that reported work only when the changed set it took is non-empty, and takes the set on every such poll with or without the feature. · why: ratified as the shell half of the changed-set item · r1
decision/accessibility-feature-gating · Accessibility feature gating in dioxus-native-dom — The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry`, `reached`, `seed` and `boundary` modules stay ungated. · why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off. · r3
decision/agent-driven-development · Agent-driven development style — Development is agent-driven: the framework's own driver is meant to become the harness. · r1
decision/animations-advance-by-resolve · Animations advance through resolve with a time — Animations, transitions and smooth scrolls advance only through a resolve given a time, and whether any is running is read by an is-animating query. · guard: pseudo_element_update, fragment_navigation · r1
decision/anonymous-blocks-in-layout-children-only · Anonymous blocks live only in layout children — Anonymous blocks exist only in the layout children, never in the DOM children. · guard: anonymous_block_cache_invalidation · r1
decision/changed-set-engine-touched · Changed set is engine-touched, a superset — The changed set records what the engine touched, a superset of what a snapshot reads as changed, so a resolve with nothing written marks nothing in either layout mode. · why: ratified · except: bounds-only movement changes a reading and marks nothing · r1
decision/changed-set-write-sites · Where the changed set is written — The changed set is written only by mutations whose condition holds, focus or checked state changes, typed text input, range moves and select choices that change a value, and a select list opening or closing; never by creation, hover, active, scroll, resize, load, animation or layout. · why: ratified as the engine half of the document-core changed-set item · except: a page scroll or a move that closes an open select list marks its select at the next resolve · r2
decision/color-scheme-change-recascades · Colour-scheme change recascades everything — A colour-scheme change recascades the whole document. · guard: device_coalescing · r1
decision/color-scheme-follows-window-theme · Colour scheme follows the window theme — The colour scheme follows the window theme, with an override, and defaults to Light. · r1
decision/damage-propagation · Damage propagation — Damage propagates up the DOM and to anonymous ancestors and is cleared in one visit. · r1
decision/default-action-event-order · Default-action event order — Default-action events are queued and processed in order, handlers running before the default action unless cancelled. · r1
decision/device-read-flushes-pending · Reading the stylist device flushes changes — Reading the stylist device first flushes pending device changes. · guard: device_coalescing · r1
decision/dioxus-root-contexts · Dioxus services as root contexts — Dioxus services (net provider, HTML parser provider, native document, window event handlers, shell provider, history, renderer, winit window) are provided as root contexts, and apps reach them as contexts. · r1
decision/dioxus-skips-unhandled-event-kinds · Dioxus skips unhandled event kinds — Dioxus skips event kinds with zero handlers. · r1
decision/dirty-descendant-invariants · Dirty and damaged descendant ancestor invariants — Every ancestor of a dirty or damaged node carries the dirty-descendants or damaged-descendants flag, and damage propagates up the DOM and to anonymous ancestors. · guard: stale_dirty_descendants, style_property_invalidation · r1
decision/document-config · Document configuration struct — Documents are configured through the `DocumentConfig` struct. · r1
decision/dom-mutation-reconstructs-box · Every DOM mutation reconstructs the box — Every DOM mutation inserts box-construction damage. · guard: anonymous_block_cache_invalidation · r1
decision/escher-driver-spans-ungated · escher-driver spans ungated, no subscriber — escher-driver takes `tracing` with no feature, builds its `command` span and its `tool_call` span (the parent of `command` on an MCP tool call) by hand with no instrument attribute and no feature gate, and installs no global subscriber. · r2
decision/escher-telemetry-sink · Escher binaries install the escher-telemetry sink — Escher binaries install escher-telemetry at start: one stderr-only sink that stamps the service identity on every line, bridges `log` records and chains the panic hook, with an EnvFilter layer for levels and no target-filter layer (the target drop sits in the formatter). · r1
decision/failed-stylesheet-stops-blocking · Failed stylesheets stop blocking render — A stylesheet that fails, comes back empty or is not CSS stops counting as a pending critical resource on every document, so it stops holding the redraw skip. · r1
decision/first-graph-state-hook-provides-registry · First graph-state hook provides the root signal registry — The first graph-state hook of a page provides a `SignalRegistry` context at the root scope, which the bridge's dispatch looks up; a page that registers no state has none. · why: The bridge's dispatch and the UI graph look the registry up at the root scope; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is · r1
decision/forwarded-event-coordinates · Forwarded events adjust coordinates — Sub-document and custom-widget events are forwarded with adjusted coordinates. · r1
decision/geometry-memo-generation · Geometry memoisation generation counter — Geometry memoisation is invalidated by a generation counter. · r1
decision/handlers-before-default-action · Handlers run before the default action — Default-action events are queued and processed in order, each running its handlers and then its default action unless cancelled. · r1
decision/ime-enabled-at-editor-construction · Focused text input enables IME when its editor is built — A text input that holds focus before its editor exists gets the shell provider's IME enabled when its editor is constructed. · why: Pinned by the bridge test input_focused_on_mount_enables_ime; TodoMVC-style onmounted focus needs IME on for macOS key bindings such as Backspace · r1
decision/interaction-state-cleared-on-removal · Interaction state cleared before node removal — Interaction state referencing a removed node is cleared or retargeted before the node is freed. · r1
decision/js-microtask-drain · JS microtask drain points — JS microtasks are drained after eval, timers and dispatch. · r1
decision/js-microtasks-drained · JS microtasks drained after each entry — JS microtasks are drained after eval, timers and event dispatch. · r1
decision/legacy-attributes-as-pres-hints · Legacy attributes cascade at the presentational-hints level — Legacy presentational attributes map to CSS at the presentational-hints level. · why: Stylo's cascade calls the hook, and replaced-element layout reads width and height as already in the style because the hints carry them · r1
decision/mounted-after-build-and-poll · Mounted listeners fire after build and poll — Dioxus mounted listeners fire after the initial build and after each poll. · r1
decision/node-send-sync-by-unsafe-impl · Node marked Send and Sync over its raw tree pointer — A `Node` holds a raw pointer to its `NodeTree` and is marked `Send` and `Sync` by unsafe impl. · why: Stylo's parallel traversal, which the windowed documents' Parallel style threading uses, needs nodes shared across threads · r1
decision/one-run-seed · One seed for a run's randomness — A run's randomness comes from one seed held as a Dioxus root context, ungated by the accessibility feature, so two runs of one seed read the same state at every step, in both layout modes and across processes. · guard: stand_seeded_randomness · r1 · contradict
decision/opt-in-time-logging · Opt-in frame and phase time logging — Frame and phase time logging are the opt-in features `log-frame-times` and `log-phase-times`. · r1
decision/optional-capabilities-cargo-features · Optional capabilities as Cargo features — Optional capabilities are compile-time Cargo features, each dioxus-native feature forwarded down the crate chain to the same-named feature of the underlying blitz crates. · r1
decision/platform-action-modifier · Platform action modifier — Keyboard shortcuts and the platform action modifier use Cmd (META) on macOS and Ctrl (CONTROL) elsewhere. · r1
decision/platform-gating · Platform behaviour gating — Platform behaviour is gated with `cfg(target_os = ...)` and an `IS_MOBILE` const. · r1
decision/platform-tree-rebuild · Platform tree rebuild — The shell rebuilds the platform tree only when a poll took a non-empty changed set. · r1
decision/reached-recorded-before-handlers · Reached record written before handlers run — Where a reached record is switched on, each node of the dispatch chain whose current listeners hold the event's name is recorded by its stable id before the handlers run. · why: so a handler whose own run removes its element still counts; every other document pays one context lookup per dispatch · r1
decision/redraw-requests · When redraws are requested — Redraws are requested after mutations, resource loads, hover, device changes, a focus set or cleared and a select list opening, moving, scrolling or closing; the Tab key asks for none of its own. · r1
decision/redraw-skip · Redraw skip — Redraw is skipped while critical resources are pending or the window is occluded. · r1
decision/removed-node-interaction-state · Interaction state on removed nodes — Interaction state on a removed node is cleared or retargeted before the node is freed. · r1
decision/resolve-before-js-geometry · Resolve before JS geometry reads — Style and layout are resolved before JS geometry reads. · r1
decision/resolve-before-js-reads · Style and layout resolved before JS reads — Style and layout are resolved before JS geometry, hit-test, selection and computed-style reads. · r1
decision/runtime-inputs · Runtime inputs — Runtime inputs are CLI arguments and environment variables. · r1
decision/select-list-changed-set · Select list marks in the changed set — Where a select list stands, a page scroll or a move that closes it marks its select at the next resolve and a wheel elsewhere marks it on close, while a highlight or a wheel move inside the list marks nothing. · r1
decision/shared-state-rc-refcell · Shared state as Rc RefCell — Shared state is `Rc<RefCell<…>>`: the blitz document, and JS state in a `DomCtx` on the Boa context. · r1
decision/shell-and-navigation-providers · Shell and navigation providers — A shell provider carries redraw, clipboard and file dialogs, and a navigation provider receives link-click navigations. · r1
decision/shortcut-modifier · Shortcut modifier key — Shortcuts use Cmd on macOS and Ctrl elsewhere. · r1
decision/signal-watcher-no-resubscribe · Signal registry watcher does not re-subscribe — The signal registry's watcher does not re-subscribe its reactive context after a notification. · why: dioxus-signals keeps a reactive context subscribed while its mark-dirty returns true · r1
decision/signal-write-recorded-against-dispatch · Signal writes recorded against their causing dispatch — Where a signal registry stands, each signal write is recorded against the dispatch that caused it, and a page with no registered state dispatches unchanged. · why: The UI graph reads each write record's dispatch to confirm declared writes; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is · r1
decision/sub-document-event-forwarding · Events forwarded to sub-documents and widgets — Events for sub-documents and custom widgets are forwarded to them with coordinates adjusted to their own space. · r1
decision/targeted-restyle-on-mutation · Targeted restyle on mutation — Attribute changes, node removal and insertion restyle through targeted attribute-change and child-list-change restyles, never a blanket subtree restyle. · r1
decision/telemetry-closed-span-lines · Telemetry prints closed spans — The telemetry sink prints a closed span's line beside an event's. · why: Ratified. · r1
decision/telemetry-scrubbing · Telemetry scrubs what prints — The telemetry sink scrubs every record that prints, judging a closed span's fields by the rule that judges an event's. · r1
decision/telemetry-target-prefix-drop · Telemetry drops targets outside the prefix sets — The telemetry sink drops every record whose target is outside the engine and escher prefix sets, at every level and whatever `RUST_LOG` names; the drop sits in the formatter. · r1
decision/typed-results · Typed results for fallible APIs — Fallible APIs return typed results. · r1
decision/upstream-fmt-subscriber · Upstream subscriber installs — The upstream fmt subscriber installs run only under the `tracing` feature and write to stdout unscrubbed. · r1
decision/wasm-font-setup · WASM font setup — On `wasm32` system-font discovery is disabled and a single-font context is the standard WASM setup. · r1
decision/wpt-failures-bucketed-by-feature · WPT failures bucketed by regex feature flags — WPT failures are bucketed by feature-usage flags computed by regex over test sources. · why: The WPT runner's end-of-run summary splits failures into per-feature counts (grid lanes, subgrid, writing-mode, direction, intrinsic size, calc, float, script) · r1
decision/wpt-runner-logging · WPT runner logging — The WPT runner logs through the `log` facade with `env_logger`. · r1
contract/telemetry-line/service-name · escher telemetry log line · service.name — Stamped on every line. · r1
contract/telemetry-line/service-version · escher telemetry log line · service.version — Stamped on every line. · r1

## project-intent · ## Project Intent
decision/capability-statement · Capability statement — docs/capabilities.md is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, and no check reads its other claims. · guard: .github/scripts/test_capability_statement.py · r1 · stale
decision/deferred-stylo-dioxus-bumps · Deferred Stylo and Dioxus bumps — Stylo stays "0.22.0" and Dioxus "0.7.3": git Stylo 0.23-pre and Dioxus 0.8.0-alpha.3 wait for escher's next version. · r1
decision/document-trait-drives-wrappers · Document trait drives every wrapper — `BaseDocument` is the core DOM's primary entry point, and the `Document` trait lets blitz-shell drive every wrapper around it. · r1
decision/escher-capitalised-in-prose · Escher capitalised in prose — The front page spells the name Escher with a capital in its prose and title, while crate, binary and path names, URLs and the e-mail stay unchanged. · r1 · stale
decision/escher-name-spelling · Spelling of the Escher name — The name is spelled «Escher» in prose and titles; crate, binary and path names, URLs and the e-mail stay unchanged. · r1 · stale
decision/lockstep-versioning · Blitz packages versioned together — The published blitz library crates are versioned together through the workspace version and the in-repo `=` pins. · r1
decision/readme-is-escher-own · Front page is escher's own — The root README.md is escher's own front page, not upstream Blitz's; at an upstream sync an upstream change to README.md resolves to escher's version. · r1
decision/readme-is-escher-page · Root README is escher's own page — The root `README.md` is escher's own page, not upstream Blitz's, and at an upstream sync an upstream change to it resolves to escher's version. · r1
decision/separately-usable-crates · Workspace of separately usable crates — The project grows as a Cargo workspace of separately usable member crates. · r1
decision/upstream-sync-merge-base · Upstream sync merge base and cherry-picks — The last merged commit of DioxusLabs/blitz `main` is the next sync's merge base; upstream commits past it may be taken by no-commit cherry-picks, leaving the merge base unmoved. · r1
decision/upstream-taken-by-cherry-pick · Upstream changes taken by cherry-pick — Upstream commits are taken by no-commit cherry-picks of upstream's text, each with its cherry-picked-from trailer, and every commit left out carries its measured reason. · r1
decision/validation-sources · Validation sources — Validation comes from web-platform-tests run by the WPT runner from a local checkout, blitz-tests' integration tests (some Rust ports of script-driven WPT tests), and regressions and fixtures traced to real sites and applications. · r1
contract/capability-statement/sections · The capability statement · sections — Five sections in order: What it can do, Stated limits, Accessibility reach, What an MCP client carries onward, Where each claim is proven. · r1
contract/root-readme/mcp-tool-count · root README.md · count of driver commands an MCP client gets as tools — The README states the count, which the capability-statement pin test reads; no build or doc step reads the README. · r1 · add-to · F3
overview/project-intent — escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness. · r1
