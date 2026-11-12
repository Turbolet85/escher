@row_add n=1 row=contract/recording/directory area=standard-contracts rec=1
boundary: Session recording
item: record directory
promise: A recording makes its directory owner-only or refuses it with `RecordDirNotPrivate`.
@row_add n=2 row=contract/kept-check/opening-record area=standard-contracts rec=1
boundary: Kept check file
item: opening record
promise: A kept check opens with the record `{"tool":"start","arguments":{"app":<label>}}`.
@row_add n=3 row=contract/kept-check/tool area=standard-contracts rec=1
boundary: Kept check file
item: tool
promise: Each answered call kept appends one record whose `tool` is the verb called.
@row_add n=4 row=contract/kept-check/arguments area=standard-contracts rec=1
boundary: Kept check file
item: arguments
promise: A record's `arguments` are written in the verb's schema order; a call naming an argument its schema does not name, or naming one twice, is not kept.
@row_add n=5 row=contract/kept-check/masked area=standard-contracts rec=1
boundary: Kept check file
item: masked
promise: `masked` is written `true` on a `type` record alone; a `type` whose element resolves to a masked control or to none keeps the text `"masked"` and `masked:true`, every other `type` keeps its text as it crossed.
@row_add n=6 row=contract/kept-check/accepted area=standard-contracts rec=1
boundary: Kept check file
item: accepted
promise: Each record states whether the call was accepted.
@row_add n=7 row=contract/kept-check/digest area=standard-contracts rec=1
boundary: Kept check file
item: digest
promise: `digest` is a hand-written FNV-1a 64 (offset basis 0xcbf29ce484222325, prime 0x100000001b3) over the answer's JSON line, written as 16 lowercase hex digits.
@row_add n=8 row=contract/kept-check/answer area=standard-contracts rec=1
boundary: Kept check file
item: answer
promise: `answer` is the answer's line as the driver wrote it, inlined as its own JSON object, masked as on every carrier, kept whole except a shot's pixels: a shot keeps `{"width":…,"height":…}` with no `png`.
@row_add n=9 row=contract/kept-check/kept-verbs area=standard-contracts rec=1
boundary: Kept check file
item: kept verbs
promise: Only calls of the ten instance verbs are kept, `lint`, `graph` and `actions` among them with their answers whole; any other call is not kept.
@row_add n=10 row=contract/kept-check/bounds area=standard-contracts rec=1
boundary: Kept check file
item: size bounds
promise: A kept check holds at most 400 records and 1,048,576 bytes.
@row_add n=11 row=contract/recording/failure area=standard-contracts rec=1
boundary: Session recording
item: recording failure
promise: Past the bounds or on a failed write the recording ends: its check is removed, nothing further is kept, the call is not rolled back, and the session reports the failure once.
@row_add n=12 row=contract/socket-host/recording-failed area=standard-contracts rec=1
boundary: Socket host reply
item: ok v3 recording-failed
promise: The host replies `ok v3 recording-failed` on the call whose recording ended.
@row_add n=13 row=contract/mcp-server/recording-failed area=standard-contracts rec=1
boundary: MCP server tool result
item: RecordingFailed
promise: The MCP server answers a call whose recording ended with a failed tool result (`isError`) carrying `SessionError::RecordingFailed`.
@row_add n=14 row=contract/kept-check-reader/strict-read area=standard-contracts rec=1
boundary: Kept check reader
item: strict read
promise: A kept check is read strictly: at most 1,048,576 bytes, a JSON array of at most 400 records, closed keys, the ten instance verbs, schema argument names, `masked` on `type` alone, `digest` 16 lowercase hex, `answer` a required JSON object; key order is not enforced.
@row_add n=15 row=contract/kept-check-reader/refusal area=standard-contracts rec=1
boundary: Kept check reader
item: malformed refusal
promise: Anything else is refused `Cause::Malformed` with no fault, nothing of the file held and no panic path.
@row_add n=16 row=contract/driver-public-api/no-serde-json-types area=standard-contracts rec=1,2
boundary: Driver public API
item: serde_json types
promise: No public accessor of a check, a step, a replay or a change returns or holds a `serde_json` type.
@row_add n=17 row=contract/check-replay/another-app area=standard-contracts rec=2
boundary: Check replay
item: another app
promise: A replay runs nothing on a session whose label is not the check's app.
@row_add n=18 row=contract/check-replay/first-difference area=standard-contracts rec=2
boundary: Check replay
item: first difference
promise: Each step's call runs through the session, validated again, and the replay stops at the first step whose acceptance differs or whose live answer is not equal to the kept answer.
@row_add n=19 row=contract/check-replay/change-keys area=standard-contracts rec=2
boundary: Check replay
item: change keys
promise: A change names the step verb's result fields that differ in table order, then `refused`; a key no table holds is not named.
@row_none n=1 rec=3 type=4
relied: Handler coverage relies on the replay calling a hook after each replayed step, so the screen can be read between steps.
@row_add n=20 row=contract/lint-verb/finding area=standard-contracts rec=4
boundary: The lint verb
item: finding
promise: The lint returns one finding `{ id, defect, bounds, reason }` per defect read on the current screen, in document order, then defect order.
@row_add n=21 row=contract/lint-verb/defects area=standard-contracts rec=4
boundary: The lint verb
item: defect set
promise: The defect set is closed at six, named `overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`.
@row_add n=22 row=contract/lint-verb/fonts-unavailable area=standard-contracts rec=4
boundary: The lint verb
item: fonts-unavailable
promise: The lint is refused `fonts-unavailable` before anything is read when text cannot be painted.
@row_add n=23 row=contract/lint-verb/changes-nothing area=standard-contracts rec=4
boundary: The lint verb
item: side effects
promise: The lint records each node's id as a snapshot does and runs no step, settle, pump or time move, so it changes nothing.
@row_add n=24 row=decision/lint-own-bounds-type area=standard-contracts rec=4
name: Lint's own rectangle type
rule: Every lint rectangle is the lint's own bounds type in the snapshot bounds' space, and lint types hold no engine node id, page text, colour or value.
why: the driver crate names neither blitz-dom nor stylo
@row_add n=25 row=contract/lint-verb/overlap area=standard-contracts rec=4
boundary: The lint verb
item: overlap
promise: Two nodes, neither an ancestor of the other, whose visible parts intersect, reported once at the highest level on the earlier node; a part scrolled out of view overlaps nothing.
@row_add n=26 row=contract/lint-verb/overlap-select-rows area=standard-contracts rec=4,6
boundary: The lint verb
item: overlap of select options and list rows
promise: A closed select's options, read by the element and never the role, and an open list's rows and groups are skipped by `overlap` on either side; a `multiple` select's rows are linted as usual.
@row_add n=27 row=contract/lint-verb/clipped-off-screen area=standard-contracts rec=4
boundary: The lint verb
item: clipped and off-screen
promise: `clipped` and `off-screen` are read against each node's clip region: its visible region cut by the padding box of every ancestor below the root whose computed `contain` includes `paint`; an open row's region takes no such cut.
@row_add n=28 row=contract/lint-verb/truncated area=standard-contracts rec=4
boundary: The lint verb
item: truncated
promise: A node that is no text control whose text box does not lie inside its clip region cut by its own padding box where it clips; a node clipped or off-screen that holds text also reads `truncated`.
@row_add n=29 row=contract/lint-verb/zero-size area=standard-contracts rec=4
boundary: The lint verb
item: zero-size
promise: A node with no area that holds a name, a value, an `enabled` reading, focusability or text descendants.
@row_add n=30 row=contract/lint-verb/contrast area=standard-contracts rec=4
boundary: The lint verb
item: contrast
promise: A node with direct non-whitespace text or a text control, reported below the required ratio, exempt when disabled; a node past an element with a `mix-blend-mode` other than `normal` is read as not measured.
@row_add n=31 row=contract/lint-verb/wire-form area=standard-contracts rec=4
boundary: The lint verb
item: written form
promise: The `lint` verb answers `{"findings":[…]}` on every carrier and is recorded as a step with its answer kept whole; its `command` span records `verb` and, refused, `cause` alone.
@row_add n=32 row=contract/verb-table/order area=standard-contracts rec=4
boundary: Driver verb table
item: VERBS
promise: The verb table holds fourteen verbs, `graph` listed after `lint` and `actions` after `graph`.
@row_add n=33 row=contract/load-wait/bound area=standard-contracts rec=5
boundary: Driver acting step
item: LOAD_WAIT
promise: An acting step waits up to 5 s of wall time for a load in flight, then returns with `settled` false and `busy` `loads`, at or past the bound and within one settle of it; a `render` or `layout` answer returns at once.
@row_add n=34 row=contract/load-wait/verbs area=standard-contracts rec=5
boundary: Driver acting step
item: waiting verbs
promise: The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph` and `actions` wait on nothing.
@row_add n=35 row=contract/load-wait/no-time-moves area=standard-contracts rec=5
boundary: Driver acting step
item: wait side effects
promise: No time moves on the held instance while it waits: no tick, no time step, no changed-set drain, no print and no tracing call.
@row_none n=2 rec=6 type=5
@row_none n=3 rec=7 type=5
@row_add n=36 row=contract/graph-verb/argument area=standard-contracts rec=8
boundary: The graph verb
item: argument
promise: `graph` takes no argument; an argument is refused `malformed` with nothing run.
@row_add n=37 row=contract/graph-verb/node area=standard-contracts rec=8
boundary: The graph verb
item: node
promise: A node is keyed `id` · `owner` · `component` · `handlers` · `reads` (`owner`, `component` left out when absent); a handler `event` · `writes`; a write `state` · `declared` · `confirmed` · `flag`, `flag` left out when none.
@row_add n=38 row=contract/graph-verb/state area=standard-contracts rec=8
boundary: The graph verb
item: state
promise: A state is keyed `name` · `component` · `declared` · `writers` · `readers` (`component` left out when absent); a declaration `id` · `event`; a writer `id` · `event` · `declared` · `confirmed` · `flag`.
@row_add n=39 row=contract/graph-verb/no-values area=standard-contracts rec=8
boundary: The graph verb
item: content
promise: The answer `{"nodes":[…],"states":[…]}` holds ids, component, state and event names, closed flag words and booleans only, never a value.
@row_add n=40 row=contract/graph-verb/unpaged area=standard-contracts rec=9
boundary: The graph verb
item: paging
promise: `graph` answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound.
@row_none n=4 rec=10 type=5
@row_add n=41 row=contract/actions-verb/target-verbs area=standard-contracts rec=11
boundary: The actions verb
item: target verbs
promise: For each node listed, the read covers each instance verb that takes an id argument, in table order, today `click` · `type` · `scroll` · `screenshot`.
@row_add n=42 row=contract/actions-verb/acceptance area=standard-contracts rec=11,12
boundary: The actions verb
item: acceptance
promise: Each verb's call is built with fixed in-bound values for other required arguments and run through the same validation and per-verb target check a real call meets.
@row_add n=43 row=contract/actions-verb/stale area=standard-contracts rec=11
boundary: The actions verb
item: stale
promise: A node that resolves to no element reads `stale`.
@row_add n=44 row=contract/target-check/click-type area=standard-contracts rec=12
boundary: Per-verb target check
item: click and type
promise: `click` and `type` are refused `disabled`, then `off-screen`, then `covered`.
@row_add n=45 row=contract/target-check/scroll area=standard-contracts rec=12
boundary: Per-verb target check
item: scroll
promise: `scroll` meets no target check.
@row_add n=46 row=contract/target-check/screenshot area=standard-contracts rec=12
boundary: Per-verb target check
item: screenshot with an id
promise: A `screenshot` given an `id` meets in-view, then an empty crop, then `fonts-unavailable`, before anything is painted.
@row_add n=47 row=contract/actions-verb/changes-nothing area=standard-contracts rec=13
boundary: The actions verb
item: side effects
promise: The actions read changes nothing: no changes are pending after it, the snapshot, both focus readings and the diff are unchanged, and an open select list stays open.
@row_add n=48 row=contract/actions-verb/meaning area=standard-contracts rec=14
boundary: The actions verb
item: accepted
promise: `actions` answers whether a call would be accepted, not what it would do.
@row_add n=49 row=contract/actions-verb/unpaged area=standard-contracts rec=14
boundary: The actions verb
item: paging
promise: `actions` takes no argument and answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound.
@row_add n=50 row=contract/handler-coverage/per-check-boot area=standard-contracts rec=15
boundary: Handler coverage reader
item: per-check boot
promise: A fresh session is booted for each check; a check whose app is not the label runs nothing and contributes nothing.
@row_add n=51 row=contract/handler-coverage/reached area=standard-contracts rec=15
boundary: Handler coverage reader
item: reached handlers
promise: The UI graph is read at boot and after each replayed step, `mounted` left out, deduplicated by id and event, and the handlers the replay reached are taken from it.
@row_add n=52 row=contract/handler-coverage/written-form area=standard-contracts rec=15
boundary: Handler coverage reader
item: written form
promise: Coverage is written `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, `owner` left out when absent.
@row_add n=53 row=contract/handler-coverage/canvas area=standard-contracts rec=16
boundary: Handler coverage reader
item: canvas handlers
promise: Handlers inside a canvas are not counted.
@row_add n=54 row=contract/handler-coverage/unreached-screens area=standard-contracts rec=16
boundary: Handler coverage reader
item: unreached screens
promise: A handler on no screen a replay reached is not listed, by construction.
@row_add n=55 row=contract/handler-coverage/bubbling area=standard-contracts rec=16
boundary: Handler coverage reader
item: bubbling dispatch
promise: A bubbling dispatch counts every ancestor listener it passed; a `stopPropagation` is not read.
@row_add n=56 row=contract/cli-coverage-role/shape area=standard-contracts rec=17
boundary: Command line coverage role
item: shape
promise: `coverage <app> <check>...` — an app the binary boots, then one or more kept-check paths, no `--session` or `--record`; the binary boots once per check.
@row_add n=57 row=contract/cli-coverage-role/0 area=standard-contracts rec=17
boundary: Command line coverage role
item: 0
promise: Accepted: the coverage line is printed on stdout.
@row_add n=58 row=contract/cli-coverage-role/1 area=standard-contracts rec=17
boundary: Command line coverage role
item: 1
promise: A file that cannot be read, one past the bound, one that is no check, or a check whose app is not `<app>` is refused `malformed`, holding nothing of the path or the file.
@row_add n=59 row=contract/cli-coverage-role/2 area=standard-contracts rec=17
boundary: Command line coverage role
item: 2
promise: Any other shape is usage.
@row_add n=60 row=contract/cli-coverage-role/3 area=standard-contracts rec=17
boundary: Command line coverage role
item: 3
promise: A boot that fails is the session error.
@row_add n=61 row=contract/cli-usage-line/role-clauses area=standard-contracts rec=18
boundary: Command line usage line
item: role clauses
promise: The usage line reads `; mcp role: mcp [--record <dir>]`, then `; coverage role: coverage <app> <check>...;` before `; apps:`.
@row_add n=62 row=decision/js-engine-boa area=standard-contracts rec=19
name: Boa as the JavaScript engine
rule: Script documents run HTML with JavaScript enabled on a Boa-based engine.
@row_add n=63 row=contract/script-fetcher/errors area=standard-contracts rec=19
boundary: Script fetcher
item: fetch errors
promise: A fetch fails with `UnsupportedScheme`, `Io`, `InvalidData` or `Forbidden` (a `file:` load for a non-`file:` document).
@row_add n=64 row=contract/script-document/first-poll area=standard-contracts rec=19
boundary: Script document
item: script execution
promise: Scripts run on first poll if not run explicitly.
@row_add n=65 row=contract/script-globals/blitz-send-message area=standard-contracts rec=19
boundary: Script globals
item: __blitz_send_message
promise: `__blitz_send_message(message)` queues a message the host drains with `take_messages`.
@row_add n=66 row=contract/script-globals/blitz-fetch-sync area=standard-contracts rec=19
boundary: Script globals
item: __blitz_fetch_sync
promise: `__blitz_fetch_sync(url)` returns `[status, url, text]`; NotFound reads as 404, other failures, a forbidden load among them, throw TypeError.
@row_add n=67 row=contract/script-events/dispatch-order area=standard-contracts rec=19
boundary: Script event handling
item: dispatch order
promise: Events reach JS listeners before Blitz default actions, and JS `preventDefault` / `stopPropagation` are fed back into the event state.
@row_add n=68 row=decision/geometry-transform-parse-stylo area=standard-contracts rec=19
name: Stylo parses geometry transforms
rule: The script geometry interfaces delegate transform parsing to the native side, which uses stylo's CSS parser.
@row_add n=69 row=decision/file-url-load-rule area=standard-contracts rec=20
name: File URLs only for file documents
rule: A `file:` URL may be loaded only by a document whose own URL is `file:`; fetch, module imports and script sources each ask before the fetcher is called and read a refusal as Forbidden.
@row_add n=70 row=contract/default-script-fetcher/schemes area=standard-contracts rec=20
boundary: Default script fetcher
item: schemes
promise: The default fetcher takes only `file:` and `data:`, a `file:` URL only for a `file:` document.
@row_add n=71 row=absence/script-same-origin-cors area=standard-contracts rec=20
capability: Same-origin and CORS checks on script loads
@row_add n=72 row=contract/css-style-declaration/property-accessors area=standard-contracts rec=21
boundary: Script CSSStyleDeclaration
item: property accessors
promise: Each Stylo non-custom property name blitz-dom supports is an accessor on the style prototype, under its own name and its camel-case form.
@row_add n=73 row=contract/script-globals/url-search-params area=standard-contracts rec=22
boundary: Script globals
item: URLSearchParams
promise: `URLSearchParams` is a global class with `URL.prototype.searchParams`, `href` and `search` live both ways.
@row_add n=74 row=contract/script-globals/no-native-parser area=standard-contracts rec=22
boundary: Script globals
item: __blitz_parse_search_params
promise: The native parser `__blitz_parse_search_params` is deleted from `globalThis` before any page script runs.
@row_add n=75 row=contract/dioxus-native-launch/config-downcast area=standard-contracts rec=23
boundary: dioxus-native launch
item: configs
promise: Launch configs are downcast to `Features`, `Limits`, `WindowAttributes` or `Config`; an unmatched config is dropped.
@row_add n=76 row=contract/dioxus-native-document/head-elements area=standard-contracts rec=23
boundary: dioxus-native Document context
item: head elements
promise: The native Document context turns title, meta, script, style and link into `CreateHeadElement` events.
