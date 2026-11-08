@carried n=1 label=architecture/standard-contracts fact=fct-kllkxz3d
why: the reserved virtual-node id form must use a separator no id of the four tiers can hold, and link 1 must read component names as the component path reads them, with no process-local value entering an id.
quote: an author key holds no `/`, and an anchored path is the only id holding `//`, a component path being non-empty names and segments and a document or app-root path starting with one `/` with no empty segment after it — so a key spelled like a component name stays distinct (`Item//b:0` beside `Item/b:0`)
@carried n=1 run=2 label=architecture/standard-contracts fact=fct-kllkxz3d
quote: no `NodeId`, `ElementId`, `ScopeId` or pointer enters an id
@carried n=1 run=3 label=architecture/standard-contracts fact=fct-kllkxz3d
quote: dioxus-core's four framework scopes (`ScopeId::ROOT` · `ROOT_SUSPENSE_BOUNDARY` · `ROOT_ERROR_BOUNDARY` · `APP`, the user root dioxus-core names `"root"`) add none, a segment is the last `::` segment of `VComponent.name` (dioxus-core's full type path) with generics stripped
@carried n=2 label=architecture/standard-contracts fact=fct-kllkxz3d
why: the reserved separator must leave what `unkeyed_actionable` reports unchanged, and its listener reading over-includes as link 2 must not.
quote: `DioxusDocument::unkeyed_actionable(&self) -> Vec<UnkeyedActionable>` is the check behind the rule that every element an agent can act on reads an author key: read-only and argument-free, it returns every actionable element whose stable id holds a `/` (a positional path of any of the three path tiers) in document pre-order, empty when every actionable element reads its key
@carried n=2 run=2 label=architecture/standard-contracts fact=fct-kllkxz3d
quote: or the `data-dioxus-id` attribute the bridge writes for every listener but `onmounted` and that stays, with its event attribute, on an element whose listener a re-render removed, so the check over-includes such an element by its listener, the conservative side (read, not run)
@carried n=2 run=3 label=architecture fact=fct-d5zuqqfb
quote: Every element an agent can act on reads an author key: `DioxusDocument::unkeyed_actionable` lists, read-only, each actionable element whose stable id holds a `/`, with a remedy naming the fix.
@carried n=3 label=architecture/standard-contracts fact=fct-kllkxz3d
why: the graph must be keyed on the agent-visible accesskit node this override names by stable id, never on a DOM `NodeId`.
quote: under the `accessibility` feature `DioxusDocument` overrides `Document::accessibility_tree`: it builds the base tree, maps `element_ids()` by `NodeId::as_u64()` and sets each element node's AccessKit `author_id` to that element's stable id, while `TextRun`s, the document root and the synthetic `Window` (`u64::MAX`) carry none and no tree id indexes the slab
@carried n=4 label=architecture/standard-contracts fact=fct-kllkxz3d
why: the «reacts to a click» closed boolean joins the node's fields, so `NodeState`'s field list, the text form's token order, the lean tasks' pinned text sizes and `NODE_FIELDS`' thirteen must each move with it.
quote: `NodeState` carries seven fields in the order `enabled: Option<bool>` · `checked: Option<bool>` · `value: Option<String>` · `selected: Option<bool>` · `pressed: Option<bool>` · `invalid: bool` · `focused: bool` — four until 2026-10-10-snapshot-states added the three after `value`, with no new public type, so a struct literal without a spread names all seven (packages/dioxus-native-dom/src/snapshot.rs:63-74)
@carried n=4 run=2 label=architecture/standard-contracts fact=fct-kllkxz3d
quote: the state tokens the model holds in the fixed order `enabled` / `disabled` · `checked` / `unchecked` · `value=` with the value quoted · `selected` / `unselected` · `pressed` / `unpressed` · `invalid` · `focused` (a token absent where the model holds nothing for it; `invalid` written only when true; the three after `value=` since 2026-10-10-snapshot-states — packages/dioxus-native-dom/src/snapshot_text.rs:112-124)
@carried n=4 run=3 label=architecture/standard-contracts fact=fct-kllkxz3d
quote: on the stand each lean task's screen at boot reads Counter 755 · FlightBooker 1295 · Timer 1259 · Crud 2055 bytes on the dev host (Linux), read through the host in its default layout mode
@carried n=4 run=4 label=architecture/standard-contracts fact=fct-h4l33tnz
quote: The snapshot carries no `expanded` node key and `NodeState` holds no field for a list, `NODE_FIELDS` keeping the thirteen keys §Driver session names.
@carried n=4 run=5 label=architecture fact=fct-srb7slak
quote: under `accessibility` the snapshot, its text form and its diff read the screen read-only, holding no `NodeId`.
@carried n=5 label=architecture/standard-contracts fact=fct-kllkxz3d
why: a new `NodeState` field moves the diff's `Reading` set, the `differing` site that destructures `NodeState` whole and the JSON nodes keyed by `NODE_FIELDS`.
quote: `differs` the readings that differ from before the step, in node-key order (`Reading::ALL`), filled at the one site that decides `changed` (`differing`, its `NodeState` destructured whole) and empty for an added node, a node that only moved reading `[Bounds]`
@carried n=5 run=2 label=architecture/standard-contracts fact=fct-kllkxz3d
quote: `pub enum Reading` — eleven variants, `Parent` · `Role` · `Name` · `Enabled` · `Checked` · `Value` · `Selected` · `Pressed` · `Invalid` · `Focused` · `Bounds`, with `Reading::ALL`
@carried n=5 run=3 label=architecture/standard-contracts fact=fct-kllkxz3d
quote: escher-driver's `Outcome::to_json` writes a returned diff as JSON — `added` and `changed` as nodes keyed by `NODE_FIELDS`
@carried n=5 run=4 label=architecture fact=fct-tev7ieys
quote: No log or event carries the snapshot, its text or a diff, except a diff's three counts — added, removed and changed — on escher-driver's command span.
@carried n=6 label=architecture/standard-contracts fact=fct-ie5oemgh
why: link 2 must read an element's handlers through the vdom walk's dynamic attributes, `mounted` included, and never through the bridge's listener attributes.
quote: An element's listeners read two ways: the vdom walk's dynamic attributes (`VNode.dynamic_attrs`, `VNode::mounted_dynamic_attribute`) read its current listeners exactly, `mounted` included, while the bridge's listener attributes — an attribute named the event, valued `"<rust func>"`, beside the `data-dioxus-id` mark, written by `create_event_listener` — over-report: a listener a re-render removes leaves its attribute and the mark standing while a click no longer acts, `remove_event_listener` lowering the per-kind count alone, and a `mounted` listener writes none, in both layout modes
@carried n=6 run=2 label=architecture/standard-contracts fact=fct-67g4co3e
quote: dioxus-core 0.7.10 reaches `remove_event_listener` only when a listener attribute leaves an element's attribute list (a spread); a listener whose value turns to `AttributeValue::None` under the same name is written through `set_attribute` as an `on{event}` attribute, so the bridge's event attribute stays either way
@carried n=6 run=3 label=architecture/standard-contracts fact=fct-kllkxz3d
quote: DOM events route to the vdom via the nearest `data-dioxus-id` attribute parsed as `usize`
@carried n=7 label=architecture/standard-contracts fact=fct-2ylcpw3b
why: Cells' grid is a role-less element with a handler whose «reacts to a click» reading the chunk re-pins, its `current` signal moves onto the registry keeping its behaviour, and its per-cell `onmounted` listeners are what link 2 reads.
quote: `cells-grid` takes `id`, `tabindex` 0, `aria_label` Cells and an `onkeydown`, with no role, so it is one Tab stop: arrows move `current`, clamped, into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`, one `onmounted` per data cell holding the handles
@carried n=8 label=architecture/standard-contracts fact=fct-ab3t3wht
why: every state or control value the graph carries must read masked through this one test and its `MASKED_VALUE` stand-in, never a second masking rule.
quote: `is_masked_input(&ElementData) -> bool`, public and re-exported beside `MASKED_VALUE`: true for an `input` whose `type` reads `password` or `file`, ASCII case-insensitive — the test the snapshot's `value` reader calls, no reading changed, and the one escher-driver's recorder reads to keep a masked `type` as its stand-in (packages/dioxus-native-dom/src/snapshot.rs:236-243).
@carried n=8 run=2 label=architecture/standard-contracts fact=fct-kllkxz3d
quote: except that a password `input` or a file `input` (the `type` compared ASCII-case-insensitively) reads `MASKED_VALUE` where that reading is non-empty and `""` where it is empty (a file input with no `value` attribute reads `None`), never the text, its length or a file's host path
@carried n=8 run=3 label=architecture fact=fct-6gp3hxh7
quote: `is_masked_input` is the one masked-control test, a password or file `input`
@carried n=9 label=architecture/standard-contracts fact=fct-6m5fkd33
why: the graph read's new verb must cross every carrier as the `lint` verb does — a `Command`, an `Outcome`, a written form, an MCP tool and a recorded step, its `command` span adding no field — and `VERBS`' twelve and its pins move with it.
quote: The `lint` verb carries it — `Command::Lint`, `Outcome::Linted`, the `{"findings":[…]}` written form, an MCP tool and a recorded step, its answer kept whole — its `command` span recording `verb` and, refused, `cause` alone, no field added, and `VERBS` holds twelve
@carried n=9 run=2 label=architecture fact=fct-ksgwc2yv
quote: a `Session` (one booted app) runs every call through `Session::run`, which validates first; nothing public runs a `Command` that did not come from `validate`; the command and refusal schema stated once as `'static` data — twelve verbs, eight closed causes, no refusal holding what a call supplied
@carried n=10 label=architecture/standard-contracts fact=fct-g75i5t3p
why: the graph read runs no step, so it must wait on nothing and read no clock, joining `snapshot`, `screenshot` and `lint` in that set.
quote: `Session::act` and the executor's settled step settle through it, so the five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier — in process, the socket, the command line and MCP — and `snapshot`, `screenshot` and `lint`, which run no step, wait on nothing. It reads `std::time::Instant` and sleeps, and nothing else: no `tick`, `advance_time` or time step, so no time moves on the held instance while it waits, no changed-set drain, no print and no `tracing` call — `session.rs` the one clock reader among the session, the executor, the schema, validation and the JSON writer.
@carried n=11 label=architecture/standard-contracts fact=fct-c5e4c7ao
why: the graph's verb must join the command line's verb list, its usage and `help`'s self-description beside the twelve of `VERBS`.
quote: the verbs are the twelve of `VERBS` — `snapshot`, `click --id`, `type --id --text`, `press --key [--shift]`, `advance --ms`, `scroll --id`, `screenshot [--id <id>]`, `lint`, `start <task> [--record <dir>]`, `status`, `stop` and, since 2026-10-12-self-description, `help` — each with `--session <dir>` exactly once but `help`
@carried n=11 run=2 label=architecture fact=fct-fukxpbvx
quote: `escher-session <verb> [arguments] --session <dir>`, the driver's command line over six served stand tasks
@binding n=1
**Dioxus DOM bridge snapshot ↔ driver session schema**: the «reacts to a click» boolean added to `NodeState` must move, together, the text form's token order, the diff's `Reading` set and `differing`, the lean tasks' pinned text sizes and `NODE_FIELDS`' thirteen that `Outcome::to_json` keys nodes by (architecture/standard-contracts fct-kllkxz3d) (architecture/standard-contracts fct-h4l33tnz).
@binding n=2
**Driver session verbs ↔ CLIs**: the graph read's verb joins `VERBS` (twelve), the eight-cause schema validated by `Session::run`, the command line's verb list and `help`, an MCP tool and a recorded step, and as a non-acting read it waits on nothing and reads no clock (architecture/standard-contracts fct-6m5fkd33) (architecture/standard-contracts fct-c5e4c7ao) (architecture/standard-contracts fct-g75i5t3p).
@binding n=3
**Stable ids ↔ unkeyed_actionable**: the reserved virtual-node separator must collide with no tier's spelling (`/`, `//`) and leave what `unkeyed_actionable` reports for an id holding a `/` unchanged (architecture/standard-contracts fct-kllkxz3d).
@binding n=4
**Listener reading ↔ masking and logging**: link 2 reads the vdom's dynamic attributes, never the bridge's over-reporting attributes, and every value the graph carries reads masked through `is_masked_input` with no log carrying it (architecture/standard-contracts fct-ie5oemgh) (architecture/standard-contracts fct-ab3t3wht) (architecture fct-tev7ieys).
