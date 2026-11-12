@row_add n=1 row=decision/falsy-boolean-attribute-cleared area=standard-contracts rec=1
name: Falsy boolean attributes cleared
rule: A falsy value of one of the 28 boolean attribute names of upstream's predicate clears the attribute on both the dynamic and the static template path; a truthy value is still written, and an unlisted attribute holding false is written as "false".
why: The project's own list of 27 names was retired for upstream's predicate, leaving the mutation writer identical to upstream's.
guard: dioxus_falsy_boolean_attrs
@row_add n=2 row=contract/stable-element-id/element-id area=standard-contracts rec=1
boundary: Stable element ids
item: element_id
promise: Returns an element's stable id, None for a non-element, stale or detached node, and never panics; the id is computed on demand and written to neither the DOM nor the vdom.
@row_add n=3 row=contract/stable-element-id/element-ids area=standard-contracts rec=1
boundary: Stable element ids
item: element_ids
promise: Returns every element reachable from the document root, in document pre-order, with ids pairwise distinct.
@row_add n=4 row=contract/stable-element-id/author-key area=standard-contracts rec=1
boundary: Stable element ids
item: author key tier
promise: The first tier: the HTML id verbatim when non-empty, free of / and the first in document pre-order to claim it.
@row_add n=5 row=contract/stable-element-id/anchored-path area=standard-contracts rec=1
boundary: Stable element ids
item: anchored path tier
promise: The second tier: for an element under an element of its own component instance that reads an author key, that key, //, then one element segment per DOM level below it, the nearest keyed element anchoring.
@row_add n=6 row=decision/anchored-id-tier area=standard-contracts rec=1
name: Anchored id tier reaches one component
rule: An author key anchors the ids of elements below it only inside its own component instance, spelled key//segment; a template root owned by another component restarts at that component's chain.
why: The anchored tier, its key//segment spelling and its one-component reach are ratified.
@row_add n=7 row=contract/stable-element-id/component-path area=standard-contracts rec=1
boundary: Stable element ids
item: component path tier
promise: The third tier: /-joined component segments below the app root, framework scopes adding none and a repeated name reading {name}:{k}, then one element segment per DOM level, {tag}[{key}] for a keyed template root else {tag}:{n}.
@row_add n=8 row=contract/stable-element-id/document-path area=standard-contracts rec=1
boundary: Stable element ids
item: document path tier
promise: The fourth tier, for an element no component renders: / then {tag}:{n} per DOM level from the document root.
@row_add n=9 row=contract/stable-element-id/tier-distinctness area=standard-contracts rec=1
boundary: Stable element ids
item: tier distinctness
promise: No id of one tier equals an id of another: an author key holds no /, only an anchored path holds //, and a document or app-root path starts with one / with no empty segment after it.
@row_add n=10 row=contract/stable-element-id/virtual-node-id area=standard-contracts rec=1
boundary: Stable element ids
item: virtual_node_id
promise: A reserved fifth form {owner}///{key}, which no tier produces, is spelled by the public VIRTUAL_KEY_SEPARATOR and virtual_node_id, which answers None for an empty or /-bearing key.
@row_add n=11 row=contract/stable-element-id/no-process-local-values area=standard-contracts rec=1
boundary: Stable element ids
item: process-local values
promise: No NodeId, ElementId, ScopeId or pointer enters an id.
@row_add n=12 row=contract/stable-element-id/code-edit-stability area=standard-contracts rec=1
boundary: Stable element ids
item: stability across code edits
promise: An element's id holds under edits of text, attributes, classes, handlers and edits outside its keyed element; an earlier same-tag sibling or wrapper inserted before an unkeyed element's path start changes that element's id and no keyed element's.
@row_add n=13 row=contract/stable-element-id/persistence area=standard-contracts rec=1
boundary: Stable element ids
item: persistence
promise: Every element present before and after a re-render reads the same id; a remount reads the same id list in order, and a second process reads the same id lists.
@row_add n=14 row=contract/stable-element-id/removed-node area=standard-contracts rec=1
boundary: Stable element ids
item: removed node
promise: A node a re-render removes is dropped with its descendants and no longer resolves: element_id reads None and element_ids holds no pair for it.
@row_add n=15 row=contract/accessibility-tree/author-id area=standard-contracts rec=1
boundary: Accessibility tree
item: author_id
promise: Under the accessibility feature each element node's AccessKit author_id is its stable id; text runs, the document root and the synthetic Window carry none.
@row_add n=16 row=decision/accessibility-feature-gate area=standard-contracts rec=1
name: Snapshot readers behind the accessibility feature
rule: A crate taking a Dioxus crate with default features off gets the snapshot, its text, its diff and the actionable-key check only by naming the accessibility feature or depending on a crate that names it.
@row_add n=17 row=contract/snapshot/snapshot-node area=standard-contracts rec=1
boundary: Snapshot
item: SnapshotNode
promise: A SnapshotNode carries id, role, name, state, bounds and children; a Snapshot offers nodes in pre-order and get by stable element id.
@row_add n=18 row=contract/snapshot/node-state area=standard-contracts rec=1
boundary: Snapshot
item: NodeState
promise: NodeState carries eight fields in the order enabled, checked, value, selected, pressed, invalid, focused, clickable.
@row_add n=19 row=contract/snapshot/membership area=standard-contracts rec=1
boundary: Snapshot
item: node membership
promise: A node is an accessibility tree node with an author_id whose element resolves; the Window, the document root and text runs are not nodes, their kept descendants attaching to the nearest kept ancestor.
@row_add n=20 row=contract/snapshot/name area=standard-contracts rec=1
boundary: Snapshot
item: name
promise: The node's non-empty label, else the concatenated names of its labelled_by targets, cycle-guarded and trimmed, else the empty string.
@row_add n=21 row=contract/snapshot/bounds area=standard-contracts rec=1
boundary: Snapshot
item: bounds
promise: A row's or group's place in an open list, else the element's viewport-relative, unclamped client bounding rect.
@row_add n=22 row=decision/transformed-element-bounds area=standard-contracts rec=1
name: Transformed element bounds
rule: An element under a transform reads its bounds where its transforms move it, so a driver click on a translated button is accepted and lands.
why: Ratified.
guard: stand_act_transformed
@row_add n=23 row=decision/scrolled-box-bounds area=standard-contracts rec=1
name: Scrolled box bounds
rule: A scrolled box reads the rect it stands at, the same before and after its content is scrolled; only the scroll offsets of boxes above it on the containing-block chain move it.
why: Fixed on a ratified ruling.
guard: scrolled_box_client_rect
@row_add n=24 row=contract/snapshot/enabled area=standard-contracts rec=1
boundary: Snapshot
item: enabled
promise: Reads the presence of the disabled attribute on an element that can be disabled, so disabled="false" and a bare disabled both read Some(false); aria-disabled moves no reading.
@row_add n=25 row=contract/snapshot/checked area=standard-contracts rec=1
boundary: Snapshot
item: checked
promise: A native checkbox's or radio's checked state, else, on one of the five checkable roles, whether its toggle is True; None where the role states nothing.
@row_add n=26 row=decision/mixed-toggle-reads-unchecked area=standard-contracts rec=1
name: Mixed toggle reads unchecked
rule: A checkable-role node whose toggle is mixed reads checked as Some(false).
why: Ratified.
@row_add n=27 row=contract/snapshot/value area=standard-contracts rec=1,11
boundary: Snapshot
item: value
promise: A select's chosen option value, unmasked (a multiple select's first chosen), else the editor text, else a range input's current value, else the value attribute of a textarea or text-entry input.
@row_add n=28 row=decision/masked-input-values area=standard-contracts rec=1
name: Password and file input values masked
rule: A password or file input's value reads MASKED_VALUE when non-empty and empty when empty, never the text, its length or a file's host path.
@row_add n=29 row=contract/snapshot/selected area=standard-contracts rec=1
boundary: Snapshot
item: selected
promise: The accessibility node's own selected state; a select's option reads its chosen state.
@row_add n=30 row=contract/snapshot/pressed area=standard-contracts rec=1
boundary: Snapshot
item: pressed
promise: The node's toggle off the five checkable roles, only True reading Some(true), False and Mixed reading Some(false); never read on a native checkable control.
@row_add n=31 row=contract/snapshot/invalid area=standard-contracts rec=1
boundary: Snapshot
item: invalid
promise: True where the accessibility node carries any invalid value, a grammar or spelling error included.
@row_add n=32 row=contract/snapshot/focused area=standard-contracts rec=1
boundary: Snapshot
item: focused
promise: The accessibility tree update's focus, never a reading that falls back to the root element.
@row_add n=33 row=contract/snapshot/clickable area=standard-contracts rec=1
boundary: Snapshot
item: clickable
promise: True where the element's current listeners hold click and the element is no disabled form control.
@row_add n=34 row=contract/snapshot/no-process-local-values area=standard-contracts rec=1
boundary: Snapshot
item: process-local values
promise: No snapshot field holds a NodeId, an AccessKit tree id, an ElementId, a ScopeId or a pointer.
@row_add n=35 row=decision/snapshot-text-never-logged area=standard-contracts rec=1
name: Snapshot text never logged
rule: The snapshot text leaves the process only as a returned value or a hosted call's answer; no log or event carries it, and a kept check holds it only in an owner-only file never logged.
why: Ratified.
except: The two committed checks are the one exception to owner-only.
@row_add n=36 row=contract/snapshot-text/lines area=standard-contracts rec=1
boundary: Snapshot text
item: lines
promise: One line per node in pre-order, each ended by a line break, a child indented two spaces deeper than its parent; a snapshot with no roots is the empty string.
@row_add n=37 row=contract/snapshot-text/line-fields area=standard-contracts rec=1
boundary: Snapshot text
item: line fields
promise: A line reads the role in its Debug spelling, the quoted name, id= and the quoted id, the state tokens, then @{x},{y} {width}x{height} with each number in f64's Display form.
@row_add n=38 row=contract/snapshot-text/state-tokens area=standard-contracts rec=1
boundary: Snapshot text
item: state tokens
promise: In fixed order enabled/disabled, checked/unchecked, value=, selected/unselected, pressed/unpressed, invalid, focused, clickable; absent where the model holds nothing, invalid and clickable written only when true.
@row_add n=39 row=contract/snapshot-text/quoting area=standard-contracts rec=1
boundary: Snapshot text
item: quoted strings
promise: A quoted string is written in str's Debug form, so it never holds a line break or a bare quote.
@row_add n=40 row=contract/snapshot-text/to-text area=standard-contracts rec=1
boundary: Snapshot text
item: to_text
promise: The snapshot's one text form, a pure function of the Snapshot with no Display impl, no second entry point, no parser and no JSON form; it never truncates or pages.
@row_add n=41 row=contract/snapshot-text/text-parts area=standard-contracts rec=1
boundary: Snapshot text
item: text_parts
promise: Cuts the text into parts of consecutive whole lines within budget bytes, a longer line a part alone, an empty snapshot one empty part; the parts concatenated equal to_text.
@row_add n=42 row=decision/snapshot-text-budget area=standard-contracts rec=1
name: Snapshot text budget
rule: One screen's text stays under SNAPSHOT_TEXT_BUDGET, 10,000 bytes, a ceiling callers compare against and text_parts cuts within, never enforced by to_text.
why: Keeps a screen inside the agent client's documented 10,000-token warning, 25,000-token maximum and 50,000-character file threshold.
@row_add n=43 row=contract/snapshot-diff/structure area=standard-contracts rec=1
boundary: Snapshot diff
item: SnapshotDiff
promise: added and changed hold flat nodes read after the step, each with its parent's id, role, name, state, bounds and differs; removed holds ids.
@row_add n=44 row=contract/snapshot-diff/membership area=standard-contracts rec=1
boundary: Snapshot diff
item: added, removed, changed
promise: A node is added when its id is only after, removed when only before, changed when in both and its role, name, state, bounds or parent's id differ; a descendant's change never names its ancestor.
@row_add n=45 row=contract/snapshot-diff/order area=standard-contracts rec=1
boundary: Snapshot diff
item: order
promise: added and changed follow the after snapshot's pre-order, removed the before snapshot's; where two nodes carry one id the first in pre-order stands for it.
@row_add n=46 row=contract/snapshot-diff/differs area=standard-contracts rec=1
boundary: Snapshot diff
item: differs
promise: The readings that differ from before the step, in Reading::ALL order, empty for an added node; a node that only moved reads Bounds.
@row_add n=47 row=contract/snapshot-diff/reading area=standard-contracts rec=1
boundary: Snapshot diff
item: Reading
promise: Twelve variants: Parent, Role, Name, Enabled, Checked, Value, Selected, Pressed, Invalid, Focused, Clickable, Bounds, with Reading::ALL.
@row_add n=48 row=contract/snapshot-diff/values area=standard-contracts rec=1
boundary: Snapshot diff
item: values
promise: Values come from the two snapshots only, so a password or file input reads MASKED_VALUE, and no field holds a process-local id or pointer.
@row_add n=49 row=decision/diff-not-engine-changed-set area=standard-contracts rec=1
name: Diff separate from the engine's changed set
rule: The diff compares two snapshots and is not derived from the engine's changed set; a set changed flag beside an empty diff is a designed case.
why: The changed set is the cheap signal that something was written; the diff is what a snapshot reads differently.
@row_add n=50 row=decision/diff-content-never-logged area=standard-contracts rec=1
name: Diff content never logged
rule: A diff's content never reaches a log or event; only its three counts of added, removed and changed may be recorded on the driver's command span.
why: Ratified.
@row_add n=51 row=contract/driver-json/diff area=standard-contracts rec=1
boundary: Driver JSON answer
item: diff
promise: added and changed are written as nodes keyed by NODE_FIELDS, a changed node carrying differs after bounds, and removed as ids.
@row_add n=52 row=decision/actionable-elements-keyed area=standard-contracts rec=1
name: Every actionable element reads an author key
rule: Every element an agent can act on reads an author key; unkeyed_actionable is the check that names each actionable element whose id holds a /.
why: A ratified public contract addition.
guard: stand_actionable_keys
@row_add n=53 row=contract/unkeyed-actionable/unkeyed-actionable area=standard-contracts rec=1
boundary: Actionable-key check
item: unkeyed_actionable
promise: Read-only and argument-free, returns every actionable element whose id holds a / in document pre-order, empty when every one reads its key; logs nothing and never panics.
@row_add n=54 row=contract/unkeyed-actionable/remedy area=standard-contracts rec=1
boundary: Actionable-key check
item: remedy
promise: Names the fix, an HTML id that is non-empty, holds no / and is unique; no field holds the element's text content, accessible name or attribute value.
@row_add n=55 row=contract/actionable-element/definition area=standard-contracts rec=1,7
boundary: Actionable element
item: definition
promise: An element is actionable when it is focusable, its role is one of the closed interactive roles, or its current listeners hold an event other than mounted; unkeyed_actionable and actionable_nodes read this one rule.
@row_add n=56 row=decision/keyed-wire-form area=standard-contracts rec=1,17
name: keyed as the actionable-key wire form
rule: ActionableNode's keyed reading, true where the id holds no /, is the actionable-key check's first wire form, carried by the actions verb on every carrier.
state: provisional
@row_add n=57 row=decision/render-with-document-unborrowed area=standard-contracts rec=2
name: Render with the document unborrowed
rule: The vdom renders into a mutation buffer with the document unborrowed; one borrow then replays the buffer and is dropped before mounted events flush.
why: So user code reaching the document through MountedData during a render hits no RefCell already-borrowed panic.
@row_none n=1 rec=3 type=4
relied: A page with no registered state reaches no registry and dispatches events as before.
@row_add n=58 row=decision/reached-record-before-dispatch area=standard-contracts rec=4
name: Reached record written before dispatch
rule: The reached record takes each node of the dispatch chain whose current listeners hold the event before the event runs, holding no borrow across the render.
why: So a handler whose own run removes its element still counts.
@row_none n=2 rec=5 type=4
relied: The cells-grid is one Tab stop whose arrows move the current cell and whose Enter opens the editor, an editor Enter committing and returning focus to the grid.
@row_add n=59 row=decision/listeners-read-from-vdom area=standard-contracts rec=6
name: Listeners read from the vdom walk
rule: Every reader of an element's listeners reads the vdom walk's current listeners, never the bridge's listener mark.
why: The listener mark over-reports: a listener a re-render removes leaves its attribute and mark standing, and a mounted listener writes none.
@row_none n=3 rec=8 type=5
@row_none n=4 rec=9 type=4
relied: The bridge's event attribute stays whether a listener leaves the attribute list or turns to None.
@row_add n=60 row=contract/dioxus-native-dom-api/is-masked-input area=standard-contracts rec=10
boundary: dioxus-native-dom public API
item: is_masked_input
promise: Public and re-exported beside MASKED_VALUE: true for an input whose type reads password or file, ASCII case-insensitive; the test the snapshot's value reader and the driver's recorder read.
@row_add n=61 row=contract/dioxus-form-events/select-change area=standard-contracts rec=11
boundary: Dioxus form events
item: select Input and Change
promise: A select's Input and Change reach the app as form data carrying its value, so an onchange handler fires.
@row_add n=62 row=contract/snapshot/closed-select-option area=standard-contracts rec=11
boundary: Snapshot
item: closed select option
promise: An option of a closed select, under an optgroup too, reads its select's client rect as bounds and is the select's child, named by its label; while the list is open a row reads its own place in the list.
@row_add n=63 row=contract/snapshot-node-keys/expanded area=standard-contracts rec=12
boundary: Snapshot node keys
item: expanded
promise: The snapshot carries no expanded key and NodeState no field for a list; NODE_FIELDS keeps fourteen keys.
@row_add n=64 row=contract/snapshot/multiple-select-bounds area=standard-contracts rec=13
boundary: Snapshot
item: multiple select rows
promise: A multiple select's rows and groups read their own client rects as their bounds.
@row_add n=65 row=contract/dioxus-form-events/multiple-select-values area=standard-contracts rec=14
boundary: Dioxus form events
item: multiple select values
promise: On Input and Change a multiple select fills values with one text entry per chosen option, so evt.values() reads the set, while value stays the select's value.
@row_add n=66 row=decision/stale-mounted-event-skipped area=standard-contracts rec=15
name: Stale mounted event skipped
rule: A mounted event queued for an element removed before the flush is skipped.
why: Differs from upstream: upstream's test for it is left out, as Dioxus 0.7 delivers one mounted event where the test asserts none.
@row_add n=67 row=contract/ui-graph/graph area=standard-contracts rec=16
boundary: UI graph
item: graph
promise: Read-only under accessibility, returns one node per snapshot node in pre-order, each with id, owner, component, handlers and reads, and states with name, component, declared, writers and readers.
@row_add n=68 row=contract/ui-graph/write-flag area=standard-contracts rec=16
boundary: UI graph
item: WriteFlag
promise: A write is flagged Undeclared or Unconfirmed, each with its word().
@row_add n=69 row=contract/actionable-nodes/actionable-nodes area=standard-contracts rec=17
boundary: Actionable nodes
item: actionable_nodes
promise: Read-only under accessibility, returns id, owner and keyed for each UI graph node the actionable rule holds for or whose owner is set, in snapshot pre-order; keyed is true where the id holds no /.
@row_add n=70 row=contract/ui-graph/owner area=standard-contracts rec=18
boundary: UI graph
item: owner
promise: The stable id of the select an option or an optgroup belongs to.
@row_add n=71 row=contract/ui-graph/repeated-registration area=standard-contracts rec=18
boundary: UI graph
item: repeated registration
promise: A repeated name and component registration reads {name}:{k}.
@row_add n=72 row=contract/ui-graph/readers area=standard-contracts rec=18
boundary: UI graph
item: readers
promise: A state's readers name each reading component once, in the pre-order of its first node; a state only the app root reads is named in its nodes' reads alone.
@row_add n=73 row=contract/ui-graph/writers area=standard-contracts rec=18
boundary: UI graph
item: writers
promise: Lists only writes a run confirmed or flagged, while declared keeps the declaration as authored.
@row_add n=74 row=contract/ui-graph/write-resolution area=standard-contracts rec=18
boundary: UI graph
item: write resolution
promise: A write resolves to the first element of its chain carrying a listener of its event, or to nothing when an earlier node of the chain no longer resolves.
@row_add n=75 row=contract/ui-graph/unconfirmed area=standard-contracts rec=18
boundary: UI graph
item: unconfirmed
promise: Fires only once a dispatch of the handler's event passed through its element and no run wrote the state.
@row_add n=76 row=contract/graph-registry/use-graph-state area=standard-contracts rec=19
boundary: Graph registry
item: use_graph_state
promise: Takes a name, declared writes and subscribers of dioxus-core types alone; the first registering hook provides the registry at the root scope, and each entry leaves with its scope.
@row_add n=77 row=contract/graph-registry/declared-write area=standard-contracts rec=19
boundary: Graph registry
item: DeclaredWrite
promise: Names an element's stable id or a prefix ending in *, and an event written without on.
@row_add n=78 row=contract/graph-registry/debug area=standard-contracts rec=19
boundary: Graph registry
item: Debug
promise: The registry's Debug prints counts alone.
@row_add n=79 row=contract/reached-record/reached-handler area=standard-contracts rec=20
boundary: Reached record
item: ReachedHandler
promise: Carries id and event, re-exported from the crate root.
@row_add n=80 row=contract/reached-record/record-reached area=standard-contracts rec=20
boundary: Reached record
item: record_reached
promise: Switches the record on at the root scope; a second call keeps the one record.
@row_add n=81 row=contract/reached-record/reached-handlers area=standard-contracts rec=20
boundary: Reached record
item: reached_handlers
promise: Returns the reached handlers in first-reached order, deduplicated by id and event, empty when the record was never switched on.
@row_add n=82 row=contract/reached-record/debug area=standard-contracts rec=20
boundary: Reached record
item: Debug
promise: The record's Debug prints its count alone.
@row_add n=83 row=contract/example-clis/paint-bench area=standard-contracts rec=21
boundary: Example CLIs
item: paint_bench
promise: paint_bench <url> [width] [height] [scale] [iters] [backend], defaulting to https://servo.org, 1366, 768, 2.0, 100, vello; reports min, median, mean and max in µs for paint_scene and rasterize.
@row_add n=84 row=contract/example-clis/screenshot area=standard-contracts rec=21
boundary: Example CLIs
item: screenshot
promise: Takes a URL (default https://www.google.com) and a width (default 1200); writes an RGBA 8-bit PNG at 144 per inch, height capped at 4000 CSS px, named by the URL's first 12 ASCII-alphanumeric characters.
@row_add n=85 row=contract/example-clis/url area=standard-contracts rec=21
boundary: Example CLIs
item: url
promise: An unparsable URL is retried with https://, and a file URL is read from disk.
@row_add n=86 row=contract/example-clis/preact-script area=standard-contracts rec=21
boundary: Example CLIs
item: preact_script
promise: preact_script [path/to/file.html], defaulting to examples/preact/index.html.
@row_add n=87 row=contract/example-clis/wgpu-texture area=standard-contracts rec=21
boundary: Example CLIs
item: wgpu_texture
promise: wgpu_texture takes an --html flag.
@row_add n=88 row=contract/app-clis/browser area=standard-contracts rec=21
boundary: App CLIs
item: browser
promise: The first argument parsing as a URL or dotted host becomes the first tab.
@row_add n=89 row=contract/app-clis/rdme area=standard-contracts rec=21
boundary: App CLIs
item: rdme
promise: Takes a URL or path, default the current directory, resolving to the nearest README.md.
@row_add n=90 row=contract/app-clis/wpt area=standard-contracts rec=21
boundary: App CLIs
item: wpt
promise: Non-dash arguments are suite paths, full runs all suites; flags --verbose or -v, --run-quarantined and --list; WPT_DIR is required.
@row_add n=91 row=contract/escher-session/invocation area=standard-contracts rec=21
boundary: escher-session command line
item: invocation
promise: escher-session <verb> [arguments] --session <dir> drives the stand's six served tasks: counter, flight-booker, timer, crud, cells, load.
@row_add n=92 row=contract/escher-session/snapshot area=standard-contracts rec=21
boundary: escher-session command line
item: snapshot
promise: The verb snapshot reads the screen.
@row_add n=93 row=contract/escher-session/click area=standard-contracts rec=21
boundary: escher-session command line
item: click --id
promise: The verb click takes --id.
@row_add n=94 row=contract/escher-session/type area=standard-contracts rec=21
boundary: escher-session command line
item: type --id --text
promise: The verb type takes --id and --text.
@row_add n=95 row=contract/escher-session/press area=standard-contracts rec=21
boundary: escher-session command line
item: press --key [--shift]
promise: The verb press takes --key and an optional --shift.
@row_add n=96 row=contract/escher-session/advance area=standard-contracts rec=21
boundary: escher-session command line
item: advance --ms
promise: The verb advance takes --ms.
@row_add n=97 row=contract/escher-session/scroll area=standard-contracts rec=21
boundary: escher-session command line
item: scroll --id
promise: The verb scroll takes --id.
@row_add n=98 row=contract/escher-session/screenshot area=standard-contracts rec=21
boundary: escher-session command line
item: screenshot [--id <id>]
promise: The verb screenshot takes an optional --id.
@row_add n=99 row=contract/escher-session/lint area=standard-contracts rec=21
boundary: escher-session command line
item: lint
promise: The verb lint takes no argument beside --session.
@row_add n=100 row=contract/escher-session/graph area=standard-contracts rec=21
boundary: escher-session command line
item: graph
promise: The verb graph takes no argument beside --session.
@row_add n=101 row=contract/escher-session/actions area=standard-contracts rec=21
boundary: escher-session command line
item: actions
promise: The verb actions takes no argument beside --session.
@row_add n=102 row=contract/escher-session/start area=standard-contracts rec=21
boundary: escher-session command line
item: start <task> [--record <dir>]
promise: The verb start takes a task and an optional --record directory and spawns the host role.
@row_add n=103 row=contract/escher-session/status area=standard-contracts rec=21
boundary: escher-session command line
item: status
promise: The verb status takes no argument beside --session.
@row_add n=104 row=contract/escher-session/stop area=standard-contracts rec=21
boundary: escher-session command line
item: stop
promise: The verb stop takes no argument beside --session.
@row_add n=105 row=contract/escher-session/help area=standard-contracts rec=21
boundary: escher-session command line
item: help
promise: Needs no --session, admits at most one and reads it not, reaches no session, and answers the self-description's one line with status 0.
@row_add n=106 row=contract/escher-session/session-flag area=standard-contracts rec=21
boundary: escher-session command line
item: --session
promise: Every verb but help takes --session <dir> exactly once.
@row_add n=107 row=contract/escher-session/serve area=standard-contracts rec=21
boundary: escher-session command line
item: serve <task> --session <dir> [--record <dir>]
promise: The host role, which the start command spawns.
@row_add n=108 row=contract/escher-session/mcp area=standard-contracts rec=21
boundary: escher-session command line
item: mcp [--record <dir>]
promise: An MCP server on the binary's standard streams whose start tool boots the same six tasks through the same boot.
@row_add n=109 row=contract/escher-session/answer-line area=standard-contracts rec=21
boundary: escher-session command line
item: answer
promise: A command answers one uncoloured line of JSON on stdout.
@row_add n=110 row=contract/escher-session/two-argument-form area=standard-contracts rec=21
boundary: escher-session command line
item: <task> <state-dir>
promise: The two-argument form is gone: its first word is no verb, so it reads refused, unknown-verb, status 1.
@row_add n=111 row=decision/advance-moves-session-clock area=standard-contracts rec=21
name: advance without a time step
rule: A served task with no time step still takes advance, which moves its session's clock alone.
why: Ratified.
@row_add n=112 row=contract/escher-session-exit-codes/0 area=standard-contracts rec=21
boundary: escher-session exit codes
item: 0
promise: Accepted.
@row_add n=113 row=contract/escher-session-exit-codes/1 area=standard-contracts rec=21
boundary: escher-session exit codes
item: 1
promise: Refused.
@row_add n=114 row=contract/escher-session-exit-codes/2 area=standard-contracts rec=21
boundary: escher-session exit codes
item: 2
promise: Usage error: a fixed line on stderr, stdout empty, nothing booted and no state created.
@row_add n=115 row=contract/escher-session-exit-codes/3 area=standard-contracts rec=21
boundary: escher-session exit codes
item: 3
promise: Session error: the error object on stdout and its fixed message on stderr.
@row_add n=116 row=contract/escher-session-host-exit-codes/0 area=standard-contracts rec=21
boundary: escher-session host role exit codes
item: 0
promise: The host role exits 0 with nothing written after a stop or an expiry.
@row_add n=117 row=contract/escher-session-host-exit-codes/3 area=standard-contracts rec=21
boundary: escher-session host role exit codes
item: 3
promise: The host role exits 3 on a session error.
@row_add n=118 row=contract/escher-session-mcp-exit-codes/0 area=standard-contracts rec=21
boundary: escher-session mcp role exit codes
item: 0
promise: The mcp role exits 0 on Ok, writing nothing on stdout at its end.
@row_add n=119 row=contract/escher-session-mcp-exit-codes/3 area=standard-contracts rec=21
boundary: escher-session mcp role exit codes
item: 3
promise: The mcp role exits 3 on Err with the error's fixed message on stderr, writing nothing on stdout at its end.
@row_add n=120 row=contract/example-clis/multi-doc area=standard-contracts rec=22
boundary: Example CLIs
item: multi_doc
promise: Takes a mode and an iteration count, reads MULTI_DOC_N, MULTI_DOC_FLAT and MULTI_DOC_SEQ, prints one summary line on stdout and panics on an unknown mode.
@row_add n=121 row=contract/browser-history/history-entry area=standard-contracts rec=23
boundary: Browser history
item: HistoryEntry
promise: Carries id, url, title, favicon_url and visited_at.
@row_add n=122 row=contract/browser-history/history-store area=standard-contracts rec=23
boundary: Browser history
item: HistoryStore
promise: Offers open, load_recent, record_visit, clear and set_favicon_by_url.
@row_add n=123 row=contract/browser-history/history-entries-table area=standard-contracts rec=23
boundary: Browser history
item: history_entries table
promise: Columns id INTEGER PK AUTOINCREMENT, url TEXT NOT NULL, title TEXT NOT NULL, favicon_url TEXT, visited_at INTEGER NOT NULL, indexed on visited_at DESC and url.
@row_add n=124 row=contract/browser-about-urls/about-newtab area=standard-contracts rec=23
boundary: Browser about URLs
item: about:newtab
promise: The browser serves about:newtab.
@row_add n=125 row=contract/browser-about-urls/about-settings area=standard-contracts rec=23
boundary: Browser about URLs
item: about:settings
promise: The browser serves about:settings.
@row_add n=126 row=contract/browser-about-urls/about-history area=standard-contracts rec=23
boundary: Browser about URLs
item: about:history
promise: The browser serves about:history.
@row_add n=127 row=contract/browser-about-urls/about-bookmarks area=standard-contracts rec=23
boundary: Browser about URLs
item: about:bookmarks
promise: The browser serves about:bookmarks.
@row_add n=128 row=contract/browser-toolbar/view-source area=standard-contracts rec=23
boundary: Browser toolbar
item: View Source
promise: View Source opens view-source:// followed by the page's URL.
