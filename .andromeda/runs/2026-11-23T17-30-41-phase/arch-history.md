@history_item n=1 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** (Established Decisions: 2 row(s)) — added `+ decision/canvas-crosses-as-pixels` (a canvas's app-painted content crosses every ratified carrier only as screenshot pixels, no new verb, field, cause, exit, log field or span; ratified, F1) and `+ decision/harness-paints-custom-widgets` (blitz-test-harness names blitz-paint's custom-widget feature). Why it matters: Circle Drawer's circles move off the DOM onto a widget, so they leave the snapshot, tree, graph and lint and reach an agent only as shot pixels; the scope's "no new verb, carrier or field" boundary rests on this row, and the stand checks' shot of the circles relies on the harness feature.

@history_item n=2 marker=2026-11-23-canvas-placement-and-text
**2026-11-23-canvas-placement-and-text** (Established Decisions: 1 row(s), amd-6man7tgt) — added `+ decision/custom-widget-content-box-paint`: a widget paints at its content box times the scale, appended at the content origin inside the element's transform, unclipped; a direct engine change flagged upstreamable, guard `stand_canvas_placement`; the founder's F1 «A+C: отрисовка и события указателя», painting and pointer events. Why it matters: the chunk states the widget's paint, placement and pointer frame stand as built, so the circle drawing and the forwarded pointer coordinates read in that content-box frame; the new click synthesis must keep the widget hearing every pointer event as that ruling made it.

@history_item n=3 marker=2026-11-23-canvas-placement-and-text
**2026-11-23-canvas-placement-and-text** (Established Decisions: 1 row(s), amd-jmuz4bwl) — `~ decision/custom-widget-content-box-paint` why gains the bare «Ratified», the plan's expected amendment for a direct engine change of the founder's ruling. Why it matters: the precedent for how this chunk's F2 A engine change (click and double-click synthesis on a custom-widget target) should land in the body: ratified, a direct engine change for every document behind `custom-widget`, flagged upstreamable.

@history_item n=4 marker=2026-11-23-canvas-placement-and-text
**2026-11-23-canvas-placement-and-text** (Standard Contracts: 2 row(s)) — added `+ contract/nodes-widgets/widget-fonts` (`Widget::fonts`, a defaulted method handing the widget the document's shared Parley font context before each paint) and `+ contract/base-document/font-ctx` (public behind `custom-widget`). Why it matters: Circle Drawer's canvas text (each circle's diameter at its centre) is painted from the document's own font context through `Widget::fonts`, as scope F3 A and matrix#v040-09 require, with no new font path.

@history_item n=5 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`CustomWidget`'s attribute replay) — `ElementData::set_custom_widget` replays each existing attribute to `Widget::attribute_changed` with no old value, behind `custom-widget` (upstream #1322, `a26c6ddeb`). Why it matters: the circle list reaches the widget as one `data-circles` attribute parsed in `attribute_changed`; the replay means the widget sees the attribute already set at mount, and the parser must take a missing old value and malformed text as no circle without a panic.

@history_item n=6 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** (Existing Scopes: 1 row(s), amd-bs66iouj) — `~ decision/no-screenshot-in-cross-os-byte-check` except: a canvas-only shot is held byte-equal on linux x86_64, windows x86_64 and macOS aarch64 by one committed digest table, no pixels committed; a shot holding document text stays out (F1 A). Why it matters: Circle Drawer's element shot of its canvas joins `stand_canvas_cross_os`'s digest table under this exception. If the canvas paints diameter text, that shot holds text drawn from the bundled font, so the watch's cross-OS equality must be measured. The whole screen stays outside every cross-OS byte check.

@history_item n=7 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** (Existing Scopes: 1 row(s), amd-pbnf5rg7) — the same row's why restated: a shot holding document text differs in PNG bytes between linux and macOS, and a canvas-only shot reads equal on the three hosts. Why it matters: this is the measured cause behind the digest table this chunk extends. A canvas shot whose diameter text comes from the bundled font is a case between those two readings, so it is the chunk's to measure on CI before its digest is committed.

@history_item n=8 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** (Occupied Resources: 1 row(s)) — `~ decision/committed-stand-checks` why scoped: only a shot holding document text differs by OS, and committed stand checks still keep no screenshot step. Why it matters: the new Circle Drawer stand checks and any kept check must keep no screenshot step. The canvas digest lives only in the cross-OS table.

@history_item n=9 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (keyboard activation and a click's focus, the sixth and seventh direct engine changes) — made `handle_click`'s submit arm a button arm that focuses and ends the walk, added keyboard activation through `dispatch_activation_click`, and set the Click as queued so app handlers run before `handle_click`. The merge surface is `events/keyboard.rs`, `events/mod.rs`, `events/pointer.rs` and `node/element.rs`. Why it matters: this is the click walk and synthesis path the F2 A change re-enters for a custom-widget target. It is the precedent for a direct engine change in `events/mod.rs` and `events/pointer.rs`, and its `decision/click-focuses-button` rule (any other element clears focus) bears on the canvas `<object>`, which takes focus on a click.

@history_item n=10 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** (the range and button arms as private named functions) — `handle_click`, `handle_pointerdown` and `handle_key_or_input_event` keep their signatures. Their escher arms became private named functions (`click_button`, `press_range_input`, …), and the parents stay at their upstream floor so the next sync's diff stays out of arms escher does not own. Why it matters: this is the precedent for the scope's "in a function of escher's own". The click and double-click synthesis after `Widget::handle_event` goes into a helper with one call in `handle_pointerup`'s or the event dispatcher's body.

@history_item n=11 marker=2026-11-22-epoch-2-corrective
**2026-11-22-epoch-2-corrective** (Stack and Technologies: 1 row(s)) — added `+ decision/fork-additions-in-own-functions`: a fork addition to an upstream function over the cognitive-complexity ceiling goes into an escher function, with one call in upstream's body (the founder's word). Why it matters: the custom-widget arm in `events/mod.rs:183-207` and the click queueing in `events/pointer.rs` are upstream functions, so the F2 A addition must follow this row.

@history_item n=12 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Cross-cutting Patterns: 4 row(s)) — added `+ decision/events-queued-on-document` (engine events are queued on the document and flushed by the event driver, never through a producer callback, and queued events for dropped nodes are discarded). It also set the except of `decision/default-action-event-order` and `decision/handlers-before-default-action` to «a cancelled pointerup still clicks and still ends a scrollbar drag». Why it matters: the synthesized Click and DoubleClick on a canvas target must be queued through this one event-queue model. Whether a widget-consumed or cancelled pointerup still clicks is constrained by this exception.

@history_item n=13 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Conventions: 1 row(s)) — retired `− decision/dispatch-event-callback` → `decision/events-queued-on-document`: no event producer takes a dispatch callback. Why it matters: the F2 A synthesis must not reintroduce a callback path in `events/mod.rs` or `events/pointer.rs`.

@history_item n=14 marker=2026-11-21-dependency-and-ci-debts
**2026-11-21-dependency-and-ci-debts** (ratification of the upstream sync's card operations — Cross-cutting Patterns, amd-cm7pmnn3) — the founder ratified the cancelled-pointerup-still-clicks exceptions on `decision/default-action-event-order` and `decision/handlers-before-default-action`, which had been PROVISIONAL. Why it matters: the click-synthesis behaviour this chunk extends to custom widgets is now a ratified rule, not a provisional one.

@history_item n=15 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Document core and [Scrolling and input]: the document's clock) — the double-click window (500 ms and 2 px) is read on the document's clock, and no engine time read stands outside `clock.rs`. Ratified at Epoch 2's close. Why it matters: the double-click synthesis this chunk runs for a canvas target must read the same document clock (`decision/repeated-click-window`, `decision/document-clock`), with no `Instant::now()` in `events/pointer.rs`.

@history_item n=16 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Driver session and CLIs: `advance` on every session, eight causes) — two `click`s on one point with no `advance` of at least 500 ms between them read as a double click, as `click`'s help says (later ratified, `decision/double-click-window`). Why it matters: scope F4 A rests on this. Driver clicks land at the canvas centre only, and a second centre click with no `advance` is a double click that opens the resize dialog. The stand checks drive the flow this way.

@history_item n=17 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (every stand task keyed; the password click caret measured; three new checks) — `unkeyed_actionable()` reads nothing on all seven stand tasks, Circle Drawer's dialog open included. It named Circle Drawer's author ids `circle-undo`, `circle-redo`, `circle-canvas`, `circle-diameter` and `circle-close`, and added `stand_non_lean_tasks`. Why it matters: these are the keys the sample's skeleton must keep when `circle-canvas` moves onto the `<object>`, so `unkeyed_actionable` still reads none. `stand_non_lean_tasks.rs` is a file this chunk edits.

@history_item n=18 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's `clickable` reading) — `clickable` reads the vdom walk's current `click` listeners on no disabled form control. `stand_non_lean_tasks`' `CLICKABLE` table reads Circle Drawer 1, in both layout modes. Why it matters: Circle Drawer's canvas becomes an `<object>` carrying `onclick` and `ondoubleclick`. The `CLICKABLE` row and the snapshot's role for it (now `role="group"`, «Drawing area», F5 A) are readings this chunk re-measures.

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (async handler writes land inside their dispatch) — an async handler's spawned task runs inside the dispatch that started it, so its signal writes are recorded against that event. This was measured on Circle Drawer's canvas writes, `stand_graph`'s first red. Why it matters: Circle Drawer's canvas handlers and their graph write declarations move with the canvas onto the `<object>`. The synthesized click must still dispatch through the bridge so the writes stay confirmed.

@history_item n=20 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (seven_guis registers its state through `use_graph_signal`) — every task's signals register through `tasks::use_graph_signal` with declared writes read off the code, written `const { &[…] }` or as a `const` item, since a bare slice errors `E0716`. Why it matters: Circle Drawer's circle state, now also rendered into `data-circles`, keeps its registered declarations, which must stay true to the handlers on the `<object>`.

@history_item n=21 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (`stand_graph` and the blitz-tests shared modules) — `stand_graph` checks that every stand task's declarations agree with its code once its handlers have run. `GRAPH_DRIVES` in `tests/blitz-tests/tests/common/mod.rs` drives each conditional write. Why it matters: Circle Drawer's canvas writes are driven there by clicks, so if the click on the canvas changes, the drive and its tables in `common/mod.rs` move with it.

@history_item n=22 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage reader handler_coverage and its stated limits) — a stated limit: handlers inside a canvas are not counted, deferred to 0.4.0 (`contract/handler-coverage/canvas`). Why it matters: Circle Drawer's canvas handlers are this version's first canvas handlers. The limit stands, and the coverage tables must not count anything inside the widget.

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stand_handler_coverage, the handler tables and the shared modules' readers) — `stand_handler_coverage` reads `common`'s `HANDLERS_AT_BOOT` and `UNEXERCISED_AFTER_DRIVES`. It opens non-served tasks such as Circle Drawer from Home by their card. Why it matters: the scope names `stand_handler_coverage.rs:58-65` as how checks open Circle Drawer today. The new in-process stand boot of Circle Drawer replaces that route for its own checks while no served list moves (F1 A).

@history_item n=24 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (Standard Contracts: 1 row(s), the stand boot) — `~ contract/headless-stand/boot` gained «one component alone» as a stand boot beside the served tasks. Why it matters: this is the precedent for adding a non-served boot to `seven_guis::stand`. Circle Drawer's in-process stand boot extends the same contract row while `escher-session`'s served tasks stay at seven.

@history_item n=25 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Standard Contracts: 4 row(s), 2 of 2) — `contract/headless-stand/boot` now lists the order page, and `escher-session` invocation and MCP name seven served tasks. Why it matters: the boot row this chunk widens again for Circle Drawer. The served lists and their counts must stay at seven (F1 A, not served until «Cold-agent bar»).

@history_item n=26 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** (Standard Contracts: 2 row(s)) — every stand boot loads the one bundled DejaVu Sans with system fonts off (`decision/stand-typography-dejavu-sans`, guard `stand_bundled_font_families`), and `+ decision/stand-readings-same-on-every-ci-os` was added. Why it matters: Circle Drawer's new stand boot must pin the same font, viewport and scheme. The canvas text comes from that bundled font through `Widget::fonts`, which is what makes a cross-OS canvas digest possible.

@history_item n=27 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the eight engine changes of this chunk) — eight more direct engine changes for every document, each flagged upstreamable with nothing sent, and their merge surface owned by an upstream-sync line. Why it matters: this is the tally (`decision/hit-focus-form-engine-changes`, `decision/direct-engine-changes-flagged-upstreamable`) that the F2 A click synthesis joins as a direct engine change behind `custom-widget`. It needs its upstreamable flag and its merge surface (`events/mod.rs`, `events/pointer.rs`) named.

@history_item n=28 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule and actionable_nodes) — the one actionable rule is: focusable, an interactive role, or current listeners holding an event other than `mounted`. Both `unkeyed_actionable` and `actionable_nodes` read it. Why it matters: the canvas `<object>` with `role="group"` and click listeners is actionable by its listeners, so it must read its author key `circle-canvas` for the skeleton's "every actionable element keyed" claim.

@history_item n=29 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the reserved virtual-node id form) — reserved a fifth id form, `{owner}///{key}`, that no tier holds and no caller produces. Why it matters: circles moved off the DOM get no snapshot node until virtual nodes (v040-11, a boundary of this chunk). This form is where they will be spelled later, and this chunk must produce none.

@history_item n=30 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (Project Intent: 1 row(s)) — the capability statement's verb and cause names and five counts are held to the driver schema, its a11y leg count and stand-repeat named-target count to the CI leg runner, and no check reads its other claims. Why it matters: `docs/capabilities.md` gains the canvas sample and its centre-click stated limit. No pinned count may move, and the limit's wording is read by no check.

@history_item n=31 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** (Established Decisions: 3 row(s), 2 of 3) — last amended `decision/click-focuses-button` except («a select focuses through its own arm, and a list box's row focuses its select») and `decision/hit-focus-form-engine-changes` (the list box added to the tally of direct engine changes). Why it matters: these are the current forms of the click-walk row and the engine-change tally that this chunk's click synthesis on a custom-widget target joins and must not contradict.
