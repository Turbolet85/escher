@task_prose part=session
The session took upstream Blitz main at 547d7514b into escher as a two-parent merge, not by cherry-pick. Every left-out commit is named with its reason, AGENTS.md stays out and the root README stays escher's own.
The nine widenings W1 to W9 are the founder's rulings. W1 adds an opt-in CDP devtools server crate with a TCP listener. W3 moves blitz-net TLS to rustls. W4 builds Skia by default in the browser and the WPT runner. W5 makes programmatic focus fire events. W6 makes script-dispatched events read isTrusted false. W7 lets a cancelled pointerup still click. W8 fetches SVG mask attribute URLs. W9 hides visibility collapse from the accessibility tree.
Engine pins moved to Stylo 0.24 (selectors 0.43), parley 0.13, skrifa 0.49, winit =0.31.0-beta.4 and svgtypes 0.16.2. Dioxus stays at 0.7.3. On top of upstream's lock, four precise lock updates were made for advisories, and the licence table gained CDLA-Permissive-2.0.
Upstream's event-queue model replaced the dispatch-callback model in every escher event path. The accessibility refresh moved from the shell's poll to its redraw, reading the changed set. blitz-dom's Stylo integration was split into a stylo module, and that module's prefs set replaces escher's own.
An upstream defect was fixed in the engine and flagged upstreamable: an inline-block in text that no font can shape was never placed. The inline-box placeholder now carries a private fallback font family, registered in every font context.
packages/parley_code_editor was retired. CI, the a11y leg, the audit leg and stand-repeat are green.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace manifest: members, shared dependency versions and engine pins (Stylo, parley, skrifa, winit, svgtypes)
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lockfile, the resolved dependency set with Taffy's git pin as the only git source
@task_file n=3 path=deny.toml mark=merge
what: the cargo-deny dependency and licence audit configuration: the licence table and the advisory ignores with their reasons
@task_file n=4 path=.github/workflows/ci.yml mark=merge
what: the GitHub Actions CI workflow and its SHA-pinned actions, including the Android job's NDK setup
@task_file n=5 path=.github/workflows/wpt.yml mark=merge
what: the WPT CI workflow
@task_file n=6 path=apps/browser/Cargo.toml mark=merge
what: the reference browser app's manifest and its default features (renderer choice, cookies, cache, screenshot, allocator)
@task_file n=7 path=packages/blitz-dom/src/document.rs mark=merge
what: BaseDocument, the core DOM document: focus setting and clearing, the changed set, the event queue and font context setup
@task_file n=8 path=packages/blitz-dom/src/lib.rs mark=merge
what: the blitz-dom crate root: its re-exports and crate-wide constants
@task_file n=9 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: inline layout construction, which builds the Parley inline content and inline boxes for a block's text
@task_file n=10 path=packages/blitz-dom/src/document_inline_box_placeholder_tests.rs
what: the unit test module for the inline-box placeholder fallback, a child of the document module
@task_file n=11 path=packages/blitz-dom/src/stylo.rs mark=merge
what: blitz-dom's Stylo integration (DOM traits, traversal, UA stylesheet, prefs), now split into a stylo module
@task_file n=12 path=packages/blitz-dom/src/stylo/prefs.rs mark=merge
what: the set of Stylo prefs a document initialises once
@task_file n=13 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=merge
what: legacy HTML attributes and SVG presentation attributes cascaded as presentational hints
@task_file n=14 path=packages/blitz-dom/src/events/mod.rs mark=merge
what: the engine's DOM event handling and default actions, entered from UI events
@task_file n=15 path=packages/blitz-dom/src/events/pointer.rs mark=merge
what: pointer-derived events and their default actions (pointerdown, pointerup, click, select rows, range input)
@task_file n=16 path=packages/blitz-dom/src/events/keyboard.rs mark=merge
what: keyboard events and their default actions (activation keys, select keys, text input)
@task_file n=17 path=packages/blitz-dom/src/events/focus.rs mark=merge
what: focus event generation for a focus change
@task_file n=18 path=packages/blitz-dom/src/events/wheel.rs mark=merge
what: wheel handling and the per-gesture wheel axis lock
@task_file n=19 path=packages/blitz-dom/src/mutator.rs mark=merge
what: DocumentMutator, the path every DOM mutation takes, including the focus fix-up under inert at flush
@task_file n=20 path=packages/blitz-dom/src/accessibility.rs mark=merge
what: the blitz-dom accessibility tree build and which nodes are hidden from it
@task_file n=21 path=packages/blitz-dom/src/scrolling.rs mark=merge
what: scrolling: programmatic and smooth scroll, scroll chaining and overscroll behaviour
@task_file n=22 path=packages/blitz-dom/src/config.rs mark=merge
what: DocumentConfig, the document configuration struct
@task_file n=23 path=packages/blitz-dom/assets/default.css mark=merge
what: the engine's default user-agent stylesheet
@task_file n=24 path=packages/blitz-net/src/lib.rs mark=merge
what: the blitz-net Provider, which fetches resources over reqwest
@task_file n=25 path=packages/blitz-paint/src/text.rs mark=merge
what: blitz-paint text painting: glyph runs, emboldening and text decorations
@task_file n=26 path=packages/blitz-paint/src/render.rs mark=merge
what: blitz-paint's paint-tree walk emitting commands to the paint scene
@task_file n=27 path=packages/blitz-shell/src/window.rs mark=merge
what: the blitz-shell window View: poll, redraw and the platform accessibility tree refresh
@task_file n=28 path=packages/blitz-shell/src/devtools.rs mark=merge
what: the shell's devtools wiring, which starts the CDP server from the environment
@task_file n=29 path=packages/blitz-vibey-cdp-server/src/transport.rs mark=merge
what: the CDP devtools server's network transport and its TCP listener
@task_file n=30 path=packages/blitz-traits/src/events.rs mark=merge
what: the blitz-traits event types: UI events, the wheel types and the event dispatch source that decides isTrusted
@task_file n=31 path=packages/blitz-vibey-script/src/runtime/events.rs mark=merge
what: the JS runtime's DOM event dispatch to script listeners
@task_file n=32 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=merge
what: the Dioxus document wrapper around BaseDocument: handle_ui_event and poll
@task_file n=33 path=packages/dioxus-native/Cargo.toml mark=merge
what: the dioxus-native manifest and its default features
@task_file n=34 path=packages/escher-driver/src/execute.rs mark=merge
what: the driver's command execution against the document, including its select focus
@task_file n=35 path=tests/blitz-tests/tests/svg_mask_attribute_fetch.rs
what: the blitz-tests integration target that measures which requests an SVG mask attribute URL makes
@task_file n=36 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests shared test module list
@task_file n=37 path=wpt/runner/src/main.rs mark=merge
what: the WPT runner's entry: test selection, renderer choice and the run loop
@task_file n=38 path=packages/blitz-vibey-cdp-server/Cargo.toml mark=merge
what: the manifest of the new CDP devtools server workspace crate
@task_file n=39 path=packages/parley_code_editor mark=merge
what: the parley_code_editor workspace crate, now removed
@task_file n=40 path=AGENTS.md mark=merge
what: upstream's agent instructions file, which escher keeps out of the tree
@task_item n=1 part=touches
Stylo prefs are set once by Stylo 0.22 names, and the Document defaults contract item for Stylo prefs says the same. Both are now stale: upstream's prefs set uses the Stylo 0.24 names and replaces escher's own initialisation.
@task_item n=2 part=touches
Some rows name engine versions. These now read Stylo 0.24 with selectors 0.43, parley 0.13, skrifa 0.49, winit exactly =0.31.0-beta.4 and svgtypes 0.16.2. This covers the Stylo, Parley, skrifa-follows-parley, winit exact beta pin and svgtypes-follows-usvg decisions, and the stack overview's version words.
@task_item n=3 part=touches
The rules that keep the parser crates and html5ever matched to Stylo's web_atoms are re-read: the Stylo bump moved selectors and the atoms with it.
@task_item n=4 part=touches
The deferred Stylo and Dioxus bumps decision is stale for its Stylo half, because Stylo is taken now. Dioxus stays held at 0.7.3, and the Dioxus 0.8 move is deferred again.
@task_item n=5 part=touches
blitz-net's reqwest-based fetching now runs TLS on rustls, not native-tls, with native-tls-vendored on Android. Any row that names native-tls for blitz-net is stale.
@task_item n=6 part=touches
The rows that state no TCP or UDP port is served are contradicted when the devtools feature is on: the served network port absence, the network listener absence, the one local Unix-domain listener and the session socket being the one listener. The CDP server listens on 127.0.0.1. The feature is opt-in, and no default feature set turns it on.
@task_item n=7 part=touches
The environment variable names and the occupied names list gain BLITZ_CDP_PORT and BLITZ_DEVTOOLS_FRONTEND_DIR, both read only under the devtools feature.
@task_item n=8 part=touches
The crate names list gains blitz-vibey-cdp-server as a workspace member. It frees parley_code_editor, which was never named in a row.
@task_item n=9 part=touches
The convention that event producers take a dispatch callback is broken. Every escher event path now queues through the document's event queue (queue_ua_event, pop_pending_event, flush_pending_events), and the dispatch_event parameter is gone.
@task_item n=10 part=touches
Programmatic focus changes now queue blur, focusout, focus and focusin. The focus fix-up under inert still fires no event, through clear_focus_silently. The driver's select focus is silent through the new public set_focus_silently. The evidence of the inert-focus-fixup and focus-change contract items moves to these two functions. The rules themselves stand.
@task_item n=11 part=touches
The site of the accessibility tree refresh moved. The shell's poll no longer refreshes it. Redraw pushes it after resolve when has_changes is true, then empties the changed set through the new clear_changes. This bears on platform tree refresh, rebuild on a non-empty changed set, refresh then drain, and the shell polling after a window event.
@task_item n=12 part=touches
The changed-set contract items (BaseDocument has_changes, document core changed set, where the changed set is written) gain clear_changes. has_changes still reads the non-empty set, and upstream's separate has_changes bool was removed. take_changed_nodes is unchanged.
@task_item n=13 part=touches
Events dispatched by script now read isTrusted false, while events from the user agent read true (EventDispatchSource). This bears on the script event dispatch contract and on any row that says every runtime event is trusted.
@task_item n=14 part=touches
A pointerup cancelled by a handler still clicks and still ends a scrollbar drag. A cancelled pointerdown still counts toward a double click without selecting. This touches the pointer-derived events, pointer compatibility events and default-action event order rows.
@task_item n=15 part=touches
An element with visibility collapse is now hidden from the accessibility tree. This touches the accessibility tree membership and name rows.
@task_item n=16 part=touches
The stamping constructor convention and the rule that engine sub-resource requests carry their document now reach the mask attribute: an SVG mask URL is fetched only through stamped_request. The SVG href fetch rows gain this sibling.
@task_item n=17 part=touches
Legacy attributes cascading as presentational hints now include hr and the SVG presentation attributes. The rows on SVG width and height as presentation attributes and on legacy hints are re-read.
@task_item n=18 part=touches
Skia is now a default renderer. The browser app's default features add skia, and the WPT runner defaults to cpu, tiny-skia and skia. This touches the WPT runner defaulting to CPU rendering, the WPT render target, the renderer chosen at compile time and the anyrender abstraction rows.
@task_item n=19 part=touches
Default features fixed per crate: dioxus-native's default drops clipboard, which is now winit's clipboard, and arboard is removed. The browser's defaults gain skia but not devtools.
@task_item n=20 part=touches
Upstream sync merge base and cherry-picks: this take is a two-parent merge whose second parent is 547d7514b. That becomes the new merge base, and the merge is the form of the take rather than cherry-picks.
@task_item n=21 part=touches
Upstream behaviour changes are ratified per take, and direct engine changes are flagged upstreamable. W1 to W9 are this take's founder-ruled behaviour changes. The inline-box placeholder fallback is a direct engine change flagged upstreamable.
@task_item n=22 part=touches
The dependency and licence audit (the cargo-deny advisory gate) changed. The licence table gains CDLA-Permissive-2.0, and the ignore reasons now name winit 0.31.0-beta.4 and accesskit_unix 0.24.1. The six ignores are unchanged. Four precise lock updates were taken beyond upstream's lock: pulldown-cmark 0.13.6, rustls 0.23.46, rustls-webpki 0.103.16 and webpki-root-certs 1.0.10.
@task_item n=23 part=touches
Actions pinned to commit SHAs: the Android CI job adds nttld/setup-ndk, pinned to commit ed92fe6c.
@task_item n=24 part=touches
No git dependencies, and git sources are Taffy's pin alone: the count of git sources stays at 1.
@task_item n=25 part=touches
The rows for AGENTS.md being kept out and the root README being escher's own are re-met by this take. HEAD's README was kept and AGENTS.md is absent.
@task_item n=26 part=touches
The keyboard_compat bridge stays out of the coverage leg, and the Dioxus bridge's buffered render stands. Upstream's 0.8 deletes of the mutation writer, events and keyboard_compat were restored to HEAD's.
@task_item n=27 part=touches
Rule on the default font when no listed family is available: an inline-box placeholder now falls back to a private family, blitz-inline-box-placeholder, holding the bullet font. It is registered in every font context, a given one included.
@task_item n=28 part=touches
The rules on one blitz-tests target per file and on the blitz-tests manifest gain nine targets: eight from upstream (clipboard, event_queue, native_wheel_data, overscroll_behavior, pointerdown_cancelled, pointerup_cancelled, pseudo_document_order, wheel_axis_lock) and svg_mask_attribute_fetch. The manifest's shape is unchanged.
@task_item n=29 part=touches
The rules that the WPT runner mirrors upstream tooling and is chosen by test kind are re-read. The runner now loads tests from the WPT manifest and has its own renderer module, and test_variants was deleted.
@task_item n=30 part=touches
Public surfaces brought by the take: blitz-dom re-exports stylo_taffy CopyStyleGetters, and dioxus-native-dom re-exports NativeWheelData, BlitzWheelPhase and BlitzWheelSource. DocumentConfig gains devtools_provider under the devtools feature. Node gains is_scroll_container. The UA stylesheet type moved into the stylo module, still used as given.
@task_fact n=1 slug=occupied-resources
fact: Under the opt-in devtools feature, the CDP devtools server binds a TCP listener on 127.0.0.1 at the port BLITZ_CDP_PORT names. It has no Origin check and no authentication, and no default feature set of any crate turns it on (founder ruling W1).
@task_fact n=2 slug=occupied-resources
fact: The env names BLITZ_CDP_PORT and BLITZ_DEVTOOLS_FRONTEND_DIR are occupied and read only under the devtools feature.
@task_fact n=3 slug=occupied-resources
fact: The crate name blitz-vibey-cdp-server is occupied by a workspace member, the CDP devtools server.
@task_fact n=4 slug=cross-cutting-patterns
fact: Engine and user-agent events are queued on the document and dispatched by the event driver's flush of pending events, never through a callback passed to the producer. Queued events for dropped nodes are discarded.
@task_fact n=5 slug=established-decisions
fact: Programmatic focus changes fire blur, focusout, focus and focusin. Only the focus fix-up under inert and the driver's select focus are silent (founder ruling W5).
@task_fact n=6 slug=established-decisions
fact: An event dispatched by script reads isTrusted false, and an event from the user agent reads true (founder ruling W6).
@task_fact n=7 slug=established-decisions
fact: An inline-box placeholder carries a private engine fallback family, so an inline-block inside text that no font can shape is placed once its own font loads. The fix is flagged upstreamable, because upstream reads it red.
@task_fact n=8 slug=established-decisions
fact: A cancelled pointerup still clicks and still ends a scrollbar drag (founder ruling W7).
@task_fact n=9 slug=established-decisions
fact: An element with visibility collapse is hidden from the accessibility tree (founder ruling W9).
@task_fact n=10 slug=conventions
fact: An SVG mask attribute URL is fetched only through the stamping constructor. A fragment-only mask URL such as url(#m) is fetched as a stamped data URL (measured, flagged upstreamable).
@task_fact n=11 slug=stack-and-technologies
fact: Skia is built by default in the browser app and the WPT runner. Its build script downloads prebuilt binaries from GitHub releases at build time with no content check (founder ruling W4).
@task_fact n=12 slug=established-decisions
fact: A wheel gesture scrolls along the axis it settles on, and each gesture decides its own axis. overscroll-behavior contain stops scroll chaining, per axis, on scroll containers only.
@task_item n=1 part=untouched
The driver's wire forms and carriers are unchanged: the session socket wire, the MCP stdio role, answer and refusal JSON, refusal causes, kept check files and the recording directory. No verb, argument or answer shape moved.
@task_item n=2 part=untouched
Stable element ids, the UI graph, the reached record and handler coverage are unchanged.
@task_item n=3 part=untouched
Browser history, persistence, about pages and the browser toolbar are not reached.
@task_item n=4 part=untouched
The cold-agent, agent-run, target-sweep and apt-install scripts, with their exit codes and events, are not reached.
@task_item n=5 part=untouched
escher-telemetry's sink, scrub and line format are unchanged. The capture checks are green, and no escher log site was added.
@task_item n=6 part=untouched
The select list model, range input rules, password masking and the multiple select rows are not changed by the take.
@task_item n=7 part=untouched
The toolchain, the MSRV policy, the Nix flake and the coverage floors are unchanged.
@task_item n=8 part=untouched
The CSSOM geometry readers, hit-testing contracts and the lint verb's defect set are not reached.
