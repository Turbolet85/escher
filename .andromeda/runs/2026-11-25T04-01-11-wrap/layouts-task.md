@task_prose part=session
The session added virtual nodes declared: a custom widget (a canvas) may now declare keyed, named, role-bearing regions inside its content box, through a defaulted widget method that returns none by default.
The document bridge gained an in-process read of a canvas's declared nodes that validates each in declaration order and refuses a bad one with a closed reason (empty key, key holding a slash, duplicate key with the first kept, empty name, region outside the content box), and gives each taken node the reserved id {owner}///{key}.
The read refuses a stale or non-canvas node as no canvas and an unlaid canvas as not laid out; it has no panic path, no log site, no changed-set mark and no invalidation signal.
No carrier reads declared nodes: the session's snapshot and graph answers hold no {owner}///{key} id and no declared name, asserted by a new check in both layout modes; the names' crossing into the accessibility tree stays deferred and PROVISIONAL.
No stand page, no stand widget, no command line, no dependency and no gate changed; 0 of the existing widget implementations were edited.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the engine's custom widget trait (the canvas widget boundary) — edited: gains the declared-node type and a defaulted method returning none
@task_file n=2 path=packages/blitz-dom/src/node/mod.rs
what: the engine's node module re-exports — edited: re-exports the declared-node type
@task_file n=3 path=packages/blitz-dom/src/lib.rs
what: the engine crate's public surface — edited: re-exports the declared-node type
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
what: the document bridge crate's public surface and module list — edited: gains the ungated virtual-node module and re-exports five types
@task_file n=5 path=packages/dioxus-native-dom/src/element_id.rs
what: the stable element id reader and the reserved virtual-node id form {owner}///{key} — edited: its docs now say the form is produced in process for a canvas's taken nodes, written by no walk, served by no carrier
@task_file n=6 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the document's in-process read of a canvas's declared virtual nodes, their validation and its unit checks — added
@task_file n=7 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the integration check that a padded canvas reads its declared nodes in its content box and that a declaring canvas puts nothing on the snapshot or graph answer — added
@task_item n=1 part=touches
The escher-session snapshot answer: unchanged, but now asserted to hold no virtual-node id ({owner}///{key}) and no declared name while a canvas declares nodes; a row on what the snapshot answer holds may need that absence beside it.
@task_item n=2 part=touches
The escher-session graph answer: unchanged, likewise asserted to hold no declared virtual node; a row on the graph answer's nodes may need that beside it.
@task_item n=3 part=touches
The escher-session MCP canvas tool and canvas draw commands: a canvas can now declare named regions, but no tool or verb reads or serves them; rows on what the canvas tool returns stand, with declared regions not among them.
@task_item n=4 part=touches
Stand stable element ids and the decision that stand ids are attributes alone: the reserved form {owner}///{key} is now produced in process for a canvas's taken nodes, yet no served or walked id holds ///; no stand id is a virtual-node id.
@task_item n=5 part=touches
Circle Drawer circles painted on a canvas and the circle-canvas id: the canvas widget kind gained an optional declared-node method; Circle Drawer's canvas declares none, so its circles stay paint alone with no per-circle element.
@task_item n=6 part=touches
Stand names, roles and states by attributes alone: a declared node carries a role and a name, but they reach no accessibility tree, snapshot or graph this session; the attribute-only rule for the stand stands.
@task_item n=7 part=touches
Escher-session refusals (off-screen and the like): the declared-node read has its own closed refusals (no canvas, not laid out, and per-declaration reasons) that are in-process and never a session refusal or exit code.
@task_fact n=1 slug=surface-cli
fact: No session answer — snapshot, graph, canvas tool or any other carrier — holds a canvas's declared virtual nodes, their {owner}///{key} ids or their names; the crossing is deferred and stays the founder's fork (PROVISIONAL).
@task_item n=1 part=untouched
The CLI test pipes (agent-run, cold-agent) and their exit codes, usage and log paths: no script or argument changed.
@task_item n=2 part=untouched
The multi_doc and paint_bench command lines: no benchmark or argument changed.
@task_item n=3 part=untouched
The ShellProvider window chrome controls: no window behaviour changed.
@task_item n=4 part=untouched
The stand page skeleton, accent colour, pinned viewport, boot lints, scroll and visible-region decisions, and the per-task stand element ids other than the canvas: no stand page or control changed.
@task_item n=5 part=untouched
The MCP server's transport, protocol revisions, result bound, screenshot result and tools list, and the session's recording, exit codes and served tasks: none changed.
@task_item n=6 part=untouched
The mobile-native and web-spa surfaces and the absence of arbitrary index.html templates: not reached.
