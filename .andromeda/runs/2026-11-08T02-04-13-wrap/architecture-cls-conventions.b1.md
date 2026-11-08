@answer n=1 entity=conventions#1.1 v=no
why: the cited gate sites (CI workflows, ci-leg.sh, apps/browser, blitz-dom, wpt) are not reached by the change, and the fmt and clippy gates still hold for the new code
@answer n=2 entity=conventions#2.1 v=yes mode=stale
why: the change reaches dioxus-native-dom/src/lib.rs, which this row cites at :1 for doc_cfg, and the cited line is not re-verified on the changed file; the manifests themselves are unchanged
@answer n=3 entity=conventions#3.1 v=no
why: none of the cited documented files (blitz-dom, blitz-traits, blitz-net, blitz-vibey-script, blitz-test-harness, blitz-shell, examples) is reached by the change
@answer n=4 entity=conventions#4.1 v=no
why: the TODO and DO-NOT-REORDER sites cited here (mutation_writer.rs, dioxus-native/src/lib.rs, dioxus_application.rs, blitz-vibey-script) are not reached by the change
@answer n=5 entity=conventions#5.1 v=no
why: the cited visibility and naming sites sit in blitz-dom, blitz-traits, blitz-vibey-script and stylo_taffy, none of which the change reaches
@answer n=6 entity=conventions#6.1 v=yes mode=add-to
why: task item 15 makes the new graph module gated under accessibility while signal_registry stays ungated, a feature-gating fact this row must carry
@answer n=7 entity=conventions#7.1 v=yes mode=stale
why: the change reaches dioxus_document.rs, which this row cites at :167-172 and :315-320 for the buffered-render borrow pattern; the buffered render is unchanged, but the cited lines move with the edit and are not re-verified
@answer n=8 entity=conventions#8.1 v=yes mode=stale
why: the change reaches dioxus_document.rs and dioxus_document_tests.rs (cited at dioxus_document.rs:908-910), schema.rs and schema_tests.rs, mcp_tests.rs and the shared common and session_common modules, so the cited test-module locations and shared-module lists can move with the change
@answer n=9 entity=conventions#9.1 v=yes mode=stale
why: the change reaches recording_tests.rs, one of the three split-out escher-driver test files this row names, so its module placement and whole-file claim can move with the edit
@answer n=10 entity=conventions#9.2 v=no
why: the fixture module tests/blitz-tests/tests/fixtures/mod.rs and the fixture pages it cites are not reached by the change
@answer n=11 entity=conventions#9.3 v=yes mode=stale
why: the MCP listing test in mcp_tests.rs is reached and its listed-tool count moves from 12 to 13 with graph, so the cited lines 419-500 can move
@answer n=12 entity=conventions#10.1 v=yes mode=add-to
why: every seven_guis task signal now registers through tasks::use_graph_signal with authored write declarations, and the write slice is written const { &[...] } (F10), which this row must carry
@answer n=13 entity=conventions#11.1 v=no
why: examples/assets and the HTML fixtures cited here are not reached by the change
