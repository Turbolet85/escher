@task_prose part=session
The session merged upstream main 547d7514b into the fork as a two-parent merge, CI green 17/17 first attempt.
It took the engine dependency bumps (Stylo 0.24, selectors 0.43, parley 0.13, skrifa 0.49, winit =0.31.0-beta.4, svgtypes 0.16.2), held Dioxus at 0.7.3, kept Taffy's git pin as the one git source, and added four precise lock updates past upstream's lock to clear eight audit advisories.
It took nine founder-ruled widenings W1 to W9: an opt-in devtools CDP server with a localhost TCP listener and two env reads, the code-editor crate removed, blitz-net TLS on rustls, Skia built by default, focus events on programmatic focus, isTrusted false for script-dispatched events, a cancelled pointerup still clicking, the SVG mask attribute fetched as a stamped sub-resource, and visibility collapse hidden from the accessibility tree.
It replaced the engine's dispatch callback with upstream's event queue, moved the accessibility tree refresh to the redraw after resolve, and added an engine fallback font for the inline-box placeholder.
It retired arboard and the code-editor crate, and added CDLA-Permissive-2.0 to the licence table.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace manifest — the workspace members, the engine dependency pins, the workspace-inherited dependency versions and the one git source
@task_file n=2 path=Cargo.lock mark=merge
what: the lockfile the locked builds and the dependency audit read
@task_file n=3 path=deny.toml mark=merge
what: the cargo-deny configuration — the licence table of the licence gate and the advisory ignores by id with their reasons
@task_file n=4 path=.github/workflows/ci.yml mark=merge
what: the CI workflow — its jobs, its SHA-pinned actions, the audit and licence legs
@task_file n=5 path=.github/workflows/wpt.yml
what: the WPT workflow that runs the web-platform tests and posts their results
@task_file n=6 path=apps/browser/Cargo.toml
what: the browser app's default features — cookies, cache, screenshot, the renderer
@task_file n=7 path=packages/blitz-net/Cargo.toml
what: blitz-net's dependencies — the reqwest client and its TLS backend
@task_file n=8 path=packages/blitz-net/src/lib.rs
what: blitz-net, the outbound HTTP provider for documents and sub-resources
@task_file n=9 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine document — focus and its focus events, the inert focus clear, the changed set, the event queue, the font contexts
@task_file n=10 path=packages/blitz-dom/src/lib.rs
what: the engine crate root — its exports and the inline-box placeholder font family name
@task_file n=11 path=packages/blitz-dom/src/layout/inline/build.rs
what: the inline layout builder — inline boxes and their placeholder in the text
@task_file n=12 path=packages/blitz-dom/src/document_inline_box_placeholder_tests.rs
what: new engine unit check that an inline-block in text no font can shape is placed once its own font loads
@task_file n=13 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer handling — pointerdown, pointerup, click, select list rows, range moves
@task_file n=14 path=packages/blitz-dom/src/events/keyboard.rs
what: the engine's keyboard handling — Enter and Space activation, select keys, range keys
@task_file n=15 path=packages/blitz-dom/src/events/mod.rs
what: the engine's event entry — the user-agent events it queues and their default actions
@task_file n=16 path=packages/blitz-dom/src/events/focus.rs
what: deleted — the engine's old focus event generation
@task_file n=17 path=packages/blitz-dom/src/events/wheel.rs
what: new upstream wheel handling — wheel gestures and axis lock on scroll
@task_file n=18 path=packages/blitz-dom/src/accessibility.rs
what: the accessibility tree builder — which nodes are hidden from the accessibility tree and what crosses to the adapter
@task_file n=19 path=packages/blitz-dom/src/mutator.rs
what: the document mutator — the inert fix-up on flush and the devtools mutation records
@task_file n=20 path=packages/blitz-dom/src/stylo.rs
what: deleted — the engine's style glue, split by upstream into its own module
@task_file n=21 path=packages/blitz-dom/src/stylo/prefs.rs
what: new — the Stylo prefs set once at start, under the Stylo 0.24 names
@task_file n=22 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: new — the presentational hint attributes read by a typed parse, now with hr and SVG presentation attributes
@task_file n=23 path=packages/blitz-dom/src/stylo/svg_attrs.rs
what: new — the SVG presentation attribute and transform parse
@task_file n=24 path=packages/blitz-dom/src/stylo/user_agent.rs
what: new — the user-agent stylesheet, moved from the document
@task_file n=25 path=packages/blitz-dom/src/net.rs
what: the engine's sub-resource loading — the one stamping constructor for sub-resource requests and import refusal
@task_file n=26 path=packages/blitz-shell/src/window.rs
what: the shell's window view — poll, redraw, the accessibility tree refresh on change
@task_file n=27 path=packages/blitz-shell/src/devtools.rs
what: new — the shell's devtools glue that starts the CDP server from its port env value and opens devtools
@task_file n=28 path=packages/blitz-vibey-cdp-server/Cargo.toml
what: new workspace member — the CDP devtools server crate and its dependencies (tungstenite, the devtools frontend)
@task_file n=29 path=packages/blitz-vibey-cdp-server/src/transport.rs
what: new — the CDP server's TCP listener on 127.0.0.1 and its HTTP and WebSocket connections
@task_file n=30 path=packages/blitz-vibey-cdp-server/src/frontend.rs
what: new — the CDP server's devtools frontend files, read from an env-named directory or embedded
@task_file n=31 path=packages/blitz-traits/src/events.rs
what: the shared event types — the event dispatch source and its trusted flag, the wheel phase and source
@task_file n=32 path=packages/blitz-traits/src/devtools.rs
what: the shared devtools provider trait and its mutation records
@task_file n=33 path=packages/blitz-vibey-script/src/dom/event.rs
what: the script runtime's DOM Event — its isTrusted value
@task_file n=34 path=packages/blitz-vibey-script/src/fetch.rs
what: the script-facing fetch, limited to GET and HEAD through the default script fetcher
@task_file n=35 path=packages/escher-driver/src/execute.rs
what: the driver's verb execution — the select focus it gives without a focus event
@task_file n=36 path=packages/dioxus-native/Cargo.toml
what: the Dioxus native crate's features — its default set, now without the clipboard feature
@task_file n=37 path=wpt/runner/Cargo.toml
what: the WPT runner's dependencies and default renderers, now with Skia
@task_file n=38 path=tests/blitz-tests/tests/svg_mask_attribute_fetch.rs
what: new integration check that an SVG mask attribute URL is fetched stamped with its document
@task_file n=39 path=packages/parley_code_editor
what: deleted — the code-editor crate, a former workspace member
@task_file n=40 path=AGENTS.md
what: deleted — upstream's agent guide, kept out of the fork
@task_item n=1 part=touches
decision/engine-dependency-pins goes stale: Stylo 0.22 to 0.24, winit =0.31.0-beta.3 to =0.31.0-beta.4, parley 0.12 to 0.13, skrifa 0.44 to 0.49; Dioxus held at 0.7.3; Taffy's pin unchanged.
@task_item n=2 part=touches
decision/upstream-sync-dependency-set: the sync took upstream's lock lines, then four precise semver-compatible lock updates past them (pulldown-cmark 0.13.6, rustls 0.23.46, rustls-webpki 0.103.16, webpki-root-certs 1.0.10) to clear eight audit advisories, an operator ruling recorded as a deviation; the merge is two-parent with base 547d7514b.
@task_item n=3 part=touches
decision/tls-via-native-tls goes stale: blitz-net's reqwest now runs on rustls (W3), native-tls-vendored only on Android; outbound HTTP rows naming the TLS backend move with it.
@task_item n=4 part=touches
decision/session-socket-sole-listener, absence/served-network-api and the API Security overview (no binary serves a TCP or UDP port, the one listener) go stale: the devtools feature brings a CDP TCP listener on 127.0.0.1, no Origin check, no authentication, off in every default feature set (W1).
@task_item n=5 part=touches
The environment-variable contract rows (contract/environment-variables/*) gain BLITZ_CDP_PORT and BLITZ_DEVTOOLS_FRONTEND_DIR, read only under the devtools feature; decision/driver-reads-only-rust-log still holds for the driver itself.
@task_item n=6 part=touches
decision/clipboard-via-arboard goes stale: arboard removed, the Dioxus native default set drops the clipboard feature and clipboard text goes through winit.
@task_item n=7 part=touches
decision/inert-focus-clear-silent still holds through a new silent clear, but programmatic focus elsewhere now fires blur, focusout, focus and focusin (W5); rows over focus and its events (decision/button-click-focuses, the driver's select focus) are read against it.
@task_item n=8 part=touches
Any row stating every runtime event is trusted: script-dispatched events now read isTrusted false, user-agent events true (W6).
@task_item n=9 part=touches
decision/svg-image-href-fetch and decision/one-stamping-request-constructor: an SVG mask attribute URL is now fetched as a sub-resource through the same stamping constructor; a fragment-only mask URL is fetched as a stamped data URL (W8, measured).
@task_item n=10 part=touches
decision/presentational-hints-typed-parse: the typed hint parse now covers hr and SVG presentation attributes and SVG transforms (upstream's).
@task_item n=11 part=touches
decision/accessibility-tree-refresh-on-change: the refresh moved from poll to redraw after resolve, reading the changed set and clearing it; poll refreshes no more.
@task_item n=12 part=touches
Rows over engine event production (decision/enter-activates-focused-control, decision/space-keyup-activates-focused-control, decision/engine-change-event-checkable, decision/select-choice-one-write-path, decision/range-pointer-actions, decision/range-keyboard-actions): each event path now queues user-agent events through the event queue instead of a dispatch callback.
@task_item n=13 part=touches
Rows over pointer release and click (decision/select-list-pointer-release-chooses and click activation): a cancelled pointerup still clicks and still ends a scrollbar drag (W7).
@task_item n=14 part=touches
Accessibility tree content rows (decision/accessibility-adapter-content-classes): an element with visibility collapse is now hidden from the accessibility tree (W9).
@task_item n=15 part=touches
decision/licence-gate: the licence table gains CDLA-Permissive-2.0.
@task_item n=16 part=touches
decision/advisory-ignores-by-id and decision/audit-ignores-per-id-with-reason: the six ignores unchanged, their reasons' version words rewritten to winit 0.31.0-beta.4 and accesskit_unix 0.24.1.
@task_item n=17 part=touches
decision/ci-actions-pinned-to-sha: the Android job gains nttld/setup-ndk pinned to commit ed92fe6c, the rule holding.
@task_item n=18 part=touches
decision/no-git-dependency and decision/no-git-dependencies: git sources remain Taffy's pin alone; decision/locked-builds: the lock was settled by one resolver pass then checked locked.
@task_item n=19 part=touches
Dependency Security supply chain: Skia is now built by default in the browser app and the WPT runner, its build script downloading prebuilt binaries from GitHub releases at build time with no content check found (W4).
@task_item n=20 part=touches
decision/cookies-and-cache-default-features: the browser's default set is now skia, cookies, cache, screenshot, apple-font-embolden, scrollbars, mimalloc, without devtools.
@task_item n=21 part=touches
Log sink rows (decision/log-sink-redacts-and-allowlists, decision/engine-target-field-allowlist): the CDP server logs on an engine target whose message escher's sink redacts.
@task_item n=22 part=touches
A row stating webpki-root-certs is depended on only for Wasm: it is reached in the native graph through rustls-platform-verifier, reqwest and blitz-net.
@task_item n=23 part=touches
Workspace member rows: plus the CDP server crate, minus the code-editor crate; new public engine surfaces arrive with the merge (the devtools provider, the event queue, the dispatch source).
@task_fact n=1 slug=api-security
fact: An opt-in devtools feature, off in every default feature set, builds a CDP server that binds a TCP listener on 127.0.0.1 at the port BLITZ_CDP_PORT names, with no Origin check and no authentication (founder's ruling W1).
@task_fact n=2 slug=secret-management
fact: BLITZ_CDP_PORT and BLITZ_DEVTOOLS_FRONTEND_DIR are read only under the devtools feature: the CDP port and the devtools frontend directory.
@task_fact n=3 slug=input-validation
fact: A script-dispatched event reads isTrusted false; an event the user agent dispatches reads true (founder's ruling W6).
@task_fact n=4 slug=input-validation
fact: A programmatic focus change fires blur, focusout, focus and focusin; the inert fix-up's focus clear and the driver's select focus fire none (founder's ruling W5).
@task_fact n=5 slug=input-validation
fact: A pointerup whose handler cancels it still clicks and still ends a scrollbar drag (founder's ruling W7).
@task_fact n=6 slug=input-validation
fact: An SVG mask attribute URL is fetched as a sub-resource through the one stamping constructor; a fragment-only mask URL is fetched as a stamped data URL (founder's ruling W8, measured).
@task_fact n=7 slug=input-validation
fact: An element with visibility collapse is hidden from the accessibility tree (founder's ruling W9).
@task_fact n=8 slug=data-protection
fact: blitz-net's outbound TLS runs on rustls; native-tls-vendored only on Android (founder's ruling W3).
@task_fact n=9 slug=data-protection
fact: Clipboard text goes through winit; arboard is no dependency and the Dioxus native default set carries no clipboard feature.
@task_fact n=10 slug=dependency-security
fact: Skia is built by default in the browser app and the WPT runner; its build script fetches prebuilt binaries from GitHub releases at build time with no content check (founder's ruling W4).
@task_fact n=11 slug=dependency-security
fact: CDLA-Permissive-2.0 is an accepted licence in the licence gate.
@task_fact n=12 slug=dependency-security
fact: An upstream sync takes the dependency set whole, except precise semver-compatible lock updates an operator rules to clear audit advisories, each named with its advisory id in the sync's lock record.
@task_fact n=13 slug=dependency-security
fact: webpki-root-certs is reached in the native graph through rustls-platform-verifier, reqwest and blitz-net, not only for Wasm targets.
@task_item n=1 part=untouched
The driver session socket's grammar, bounds, replies and lifecycle, and the MCP stdio server's requests, results and protocol errors: neither moved.
@task_item n=2 part=untouched
The stable element id, snapshot text, graph, actions and coverage answers and their contracts.
@task_item n=3 part=untouched
Kept check files, recorded checks, the recorder and its directory, and every cold-agent row.
@task_item n=4 part=untouched
Password and file value masking and the masked stand-ins on every carrier.
@task_item n=5 part=untouched
Browser history in sqlite, the private HTTP cache, the cookie store and clear-on-demand.
@task_item n=6 part=untouched
CI secrets, publish guards, release publishing and signing material removal.
@task_item n=7 part=untouched
Coverage floors and the coverage tool, the install and agent-run scripts, the build-directory sweep and its argument checks.
@task_item n=8 part=untouched
The escher-session command line: its flags, roles, exit codes, stdout and stderr contracts, and the driver refusal causes.
@task_item n=9 part=untouched
Select list painting rows (engine colours, twenty rows, painted labels, label never logged) and the select size cap.
