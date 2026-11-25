@task_prose part=chunk
Lets an app declare virtual nodes inside a canvas custom widget (the Rust-painted Widget behind the custom-widget feature): each node has an author key (non-empty, no slash), a role from a closed set, a non-empty name and a region inside the canvas's content box.
A declaration that breaks a rule is not taken and nothing panics; two nodes in one canvas with one key are refused; each taken node gets the reserved virtual-node id form, owner stable id then the separator then the key.
The declared nodes are readable in process only, validated and with their ids; the chunk adds no verb, carrier, log field, span or reading, and claims no capability.
It writes every founder's fork (declaration form, names' crossing, role set, where the declaration lives, region frame) into the founder-forks evidence with what crosses, the rule met, each option's measured cost, dependences, whose it is and the most conservative option, then stops at P4 with no default.
It carries two CARRYs to its wrap: the OS drag-and-drop and dropped-file-reader gaps in the security summary, and the test plan's one-host screenshot clause.
@task_file n=1 path=packages/dioxus-native-dom/src/element_id.rs
what: the stable element id derivation, holding the reserved virtual-node id form and its key separator
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget component, the engine's Widget trait with its commented accessibility tree for multiple nodes per widget
@task_item n=1 part=needs
The virtual node declaration on a canvas custom widget: author key non-empty with no slash, role from a closed set, name non-empty, region inside the canvas's content box; a declaration breaking any of these is not taken and no node stands for it.
@task_item n=2 part=needs
No panic path on untrusted input: every malformed declaration (empty key or name, slash in key, role outside the set, region outside the box, non-finite region) is refused without a panic.
@task_item n=3 part=needs
The stable element id's reserved virtual-node form: owner canvas stable id, the virtual key separator, then the author key, produced through the one virtual-node id constructor; the author-key rule and the id derivation it extends.
@task_item n=4 part=needs
Stable element id uniqueness: two nodes in one canvas with the same key are refused so an id names one node only, and no virtual id collides with an element's anchored or keyed id.
@task_item n=5 part=needs
No process-local handle crosses with an id: a virtual node's id carries no NodeId and is derived from the owner's stable id and the key alone.
@task_item n=6 part=needs
Custom widget painted and pointed in its content box: the node region is held in the canvas's content-box frame, the frame the widget paints in and hears pointer events in.
@task_item n=7 part=needs
The in-process read of declared nodes: what a canvas declares now, validated and with ids, reached in process only and on no ratified carrier.
@task_item n=8 part=needs
The boundary widening rule over the names' crossing fork: a virtual node's name is app-authored text, and authored text on a ratified carrier is a widening; the plan crosses no name or key, and the fork's evidence states which carriers a crossing would reach.
@task_item n=9 part=needs
Ratified exits of accessible names and of the stable id, and attribute text leaving only by ratified carriers: the rules a declared name or key meets if a later entry lets it cross, stated in the fork evidence.
@task_item n=10 part=needs
Accessible names and stable ids kept out of logs: declared names, keys and virtual ids are written to no log line, span or field.
@task_item n=11 part=needs
The declaration's form fork (a list at each paint, or callbacks with invalidation) and where the declaration lives (engine Widget trait or the Dioxus renderer), each with measured cost and the most conservative option that still builds the entry.
@task_item n=12 part=needs
The custom-widget feature gate: the Widget and the declaration sit behind the custom-widget feature the harness turns on for blitz-paint.
@task_item n=13 part=needs
The accessibility premises re-read at P3: the accessibility adapter carries nodes with no DOM behind them, with bounds and a transform on the parent, and the content classes on the accessibility adapter crossing a name would join.
@task_item n=14 part=needs
CARRY cry-tjzqz3hz: OS drag-and-drop as an untrusted input source and the dropped-file reader with no path restriction, two gaps the security summary's leaf pass writes at this chunk's wrap, never by hand and with no code step.
@task_item n=15 part=needs
Evidence holds no credentials and no raw log with a temp or home path; a probe worktree stands outside the build directory and no plan step deletes it.
@task_standing n=1
convention: each new reader of untrusted input owes a test that a malformed value is refused without a panic
home: input-validation
@task_standing n=2
convention: each new log line keeps names, keys, stable ids and other content-named fields out
home: logging-monitoring
@task_item n=1 part=not_needed
Driver verbs, driver argument kinds, command-line flags, roles and exit codes: the chunk adds no verb, argument kind, flag or exit status.
@task_item n=2 part=not_needed
Session socket request and reply contracts and MCP request, result and protocol-error contracts: the chunk adds no reading on either wire.
@task_item n=3 part=not_needed
Click synthesis and the refusal causes covered, off-screen, disabled and stale on virtual nodes: owned by the later entry Virtual nodes as elements.
@task_item n=4 part=not_needed
The graph, actions, coverage and layout answers and the exploration report: owned by the later entry Virtual nodes in every read.
@task_item n=5 part=not_needed
Keyboard rows (cells grid, range keys, select keys, Enter and Space activation): canvas keyboard reach is owned by the later entry Canvas keyboard path.
@task_item n=6 part=not_needed
Select list rows (opening, painting, rows, wheel, typeahead, choice exits): no select is touched.
@task_item n=7 part=not_needed
Password and file value masking and the masked stand-in values: no control value is read or carried.
@task_item n=8 part=not_needed
The authentication-authorization area: no access control, token or publish secret is touched.
@task_item n=9 part=not_needed
Outbound fetches, sub-resource requests, CORS, CSP, navigation schemes and response limits in the api-security area: the chunk makes no request.
@task_item n=10 part=not_needed
Browser history, cookie store, HTTP cache, TLS and the history database: no stored data is touched.
@task_item n=11 part=not_needed
Cold-agent and agent-run scripts, their recordings, events and transcripts: no harness run is added.
@task_item n=12 part=not_needed
Kept check file contents and lifecycle and the recorder's directory rules: no reading reaches a kept check or a recording.
