@task_prose part=chunk
Takes seven upstream commits, in upstream order, onto the build branch: the reversed-scroll-origin change (#1294), its fix for hit testing elements with no computed styles yet (#1304), and five Taffy bumps that move the engine's Taffy pin to upstream's rev.
Reconciles upstream's reversed scroll origins with escher's ratified readings. A scrolled box reads its own bounds where it stands. A hit stops at the padding box of a clipping box. The absolute position subtracts only containing blocks' offsets. Into-view scrolling passes through every scrolling box that holds the target.
Adds three seam edits so the engine compiles and does not panic: a dropped style-getter import, into-view scroll reading the scroll state as current, min and max, and a private scroll-container read that answers false for a node with no computed styles.
Brings in upstream's tests that ride the picks. The take was already measured in a trial worktree, and this chunk does not re-measure it. It applies the measured staged diff and re-reads the committed cross-OS digests and Counter checks, which pass unchanged.
@task_file n=1 path=Cargo.toml mark=edits
what: the root manifest's engine dependency pins, where Taffy is the one git dependency excepted from the no-git-dependency rule — edits: the taffy rev line
@task_file n=2 path=Cargo.lock mark=conflict
what: the workspace lockfile that locked builds and the dependency audit read, holding Taffy's git source line
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=conflict
what: the engine document, its stylo prefs set at init
@task_file n=4 path=packages/blitz-dom/src/layout/writing_mode.rs mark=conflict
what: the engine's physical layout geometry: a box's bounds, its own-scroll rule and each placer's scroll offset
@task_file n=5 path=packages/blitz-dom/src/node/node.rs mark=conflict
what: the engine node: hit testing (the padding-box hit stop, the self match and the content test) and the node's absolute position
@task_file n=6 path=packages/blitz-dom/src/scrolling.rs mark=edits
what: the engine's scrolling: programmatic scroll, the scroll state and into-view scrolling — edits: scroll_into_view
@task_file n=7 path=packages/blitz-dom/src/debug.rs mark=merge
what: the engine's debug readout of a node's layout
@task_file n=8 path=packages/blitz-dom/src/events/pointer.rs mark=merge
what: the engine's pointer event handling, routed by the hit test
@task_file n=9 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the engine's inline layout, inline-box sizes and their scrollable overflow
@task_file n=10 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the engine's layout tree, the bridge between Taffy and the document
@task_file n=11 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: the engine's layout of replaced elements
@task_file n=12 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the engine's document mutator and its unit tests, including hit testing before the first style resolve
@task_file n=13 path=packages/blitz-dom/src/node/scrollbar.rs mark=merge
what: the engine's scrollbar geometry, read from a box's scroll range
@task_file n=14 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the unpublished engine test crate's display-contents layout tests
@task_file n=15 path=tests/blitz-tests/tests/inline_box_scrollable_overflow.rs mark=merge
what: a new upstream test of an inline box's scrollable overflow, in the unpublished engine test crate
@task_file n=16 path=tests/blitz-tests/tests/pre_overflow_scroll.rs mark=merge
what: the unpublished engine test crate's overflow-scroll tests
@task_file n=17 path=wpt/runner/src/test_runners/attr_test.rs mark=merge
what: the WPT runner's attribute test runner
@task_item n=1 part=needs
The engine's Taffy pin moves to upstream's rev. Taffy stays a git source pinned at a full commit rev, with no patch section and no checksum, and the dependency list is unchanged. Taffy remains the one exception to the no-git-dependency rule, and the rev is the only thing that changes. The exception must stay as stated, together with the engine dependency pins and the rule that an upstream sync takes its dependency set whole.
@task_item n=2 part=needs
The moved lockfile must pass the gates every push owes: locked builds, the MSRV leg, the cargo-deny advisory audit (where ignores are allowed only by ID, each with a reason) and the licence gate.
@task_item n=3 part=needs
New behaviour for every document: a box's content is measured against Taffy's scrollable-overflow rect, and its scroll range can run negative. Programmatic scroll still clamps to that range and still ignores a missing node.
@task_item n=4 part=needs
The hit test keeps escher's stop at the padding box of every box that clips by overflow, and its self match keeps reading the box's own position. The content test now takes upstream's form: the point is checked against the four sides of the scrollable-overflow rect plus the client offset. A non-invertible transform still takes no hit.
@task_item n=5 part=needs
Hit testing an element that has no computed styles yet (upstream's #1304) must neither panic nor take the hit wrongly. This joins the rules that a never-styled node is not read as styled and that a stale node id never panics.
@task_item n=6 part=needs
New private scroll-container read: it answers false for a node with no computed styles and is used at three layout sites. Without it, upstream's new overflow read panicked when an inert, disabled, hidden or removed select closed its open list. The engine must not panic on content the page supplies or on a node a call names.
@task_item n=7 part=needs
Into-view scrolling reads the scroll state as current, min and max, and skips a box whose min and max are both zero. It brings a target into view through every scrolling box that holds it. That includes the open select list, which scrolls a row into view by the row's own offset.
@task_item n=8 part=needs
The node's absolute position subtracts only its containing blocks' scroll offsets, and a scrolled box reads its own bounds without subtracting its own offset. The visible region read through transforms, the visible region of an open select list row, and the off-screen and covered refusals all rest on these readings.
@task_item n=9 part=needs
Boundary held: no new carrier, verb, argument, result field, node key, refusal cause, session error, exit code, log field or span. The driver's answers change only where the engine's readings change, and the measurement found none changed on the stand.
@task_item n=10 part=needs
Upstream's scroll-shifted readings in the second body of the physical geometry and in the hit test's self match do not enter. Escher's own-scroll rule and each placer's subtracted offset stay as they are.
@task_item n=11 part=needs
No other upstream commit enters this take. The later upstream sync stays a chunk of its own.
@task_item n=12 part=needs
The scrollbar geometry and its fade read the new scroll range. The fade still runs on the document clock, and time moves only on advance.
@task_item n=13 part=needs
Inline-box sizes, now read with scrollable overflow, stay finite.
@task_item n=14 part=needs
Re-read, not rebuilt: the committed cross-OS stand digests and the committed stand-Counter checks pass unchanged against the take.
@task_item n=15 part=needs
The WPT runner's attribute test runner follows upstream. Its results stay within the runner's result kinds, and only JavaScript script types are treated as scripts.
@task_standing n=1
convention: A new test in the engine test crate takes no tracing and no style setup of its own, and the test crate stays unpublished.
home: dependency-security
@task_standing n=2
convention: A new engine unit test that hits or lays out a node before its styles resolve reads none or false rather than panicking.
home: error-handling
@task_item n=1 part=not_needed
Authentication and authorization: no auth code, token or CI permission is touched.
@task_item n=2 part=not_needed
Data protection over stored data: browser history, the cookie store, the HTTP cache, the clipboard, file dialogs and kept-check files are untouched.
@task_item n=3 part=not_needed
Secret management: CI secrets, signing material, environment variables and live-run logins are untouched.
@task_item n=4 part=not_needed
The MCP server: its stdio transport, request and result bounds, protocol errors and tool listing are untouched.
@task_item n=5 part=not_needed
The driver session socket, the state directory and recording: socket grammar, replies, connections, idle expiry, owner-only directories, recorded and kept check files are untouched.
@task_item n=6 part=not_needed
Cold-agent and agent-run scripts, install scripts and the coverage floor checker: their argument allowlists, events and logs are untouched.
@task_item n=7 part=not_needed
Outbound network and fetching: script fetchers, URL schemes, per-host limits, CORS and same-origin, request timeouts and navigation routing are untouched.
@task_item n=8 part=not_needed
Stable element ids, snapshot text, accessible names, the actions and graph answers and their crossings: no carrier, field or id derivation changes.
@task_item n=9 part=not_needed
Form-control semantics: password and file masking, range inputs, select choice and typeahead, keyboard activation and optgroup rules are not changed by this take.
@task_item n=10 part=not_needed
HTML, CSS and CSSOM parsing and serialization: invalid declarations, selectors, imports, presentational hints and inline SVG output are untouched.
@task_item n=11 part=not_needed
CI publishing and release: the publish job guards, pinned action SHAs and the WPT post-results workflow are untouched.
@task_item n=12 part=not_needed
The Dioxus native bridge: its scheme, JS evaluation, form data and render buffering are untouched.
