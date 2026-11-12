fct-j5k4uy7t@1 · A failed, empty or non-CSS stylesheet stops blocking render, because a failed fetch's error answer takes the request out of the pending critical resources.
fct-54e4fuk7@1 · Every shell window event is followed by a Poll.
fct-aj6cmiwc@1 · Launch paths built with `net` off wasm32 enter a tokio multi-thread runtime before the event loop runs.
fct-35cb7qc2@1 · Each JS entry point requests a redraw and re-arms the timer thread that wakes the event loop at the next JS timer deadline.
fct-2r7bauw3@1 · Headless loading resolves the document repeatedly until the net provider is empty, then resolves once more.
fct-dajqtqwx@1 · The WPT runner resets the net provider before each test and runs each test inside a panic catch, a panic yielding Unknown or Crash.
fct-z4p5ffso@1 · Each browser document load aborts the previous one, as reload and drop do.
fct-6decztyx@1 · Legacy presentational attributes map to CSS at the presentational-hints level.
fct-2es3quc6@1 · A `Node` holds a raw pointer to its `NodeTree` and is marked `Send` and `Sync` by unsafe impl.
fct-kknd7ewa@1 · WPT failures are bucketed by feature-usage flags computed by regex over test sources.
fct-4mapcv7w@1 · A text input that holds focus before its editor exists gets the shell provider's IME enabled when its editor is constructed.
fct-biqhin7y@1 · The first graph-state hook of a page provides a `SignalRegistry` context at the root scope, which the bridge's dispatch looks up; a page that registers no state has none.
fct-tzrr5o2d@1 · The bridge's dispatch finds the reached record by a root-context lookup, and a document never switched on has no record.
fct-3kximnte@1 · A queued mounted event whose element was removed since it was queued is skipped and never fires.
fct-56u6r4qr@1 · Where a signal registry stands, each signal write is recorded against the dispatch that caused it, and a page with no registered state dispatches unchanged.
fct-xcqj3bw6@1 · An open select list closes with no event on Escape, a second click, a press or wheel outside, a scroll or move of the select's box, and on removal, disabled, hidden or inert.
fct-t7ezjz4v@1 · On an open select list Tab chooses the current row and closes before focus moves, and the move keys shift the highlight without choosing.
fct-hxuv6j35@1 · A multiple select's arrow, Home and End keys choose one row alone, with Shift spanning from the anchor and Ctrl or Meta moving the current row without changing the choice.
fct-jyueaf6l@1 · A plain click on a multiple select's row chooses it alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing.
fct-75lrk5x7@1 · A replaced element's natural height follows its usable aspect-ratio, clamped at 0.
fct-3oxpzjue@1 · Every box-generating ::before or ::after votes in flow classification, so a pseudo-only inline-block exports its line baseline.
fct-kvuycvyd@1 · A text input already focused when its editor is constructed enables IME.
fct-jzkmudp4@1 · A focus move keeps focus-within on the shared ancestors, requests a redraw, and marks exactly the node that lost focus and the node that gained it as changed.
fct-qcw5degj@1 · The accessibility tree gives an optgroup the group role named by its non-empty label.
fct-wau2zivr@1 · In an open select list a group's label paints as a header that chooses nothing.
fct-7bd54l7n@1 · A multiple select's size hints a height of size times 1.2em, capped at 65534, with vertical overflow scrolling.
fct-76r3ni6k@1 · A hit test and the paint tree's walk skip a node dropped since the last resolve.
fct-nhbeifug@1 · :open matches a details or dialog that carries the open attribute.
fct-stpw6pf4@1 · The windowed shell's redraw requests travel through its event loop and wake it.
fct-5k5i3eup@1 · An element's border box and culling stand on the scaled size, its location kept for the transform.
fct-gosirpve@2 · A task polled during render acts on the document as the previous render left it, so a node created in the same render cannot yet take focus.
fct-xlm5bxtz@1 · A node's owning component is read through Dioxus 0.7.10's public API, a deleted row resolving to none.
fct-zmj47ciz@1 · A tab's lenses survive a replacement of its webview props.
fct-b5fra4pv@1 · The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network.
fct-qz4rtwlm@2 · The stand's LoadNet answers each fetch from a thread of its own with the fixed CSS after 200 ms, with no socket or listener.
fct-oiwxbp2o@1 · Stylo's prefs are set once per process before the user-agent sheet is parsed.
fct-h2vgfhd5@1 · Every read of a select list's state goes through one reader, with only reconcile and the removal arm reading past it.
fct-udw5de5d@1 · Resolve reconciles the select list after layout and before refreshing hover, and removing a select clears its list.
fct-vgemfdm2@1 · Removing multiple from a select through the mutator settles its chosen set to one option.
fct-oti3svjj@1 · Attribute and child-list mutations restyle only what they affect, with no blanket subtree restyle.
fct-dh6z3pmy@1 · The driver's in-view check and crop read the visible region through transforms, a rotation reading the rotated box's bounding box.
fct-bq34zg7d@1 · The hit, the driver's covered check, hover and pointerdown pass over child nodes dropped since the last resolve, and an open select list answers a hit first.
fct-jypzod7u@1 · A subtree under a non-invertible transform answers no hit.
fct-czgfalox@1 · An open select list shows at most 20 rows, opens where its rows fit or on the roomier side, and a select with no row to show opens nothing.
fct-yn7idpix@1 · The layout pass reads a node's out-of-flow state from the flag recorded by the style traversal.
fct-wazi5pch@1 · Stylo's class matching reads the element's class atoms, kept in sync on every class attribute write.
fct-lg3bylgz@1 · The :open pseudo-class matches a details or dialog carrying an open attribute in every document.
fct-z2rkxrmy@1 · An open select list paints last, past the blend layers.
fct-stih5qpt@1 · The windowed shell's redraw requests travel through its event loop.
fct-hczk2fzd@1 · The load page's Load button links one stylesheet, and the Loaded paragraph stays hidden until that sheet lands.
fct-dl75efux@1 · The load page's network stand-in answers its one stylesheet after 200 ms and is counted as a load by the harness.
fct-igzdfobd@1 · Handler coverage relies on the replay calling a hook after each replayed step, so the screen can be read between steps.
fct-uzcbujjs@1 · A page with no registered state reaches no registry and dispatches events as before.
fct-2ylcpw3b@1 · The cells-grid is one Tab stop whose arrows move the current cell and whose Enter opens the editor, an editor Enter committing and returning focus to the grid.
fct-67g4co3e@2 · The bridge's event attribute stays whether a listener leaves the attribute list or turns to None.
