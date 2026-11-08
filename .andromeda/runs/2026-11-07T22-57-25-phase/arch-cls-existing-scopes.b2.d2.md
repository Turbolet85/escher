@answer n=1 entity=existing-scopes#1.26 v=yes
why: escher-driver's Session, the VERBS schema, Outcome, json, mcp and recording are the carriers the graph read crosses (a new verb, outcome, written form, MCP tool and recorded step).
@answer n=2 entity=existing-scopes#1.27 v=no
why: apps/browser is outside the chunk and the task's not_needed list excludes the browser app from the graph read.
@answer n=3 entity=existing-scopes#1.28 v=no
why: browser-persistence's HistoryStore is not read by the graph and the chunk does not touch it.
@answer n=4 entity=existing-scopes#1.29 v=no
why: the rdme markdown viewer is not reached by the driver, the Dioxus bridge or the stand.
@answer n=5 entity=existing-scopes#1.30 v=no
why: the WPT runner and its outputs are not_needed, and the chunk adds no WPT test and changes no runner.
@answer n=6 entity=existing-scopes#1.31 v=yes
why: the blitz-tests stand, session, recording and snapshot checks named here (stand_non_lean_tasks' starting click reading, stand_replay's committed checks, the masked-value and session checks) are the ones the new field, registry and graph read move.
@answer n=7 entity=existing-scopes#1.32 v=yes
why: seven_guis's headless stand, its signals moving onto the registry, and the escher-session served tasks and recording script are the chunk's own surfaces.
@answer n=8 entity=existing-scopes#1.33 v=no
why: the separate example crates counter, todomvc, transparent, wasm_hello and wgpu_texture are not the stand and the chunk does not touch them.
@answer n=9 entity=existing-scopes#1.34 v=no
why: the root examples exercise engine features (box shadow, flex, gradients, SVG and the like) that the graph read and the driver do not change.
@answer n=10 entity=existing-scopes#1.35 v=no
why: the HTML fixtures under examples/assets are not_needed, and the chunk adds no HTML fixture page or licensed asset.
@answer n=11 entity=existing-scopes#2.1 v=yes
why: the snapshot's new field changes the Cells and load-page screens these checks measure (the 47,089-byte, 740-line, five-part Cells screen), and the stand boots both.
@answer n=12 entity=existing-scopes#3.1 v=no standing=S1
why: the rule that escher-driver's cli and recording unit tests sit in #[path] children is the S1 home convention, and the chunk's new escher-driver tests take that home without adding an instance the entity must record.
@answer n=13 entity=existing-scopes#3.2 v=yes
why: the chunk's new stand checks and driver call builders read tests/common, tests/session_common and tests/fixtures, so their reader counts of 21, 32 and 31 and their contents must record them.
@answer n=14 entity=existing-scopes#3.3 v=no
why: inside_stand_repeat reads only the 0.2.0 WITNESSES_020 record, which the 0.3.0 graph chunk does not change.
@answer n=15 entity=existing-scopes#3.4 v=no
why: blitz-vibey-script's JavaScript runtime is not_needed, and the stand is a Dioxus app with no script involved.
@answer n=16 entity=existing-scopes#3.5 v=no
why: pseudo-element flow classification is layout, which the graph reads already-built nodes for and does not change.
@answer n=17 entity=existing-scopes#4.1 v=no
why: the engine-fact targets (visible region, hit testing, open state, change events, key press after a dropped box) are engine behaviour the graph reads without changing, and no rule in them governs a snapshot field, a handler or a signal.
@answer n=18 entity=existing-scopes#4.2 v=yes
why: dioxus_runtime_probe reads the Dioxus public API for a node's owning component, its listeners and a held signal's readers, the same surfaces the graph's links 1, 2 and 4 read, and it must keep holding no subscriber or listener.
@answer n=19 entity=existing-scopes#4.3 v=yes
why: the bridge's buffered render and its discriminating probe witness are the Dioxus integration the graph's DioxusDocument vdom reads run through, so the read must not reenter the render borrow.
