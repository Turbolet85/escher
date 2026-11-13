@word proposal=2026-11-12T03-36-23 of=5becb89730a51d1ea403c3bef643db30095787a835ff4d7428e6ff5cfb9d9569
by: the founder's approval by dialog, 2026-11-12 ≈ 06:14Z (rulings.md, the section of that time: the file rows-fates-2026-11-12T03-36-23.md, md5 cce4e61350d3d5f8e6e2cdfa158abef6, as it stands), on escher-overseer's read of every line against escher's code at fc2d17afd; the cells, the stale fates and the relied fates are escher-overseer's read of the same tree, its new text for his approval

@fate n=1 list=gone master=obs-plan row=decision/runtime-probe-leaks-no-read-value v=keep
because: the probe stands as a test file the search could not see by name; its messages name a mode, a task, a link or an index, and the one id a message names is a literal of the file, never a read value (tests/blitz-tests/tests/dioxus_runtime_probe.rs:146)
@fate n=2 list=gone master=architecture row=decision/cognitive-complexity-ceiling v=keep
because: the row places the reading on the dev host, never in CI: rust-code-analysis-cli 0.0.25 stands in the dev host's cargo bin, off the tree by the row's own words
@fate n=3 list=gone master=architecture row=contract/wpt-harness-codes/1 v=keep
because: harness code 1 is named ERROR and handled by the catch-all arm as a failure; no named constant is needed for the promise (wpt/runner/src/test_runners/harness_test.rs:185)
@fate n=4 list=gone master=test-plan row=decision/mutation-baseline-read-per-unit v=keep
because: a dev-host run recipe whose tracked half stands: cargo-mutants under the nextest mutants profile, fail-fast (the search's «cargo mutants» with a space missed the hyphen) (.config/nextest.toml:1)
@fate n=5 list=gone master=test-plan row=decision/ci-log-ansi-strip v=keep
because: a reading recipe over CI job-log text, held by no tree file by design: the 0x1b hits are a test's byte counts and the WPT runner's progress line
@fate n=6 list=guards master=obs-plan row=decision/checks-silent v=absent
@fate n=7 list=guards master=security-plan row=decision/dropped-anonymous-block-clears-parent v=stands
guard: key_press_after_container_lost_its_box
at: tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
@fate n=8 list=divergences master=obs-plan row=decision/checks-silent v=code
cell: except
value: Checks whose subject is telemetry install escher's sink, the MCP tools check its own recorder, the paint-tree bench reads one opt-in env var, and a re-run child prints ids its parent reads.
@fate n=9 list=divergences master=obs-plan row=decision/verdict-fields-closed v=code
cell: rule
value: Each cold-agent verdict field is a count, a boolean, a closed word, the task, the client's tool names, version and model, or the transcript file's path, never transcript text, a tool argument, a tool answer, a reply or the check.
@fate n=10 list=divergences master=obs-plan row=decision/escher-session-fixed-diagnostics v=holds
because: InitError has one variant whose Display is a fixed string, so the diagnostic prints fixed text and holds no value (packages/escher-telemetry/src/lib.rs:79-90; examples/seven_guis/src/session_host.rs:33)
@fate n=11 list=divergences master=architecture row=decision/apple-system-font-alias v=code
cell: rule
value: `-apple-system` maps to system-ui on Apple targets and `BlinkMacSystemFont` maps to system-ui on macOS.
@fate n=12 list=divergences master=architecture row=decision/lang-and-dir-attributes v=code
cell: except
value: `dir=auto` isolates its content without taking a direction from it.
@fate n=13 list=divergences master=architecture row=decision/clippy-allow-scoping v=code
cell: except
value: `clippy::collapsible_if` is allowed crate-wide in five crates (browser, blitz-dom, blitz-html, blitz-paint, blitz-vibey-script) and `clippy::module_inception` in blitz-dom's node module.
@fate n=14 list=divergences master=architecture row=decision/justified-expect-unwrap v=code
cell: except
value: The browser app's test module allows `clippy::expect_used` with no justification beside it.
@fate n=15 list=divergences master=architecture row=decision/in-repo-crates-no-default-features v=code
cell: rule
value: The in-repo engine crates (blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy, dioxus-native, dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`; the other in-repo crates without it.
@fate n=16 list=divergences master=architecture row=decision/examples-and-apps-unpublished v=code
cell: except
value: Upstream's `rdme` readme app sets `publish = true`.
@fate n=17 list=divergences master=architecture row=decision/module-and-item-docs v=code
cell: except
value: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`.
@fate n=18 list=divergences master=architecture row=decision/unsafe-justified v=code
cell: except
value: Upstream engine code holds unannotated blocks (a node's tree pointer, stylo data, the taffy conversion, layout); escher's own crates hold no `unsafe`.
@fate n=19 list=divergences master=architecture row=decision/escher-telemetry-sink v=code
cell: rule
value: Escher binaries install escher-telemetry at start: one stderr-only sink that stamps the service identity on every line, bridges `log` records and chains the panic hook, with an EnvFilter layer for levels and no target-filter layer (the target drop sits in the formatter).
@fate n=20 list=divergences master=architecture row=decision/local-lint-allows v=code
cell: except
value: `clippy::collapsible_if` crate-wide in browser, blitz-dom, blitz-html, blitz-paint and blitz-vibey-script; `clippy::module_inception` in the node module.
@fate n=21 list=divergences master=architecture row=decision/in-repo-default-features-off v=code
cell: rule
value: The in-repo engine crates (blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy, dioxus-native, dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`; the other in-repo crates without it.
@fate n=22 list=divergences master=architecture row=decision/module-docs v=code
cell: except
value: The driver crate opens every file with `//!`; upstream engine modules largely open without it.
@fate n=23 list=divergences master=architecture row=decision/public-item-docs v=code
cell: except
value: Some upstream public items carry no `///`, a node's tree accessor among them.
@fate n=24 list=divergences master=architecture row=decision/example-docs v=code
cell: except
value: Most upstream examples open with a block comment or none.
@fate n=25 list=divergences master=architecture row=decision/annotated-unsafe v=code
cell: except
value: Upstream engine code holds unannotated blocks (a node's tree pointer, stylo data, the taffy conversion, layout); escher's own crates hold no `unsafe`.
@fate n=26 list=divergences master=architecture row=decision/file-script-load-refused v=code
cell: rule
value: A script load from a `file:` URL is refused unless the loading document is itself a `file:` document.
@fate n=27 list=divergences master=architecture row=decision/headless-documents-sequential-style v=code
cell: rule
value: Sequential style threading stays the default: headless documents of the harness, the stand and the driver never set Parallel; the windowed launch, the browser and the multi-document bench example do.
@fate n=28 list=divergences master=architecture row=decision/ci-scripts-stdlib-python v=code
cell: rule
value: The CI helper scripts, the coverage floor among them, use the Python stdlib and PyYAML, which the ci-scripts leg installs when absent; that leg needs Python 3.11 or later for tomllib.
@fate n=29 list=divergences master=architecture row=decision/one-apt-install-script v=code
cell: except
value: The test matrix installs its one package through a SHA-pinned apt cache action under its step timeout.
@fate n=30 list=divergences master=architecture row=decision/actions-pinned-to-commit-sha v=rule
work: Pin every action of wpt.yml, publish-browser.yml and wpt-post-results.yml to a 40-hex commit SHA, its ref a trailing comment, and widen the pinning check past ci.yml to every workflow.
@fate n=31 list=divergences master=architecture row=decision/answers-written-by-own-json-writer v=code
cell: rule
value: Every answer the driver gives has one written form, a line of JSON keyed by the schema's own words and written by the driver's own output-only writer; no serialization crate writes one, and serde_json only reads MCP requests and kept checks.
@fate n=32 list=divergences master=architecture row=decision/one-ci-leg-runner v=code
cell: except
value: The cross-platform test matrix, its linux android entry included, runs cargo or cross directly.
@fate n=33 list=divergences master=test-plan row=decision/no-wall-time-in-session-checks v=code
cell: except
value: The checks whose subject is wall time: the virtual-clock check's 800 ms sleep, the load-wait check's clock reads and stand-in sleep, and the lifecycle check's wait on the host's idle expiry.
@fate n=34 list=divergences master=test-plan row=decision/stand-timer-ticks v=code
cell: rule
value: Every headless-stand boot of a lean task provides TimerTicks in place of the timer's real delay.
@fate n=35 list=divergences master=test-plan row=decision/package-installs-bounded-retried v=code
cell: except
value: The test matrix's one package install, through a SHA-pinned apt cache action, is bounded by its step timeout and not retried.
@fate n=36 list=divergences master=test-plan row=decision/no-sleep-or-wall-clock-in-checks v=code
cell: except
value: stand_virtual_clock's 800 ms sleep, stand_load_wait's clock reads and 200 ms stand-in sleep, stand_session_lifecycle's idle-expiry wait and stand_contrast's bound on a load's answer.
@fate n=37 list=divergences master=test-plan row=decision/cli-and-flow-targets-unix-gated v=code
cell: rule
value: The command-line check target and the shell flow target are gated to unix; the recording target is gated to unix item by item, beside an off-unix check that runs elsewhere.
@fate n=38 list=divergences master=security-plan row=decision/ci-actions-pinned-to-sha v=rule
work: Pin every action of wpt.yml, publish-browser.yml and wpt-post-results.yml to a 40-hex commit SHA, its ref a trailing comment, and widen the pinning check past ci.yml to every workflow.
@fate n=39 list=divergences master=security-plan row=decision/driver-outcomes-never-logged v=code
cell: rule
value: The driver crate logs and fields no command, call, argument, outcome, refusal, screen text, image, finding, session label or id record; an outcome is written only as a command-line answer, an MCP tool result, a session-socket reply or a recording file the caller asked for.
@fate n=40 list=divergences master=security-plan row=decision/driver-installs-no-subscriber v=code
cell: rule
value: The driver crate and its command line install no tracing subscriber and read no environment variable; clocks are read only by the host's idle expiry, the MCP server on a request, the session's bounded load wait, and the client's bounded stop and ready waits.
@fate n=41 list=divergences master=security-plan row=decision/select-list-engine-colours v=code
cell: rule
value: The open select list's border, highlight and disabled-text colours are the engine's own constants; its background and an ordinary row's text colour are the select's own page styles.
@fate n=42 list=stale master=architecture row=contract/select-list/list-machinery v=code
cell: promise
value: Beyond its public readers and closer, the list's view and row types and the view's colour and border constants are public; every other part of the list's machinery is crate-private or private.
@fate n=43 list=stale master=architecture row=contract/example-clis/url v=code
cell: promise
value: The example hands its first argument, or https://www.google.com, to blitz::launch_url, which parses it strictly and panics on an unparsable URL: no https:// retry and no file read from disk.
@fate n=44 list=stale master=test-plan row=contract/agent-run-selections/all v=code
cell: promise
value: Runs every reader of the blitz-tests crate, the stand readers included; there is no `run fixtures` selection.
@fate n=45 list=stale master=security-plan row=absence/monitoring-and-alerts v=code
cell: capability
value: Monitoring beyond the stderr log: no metrics, no alerting and no telemetry export
@fate n=46 list=stale master=security-plan row=contract/escher-session-output/caller-text v=code
cell: promise
value: No output line echoes a caller's argument but the admitted request id the mcp role echoes; an answer carries each node's current value, so text typed into an unmasked control returns, a masked input reading only the mask.
@fate n=47 list=relied master=layout-templates rec=fct-bla7ziiz@1 v=keep as=decision/flight-type-select-boot-reading
name: Flight type select reads its two options at boot
rule: The snapshot reads the flight booker's mode select at boot as ComboBox "Flight type" with value one-way and two ListBoxOptions, each at the select's client rect.
why: the_flight_type_select_holds_its_two_options in stand_snapshot pins role, name, value, both ListBoxOptions and their select-rect bounds in both layout modes
at: tests/blitz-tests/tests/stand_snapshot.rs:122
@fate n=48 list=relied master=layout-templates rec=fct-zk3a7kfe@1 v=keep as=decision/open-list-rows-own-bounds
name: Open list rows read their own bounds
rule: While a select list is open each row reads its own bounds in the snapshot and the diff, and after a close the rows read back at the select's client rect.
why: stand_act_popup checks a click opening the list names each row's bounds in the diff, and escape closing it puts the rows back at the select's box
at: tests/blitz-tests/tests/stand_act_popup.rs:141
@fate n=49 list=relied master=layout-templates rec=fct-3yebjmlo@1 v=keep as=decision/fixed-document-skeleton-main-mount
name: Fixed document skeleton with main mount
rule: Every document starts with the fixed skeleton html, head, body and main id="main", and the app mounts into main.
why: stand_snapshot's tree_follows_the_task_shell reads the single /html:0 root with nothing under head, and the bridge's own crash test counts the app's children under main
at: packages/dioxus-native-dom/src/dioxus_document.rs:148
@fate n=50 list=relied master=obs-plan rec=fct-w6etu55s@1 v=cut
@fate n=51 list=relied master=obs-plan rec=fct-ip3ek2mr@1 v=cut
@fate n=52 list=relied master=obs-plan rec=fct-6jm2c55w@1 v=keep as=decision/failed-stylesheet-leaves-pending-set
name: Failed stylesheet leaves the pending critical set
rule: A failed stylesheet fetch drops its request from the pending critical resources, so loading does not wait on it.
why: render_blocking_stylesheet's failed_stylesheet_unblocks_rendering pins it, and the harness settle and paint_scene both read the pending critical resources
at: packages/blitz-dom/src/document.rs:1822
@fate n=53 list=relied master=obs-plan rec=fct-lrj2spd7@1 v=cut
@fate n=54 list=relied master=obs-plan rec=fct-7au5a6cr@1 v=cut
@fate n=55 list=relied master=obs-plan rec=fct-omr4vkvp@1 v=cut
@fate n=56 list=relied master=obs-plan rec=fct-x7v6upkc@1 v=cut
@fate n=57 list=relied master=obs-plan rec=fct-7tp6qj2l@1 v=keep as=decision/actions-read-changes-nothing
name: Actions read changes nothing in the document
rule: The actions read acts on nothing and changes nothing in the document.
why: stand_actions' the_read_changes_nothing_and_holds_no_typed_text_or_mask pins no change mark, unchanged snapshot, focus and empty diff
at: tests/blitz-tests/tests/stand_actions.rs:487
@fate n=58 list=relied master=obs-plan rec=fct-v53ms3q6@1 v=keep as=decision/no-state-page-skips-registry
name: Page without registered state skips the registry
rule: A page with no registered state reaches no signal registry: its events run their handlers and no dispatch is recorded.
why: signal_registry's a_page_with_no_registered_state_has_no_registry pins no registry context and a click that still acts; the load page's handler-coverage check relies on it
at: packages/dioxus-native-dom/src/dioxus_document.rs:461
@fate n=59 list=relied master=obs-plan rec=fct-5rpr77we@1 v=keep as=decision/reached-record-ids-and-events-only
name: Reached record holds ids and event names only
rule: The reached record holds stable ids and event names only, no node number, and its debug form prints its count alone.
why: the reached module's tests pin id and event rows and the_records_debug_prints_its_count_alone; handler coverage reads the record through reached_handlers
at: packages/dioxus-native-dom/src/reached.rs:35
@fate n=60 list=relied master=obs-plan rec=fct-ovjvwkua@1 v=cut
@fate n=61 list=relied master=obs-plan rec=fct-ukvug6pr@1 v=cut
@fate n=62 list=relied master=obs-plan rec=fct-jyfscbc7@2 v=keep as=decision/hosted-call-through-session-run
name: Hosted call runs through the session's run alone
rule: A hosted part read runs through the session's run alone, so its spans are command spans recording verb alone.
why: stand_act_spans pins a part read's command line carrying its verb alone, and the session host answers each call through Session::run with no span of its own
at: packages/escher-driver/src/host.rs:199
@fate n=63 list=relied master=obs-plan rec=fct-46rgllvo@1 v=cut
@fate n=64 list=relied master=design-system rec=fct-blf3mieo@1 v=cut
@fate n=65 list=relied master=design-system rec=fct-g6x7bhkj@1 v=keep as=decision/stacking-context-groups-paint-and-hit
name: Stacking-context root groups paint and hit-testing
rule: Paint order and hit-testing both group a subtree under the node that the stacking-context rules make a root.
why: pinned by hoisted_children_of_scrolled_sc_root_are_hit and paint_tree_z_index_and_stacking_contexts; blitz-paint's render walk reads the same hoisted list
at: packages/blitz-dom/src/node/node.rs:1510
@fate n=66 list=relied master=design-system rec=fct-z3qfoykp@1 v=keep as=decision/open-select-list-hit-first
name: Open select list answers hits first
rule: hit, set_hover_to, handle_pointerdown and the driver's covered all read the open select list first, so a button under the list is refused covered.
why: the driver's covered check reads it; pinned by a_button_under_the_list_is_refused_covered and the covered fixture in stand_actions
at: packages/blitz-dom/src/document.rs:2436
@fate n=67 list=relied master=design-system rec=fct-cbe7vaww@1 v=keep as=decision/select-list-painted-after-root
name: Select list painted after the root render
rule: The open select list paints after the root's render and the blend layers, so it stands above every box of the page.
why: pinned by a_list_paints_whole_past_an_opacity_ancestor_and_a_box_that_clips_its_select; driver screenshots of an open list read it
at: packages/blitz-paint/src/render.rs:248
@fate n=68 list=relied master=design-system rec=fct-keknjqam@1 v=keep as=decision/favicon-from-first-icon-link
name: Favicon read from the first icon link
rule: The favicon read returns the href of the first link element whose rel contains icon.
why: the browser app's document loader takes favicon_url as its favicon candidate, which the tab strip and history show
at: packages/blitz-dom/src/document.rs:649
@fate n=69 list=relied master=design-system rec=fct-wr73fuj5@1 v=keep as=decision/smooth-scroll-behavior-and-wheel-cancel
name: scroll-behavior smooth animates, wheel cancels
rule: scroll-behavior smooth makes Auto scrolls animate, and a wheel over a scroller cancels an in-progress smooth scroll.
why: pinned by fragment_link_uses_smooth_scroll_behavior_from_root_style, fragment_link_uses_auto_scroll_behavior_from_root_style and wheel_scroll_cancels_smooth_scroll
at: packages/blitz-dom/src/scrolling.rs:530
@fate n=70 list=relied master=design-system rec=fct-zbk2czd3@1 v=keep as=decision/select-list-wheel-and-key-scroll
name: Wheel and key scroll the open select list
rule: A wheel over the open select list scrolls the list, a wheel elsewhere closes it first, and a key brings the current row into view at once.
why: pinned by a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel in the select_popup tests
at: packages/blitz-dom/src/events/pointer.rs:1194
@fate n=71 list=relied master=design-system rec=fct-dxtawno3@1 v=keep as=decision/shown-scrollbar-takes-thumb-drag
name: Shown overlay scrollbar takes a pointerdown as drag
rule: Right after a scroll the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag, so a driver click there needs an advance past the fade first.
why: pinned by dragging_the_thumb_scrolls_the_container and faded_out_thumb_does_not_capture_drags; driver clicks after a scroll advance past the fade
at: packages/blitz-dom/src/events/pointer.rs:431
@fate n=72 list=relied master=design-system rec=fct-qpwq4lud@1 v=cut
@fate n=73 list=relied master=design-system rec=fct-5pxpwwyq@1 v=keep as=decision/select-list-opens-without-motion
name: Select list opens and closes with no motion
rule: The select list has no motion, so a screen read after a step reads the new state without an advance.
why: pinned by a_screen_read_after_a_step_reads_the_new_state_with_no_advance in the stand_act_popup driver tests
at: packages/blitz-dom/src/document.rs:840
@fate n=74 list=relied master=design-system rec=fct-335wqsvp@1 v=cut
@fate n=75 list=relied master=design-system rec=fct-lwnjcsh2@1 v=keep as=decision/select-list-placed-in-viewport
name: Open select list placed inside the viewport
rule: An open select list is placed inside the viewport, at least its select's width and at most the viewport's, every edge finite.
why: pinned by every_edge_of_a_placed_list_is_finite and hostile_markup_keeps_every_box_finite_and_panics_nowhere
at: packages/blitz-dom/src/node/select_list.rs:253
@fate n=76 list=relied master=design-system rec=fct-z4dmblpa@1 v=keep as=decision/default-ua-sheet-via-default-config
name: Default user-agent sheet through the default config
rule: Every document gets the blitz default user-agent stylesheet once, through the default document config.
why: pinned by default_config_uses_the_default_sheet; the test harness and Dioxus documents take the sheet only through DocumentConfig default
at: packages/blitz-dom/src/config.rs:79
@fate n=77 list=relied master=design-system rec=fct-aboeejrv@1 v=keep as=decision/single-select-box-shows-chosen-label
name: Single select box shows its chosen option
rule: A single-choice select lays out as a box holding its chosen option's label, from engine defaults with no design token.
why: pinned by a_select_lays_out_as_a_box_holding_its_chosen_option in the select_interaction tests
at: packages/blitz-dom/assets/default.css:157
@fate n=78 list=relied master=design-system rec=fct-epbt6wng@1 v=keep as=decision/single-select-open-gestures
name: Click, Enter or Space opens a single select
rule: A single select's list opens on a click, Enter, or a Space with no typed text standing.
why: pinned by a_click_enter_and_space_open_the_list_on_its_chosen_option_and_fire_nothing and enter_and_space_open_the_list_and_a_released_space_does_nothing_more
at: packages/blitz-dom/src/events/keyboard.rs:195
@fate n=79 list=relied master=design-system rec=fct-ypgr7m5t@1 v=keep as=decision/selects-that-open-no-list
name: Selects that open no list
rule: A disabled, inert or hidden select, a select with no row to show and a multiple select open no list.
why: pinned by a_disabled_inert_hidden_singular_or_empty_select_opens_nothing in the select_popup tests
at: packages/blitz-dom/src/document.rs:799
@fate n=80 list=relied master=design-system rec=fct-ih466day@1 v=keep as=decision/boxless-focused-select-keeps-focus
name: Focused select with no box keeps focus and keys
rule: A focused select whose container lost its box keeps focus and takes a key with no panic.
why: pinned by a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes, which presses a key after hiding the select's container
at: packages/blitz-dom/src/mutator.rs:1128
@fate n=81 list=relied master=design-system rec=fct-lyaafumo@1 v=keep as=decision/logical-viewport-from-scale
name: Logical viewport is physical size over total scale
rule: The logical viewport size is the physical window size divided by hidpi scale times zoom.
why: the stylo device is built from logical_size; pinned by resize_and_zoom_coalesce in the device_coalescing tests
at: packages/blitz-traits/src/shell.rs:121
@fate n=82 list=relied master=design-system rec=fct-qmikpmeb@1 v=cut
@fate n=83 list=relied master=design-system rec=fct-xmoyplif@1 v=cut
@fate n=84 list=relied master=design-system rec=fct-plskesan@1 v=cut
@fate n=85 list=relied master=design-system rec=fct-mrtzanbs@1 v=keep as=decision/no-generic-family-lays-out-sans
name: No generic family lays out as sans-serif
rule: An element with no generic font family is laid out as sans-serif.
why: can_paint_text, behind the driver's screenshot refusal, reads only the sans-serif generic because unstyled text lays out with it
at: packages/blitz-dom/src/stylo_to_parley.rs:52
@fate n=86 list=relied master=design-system rec=fct-va3exjt4@1 v=cut
@fate n=87 list=relied master=design-system rec=fct-ua34srkn@1 v=keep as=decision/lang-attribute-sets-text-locale
name: lang attribute sets the text locale
rule: The `lang` attribute decides the locale text is shaped and broken with.
why: pinned by the lang_attribute tests, among them the_language_is_inherited_from_the_root; text layout reads the inherited -x-lang as its locale
at: packages/blitz-dom/src/stylo_to_parley.rs:495
@fate n=88 list=relied master=design-system rec=fct-etlx3awr@1 v=cut
@fate n=89 list=relied master=a11y-plan rec=fct-xhpyuh5z@1 v=cut
@fate n=90 list=relied master=a11y-plan rec=fct-sidyoogt@1 v=keep as=decision/role-attribute-then-element-then-unknown
name: Role from role attribute, element, else Unknown
rule: A node's role is taken from its role attribute, else from the HTML element mapping, else Unknown.
why: snapshot role readings and the stand checks controls_carry_role_and_name and aria_checked_reads_its_three_values_on_a_checkable_role_alone read roles set this way
at: packages/blitz-dom/src/accessibility.rs:151
@fate n=91 list=relied master=a11y-plan rec=fct-2bz7tofn@1 v=cut
@fate n=92 list=relied master=a11y-plan rec=fct-7hugl7cl@1 v=cut
@fate n=93 list=relied master=a11y-plan rec=fct-dzuhkwn2@1 v=keep as=decision/single-select-engine-overlay-list
name: Single select opens an engine overlay list
rule: A closed single select opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root.
why: driver click on a select opens it; pinned by a_click_opens_the_list_and_a_click_on_a_row_chooses_it and a_button_under_the_list_is_refused_covered
at: packages/blitz-dom/src/events/pointer.rs:956
@fate n=94 list=relied master=a11y-plan rec=fct-pqyrk5rl@1 v=keep as=decision/tab-and-shift-tab-move-focus
name: Tab and Shift+Tab move focus with focus events
rule: Tab moves focus to the next focusable node and Shift+Tab to the previous one, dispatching focus events.
why: pinned by tab_and_shift_tab_walk_the_stated_sequence_and_wrap and tab_and_shift_tab_skip_an_inert_subtree; the driver's press tab relies on it
at: packages/blitz-dom/src/events/keyboard.rs:35
@fate n=95 list=relied master=a11y-plan rec=fct-ag43xb7g@1 v=keep as=decision/tab-chooses-and-closes-select-list
name: Tab chooses the row and closes the list first
rule: With a select's list open, Tab chooses the current row and closes the list before moving focus, in either direction.
why: pinned by tab_chooses_and_closes_before_focus_moves_either_way in the select_popup tests
at: packages/blitz-dom/src/events/keyboard.rs:32
@fate n=96 list=relied master=a11y-plan rec=fct-yxl65pxf@1 v=keep as=decision/focus-stays-on-select-with-list-open
name: Focus stays on a select while its list is open
rule: Focus stays on a select while its list is open, and the Tab walk holds with the list open and after each close.
why: pinned by the_tree_reads_the_list_open_on_the_select_its_focus_staying_there and escape_closes_with_the_value_unchanged_and_the_rows_back_at_the_selects_box
at: packages/blitz-dom/src/events/pointer.rs:953
@fate n=97 list=relied master=a11y-plan rec=fct-is66hfzy@1 v=keep as=decision/focusability-recomputed-on-attribute-write
name: Focusability recomputed on tabindex, href, disabled
rule: Cached focusability is recomputed when tabindex, href or disabled is set or removed, so a roving tabindex works.
why: pinned by a_tabindex_set_after_creation_takes_effect in the focusability_updates tests; roving-tabindex widgets rely on it
at: packages/blitz-dom/src/mutator.rs:338
@fate n=98 list=relied master=a11y-plan rec=fct-jc7cdake@1 v=keep as=decision/focus-visible-never-matches
name: :focus-visible never matches, :focus does
rule: The :focus-visible selector never matches, while :focus and :focus-within do.
why: the stand task stylesheets (flight booker, temperature converter, cells) draw their focus cue with :focus rules, which match where :focus-visible would not
at: packages/blitz-dom/src/stylo.rs:495
@fate n=99 list=relied master=a11y-plan rec=fct-3rgd2unt@1 v=keep as=decision/removed-focused-node-drops-focus
name: Removing the focused node drops focus with blur steps
rule: Removing the focused node resets focus to the body and runs blur side effects.
why: pinned by ime_is_disabled_when_focused_text_input_is_removed and removing_the_subtree_containing_the_focus_clears_the_ancestors
at: packages/blitz-dom/src/document.rs:1396
@fate n=100 list=relied master=a11y-plan rec=fct-7uphfywk@1 v=keep as=decision/checkable-click-toggles-and-focuses
name: Checkbox or radio click toggles, fires input, focuses
rule: Clicking a checkbox or radio toggles it, dispatches input and moves focus to the control.
why: pinned by a_click_and_a_space_on_a_checkbox_each_fire_input_then_change; the stand's checkable controls and the driver's click rely on it
at: packages/blitz-dom/src/events/pointer.rs:863
@fate n=101 list=relied master=a11y-plan rec=fct-o3zz2arg@1 v=keep as=decision/one-engine-change-per-checkable-click
name: One engine change per checkable click
rule: A page's script hears exactly one change per checkable click, from the engine, since the script runtime synthesizes none.
why: pinned by checkbox_click_fires_input_and_change_events in the vibey-script dom tests and by the checkable_change_event tests
at: packages/blitz-dom/src/events/pointer.rs:869
@fate n=102 list=relied master=a11y-plan rec=fct-y4ncelk5@1 v=keep as=decision/select-key-change-fires-input-change
name: Select key change fires one input then one change
rule: Each key change on a single select dispatches one input then one change carrying the select's value, a key that changes nothing firing none.
why: pinned by the_arrows_home_and_end_choose_an_enabled_option, which counts input and change per key, none where nothing changes
at: packages/blitz-dom/src/events/keyboard.rs:480
@fate n=103 list=relied master=a11y-plan rec=fct-k6bfqrau@1 v=keep as=decision/select-list-choice-one-write-path
name: List choices share one write path
rule: Every choice from an open list goes through one write path that fires one input then one change, and choosing the already chosen option fires nothing.
why: pinned by enter_space_tab_and_a_click_choose_the_row_one_input_then_one_change and a_choice_of_the_chosen_option_fires_nothing
at: packages/blitz-dom/src/events/keyboard.rs:485
@fate n=104 list=relied master=a11y-plan rec=fct-lvplfrnk@1 v=keep as=decision/disabled-optgroup-one-rule
name: Disabled optgroup rows read disabled by one rule
rule: A disabled optgroup's rows read as disabled through one rule that the readings and the form's entries share.
why: form entry building, the list's row readings and key choice all call option_disabled; pinned by a_multiple_selects_groups_lay_out_and_a_disabled_groups_rows_are_not_chosen
at: packages/blitz-dom/src/node/select.rs:177
@fate n=105 list=relied master=a11y-plan rec=fct-6opcw6lo@1 v=keep as=decision/driver-type-focuses-select-no-click
name: Driver type focuses a select with no click
rule: The driver's type on a select focuses it with no click and adds nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors or exits.
why: pinned by type_leaves_no_list_open_and_chooses_by_its_text in the stand_act_popup driver tests
at: packages/escher-driver/src/execute.rs:240
@fate n=106 list=relied master=a11y-plan rec=fct-gp3teo4p@1 v=keep as=decision/label-click-focuses-select-only
name: Label click focuses its select and opens nothing
rule: A click on a label bound to a single select focuses the select, opens nothing and re-dispatches nothing to it.
why: pinned by a_labels_click_focuses_the_select_and_opens_nothing in the select_popup tests
at: packages/blitz-dom/src/events/pointer.rs:991
@fate n=107 list=relied master=a11y-plan rec=fct-55wog4z5@1 v=keep as=decision/synthetic-click-at-node-centre
name: Synthetic click is a primary click at the centre
rule: A synthetic click is a primary mouse click at the node's center.
why: Enter and Space activation, label click forwarding and the Dioxus event bridge build their synthetic Click from it
at: packages/blitz-dom/src/node/node.rs:2033
@fate n=108 list=relied master=a11y-plan rec=fct-v3jtrgvp@1 v=keep as=decision/script-style-through-inline-attribute
name: Script element.style goes through the style attribute
rule: A script's element.style read or write goes through the inline style, so display and visibility reach the tree's hidden-content exclusion.
why: pinned by style_bindings in the vibey-script dom tests; hidden-content exclusion reads the computed display and visibility written there
at: packages/blitz-vibey-script/src/dom/style.rs:101
@fate n=109 list=relied master=a11y-plan rec=fct-tc36t6wc@1 v=cut
@fate n=110 list=relied master=a11y-plan rec=fct-jn5hbj7a@1 v=keep as=decision/text-runs-label-their-parent
name: Text runs label their parent element
rule: Text nodes become TextRun nodes carrying their text, and their parent is labelled by them, so an element's text reaches its accessible name.
why: accessible names of buttons and controls come from it; pinned by every_interactive_role_element_reads_a_name and role_and_name_match_the_accessibility_tree
at: packages/blitz-dom/src/accessibility.rs:270
@fate n=111 list=relied master=a11y-plan rec=fct-5vvi2itr@1 v=keep as=decision/accesskit-events-to-one-handler
name: AccessKit requests reach one event handler
rule: Callers receive no initial tree synchronously; initial-tree requests, action requests and deactivation all reach one EventHandler.
why: blitz-shell's accessibility handler and its application InitialTreeRequested arm build and send the first tree through it
at: packages/accesskit_xplat/src/lib.rs:155
@fate n=112 list=relied master=a11y-plan rec=fct-fod53y3h@1 v=keep as=decision/ime-enabled-for-prefocused-input
name: IME enabled for an input focused before its editor
rule: A text input already focused when its text editor is constructed has IME enabled with no fresh focus, in every document.
why: pinned by input_focused_on_mount_enables_ime; TodoMVC's onmounted focus needs it so macOS key bindings arrive
at: packages/blitz-dom/src/layout/construct.rs:888
@fate n=113 list=relied master=a11y-plan rec=fct-ajar2qgv@1 v=keep as=decision/open-pseudo-class-on-details-dialog
name: :open matches an open details or dialog
rule: Author styles keyed on `:open` apply to an open details or dialog, `:open` reading the element's open state.
why: pinned by the open_state tests, among them at_construction_open_is_read_from_the_attribute and a_summary_click_flips_its_details
at: packages/blitz-dom/src/node/element.rs:544
@fate n=114 list=relied master=a11y-plan rec=fct-jajzvsxm@1 v=keep as=decision/open-select-list-not-open-state
name: Open select list sets no :open state
rule: An open select's list is no `:open` state, so no author style keys on a list standing open.
why: pinned by an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match in the select_popup tests
at: packages/blitz-dom/src/node/element.rs:546
@fate n=115 list=relied master=a11y-plan rec=fct-o4ijraeq@1 v=keep as=decision/viewport-scheme-and-zoom-level
name: Viewport carries scheme and zoom, 1.0 unzoomed
rule: The viewport carries a light/dark color scheme and a document zoom level, 1.0 unzoomed.
why: the stylo device reads the scheme for prefers-color-scheme and the scale from zoom; the shell and harness configs set both
at: packages/blitz-traits/src/shell.rs:89
@fate n=116 list=relied master=a11y-plan rec=fct-zd6d5frj@1 v=keep as=decision/scheme-change-is-device-change
name: Scheme change counts as a device change
rule: A color-scheme change on the viewport is a device change that rebuilds the stylist device.
why: pinned by color_scheme_change_recascades_styles in the device_coalescing tests
at: packages/blitz-dom/src/stylo_device.rs:55
@fate n=117 list=relied master=a11y-plan rec=fct-dwhu55j2@1 v=keep as=decision/hidpi-or-zoom-change-reshapes-text
name: Hidpi or zoom change reshapes text
rule: A total scale change from hidpi or zoom invalidates text shaping.
why: the shell's scale-factor change and the browser zoom keys write hidpi scale and zoom, and rely on text being reshaped at the new scale
at: packages/blitz-dom/src/document.rs:2709
@fate n=118 list=relied master=a11y-plan rec=fct-anrtdftj@1 v=keep as=decision/driver-click-in-shown-scrollbar-drags
name: Driver click in a shown scrollbar band drags
rule: Right after a scroll, a driver click in the shown overlay scrollbar's band is taken as a thumb drag, so an advance past the fade is needed first.
why: pinned by dragging_the_thumb_scrolls_the_container and faded_out_thumb_does_not_capture_drags; driver clicks after a scroll advance past the fade
at: packages/blitz-dom/src/events/pointer.rs:431
@fate n=119 list=relied master=architecture rec=fct-j5k4uy7t@1 v=cut
@fate n=120 list=relied master=architecture rec=fct-54e4fuk7@1 v=keep as=decision/shell-polls-after-window-event
name: Shell polls its window after each window event
rule: Every shell window event is followed by a Poll.
why: DioxusNativeApplication forwards window events to the shell and runs the vdom work handlers scheduled in the Poll that follows (CloseRequested excepted)
at: packages/blitz-shell/src/application.rs:165
@fate n=121 list=relied master=architecture rec=fct-aj6cmiwc@1 v=keep as=decision/net-launch-enters-tokio-runtime
name: Net launch enters a tokio multi-thread runtime
rule: Launch paths built with `net` off wasm32 enter a tokio multi-thread runtime before the event loop runs.
why: blitz-net's Provider spawns every fetch with tokio::spawn, so dioxus-native and blitz launches with net need the entered runtime for loads to run
at: packages/dioxus-native/src/lib.rs:159
@fate n=122 list=relied master=architecture rec=fct-35cb7qc2@1 v=keep as=decision/js-entry-redraws-and-arms-timer
name: JS entry points request a redraw and arm the timer thread
rule: Each JS entry point requests a redraw and re-arms the timer thread that wakes the event loop at the next JS timer deadline.
why: The windowed shell repaints after script work and wakes for due JS timers only through this; the WPT runner opts out with without_timer_thread
at: packages/blitz-vibey-script/src/document.rs:419
@fate n=123 list=relied master=architecture rec=fct-2r7bauw3@1 v=cut
@fate n=124 list=relied master=architecture rec=fct-dajqtqwx@1 v=keep as=decision/wpt-test-reset-and-panic-catch
name: WPT runner resets the net provider and catches test panics
rule: The WPT runner resets the net provider before each test and runs each test inside a panic catch, a panic yielding Unknown or Crash.
why: The WPT run's per-test isolation and its CRASH count in the summary and wpt report depend on the reset and the caught panic mapped to Unknown kind and Crash status
at: wpt/runner/src/main.rs:629
@fate n=125 list=relied master=architecture rec=fct-z4p5ffso@1 v=keep as=decision/browser-load-aborts-previous
name: A browser document load aborts the previous load
rule: Each browser document load aborts the previous one, as reload and drop do.
why: The browser's tab navigation calls load_document and the toolbar reload calls reload, so a superseded page's fetches stop; DocumentLoader's Drop aborts too
at: apps/browser/src/document_loader.rs:109
@fate n=126 list=relied master=architecture rec=fct-6decztyx@1 v=keep as=decision/legacy-attributes-as-pres-hints
name: Legacy attributes cascade at the presentational-hints level
rule: Legacy presentational attributes map to CSS at the presentational-hints level.
why: Stylo's cascade calls the hook, and replaced-element layout reads width and height as already in the style because the hints carry them
at: packages/blitz-dom/src/stylo.rs:901
@fate n=127 list=relied master=architecture rec=fct-2es3quc6@1 v=keep as=decision/node-send-sync-by-unsafe-impl
name: Node marked Send and Sync over its raw tree pointer
rule: A `Node` holds a raw pointer to its `NodeTree` and is marked `Send` and `Sync` by unsafe impl.
why: Stylo's parallel traversal, which the windowed documents' Parallel style threading uses, needs nodes shared across threads
at: packages/blitz-dom/src/node/node.rs:142
@fate n=128 list=relied master=architecture rec=fct-kknd7ewa@1 v=keep as=decision/wpt-failures-bucketed-by-feature
name: WPT failures bucketed by regex feature flags
rule: WPT failures are bucketed by feature-usage flags computed by regex over test sources.
why: The WPT runner's end-of-run summary splits failures into per-feature counts (grid lanes, subgrid, writing-mode, direction, intrinsic size, calc, float, script)
at: wpt/runner/src/main.rs:653
@fate n=129 list=relied master=architecture rec=fct-4mapcv7w@1 v=keep as=decision/ime-enabled-at-editor-construction
name: Focused text input enables IME when its editor is built
rule: A text input that holds focus before its editor exists gets the shell provider's IME enabled when its editor is constructed.
why: Pinned by the bridge test input_focused_on_mount_enables_ime; TodoMVC-style onmounted focus needs IME on for macOS key bindings such as Backspace
at: packages/blitz-dom/src/layout/construct.rs:888
@fate n=130 list=relied master=architecture rec=fct-biqhin7y@1 v=keep as=decision/first-graph-state-hook-provides-registry
name: First graph-state hook provides the root signal registry
rule: The first graph-state hook of a page provides a `SignalRegistry` context at the root scope, which the bridge's dispatch looks up; a page that registers no state has none.
why: The bridge's dispatch and the UI graph look the registry up at the root scope; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is
at: packages/dioxus-native-dom/src/signal_registry.rs:58
@fate n=131 list=relied master=architecture rec=fct-tzrr5o2d@1 v=cut
@fate n=132 list=relied master=architecture rec=fct-3kximnte@1 v=cut
@fate n=133 list=relied master=architecture rec=fct-56u6r4qr@1 v=keep as=decision/signal-write-recorded-against-dispatch
name: Signal writes recorded against their causing dispatch
rule: Where a signal registry stands, each signal write is recorded against the dispatch that caused it, and a page with no registered state dispatches unchanged.
why: The UI graph reads each write record's dispatch to confirm declared writes; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is
at: packages/dioxus-native-dom/src/signal_registry.rs:208
@fate n=134 list=relied master=architecture rec=fct-xcqj3bw6@1 v=keep as=decision/select-list-closes-without-event
name: Open select list closes with no event
rule: An open select list closes with no event on Escape, a second click, a press or wheel outside, a scroll or move of the select's box, and on removal, disabled, hidden or inert.
why: Pinned by escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event and a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes
at: packages/blitz-dom/src/events/keyboard.rs:232
@fate n=135 list=relied master=architecture rec=fct-t7ezjz4v@1 v=keep as=decision/select-list-tab-chooses-and-closes
name: Tab on an open select list chooses and closes first
rule: On an open select list Tab chooses the current row and closes before focus moves, and the move keys shift the highlight without choosing.
why: Pinned by tab_chooses_and_closes_before_focus_moves_either_way and the_keys_move_the_highlight_over_enabled_rows_and_choose_nothing
at: packages/blitz-dom/src/events/keyboard.rs:33
@fate n=136 list=relied master=architecture rec=fct-hxuv6j35@1 v=keep as=decision/multiple-select-row-keys
name: Multiple select keys choose alone, span or move
rule: A multiple select's arrow, Home and End keys choose one row alone, with Shift spanning from the anchor and Ctrl or Meta moving the current row without changing the choice.
why: Pinned by the_keys_of_a_multiple_select_choose_span_and_flip_its_rows and the unit test a_gesture_on_a_row_leaves_the_set_it_states
at: packages/blitz-dom/src/events/keyboard.rs:320
@fate n=137 list=relied master=architecture rec=fct-jyueaf6l@1 v=keep as=decision/multiple-select-row-clicks
name: Multiple select row clicks choose, flip or span
rule: A plain click on a multiple select's row chooses it alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing.
why: Pinned by a_click_on_a_row_chooses_flips_or_spans_it and the driver test a_press_with_shift_spans_the_rows
at: packages/blitz-dom/src/events/pointer.rs:1151
@fate n=138 list=relied master=architecture rec=fct-75lrk5x7@1 v=cut
@fate n=139 list=relied master=architecture rec=fct-3oxpzjue@1 v=keep as=decision/pseudo-elements-vote-in-flow-classification
name: Before and after pseudo-elements vote in flow classification
rule: Every box-generating ::before or ::after votes in flow classification, so a pseudo-only inline-block exports its line baseline.
why: Pinned by inline_block_pseudo_only_child_exports_line_baseline and grid_row_grows_by_baseline_shim_of_pseudo_only_items
at: packages/blitz-dom/src/layout/construct.rs:320
@fate n=140 list=relied master=architecture rec=fct-kvuycvyd@1 v=cut
@fate n=141 list=relied master=architecture rec=fct-jzkmudp4@1 v=cut
@fate n=142 list=relied master=architecture rec=fct-qcw5degj@1 v=keep as=decision/optgroup-group-role-named-by-label
name: Optgroup reads the group role named by its label
rule: The accessibility tree gives an optgroup the group role named by its non-empty label.
why: The driver's snapshot reads it; pinned by a_multiple_select_reads_its_name_role_and_rows_each_with_its_own_bounds
at: packages/blitz-dom/src/accessibility.rs:521
@fate n=143 list=relied master=architecture rec=fct-wau2zivr@1 v=cut
@fate n=144 list=relied master=architecture rec=fct-7bd54l7n@1 v=cut
@fate n=145 list=relied master=architecture rec=fct-76r3ni6k@1 v=keep as=decision/hit-and-paint-skip-dropped-nodes
name: Hit test and paint tree skip nodes dropped before resolve
rule: A hit test and the paint tree's walk skip a node dropped since the last resolve.
why: Pinned by hit_test_skips_dropped_nodes; hover and pointer events between a mutation and the next resolve rely on it
at: packages/blitz-dom/src/layout/paint_tree.rs:164
@fate n=146 list=relied master=architecture rec=fct-nhbeifug@1 v=keep as=decision/open-pseudo-class-details-dialog
name: :open matches details or dialog carrying open
rule: :open matches a details or dialog that carries the open attribute.
why: Pinned by the open_state tests at_construction_open_is_read_from_the_attribute and a_write_and_a_removal_through_the_mutator_move_it; select_popup checks a select never matches
at: packages/blitz-dom/src/node/element.rs:544
@fate n=147 list=relied master=architecture rec=fct-stpw6pf4@1 v=keep as=decision/shell-redraw-through-event-loop
name: Windowed redraw requests ride the event loop proxy
rule: The windowed shell's redraw requests travel through its event loop and wake it.
why: Every document request_redraw (mutations, loads, focus, select list) reaches the window only as a RequestRedraw event BlitzApplication handles
at: packages/blitz-shell/src/lib.rs:104
@fate n=148 list=relied master=architecture rec=fct-5k5i3eup@1 v=cut
@fate n=149 list=relied master=architecture rec=fct-gosirpve@2 v=cut
@fate n=150 list=relied master=architecture rec=fct-xlm5bxtz@1 v=keep as=decision/owning-component-via-public-api
name: Owning component read through Dioxus public API
rule: A node's owning component is read through Dioxus 0.7.10's public API, a deleted row resolving to none.
why: The stable element ids and the UI graph's owner field read it, and the driver's graph verb surfaces it to agents
at: packages/dioxus-native-dom/src/element_id.rs:182
@fate n=151 list=relied master=architecture rec=fct-zmj47ciz@1 v=keep as=decision/tab-lens-survives-props-replacement
name: Tab store lens survives webview props replacement
rule: A tab's lenses survive a replacement of its webview props.
why: Pinned by tab_lens_survives_webview_props_replacement; the browser tab's resource and favicon tasks keep the lens across parent renders
at: apps/browser/src/tab.rs:327
@fate n=152 list=relied master=architecture rec=fct-b5fra4pv@1 v=keep as=decision/stand-load-page-in-process-url
name: Load page stylesheet URL under .test answered in process
rule: The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network.
why: The stand_load_page driver checks and the session host's load app rely on the boot answering offline with no resolver reached
at: examples/seven_guis/src/stand.rs:158
@fate n=153 list=relied master=architecture rec=fct-qz4rtwlm@2 v=keep as=decision/stand-load-net-answers-after-delay
name: Load stand-in answers from its own thread after 200 ms
rule: The stand's LoadNet answers each fetch from a thread of its own with the fixed CSS after 200 ms, with no socket or listener.
why: the_load_page_click_leaves_a_load_in_flight and the stand_load_page checks need a load in flight that the settle waits out
at: examples/seven_guis/src/stand.rs:170
@fate n=154 list=relied master=architecture rec=fct-oiwxbp2o@1 v=cut
@fate n=155 list=relied master=architecture rec=fct-h2vgfhd5@1 v=cut
@fate n=156 list=relied master=architecture rec=fct-udw5de5d@1 v=keep as=decision/resolve-reconciles-select-list
name: Resolve reconciles the select list before hover
rule: Resolve reconciles the select list after layout and before refreshing hover, and removing a select clears its list.
why: Pinned by a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes and a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened
at: packages/blitz-dom/src/resolve.rs:134
@fate n=157 list=relied master=architecture rec=fct-vgemfdm2@1 v=keep as=decision/multiple-removal-settles-one-option
name: Removing multiple settles the chosen set to one
rule: Removing multiple from a select through the mutator settles its chosen set to one option.
why: Pinned by multiple_removed_settles_the_set_to_one_option; the Dioxus bridge removes attributes through the mutator
at: packages/blitz-dom/src/mutator.rs:463
@fate n=158 list=relied master=architecture rec=fct-oti3svjj@1 v=cut
@fate n=159 list=relied master=architecture rec=fct-dh6z3pmy@1 v=cut
@fate n=160 list=relied master=architecture rec=fct-bq34zg7d@1 v=cut
@fate n=161 list=relied master=architecture rec=fct-jypzod7u@1 v=cut
@fate n=162 list=relied master=architecture rec=fct-czgfalox@1 v=keep as=decision/select-list-rows-and-placement
name: Select list shows at most 20 rows, placed where they fit
rule: An open select list shows at most 20 rows, opens where its rows fit or on the roomier side, and a select with no row to show opens nothing.
why: Pinned by a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel, a_disabled_inert_hidden_singular_or_empty_select_opens_nothing and the place unit test
at: packages/blitz-dom/src/node/select_list.rs:241
@fate n=163 list=relied master=architecture rec=fct-yn7idpix@1 v=cut
@fate n=164 list=relied master=architecture rec=fct-wazi5pch@1 v=keep as=decision/class-atoms-synced-on-write
name: Class atoms kept in sync on class attribute writes
rule: Stylo's class matching reads the element's class atoms, kept in sync on every class attribute write.
why: Stylo's has_class and each_class matching read the atoms, and every class-toggling Dioxus page writes class through the mutator
at: packages/blitz-dom/src/mutator.rs:368
@fate n=165 list=relied master=architecture rec=fct-lg3bylgz@1 v=cut
@fate n=166 list=relied master=architecture rec=fct-z2rkxrmy@1 v=cut
@fate n=167 list=relied master=architecture rec=fct-stih5qpt@1 v=cut
@fate n=168 list=relied master=architecture rec=fct-hczk2fzd@1 v=keep as=decision/load-page-message-hidden-until-sheet
name: Load button links one sheet that reveals Loaded
rule: The load page's Load button links one stylesheet, and the Loaded paragraph stays hidden until that sheet lands.
why: Pinned by the_load_page_click_waits_out_its_load_and_shows_the_message and the_load_page_enter_waits_out_its_load_and_shows_the_message
at: examples/seven_guis/src/stand.rs:194
@fate n=169 list=relied master=architecture rec=fct-dl75efux@1 v=cut
@fate n=170 list=relied master=architecture rec=fct-igzdfobd@1 v=keep as=decision/replay-hook-after-each-step
name: Replay calls a hook after each replayed step
rule: Handler coverage relies on the replay calling a hook after each replayed step, so the screen can be read between steps.
why: The driver's coverage role passes a hook to replay_each that reads the screen's reached handlers between steps
at: packages/escher-driver/src/replay.rs:67
@fate n=171 list=relied master=architecture rec=fct-uzcbujjs@1 v=cut
@fate n=172 list=relied master=architecture rec=fct-2ylcpw3b@1 v=keep as=decision/cells-grid-single-tab-stop
name: Cells grid is one Tab stop with arrow and Enter editing
rule: The cells-grid is one Tab stop whose arrows move the current cell and whose Enter opens the editor, an editor Enter committing and returning focus to the grid.
why: Pinned by the stand_keyboard Cells walk and stand_cells_served, which read focus back on cells-grid after an editor Enter
at: examples/seven_guis/src/tasks/cells.rs:414
@fate n=173 list=relied master=architecture rec=fct-67g4co3e@2 v=keep as=decision/listener-mark-outlives-listener
name: Bridge listener mark outlives its listener
rule: The bridge's event attribute stays whether a listener leaves the attribute list or turns to None.
why: Pinned by a_listener_removed_on_a_re_render_is_read_by_neither_reader, which asserts the mark stays so readers must use the vdom walk
at: packages/dioxus-native-dom/src/mutation_writer.rs:355
@fate n=174 list=relied master=test-plan rec=fct-bloeybfr@1 v=cut
@fate n=175 list=relied master=test-plan rec=fct-vu43sciq@1 v=keep as=decision/recorded-dispatch-bypasses-document
name: Recorded dispatch bypasses document handling
rule: Harness input reaches the document through its real event-dispatch pipeline without a window, while recorded dispatch bypasses document-specific handling such as Dioxus forwarding.
why: pointer_leave, touch_events and range_input_interaction read event names through dispatch_recorded while other checks drive the real pipeline through dispatch
at: packages/blitz-test-harness/src/harness.rs:186
@fate n=176 list=relied master=test-plan rec=fct-px3qbelc@1 v=keep as=decision/harness-counts-fetching-providers-only
name: Harness counts loads of fetching providers only
rule: Harness documents always use HtmlProvider, and only a fetching net_provider is wrapped in the load counter that settle reads.
why: settle reads in-flight loads off the counter; the settle tests and stand_settle's load-in-flight check rely on a fetching stand-in being wrapped
at: packages/blitz-test-harness/src/harness.rs:53
@fate n=177 list=relied master=test-plan rec=fct-4qfz5352@1 v=keep as=decision/wrapped-document-keeps-own-clock
name: Wrapped document keeps the clock its config gave
rule: Documents the harness builds read the harness's virtual clock, while a wrapped document keeps the clock its own config gave it.
why: virtual_clock_engine wraps documents built on their own VirtualClock and moves that clock to time double clicks; the harness-built half is the harness clock rule
at: packages/blitz-test-harness/src/harness.rs:128
@fate n=178 list=relied master=test-plan rec=fct-pmze6kz2@1 v=cut
@fate n=179 list=relied master=test-plan rec=fct-sys2coyr@1 v=cut
@fate n=180 list=relied master=test-plan rec=fct-cd63wvqs@1 v=keep as=decision/select-list-painted-by-paint-scene
name: Select list painted by the scene painter alone
rule: The harness, the session and the binary drive the select list headlessly because the list paints through `paint_scene` alone, with no shell or renderer code of its own.
why: the harness screenshot, the session's screenshot of an open-list row in stand_act_popup and select_popup_painted all read the open list with no shell present
at: packages/blitz-paint/src/render.rs:248
@fate n=181 list=relied master=test-plan rec=fct-ome33psz@1 v=keep as=decision/harness-click-waits-on-nothing
name: Harness click waits on no load
rule: A check that needs a load in flight uses the harness's `click`, which waits on nothing, while a driver `click` waits out `LOAD_WAIT`.
why: stand_actions' load_in_flight helper clicks load-fetch through the harness to hold the fetch in flight, and stand_contrast waits out the load itself afterwards
at: packages/blitz-test-harness/src/input.rs:104
@fate n=182 list=relied master=test-plan rec=fct-qbzx34mx@2 v=keep as=decision/stand-load-page-fixed-answer
name: Load page stand-in answers after a fixed delay
rule: The stand load page's checks rely on its stand-in network answering the fixed URL with fixed bytes after a fixed delay, every other stand boot having no network provider.
why: stand_load_wait times the driver's wait against LOAD_ANSWER_DELAY, and stand_contrast reads the load-done message once the stand-in answered
at: examples/seven_guis/src/stand.rs:168
@fate n=183 list=relied master=test-plan rec=fct-a24rqe7v@2 v=keep as=decision/graph-drives-confirm-every-write
name: Shared graph drives exercise every stand write
rule: The graph and handler-coverage checks rely on the shared graph drives driving each stand task's conditional writes once, so no stand write reads a flag.
why: stand_graph's every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run and stand_handler_coverage both run GRAPH_DRIVES and read no flag
at: tests/blitz-tests/tests/stand_graph.rs:176
@fate n=184 list=relied master=test-plan rec=fct-romvomei@1 v=keep as=decision/key-after-dropped-anonymous-block
name: Key press after a dropped anonymous block
rule: A key press on a focused control whose anonymous block was dropped does not panic, focus stays and the key still acts.
why: key_press_after_container_lost_its_box pins no panic, focus staying and the key still moving a select's choice, in both layout modes
at: tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:137
@fate n=185 list=relied master=test-plan rec=fct-4q3czvm2@2 v=cut
@fate n=186 list=relied master=test-plan rec=fct-4qy7sqin@1 v=keep as=decision/unrun-declared-write-reads-unconfirmed
name: Declared write unrun after its event reads unconfirmed
rule: The UI graph reads a declared write unconfirmed once its handler's event passed through its element and no run wrote the state, so a conditional write reads unconfirmed after runs that skipped it.
why: graph's a_declaration_that_drifts_from_the_code_reads_both_flags pins the flag, and stand_graph needs GRAPH_DRIVES to run each conditional write to read no flag
at: packages/dioxus-native-dom/src/graph.rs:223
@fate n=187 list=relied master=test-plan rec=fct-n4zvuqt2@1 v=keep as=decision/open-pseudo-class-follows-attribute
name: Open pseudo-class follows the open attribute
rule: The `:open` selector matches a details or dialog element carrying `open`, set at construction and kept on each write or removal of the attribute.
why: open_state pins it at construction, on mutator write and removal, and on summary click and Enter, read through an author :open rule
at: packages/blitz-dom/src/node/element.rs:548
@fate n=188 list=relied master=test-plan rec=fct-j4q5jgse@1 v=keep as=decision/failed-load-ends-settle-wait
name: Failed load ends the settle wait
rule: A failed load ends the harness and driver settle wait, so a failed stylesheet does not leave the document not settled on loads for good.
why: the harness test a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it pins it, and the driver's load wait settles through the same harness settle
at: packages/blitz-test-harness/src/settle.rs:121
@fate n=189 list=relied master=test-plan rec=fct-vwiy3q7s@1 v=cut
@fate n=190 list=relied master=security-plan rec=fct-cq2qgcv5@1 v=cut
@fate n=191 list=relied master=security-plan rec=fct-um6i723w@1 v=keep as=decision/invalid-css-declaration-ignored
name: Invalid CSS declaration ignored
rule: Invalid CSS declarations are ignored and unsupported property names or invalid values refused, so no invalid declaration reaches style.
why: The CSSStyleDeclaration setProperty binding in blitz-vibey-script writes the style attribute only when style_attr_set_property returns Some, ignoring an invalid declaration per CSSOM
at: packages/blitz-dom/src/resolved_style.rs:231
@fate n=192 list=relied master=security-plan rec=fct-uzjgvwv6@1 v=keep as=decision/invalid-selector-leaves-rule-unchanged
name: Invalid selector leaves rule and caller unharmed
rule: An invalid selector list leaves its rule unchanged, and query parse errors are returned to the caller rather than panicking.
why: blitz-vibey-script querySelector bindings turn the returned ParseError into a SyntaxError, and the CSSStyleRule selectorText setter relies on the unchanged rule
at: packages/blitz-dom/src/query_selector.rs:62
@fate n=193 list=relied master=security-plan rec=fct-5wlhxhdv@1 v=cut
@fate n=194 list=relied master=security-plan rec=fct-ht5tidu5@1 v=cut
@fate n=195 list=relied master=security-plan rec=fct-bh2jhziy@1 v=keep as=decision/programmatic-scroll-clamped
name: Programmatic scroll clamped and missing node ignored
rule: Scroll targets are clamped to the scroll range, and a programmatic scroll to a missing node does nothing.
why: The fragment_navigation tests scroll_to_sets_element_offset_and_clamps and scroll_by_uses_relative_offsets_and_clamps pin the clamp; script scrollTo and scrollBy call it
at: packages/blitz-dom/src/scrolling.rs:566
@fate n=196 list=relied master=security-plan rec=fct-34fqtojx@1 v=cut
@fate n=197 list=relied master=security-plan rec=fct-66vufcs4@1 v=cut
@fate n=198 list=relied master=security-plan rec=fct-qk7naa6l@1 v=keep as=decision/dangerous-inner-html-parsed-unsanitized
name: Dioxus dangerous_inner_html set as raw inner HTML
rule: Dioxus `dangerous_inner_html` is applied as inner HTML of the element, unsanitized.
why: The stand_keyboard and stand_snapshot_state fixtures build their button rows through dangerous_inner_html and depend on the markup being parsed as given
at: packages/dioxus-native-dom/src/mutation_writer.rs:431
@fate n=199 list=relied master=security-plan rec=fct-ydvrpbvz@1 v=cut
@fate n=200 list=relied master=security-plan rec=fct-b6vsgrbb@1 v=keep as=decision/wpt-runner-js-script-type-detection
name: WPT runner treats only JavaScript script types as scripts
rule: The WPT runner runs only JavaScript script types and escapes its wrapper HTML.
why: The WPT ref test runner calls document_has_scripts to decide whether to run the script runtime, skipping JSON data blocks; the wrapper HTML is built with html_escape
at: wpt/runner/src/test_runners/mod.rs:59
@fate n=201 list=relied master=security-plan rec=fct-zwiprjdk@1 v=keep as=decision/wpt-pr-lookup-by-head-sha-and-marker-splice
name: WPT PR lookup by head sha and marker-bounded splice
rule: The CI PR lookup takes only an open PR whose head sha equals the run's, and the comment splice replaces only when both markers exist in order.
why: The wpt-post-results workflow posts only to the open PR found by head sha, and test_wpt_diff_to_pr pins that a re-run splice leaves the body as it was
at: .github/scripts/wpt_diff_to_pr.py:256
@fate n=202 list=relied master=security-plan rec=fct-63f2oo75@1 v=cut
@fate n=203 list=relied master=security-plan rec=fct-ij663iad@1 v=keep as=decision/clear-history-and-http-cache
name: Clear history and clear cache on demand
rule: Clear history deletes every history row both in memory and on disk, and the HTTP cache can be cleared on demand.
why: The browser toolbar's Clear History and Clear Cache menu items call these; the persistence test clear_empties_table pins the on-disk delete
at: apps/browser/persistence/src/lib.rs:262
@fate n=204 list=relied master=security-plan rec=fct-ecpelafu@1 v=keep as=decision/failed-fetch-error-page-shows-debug-text
name: Failed fetch shows the error page with Debug text
rule: A failed fetch shows the user error.html with the error's Debug text in its error paragraph.
why: The browser app's DocumentLoader shows this page to the user on every failed navigation, the error's Debug text written into the page's error paragraph
at: apps/browser/src/document_loader.rs:154
@fate n=205 list=relied master=security-plan rec=fct-fcagkjyj@1 v=keep as=decision/error-pages-skip-history-and-favicon
name: Error pages skip history and favicon probing
rule: Error pages are flagged is_error and skip history recording and favicon probing.
why: The browser tab's load handler reads is_error to skip record_visit and the favicon probe for synthesized 404 and error pages
at: apps/browser/src/tab.rs:143
@fate n=206 list=relied master=security-plan rec=fct-mcdiomir@1 v=keep as=decision/history-store-errors-logged-and-swallowed
name: History store errors logged and swallowed
rule: Persistence callers never receive a Result; errors are logged at warn and swallowed.
why: BrowsingHistory hands record_visit, clear and set_favicon_by_url to dispatch_disk_write as closures returning nothing, relying on the store never returning a Result
at: apps/browser/persistence/src/lib.rs:202
@fate n=207 list=relied master=security-plan rec=fct-svn2b5w4@1 v=cut
@fate n=208 list=relied master=security-plan rec=fct-xkfthmc4@1 v=keep as=decision/net-handler-error-default-noop
name: NetHandler error default does nothing
rule: NetHandler::error is a default that does nothing, so a handler sees no failure unless it overrides it.
why: The image, font-face and document-src handlers take the no-op default, while the stylesheet handler and the test harness CountedHandler override error to act on a failure
at: packages/blitz-traits/src/net.rs:38
@fate n=209 list=relied master=security-plan rec=fct-27ntkqqx@1 v=keep as=decision/script-errors-recorded-not-propagated
name: Script errors recorded and failed scripts skipped
rule: Uncaught script errors are logged and not propagated, and a script that fails to resolve or fetch is skipped.
why: The WPT harness, ref and crash test runners and the browser's DocumentLoader drain take_js_errors; blitz-vibey-script dom and selection tests read it too
at: packages/blitz-vibey-script/src/document.rs:174
@fate n=210 list=relied master=security-plan rec=fct-hrr3a26v@1 v=cut
@fate n=211 list=relied master=security-plan rec=fct-c4rr3jfv@1 v=keep as=decision/try-doc-none-while-borrowed
name: try_doc answers none while the document is borrowed
rule: Event handling may ask for the document while it is mutably borrowed and gets None instead of a panic.
why: The browser status bar reads the document through try_doc and returns early instead of panicking while event handling holds the borrow
at: packages/dioxus-native-dom/src/events.rs:158
@fate n=212 list=relied master=security-plan rec=fct-p2nck7ms@1 v=keep as=decision/document-abort-signal-cancels-fetches
name: Document abort signal cancels its fetches
rule: Aborting the configured AbortSignal cancels every in-flight fetch tied to the document.
why: The browser DocumentLoader aborts the previous page's controller on each navigation and in abort_current, and iframe reloads abort the prior generation
at: packages/blitz-dom/src/document.rs:646
@fate n=213 list=relied master=security-plan rec=fct-3vrrikft@1 v=cut
@fate n=214 list=relied master=security-plan rec=fct-upgu656r@1 v=keep as=decision/select-row-layout-none-without-styles
name: Select row layout reads none for an unstyled row
rule: The select list's public readers and writer read a row with no styles as None and add no panic path.
why: The blitz-paint select list painter skips a row whose select_row_layout answers None instead of failing
at: packages/blitz-dom/src/document.rs:1068
@fate n=215 list=relied master=security-plan rec=fct-wogfa4ah@1 v=cut
@fate n=216 list=relied master=security-plan rec=fct-64ubemwx@1 v=keep as=decision/select-list-closes-when-select-unfit
name: Open select list closes when its select becomes unfit
rule: A select removed, disabled, hidden or made inert with its list open closes the list with no panic, and a dropped option is no row.
why: The select_popup test a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes pins the close after each layout
at: packages/blitz-dom/src/document.rs:915
@fate n=217 list=relied master=security-plan rec=fct-5onuddg3@1 v=cut
@fate n=218 list=relied master=security-plan rec=fct-36ipyrv2@1 v=cut
@fate n=219 list=relied master=security-plan rec=fct-dxkboam6@1 v=keep as=decision/actions-read-shares-target-check
name: Actions read and verbs share one target check
rule: Every id-taking verb and the actions read meet one shared target check, so what the actions read lists agrees with what a call would be refused.
why: The stand_actions test every_listed_cause_on_the_lean_tasks_is_what_the_call_returns pins that each listed cause equals the refusal the call returns
at: packages/escher-driver/src/actions.rs:83
@fate n=220 list=relied master=security-plan rec=fct-cfr7fwzo@1 v=keep as=decision/actions-read-changes-nothing
name: Actions read runs nothing and moves no session state
rule: The actions read runs nothing and changes no snapshot, focus reading or diff, so it can be called without moving the session.
why: The stand_actions test the_read_changes_nothing_and_holds_no_typed_text_or_mask pins it; agents call the actions verb between steps over socket and MCP
at: packages/escher-driver/src/actions.rs:53
@fate n=221 list=relied master=security-plan rec=fct-lnusqzqb@1 v=keep as=decision/open-pseudo-class-by-open-attribute
name: Open pseudo-class matches by the open attribute
rule: The :open pseudo-class matches a details or dialog by its open attribute's presence, kept current by attribute writes and removals.
why: The open_state tests pin :open on details and dialog through attribute writes and removals, and select_popup checks :open never matches a select
at: packages/blitz-dom/src/node/element.rs:547
@fate n=222 list=relied master=security-plan rec=fct-rc3jl2xp@1 v=keep as=decision/reached-record-deduplicated-survives-removal
name: Reached record deduplicated and kept past element removal
rule: Handler coverage relies on the reached record holding the stable id and event of each node whose listeners held a dispatched event, deduplicated, so a handler whose run removes its element still counts.
why: The reached test a_handler_whose_run_removes_its_element_is_recorded_by_its_id pins it; the coverage role reads reached_handlers, each handler once
at: packages/dioxus-native-dom/src/reached.rs:67
@fate n=223 list=relied master=security-plan rec=fct-6dak5se6@1 v=keep as=decision/one-actionable-rule
name: One actionable rule for both readings
rule: The unkeyed-actionable reading and the actionable-nodes reading both read one actionable rule: focusable, an interactive role, or current listeners holding an event other than `mounted`.
why: The actionable test a_listener_removed_on_a_re_render_is_read_by_neither_reader pins it; the driver's actions verb and the unkeyed lint read these readers
at: packages/dioxus-native-dom/src/actionable.rs:78
@fate n=224 list=relied master=security-plan rec=fct-pnrbnsjg@2 v=keep as=decision/select-readers-resolve-node-closed-select-none
name: Select readers resolve the node and skip an open list
rule: Each public select reader resolves its node so a stale id reads none, and the closed-select reader answers none for a select whose list is open.
why: The escher-driver lint and the snapshot's option bounds call closed_select_of and select_list_bounds, relying on none for a stale id or an open list
at: packages/dioxus-native-dom/src/snapshot.rs:262
@fate n=225 list=relied master=security-plan rec=fct-exdlckxb@1 v=cut
@fate n=226 list=relied master=security-plan rec=fct-o56d7tkg@1 v=keep as=decision/text-extent-read-only-none-when-unresolved
name: Text extent reader read-only with none answers
rule: The text extent reader is read-only and answers none for an id that does not resolve or a node with no inline layout.
why: The text_extent test an_element_with_no_inline_content_reads_none pins the none answers; the escher-driver lint and stand_lint read it for truncation
at: packages/blitz-dom/src/text_extent.rs:25
@fate n=227 list=relied master=security-plan rec=fct-pwj6p2fo@1 v=cut
@fate n=228 list=relied master=security-plan rec=fct-gukbahaa@1 v=cut
@fate n=229 list=relied master=security-plan rec=fct-q43mkq7y@1 v=cut
@fate n=230 list=relied master=security-plan rec=fct-3atajnay@1 v=keep as=decision/dispatch-recorded-only-with-signal-registry
name: Dispatch recorded only where a signal registry stands
rule: The Dioxus event handler records a dispatch only where a signal registry is registered; on a page with none, the event's handlers run and nothing is recorded.
why: The dioxus_document tests a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is and the_reached_record_holds_on_a_page_with_no_registered_state pin it
at: packages/dioxus-native-dom/src/dioxus_document.rs:461
@fate n=231 list=relied master=security-plan rec=fct-et47i5mq@1 v=cut
@fate n=232 list=relied master=security-plan rec=fct-wlz77kgt@1 v=keep as=decision/falsy-boolean-attribute-removed
name: Falsy boolean attribute removed
rule: A falsy value of a listed boolean attribute clears the attribute rather than writing it.
why: The tests falsy_boolean_attributes_are_removed and every_listed_boolean_attribute_is_removed_when_falsy pin it; snapshot disabled readings depend on it
at: packages/dioxus-native-dom/src/mutation_writer.rs:428
@fate n=233 list=relied master=security-plan rec=fct-73s6fq52@1 v=keep as=decision/css-property-named-accessors
name: Named CSS property accessors on style declarations
rule: Script reads and writes CSS properties through named accessors on CSSStyleDeclaration, one per Stylo property name.
why: blitz-vibey-script selection tests write style.textTransform, and WPT computed-value helpers read properties through these accessors
at: packages/blitz-vibey-script/src/runtime/bootstrap/style.js:28
@fate n=234 list=relied master=security-plan rec=fct-62emqkgs@1 v=cut
@fate n=235 list=relied master=security-plan rec=fct-txc2a4of@1 v=cut
@fate n=236 list=relied master=security-plan rec=fct-4ck7btcl@1 v=cut
@fate n=237 list=relied master=security-plan rec=fct-ksl3fslx@1 v=cut
@fate n=238 list=relied master=security-plan rec=fct-4cylhur3@1 v=cut
