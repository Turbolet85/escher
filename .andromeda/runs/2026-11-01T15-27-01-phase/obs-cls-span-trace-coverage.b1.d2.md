# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · yes · the examples slice holds `cells.rs`, a conflict surface a resolution can add a span to, and the slice holds the re-read `host_log` and `mcp_log` checks
span-trace-coverage#1.2 · no · the apps slice is outside the reached files, and the Not needed list drops upstream apps' logging, the re-check being of escher's sink
span-trace-coverage#1.3 · no · the 15 s05 files are not named and the entity states no reached file among them
span-trace-coverage#1.4 · no · the 17 listed s06 files are not named and the entity states no reached file among them
span-trace-coverage#1.5 · yes · the layout slice holds `layout/construct.rs`, `layout/inline/build.rs` and `layout/writing_mode.rs`, all conflict surfaces where a merge can move any line and add a span
span-trace-coverage#1.6 · yes · the only match is the doc comment "The node id for the span", the orphan-comment kind the chunk edits in blitz-dom `lib.rs` lines 96-97, and the record must hold after that edit
span-trace-coverage#1.7 · no · the 32 listed s09 files are not named and the entity states no reached file among them
span-trace-coverage#1.8 · yes · the blitz-vibey-script slice holds `runtime/bootstrap.rs` and `runtime/events.rs`, conflict surfaces the take resolves, and the take must add no span there
span-trace-coverage#1.9 · no · the 21 listed s11 files are not named and the entity states no reached file among them
span-trace-coverage#1.10 · yes · the test-crate slice is re-run, and the take carries upstream's new target `pseudo_element_flow_classification`, a conflict surface, which the S1 census must record (0 env, 0 print, 0 subscriber)
span-trace-coverage#1.11 · no · the WPT runner slice is in the Not needed list (upstream WPT tooling, conformance out of scope)
span-trace-coverage#2.1 · yes · the scope names the escher-driver span surface as a read-back of the take: 2 `info_span!` sites, `command`'s eight fields and `stand_act_spans`' children
span-trace-coverage#2.2 · yes · `host_log`'s fourth test, a re-read standing capture check at 0 needles, counts the host's closed `command` span lines and reads 0 id and 0 name needles
span-trace-coverage#2.3 · yes · the select list adds no span or field, and the re-read `host_log` and `mcp_log` select-list tests read 0 needles at trace, the select surfaces being Needs silence records a resolution must keep
span-trace-coverage#3.1 · yes · the scope names the `tool_call` span's two fields and its `stand_act_spans` children as a read-back, and the stand checks re-run in both layout modes measure its `tool` and `ending` fields
span-trace-coverage#3.2 · yes · the scope names `tool_call`'s two fields as a read-back, so the take must add none; the cold-agent pipe's counts are Not needed and are not bound here
