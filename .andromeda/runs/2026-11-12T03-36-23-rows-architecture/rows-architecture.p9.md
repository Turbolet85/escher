@row_add n=1 row=decision/anyrender-rendering-abstraction area=stack-and-technologies rec=1
name: anyrender as the rendering abstraction
rule: Painting goes through the anyrender abstraction, with its vello, vello-cpu, vello-hybrid, skia and svg backends and its serializer.
@row_add n=2 row=decision/window-renderer-chosen-at-compile-time area=stack-and-technologies rec=1
name: Window renderer chosen at compile time
rule: dioxus-native chooses its window renderer at compile time among vello, vello-cpu-base, skia, skia-raster-base and vello-hybrid, while blitz uses the vello window renderer.
@row_add n=3 row=decision/geometry-and-color-crates area=stack-and-technologies rec=2
name: Geometry and color crates
rule: Geometry and color come from kurbo, euclid, app_units, color and peniko.
@row_add n=4 row=decision/wgpu-gpu-layer area=stack-and-technologies rec=2
name: wgpu as the GPU layer
rule: wgpu is the GPU layer of the graphics stack.
@row_add n=5 row=decision/usvg-svg-parsing area=stack-and-technologies rec=2
name: usvg for SVG parsing
rule: SVG is parsed with usvg and svgtypes.
@row_add n=6 row=decision/winit-window-shell area=stack-and-technologies rec=3
name: winit as the window shell
rule: The window shell is built on winit's application handler and event loop.
@row_add n=7 row=decision/keyboard-types-named-by-absolute-path area=stack-and-technologies rec=3
name: Workspace keyboard-types named by absolute path
rule: A module that glob-imports the Dioxus prelude names the workspace's keyboard-types crate as ::keyboard_types.
why: Dioxus carries its own keyboard-types 0.7 beside the workspace's 0.8, so two versions stand in the graph.
@row_add n=8 row=decision/accesskit-accessibility-tree area=stack-and-technologies rec=4
name: AccessKit for the accessibility tree
rule: The accessibility tree is built with accesskit, its platform adapters taken per target through accesskit_xplat.
@row_add n=9 row=decision/blitz-net-reqwest-fetching area=stack-and-technologies rec=5
name: reqwest-based resource fetching in blitz-net
rule: blitz-net fetches HTTP, filesystem and data-URI resources, HTTP through reqwest with native-tls, vendored on Android.
@row_add n=10 row=decision/optional-http-cache area=stack-and-technologies rec=5
name: Optional HTTP cache
rule: blitz-net's HTTP cache is optional, through http-cache-reqwest.
@row_add n=11 row=decision/boa-javascript-engine area=stack-and-technologies rec=6
name: Boa as the JavaScript engine
rule: Boa, with annex-b, is what executes script elements in blitz-vibey-script.
@row_add n=12 row=decision/dioxus-drives-blitz area=stack-and-technologies rec=7
name: Dioxus as the UI framework
rule: The Dioxus virtual DOM drives Blitz through dioxus-native and dioxus-native-dom.
@row_add n=13 row=decision/image-raster-decoding area=stack-and-technologies rec=8
name: image for raster decoding
rule: Raster images are decoded with the image crate, with an SVG fallback.
@row_add n=14 row=decision/serde-json-driver-reader area=stack-and-technologies rec=8
name: serde_json as the driver's JSON reader
rule: serde_json is escher-driver's reader of MCP requests, of kept checks and of the live answer line a replay compares.
why: already in the graph through dioxus-html
@row_add n=15 row=decision/png-shot-encoder area=stack-and-technologies rec=8
name: png as the shot encoder
rule: The workspace's png crate is escher-driver's encoder of a shot and the decoder tests read a shot with.
why: no package enters the lockfile
@row_add n=16 row=decision/no-base64-crate area=stack-and-technologies rec=8
name: No base64 crate
rule: No base64 crate is taken.
@row_add n=17 row=decision/trunk-wasm-example-builds area=stack-and-technologies rec=9
name: Trunk for browser-target example builds
rule: Browser-target builds of the examples go through Trunk and wasm-bindgen.
@row_add n=18 row=decision/parallel-style-traversal area=stack-and-technologies rec=10
name: Parallel style traversal on Stylo's pool
rule: Style traversal runs in parallel on Stylo's global style thread pool.
@row_add n=19 row=decision/parallel-construct-feature area=stack-and-technologies rec=10
name: Parallel construction behind a feature
rule: Parallel construction is the parallel-construct feature, built on rayon and thread_local.
@row_add n=20 row=decision/browser-history-sqlite area=stack-and-technologies rec=11
name: SQLite for browser history
rule: The browser app persists history with bundled rusqlite and rusqlite_migration.
@row_add n=21 row=decision/nucleo-url-suggestions area=stack-and-technologies rec=12
name: nucleo for URL suggestions
rule: The browser's fuzzy URL suggestions use nucleo.
@row_add n=22 row=decision/browser-mimalloc-allocator area=stack-and-technologies rec=12
name: mimalloc as the browser's allocator
rule: mimalloc is the browser's allocator, in its default features.
@row_add n=23 row=decision/rfd-save-dialogs area=stack-and-technologies rec=12
name: rfd for save dialogs
rule: The browser's save dialogs use rfd through the xdg portal.
@row_add n=24 row=decision/android-native-activity-entry area=stack-and-technologies rec=12
name: Android entry through NativeActivity
rule: Android apps enter through android-activity and a Kotlin NativeActivity.
@row_add n=25 row=decision/rdme-markdown-engines area=stack-and-technologies rec=13
name: rdme Markdown engines
rule: rdme renders Markdown with comrak or pulldown-cmark.
@row_add n=26 row=decision/rdme-file-watching area=stack-and-technologies rec=13
name: rdme file watching
rule: rdme watches files with notify.
@row_add n=27 row=decision/rdme-no-wikilinks area=stack-and-technologies rec=13
name: rdme leaves wikilinks off
rule: rdme never enables pulldown-cmark wikilinks, so no wikilinks reader is exposed.
@row_add n=28 row=decision/cli-flow-under-plain-sh area=stack-and-technologies rec=14
name: CLI flow tests under plain sh
rule: seven_guis' unix-gated cli_flow test target runs two POSIX sh scripts under plain sh against the built escher-session binary, a shell being that target's only requirement.
why: no dependency, manifest line or test framework is added for it
@row_add n=29 row=decision/wpt-runner-mimalloc area=stack-and-technologies rec=14
name: mimalloc as the WPT runner's allocator
rule: The WPT runner's global allocator is mimalloc.
@row_add n=30 row=decision/nix-flake-dev-shell area=stack-and-technologies rec=15
name: Nix flake dev shell
rule: A Nix flake provides the reproducible dev shell and packages.
@row_add n=31 row=decision/flake-rust-pin-matches-rust-version area=stack-and-technologies rec=15
name: Flake Rust pin matches rust-version
rule: The flake's pinned Rust toolchain matches the workspace rust-version.
why: the flake's own comment says to keep them in sync
@row_add n=32 row=decision/python3-build-time-for-stylo area=stack-and-technologies rec=15
name: python3 at build time for Stylo
rule: python3 is required at build time to generate code for Stylo.
@row_add n=33 row=decision/github-actions-ci area=stack-and-technologies rec=16
name: GitHub Actions as CI
rule: Build, test, lint, WPT and browser bundling run as GitHub Actions workflows.
@row_add n=34 row=decision/ci-scripts-stdlib-python area=stack-and-technologies rec=16
name: CI helper scripts on the Python stdlib
rule: The CI helper scripts, the coverage floor among them, use the Python standard library only, so the ci-scripts leg needs Python 3.11 or later for tomllib.
why: no package added
@row_add n=35 row=decision/pyyaml-guarded-install area=stack-and-technologies rec=16
name: PyYAML ensured by a guarded step
rule: The ci-scripts job ensures PyYAML as Ubuntu python3-yaml by a step that calls the install script only where importing yaml fails.
@row_add n=36 row=decision/one-ci-leg-runner area=stack-and-technologies rec=16
name: One bash leg runner
rule: Every linux CI job and the dev host run their legs through one bash leg runner.
@row_add n=37 row=decision/one-apt-install-script area=stack-and-technologies rec=16
name: One package-install script
rule: One bash install script is the only statement of fork CI's package install, retrying with a bounded dpkg repair, and adds no package and no action.
@row_add n=38 row=decision/actions-pinned-to-commit-sha area=stack-and-technologies rec=16
name: Actions pinned to commit SHAs
rule: Every action a CI workflow uses is pinned to a 40-hex commit SHA, its ref kept as a trailing comment.
guard: test_ci_workflows.py
@row_add n=39 row=decision/rust-cache-on-compiling-jobs area=stack-and-technologies rec=16
name: Rust cache on compiling jobs
rule: Every compiling CI job uses Swatinem/rust-cache.
except: the coverage job
@row_add n=40 row=decision/cargo-deny-and-llvm-cov-tools area=stack-and-technologies rec=16
name: CI-installed audit and coverage tools
rule: CI installs pinned cargo-deny for the dependency audit and pinned cargo-llvm-cov for the coverage report.
@row_add n=41 row=decision/publish-workflow-upstream-only area=stack-and-technologies rec=16
name: Browser publish workflow upstream-only
rule: The browser publish workflow is upstream-only and guarded by repository.
@row_add n=42 row=decision/cold-agent-client-host-only area=stack-and-technologies rec=16
name: Cold-agent client is a host tool
rule: The cold-agent pipe's agent client, the Claude Code CLI, is a host tool on the operator host only, a run precondition failing with exit 3, and no CI requirement; CI runs only the pipe's shim tests.
@row_add n=43 row=decision/cold-agent-mcp-server-is-own area=stack-and-technologies rec=16
name: Cold-agent MCP server is the workspace's own
rule: The cold-agent pipe's MCP server is the workspace's own escher-session mcp, a run precondition built by the operator and never by the pipe.
@row_add n=44 row=decision/cold-agent-judge-stdlib-python area=stack-and-technologies rec=16
name: Cold-agent judge on stdlib Python
rule: The cold-agent pipe's judge is python3 on its standard library, with no package added.
why: ratified
@row_add n=45 row=decision/fmt-and-clippy-gates area=stack-and-technologies rec=17
name: Format and lint gates
rule: Formatting is checked with cargo fmt and the workspace is linted with clippy with warnings denied, as CI legs.
@row_add n=46 row=decision/local-pre-push-gate area=stack-and-technologies rec=17
name: Local pre-push gate
rule: The fast leg run — fmt, clippy, test, ci-scripts, embolden, stopping at the first red — is the local pre-push gate.
@row_add n=47 row=decision/rustdoc-warnings-gate area=stack-and-technologies rec=17
name: Workspace rustdoc gate
rule: The doc leg documents every workspace library crate with rustdoc warnings denied, with no output-filename collision.
except: the browser binary blitz is not documented, so the lib blitz alone writes its docs
@row_add n=48 row=decision/dependency-and-licence-audit area=stack-and-technologies rec=17
name: Dependency and licence audit
rule: cargo deny checks the advisories and licences of the whole workspace's graph over the root deny.toml, each per-ID ignore carrying a reason.
@row_add n=49 row=decision/per-crate-coverage-floors area=stack-and-technologies rec=17
name: Per-crate line coverage floors
rule: The coverage leg gates line coverage by per-crate floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %.
guard: coverage_floor.py
@row_add n=50 row=decision/blitz-dom-feature-subset-clippy area=stack-and-technologies rec=17
name: Upstream blitz-dom feature-subset clippy line
rule: When the workspace clippy line is green, the clippy leg also runs upstream's clippy over blitz-dom with no default features and its named feature subset, warnings denied.
@row_add n=51 row=decision/stand-repeat-zero-failure-budget area=stack-and-technologies rec=17
name: Stand flakiness budget
rule: The stand-repeat leg holds the stand checks to zero failures over three rounds.
@row_add n=52 row=decision/mutation-tier-host-only area=stack-and-technologies rec=17
name: Host-run mutation tier
rule: A mutation tier runs on the dev host alone, never in CI: cargo-mutants with nextest under a fail-fast mutants profile, each unit's mutant test command read green on the unmutated tree first.
@row_add n=53 row=decision/cognitive-complexity-ceiling area=stack-and-technologies rec=18
name: Cognitive complexity ceiling
rule: A cognitive-complexity reading on the dev host alone, never in CI, reads each function against a ceiling of 15 with rust-code-analysis-cli.
@row_add n=54 row=contract/document-trait/accessibility-tree area=standard-contracts rec=19
boundary: Document trait
item: accessibility_tree
promise: Under the accessibility feature, defaults to the inner document's accessibility tree; a wrapper may override it to enrich nodes, as DioxusDocument does with stable ids.
@row_add n=55 row=contract/document-trait/handle-ui-event area=standard-contracts rec=19
boundary: Document trait
item: handle_ui_event
promise: Defaults to dispatch through an EventDriver with a NoopEventHandler.
@row_add n=56 row=contract/document-trait/poll area=standard-contracts rec=19
boundary: Document trait
item: poll
promise: Defaults to returning false.
@row_add n=57 row=contract/document-config/ua-stylesheets area=standard-contracts rec=19
boundary: DocumentConfig
item: ua_stylesheets
promise: A Vec of UserAgentStylesheet, defaulting to the one default sheet; a config that sets it is used as given.
@row_add n=58 row=contract/document-event/resource-load area=standard-contracts rec=19
boundary: DocumentEvent messages
item: ResourceLoad
promise: Carries a ResourceLoadResponse of request id, node id, resolved URL and a Result of Resource or error string.
@row_add n=59 row=contract/document-event/navigate-iframe area=standard-contracts rec=19
boundary: DocumentEvent messages
item: NavigateIframe
promise: Carries the node id and URL of an iframe to navigate.
@row_add n=60 row=contract/net-resource/resource area=standard-contracts rec=19
boundary: Network resources
item: Resource
promise: An enum of Image, Svg, Css, ImportedCss, Font, DocumentSrc and None.
@row_add n=61 row=contract/net-handler/bytes area=standard-contracts rec=19
boundary: NetHandler
item: bytes
promise: Receives the resolved URL and the fetched bytes, handled per resource kind.
@row_add n=62 row=decision/changed-set-refresh-then-drain area=standard-contracts rec=19
name: Changed set, refresh then drain
rule: A document's changed set holds every tracked write until drained, node creation marking nothing; the windowed shell drains it, and a drained id is read as possibly dropped, never by indexing.
why: ratified; the engine half of one item with the shell's refresh on change, changing behaviour for every Blitz document
@row_add n=63 row=contract/base-document/has-changes area=standard-contracts rec=19
boundary: BaseDocument
item: has_changes
promise: Reads true while the changed set is non-empty, from the first tracked write until the next drain.
@row_add n=64 row=contract/base-document/take-changed-nodes area=standard-contracts rec=19
boundary: BaseDocument
item: take_changed_nodes
promise: Returns the changed set and leaves it empty; a returned id may name a dropped node, so it is read through get_node.
@row_add n=65 row=contract/base-document/can-paint-text area=standard-contracts rec=19
boundary: BaseDocument
item: can_paint_text
promise: Public and read-only: whether the font context resolves a family for the sans-serif generic.
@row_add n=66 row=contract/base-document/layout-as-text-input area=standard-contracts rec=19
boundary: BaseDocument
item: layout_as_text_input
promise: Public and read-only: lays text out with a text input's editor styles, scale and direction in the document's fonts, one unbroken line, None for a node that is no text input.
@row_add n=67 row=decision/document-clock area=standard-contracts rec=20
name: Document time through one clock
rule: A document reads time only through its clock — the config's clock, else a wall clock — a sub-document carrying its parent's clock, and no direct wall-time read stands in blitz-dom outside its clock module.
why: ratified
@row_add n=68 row=contract/document-config/clock area=standard-contracts rec=20
boundary: DocumentConfig
item: clock
promise: An optional shared Clock, None by default, in which case the document uses a fresh WallClock.
@row_add n=69 row=contract/blitz-dom-clock/clock area=standard-contracts rec=20
boundary: blitz-dom clock API
item: Clock
promise: Send + Sync, now() returns a Duration, monotonic from its own origin.
@row_add n=70 row=contract/blitz-dom-clock/wall-clock area=standard-contracts rec=20
boundary: blitz-dom clock API
item: WallClock
promise: Wall time from a web_time Instant origin; new and Default.
@row_add n=71 row=contract/blitz-dom-clock/virtual-clock area=standard-contracts rec=20
boundary: blitz-dom clock API
item: VirtualClock
promise: Clone and Default, every clone reading one shared nanosecond counter; new() reads zero and advance(Duration) adds with saturation.
@row_add n=72 row=contract/base-document/select-value area=standard-contracts rec=21
boundary: BaseDocument
item: select_value
promise: Public and read-only HTML select.value: the chosen option's value, a multiple select's first chosen option in tree order, None with none chosen or for a node that is no select.
@row_none n=1 rec=22 type=5
@row_none n=2 rec=23 type=4
relied: Every read of a select list's state goes through one reader, with only reconcile and the removal arm reading past it.
@row_none n=3 rec=24 type=4
relied: Resolve reconciles the select list after layout and before refreshing hover, and removing a select clears its list.
@row_add n=73 row=contract/base-document/select-values area=standard-contracts rec=25
boundary: BaseDocument
item: select_values
promise: Public and read-only: the select's chosen values.
@row_add n=74 row=contract/base-document/select-of area=standard-contracts rec=25
boundary: BaseDocument
item: select_of
promise: Public and read-only: the select a node belongs to, if any.
@row_add n=75 row=contract/dioxus-native-dom/closed-select-of area=standard-contracts rec=25
boundary: dioxus-native-dom
item: closed_select_of
promise: Answers None for a select whose list is open.
@row_add n=76 row=decision/is-select-public-re-export area=standard-contracts rec=25
name: Select test public and re-exported
rule: The engine's select test is public and re-exported so the driver can test for a select.
why: escher-driver has no blitz-dom dependency to name local_name! with; the masked-input test is the precedent
@row_none n=4 rec=26 type=4
relied: Removing multiple from a select through the mutator settles its chosen set to one option.
@row_add n=77 row=contract/net-handler/error area=standard-contracts rec=27
boundary: NetHandler
item: error
promise: A failed stylesheet responds with its error, so the request leaves the pending critical resources and stops blocking render; other resource kinds do nothing.
@row_add n=78 row=contract/selector-api/query-results area=standard-contracts rec=28
boundary: Selector API
item: query results
promise: Queries return a Result, a parse error for a bad selector, and a scoped query never matches the scope node.
@row_add n=79 row=contract/cssom/addressing area=standard-contracts rec=28
boundary: CSSOM
item: addressing
promise: Stylesheets are addressed by owner NodeId and rules by an index path, read as CssRuleInfo snapshots.
@row_add n=80 row=contract/resolved-style/resolved-style-value area=standard-contracts rec=28
boundary: Resolved style
item: resolved_style_value
promise: Returns an empty string for an unknown property and the used value for a layout-dependent one.
@row_add n=81 row=contract/form-submission/request-content-type area=standard-contracts rec=28
boundary: Form submission
item: RequestContentType
promise: One of urlencoded, multipart or text/plain.
@row_add n=82 row=contract/geometry/bounding-rect area=standard-contracts rec=28
boundary: Geometry
item: BoundingRect
promise: x, y, width and height in f64.
@row_add n=83 row=contract/accessibility-tree/name-sources area=standard-contracts rec=28
boundary: Accessibility tree
item: accessible name
promise: A node is named from its direct text children, a trimmed non-empty aria-label, a bound label element, an option's or optgroup's label attribute, and aria-labelledby.
@row_add n=84 row=contract/accessibility-tree/aria-labelledby area=standard-contracts rec=28
boundary: Accessibility tree
item: aria-labelledby
promise: The referenced built nodes' text becomes the label, outranking a label element and an aria-label; an id naming no element is ignored, and aria-label is the fallback when the targets hold no text.
@row_add n=85 row=contract/accessibility-tree/descriptions area=standard-contracts rec=28
boundary: Accessibility tree
item: aria-describedby and aria-description
promise: aria-describedby's built targets are pushed as described_by and aria-description's text fills the description field, to the adapter only.
@row_add n=86 row=contract/accessibility-tree/aria-expanded area=standard-contracts rec=28,29
boundary: Accessibility tree
item: aria-expanded
promise: Read as a boolean on every element but a single select, which reads none and writes expanded from whether its list is open.
@row_add n=87 row=contract/accessibility-tree/aria-disabled area=standard-contracts rec=28
boundary: Accessibility tree
item: aria-disabled
promise: A value of true sets AccessKit disabled.
@row_add n=88 row=contract/accessibility-tree/aria-level area=standard-contracts rec=28
boundary: Accessibility tree
item: aria-level
promise: Read as a number, with defaults by tag and by role.
@row_add n=89 row=contract/accessibility-tree/aria-selected area=standard-contracts rec=28
boundary: Accessibility tree
item: aria-selected
promise: true or false sets selected; anything else nothing; a select's option reads none, its selected being its chosen state.
@row_add n=90 row=contract/accessibility-tree/aria-pressed area=standard-contracts rec=28
boundary: Accessibility tree
item: aria-pressed
promise: true, false or mixed sets toggled True, False or Mixed on every role but the five checkable ones; anything else nothing.
@row_add n=91 row=contract/accessibility-tree/aria-checked area=standard-contracts rec=28
boundary: Accessibility tree
item: aria-checked
promise: true, false or mixed sets toggled only on CheckBox, Switch, RadioButton, MenuItemCheckBox and MenuItemRadio roles.
@row_add n=92 row=contract/accessibility-tree/aria-invalid area=standard-contracts rec=28
boundary: Accessibility tree
item: aria-invalid
promise: true, grammar or spelling sets invalid True, Grammar or Spelling; absent, empty or false sets nothing; any other value sets True.
@row_add n=93 row=decision/aria-state-values-exact-lowercase area=standard-contracts rec=28
name: ARIA state values matched exactly in lower case
rule: ARIA state values are trimmed of ASCII whitespace and compared exactly in lower case, so an upper-case value reads unrecognized.
why: a choice: the specification as read states no case rule and no browser was measured
@row_add n=94 row=decision/aria-invalid-unknown-is-true area=standard-contracts rec=28
name: Unknown aria-invalid reads invalid
rule: An aria-invalid value outside the allowed list reads as invalid true.
why: WAI-ARIA 1.2's rule for a value outside the allowed list
@row_add n=95 row=decision/toggle-read-per-role area=standard-contracts rec=28
name: Toggle state read per role
rule: aria-checked sets the toggle only on checkable roles and aria-pressed only on others, and a native checkbox or radio reads neither, its native checkedness winning.
why: ratified; native checkedness winning follows HTML-AAM
@row_add n=96 row=decision/each-aria-attribute-read-once area=standard-contracts rec=28
name: Each ARIA attribute read once
rule: The accessibility builder reads each attribute once, keeping the trimmed aria-label and the closed-set state reads over upstream's.
why: differs from upstream, whose bool-parsed aria-selected and unfiltered aria-label reads were dropped in the merge
@row_add n=97 row=decision/accessibility-tree-refresh-on-change area=standard-contracts rec=28
name: Platform accessibility tree refreshed on change
rule: On a poll that reported work the shell drains the document's changed set, with or without the accessibility feature, and hands the adapter a fresh tree when the set was non-empty.
why: ratified as refresh, then drain — the shell half of one item with the engine's changed-set contract
@row_add n=98 row=decision/adapter-crossing-carries-states area=standard-contracts rec=28
name: Adapter crossing carries states
rule: Nodes handed to the platform adapter carry states as closed enums and booleans, never an attribute's value as typed.
why: a widening of the adapter crossing, ratified
@row_add n=99 row=decision/adapter-crossing-carries-descriptions area=standard-contracts rec=28
name: Adapter crossing carries descriptions
rule: Nodes handed to the platform adapter carry descriptions, free page text, to the adapter only; the snapshot reads no description.
why: a widening by upstream merge, ratified as upstream's behaviour
@row_add n=100 row=contract/html-open-attribute/select area=standard-contracts rec=29
boundary: HTML open attribute
item: select
promise: An author's open on a select steers nothing; open state and :open hold to a details or a dialog.
@row_add n=101 row=contract/accessibility-tree/focus area=standard-contracts rec=30
boundary: Accessibility tree
item: focus
promise: Names the focused node only where it is among the built nodes, else the window, so the tree never names a node it did not build.
@row_add n=102 row=contract/accessibility-tree/native-control-toggled area=standard-contracts rec=31
boundary: Accessibility tree
item: native control toggled
promise: A native checkbox or radio under a checkable role writes toggled True or False from its checked state; under any other role it writes nothing, and no author toggle attribute is read on it.
@row_add n=103 row=contract/resolved-style/width-height area=standard-contracts rec=32
boundary: Resolved style
item: width and height
promise: Read from the unrounded layout, so a reader gets sub-pixel values where the final layout rounds.
@row_none n=5 rec=33 type=4
relied: Attribute and child-list mutations restyle only what they affect, with no blanket subtree restyle.
@row_add n=104 row=contract/event-dispatch/pointer-compatibility-events area=standard-contracts rec=34
boundary: Event dispatch
item: pointer compatibility events
promise: Pointer events produce mouse compatibility events for mouse input and touch events for finger and pen.
@row_add n=105 row=contract/event-dispatch/non-bubbling area=standard-contracts rec=34
boundary: Event dispatch
item: non-bubbling events
promise: A non-bubbling event reaches only its target.
@row_add n=106 row=contract/event-dispatch/focus-change area=standard-contracts rec=34
boundary: Event dispatch
item: focus change
promise: A focus change dispatches blur, focusout, focus, focusin.
@row_add n=107 row=contract/event-dispatch/targeting area=standard-contracts rec=34
boundary: Event dispatch
item: event target
promise: Pointer and wheel events target the hovered node, keyboard and IME events the focused node, falling back to the root.
@row_add n=108 row=contract/event-dispatch/range-input area=standard-contracts rec=34
boundary: Event dispatch
item: range input input event
promise: A range input's default actions dispatch input carrying the formatted current value, one per change and none for a move that changes nothing.
@row_add n=109 row=contract/event-dispatch/space-release-click area=standard-contracts rec=34
boundary: Event dispatch
item: KeyUp Space
promise: A released Space on the focused node dispatches a Click.
@row_add n=110 row=contract/event-dispatch/enter-click area=standard-contracts rec=34
boundary: Event dispatch
item: KeyDown Enter
promise: A pressed Enter on the focused node dispatches a Click, after the text-input and range arms.
@row_add n=111 row=contract/event-dispatch/activation-click-order area=standard-contracts rec=34
boundary: Event dispatch
item: activation click
promise: A keyboard activation Click is queued so the app's handlers, such as a Dioxus onclick, run before the default click action.
@row_add n=112 row=contract/event-dispatch/button-click-focus area=standard-contracts rec=34
boundary: Event dispatch
item: button click
promise: Clicking a button or an input of type submit, reset or button focuses it before a submit button submits its form owner.
@row_add n=113 row=contract/event-state/fields area=standard-contracts rec=34
boundary: EventState
item: fields
promise: cancelled, propagation_stopped and redraw_requested, combined by merge.
@row_add n=114 row=contract/dom-event-kind/from-str area=standard-contracts rec=34
boundary: DomEventKind
item: from_str
promise: Strips a leading on, maps composition to Ime and change to Change.
@row_add n=115 row=contract/dom-events/change area=standard-contracts rec=34
boundary: DOM events
item: change
promise: Not cancelable, bubbling, no UI event mapped and no default action; dispatched by the engine after input on a select's choice and a checkbox's or radio's click, never by a text or range input.
@row_add n=116 row=contract/ui-event/pointer-leave area=standard-contracts rec=34
boundary: UiEvent
item: PointerLeave
promise: Clears hover, in a hosted sub-document too, and fires the leave events.
@row_add n=117 row=contract/select-list-default-actions/pointerdown area=standard-contracts rec=35
boundary: Select list default actions
item: pointerdown
promise: A press in the open list stays there; a press elsewhere closes the list.
@row_add n=118 row=contract/select-list-default-actions/click-select area=standard-contracts rec=35
boundary: Select list default actions
item: click on select
promise: Clicking a single select opens its list, or closes one standing, focusing the select.
@row_add n=119 row=contract/select-list-default-actions/pointermove area=standard-contracts rec=35
boundary: Select list default actions
item: pointermove
promise: Moving over the list hovers a row, and a press in the list starts no text drag.
@row_add n=120 row=contract/select-list-default-actions/wheel area=standard-contracts rec=35
boundary: Select list default actions
item: wheel
promise: A wheel over the list scrolls the list; elsewhere it closes the list and then scrolls.
@row_add n=121 row=contract/select-list-default-actions/tab area=standard-contracts rec=35
boundary: Select list default actions
item: Tab
promise: Tab, either direction, chooses the current row and closes the list before moving focus.
@row_add n=122 row=contract/select-list-default-actions/open-keys area=standard-contracts rec=35
boundary: Select list default actions
item: Enter and Space
promise: Enter opens the list, and so does a Space with no typed text standing; a released Space does nothing more.
@row_add n=123 row=contract/select-list-default-actions/choice area=standard-contracts rec=35
boundary: Select list default actions
item: choice
promise: A choice from the list dispatches input then change.
@row_add n=124 row=contract/select-list-default-actions/label-click area=standard-contracts rec=36
boundary: Select list default actions
item: label click
promise: Clicking a label bound to a single select focuses the select and opens nothing, with no re-dispatch to it.
@row_add n=125 row=contract/dom-events/change-on-checkable area=standard-contracts rec=37
boundary: DOM events
item: change on a checkable control
promise: A checkbox click always dispatches change after input with the same value, a radio only where it was not checked before; scripts hear one change, from the engine.
