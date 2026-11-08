@carried n=1 label=architecture/design-philosophy fact=fct-mntz5vy6
why: the graph reader, the signal registry and the snapshot field must live in dioxus-native-dom as part of the headless Dioxus renderer, leaving blitz-dom a headless DOM driven from outside with no engine change.
quote: `dioxus-native-dom` is the "Core headless native renderer for Dioxus based on blitz" (packages/dioxus-native-dom/Cargo.toml:6; packages/dioxus-native-dom/src/lib.rs:3)
@carried n=1 run=2 label=architecture fact=fct-p43yq5ww
quote: the engine is a set of separately usable crates around a headless DOM that external code drives and any renderer can paint
@carried n=2 label=architecture/design-philosophy fact=fct-do226dcu
why: the graph must read the VirtualDom that DioxusDocument integrates with BaseDocument, through that same document, so the read follows the bridge's own handle_ui_event and poll flow and adds no second integration path.
quote: `DioxusDocument` integrates `BaseDocument` with `VirtualDom`; UI events are pushed with `handle_ui_event` and changes flushed with `poll` (packages/dioxus-native-dom/src/dioxus_document.rs:58; packages/dioxus-native-dom/src/dioxus_document.rs:84-85)
@carried n=3 label=architecture/design-philosophy fact=fct-rxrxih7j
why: the graph names the nodes of a held instance's current screen, so the nodes it holds must resolve through `NodeId` alone, never the slotmap key, and a node dropped since the read must stop resolving rather than alias a reused slot.
quote: Node storage is a versioned slotmap so a dropped node's id stops resolving instead of aliasing a reused slot (packages/blitz-dom/src/tree.rs:27-35); the slotmap key is used only at the storage boundary and all public APIs use `NodeId` (packages/blitz-dom/src/tree.rs:11-15)
@carried n=3 run=2 label=architecture fact=fct-57klyykw
quote: a dropped node's id stops resolving instead of aliasing a reused slot, the slotmap key is used only at the storage boundary and every public API takes `NodeId`
@carried n=4 label=architecture/stack-and-technologies fact=fct-j3ak3xwu
why: the graph must be keyed on the agent-visible AccessKit node of the snapshot's build, of the workspace's accesskit 0.25, with no second accesskit version taken.
quote: accesskit "0.25" (Cargo.toml:139-146)
@carried n=5 label=architecture/stack-and-technologies fact=fct-mcwsssza
why: links 3 and 4 must be read through Dioxus 0.7.3's own public APIs (`Readable::subscribers()`, `ReactiveContext::origin_scope()`, `VirtualDom::in_runtime`) from the workspace's dioxus-core and dioxus-signals, with no Dioxus fork and no version move.
quote: Dioxus crates "0.7.3" (dioxus, dioxus-core, dioxus-html, dioxus-hooks, dioxus-signals, dioxus-stores, dioxus-asset-resolver, manganis, dioxus-document, dioxus-history, dioxus-devtools, dioxus-cli-config, dioxus-core-macro) (Cargo.toml:85-102)
@carried n=5 run=2 label=architecture fact=fct-d4plx3kh
quote: Dioxus 0.7, tokio and reqwest
@carried n=6 label=architecture/stack-and-technologies fact=fct-ovyord5g
why: the graph read's recorded step and its answer line are read back by escher-driver's serde_json only as a reader of kept checks and the live answer a replay compares, so the answer itself must be written by hand and no serialization crate may be taken to write it.
quote: serde_json "1" — resolved 1.0.151, already in the graph through dioxus-html, taken by escher-driver alone as its reader of MCP requests, of kept checks and of the live answer line a replay compares (packages/escher-driver/Cargo.toml:17)
@binding n=1
**stack-and-technologies Dioxus 0.7.3 ↔ design-philosophy headless renderer**: links 3 and 4 must come through the public Dioxus 0.7.3 APIs the workspace pins (architecture/stack-and-technologies fct-mcwsssza), read inside dioxus-native-dom, the headless Dioxus renderer (architecture/design-philosophy fct-mntz5vy6), through the VirtualDom DioxusDocument already integrates (architecture/design-philosophy fct-do226dcu), so the registry needs no Dioxus fork and no engine change for every document.
@binding n=2
**stack-and-technologies accesskit 0.25 ↔ design-philosophy safe identities**: the graph's key must be the AccessKit node of the workspace's accesskit 0.25 (architecture/stack-and-technologies fct-j3ak3xwu), with any owner element resolved through a versioned `NodeId` that stops resolving once dropped and never the slotmap key (architecture/design-philosophy fct-rxrxih7j).
@binding n=3
**stack-and-technologies serde_json ↔ the body's driver session**: the graph read's answer must be written by hand in escher-driver's crate-private `json` module, serde_json kept to reading MCP requests, kept checks and the live answer line a replay compares (architecture/stack-and-technologies fct-ovyord5g), as every answer is (architecture fct-h6zygr3f).
