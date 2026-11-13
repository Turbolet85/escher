# architecture · the rows that bind 2026-11-13-cognitive-load-corrective — picked 447 of 1477 (266 by every draw) · rule any · pin 627

## design-philosophy · ## Design Philosophy
decision/interaction-state-on-dom-nodes · Interaction state references DOM nodes only — Persistent interaction state references DOM nodes only, never layout-generated nodes. · why: so it survives box-tree reconstruction · guard: interaction_state_canonicalization
decision/layout-modes-identical · Both layout modes give identical layouts — Both layout modes produce identical layouts.
decision/one-incremental-pipeline · One pipeline for incremental and full layout — Incremental and non-incremental layout run the same style, damage, box construction and layout pipeline, non-incremental marking every node damaged each resolve, and both must produce identical layouts after any sequence of mutations. · guard: incremental_oracle
decision/state-changes-snapshot-driven · State changes invalidate through snapshots — Hover, focus, active and disabled state changes set no restyle hint; their invalidation is driven by element snapshots.
decision/headless-test-harness · Headless test harness — The engine is tested without a window, GPU or compositor, and synthesized input routes through the real event-dispatch pipeline.
decision/injected-embedder-services · Embedder services injected, with no-op defaults — Networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies, and a provider not supplied falls back to a no-op default.
decision/one-scrolling-primitive · One scrolling primitive — User-initiated and programmatic scrolls share one scrolling primitive, differing only in the scroll request. · except: an open select list's rows: a wheel over the list and scroll-into-view on a row move only the list's own scroll offset, with no chain or viewport moving
decision/select-list-painted-in-scene · Open select list painted inside the scene — An open select's list is painted after the root inside the paint-scene call, from a public read, and reaches a shell or renderer through that call alone.
decision/single-incremental-pipeline · One incremental pipeline — Style, damage, box construction, layout and paint topology are invalidated incrementally through a single code path.
contract/dioxus-document/handle-ui-event · The Dioxus document · handle_ui_event — UI events are pushed into the document through it.
contract/dioxus-document/poll · The Dioxus document · poll — Changes are flushed to the DOM through it.

## stack-and-technologies · ## Stack and Technologies
decision/cognitive-complexity-ceiling · Cognitive complexity ceiling — A cognitive-complexity reading on the dev host alone, never in CI, reads each function against a ceiling of 15 with rust-code-analysis-cli.
decision/local-pre-push-gate · Local pre-push gate — The fast leg run — fmt, clippy, test, ci-scripts, embolden, stopping at the first red — is the local pre-push gate.
decision/blitz-dom-feature-subset-clippy · Upstream blitz-dom feature-subset clippy line — When the workspace clippy line is green, the clippy leg also runs upstream's clippy over blitz-dom with no default features and its named feature subset, warnings denied.
decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg gates line coverage by per-crate floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %. · guard: coverage_floor.py
decision/png-shot-encoder · png as the shot encoder — The workspace's png crate is escher-driver's encoder of a shot and the decoder tests read a shot with. · why: no package enters the lockfile
decision/stand-repeat-zero-failure-budget · Stand-repeat zero-failure budget — The stand-repeat leg holds the stand checks to a zero-failure budget over three rounds: the first red command ends the leg with its status, with no retry, no `--ignored` and no environment variable, and three green rounds are needed. · guard: test_stand_repeat_leg_runs_three_rounds

## established-decisions · ## Established Decisions
decision/a11y-input-focus-engine-changes · Accessibility, input and focus engine changes — The accessibility builder's state writes and the range input, keyboard activation and focus rules are direct engine changes for every document, each flagged upstreamable.
decision/actions-answer-crossing · Actions answer crosses the carriers — The actions answer crosses the session socket's call, the command line and MCP and rests in a kept check as the actions verb's answer, holding stable ids, owner ids, the table's verb words, the closed cause words and booleans, never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
decision/actions-keyed-reading · Actions answer's keyed reading — The actions answer carries the keyed reading.
decision/checkbox-change-from-engine · Checkbox and radio change from the engine — A checkbox's or radio's change event comes from the engine and the script runtime synthesises none. · why: so a page's script hears one change
decision/click-after-scroll-advances-past-fade · Click after scroll waits out the scrollbar fade — A driver click inside a just-scrolled box advances the virtual clock past the overlay scrollbar's fade first, since a main-button pointerdown in the shown scrollbar band is taken as a thumb drag and dispatches nothing. · why: a measured hazard of the overlay scrollbar, no engine change · except: A row of an open select list is never taken as a thumb drag.
decision/click-focuses-button · A click focuses a button — A click on a button of any type or a submit, reset or button input focuses it, then submits the form owner for a submit button; a disabled form control stops the walk first, and a click on any other element still clears focus. · why: ratified · except: a select focuses through its own arm, and a multiple select's row focuses its select
decision/closed-select-options-covered · Closed select options refused as covered — An option of a closed single select, under a group too, takes no click and stays refused `covered`.
decision/controlled-select-writes-selected · Controlled select writes selected from its input event — A controlled app writes `selected` from its `input` event's value, or a choice reverts. · why: Dioxus writes a dynamic `selected` again on each re-render
decision/dioxus-event-path-records-opt-in · Dispatch records on the Dioxus event path are opt-in — The bridge writes the signal registry dispatch and the reached record only where their context is present at the root scope, so a document that registers neither dispatches unchanged, paying one context lookup a dispatch.
decision/direct-engine-changes-flagged-upstreamable · Direct engine changes flagged upstreamable — Every direct engine change escher makes for every document is flagged upstreamable and nothing is sent upstream.
decision/disabled-one-reading · One reading of disabled — A form control's `disabled` is read by its presence through one reading, which focusability, the pointer and click gates and the Enter and Space activation all use.
decision/disabled-read-on-form-control-only · Disabled read on a form control alone — The disabled attribute is read by its presence alone, bare, "false" or "true", and only on a form control (button, input, select, textarea); focusability, the click walk, activation and the snapshot's clickable all read it so. · why: ratified
decision/driver-library-side-effect-free · Driver library without side channels — The driver library depends on no app, emits no event, installs no subscriber and reads no env var.
decision/driver-spans-only · Driver opens spans and emits no event — The driver opens one tracing span per call it runs and one tool_call span per MCP tools/call request, and emits no event, installs no subscriber, reads no env var and has no default state location.
decision/focus-fixup-without-events · Focus fixup without events — At flush, focus on a node that has become inert is cleared with no blur or focusout fired, a subtree moved under an inert box included. · why: a ratified direct engine change for every document, flagged upstreamable · guard: inert_subtree::a_node_made_inert_loses_focus_and_takes_no_key
decision/focusability-disabled-presence · Focusability reads disabled by presence — Focusability reads the disabled attribute by presence, and on a form control alone. · why: ratified
decision/fragment-link-scrolls-in-page · Fragment-only links scroll in page — A link that changes only the fragment scrolls within the page; an empty or unmatched top fragment scrolls to the top.
decision/graph-state-by-registration · UI graph state reached by registration — App state is reachable for the UI graph only when registered through use_graph_state; hook-local use_signal state an app does not register is not reached. · why: dioxus-core keeps a scope's hooks crate-private, so hook-local state needs a fork-level change
decision/hit-focus-form-engine-changes · Hit, focus and form engine changes — Visible region through transforms, no hit on a non-invertible transform, the focus fixup under inert, tree focus falling back to the window, native toggled, the multiple select, optgroup and change on a checkbox and a radio are direct engine changes for every document, flagged upstreamable.
decision/incremental-layout-default · Incremental layout on by default — Incremental layout defaults to on.
decision/inert-focus-fixup · Focus fixup under inert — A node that holds focus when it comes under `inert`, a subtree moved under an inert box included, gives focus up at the mutator's flush with no DOM `blur` or `focusout`, reading neither `disabled` nor `hidden`.
decision/inert-honoured · inert honoured by presence — An element carrying `inert`, or under an inert ancestor, is not focusable, skipped by the Tab walk and autofocus, takes no hit so a hit passes through it, and leaves the accessibility tree and the snapshot. · why: ratified
decision/key-types-through-harness · Key types through the harness's re-exports — The key types the driver's press needs come through the test harness's re-exports Key, Modifiers and NamedKey, not as a dependency of the driver's own.
decision/keyboard-activation-enter · Enter activates the focused control — A pressed Enter on the focused node, after the text and range input arms, dispatches a click on a non-disabled button, summary, submit, reset or button input, or a link with href, the app's handlers running before the default action. · why: ratified
decision/keyboard-activation-space · Space activates the focused control — A released Space on the focused node dispatches a click on a non-disabled button, summary, or submit, reset, button, checkbox or radio input; a link takes no Space, and text and range inputs take neither key. · why: ratified
decision/label-click-focuses-select · A label click focuses its select — A bound label's click focuses the select and opens nothing.
decision/link-load-start · When a link starts loading — A link starts loading once it has both rel=stylesheet and an href, and an href the document URL refuses to resolve starts no load. · guard: link_rel_attribute, render_blocking_stylesheet
decision/lint-findings-on-carriers · Lint findings cross the carriers — A lint's findings cross the ratified carriers and rest in a kept check as the lint verb's answer, holding stable ids, the closed defect words and numbers, never page text, colour, control value or NodeId, with no new carrier, exit, path argument, cause, session error, span field or dependency. · why: A boundary widening, ratified.
decision/multiple-and-optgroup-modelled · Multiple select and optgroup modelled throughout — A `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge.
decision/multiple-select-chosen-row-colours · Multiple select chosen-row colours — A multiple select's chosen row is painted with a #c8c8c8 background and #000000 text from the default stylesheet. · why: Chromium's inactive selected pair, hardcoded since the engine holds no tokens · guard: select_interaction
decision/multiple-select-row-clicks · Multiple select row clicks choose, flip or span — A plain click on a multiple select's row chooses it alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing. · why: Pinned by a_click_on_a_row_chooses_flips_or_spans_it and the driver test a_press_with_shift_spans_the_rows
decision/new-carrier-widens-boundary · New carriers widen the session boundary — A new carrier or exit for a driver session is a widening of its boundary.
decision/no-node-id-across-session · No NodeId crosses a session — Only ids, names, masked values, snapshot text, diffs, shots, lint findings, closed words, booleans and counts cross a driver session, never a `NodeId`.
decision/option-disabled-own-or-group · Option disabled by itself or its group — An option is disabled by its own attribute or its group's.
decision/owning-component-via-public-api · Owning component read through Dioxus public API — A node's owning component is read through Dioxus 0.7.10's public API, a deleted row resolving to none. · why: The stable element ids and the UI graph's owner field read it, and the driver's graph verb surfaces it to agents
decision/select-choice-as-checked · Select choice held as options' CHECKED state — A select's choice, or a `multiple` select's chosen set, is held as its options' `CHECKED` state, a single select's settled by HTML's selectedness algorithm.
decision/select-choice-as-checked-state · Select choice held as checked state — A select's choice is held as its options' checked element state, so :checked matches the chosen option, and a single-choice select is settled by HTML's selectedness setting algorithm with no event fired. · why: the rules are cited to the WHATWG select element
decision/select-choice-one-path · Select choices go through one path — Every choice that changes a select's chosen option or a `multiple` select's chosen set goes through one path.
decision/select-choice-single-path · One path for every select choice — Every choice that changes a select's chosen option or chosen set goes through one path that writes the checked state and fires input then change carrying the select's value; a choice of the already chosen option fires nothing. · why: a direct engine change for every document, flagged upstreamable
decision/select-list-closes-without-event · Open select list closes with no event — An open select list closes with no event on Escape, a second click, a press or wheel outside, a scroll or move of the select's box, and on removal, disabled, hidden or inert. · why: Pinned by escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event and a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes
decision/select-list-painted-overlay · Select list painted over the page — A single select opens a list the engine paints over the page, by a click, by Enter, or by a Space with no typed text standing. · why: a ratified direct engine change for every document, remote pages included
decision/single-select-choice-fires-nothing · A single select's choice of its chosen option is silent — A single select's choice of its chosen option fires nothing.
decision/single-select-list-opening · Opening a single select's list — A click, Enter and a Space with no typed text open a single select's list, which the engine paints over the page with a group's `label` as a header.
decision/ui-graph-crossing · UI graph crosses the carriers — The UI graph crosses socket call, command line and MCP as a text block and rests in a kept check as the graph verb's answer: stable ids, owner ids, component, state and handler event names, declared writes, the flag words undeclared and unconfirmed, booleans; never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
decision/unresolvable-url-skipped · Unresolvable URLs skipped, never a panic — An unresolvable href or relative URL is skipped, never a panic: the document's URL resolution yields nothing and every caller skips it. · why: an upstream change taken, ratified for its take
contract/actions-verb/answer · the actions verb · answer — The actions verb answers the whole screen at once, unpaged.
contract/actions-verb/arguments · the actions verb · arguments — The actions verb takes no argument; a call given one is refused malformed with nothing run.
contract/coverage-role/answer · the command line's coverage role · answer — One line on stdout of ids, owner ids, event names and two counts.
contract/document-defaults/incremental-layout · Document defaults · incremental layout — Incremental layout is on.
contract/graph-verb/arguments · the graph verb · arguments — The graph verb takes no argument; a call given one is refused malformed with nothing run.
contract/snapshot-verb/part · the snapshot verb · part — An optional whole number from 1 to MAX_PART (1000); one part's text comes out with the closed counts part and parts, and a part past the last gives the empty text.
decision/differs-closed-set · A changed node's differs is a closed set — A changed node's differs crosses the carriers as a closed set.
decision/document-clock · The document's clock — Every engine time read goes through the document's configured clock, wall time when it is unset and a virtual clock headlessly, a sub-document carrying its parent's clock, with every timing constant unchanged. · why: ratified; an API addition whose default keeps every windowed and embedder-built document's behaviour
decision/hit-stops-at-paint-clip · Hit walk stops where paint clips — A hit stops at the padding box of every box paint clips, by overflow on either axis or by a computed contain including paint on a box paint applies it to; outside that box none of its children or inline content is reached, and an open select list answers first above the walk. · why: Uses paint's own clipping predicate so hit and paint agree; ratified. · guard: hit_clipped_at_scrolling_box · except: the root element
decision/input-control-data-at-construction · Input control data at construction — Text, checkbox, radio and range inputs get their control data at construction.
decision/kept-check-masking · Kept check keeps masking — A kept check records a masked or unresolved type as one fixed stand-in, and keeps each answer as a 64-bit digest and whole, masked as on every carrier, a shot's without its pixels.
decision/kept-check-never-logged · Kept check never logged — A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph and actions answers rest at rest only, never logged and never named by a tool call.
decision/kept-check-replays-through-run · Kept check replays through the one run path — A kept check replays through the session's one call-running path alone, and a coverage role reading one from a named path bounds the read at the check size limit plus one byte.
decision/large-screen-in-parts · Screen over the snapshot budget crosses in parts — A screen over the snapshot budget crosses in parts, a screen within the budget answering whole, with no new verb, carrier, exit, cause, session error, span field or dependency; over MCP a paged answer is its JSON line. · why: A boundary widening, ratified.
decision/multiple-select-row-keys · Multiple select keys choose alone, span or move — A multiple select's arrow, Home and End keys choose one row alone, with Shift spanning from the anchor and Ctrl or Meta moving the current row without changing the choice. · why: Pinned by the_keys_of_a_multiple_select_choose_span_and_flip_its_rows and the unit test a_gesture_on_a_row_leaves_the_set_it_states
decision/open-pseudo-class-details-dialog · :open matches details or dialog carrying open — :open matches a details or dialog that carries the open attribute. · why: Pinned by the open_state tests at_construction_open_is_read_from_the_attribute and a_write_and_a_removal_through_the_mutator_move_it; select_popup checks a select never matches
decision/open-select-list-hit-first · An open select list answers hits first — Where a select list stands open, a hit answers the list first, with no scrollbar, so a list row lies past any box that clips its select.
decision/programmatic-scroll-one-box · Programmatic scrolls clamp to one box — `scroll_to` and `scroll_by` clamp to one box.
decision/range-pointer · Range pointerdown moves, click focuses — A main-button pointerdown on a range input sets its value from the hit's fraction of the content box, clamped to [0, 1]; a click focuses it and changes no value. · why: ratified
decision/removed-node-state-cleared · Interaction state of removed nodes cleared — Interaction state on removed nodes is cleared or retargeted.
decision/screenshot-in-band · Screenshot pixels cross in band — A shot's pixels cross in band, its PNG as base64 in the answer on the session socket and the command line's stdout and as an MCP image block on the mcp stdio, with no new carrier, exit or path argument; the 1,048,576-byte bounds stay, a bigger image ending oversize / AnswerTooLarge. · why: A boundary widening, ratified.
decision/scroll-into-view-nested-instant · Scroll into view writes nested boxes instantly — Scrolling into view writes every scrolling box on the chain, nested ones instantly, so a nested box is never animated.
decision/scroll-into-view-whole-chain · Scroll into view writes the whole containing-block chain — Scrolling an element into view writes every scrolling box on its target's containing-block chain, innermost first, then the viewport, through the one existing method for every caller; each nested write is an instant clamped scroll and the requested behaviour applies to the viewport alone. · why: Follows the CSSOM View scroll-an-element-into-view steps; ratified as a behaviour change for every document, the method widened and no second method beside it. · except: a row of an open select list, which moves only the list's own scroll offset
decision/select-list-crossing · Select list crosses only as bounds and pixels — The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff, differs naming bounds, and as pixels in a screenshot, with no new carrier, exit, cause, session error, node key, log field or span. · why: An option's and a group's label leaving as pixels in every shot of an open list is a ratified boundary widening.
decision/select-options-include-optgroup · Select options include optgroup children — A select's options include its `optgroup` children's options.
decision/select-row-scroll-into-view · Scrolling a select row into view moves only the list — Scrolling a select list row into view writes the list's scroll offset at once and moves no scroll chain or viewport.
decision/socket-carries-driver-calls · Session socket carries driver calls — The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses. · why: A boundary widening, ratified.
contract/snapshot-node-state/clickable · the snapshot's node state · clickable — True where the element holds a current click listener and is not a form control reported disabled, so a disabled button with a listener reads not clickable and a div with disabled and a listener reads clickable.
contract/snapshot-node/clickable · the snapshot node · clickable — Each snapshot node carries the closed bool clickable.
contract/tool-call-span/tool · tool_call span · tool — The table's word for the tool, absent for an unknown tool; one of the two fields the tool_call span under the target escher_driver holds, beside ending and no other.

## conventions · ## Conventions
decision/borrow-take-and-restore · Take and restore for borrow conflicts — Borrow conflicts are worked around by taking a child list or layout out and restoring it.
decision/clippy-allow-scoping · Clippy allows kept local — Clippy allows are local to the code they cover. · except: `clippy::collapsible_if` is allowed crate-wide in five crates (browser, blitz-dom, blitz-html, blitz-paint, blitz-vibey-script) and `clippy::module_inception` in blitz-dom's node module.
decision/dispatch-event-callback · Event producers take a dispatch callback — Event-producing functions take a `dispatch_event` callback.
decision/do-not-reorder-marker · Ordering-sensitive block marker — An ordering-sensitive block is marked "WARNING: DO NOT REORDER".
decision/driver-hand-built-spans · Driver spans built by hand — escher-driver builds its spans by hand, with no `#[instrument]`.
decision/fmt-and-clippy-gate · Formatting and clippy gate — `cargo fmt --all --check` and workspace clippy with `-D warnings` run as legs in CI and on the dev host, upstream's blitz-dom clippy line only when those are green.
decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification. · except: The browser app's test module allows `clippy::expect_used` with no justification beside it.
decision/local-lint-allows · Lints allowed locally — Clippy lints are allowed locally, some with a reason, not crate-wide. · except: `clippy::collapsible_if` crate-wide in browser, blitz-dom, blitz-html, blitz-paint and blitz-vibey-script; `clippy::module_inception` in the node module.
decision/mutations-through-mutator · DOM mutations through DocumentMutator — DOM mutations go through `DocumentMutator`, which flushes deferred work on drop and requests a redraw only for in-document mutations.
decision/mutator-doc-escape-hatch · Mutator's document escape hatch — The mutator's public document access is an escape hatch; extending the mutator is preferred.
decision/snapshot-before-attribute-mutation · Stylo snapshot before attribute mutation — A Stylo snapshot is taken before attribute mutation of in-document nodes.
decision/unwrap-expect-justified · Justified unwrap and expect — `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification.
decision/both-layout-modes · Scenarios in both layout modes — Test scenarios run in both layout modes, incremental off and on. · except: upstream-taken blitz-tests targets are kept as taken even where they run one layout mode; escher's own targets run both
decision/known-gap-markers · Known gaps marked TODO or FIXME — Known gaps are marked with `TODO` or `FIXME`.
decision/non-exhaustive-option-types · Non-exhaustive navigation options and requests — `NavigationOptions` and `Request` are `#[non_exhaustive]`.
decision/non-exhaustive-request-types · Non-exhaustive navigation and request types — `NavigationOptions` and `Request` are `#[non_exhaustive]`.
decision/scenarios-both-layout-modes · Scenarios in both layout modes — Scenarios run in both layout modes.
decision/seven-guis-graph-signals · seven_guis signals registered with the graph — seven_guis' tasks register each signal through the tasks module's graph-signal hook, which wraps the plain signal and graph-state hooks; plain signals are left to the stand shell's `active` and the stand's `linked`.
decision/todo-fixme-gap-markers · Known gaps marked TODO/FIXME — Known gaps are marked with `TODO`/`FIXME`.

## standard-contracts · ## Standard Contracts
decision/advance-moves-session-clock · Advance moves the session's clock — An advance moves the session's clock by exactly the asked milliseconds on every session, and the app's own time through a caller's time step where one was handed; a session with no time step still takes advance, which moves its clock alone; advance reports the clock's move. · why: Ratified.
decision/advanced-ms-is-the-clock-move · advanced_ms reports the clock's move — An advance's advanced_ms is exactly the milliseconds asked, the clock's move, and its diff can be non-empty on any animated page. · why: Ratified.
decision/boa-script-engine · Boa runs document scripts — A document's scripts run on Boa, JS listeners before Blitz default actions.
decision/cells-grid-single-tab-stop · Cells grid is one Tab stop with arrow and Enter editing — The cells-grid is one Tab stop whose arrows move the current cell and whose Enter opens the editor, an editor Enter committing and returning focus to the grid. · why: Pinned by the stand_keyboard Cells walk and stand_cells_served, which read focus back on cells-grid after an editor Enter
decision/diff-content-never-logged · Diff content never logged — A diff's content never reaches a log or event; only its three counts of added, removed and changed may be recorded on the driver's command span. · why: Ratified.
decision/diff-not-engine-changed-set · Diff separate from the engine's changed set — The diff compares two snapshots and is not derived from the engine's changed set; a set changed flag beside an empty diff is a designed case. · why: The changed set is the cheap signal that something was written; the diff is what a snapshot reads differently.
decision/double-click-window · Two clicks without an advance read as a double click — Two clicks on one point with no advance of at least 500 ms between them read as a double click, as the click verb's help says. · why: Ratified.
decision/driver-logs-no-content · Driver logs no content — Nothing in escher-driver logs an id, a name or a value.
decision/font-embolden-local-witness · Local witness for font-embolden paint — The font-embolden feature is built and tested locally by the embolden step of the fast gate, with no CI job; macOS exercises the feature in CI. · why: Running the painted select-popup test under the feature on linux reproduces macOS's two readings. · guard: test_fast_leg_ends_on_the_font_embolden_witness
decision/fresh-list-survives-focus-restyle · A just-opened list survives the focus restyle — A select list opened by a click is held fresh against the restyle of a :focus rule, so a rule that moves the select's box cannot close the list the click opened. · why: A click focuses and opens in one step, before the focus rule's restyle. · guard: a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened
decision/input-helpers-do-not-settle · Input helpers do not settle — Pump, tick, advance-time, dispatch and every input helper settle nothing; settling is asked for separately.
decision/load-page-message-hidden-until-sheet · Load button links one sheet that reveals Loaded — The load page's Load button links one stylesheet, and the Loaded paragraph stays hidden until that sheet lands. · why: Pinned by the_load_page_click_waits_out_its_load_and_shows_the_message and the_load_page_enter_waits_out_its_load_and_shows_the_message
decision/nested-scroll-written-instantly · Nested scrolling boxes are written instantly — When scrolling into view, a nested scrolling box is written at once whatever behaviour was asked, and the requested behaviour applies only to the viewport, whose offset is computed after the nested writes. · why: The document holds one scroll animation at a time.
decision/no-echo-of-typed-input · No echo of typed input — No driver command's ending writes back anything the caller typed.
decision/no-snapshot-in-logs · No snapshot content in logs — No log or event carries the snapshot, its text or a diff. · except: a diff's three counts, added, removed and changed, on escher-driver's command span.
decision/one-axis-clip-readings-kept · One-axis clip readings kept as measured — A box clipped on one axis stops the hit at its whole padding box while the visible region narrows only on the clipped axis, so a driver click on a target outside along the visible axis is refused as covered; neither reader is changed. · why: Ratified to measure and record the difference rather than move either reader. · guard: stand_act_obstructed
decision/one-disabled-reading · One shared disabled reading — Focusability, the pointer gates and the activation keys all share one reading of disabled: a button, input, select or textarea carrying a disabled attribute.
decision/one-virtual-clock · One virtual clock per harness — The harness holds one virtual clock its built documents share; only tick and advance-time move it, a negative or non-finite step moving nothing, and screenshot, text contrast and settle never move it; a wrapped document keeps its own clock. · why: Ratified.
decision/reached-record-before-dispatch · Reached record written before dispatch — The reached record takes each node of the dispatch chain whose current listeners hold the event before the event runs, holding no borrow across the render. · why: So a handler whose own run removes its element still counts.
decision/refusal-causes-closed-at-eight · Refusal causes closed at eight — The set of refusal causes is closed at eight, each with a fixed name, meaning and remedy, and a refusal holds nothing a call supplied. · why: Ratified.
decision/refused-call-records-no-ids · A call validation refuses records no ids — A call the schema's validation refuses records nothing in the session's record of seen ids; every snapshot a validated call takes is recorded, on a refusal too. · why: Ratified.
decision/replay-hook-after-each-step · Replay calls a hook after each replayed step — Handler coverage relies on the replay calling a hook after each replayed step, so the screen can be read between steps. · why: The driver's coverage role passes a hook to replay_each that reads the screen's reached handlers between steps
decision/resolve-reconciles-select-list · Resolve reconciles the select list before hover — Resolve reconciles the select list after layout and before refreshing hover, and removing a select clears its list. · why: Pinned by a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes and a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened
decision/screenshot-reads-session-clock · Screenshots read the session's clock — The overlay scrollbar fade reads the session's clock, so two screenshots of one state with no advance between them read the same pixels. · why: Ratified.
decision/select-choice-single-write-path · Select choice has one write path — Every change of a select's chosen options goes through the one select-choice write path, which takes a set of options.
decision/select-list-rows-and-placement · Select list shows at most 20 rows, placed where they fit — An open select list shows at most 20 rows, opens where its rows fit or on the roomier side, and a select with no row to show opens nothing. · why: Pinned by a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel, a_disabled_inert_hidden_singular_or_empty_select_opens_nothing and the place unit test
decision/session-moves-clock-not-time-step · Session, not the time step, moves the clock — A stand's time step only delivers the whole timer ticks the asked milliseconds hold, carrying the remainder across calls; the driver session, not the step, moves the harness's clock. · why: Ratified.
decision/settle-never-waits-on-time-or-loads · Settle never waits on loads or time — Settle drives render and layout to quiet, reports a load in flight and never waits on one, and neither waits on nor advances a source only time moves; it reads no clock but the harness's own, logs nothing and starts no thread.
decision/transformed-element-bounds · Transformed element bounds — An element under a transform reads its bounds where its transforms move it, so a driver click on a translated button is accepted and lands. · why: Ratified. · guard: stand_act_transformed
decision/type-replaces-control-content · Type replaces what a control holds — The type verb replaces what the target control holds, selecting all and typing over it, and an empty text clears the control; a target that reads no value is typed into as is. · why: Chosen in place of appending the text after the old value.
decision/validated-commands-only · Driver commands run only after validation — Every driver call runs through the session's one run entry, which validates first; nothing public runs a command that did not come from validation.
decision/validated-verb-set-widened-by-ratification · Validated verb set widened only by ratification — The driver's validated verb set is closed; a verb joins it only as a ratified widening of that set, at the instance, session or tool level. · why: Each verb beyond the first acting verbs joined as a ratified widening of the validated verb set.
decision/visibility-from-geometry-not-hit · Visibility is read from geometry, never from a hit — Whether an element can be seen is read from the visible-region geometry of its clipping chain and never from a hit test.
contract/acting-verb-results/added · acting verb results · added — Nodes on the screen after the step that were not before, written without differs.
contract/acting-verb-results/busy · acting verb results · busy — One of the busy classes, present only when settled is false.
contract/acting-verb-results/changed · acting verb results · changed — Elements before and after the step whose role, name, state, bounds or parent differ, as they read after it, each carrying differs.
contract/acting-verb-results/removed · acting verb results · removed — The ids of the elements the step removed.
contract/acting-verb-results/settled · acting verb results · settled — A flag; a step that did not go quiet is a result with settled false and its busy class, never a refusal or a session error, and is not rolled back.
contract/actionable-nodes/actionable-nodes · Actionable nodes · actionable_nodes — Read-only under accessibility, returns id, owner and keyed for each UI graph node the actionable rule holds for or whose owner is set, in snapshot pre-order; keyed is true where the id holds no /.
contract/actionable-readers/actionable-nodes · Dioxus actionable readers · DioxusDocument::actionable_nodes — actionable_nodes lists, read-only and in snapshot pre-order, every UI graph node the actionable rule holds for or the graph gives an owner, as an ActionableNode (id, owner, keyed where the id holds no /).
contract/actions-verb/acceptance · The actions verb · acceptance — Each verb's call is built with fixed in-bound values for other required arguments and run through the same validation and per-verb target check a real call meets.
contract/actions-verb/changes-nothing · The actions verb · side effects — The actions read changes nothing: no changes are pending after it, the snapshot, both focus readings and the diff are unchanged, and an open select list stays open.
contract/actions-verb/meaning · The actions verb · accepted — `actions` answers whether a call would be accepted, not what it would do.
contract/actions-verb/stale · The actions verb · stale — A node that resolves to no element reads `stale`.
contract/actions-verb/target-verbs · The actions verb · target verbs — For each node listed, the read covers each instance verb that takes an id argument, in table order, today `click` · `type` · `scroll` · `screenshot`.
contract/actions-verb/unpaged · The actions verb · paging — `actions` takes no argument and answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound.
contract/argument-kinds/id · argument kinds · id — 1 to 1024 bytes of text.
contract/argument-kinds/key · argument kinds · key — One of the twelve key names, else out of bound.
contract/argument-kinds/milliseconds · argument kinds · milliseconds — A whole number from 1 to 60000.
contract/argument-kinds/part · argument kinds · part — A whole number from 1 to 1000, the snapshot part asked for.
contract/argument-kinds/text · argument kinds · text — 0 to 4096 bytes of text.
contract/base-document/has-changes · BaseDocument · has_changes — Reads true while the changed set is non-empty, from the first tracked write until the next drain.
contract/base-document/take-changed-nodes · BaseDocument · take_changed_nodes — Returns the changed set and leaves it empty; a returned id may name a dropped node, so it is read through get_node.
contract/blitz-dom-clock/virtual-clock · blitz-dom clock API · VirtualClock — Clone and Default, every clone reading one shared nanosecond counter; new() reads zero and advance(Duration) adds with saturation.
contract/busy-classes/layout · busy classes · layout — A busy class word of an instance that did not go quiet.
contract/busy-classes/loads · busy classes · loads — A busy class word of an instance that did not go quiet; a load in flight is waited for at most the load wait.
contract/busy-classes/render · busy classes · render — A busy class word of an instance that did not go quiet.
contract/ci-leg-legs/ci-scripts · ci-leg.sh legs · ci-scripts — `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts.
contract/ci-leg-legs/clippy · ci-leg.sh legs · clippy — The `clippy` leg runs the workspace clippy line and, only when that is green, upstream's own blitz-dom clippy line with `-D warnings`.
contract/ci-leg-legs/embolden · ci-leg.sh legs · embolden — The `embolden` leg runs `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, has no CI job, and is the last step of `fast`.
contract/ci-leg-legs/fast · ci-leg.sh legs · fast — The `fast` leg runs fmt, clippy, test, ci-scripts and embolden in order, stopping at the first red: the local pre-push gate.
contract/ci-leg-legs/fmt · ci-leg.sh legs · fmt — The `fmt` leg runs `cargo fmt --all --check`.
contract/ci-leg-legs/test · ci-leg.sh legs · test — `test` is a leg of the CI leg script.
contract/command-line/verb-flags · command line · verb flags — --id, --text, --key, --ms and --part each take one value, --shift is bare, and --session <dir> exactly once; the call is validated before the session is asked for.
contract/command-span/added · command span · added — The length of the diff's added list.
contract/command-span/busy · command span · busy — The busy class word, recorded when the outcome carries one.
contract/command-span/cause · command span · cause — The refusal's cause name, recorded for every refused call.
contract/command-span/changed · command span · changed — The length of the diff's changed list.
contract/command-span/never-fielded · command span · never fielded — No screen text, image, findings, graph, actions answer, label, id record, argument, call, outcome or refusal value is ever a field.
contract/command-span/passes · command span · passes — Settle's pass count, recorded when settle answered quiet.
contract/command-span/removed · command span · removed — The length of the diff's removed list.
contract/command-span/settled · command span · settled — Recorded when the outcome carries one.
contract/command-span/verb · command span · verb — Target escher_driver, span command at INFO: the verb table's word, recorded for a known verb, never the caller's text.
contract/dioxus-bridge/reached-record · Dioxus DOM bridge · reached record — The ungated reached record is written only where it was switched on, inert elsewhere at one has_context lookup per dispatch.
contract/dioxus-bridge/registry-dispatch · Dioxus DOM bridge · handle_event registry dispatch — The bridge's handle_event sets the registry's dispatch around the vdom's only where a registry stands.
contract/dioxus-bridge/screen-readers · Dioxus DOM bridge · snapshot, text form, diff and UI graph — Under accessibility, the snapshot, its text form, its diff and the UI graph read the screen read-only, holding no NodeId.
contract/dioxus-bridge/stable-id · Dioxus DOM bridge · element_id and element_ids — element_id and element_ids compute a stable id on demand in four tiers (author key, anchored path, component path, document path), written to neither DOM nor vdom.
contract/dioxus-bridge/use-graph-state · Dioxus DOM bridge · use_graph_state — An app registers graphed state through the ungated use_graph_state.
contract/dom-events/change · DOM events · change — Not cancelable, bubbling, no UI event mapped and no default action; dispatched by the engine after input on a select's choice and a checkbox's or radio's click, never by a text or range input.
contract/dom-events/change-after-input · DOM event dispatch · change — The engine dispatches change after input on a select's choice and on a checkbox's or radio's click, a radio only where it was not checked before, and a page's script hears that one change.
contract/dom-events/change-on-checkable · DOM events · change on a checkable control — A checkbox click always dispatches change after input with the same value, a radio only where it was not checked before; scripts hear one change, from the engine.
contract/dom-events/driver-type-on-select · DOM event dispatch · driver type on a select — The driver's type on any select focuses it with no click and dispatches no blur, focusout, focus or focusin.
contract/dom-events/enter-space-click · DOM event dispatch · Enter and released Space — Enter and a released Space dispatch a Click the driver queues, so the app's handlers run before the default action.
contract/dom-events/focus-change · DOM event dispatch · focus change — A focus change dispatches blur, focusout, focus, focusin.
contract/dom-events/inert-focus-fixup · DOM event dispatch · focus fixup under inert — The focus fixup under inert clears focus and dispatches no blur, focusout, focus or focusin.
contract/dom-events/range-input-default · DOM event dispatch · range input default actions — A range input's default actions dispatch input.
contract/dom-events/text-range-input-no-change · DOM event dispatch · text or range input's input — A text or range input's input brings no change event.
contract/driver-json/diff · Driver JSON answer · diff — added and changed are written as nodes keyed by NODE_FIELDS, a changed node carrying differs after bounds, and removed as ids.
contract/driver-recording/handler-coverage · escher-driver recording · handler_coverage — handler_coverage replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised.
contract/driver-session/command-schema · escher-driver session · command and refusal schema — The command and refusal schema is stated once as 'static data: fourteen verbs, eight closed causes.
contract/driver-session/lint · escher-driver session · Session::lint — Session::lint reads the screen and changes nothing.
contract/driver-session/load-wait · escher-driver session · LOAD_WAIT — LOAD_WAIT (5 s) bounds an acting step's wait for a load, moving no time.
contract/driver-session/no-nodeid · escher-driver session · NodeId — No NodeId crosses the driver session boundary.
contract/driver-session/refusal-content · escher-driver session · refusal — No refusal holds what a call supplied.
contract/driver-session/schema-carriers · escher-driver session · schema carriers — The v3 wire, the command line, the MCP server and the command and tool_call spans carry the one command and refusal schema.
contract/driver-verbs/actions · driver verbs · actions — No argument; result nodes, what each actionable node can take now; dispatches nothing, runs no step.
contract/driver-verbs/advance · driver verbs · advance — Takes ms, 1 to 60000; one settled step; adds advanced_ms, exactly ms.
contract/driver-verbs/click · driver verbs · click — Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a select opens its list.
contract/driver-verbs/graph · driver verbs · graph — No argument; result nodes and states, the screen's UI graph read whole; dispatches nothing, runs no step.
contract/driver-verbs/lint · driver verbs · lint — No argument; result findings, always present; geometry only, no hit, right-to-left and vertical text read as left-to-right lines; dispatches nothing.
contract/driver-verbs/press · driver verbs · press — Takes key and optional shift, false when absent; presses on what holds focus, carrying no Ctrl, so non-adjacent rows of a multiple select cannot be built.
contract/driver-verbs/screenshot · driver verbs · screenshot — One optional id; result width, height and png (base64), always present; an element shot is cropped to its visible bounds; no step, nothing changes.
contract/driver-verbs/scroll · driver verbs · scroll — Takes id; accepted on a disabled or covered element, scrolls it into view as one settled step and adds in_view, always present, false when no scroll could bring it in view or it was removed.
contract/driver-verbs/snapshot · driver verbs · snapshot — One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text.
contract/driver-verbs/type · driver verbs · type — Takes id and text; on a select chooses the first option whose label starts with the text, on a multiple select its match alone, a text opening on a space typing nothing.
contract/event-dispatch/activation-click-order · Event dispatch · activation click — A keyboard activation Click is queued so the app's handlers, such as a Dioxus onclick, run before the default click action.
contract/event-dispatch/button-click-focus · Event dispatch · button click — Clicking a button or an input of type submit, reset or button focuses it before a submit button submits its form owner.
contract/event-dispatch/enter-click · Event dispatch · pressed Enter — A pressed Enter on the focused node dispatches a Click, after the text-input and range arms, queued so the app's handlers run before the default action.
contract/event-dispatch/focus-change · Event dispatch · focus change — A focus change dispatches blur, focusout, focus, focusin.
contract/event-dispatch/range-input · Event dispatch · range input input event — A range input's default actions dispatch input carrying the formatted current value, one per change and none for a move that changes nothing.
contract/event-dispatch/space-release-click · Event dispatch · KeyUp Space — A released Space on the focused node dispatches a Click.
contract/event-state/fields · EventState · fields — cancelled, propagation_stopped and redraw_requested, combined by merge.
contract/form-controls/is-disabled · blitz-dom form control readers · ElementData::is_disabled — ElementData::is_disabled is the one disabled reading of a form control: a disabled attribute present.
contract/form-controls/option-disabled · blitz-dom form control readers · option_disabled — An option's disabled reading is the one rule option_disabled: its own attribute or its group's.
contract/form-controls/select-list-state · blitz-dom form control readers · node::select_list — An open list's state lives crate-private in node::select_list but for two public read types re-exported from the crate root: SelectListView, the list's colours its associated constants, and SelectListRow.
contract/graph-registry/declared-write · Graph registry · DeclaredWrite — Names an element's stable id or a prefix ending in *, and an event written without on.
contract/graph-registry/use-graph-state · Graph registry · use_graph_state — Takes a name, declared writes and subscribers of dioxus-core types alone; the first registering hook provides the registry at the root scope, and each entry leaves with its scope.
contract/graph-verb/argument · The graph verb · argument — `graph` takes no argument; an argument is refused `malformed` with nothing run.
contract/graph-verb/no-values · The graph verb · content — The answer `{"nodes":[…],"states":[…]}` holds ids, component, state and event names, closed flag words and booleans only, never a value.
contract/graph-verb/node · The graph verb · node — A node is keyed `id` · `owner` · `component` · `handlers` · `reads` (`owner`, `component` left out when absent); a handler `event` · `writes`; a write `state` · `declared` · `confirmed` · `flag`, `flag` left out when none.
contract/graph-verb/state · The graph verb · state — A state is keyed `name` · `component` · `declared` · `writers` · `readers` (`component` left out when absent); a declaration `id` · `event`; a writer `id` · `event` · `declared` · `confirmed` · `flag`.
contract/graph-verb/unpaged · The graph verb · paging — `graph` answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound.
contract/handler-coverage/bubbling · Handler coverage reader · bubbling dispatch — A bubbling dispatch counts every ancestor listener it passed; a `stopPropagation` is not read.
contract/handler-coverage/reached · Handler coverage reader · reached handlers — The UI graph is read at boot and after each replayed step, `mounted` left out, deduplicated by id and event, and the handlers the replay reached are taken from it.
contract/headless-stand/offline-boots · seven_guis headless stand · network — Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms).
contract/headless-stand/timer-step · seven_guis headless stand · timer_step — timer_step hands the Timer its ticks for a driver session.
contract/html-open-attribute/select · HTML open attribute · select — An author's open on a select steers nothing; open state and :open hold to a details or a dialog.
contract/lint-verb/changes-nothing · The lint verb · side effects — The lint records each node's id as a snapshot does and runs no step, settle, pump or time move, so it changes nothing.
contract/lint-verb/fonts-unavailable · The lint verb · fonts-unavailable — The lint is refused `fonts-unavailable` before anything is read when text cannot be painted.
contract/lint-verb/wire-form · The lint verb · written form — The `lint` verb answers `{"findings":[…]}` on every carrier and is recorded as a step with its answer kept whole; its `command` span records `verb` and, refused, `cause` alone.
contract/load-wait/bound · Driver acting step · LOAD_WAIT — An acting step waits up to 5 s of wall time for a load in flight, then returns with `settled` false and `busy` `loads`, at or past the bound and within one settle of it; a `render` or `layout` answer returns at once.
contract/load-wait/no-time-moves · Driver acting step · wait side effects — No time moves on the held instance while it waits: no tick, no time step, no changed-set drain, no print and no tracing call.
contract/load-wait/verbs · Driver acting step · waiting verbs — The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph` and `actions` wait on nothing.
contract/mutation-query/mutate · blitz-dom mutation, query and CSSOM · doc.mutate() — Mutations go through doc.mutate().
contract/node-keys/enabled · node keys · enabled — Follows the disabled attribute's presence whatever its value; left out when absent.
contract/reached-record/reached-handlers · Reached record · reached_handlers — Returns the reached handlers in first-reached order, deduplicated by id and event, empty when the record was never switched on.
contract/refusal-causes/covered · refusal causes · covered — Another element is hit at the point the action would land; its meaning states the closed-select option, one-axis clip and zero-size box readings.
contract/refusal-causes/disabled · refusal causes · disabled — A click or type target reads enabled false; checked first of the three screen-level causes.
contract/refusal-causes/fonts-unavailable · refusal causes · fonts-unavailable — The app has no font it can paint text with; screenshot and lint refuse with nothing painted or read; remedy: boot the app with a font it can decode.
contract/refusal-causes/malformed · refusal causes · malformed — An argument is unnamed, repeated, missing, of another kind or out of bound, carried with its fault; returned by validation.
contract/refusal-causes/not-found · refusal causes · not-found — The id names no element on the screen as it reads now and the session's record does not hold it; nothing runs.
contract/refusal-causes/off-screen · refusal causes · off-screen — The element lies outside the viewport or a scrolling box's visible part, or a screenshot crop holds no pixel; remedy: bring it into view with scroll, then act again.
contract/refusal-causes/stale · refusal causes · stale — The id names no element on the screen now but the session's record of read ids holds it; nothing runs.
contract/refusal-causes/unknown-verb · refusal causes · unknown-verb — The verb is not in the table by exact name, or a session- or tool-level verb is handed where an instance verb is run; returned by validation.
contract/refusal-faults/missing · refusal faults · Missing — A required argument is absent; beside malformed only.
contract/refusal-faults/out-of-bound · refusal faults · OutOfBound — An argument's value is outside its bound, or a text is no key name; beside malformed only.
contract/refusal-faults/repeated · refusal faults · Repeated — An argument name was passed before in the same call; beside malformed only.
contract/refusal-faults/unnamed · refusal faults · Unnamed — An argument names nothing the verb takes; beside malformed only.
contract/refusal-faults/wrong-kind · refusal faults · WrongKind — An argument's value is of another kind than the schema's; beside malformed only.
contract/screenshot-error/empty-rect · ScreenshotError · EmptyRect — Returned when the requested rectangle, clipped to the viewport, is empty.
contract/screenshot-error/fonts-unavailable · ScreenshotError · FontsUnavailable — Returned with nothing painted when the document cannot paint text.
contract/script-events/dispatch-order · Script event handling · dispatch order — Events reach JS listeners before Blitz default actions, and JS `preventDefault` / `stopPropagation` are fed back into the event state.
contract/scroll-hit/scroll-into-view · BaseDocument scroll and hit testing · scroll_into_view — scroll_into_view scrolls every scrolling box on the target's chain, nested ones instantly with no scroll event, then the viewport; a row of an open select list writes the list's scroll_y at once and moves no chain or viewport.
contract/select-list-default-actions/choice · Select list default actions · choice — A choice from the list dispatches input then change.
contract/select-list-default-actions/click-select · Select list default actions · click on select — Clicking a single select opens its list, or closes one standing, focusing the select.
contract/select-list-default-actions/label-click · Select list default actions · label click — Clicking a label bound to a single select focuses the select and opens nothing, with no re-dispatch to it.
contract/select-list/close-select-list · BaseDocument open select list · close_select_list — The public close_select_list closes an open list; its callers are keyboard, pointer, reconcile and the driver's focus_select.
contract/select-readers/choice-writers · BaseDocument select readers · select choice writers — The writers of a select's choice are crate-private.
contract/target-check/click-type · Per-verb target check · click and type — `click` and `type` are refused `disabled`, then `off-screen`, then `covered`.
contract/target-check/screenshot · Per-verb target check · screenshot with an id — A `screenshot` given an `id` meets in-view, then an empty crop, then `fonts-unavailable`, before anything is painted.
contract/target-check/scroll · Per-verb target check · scroll — `scroll` meets no target check.
contract/test-harness/busy · Test harness · Busy — Names only a class, Render, Layout or Loads, with no node, element, name, URL or value; NotSettled displays one fixed message per class.
contract/test-harness/harness · blitz-test-harness · Harness — Harness is generic over the document; its input helpers pump and never settle.
contract/test-harness/screenshot · blitz-test-harness · screenshot and screenshot_rect — `screenshot` and `screenshot_rect` resolve at the harness's own time without tick or pump, paint a raster of the viewport through blitz-paint, composite it over opaque white, and print, panic and log nothing.
contract/test-harness/settle · Test harness · settle — settle drives render and layout to quiet, reports a load in flight and never waits on one or moves time; it returns NotSettled with Loads when a request or render-blocking resource is pending after a quiet pass, else Settled with its pass count and whether animating.
contract/test-harness/virtual-clock · blitz-test-harness · harness VirtualClock — The harness holds one VirtualClock its documents share; tick and advance_time move it, and screenshot, screenshot_rect, text_contrast and settle never do.
contract/ui-graph/graph · UI graph · graph — Read-only under accessibility, returns one node per snapshot node in pre-order, each with id, owner, component, handlers and reads, and states with name, component, declared, writers and readers.
contract/ui-graph/owner · UI graph · owner — The stable id of the select an option or an optgroup belongs to.
contract/ui-graph/readers · UI graph · readers — A state's readers name each reading component once, in the pre-order of its first node; a state only the app root reads is named in its nodes' reads alone.
contract/ui-graph/repeated-registration · UI graph · repeated registration — A repeated name and component registration reads {name}:{k}.
contract/ui-graph/unconfirmed · UI graph · unconfirmed — Fires only once a dispatch of the handler's event passed through its element and no run wrote the state.
contract/ui-graph/write-flag · UI graph · WriteFlag — A write is flagged Undeclared or Unconfirmed, each with its word().
contract/ui-graph/write-resolution · UI graph · write resolution — A write resolves to the first element of its chain carrying a listener of its event whose owning scope is the writing scope, else the first carrying one, or to nothing when an earlier node of the chain no longer resolves.
contract/ui-graph/writers · UI graph · writers — Lists only writes a run confirmed or flagged, while declared keeps the declaration as authored.
contract/validation-order/first-failure · call validation · first failure — One answer a call, the first failure in order: the verb by exact name, then the call's arguments as passed, then the verb's arguments in the schema's order; a refused call changes nothing.
contract/verb-table/order · Driver verb table · VERBS — The verb table holds fourteen verbs, `graph` listed after `lint` and `actions` after `graph`.
decision/absolute-position-ancestor-scroll-only · Absolute position subtracts only ancestors' scroll — The absolute position of a node subtracts only its ancestors' scroll offsets, so a box reads at the same position at every scroll offset of its own, and every caller reads such a box where it stands. · why: Ratified. · guard: scrolled_box_absolute_position
decision/anchored-id-tier · Anchored id tier reaches one component — An author key anchors the ids of elements below it only inside its own component instance, spelled key//segment; a template root owned by another component restarts at that component's chain. · why: The anchored tier, its key//segment spelling and its one-component reach are ratified.
decision/changed-set-refresh-then-drain · Changed set, refresh then drain — A document's changed set holds every tracked write until drained, node creation marking nothing; the windowed shell drains it, and a drained id is read as possibly dropped, never by indexing. · why: ratified; the engine half of one item with the shell's refresh on change, changing behaviour for every Blitz document
decision/file-url-load-rule · File URLs only for file documents — A `file:` URL may be loaded only by a document whose own URL is `file:`; fetch, module imports and script sources each ask before the fetcher is called and read a refusal as Forbidden.
decision/graph-signal-hook-in-seven-guis · Graph signal hook lives in seven_guis — The Signal form of the graph-state hook lives in the stand application, not in the Dioxus DOM bridge. · why: dioxus-native-dom names no crate that defines Signal.
decision/harness-counts-loads-at-net-provider · Harness counts loads at the net provider — A supplied non-noop net provider is wrapped in a counting provider that counts a request in flight until its handler is consumed or dropped and forwards errors; an offline harness or a wrapped document is read through its pending critical resources alone. · why: Loads are counted where every request crosses.
decision/harness-reexports-key-types · Harness re-exports keyboard types — The test harness re-exports the keyboard types a key press is named with. · why: A crate depending on the harness names a key without a keyboard dependency of its own.
decision/is-select-public-re-export · Select test public and re-exported — The engine's select test is public and re-exported so the driver can test for a select. · why: escher-driver has no blitz-dom dependency to name local_name! with; the masked-input test is the precedent
decision/keyed-wire-form · keyed as the actionable-key wire form — ActionableNode's keyed reading, true where the id holds no /, is the actionable-key check's first wire form, carried by the actions verb on every carrier.
decision/lint-own-bounds-type · Lint's own rectangle type — Every lint rectangle is the lint's own bounds type in the snapshot bounds' space, and lint types hold no engine node id, page text, colour or value. · why: the driver crate names neither blitz-dom nor stylo
decision/listeners-read-from-vdom · Listeners read from the vdom walk — Every reader of an element's listeners reads the vdom walk's current listeners, never the bridge's listener mark. · why: The listener mark over-reports: a listener a re-render removes leaves its attribute and mark standing, and a mounted listener writes none.
decision/multiple-removal-settles-one-option · Removing multiple settles the chosen set to one — Removing multiple from a select through the mutator settles its chosen set to one option. · why: Pinned by multiple_removed_settles_the_set_to_one_option; the Dioxus bridge removes attributes through the mutator
decision/password-caret-placed-by-bullets · Password caret placed by bullets — A click inside a password field places the caret by the laid-out bullets, so no width of the real characters reaches the caret; a double click selects the whole value. · why: Ratified. · guard: password_caret_click
decision/scrolled-box-bounds · Scrolled box bounds — A scrolled box reads the rect it stands at, the same before and after its content is scrolled; only the scroll offsets of boxes above it on the containing-block chain move it. · why: Fixed on a ratified ruling. · guard: scrolled_box_client_rect
decision/snapshot-text-budget · Snapshot text budget — One screen's text stays under SNAPSHOT_TEXT_BUDGET, 10,000 bytes, a ceiling callers compare against and text_parts cuts within, never enforced by to_text. · why: Keeps a screen inside the agent client's documented 10,000-token warning, 25,000-token maximum and 50,000-character file threshold.
decision/snapshot-text-never-logged · Snapshot text never logged — The snapshot text leaves the process only as a returned value or a hosted call's answer; no log or event carries it, and a kept check holds it only in an owner-only file never logged. · why: Ratified. · except: The two committed checks are the one exception to owner-only.
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
decision/stand-typography-dejavu-sans · Stand typography is DejaVu Sans — The stand's one bundled font, DejaVu Sans, is registered for every generic family with system fonts off in every stand boot; the typography is stated in the stand contract, not on the design-system product surface.
decision/tool-call-span-tool-and-ending · MCP tool call span with tool and ending — Each MCP tool call leaves one span carrying only the tool's table word and its ending, the command span of a session run nested under it. · why: Ratified.
contract/actionable-element/definition · Actionable element · definition — An element is actionable when it is focusable, its role is one of the closed interactive roles, or its current listeners hold an event other than mounted; unkeyed_actionable and actionable_nodes read this one rule.
contract/actionable-readers/unkeyed-actionable · Dioxus actionable readers · DioxusDocument::unkeyed_actionable — unkeyed_actionable lists, read-only, each actionable element whose stable id holds a /, with a remedy naming the fix.
contract/answer-json/actions · answer JSON · actions — {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent; never a value, typed text or NodeId.
contract/answer-json/node · answer JSON · node — An object keyed by the node keys in order, parent, enabled, checked, value, selected and pressed left out when absent; id, role, name, invalid, focused, clickable and bounds always written.
contract/answer-json/outcome · answer JSON · outcome — An object keyed by its verb's result fields in the table's order, busy present only when not settled.
contract/argument-kinds/flag · argument kinds · flag — A boolean.
contract/base-document/can-paint-text · BaseDocument · can_paint_text — Public and read-only: whether the font context resolves a family for the sans-serif generic.
contract/base-document/select-of · BaseDocument · select_of — Public and read-only: the select a node belongs to, if any.
contract/base-document/select-value · BaseDocument · select_value — Public and read-only HTML select.value: the chosen option's value, a multiple select's first chosen option in tree order, None with none chosen or for a node that is no select.
contract/base-document/select-values · BaseDocument · select_values — Public and read-only: the select's chosen values.
contract/blitz-dom-crate-root/select-list-view-types · blitz-dom crate root · SelectListView and SelectListRow — The open select list's view and rows are public read types re-exported from the crate root.
contract/ci-leg-legs/stand-repeat · ci-leg.sh legs · stand-repeat — The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
contract/cli-exit-statuses/1 · CLI exit statuses · 1 — refused: the refusal's JSON on stdout.
contract/dioxus-bridge/event-routing · Dioxus DOM bridge · data-dioxus-id — DOM events reach the vdom through data-dioxus-id.
contract/dioxus-bridge/virtual-node-id · Dioxus DOM bridge · virtual_node_id — A reserved fifth id form {owner}///{key} is spelled by the public virtual_node_id and written by no walk.
contract/dioxus-form-events/multiple-select-values · Dioxus form events · multiple select values — On Input and Change a multiple select fills values with one text entry per chosen option, so evt.values() reads the set, while value stays the select's value.
contract/dioxus-form-events/select-change · Dioxus form events · select Input and Change — A select's Input and Change reach the app as form data carrying its value, so an onchange handler fires.
contract/dioxus-native-dom/closed-select-of · dioxus-native-dom · closed_select_of — Answers None for a select whose list is open.
contract/document-clock/documentconfig-clock · blitz-dom document clock · DocumentConfig.clock — The document's clock is the config's clock, or else a fresh WallClock.
contract/document-clock/virtualclock · blitz-dom document clock · VirtualClock — Every clone of a VirtualClock reads one time.
contract/document-config/clock · DocumentConfig · clock — An optional shared Clock, None by default, in which case the document uses a fresh WallClock.
contract/document-core/changed-set · blitz-dom document core · changed set — The changed set is read by has_changes and drained by take_changed_nodes; its ids are read through get_node, never by indexing.
contract/document-core/documentconfig · blitz-dom document core · DocumentConfig — DocumentConfig carries the viewport, providers, user-agent sheets, fonts, media type, threading, incremental flag, abort signal, depth and clock.
contract/dom-events/non-bubbling · DOM event dispatch · non-bubbling events — Non-bubbling events are dispatched to the target only.
contract/dom-events/pointer-events · DOM event dispatch · pointer-derived events — Mouse compatibility events and touch events are dispatched from pointers.
contract/driver-forms/closed-select-of · driver form-control readers · closed_select_of — The public closed_select_of answers None for a select whose list is open.
contract/driver-forms/select-form-data · driver form-control readers · select form data — A select's input and change reach the app as form data, a multiple select's as one (name, FormValue::Text(value)) per chosen option, so evt.values() reads the set and value stays select_value.
contract/driver-forms/select-snapshot-options · driver form-control readers · select options in the snapshot — A select's options are its children in the snapshot; a multiple select's rows and groups read client rects of their own, and an open list's rows and groups read select_list_bounds before the client rect.
contract/driver-recording/kept-check · escher-driver recording · kept check — The recording keeps a session's answered calls as a kept check, a masked type as its stand-in and a shot without its pixels.
contract/driver-recording/replay · escher-driver recording · Check::replay — Check::replay runs a kept check's calls again through Session::run.
contract/escher-session/actions · escher-session command line · actions — The verb actions takes no argument beside --session.
contract/escher-session/advance · escher-session command line · advance --ms — The verb advance takes --ms.
contract/escher-session/click · escher-session command line · click --id — The verb click takes --id.
contract/escher-session/graph · escher-session command line · graph — The verb graph takes no argument beside --session.
contract/escher-session/lint · escher-session command line · lint — The verb lint takes no argument beside --session.
contract/escher-session/press · escher-session command line · press --key [--shift] — The verb press takes --key and an optional --shift.
contract/escher-session/screenshot · escher-session command line · screenshot [--id <id>] — The verb screenshot takes an optional --id.
contract/escher-session/scroll · escher-session command line · scroll --id — The verb scroll takes --id.
contract/escher-session/snapshot · escher-session command line · snapshot — The verb snapshot reads the screen.
contract/escher-session/two-argument-form · escher-session command line · <task> <state-dir> — The two-argument form is gone: its first word is no verb, so it reads refused, unknown-verb, status 1.
contract/escher-session/type · escher-session command line · type --id --text — The verb type takes --id and --text.
contract/form-controls/select-rules · blitz-dom form control readers · node::select — The select's rules live in the crate-private node::select, with no public type.
contract/geometry/get-client-bounding-rect · blitz-dom geometry readers · get_client_bounding_rect — get_client_bounding_rect reads an element where its transforms move it.
contract/geometry/visible-region-non-finite · blitz-dom geometry readers · visible_region on a non-finite box — visible_region answers None for a mapped box with a non-finite edge.
contract/geometry/visible-region-select-list · blitz-dom geometry readers · visible_region of an open select list's row — A row or a group of an open select list reads the list's scrollport in place of its clipping boxes.
contract/geometry/visible-region-transforms · blitz-dom geometry readers · visible_region through transforms — visible_region reads each clipping box through its transforms, exact under a translate and a scale, under a rotation the rotated padding box's bounding box, so it holds corners the box does not show.
contract/handler-coverage/canvas · Handler coverage reader · canvas handlers — Handlers inside a canvas are not counted.
contract/handler-coverage/per-check-boot · Handler coverage reader · per-check boot — A fresh session is booted for each check; a check whose app is not the label runs nothing and contributes nothing.
contract/handler-coverage/unreached-screens · Handler coverage reader · unreached screens — A handler on no screen a replay reached is not listed, by construction.
contract/handler-coverage/written-form · Handler coverage reader · written form — Coverage is written `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, `owner` left out when absent.
contract/headless-stand/boot · seven_guis headless stand · seven_guis::stand — seven_guis::stand, native only, boots the four lean tasks, Cells and a load page in a pinned 800x600 Light viewport with one bundled font.
contract/key-names/arrow-down · key names · arrow-down — A key name press accepts.
contract/key-names/arrow-left · key names · arrow-left — A key name press accepts.
contract/key-names/arrow-right · key names · arrow-right — A key name press accepts.
contract/key-names/arrow-up · key names · arrow-up — A key name press accepts.
contract/key-names/backspace · key names · backspace — A key name press accepts; on macOS it also dispatches deleteBackward.
contract/key-names/delete · key names · delete — A key name press accepts.
contract/key-names/end · key names · end — A key name press accepts.
contract/key-names/enter · key names · enter — A key name press accepts.
contract/key-names/escape · key names · escape — A key name press accepts.
contract/key-names/home · key names · home — A key name press accepts.
contract/key-names/space · key names · space — A key name press accepts.
contract/key-names/tab · key names · tab — A key name press accepts.
contract/mcp-server/tools-list · MCP server · tools/list — The fourteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.
contract/node-keys/clickable · node keys · clickable — True where the element's current listeners hold click and it is no disabled form control; a boolean on every node.
contract/reached-record/reached-handler · Reached record · ReachedHandler — Carries id and event, re-exported from the crate root.
contract/reached-record/record-reached · Reached record · record_reached — Switches the record on at the root scope; a second call keeps the one record.
contract/refusal-json/shape · refusal JSON · shape — A refusal is written {"refused":{"cause","meaning","remedy"[,"fault"]}} from the cause's fixed strings, holding nothing the call supplied.
contract/scroll-hit/hit-clip · BaseDocument scroll and hit testing · hit on a clipping box — A hit stops at the padding box of a box paint clips.
contract/scroll-hit/hit-degenerate-transform · BaseDocument scroll and hit testing · hit under a degenerate transform — A hit answers nothing for the whole subtree of a transform whose determinant is 0 or not finite.
contract/scroll-hit/hit-dropped-node · BaseDocument scroll and hit testing · hit on a dropped node — A hit skips a node dropped since the last resolve.
contract/scroll-hit/hit-with-scrollbar · BaseDocument scroll and hit testing · hit_with_scrollbar — hit_with_scrollbar answers an open select list first and no scrollbar with it, hit_inner holding none of it, so hit, set_hover_to and handle_pointerdown all read the list.
contract/scroll-hit/stale-id-indexing · BaseDocument scroll and hit testing · NodeTree indexing — Indexing the NodeTree with a stale id panics.
contract/scroll-hit/visible-region · BaseDocument scroll and hit testing · visible_region — visible_region is the read-only reader of what can be seen; seeing is never read from a hit.
contract/select-list-default-actions/pointerdown · Select list default actions · pointerdown — A press in the open list stays there; a press elsewhere closes the list.
contract/select-list/open-select · BaseDocument open select list · open_select — open_select is public and read-only.
contract/select-readers/select-of · BaseDocument select readers · select_of — select_of is public and read-only and reads an option's or group's select.
contract/session-errors/not-settled · session errors · not-settled — An in-process act whose instance did not go quiet, the message appending the busy class; the step is not rolled back.
contract/snapshot-diff/differs · Snapshot diff · differs — The readings that differ from before the step, in Reading::ALL order, empty for an added node; a node that only moved reads Bounds.
contract/snapshot-diff/membership · Snapshot diff · added, removed, changed — A node is added when its id is only after, removed when only before, changed when in both and its role, name, state, bounds or parent's id differ; a descendant's change never names its ancestor.
contract/snapshot-diff/order · Snapshot diff · order — added and changed follow the after snapshot's pre-order, removed the before snapshot's; where two nodes carry one id the first in pre-order stands for it.
contract/snapshot-diff/reading · Snapshot diff · Reading — Twelve variants: Parent, Role, Name, Enabled, Checked, Value, Selected, Pressed, Invalid, Focused, Clickable, Bounds, with Reading::ALL.
contract/snapshot-diff/structure · Snapshot diff · SnapshotDiff — added and changed hold flat nodes read after the step, each with its parent's id, role, name, state, bounds and differs; removed holds ids.
contract/snapshot-diff/values · Snapshot diff · values — Values come from the two snapshots only, so a password or file input reads MASKED_VALUE, and no field holds a process-local id or pointer.
contract/snapshot/bounds · Snapshot · bounds — A row's or group's place in an open list, else the element's viewport-relative, unclamped client bounding rect.
contract/snapshot/checked · Snapshot · checked — A native checkbox's or radio's checked state, else, on one of the five checkable roles, whether its toggle is True; None where the role states nothing.
contract/snapshot/clickable · Snapshot · clickable — True where the element's current listeners hold click and the element is no disabled form control.
contract/snapshot/closed-select-option · Snapshot · closed select option — An option of a closed select, under an optgroup too, reads its select's client rect as bounds and is the select's child, named by its label; while the list is open a row reads its own place in the list.
contract/snapshot/enabled · Snapshot · enabled — Reads the presence of the disabled attribute on an element that can be disabled, so disabled="false" and a bare disabled both read Some(false); aria-disabled moves no reading.
contract/snapshot/focused · Snapshot · focused — The accessibility tree update's focus, never a reading that falls back to the root element.
contract/snapshot/membership · Snapshot · node membership — A node is an accessibility tree node with an author_id whose element resolves; the Window, the document root and text runs are not nodes, their kept descendants attaching to the nearest kept ancestor.
contract/snapshot/selected · Snapshot · selected — The accessibility node's own selected state; a select's option reads its chosen state.
contract/snapshot/snapshot-node · Snapshot · SnapshotNode — A SnapshotNode carries id, role, name, state, bounds and children; a Snapshot offers nodes in pre-order and get by stable element id.
contract/snapshot/value · Snapshot · value — A select's chosen option value, unmasked (a multiple select's first chosen), else the editor text, else a range input's current value, else the value attribute of a textarea or text-entry input.
contract/stable-element-id/element-id · Stable element ids · element_id — Returns an element's stable id, None for a non-element, stale or detached node, and never panics; the id is computed on demand and written to neither the DOM nor the vdom.
contract/stable-element-id/element-ids · Stable element ids · element_ids — Returns every element reachable from the document root, in document pre-order, with ids pairwise distinct.
contract/stable-element-id/removed-node · Stable element ids · removed node — A node a re-render removes is dropped with its descendants and no longer resolves: element_id reads None and element_ids holds no pair for it.
contract/telemetry-line/span-close-line · Telemetry line · span close line — One line per closed admitted span, none on create, record, enter or exit: the same prefix, span=name, its fields in recorded order, then message, time.busy and time.idle, each judged by the event field rule.
contract/unkeyed-actionable/remedy · Actionable-key check · remedy — Names the fix, an HTML id that is non-empty, holds no / and is unique; no field holds the element's text content, accessible name or attribute value.
contract/unkeyed-actionable/unkeyed-actionable · Actionable-key check · unkeyed_actionable — Read-only and argument-free, returns every actionable element whose id holds a / in document pre-order, empty when every one reads its key; logs nothing and never panics.

## occupied-resources · ## Occupied Resources
decision/stand-load-net-answers-after-delay · Load stand-in answers from its own thread after 200 ms — The stand's LoadNet answers each fetch from a thread of its own with the fixed CSS after 200 ms, with no socket or listener. · why: the_load_page_click_leaves_a_load_in_flight and the stand_load_page checks need a load in flight that the settle waits out
contract/actions-answer/argument · The actions verb's answer · argument — The actions verb takes no argument; a call that gives one is refused malformed.
contract/command-span/actions-call · The command span · actions call — An actions call's command span records verb alone, and cause when refused, parented by its tool_call span under the mcp role.
contract/command-span/graph-call · The command span · graph call — A graph call's command span records verb alone, parented by its tool_call span under the mcp role.
contract/driver-graph/answer · the graph verb · answer — On both carriers and in a kept check, the answer holds stable ids, owner ids, component names, handler event names, registered state names, declared writes, the flags undeclared and unconfirmed and booleans; never a value, typed text or NodeId.
contract/driver-graph/argument · the graph verb · argument — graph takes no argument; a call that gives one is refused malformed with nothing run.
decision/headless-documents-sequential-style · Headless documents style sequentially — Sequential style threading stays the default: headless documents of the harness, the stand and the driver never set Parallel; the windowed launch, the browser and the multi-document bench example do.
decision/screen-pixels-in-band · Screen pixels in band — A screenshot leaves in band inside the answer on both carriers, open select labels as painted pixels included, adding no listener or port and leaving the wire's bounds unchanged. · why: A boundary widening, ratified.
decision/stand-load-page-in-process-url · Load page stylesheet URL under .test answered in process — The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network. · why: The stand_load_page driver checks and the session host's load app rely on the boot answering offline with no resolver reached
contract/actions-answer/node-fields · The actions verb's answer · node fields — Each actionable node gives its stable id, owner id and keyed boolean, the table's verb words and the closed cause words; never a value, typed text or NodeId.
contract/actions-answer/paging · The actions verb's answer · paging — The whole screen is answered unpaged, within the 1,048,576-byte answer bound; the wire adds no listener or port.
contract/driver-diff/differs · the driver diff · differs — A changed node's differs names closed node-key words, bounds among them.
contract/driver-lint/answer · the lint verb · answer — On both carriers and in a kept check, the answer holds stable ids and the closed defect words and numbers, never a NodeId, page text, colour or control value.
contract/driver-snapshot/clickable · the driver snapshot · clickable — A snapshot node's clickable is a closed bool, crossing in the snapshot and the diff.
contract/driver-snapshot/row-bounds · the driver snapshot · row bounds — An open select list's row carries its own bounds in the snapshot and the diff.

## infrastructure-patterns · ## Infrastructure Patterns
decision/fmt-clippy-gates · fmt and clippy gates — CI gates every change on rustfmt and clippy.
decision/ci-fast-slow-split · CI fast/slow split — fmt, clippy, test-features-default and ci-scripts carry no needs, and every other CI job, the matrix included, needs all four.
decision/linux-tested-by-fast-leg · Linux tested by the fast leg — Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/animations-advance-by-resolve · Animations advance through resolve with a time — Animations, transitions and smooth scrolls advance only through a resolve given a time, and whether any is running is read by an is-animating query. · guard: pseudo_element_update, fragment_navigation
decision/changed-set-write-sites · Where the changed set is written — The changed set is written only by mutations whose condition holds, focus or checked state changes, typed text input, range moves and select choices that change a value, and a select list opening or closing; never by creation, hover, active, scroll, resize, load, animation or layout. · why: ratified as the engine half of the document-core changed-set item · except: a page scroll or a move that closes an open select list marks its select at the next resolve
decision/default-action-event-order · Default-action event order — Default-action events are queued and processed in order, handlers running before the default action unless cancelled.
decision/escher-driver-spans-ungated · escher-driver spans ungated, no subscriber — escher-driver takes `tracing` with no feature, builds its `command` span and its `tool_call` span (the parent of `command` on an MCP tool call) by hand with no instrument attribute and no feature gate, and installs no subscriber.
decision/first-graph-state-hook-provides-registry · First graph-state hook provides the root signal registry — The first graph-state hook of a page provides a `SignalRegistry` context at the root scope, which the bridge's dispatch looks up; a page that registers no state has none. · why: The bridge's dispatch and the UI graph look the registry up at the root scope; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is
decision/handlers-before-default-action · Handlers run before the default action — Default-action events are queued and processed in order, each running its handlers and then its default action unless cancelled.
decision/reached-recorded-before-handlers · Reached record written before handlers run — Where a reached record is switched on, each node of the dispatch chain whose current listeners hold the event's name is recorded by its stable id before the handlers run. · why: so a handler whose own run removes its element still counts; every other document pays one context lookup per dispatch
decision/redraw-requests · When redraws are requested — Redraws are requested after mutations, resource loads, hover, device changes, a focus set or cleared and a select list opening, moving, scrolling or closing; the Tab key asks for none of its own.
decision/select-list-changed-set · Select list marks in the changed set — Where a select list stands, a page scroll or a move that closes it marks its select at the next resolve and a wheel elsewhere marks it on close, while a highlight or a wheel move inside the list marks nothing.
decision/shell-and-navigation-providers · Shell and navigation providers — A shell provider carries redraw, clipboard and file dialogs, and a navigation provider receives link-click navigations.
decision/signal-write-recorded-against-dispatch · Signal writes recorded against their causing dispatch — Where a signal registry stands, each signal write is recorded against the dispatch that caused it, and a page with no registered state dispatches unchanged. · why: The UI graph reads each write record's dispatch to confirm declared writes; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is
decision/accessibility-feature-gating · Accessibility feature gating in dioxus-native-dom — The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry` and `reached` modules stay ungated. · why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off.
decision/changed-set-engine-touched · Changed set is engine-touched, a superset — The changed set records what the engine touched, a superset of what a snapshot reads as changed, so a resolve with nothing written marks nothing in either layout mode. · why: ratified · except: bounds-only movement changes a reading and marks nothing
decision/dom-mutation-reconstructs-box · Every DOM mutation reconstructs the box — Every DOM mutation inserts box-construction damage. · guard: anonymous_block_cache_invalidation
decision/shared-state-rc-refcell · Shared state as Rc RefCell — Shared state is `Rc<RefCell<…>>`: the blitz document, and JS state in a `DomCtx` on the Boa context.
decision/targeted-restyle-on-mutation · Targeted restyle on mutation — Attribute changes, node removal and insertion restyle through targeted attribute-change and child-list-change restyles, never a blanket subtree restyle.
decision/typed-results · Typed results for fallible APIs — Fallible APIs return typed results.

## project-intent · ## Project Intent
decision/capability-statement · Capability statement — docs/capabilities.md is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, and no check reads its other claims. · guard: .github/scripts/test_capability_statement.py
contract/root-readme/mcp-tool-count · root README.md · count of driver commands an MCP client gets as tools — The README states the count, which the capability-statement pin test reads; no build or doc step reads the README.

## inherited-defaults · ## Inherited Defaults
decision/ci-quality-gates · CI quality gates — Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget. · guard: the CI legs of .github/scripts/ci-leg.sh
decision/fmt-and-clippy-gates · Format and clippy gates — `cargo fmt --all --check` and workspace clippy with warnings denied run, then, only when that line is green, upstream's blitz-dom clippy line, each as its CI leg script leg in CI and on the dev host.
decision/coverage-floors · Per-crate coverage floors — A cargo-llvm-cov coverage report is gated by per-crate floors: CI fails when line coverage falls under 97 % for escher-driver, 83 % for dioxus-native-dom or 90 % for blitz-test-harness. · guard: CoverageFloorTest

## existing-scopes · ## Existing Scopes
contract/escher-session-command-line/coverage · escher-session command line · coverage <app> <check>... — The coverage role lists the handlers its named kept checks left unexercised, the same boot closure called once per check.

# architecture · its absences, names and overviews, whole

## design-philosophy · ## Design Philosophy
absence/browsing-history-in-engine — Browsing history in the engine; every link is unvisited

## stack-and-technologies · ## Stack and Technologies
overview/stack-and-technologies — The stack is a Rust 2024 Cargo workspace (rust-version 1.91.0) on Servo Stylo, html5ever and xml5ever, Taffy, Parley with ICU4X, anyrender over Vello and Skia, winit, AccessKit, Boa, Dioxus 0.7, tokio and reqwest, with rusqlite in the browser app and Trunk for wasm; GitHub Actions CI runs fmt, clippy, rustdoc, cargo-deny and coverage gates, beside a host-run mutation tier and a Nix flake.

## established-decisions · ## Established Decisions
absence/3d-transforms — 3D transforms (a z-axis rotate is applied as planar)
absence/anchor-positioning — Anchor positioning
absence/anonymous-table-wrappers — Anonymous table wrapper boxes
absence/background-clip-text — background-clip: text
absence/container-queries — Container queries
absence/css-offset-in-transforms — CSS `offset` in transforms
absence/dioxus-native-js-eval — JS eval from a Dioxus native app
absence/grid-masonry — Grid masonry
absence/grid-subgrid — Grid subgrid
absence/in-repo-release-tool — An in-repo release tool
absence/non-string-generated-content — Pseudo-element content other than strings
absence/non-string-pseudo-content — Non-string pseudo-element `content`
absence/shadow-dom — Shadow DOM
absence/visited-link-styling — Visited-link styling

## standard-contracts · ## Standard Contracts
absence/script-same-origin-cors — Same-origin and CORS checks on script loads
absence/settle-verb-on-socket — A settle verb or a busy-source reply of its own on the session socket

## occupied-resources · ## Occupied Resources
absence/served-network-port — A served network port: no TCP or UDP port any binary serves
name/binary/blitz — blitz
name/binary/escher-session — escher-session
name/binary/seven-guis-native — seven_guis_native
name/binary/todomvc-native — todomvc_native
name/crate/accesskit-xplat — accesskit_xplat
name/crate/blitz — blitz
name/crate/blitz-dom — blitz-dom
name/crate/blitz-traits — blitz-traits
name/crate/blitz-vibey-script — blitz-vibey-script
name/crate/browser — browser
name/crate/browser-persistence — browser-persistence
name/crate/counter — counter
name/crate/debug-timer — debug_timer
name/crate/escher-driver — escher-driver
name/crate/escher-telemetry — escher-telemetry
name/crate/rdme — rdme
name/crate/seven-guis — seven_guis
name/crate/todomvc — todomvc
name/crate/transparent — transparent
name/crate/wasm-hello — wasm_hello
name/crate/wgpu-texture — wgpu_texture
name/other/blitz-target — blitz-target
name/other/blitz-user-agent — Mozilla/5.0 (compatible; Blitz)
name/other/cold-agent-ps1 — cold-agent.ps1
name/other/cold-agent-sh — cold-agent.sh
name/other/com-dioxuslabs-blitz — com.dioxuslabs.blitz
name/other/dioxus-scheme — dioxus://index.html
name/other/mcp-escher — mcp__escher
name/other/multi-doc-flat — MULTI_DOC_FLAT
name/other/multi-doc-n — MULTI_DOC_N
name/other/multi-doc-seq — MULTI_DOC_SEQ
name/other/paint-tree-bench-html — PAINT_TREE_BENCH_HTML
name/other/rust-log — RUST_LOG
name/other/tmpdir — TMPDIR
name/other/wpt-dir — WPT_DIR
name/server/escher — escher

## project-intent · ## Project Intent
overview/project-intent — escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.

## inherited-defaults · ## Inherited Defaults
absence/network-listener — A served network (TCP or UDP) listener
