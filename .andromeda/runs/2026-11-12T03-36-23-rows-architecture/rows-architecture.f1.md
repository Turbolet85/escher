@row_add n=1 row=decision/modular-embeddable-engine area=design-philosophy rec=1,2
name: Radically modular, embeddable engine
rule: The engine is a set of separately usable crates around a headless DOM that external code drives and any renderer can paint.
@row_add n=2 row=decision/injected-embedder-services area=design-philosophy rec=3,4
name: Embedder services injected, with no-op defaults
rule: Networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies, and a provider not supplied falls back to a no-op default.
@row_add n=3 row=decision/cite-the-spec area=design-philosophy rec=5,6
name: Behave like a browser, and cite the spec
rule: Web semantics are written against named specifications and named browser engines.
@row_add n=4 row=decision/versioned-node-ids area=design-philosophy rec=7,8
name: Versioned node ids
rule: Node ids are versioned, so a dropped node's id stops resolving instead of aliasing a reused slot; the slotmap key is used only at the storage boundary and every public API takes NodeId.
@row_add n=5 row=decision/interaction-state-on-dom-nodes area=design-philosophy rec=7,8
name: Interaction state references DOM nodes only
rule: Persistent interaction state references DOM nodes only, never layout-generated nodes.
why: so it survives box-tree reconstruction
guard: interaction_state_canonicalization
@row_add n=6 row=decision/history-service-single-entry area=design-philosophy rec=9,10
name: HistoryService as the single history entry point
rule: The reference browser's HistoryService is the single entry point for its visit and favicon writes.
@row_add n=7 row=decision/chrome-writes-one-block area=design-philosophy rec=9,10
name: Chrome writes as one straight-line block
rule: A loaded document's chrome writes run as one straight-line block.
why: so a render cannot sample a half-applied state
@row_add n=8 row=decision/mimalloc-global-allocator area=stack-and-technologies rec=11,12
name: mimalloc as global allocator of two binaries
rule: mimalloc is the global allocator of the browser app (by default) and of the WPT runner.
@row_add n=9 row=decision/svgtypes-matches-usvg area=established-decisions rec=13,14
name: svgtypes version follows usvg
rule: The svgtypes version matches usvg's version.
@row_add n=10 row=decision/no-git-dependencies area=established-decisions rec=13,14,15
name: No git dependencies
rule: No git dependency is left in the manifest; taffy is a registry version.
@row_add n=11 row=decision/winit-exact-beta-pin area=established-decisions rec=13,14
name: winit pinned exactly to a beta
rule: winit is pinned exactly to a beta version, 0.31.0-beta.3.
@row_add n=12 row=absence/in-repo-release-tool area=established-decisions rec=16,17,18
capability: An in-repo release tool
@row_add n=13 row=decision/woff-default-in-dioxus-native area=established-decisions rec=19,20
name: woff on by default in dioxus-native
rule: The `woff` feature is default-on in dioxus-native.
why: so wasm callers, which cannot use system fonts, can bundle compressed fonts
@row_add n=14 row=decision/parallel-traversals-serialised area=established-decisions rec=21,22
name: Parallel style traversals serialised
rule: Parallel style traversals are serialised process-wide by one static lock, its poison recovered with no panic path.
why: so two documents resolving Parallel concurrently on Stylo's global pool no longer panic (blitz issue 430)
