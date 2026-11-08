@carried n=1 label=architecture/design-philosophy fact=fct-do226dcu
why: the record of which (element, event) handlers a run reached must be taken on DioxusDocument's own UI-event path, so it holds on every app whether or not a graph-state registry stands.
quote: `DioxusDocument` integrates `BaseDocument` with `VirtualDom`; UI events are pushed with `handle_ui_event` and changes flushed with `poll` (packages/dioxus-native-dom/src/dioxus_document.rs:59; packages/dioxus-native-dom/src/dioxus_document.rs:85-86)
@carried n=2 label=architecture/design-philosophy fact=fct-rxrxih7j
why: the exercised record spans every screen a run reaches, so it must key on the graph's stable node id with its optional owner, never on a NodeId that stops resolving once its node drops.
quote: Node storage is a versioned slotmap so a dropped node's id stops resolving instead of aliasing a reused slot (packages/blitz-dom/src/tree.rs:27-35); the slotmap key is used only at the storage boundary and all public APIs use `NodeId` (packages/blitz-dom/src/tree.rs:11-15); persistent interaction state references DOM nodes only, never layout-generated nodes, so it survives box-tree reconstruction (tests/blitz-tests/tests/interaction_state_canonicalization.rs:1-10)
@carried n=2 run=2 label=architecture fact=fct-57klyykw
quote: a dropped node's id stops resolving instead of aliasing a reused slot, the slotmap key is used only at the storage boundary and every public API takes `NodeId`
@carried n=3 label=architecture/stack-and-technologies fact=fct-mcwsssza
why: the exercised record and the coverage read must be built on the Dioxus 0.7.3 crates already in the graph, moving no Dioxus version.
quote: Dioxus crates "0.7.3" (dioxus, dioxus-core, dioxus-html, dioxus-hooks, dioxus-signals, dioxus-stores, dioxus-asset-resolver, manganis, dioxus-document, dioxus-history, dioxus-devtools, dioxus-cli-config, dioxus-core-macro) (Cargo.toml:85-102)
@carried n=3 run=2 label=architecture fact=fct-d4plx3kh
quote: Boa, Dioxus 0.7, tokio and reqwest
@carried n=4 label=architecture/stack-and-technologies fact=fct-ovyord5g
why: the owed replay test reads a kept check back and compares the live actions answer line key by key, so serde_json must stay escher-driver's reader only, adding no serialization crate.
quote: serde_json "1" — resolved 1.0.151, already in the graph through dioxus-html, taken by escher-driver alone as its reader of MCP requests, of kept checks and of the live answer line a replay compares (packages/escher-driver/Cargo.toml:17)
@carried n=5 label=architecture/stack-and-technologies fact=fct-du6rwwjz
why: the coverage acceptance and the replay test must run on the headless stand through the driver session using blitz-tests' existing dev-dependencies, adding none.
quote: seven_guis (the headless stand, tests/blitz-tests/Cargo.toml:22), escher-driver (the driver session, tests/blitz-tests/Cargo.toml:23)
@carried n=6 label=architecture/stack-and-technologies fact=fct-5nyfapub
why: where a new verb moves the capability-statement pin, test_capability_statement.py must stay a stdlib-only Python helper of the ci-scripts leg, no package added.
quote: Python 3 helper scripts (.github/scripts/wpt_diff_to_pr.py:1; .github/scripts/test_wpt_diff_to_pr.py:1) — among them upstream's `wpt_area_changes.py` with its test file and escher's `test_blitz_tests_targets.py`, which reads a manifest with the stdlib `tomllib`, so the `ci-scripts` leg needs Python 3.11 or later, no package added
@carried n=6 run=2 label=architecture fact=fct-j2ltw335
quote: the `ci-scripts` leg needs Python 3.11 or later
@carried n=7 label=architecture fact=fct-77tjcb3p
why: the replay test records and keeps its own check at test time, so the two committed checks under tests/blitz-tests/checks/ stay the one committed exception, and docs/capabilities.md is escher's capability statement a new verb would move.
quote: tests/blitz-tests/     checks/ — the two committed kept checks `stand_replay` replays
@carried n=7 run=2 label=architecture fact=fct-77tjcb3p
quote: capabilities.md (escher's capability statement) beside upstream's three files — an upstream change under docs/ never conflicts with it
@binding n=1
**design-philosophy ↔ standard contracts (Dioxus DOM bridge)**: the exercised record is taken where UI events enter DioxusDocument (architecture/design-philosophy fct-do226dcu), and must hold on every app, unlike the registry's dispatch, which the bridge's handle_event sets only where a registry stands (architecture fct-srb7slak).
@binding n=2
**design-philosophy ↔ standard contracts (driver session) and the plan's scope**: since a dropped node's NodeId stops resolving (architecture/design-philosophy fct-rxrxih7j), the coverage record and list key on the graph's stable id with its optional owner, and no NodeId crosses the driver's wire (architecture fct-ksgwc2yv).
@binding n=3
**stack-and-technologies ↔ standard contracts (driver session)**: serde_json only reads kept checks and the live answer line a replay compares (architecture/stack-and-technologies fct-ovyord5g), while every answer, the coverage answer included, is written by hand in the crate-private json module (architecture fct-h6zygr3f).
@binding n=4
**stack-and-technologies ↔ standard contracts (headless stand)**: the stand tests reach seven_guis and escher-driver through blitz-tests' existing dev-dependencies (architecture/stack-and-technologies fct-du6rwwjz), on the stand's pinned viewport and one bundled font (architecture fct-tewgiusr).
@binding n=5
**stack-and-technologies ↔ project intent (front page) and Infrastructure Patterns**: a new verb moves the stdlib-only capability-statement pin (architecture/stack-and-technologies fct-5nyfapub), which reads README's tool count (architecture fct-3eyld63q) and docs/capabilities.md (architecture fct-77tjcb3p).
