# architecture · the rows 2026-11-15-session-binary-smoke touches — picked 2 of 1498 · rule any · pin 1256

## conventions · ## Conventions
decision/android-entry-point · Android entry points — Android entry points are `android_main` calling `set_android_app`. · r1
decision/annotated-unsafe · Annotated unsafe blocks — `unsafe` blocks carry an `allow unsafe_code` or a `// SAFETY:` comment. · except: Upstream engine code holds unannotated blocks (a node's tree pointer, stylo data, the taffy conversion, layout); escher's own crates hold no `unsafe`. · r1
decision/asset-macro · Assets through the asset macro — Apps reference assets through `asset!`. · r1
decision/assets-through-asset-macro · Assets through the asset macro — Assets in apps and examples come through the asset! macro. · r1
decision/bem-chrome-css · BEM-style chrome CSS names — Browser chrome CSS uses BEM-style names. · r1
decision/blitz-dom-stamped-requests · blitz-dom requests only through the stamping constructor — blitz-dom builds a network Request only through its one crate-private constructor, which stamps the loading document as the request's initiator. · guard: blitz-dom's clippy disallowed-methods rule on Request::get · except: one local expect of the lint at that constructor · r1
decision/blitz-js-native-prefix · Internal JS native prefix — Internal JS natives are prefixed `__blitz_`. · r1
decision/blitz-tests-one-target-per-file · One test target per file in blitz-tests — blitz-tests keeps one test target per file. · except: upstream's tests/all.rs, declared and held at test = false. · r1
decision/blitz-tests-target-per-file · One blitz-tests target per file — blitz-tests keeps one test target per file directly under `tests/`, setting no `autotests`; upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, is held at `test = false`. · why: Ratified; a standing two-line difference from upstream's manifest. · guard: BlitzTestsTargetsTest · r1
decision/borrow-take-and-restore · Take and restore for borrow conflicts — Borrow conflicts are worked around by taking a child list or layout out and restoring it. · r1
decision/both-layout-modes · Scenarios in both layout modes — Test scenarios run in both layout modes, incremental off and on. · except: upstream-taken blitz-tests targets are kept as taken even where they run one layout mode; escher's own targets run both · r2
decision/builder-method-naming · Consuming builder method names — Consuming builder methods are named `with_…` / `without_…`. · except: blitz-traits' Request builders are named by their field, `signal` and `initiator`. · r2
decision/captures-kept-as-captured · Captured fixtures kept as captured — Captured HTML fixtures stand as captured: graphite.html minified, some without a closing head, body or html tag or a doctype. · r1
decision/cargo-feature-gating · Optional functionality behind cargo features — Optional functionality is gated by cargo features. · r1
decision/ci-python-unittest · CI Python tests use unittest — The CI Python tests use `unittest`, one `TestCase` per function. · r1
decision/clippy-allow-scoping · Clippy allows kept local — Clippy allows are local to the code they cover. · except: `clippy::collapsible_if` is allowed crate-wide in five crates (browser, blitz-dom, blitz-html, blitz-paint, blitz-vibey-script) and `clippy::module_inception` in blitz-dom's node module. · r1
decision/const-declared-writers · Task writers written as const — A seven_guis task's writers, a `&'static [DeclaredWrite]`, are written `const { &[…] }` or as a `const` item. · why: A bare slice of writes is a temporary not promoted to `'static` and errors `E0716` at the parameter. · r1
decision/constant-rationale-docs · Rationale for constants — Doc comments state the rationale for constants. · r1
decision/consuming-builder-methods · Consuming builder methods — Builder methods on the script document consume and return the value, named `with_…` or `without_…`. · r1
decision/crate-internal-visibility · Crate internals restricted — Crate internals are `pub(crate)` / `pub(super)`. · r1
decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names. · r1
decision/desktop-cfg-list · Desktop-only code behind a cfg list — Desktop-only code is gated by a repeated cfg list. · r1
decision/desktop-only-cfg-list · Desktop-only cfg list — Desktop-only code is gated by a repeated cfg list of windows, macos, linux, dragonfly, freebsd, netbsd and openbsd. · r1
decision/dioxus-example-shape · Dioxus example shape — Dioxus examples declare `dioxus_native::launch(app)` with `#[component]` components and signal and store hooks, their CSS in `const` raw strings. · r1
decision/dioxus-mutation-buffer-replay · Dioxus bridge buffers then replays mutations — The Dioxus bridge renders the vdom into a crate-private mutation buffer with the document's `RefCell` unborrowed, then borrows the document once and replays the buffer through one mutation writer. · why: Works around borrow conflicts on the Dioxus bridge. · r1
decision/dispatch-event-callback · Event producers take a dispatch callback — Event-producing functions take a `dispatch_event` callback. · r1
decision/do-not-reorder-marker · Ordering-sensitive block marker — An ordering-sensitive block is marked "WARNING: DO NOT REORDER". · r1 · stale
decision/docsrs-all-features · docs.rs builds with all features — docs.rs builds with all features and `doc_cfg` under `docsrs`. · r1
decision/driver-hand-built-spans · Driver spans built by hand — escher-driver builds its spans by hand, with no `#[instrument]`. · r1
decision/dummy-noop-prefix · No-op implementations prefixed Dummy — No-op implementations are prefixed `Dummy`. · r1
decision/dummy-prefix · Dummy prefix for no-op implementations — No-op implementations are prefixed `Dummy`. · r1
decision/error-enums · Error types as enums — Error types are enums with `Display` and a `From` per wrapped error. · r1
decision/example-css-const · Example CSS in a const string — Example CSS is held in a `const CSS: &str` raw string emitted in a `style` element, some named `STYLES`. · r1
decision/example-docs · Example header docs — Examples open with a `//!` comment describing what they demonstrate. · except: Most upstream examples open with a block comment or none. · r1
decision/examples-and-apps-unpublished · Example and app crates unpublished — Example and app crates set `publish = false`. · except: Upstream's `rdme` readme app sets `publish = true`. · r1
decision/examples-panic-on-error · Examples fail by panicking — Examples fail with `.unwrap()`, `.expect(..)` or `panic!`. · r1
decision/feature-fixture-numbered-cases · Numbered cases in feature fixtures — Feature HTML fixtures group numbered cases under h2/h3 headings with a CSS comment per case. · r1
decision/fixture-precondition-first · Assert the fixture's condition first — Tests first assert the fixture produces the condition under test. · r1
decision/fixture-relative-asset-urls · Relative asset URLs in fixtures — HTML fixtures reference sibling assets with ./-relative URLs. · r1
decision/fixtures-test-module · Shared fixtures test module — A `fixtures` test module, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; a page with one reader stays in its check's file. · r1
decision/fmt-and-clippy-gate · Formatting and clippy gate — `cargo fmt --all --check` and workspace clippy with `-D warnings` run as legs in CI and on the dev host, upstream's blitz-dom clippy line only when those are green. · r1
decision/forwarding-accessor-macro · Macro-generated forwarding accessors — A macro generates forwarding accessors for `ElementData`/`DocumentData` fields. · r1
decision/in-repo-crates-no-default-features · In-repo crates without default features — The in-repo engine crates (blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy, dioxus-native, dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`; the other in-repo crates without it. · except: the `seven_guis` path entry keeps its renderer feature · r1
decision/in-repo-default-features-off · In-repo crates without default features — The in-repo engine crates (blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy, dioxus-native, dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`; the other in-repo crates without it. · except: The `seven_guis` path entry keeps its default `hybrid` renderer feature, because `dioxus-native` raises `compile_error!` with no renderer. · r1
decision/inherited-package-metadata · Package metadata inherited from the workspace — Package metadata is inherited with `*.workspace = true`. · r1
decision/integration-test-one-behavior · One behavior per integration test file — Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug. · r1
decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification. · r3
decision/known-gap-markers · Known gaps marked TODO or FIXME — Known gaps are marked with `TODO` or `FIXME`. · r1
decision/let-chains · Let-chains in if conditions — `if` conditions use let-chains. · r1
decision/local-lint-allows · Lints allowed locally — Clippy lints are allowed locally, some with a reason, not crate-wide. · except: `clippy::collapsible_if` crate-wide in browser, blitz-dom, blitz-html, blitz-paint and blitz-vibey-script; `clippy::module_inception` in the node module. · r1
decision/module-and-item-docs · Module and item documentation — Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`. · except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`. · r1
decision/module-docs · Module and file docs — Modules and files open with `//!` docs. · except: The driver crate opens every file with `//!`; upstream engine modules largely open without it. · r1
decision/moved-unit-tests-keep-paths · Moved unit-test modules keep their test paths — Unit-test modules moved to files of their own keep their test paths. · r1
decision/mutations-through-mutator · DOM mutations through DocumentMutator — DOM mutations go through `DocumentMutator`, which flushes deferred work on drop and requests a redraw only for in-document mutations. · r1
decision/mutator-doc-escape-hatch · Mutator's document escape hatch — The mutator's public document access is an escape hatch; extending the mutator is preferred. · r1
decision/mutex-poison-recovery · Mutex locks recover from poisoning — Mutex locks recover from poisoning by taking the inner value. · r1
decision/non-exhaustive-option-types · Non-exhaustive navigation options and requests — `NavigationOptions` and `Request` are `#[non_exhaustive]`. · r1
decision/non-exhaustive-request-types · Non-exhaustive navigation and request types — `NavigationOptions` and `Request` are `#[non_exhaustive]`. · r1
decision/numbered-fixture-cases · Numbered fixture cases — Feature fixtures group numbered cases under h2/h3 headings, with a CSS comment per case. · r1
decision/paint-module-per-concern · Paint rendering one module per concern — Paint rendering is split one module per concern. · r1
decision/public-item-docs · Public item docs — Public items carry `///` docs. · except: Some upstream public items carry no `///`, a node's tree accessor among them. · r1
decision/reactive-state-store-derive · Reactive state derives Store — Reactive state in apps and examples uses the Store derive. · r1
decision/reduced-fixture-variants · Reduced fixture variants — Reduced variants of HTML fixtures sit beside the full pages. · r1
decision/regex-caching · Repeated regexes compiled once — Repeated regexes are held in `LazyLock<Regex>` or compiled per worker. · r1
decision/regression-tests-cite-upstream · Regression tests cite upstream — Regression tests cite upstream issue or PR URLs. · r1
decision/relative-fixture-urls · Relative URLs between fixtures — Fixtures reference sibling assets with `./`-relative URLs. · r1
decision/restricted-crate-internals · Restricted crate internals — Crate internals are `pub(crate)` or `pub(super)`. · r1
decision/rustdoc-warnings-are-errors · Rustdoc warnings are errors — Rustdoc warnings are errors workflow-wide, over every library crate. · r1
decision/scenarios-both-layout-modes · Scenarios in both layout modes — Scenarios run in both layout modes. · r1
decision/section-delimiter-comments · Section delimiter comments — Code sections are delimited with `// === … ===` comments. · r1
decision/section-delimiters · Section delimiters — Sections are delimited with `// === … ===`. · r1
decision/seven-guis-graph-signals · seven_guis signals registered with the graph — seven_guis' tasks register each signal through the tasks module's graph-signal hook, which wraps the plain signal and graph-state hooks; plain signals are left to the stand shell's `active` and the stand's `linked`. · r1
decision/shared-fixtures-stated-once · Shared blitz-tests fixtures stated once — The blitz-tests fixture module states the fixture boot and hold, the shared fixture pages and the held network stand-in once. · r1
decision/shared-test-modules · Shared test modules in subdirectories — The seven_guis stand checks state the tables and helpers they share once, in a `common` module read by `mod common;`, and the checks that hold a `Session` keep theirs in a second such module, `session_common`. · why: A module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior. · r1
decision/single-reader-page-stays-local · Single-reader fixture page stays local — A fixture page with one reader stays in its check's file. · r1
decision/snapshot-before-attribute-mutation · Stylo snapshot before attribute mutation — A Stylo snapshot is taken before attribute mutation of in-document nodes. · r1
decision/spec-url-citations · Spec URLs cited at implementations — Spec URLs are cited above or next to implementations. · r1
decision/split-test-module-as-child · Split test modules stay children of their module — A unit-test module in a file of its own is declared through `#[path]` as a `#[cfg(test)]` child of the module under test, moved out whole with its test paths unchanged. · why: As a child it reaches the module's private fields with no constructor, `pub` or `cfg(test)` shim added. · r1
decision/stand-check-shared-helpers · Stand-check helpers in non-target modules — Stand checks share helpers through modules under the tests directory that are no test target. · r1
decision/std-result-under-dioxus-prelude · std Result spelled under the Dioxus prelude glob — A file that glob-imports the dioxus_native prelude spells a two-parameter result as std::result::Result. · why: On Android the glob brings manganis's one-parameter Result alias, a failure only the CI Android job sees. · r1
decision/store-reactive-state · Store-derived reactive state — Reactive app state uses `#[derive(Store)]` with `#[store]` impls. · r1
decision/stylo-parley-alias-modules · Stylo and Parley type aliases — Stylo and Parley types are aliased under `stylo` and `parley` modules. · r1
decision/stylo-parley-type-aliases · Stylo and Parley types aliased by module — Stylo and Parley types are aliased under `stylo` / `parley` modules. · r1
decision/test-document-setup · Test document setup — Test documents are built from HTML with a `DocumentConfig` holding a `ColorScheme::Light` viewport and the HTML provider, then resolved at time zero. · r1
decision/tests-assert-fixture-first · Tests assert the fixture first — A test first asserts that its fixture produces the condition under test. · r1
decision/todo-fixme-gap-markers · Known gaps marked TODO/FIXME — Known gaps are marked with `TODO`/`FIXME`. · r1
decision/tracing-gated-per-call-site · Tracing gated per call site — In the engine and upstream crates, tracing is gated per call site with a `cfg(not(feature = "tracing"))` fallback. · except: escher's own two crates take `tracing` ungated, escher-telemetry emitting its startup and panic events ungated · r1
decision/track-caller-row-helpers · Track-caller assertion helpers — Assertion helpers are `#[track_caller]`; the MCP listing test and the requirement-sweep test keep their outer assertions and row walk in the test function and assert each row through private ones whose messages name the row and index only. · r1
decision/unit-test-placement · Unit-test placement — Unit tests sit in a test module at the end of the source file, or in a child file of their own. · r1
decision/unit-tests-at-file-end · Unit tests at the end of the source file — Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file. · except: Unit-test modules in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent. · r2
decision/unsafe-justified · Unsafe blocks annotated — `unsafe` blocks carry `allow unsafe_code` or `// SAFETY:` comments. · except: Upstream engine code holds unannotated blocks (a node's tree pointer, stylo data, the taffy conversion, layout); escher's own crates hold no `unsafe`. · r1
decision/unwrap-expect-justified · Justified unwrap and expect — `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification. · r1
decision/windows-subsystem · Windows subsystem for binaries — Binaries set `windows_subsystem = "windows"` outside tests. · r1
decision/windows-subsystem-outside-tests · Windows subsystem for binaries — Binaries set the Windows subsystem to "windows" outside tests. · r1
decision/workspace-dependencies · Dependencies declared once in the workspace — Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`. · except: The WPT runner's direct `mimalloc` dependency. · r1
decision/workspace-licence · Crates take the workspace licence — Crates take their licence from the workspace's dual licence. · except: wgpu_texture is MIT with a SixtyFPS header; accesskit_xplat is Apache-2.0; blitz-dom's default.css carries MPL 2.0; stylo_taffy adds MPL 2.0 · r1
decision/wpt-test-names · WPT test names — WPT test names use forward slashes, relative to `WPT_DIR`. · r1
contract/blitz-vibey-script-api/defaultscriptfetcher · blitz-vibey-script public API · DefaultScriptFetcher — Re-exported, one of the five items blitz-vibey-script re-exports. · r1
contract/blitz-vibey-script-api/document-may-load · blitz-vibey-script public API · document_may_load — Re-exported; the check that lets a `file:` URL load only for a `file:` document. · r1
contract/blitz-vibey-script-api/fetcherror · blitz-vibey-script public API · FetchError — Re-exported, one of the five items blitz-vibey-script re-exports. · r1
contract/blitz-vibey-script-api/scriptdocument · blitz-vibey-script public API · ScriptDocument — Re-exported, one of the five items blitz-vibey-script re-exports. · r1
contract/blitz-vibey-script-api/scriptfetcher · blitz-vibey-script public API · ScriptFetcher — Re-exported, one of the five items blitz-vibey-script re-exports. · r1

## standard-contracts · ## Standard Contracts · too large to hand whole: its other boundaries under `also here:`
decision/a11y-leg-widens-without-new-job · Accessibility leg widens in place — The a11y leg gains tests with no new job, no required status check and no workflow change. · guard: LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log · r1
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
decision/blitz-tests-single-test-target · blitz-tests manifest declares one test table — The blitz-tests package keeps test autodiscovery on and declares exactly one test table, `all`, with test set to false. · guard: BlitzTestsTargetsTest · r1 · add-to · F1
decision/boa-script-engine · Boa runs document scripts — A document's scripts run on Boa, JS listeners before Blitz default actions. · r1
decision/bounded-package-install · Bounded package install — Every fork CI package install goes through the one install script: up to 3 attempts, each update and install under the timeout bound BOUND=120 with the kill grace GRACE=10, pausing 10 s when another attempt remains. · guard: InstallScriptTest; test_o_package_installs_are_bounded · r1
decision/bounded-retried-package-install · Bounded, retried package install — Every CI package install goes through one bounded, retried install script; no ci.yml line calls apt-get. · r1
decision/capability-statement-matches-schema · Capability statement held to the driver schema — The capability statement and README name every verb and refusal cause and state in words the counts of verbs, refusal causes, session errors, node keys and exit statuses that the driver schema holds. · guard: CapabilityStatementTest · r1
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of the CI leg script runs with `--locked`. · except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock` · r1
decision/cells-grid-single-tab-stop · Cells grid is one Tab stop with arrow and Enter editing — The cells-grid is one Tab stop whose arrows move the current cell and whose Enter opens the editor, an editor Enter committing and returning focus to the grid. · why: Pinned by the stand_keyboard Cells walk and stand_cells_served, which read focus back on cells-grid after an editor Enter · r1
decision/cells-not-a-lean-task · Cells is not a lean task — The Cells task boots through its own stand entry and is kept out of the lean task set. · why: Its screen passes the snapshot text's budget and reads in parts. · r1
decision/changed-set-refresh-then-drain · Changed set, refresh then drain — A document's changed set holds every tracked write until drained, node creation marking nothing; the windowed shell drains it, and a drained id is read as possibly dropped, never by indexing. · why: ratified; the engine half of one item with the shell's refresh on change, changing behaviour for every Blitz document · r1
decision/ci-contract-pinned · CI workflow contract pinned by tests — The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg. · guard: test_ci_workflows.py · r1
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
decision/lint-own-bounds-type · Lint's own rectangle type — Every lint rectangle is the lint's own bounds type in the snapshot bounds' space, and lint types hold no engine node id, page text, colour or value. · why: the driver crate names neither blitz-dom nor stylo · r1
decision/listener-mark-outlives-listener · Bridge listener mark outlives its listener — The bridge's event attribute stays whether a listener leaves the attribute list or turns to None. · why: Pinned by a_listener_removed_on_a_re_render_is_read_by_neither_reader, which asserts the mark stays so readers must use the vdom walk · r1
decision/listeners-read-from-vdom · Listeners read from the vdom walk — Every reader of an element's listeners reads the vdom walk's current listeners, never the bridge's listener mark. · why: The listener mark over-reports: a listener a re-render removes leaves its attribute and mark standing, and a mounted listener writes none. · r1
decision/load-page-message-hidden-until-sheet · Load button links one sheet that reveals Loaded — The load page's Load button links one stylesheet, and the Loaded paragraph stays hidden until that sheet lands. · why: Pinned by the_load_page_click_waits_out_its_load_and_shows_the_message and the_load_page_enter_waits_out_its_load_and_shows_the_message · r1
decision/masked-input-values · Password and file input values masked — A password or file input's value reads MASKED_VALUE when non-empty and empty when empty, never the text, its length or a file's host path. · r1
decision/mcp-current-revision-not-taken · Current MCP revision not taken up — The MCP server does not take up the current MCP revision; a handshake asking it is answered with the latest of the four revisions the server speaks. · why: Deferred by decision, left among the remainders. · r1
decision/measured-limits-stated-in-the-tool · Measured limits stated where an agent reads the tool — A measured limit of the driver is stated where an agent reads the tool, in the fixed help texts of the verb table, the cause texts and the node keys' help. · why: Ratified. · r1
decision/mixed-toggle-reads-unchecked · Mixed toggle reads unchecked — A checkable-role node whose toggle is mixed reads checked as Some(false). · why: Ratified. · r1
decision/multiple-removal-settles-one-option · Removing multiple settles the chosen set to one — Removing multiple from a select through the mutator settles its chosen set to one option. · why: Pinned by multiple_removed_settles_the_set_to_one_option; the Dioxus bridge removes attributes through the mutator · r1
decision/nested-scroll-written-instantly · Nested scrolling boxes are written instantly — When scrolling into view, a nested scrolling box is written at once whatever behaviour was asked, and the requested behaviour applies only to the viewport, whose offset is computed after the nested writes. · why: The document holds one scroll animation at a time. · r1
decision/no-base64-crate · No base64 crate — A screenshot's PNG base64 is written by hand in the driver, RFC 4648 §4 padded; no base64 crate is taken. · r1
decision/no-echo-of-typed-input · No echo of typed input — No driver command's ending writes back anything the caller typed. · r1
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
decision/only-stand-repeat-frees-disk · Only stand-repeat frees disk — Only the stand-repeat CI job frees disk. · guard: test_only_stand_repeat_frees_disk · r1
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
absence/exploration-follows-navigation — Exploration that follows navigation to another document · r1
absence/exploration-on-carriers — Exploration on the driver's carriers: no verb, role, argument, MCP tool, session socket call or kept check reaches it · r1
absence/fixture-state-on-carriers — A fixture state on the driver's carriers: the command line, MCP, the session socket or a kept check · r1
absence/script-same-origin-cors — Same-origin and CORS checks on script loads · r1
absence/settle-verb-on-socket — A settle verb or a busy-source reply of its own on the session socket · r1

also here:
- Accessibility tree · 14 row(s)
- blitz-dom accessibility tree · 7 row(s)
- accesskit_xplat adapter · 3 row(s)
- acting verb results · 5 row(s)
- Actionable element · 1 row(s)
- Actionable nodes · 1 row(s)
- Dioxus actionable readers · 2 row(s)
- The actions verb · 6 row(s)
- agent-run.sh JSON-line events · 6 row(s)
- agent-run.sh exit codes · 4 row(s)
- agent-run.sh state · 1 row(s)
- answer JSON · 7 row(s)
- App CLIs · 3 row(s)
- apt-install.sh arguments · 2 row(s)
- apt-install.sh exit codes · 3 row(s)
- apt-install.sh stderr · 1 row(s)
- argument kinds · 7 row(s)
- BaseDocument · 7 row(s)
- BaseDocument readers · 1 row(s)
- blitz-dom clock API · 3 row(s)
- blitz-dom crate root · 1 row(s)
- blitz-net Provider · 1 row(s)
- blitz-traits net types · 1 row(s)
- Browser about URLs · 4 row(s)
- Browser history · 3 row(s)
- Browser toolbar · 1 row(s)
- busy classes · 3 row(s)
- Check replay · 3 row(s)
- ci-leg.sh legs · 15 row(s)
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
- dioxus-native-dom public API · 3 row(s)
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
- escher-driver session · 7 row(s)
- driver verbs · 14 row(s)
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
- seven_guis headless stand · 5 row(s)
- blitz-html and blitz-net crates · 2 row(s)
- HTML open attribute · 1 row(s)
- Kept check reader · 2 row(s)
- Kept check file · 9 row(s)
- key names · 12 row(s)
- high-level launch · 1 row(s)
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
- blitz-dom nodes and widgets · 3 row(s)
- blitz-paint · 3 row(s)
- PR body WPT results section · 2 row(s)
- blitz-traits provider traits · 3 row(s)
- Reached record · 4 row(s)
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
- Telemetry init · 3 row(s)
- Telemetry line · 2 row(s)
- Telemetry panic event · 1 row(s)
- Telemetry scrub · 4 row(s)
- escher-telemetry bootstrap · 5 row(s)
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
