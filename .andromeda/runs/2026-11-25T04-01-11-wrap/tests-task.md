@task_prose part=session
The session let an app declare virtual nodes inside a canvas: keyed, named, role-bearing regions, each one validated and given the reserved id owner///key.
The engine gained a defaulted widget trait method, virtual_nodes, and a declaration type behind the custom-widget feature. Nothing in the engine calls the method, no existing widget implementation was edited, and the addition is flagged upstreamable.
The dioxus bridge gained an ungated in-process read, DioxusDocument::virtual_nodes. A stale or non-canvas node reads NoCanvas and an unlaid canvas reads NotLaidOut. Declarations are checked in order, and a failing one gets a closed reason: EmptyKey, KeyHoldsSlash, DuplicateKey (the first is kept), EmptyName, or RegionOutside the content box. The read never panics and has no log site.
The reserved owner///key form is now produced in process for a canvas's taken nodes. No walk writes it and no carrier serves it: snapshot and graph still hold no ///. There is no changed-set mark and no invalidation signal.
Nine unit cases and one new integration target with three cases were added. Each integration case runs in both layout modes, and 14 controls held. CI on the final HEAD was green, 17 of 17 checks.
No dependency, manifest, CI, gate or harness surface changed. Nothing was retired.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the engine's custom widget trait that an app widget implements (paint, layout); it now holds the virtual-node declaration type and the defaulted virtual_nodes method
@task_file n=2 path=packages/blitz-dom/src/node/mod.rs
what: the engine's node module and its public re-exports of the widget items
@task_file n=3 path=packages/blitz-dom/src/lib.rs
what: the engine crate root, the blitz-dom public surface and its re-exports
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
what: the dioxus bridge crate root, its module list and its public re-export list, where the ungated modules stand
@task_file n=5 path=packages/dioxus-native-dom/src/element_id.rs
what: the bridge's stable element ids, holding the reserved owner-key virtual form and virtual_node_id with its unit tests
@task_file n=6 path=packages/dioxus-native-dom/src/virtual_node.rs
what: new bridge module for the in-process read of a canvas's declared virtual nodes, its validation, and its inline unit test module
@task_file n=7 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: new blitz-tests integration test target with one behaviour per file; it checks declared nodes in the content box, a widget that declares nothing, and that a declaring canvas puts nothing on the snapshot or graph, in both layout modes
@task_item n=1 part=touches
The element-id virtual form row and the reserved owner-key graph id form decision: the form owner///key is now produced in process by the bridge's declared-node read for a canvas's taken nodes. No walk writes it and no carrier serves it, so no served id holds ///.
@task_item n=2 part=touches
The unit test strategy's scope for the dioxus bridge: the bridge's unit scope now covers the declared-node validation. That means each refusal reason, a region touching the content edges, the owner-key id of a taken node, an unlaid canvas, and a stale or non-canvas node.
@task_item n=3 part=touches
Rows saying bridge tests are a child of the dioxus document and test without widening visibility: the new unit module is such a child and calls the crate-private validate.
@task_item n=4 part=touches
Unit tests sit in inline test modules in their source file: the new module follows this. Its fixtures use the default role so that it names no accessibility item, because the bridge's accesskit dependency is optional under the accessibility feature.
@task_item n=5 part=touches
One integration test target per behaviour and per file: blitz-tests gains the auto-discovered target virtual_nodes_declared with three cases, and no manifest changed. The blitz-tests manifest rows hold.
@task_item n=6 part=touches
Layout and stand checks run in both layout modes: every case of the new target loops over incremental off and on.
@task_item n=7 part=touches
Tests controlled red-before-green, and control edits leave earlier targets green: 14 controls held for this chunk. One of them shows the content-box subtraction going red without it, and one shows the unlaid canvas reading NotLaidOut.
@task_item n=8 part=touches
The graph and snapshot answers of the driver session, and graph answer holds no value: the new crossing check asserts that snapshot and graph through Session::run hold no /// and no planted name while a canvas declares nodes. Its live-needle precondition is asserted first.
@task_item n=9 part=touches
Shared fixtures in one module, and test data in code: the new target declares the shared fixtures module and builds its snapshot and graph calls in-file as escher_driver Call. All of its declaration data is stated in code.
@task_item n=10 part=touches
The stand-repeat named list and requirement witnessing inside stand-repeat: the new target is no stand_ target. The stand-repeat list does not gain it, and no row's count moves.
@task_item n=11 part=touches
The decision on no screenshot step in committed checks, in its one-host clause (screenshot repeatability asserted on one host only): this is now stale. Canvas-only shot digests, whole and as the canvas element in both layout modes, plus Circle Drawer's canvas element shot, equal one committed table on linux, windows and macOS. A shot holding page text still differs across hosts, so the committed-check half and its reason stand.
@task_item n=12 part=touches
Per-crate coverage floors: the bridge crate gains a new module and its nine unit cases. The floors are unchanged and no row's floor value moves.
@task_fact n=1 slug=unit-test-strategy
fact: The bridge's declared-node read is unit-tested in its own inline module. There is one case per refusal reason (empty key, key holding a slash, later duplicate refused with the first kept, empty name, region outside the content box), plus a region touching the content edges taken, a taken node reading its owner///key id, an unlaid canvas reading NotLaidOut, and a stale or non-canvas node reading NoCanvas. Each is controlled red-before-green.
@task_fact n=2 slug=integration-test-strategy
fact: A declaring canvas putting nothing on the snapshot or graph is asserted in both layout modes, with no mutation control. The control is owed once code exists that could carry a virtual node to a carrier.
@task_item n=1 part=untouched
CI integration rows (legs, caches, matrix, package installs, actions pinning, secrets, permissions, trigger branches, MSRV): no workflow, leg script or manifest changed.
@task_item n=2 part=untouched
Cold-agent and agent-run rows (events, verbs, exit codes, verdicts, selections, isolation): untouched.
@task_item n=3 part=untouched
Session wire, session host, state directory, recording and escher-session command-line, MCP role and exit-code contracts: no verb, carrier, bound or wire shape changed.
@task_item n=4 part=untouched
WPT runner, conformance and expectations rows: untouched.
@task_item n=5 part=untouched
Mocking and stand-in rows, virtual clock, advance and timer-tick rows, and harness settle and time contracts: untouched.
@task_item n=6 part=untouched
Lint defects and findings, contrast reading, handler coverage, and driver refusal-cause rows (covered, off-screen, disabled, stale): unchanged. Virtual nodes taking clicks or being refused is deferred to a later entry.
@task_item n=7 part=untouched
Quality gate rows on mutation runs, the complexity ceiling, oversize targets, the zero-failure budget and the coverage exclusions: no threshold or exclusion moved.
