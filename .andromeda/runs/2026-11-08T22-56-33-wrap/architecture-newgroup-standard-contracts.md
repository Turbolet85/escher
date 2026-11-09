@verdict n=1 ref_file=fanout-architecture--cascade-g3.md ref_n=2 v=distinct
why: no standing group states a coverage role of the command line; the reader's rules name serve, mcp and help roles only

@verdict n=2 ref_file=fanout-architecture--cascade-g3.md ref_n=5 v=contradiction fact=fct-t7zq5feb
quote: the crate re-exports 59 names under `#![deny(missing_docs)]`
why: a public handler_coverage with Coverage and HandlerRef at the crate root makes the standing count and list of 59 re-exported names stale

@verdict n=3 ref_file=fanout-architecture--cascade-g3.md ref_n=6 v=distinct
why: no standing group states handler coverage's limits; the standing stated limits are the graph's and the actions read's

@verdict n=4 ref_file=fanout-architecture--cascade-g4.md ref_n=1 v=contradiction fact=fct-t7zq5feb
quote: the crate re-exports 59 names under `#![deny(missing_docs)]`
why: handler_coverage, Coverage and HandlerRef each re-exported contradict the standing count and list of 59 re-exported names; otherwise the same fact as g3 n=5

@verdict n=5 ref_file=fanout-architecture--cascade-g4.md ref_n=2 v=twin to_file=fanout-architecture--cascade-g3.md to_n=6
why: the same three stated limits of handler coverage, with their owners added

@verdict n=6 ref_file=fanout-architecture--cascade-g4.md ref_n=3 v=distinct
why: the standing dispatch group states the SignalRegistry arm only; no group states the reached record's write on handle_event

@verdict n=7 ref_file=fanout-architecture--cascade-g4.md ref_n=5 v=distinct
why: no standing group states the reached module, ReachedHandler, record_reached or reached_handlers

@verdict n=8 ref_file=fanout-architecture--cascade-g4.md ref_n=6 v=twin to_file=fanout-architecture--cascade-g3.md to_n=2
why: the same coverage role, shape, bounded read and four endings, with the boot closure and usage clause added
