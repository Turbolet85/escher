@row_add n=1 row=decision/custom-widget-initial-attribute-replay area=input-validation rec=1
name: Custom widget initial attribute replay
rule: Behind the custom-widget feature, setting a custom widget replays each existing attribute, parsed page content included, to the widget with no old value, so the widget receives them at set-up and checking each value stays the widget's own job.
why: An engine change for every document behind the feature, ratified.
guard: custom_widget_replays_initial_attributes
@row_add n=2 row=contract/stable-element-id/author-key area=input-validation rec=2
boundary: Stable element id
item: author key
promise: An element's HTML `id` is its author key only when it is non-empty, holds no `/` and is the first element in document pre-order to claim it; an empty, `/`-bearing or later-duplicate value gives no key and anchors nothing.
@row_add n=3 row=contract/stable-element-id/anchored-path area=input-validation rec=2
boundary: Stable element id
item: anchored path
promise: An element with no author key reads `{key}//{segment}` (then `/{segment}` per further DOM level) under the nearest keyed element of its own component instance, else its component path, else its document path; an anchor acts inside one component only.
@row_add n=4 row=decision/anchored-id-tier area=input-validation rec=2
name: Anchored stable-id tier
rule: An unkeyed element's stable id is anchored under the nearest author-keyed element of its own component instance before falling back to its component path or document path.
why: The anchored tier was ratified.
@row_add n=5 row=contract/stable-element-id/no-collision area=input-validation rec=2
boundary: Stable element id
item: uniqueness
promise: An author key holds no `/`, an anchored path is the only id holding `//`, and a component or document path has no empty segment, so no `id` attribute can make two ids equal, a key spelled like a component name included.
@row_add n=6 row=contract/stable-element-id/unresolved-node area=input-validation rec=2
boundary: Stable element id
item: non-element, stale or detached node
promise: A non-element, stale or detached node reads no id, and no path computation panics.
@row_add n=7 row=contract/stable-element-id/keyed-list-segment area=input-validation rec=2
boundary: Stable element id
item: keyed list row segment
promise: A keyed list row's segment carries its Dioxus key as `{tag}[{key}]`; a Dioxus key that is empty or holds `/` is refused and the row reads the positional `{tag}:{n}` segment instead.
@row_add n=8 row=contract/stable-element-id/stability area=input-validation rec=2
boundary: Stable element id
item: stability
promise: A stable id is never a list index, pointer, hash, clock or process-local value, so a row reads the same id across a re-render, a remount and a second process.
@row_add n=9 row=decision/no-process-handle-crosses area=input-validation rec=2
name: No process-local handle crosses with an id
rule: Neither the stable id, the snapshot, the diff nor any driver session, answer or refusal carries a `NodeId`, `ElementId`, `ScopeId` or pointer; only the stable id text crosses.
@row_add n=10 row=decision/stable-id-not-logged area=input-validation rec=2
name: Stable ids kept out of logs
rule: Escher's own code writes the stable id to no log, and a host that installs escher's sink drops third-party style records carrying an element's HTML id whole, printing that value at no `RUST_LOG` level.
why: Those third-party targets are outside the sink's allowlist.
@row_add n=11 row=decision/author-id-on-accessibility-nodes area=input-validation rec=2
name: Stable id as accessibility author id
rule: Under the accessibility feature each element node of the accessibility tree carries its stable id as its AccessKit author id, never on a text run, the document root or the synthetic window node, so it leaves the process only through the platform adapter.
why: Ratified; the crossing text runs and labels already take.
@row_add n=12 row=decision/accessibility-tree-refresh-on-change area=input-validation rec=2
name: Accessibility tree refreshed on change
rule: The platform accessibility adapter receives the tree on change as well as on its initial request: a poll that took a non-empty changed set rebuilds the platform tree.
why: Ratified: the same crossing and the same content classes, more often.
@row_add n=13 row=decision/accessibility-adapter-content-classes area=input-validation rec=2,8
name: Content classes on the accessibility adapter crossing
rule: The platform accessibility adapter crossing carries four author-derived content classes: element ids, accessible names, control states and descriptions.
why: Each widening ratified; the states by ruling, the names' and descriptions' upstream additions ratified as upstream's behaviour.
@row_add n=14 row=contract/snapshot-text/id-field area=input-validation rec=2
boundary: Snapshot text
item: id field
promise: Each node's id is written as a quoted `id=` field in the string's Debug form, so the field never holds a line break or a bare `"`; the snapshot has one text form and no JSON form, Display form or parser of the text.
@row_add n=15 row=decision/stable-id-exits area=input-validation rec=2
name: Ratified exits of the stable id
rule: The stable id leaves the process only through the platform accessibility adapter, the driver's answers over the session socket and the client's stdout, MCP tool results on the `mcp` role's stdout, and kept check files.
why: Each carrier ratified as a boundary widening.
@row_add n=16 row=decision/kept-checks-owner-only area=input-validation rec=2
name: Kept checks owner-only and unlogged
rule: A recorded session's kept check holds its calls and each answer whole, snapshot text and diffs included, a shot's pixels excepted, in an owner-only file that is never logged.
why: Ratified.
except: The two committed stand checks live in git.
@row_add n=17 row=decision/diff-counts-only-in-logs area=input-validation rec=2
name: Only diff sizes reach a log
rule: A diff's content never reaches a log or event, while its three sizes (counts of added, removed and changed nodes) are recorded on the driver's command span, printed at `info` or below.
why: Ratified: a diff's content never reaches a log while its three sizes may.
@row_add n=18 row=decision/id-lookup-anew-each-call area=input-validation rec=2
name: Caller-supplied id looked up anew
rule: An acting command that takes a caller-supplied id looks it up on the screen as it reads now, on every call, never by index and keeping no node handle between calls; an id that names nothing is refused with nothing run and no panic.
@row_add n=19 row=contract/driver-id-refusal/stale area=input-validation rec=2
boundary: Driver id refusal
item: stale
promise: A call naming an id that no element on the current screen carries is refused as `stale` when the session's record holds that id.
@row_add n=20 row=contract/driver-id-refusal/not-found area=input-validation rec=2
boundary: Driver id refusal
item: not-found
promise: A call naming an id that no element on the current screen carries is refused as `not-found` when the session's record does not hold that id.
@row_add n=21 row=contract/driver-call-id/length-bound area=input-validation rec=2
boundary: Driver call id
item: length bound
promise: An id a call names, from the command-line `--id` flag, the socket's `call` request or an MCP tool's `id` argument, is bounded at 1 to 1024 bytes, checked in the client before a session is touched and again in the host.
@row_add n=22 row=decision/session-id-record-bounded area=input-validation rec=2
name: Bounded in-process record of read ids
rule: A session keeps a private record of ids its own snapshots read, at most 4096 ids of at most 1024 bytes each, the oldest forgotten first, read by no public item and written to no log, event, socket or file.
why: Kept to tell the stale and not-found refusals apart.
guard: the_record_holds_no_id_longer_than_a_call_can_name
@row_add n=23 row=decision/lint-findings-closed-content area=input-validation rec=2
name: Lint findings hold no page content
rule: A lint finding holds ids, closed defect words and numbers only, never page text, colour or a control value, and no log site, print, span or field carries a finding.
why: A ratified boundary widening.
@row_add n=24 row=decision/actions-keyed-field area=input-validation rec=2
name: Keyed flag in the actions answer
rule: The `actions` verb's answer carries, per actionable node, its id, owner id, a `keyed` boolean and closed verb and cause words, no process-local handle.
why: A ratified boundary widening.
state: provisional
@row_none n=1 rec=3 type=4
relied: Handler coverage relies on the reached record holding the stable id and event of each node whose listeners held a dispatched event, deduplicated, so a handler whose run removes its element still counts.
@row_add n=25 row=contract/escher-session-coverage/answer-line area=input-validation rec=4
boundary: escher-session coverage role
item: answer line
promise: The coverage answer holds handler references of ids, owner ids and event names and two counts, no process-local handle, and crosses only as one line on the command line's stdout, with no verb, socket or MCP carrier.
@row_none n=2 rec=5 type=4
relied: The unkeyed-actionable reading and the actionable-nodes reading both read one actionable rule: focusable, an interactive role, or current listeners holding an event other than `mounted`.
@row_add n=26 row=contract/stable-element-id/virtual-node-form area=input-validation rec=6
boundary: Stable element id
item: virtual-node form
promise: The id grammar reserves `{owner}///{key}` for a virtual node, spelled for a non-empty key with no `/`; no served id holds `///`, and the anchored path stays the only served id holding `//`.
@row_add n=27 row=decision/aria-label-trimmed-once area=input-validation rec=7
name: One trimmed aria-label read
rule: An `aria-label` non-empty after trimming becomes the node's label and a whitespace-only one names nothing; there is one trimmed, non-empty `aria-label` read.
why: Follows accname-1.2; upstream's unfiltered read was dropped in the merge.
guard: accessibility_label
@row_add n=28 row=decision/aria-labelledby-outranks-label area=input-validation rec=7
name: aria-labelledby outranks label and aria-label
rule: An element's `aria-labelledby` targets' text, gathered cycle-guarded, is its computed label, outranking a `<label>` and an `aria-label`; an id naming no element is ignored and `aria-label` is the fallback when the targets hold no text.
why: Upstream's behaviour, ratified at the upstream sync.
guard: accessibility_label
@row_add n=29 row=decision/accessible-names-not-logged area=input-validation rec=7
name: Accessible names kept out of logs
rule: Escher's own code logs no accessible name, and a host that installs escher's sink writes none to stderr at any `RUST_LOG` level, the third-party record that carried one being dropped whole.
why: That third-party target is outside the sink's allowlist.
guard: mcp_log
@row_add n=30 row=decision/accessible-name-exits area=input-validation rec=7
name: Ratified exits of accessible names
rule: Accessible names leave the process only through the platform accessibility adapter, a hosted call's answer in the snapshot text and diff JSON over the session socket and the client's stdout, and MCP tool results, and reach no log.
why: Each carrier ratified.
@row_add n=31 row=contract/snapshot-text/name-field area=input-validation rec=7
boundary: Snapshot text
item: name field
promise: Each line's accessible name is written as a quoted string in the string's Debug form, so a name holding a line break or a `"` stays inside its one field.
@row_add n=32 row=decision/descriptions-adapter-only area=input-validation rec=8
name: Descriptions cross only to the accessibility adapter
rule: `aria-describedby` targets and `aria-description` text reach only the platform accessibility adapter; the snapshot reads no description, so no driver or MCP carrier, verb, field, log field or span carries one.
why: Upstream's behaviour, ratified as a boundary widening by upstream merge.
guard: accessibility_description
@row_add n=33 row=decision/merged-aria-states-closed-parse area=input-validation rec=9
name: Closed parse of expanded, disabled and level
rule: `aria-expanded`, `aria-disabled` and `aria-level` are read as closed values carrying none of the attribute's text and exit only on the accessibility adapter; `aria-disabled` moves no snapshot reading.
why: As upstream wrote them, inside the ratified states carrier.
guard: accessibility_merged_states
@row_add n=34 row=decision/select-expanded-from-engine-state area=input-validation rec=10
name: Select expansion from engine state only
rule: An author's `open` on a select and `aria-expanded` on a single select steer nothing: a single select's expanded state is the engine's own closed state, and `open` on a select does not match `:open`.
why: An engine change for every document.
guard: an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match
@row_add n=35 row=decision/aria-state-lowercase-exact-match area=input-validation rec=11
name: Control-state values matched exactly in lower case
rule: `aria-selected`, `aria-pressed`, `aria-checked` and `aria-invalid` values are trimmed of ASCII whitespace and compared exactly, in lower case, against a closed set; an unrecognized value sets nothing, but for `aria-invalid`, which reads invalid.
why: A choice: the specification as read states no case rule and no browser was measured; the `aria-invalid` fallback is WAI-ARIA 1.2's rule.
except: A select's option reads its chosen state as selected, never `aria-selected`.
@row_add n=36 row=decision/toggle-read-per-role area=input-validation rec=11,12
name: Toggle state read per role
rule: `aria-checked` sets the toggle only on the five checkable roles and `aria-pressed` only on every other role; a native checkbox or radio reads neither attribute and gives its own checked state under a checkable role and nothing under any other.
why: Ratified.
@row_add n=37 row=decision/control-states-closed-values-only area=input-validation rec=11
name: Control states cross as closed values
rule: Control states leave the process only as closed enums or booleans, on the accessibility adapter and in the snapshot's words and JSON booleans, never as the attribute's text, and no unrecognized or long value is written anywhere as typed.
why: A ratified widening of the adapter crossing by a new author-derived class.
@row_add n=38 row=contract/snapshot-text/state-words area=input-validation rec=11
boundary: Snapshot text
item: state words
promise: Control states are written as the bare words `selected` / `unselected`, `pressed` / `unpressed` and `invalid`; in JSON answers they are booleans.
@row_add n=39 row=decision/inert-subtree area=input-validation rec=13
name: Inert subtree
rule: An element carrying `inert`, or under an ancestor that does, is not focusable, takes no hit, leaves the accessibility tree and the snapshot, and a node holding focus when it comes under `inert` gives focus up; presence only, no value read.
why: An engine change for every document, ratified.
guard: inert_subtree
@row_add n=40 row=decision/select-list-open-gate area=input-validation rec=14
name: Which select opens a list
rule: A disabled, inert or hidden select, a select with no row to show, and one under a non-invertible transform open no list.
why: An engine change for every document and a boundary widening, ratified.
guard: a_disabled_inert_hidden_singular_or_empty_select_opens_nothing
@row_add n=41 row=decision/select-list-row-choice area=input-validation rec=14
name: What a select list row takes
rule: A disabled row of an open select list changes nothing and leaves the list open, and a group header chooses nothing.
why: An engine change for every document and a boundary widening, ratified.
guard: a_disabled_row_changes_nothing_and_leaves_the_list_open
@row_add n=42 row=decision/accessibility-focus-built-nodes-only area=input-validation rec=15
name: Accessibility focus names a built node
rule: The accessibility tree's focus names the focused node only when that node is among the built nodes, else the window, so the tree never names a node it did not build.
why: An engine change for every document, ratified.
guard: inert_subtree
@row_add n=43 row=decision/inert-focus-clear-silent area=input-validation rec=15,13
name: Inert focus clear fires no blur
rule: A node that loses focus by coming under `inert` fires no DOM `blur` or `focusout`.
why: An engine change for every document, ratified.
guard: inert_subtree
@row_add n=44 row=decision/range-current-value-apart area=input-validation rec=16
name: Range input current value apart from its attribute
rule: A range input holds a current value apart from its `value` attribute; a `value` write re-seeds it and a `min`, `max` or `step` write or removal re-sanitizes it.
why: A departure from the spec's dirty-value flag, the cost the ratified ruling took on.
@row_add n=45 row=decision/range-value-sanitized area=input-validation rec=16
name: Range value always finite and on grid
rule: Every range seed, attribute write and move is clamped to its bounds and moved to the nearest grid value, the greater on a tie, so the value stays finite, bounded and on its grid on degenerate markup; unparsable numbers read as absent.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=46 row=decision/range-keyboard-actions area=input-validation rec=16
name: Range input key actions
rule: On a focused range input with no `disabled` attribute, `ArrowRight` or `ArrowUp` steps up, `ArrowLeft` or `ArrowDown` steps down, `Home` and `End` go to the minimum and maximum, and every other key does nothing.
why: Ratified.
@row_add n=47 row=decision/range-pointer-actions area=input-validation rec=16
name: Range input pointer actions
rule: A main-button pointerdown on an enabled range input sets the value from the hit's fraction of the content box, and a click focuses it and changes no value.
why: Ratified.
@row_add n=48 row=decision/range-value-exits area=input-validation rec=16
name: Range current value exits
rule: A range input's current value leaves only as a form submission entry, the `input` event's value and the snapshot's control value, and no accessibility node value or action changes.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=49 row=decision/enter-activates-focused-control area=input-validation rec=17
name: Enter activates the focused control
rule: A pressed `Enter` on the focused node dispatches a click on it when it is no disabled form control and is a `button`, a `summary`, an `input` of type `submit`, `reset` or `button`, or an `a` with `href`.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=50 row=decision/space-keyup-activates-focused-control area=input-validation rec=17
name: Space release activates the focused control
rule: A released Space on the focused node dispatches a click for a `button`, a `summary`, or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` that is not disabled; a link, a text input and a range input take no Space.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=51 row=decision/button-click-focuses area=input-validation rec=17
name: A button click focuses the button
rule: A click on a `button` of any type or an `input` of type `submit`, `reset` or `button` focuses it and submits its form owner when it is a submit button, a disabled form control stopping the walk first.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=52 row=decision/engine-change-event-checkable area=input-validation rec=18
name: Engine fires change on checkbox and radio
rule: A click on a checkbox always, and on a radio only where it was not checked before, sends `change` after `input` with the same value; the page script runtime synthesizes no `change`, so a script hears one, from the engine.
why: An engine change for every document and a boundary widening, ratified.
guard: checkable_change_event
