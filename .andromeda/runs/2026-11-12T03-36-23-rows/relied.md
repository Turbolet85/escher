layout-templates · fct-bla7ziiz@1 · The snapshot reads the flight booker's mode select at boot as ComboBox "Flight type" with value one-way and two ListBoxOptions, each at the select's client rect.
layout-templates · fct-zk3a7kfe@1 · While a select list is open each row reads its own bounds in the snapshot and the diff, and after a close the rows read back at the select's client rect.
layout-templates · fct-3yebjmlo@1 · Every document starts with the fixed skeleton html, head, body and main id="main", and the app mounts into main.
obs-plan · fct-w6etu55s@1 · The escher telemetry crate's formatter scrubs engine and escher targets, drops every other target, and prints one stderr line per event and one per closed span, with a chaining panic hook.
obs-plan · fct-ip3ek2mr@1 · Resource load errors in blitz-dom end in a warn event with an `error` field and are not propagated to callers.
obs-plan · fct-6jm2c55w@1 · A failed stylesheet fetch drops its request from the pending critical resources, so loading does not wait on it.
obs-plan · fct-lrj2spd7@1 · A log record from a target outside the sink's allowlist never reaches stderr.
obs-plan · fct-7au5a6cr@1 · A failed, empty or non-CSS stylesheet is answered as an error, so it leaves the pending critical resources and stops blocking render.
obs-plan · fct-omr4vkvp@1 · A net handler that does not override the error method drops the error, the trait's default doing nothing.
obs-plan · fct-x7v6upkc@1 · The mutation buffer's replay re-fires the mutation writer's own trace calls, so the log stream is the one the direct write gives.
obs-plan · fct-7tp6qj2l@1 · The actions read acts on nothing and changes nothing in the document.
obs-plan · fct-v53ms3q6@1 · A page with no registered state reaches no signal registry and dispatches as before.
obs-plan · fct-5rpr77we@1 · The reached record holds stable ids and event names only, no node number, and its debug form prints its count alone.
obs-plan · fct-ovjvwkua@1 · Page scripts never see the native search-params parser, which is deleted from globalThis before any page script runs.
obs-plan · fct-ukvug6pr@1 · A failed load's error text reaches only the Resource load failed line, where the sink redacts url, error and the message.
obs-plan · fct-jyfscbc7@2 · A hosted part read runs through the session's run alone, so its spans are command spans recording verb alone.
obs-plan · fct-46rgllvo@1 · A workspace-wide build turns the engine's tracing feature on, so a test child that installs escher's sink at info also receives engine events.
design-system · fct-blf3mieo@1 · Iframe sub-documents take the parent viewport's color scheme.
design-system · fct-g6x7bhkj@1 · Paint order and hit-testing both group a subtree under the node that the stacking-context rules make a root.
design-system · fct-z3qfoykp@1 · hit, set_hover_to, handle_pointerdown and the driver's covered all read the open select list first, so a button under the list is refused covered.
design-system · fct-cbe7vaww@1 · The open select list paints after the root's render and the blend layers, so it stands above every box of the page.
design-system · fct-keknjqam@1 · The favicon read returns the href of the first link element whose rel contains icon.
design-system · fct-wr73fuj5@1 · scroll-behavior smooth makes Auto scrolls animate, and a wheel over a scroller cancels an in-progress smooth scroll.
design-system · fct-zbk2czd3@1 · A wheel over the open select list scrolls the list, a wheel elsewhere closes it first, and a key brings the current row into view at once.
design-system · fct-dxtawno3@1 · Right after a scroll the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag, so a driver click there needs an advance past the fade first.
design-system · fct-qpwq4lud@1 · A harness settle neither waits on nor advances any animating member and returns Settled with animating reading the document's flag.
design-system · fct-5pxpwwyq@1 · The select list has no motion, so a screen read after a step reads the new state without an advance.
design-system · fct-335wqsvp@1 · While an acting step waits on a load, the harness virtual clock stands still, so no timer or animation runs during the wait.
design-system · fct-lwnjcsh2@1 · An open select list is placed inside the viewport, at least its select's width and at most the viewport's, every edge finite.
design-system · fct-z4dmblpa@1 · Every document gets the blitz default user-agent stylesheet once, through the default document config.
design-system · fct-aboeejrv@1 · A single-choice select lays out as a box holding its chosen option's label, from engine defaults with no design token.
design-system · fct-epbt6wng@1 · A single select's list opens on a click, Enter, or a Space with no typed text standing.
design-system · fct-ypgr7m5t@1 · A disabled, inert or hidden select, a select with no row to show and a multiple select open no list.
design-system · fct-ih466day@1 · A focused select whose container lost its box keeps focus and takes a key with no panic.
design-system · fct-lyaafumo@1 · The logical viewport size is the physical window size divided by hidpi scale times zoom.
design-system · fct-qmikpmeb@1 · The shell sets the window title from the document's title node when it was set before the window existed.
design-system · fct-xmoyplif@1 · WASM builds render sans-serif, serif, monospace and system-ui text with the bundled DejaVu Sans.
design-system · fct-plskesan@1 · List bullets can always draw from the bullet font, which the default font context always registers.
design-system · fct-mrtzanbs@1 · An element with no generic font family is laid out as sans-serif.
design-system · fct-va3exjt4@1 · Layout treats `line-height: normal` as 1.2 times the font size.
design-system · fct-ua34srkn@1 · The `lang` attribute decides the locale text is shaped and broken with.
design-system · fct-etlx3awr@1 · Every caller other than the select group header passes the default run paint and adds no synthesized bold.
a11y-plan · fct-xhpyuh5z@1 · A check reading has_changes() as "this step changed something" must drain the changed set first, since a harness settle does not drain it, a fresh harness document's set is non-empty and a pointer move alone marks nothing.
a11y-plan · fct-sidyoogt@1 · A node's role is taken from its role attribute, else from the HTML element mapping, else Unknown.
a11y-plan · fct-2bz7tofn@1 · Focus order relies on an element being focusable only when not inert, not a disabled form control and holding tabindex >= 0 or being a native focusable element, and a focused node coming under inert gives focus up with no blur or focusout.
a11y-plan · fct-7hugl7cl@1 · A pointer click focuses a button, a button-type input or a select, while a click on any other element, such as a CRUD row or the list, clears focus.
a11y-plan · fct-dzuhkwn2@1 · A closed single select opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root.
a11y-plan · fct-pqyrk5rl@1 · Tab moves focus to the next focusable node and Shift+Tab to the previous one, dispatching focus events.
a11y-plan · fct-ag43xb7g@1 · With a select's list open, Tab chooses the current row and closes the list before moving focus, in either direction.
a11y-plan · fct-yxl65pxf@1 · Focus stays on a select while its list is open, and the Tab walk holds with the list open and after each close.
a11y-plan · fct-is66hfzy@1 · Cached focusability is recomputed when tabindex, href or disabled is set or removed, so a roving tabindex works.
a11y-plan · fct-jc7cdake@1 · The :focus-visible selector never matches, while :focus and :focus-within do.
a11y-plan · fct-3rgd2unt@1 · Removing the focused node resets focus to the body and runs blur side effects.
a11y-plan · fct-7uphfywk@1 · Clicking a checkbox or radio toggles it, dispatches input and moves focus to the control.
a11y-plan · fct-o3zz2arg@1 · A page's script hears exactly one change per checkable click, from the engine, since the script runtime synthesizes none.
a11y-plan · fct-y4ncelk5@1 · Each key change on a single select dispatches one input then one change carrying the select's value, a key that changes nothing firing none.
a11y-plan · fct-k6bfqrau@1 · Every choice from an open list goes through one write path that fires one input then one change, and choosing the already chosen option fires nothing.
a11y-plan · fct-lvplfrnk@1 · A disabled optgroup's rows read as disabled through one rule that the readings and the form's entries share.
a11y-plan · fct-6opcw6lo@1 · The driver's type on a select focuses it with no click and adds nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors or exits.
a11y-plan · fct-gp3teo4p@1 · A click on a label bound to a single select focuses the select, opens nothing and re-dispatches nothing to it.
a11y-plan · fct-55wog4z5@1 · A synthetic click is a primary mouse click at the node's center.
a11y-plan · fct-v3jtrgvp@1 · A script's element.style read or write goes through the inline style, so display and visibility reach the tree's hidden-content exclusion.
a11y-plan · fct-tc36t6wc@1 · The Android native view is focusable in touch mode and requests focus, so key presses reach the app.
a11y-plan · fct-jn5hbj7a@1 · Text nodes become TextRun nodes carrying their text, and their parent is labelled by them, so an element's text reaches its accessible name.
a11y-plan · fct-5vvi2itr@1 · Callers receive no initial tree synchronously; initial-tree requests, action requests and deactivation all reach one EventHandler.
a11y-plan · fct-fod53y3h@1 · A text input already focused when its text editor is constructed has IME enabled with no fresh focus, in every document.
a11y-plan · fct-ajar2qgv@1 · Author styles keyed on `:open` apply to an open details or dialog, `:open` reading the element's open state.
a11y-plan · fct-jajzvsxm@1 · An open select's list is no `:open` state, so no author style keys on a list standing open.
a11y-plan · fct-o4ijraeq@1 · The viewport carries a light/dark color scheme and a document zoom level, 1.0 unzoomed.
a11y-plan · fct-zd6d5frj@1 · A color-scheme change on the viewport is a device change that rebuilds the stylist device.
a11y-plan · fct-dwhu55j2@1 · A total scale change from hidpi or zoom invalidates text shaping.
a11y-plan · fct-anrtdftj@1 · Right after a scroll, a driver click in the shown overlay scrollbar's band is taken as a thumb drag, so an advance past the fade is needed first.
architecture · fct-j5k4uy7t@1 · A failed, empty or non-CSS stylesheet stops blocking render, because a failed fetch's error answer takes the request out of the pending critical resources.
architecture · fct-54e4fuk7@1 · Every shell window event is followed by a Poll.
architecture · fct-aj6cmiwc@1 · Launch paths built with `net` off wasm32 enter a tokio multi-thread runtime before the event loop runs.
architecture · fct-35cb7qc2@1 · Each JS entry point requests a redraw and re-arms the timer thread that wakes the event loop at the next JS timer deadline.
architecture · fct-2r7bauw3@1 · Headless loading resolves the document repeatedly until the net provider is empty, then resolves once more.
architecture · fct-dajqtqwx@1 · The WPT runner resets the net provider before each test and runs each test inside a panic catch, a panic yielding Unknown or Crash.
architecture · fct-z4p5ffso@1 · Each browser document load aborts the previous one, as reload and drop do.
architecture · fct-6decztyx@1 · Legacy presentational attributes map to CSS at the presentational-hints level.
architecture · fct-2es3quc6@1 · A `Node` holds a raw pointer to its `NodeTree` and is marked `Send` and `Sync` by unsafe impl.
architecture · fct-kknd7ewa@1 · WPT failures are bucketed by feature-usage flags computed by regex over test sources.
architecture · fct-4mapcv7w@1 · A text input that holds focus before its editor exists gets the shell provider's IME enabled when its editor is constructed.
architecture · fct-biqhin7y@1 · The first graph-state hook of a page provides a `SignalRegistry` context at the root scope, which the bridge's dispatch looks up; a page that registers no state has none.
architecture · fct-tzrr5o2d@1 · The bridge's dispatch finds the reached record by a root-context lookup, and a document never switched on has no record.
architecture · fct-3kximnte@1 · A queued mounted event whose element was removed since it was queued is skipped and never fires.
architecture · fct-56u6r4qr@1 · Where a signal registry stands, each signal write is recorded against the dispatch that caused it, and a page with no registered state dispatches unchanged.
architecture · fct-xcqj3bw6@1 · An open select list closes with no event on Escape, a second click, a press or wheel outside, a scroll or move of the select's box, and on removal, disabled, hidden or inert.
architecture · fct-t7ezjz4v@1 · On an open select list Tab chooses the current row and closes before focus moves, and the move keys shift the highlight without choosing.
architecture · fct-hxuv6j35@1 · A multiple select's arrow, Home and End keys choose one row alone, with Shift spanning from the anchor and Ctrl or Meta moving the current row without changing the choice.
architecture · fct-jyueaf6l@1 · A plain click on a multiple select's row chooses it alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing.
architecture · fct-75lrk5x7@1 · A replaced element's natural height follows its usable aspect-ratio, clamped at 0.
architecture · fct-3oxpzjue@1 · Every box-generating ::before or ::after votes in flow classification, so a pseudo-only inline-block exports its line baseline.
architecture · fct-kvuycvyd@1 · A text input already focused when its editor is constructed enables IME.
architecture · fct-jzkmudp4@1 · A focus move keeps focus-within on the shared ancestors, requests a redraw, and marks exactly the node that lost focus and the node that gained it as changed.
architecture · fct-qcw5degj@1 · The accessibility tree gives an optgroup the group role named by its non-empty label.
architecture · fct-wau2zivr@1 · In an open select list a group's label paints as a header that chooses nothing.
architecture · fct-7bd54l7n@1 · A multiple select's size hints a height of size times 1.2em, capped at 65534, with vertical overflow scrolling.
architecture · fct-76r3ni6k@1 · A hit test and the paint tree's walk skip a node dropped since the last resolve.
architecture · fct-nhbeifug@1 · :open matches a details or dialog that carries the open attribute.
architecture · fct-stpw6pf4@1 · The windowed shell's redraw requests travel through its event loop and wake it.
architecture · fct-5k5i3eup@1 · An element's border box and culling stand on the scaled size, its location kept for the transform.
architecture · fct-gosirpve@2 · A task polled during render acts on the document as the previous render left it, so a node created in the same render cannot yet take focus.
architecture · fct-xlm5bxtz@1 · A node's owning component is read through Dioxus 0.7.10's public API, a deleted row resolving to none.
architecture · fct-zmj47ciz@1 · A tab's lenses survive a replacement of its webview props.
architecture · fct-b5fra4pv@1 · The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network.
architecture · fct-qz4rtwlm@2 · The stand's LoadNet answers each fetch from a thread of its own with the fixed CSS after 200 ms, with no socket or listener.
architecture · fct-oiwxbp2o@1 · Stylo's prefs are set once per process before the user-agent sheet is parsed.
architecture · fct-h2vgfhd5@1 · Every read of a select list's state goes through one reader, with only reconcile and the removal arm reading past it.
architecture · fct-udw5de5d@1 · Resolve reconciles the select list after layout and before refreshing hover, and removing a select clears its list.
architecture · fct-vgemfdm2@1 · Removing multiple from a select through the mutator settles its chosen set to one option.
architecture · fct-oti3svjj@1 · Attribute and child-list mutations restyle only what they affect, with no blanket subtree restyle.
architecture · fct-dh6z3pmy@1 · The driver's in-view check and crop read the visible region through transforms, a rotation reading the rotated box's bounding box.
architecture · fct-bq34zg7d@1 · The hit, the driver's covered check, hover and pointerdown pass over child nodes dropped since the last resolve, and an open select list answers a hit first.
architecture · fct-jypzod7u@1 · A subtree under a non-invertible transform answers no hit.
architecture · fct-czgfalox@1 · An open select list shows at most 20 rows, opens where its rows fit or on the roomier side, and a select with no row to show opens nothing.
architecture · fct-yn7idpix@1 · The layout pass reads a node's out-of-flow state from the flag recorded by the style traversal.
architecture · fct-wazi5pch@1 · Stylo's class matching reads the element's class atoms, kept in sync on every class attribute write.
architecture · fct-lg3bylgz@1 · The :open pseudo-class matches a details or dialog carrying an open attribute in every document.
architecture · fct-z2rkxrmy@1 · An open select list paints last, past the blend layers.
architecture · fct-stih5qpt@1 · The windowed shell's redraw requests travel through its event loop.
architecture · fct-hczk2fzd@1 · The load page's Load button links one stylesheet, and the Loaded paragraph stays hidden until that sheet lands.
architecture · fct-dl75efux@1 · The load page's network stand-in answers its one stylesheet after 200 ms and is counted as a load by the harness.
architecture · fct-igzdfobd@1 · Handler coverage relies on the replay calling a hook after each replayed step, so the screen can be read between steps.
architecture · fct-uzcbujjs@1 · A page with no registered state reaches no registry and dispatches events as before.
architecture · fct-2ylcpw3b@1 · The cells-grid is one Tab stop whose arrows move the current cell and whose Enter opens the editor, an editor Enter committing and returning focus to the grid.
architecture · fct-67g4co3e@2 · The bridge's event attribute stays whether a listener leaves the attribute list or turns to None.
test-plan · fct-bloeybfr@1 · Snapshot-style assertions rely on the harness's DOM string being a stable one-node-per-line tree serialization.
test-plan · fct-vu43sciq@1 · Harness input reaches the document through its real event-dispatch pipeline without a window, while recorded dispatch bypasses document-specific handling such as Dioxus forwarding.
test-plan · fct-px3qbelc@1 · Harness documents always use HtmlProvider, and only a fetching net_provider is wrapped in the load counter that settle reads.
test-plan · fct-4qfz5352@1 · Documents the harness builds read the harness's virtual clock, while a wrapped document keeps the clock its own config gave it.
test-plan · fct-pmze6kz2@1 · A red job's later targets go unread on that run, since cargo stops at the first failing target.
test-plan · fct-sys2coyr@1 · The buffered render's key-handler pins discriminate on Dioxus 0.7 because a task an event handler spawns while dirtying a scope is first polled inside `render_immediate`.
test-plan · fct-cd63wvqs@1 · The harness, the session and the binary drive the select list headlessly because the list paints through `paint_scene` alone, with no shell or renderer code of its own.
test-plan · fct-ome33psz@1 · A check that needs a load in flight uses the harness's `click`, which waits on nothing, while a driver `click` waits out `LOAD_WAIT`.
test-plan · fct-qbzx34mx@2 · The stand load page's checks rely on its stand-in network answering the fixed URL with fixed bytes after a fixed delay, every other stand boot having no network provider.
test-plan · fct-a24rqe7v@2 · The graph and handler-coverage checks rely on the shared graph drives driving each stand task's conditional writes once, so no stand write reads a flag.
test-plan · fct-romvomei@1 · A key press on a focused control whose anonymous block was dropped does not panic, focus stays and the key still acts.
test-plan · fct-4q3czvm2@2 · A task polled during render acts on the document as the previous render left it, so a node created in that same render cannot yet be focused.
test-plan · fct-4qy7sqin@1 · The UI graph reads a declared write unconfirmed once its handler's event passed through its element and no run wrote the state, so a conditional write reads unconfirmed after runs that skipped it.
test-plan · fct-n4zvuqt2@1 · The `:open` selector matches a details or dialog element carrying `open`, set at construction and kept on each write or removal of the attribute.
test-plan · fct-j4q5jgse@1 · A failed load ends the harness and driver settle wait, so a failed stylesheet does not leave the document not settled on loads for good.
test-plan · fct-vwiy3q7s@1 · The lint's overlap reading skips any entry inside a select's box, closed or open, so the stand's lint reads no overlap on select options and open-list rows.
security-plan · fct-cq2qgcv5@1 · A parse error in inline SVG source is logged and leaves the element without image data instead of failing the document.
security-plan · fct-um6i723w@1 · Invalid CSS declarations are ignored and unsupported property names or invalid values refused, so no invalid declaration reaches style.
security-plan · fct-uzjgvwv6@1 · An invalid selector list leaves its rule unchanged, and query parse errors are returned to the caller rather than panicking.
security-plan · fct-5wlhxhdv@1 · Legacy presentational attributes pass closed parsers, and any other value is ignored or rejected.
security-plan · fct-ht5tidu5@1 · `text_extent` is a read-only reader returning geometry only.
security-plan · fct-bh2jhziy@1 · Scroll targets are clamped to the scroll range, and a programmatic scroll to a missing node does nothing.
security-plan · fct-34fqtojx@1 · A file input's `accept` is not applied as a filter on the files chosen.
security-plan · fct-66vufcs4@1 · Form data validity in dioxus-native always reports valid, so no form is ever held back as invalid.
security-plan · fct-qk7naa6l@1 · Dioxus `dangerous_inner_html` is applied as inner HTML of the element, unsanitized.
security-plan · fct-ydvrpbvz@1 · The test harness query helper panics on an invalid selector, while selector parsing itself returns a `Result`.
security-plan · fct-b6vsgrbb@1 · The WPT runner runs only JavaScript script types and escapes its wrapper HTML.
security-plan · fct-zwiprjdk@1 · The CI PR lookup takes only an open PR whose head sha equals the run's, and the comment splice replaces only when both markers exist in order.
security-plan · fct-63f2oo75@1 · Request URLs and the cache directory path appear in the logs.
security-plan · fct-ij663iad@1 · Clear history deletes every history row both in memory and on disk, and the HTTP cache can be cleared on demand.
security-plan · fct-ecpelafu@1 · A failed fetch shows the user error.html with the error's Debug text in its error paragraph.
security-plan · fct-fcagkjyj@1 · Error pages are flagged is_error and skip history recording and favicon probing.
security-plan · fct-mcdiomir@1 · Persistence callers never receive a Result; errors are logged at warn and swallowed.
security-plan · fct-svn2b5w4@1 · A failed stylesheet request leaves the pending critical resources, so it blocks no render.
security-plan · fct-xkfthmc4@1 · NetHandler::error is a default that does nothing, so a handler sees no failure unless it overrides it.
security-plan · fct-27ntkqqx@1 · Uncaught script errors are logged and not propagated, and a script that fails to resolve or fetch is skipped.
security-plan · fct-hrr3a26v@1 · Resolve and hit testing may run with no DOM and return early with a warning, and a UI event on a document with no target is ignored.
security-plan · fct-c4rr3jfv@1 · Event handling may ask for the document while it is mutably borrowed and gets None instead of a panic.
security-plan · fct-p2nck7ms@1 · Aborting the configured AbortSignal cancels every in-flight fetch tied to the document.
security-plan · fct-3vrrikft@1 · Sub-document navigations route back to the parent document instead of navigating the host.
security-plan · fct-upgu656r@1 · The select list's public readers and writer read a row with no styles as None and add no panic path.
security-plan · fct-wogfa4ah@1 · The render-path recorder holds no unwrap, expect or panic and has no slicing panic path.
security-plan · fct-64ubemwx@1 · A select removed, disabled, hidden or made inert with its list open closes the list with no panic, and a dropped option is no row.
security-plan · fct-5onuddg3@1 · A failed, empty or non-CSS stylesheet does not block render, and a net handler wrapper that does not forward error drops the failure silently.
security-plan · fct-36ipyrv2@1 · An open select list's highlighted row is in no snapshot and no tree, and the lint does not read its contrast.
security-plan · fct-dxkboam6@1 · Every id-taking verb and the actions read meet one shared target check, so what the actions read lists agrees with what a call would be refused.
security-plan · fct-cfr7fwzo@1 · The actions read runs nothing and changes no snapshot, focus reading or diff, so it can be called without moving the session.
security-plan · fct-lnusqzqb@1 · The :open pseudo-class matches a details or dialog by its open attribute's presence, kept current by attribute writes and removals.
security-plan · fct-rc3jl2xp@1 · Handler coverage relies on the reached record holding the stable id and event of each node whose listeners held a dispatched event, deduplicated, so a handler whose run removes its element still counts.
security-plan · fct-6dak5se6@1 · The unkeyed-actionable reading and the actionable-nodes reading both read one actionable rule: focusable, an interactive role, or current listeners holding an event other than `mounted`.
security-plan · fct-pnrbnsjg@2 · Each public select reader resolves its node so a stale id reads none, and the closed-select reader answers none for a select whose list is open.
security-plan · fct-exdlckxb@1 · Degenerate aspect ratios are discarded before layout uses them.
security-plan · fct-o56d7tkg@1 · The text extent reader is read-only and answers none for an id that does not resolve or a node with no inline layout.
security-plan · fct-pwj6p2fo@1 · Geometry whose transformed bounds are non-finite or beyond the f32 range is culled before painting.
security-plan · fct-gukbahaa@1 · Scroll targets are clamped to the scrollable range, and a programmatic scroll to a missing node does nothing.
security-plan · fct-q43mkq7y@1 · Form submission excludes controls under a datalist, disabled controls, non-submitter buttons and unchecked checkboxes and radios.
security-plan · fct-3atajnay@1 · The Dioxus event handler records a dispatch only where a signal registry is registered, and a page with none dispatches as before.
security-plan · fct-et47i5mq@1 · Dioxus dangerous_inner_html is applied as inner HTML.
security-plan · fct-wlz77kgt@1 · A falsy value of a listed boolean attribute clears the attribute rather than writing it.
security-plan · fct-73s6fq52@1 · Script reads and writes CSS properties through named accessors on CSSStyleDeclaration, one per Stylo property name.
security-plan · fct-62emqkgs@1 · Stored URLs are re-parsed on load and unparsable rows are skipped.
security-plan · fct-txc2a4of@1 · Serialization escapes text and attribute values but writes text inside raw-text elements such as style and script unescaped.
security-plan · fct-4ck7btcl@1 · A failed load's error text stops in the load response and the redacted Resource load failed line, and no carrier takes it.
security-plan · fct-ksl3fslx@1 · A select list's option and group labels leave only as painted pixels and the snapshot's existing name, the list holding no log site, span or field.
security-plan · fct-4cylhur3@1 · The render-path mutation buffer logs nothing itself and its replay re-fires the mutation writer's own trace calls, so the log stream is the one the direct write gives.
