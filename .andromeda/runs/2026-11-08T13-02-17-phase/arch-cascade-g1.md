@carried n=1 label=architecture/design-philosophy fact=fct-rxrxih7j
why: the read must be proven on the stand in both layout modes, and its off-screen and covered causes read layouts the two modes are required to make identical, so each case must answer the same in both.
quote: Incremental and non-incremental layout run the same style → damage → box construction → layout pipeline; non-incremental marks every node damaged each resolve (packages/blitz-dom/src/document.rs:2624-2641; packages/blitz-dom/src/resolve.rs:87-93), so the two must produce identical layouts after any sequence of mutations (tests/blitz-tests/tests/incremental_oracle.rs:1-7)
@carried n=1 run=2 label=architecture fact=fct-57klyykw
quote: both layout modes produce identical layouts, a dropped node's id stops resolving instead of aliasing a reused slot
@carried n=2 label=architecture/stack-and-technologies fact=fct-ovyord5g
why: the new read's MCP request, its kept-check record and the live answer line a replay compares are read by serde_json alone, with no further serialization crate taken for its answer.
quote: since 2026-10-10-mcp-surface serde_json "1" — resolved 1.0.151, already in the graph through dioxus-html, taken by escher-driver alone as its reader of MCP requests, of kept checks and of the live answer line a replay compares (packages/escher-driver/Cargo.toml:17)
@binding n=1
**design-philosophy's identical layout modes ↔ the scope's proof on the stand**: every actionable node the read lists must carry the same verbs, acceptance and refusal cause (disabled, off-screen, covered) in incremental and non-incremental layout, each stand case and minimal fixture run in both modes (architecture/design-philosophy fct-rxrxih7j) (architecture fct-57klyykw).
@binding n=2
**stack-and-technologies' serde_json reader ↔ conventions' hand-written answers**: the new read's answer must be written by hand in the crate-private json module, serde_json only reading its MCP request, its kept check and the live answer line replay compares, with no crate added to escher-driver for it (architecture/stack-and-technologies fct-ovyord5g) (architecture fct-h6zygr3f).
