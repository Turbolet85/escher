@account n=1 entity=data-protection#1.1 v=proposed
why: it says the kept `actions` step's replay is owed on «Handler coverage», which stand_replay's new test now exercises
@account n=2 entity=data-protection#2.1 v=not_touched
why: the committed Counter checks are byte-identical (entry 8) and still read only through the strict Check::from_bytes, the coverage role using that same reader
@account n=3 entity=api-security#2.6 v=not_touched
why: VERBS stays at fourteen with no wait argument or field, and the coverage role is a command-line role that adds no verb or transport bound
@account n=4 entity=api-security#2.21 v=not_touched
why: the graph verb and its answer are unchanged; coverage reads the graph in process
@account n=5 entity=api-security#2.22 v=not_touched
why: the actions verb and its answer are unchanged by the session
@account n=6 entity=dependency-security#4.1 v=not_touched
why: Dependencies none and Cargo.lock byte-identical; serde_json still reads a kept check through Check::from_bytes and writes nothing, Coverage::to_json being hand-written
@account n=7 entity=secret-management#2.13 v=not_touched
why: a dated search reading as measured at 2026-10-27; capabilities.md's added ranges are handler-coverage limits prose and the report moves no secret source or env read
@account n=8 entity=error-handling#1.1 v=proposed
why: the coverage role adds a new malformed refusal site and a status-3 boot failure on the command line, the cause set staying eight
@account n=9 entity=error-handling#6.1 v=not_touched
why: the report states poll, initial_build and flush_queued_mounted_events untouched and the record takes no borrow across render_immediate
@account n=10 entity=error-handling#8.1 v=not_touched
why: the render-time task limit is unchanged; the record touches no render path
@account n=11 entity=error-handling#9.1 v=not_touched
why: flush_queued_mounted_events is untouched by the report's Changes
@account n=12 entity=logging-monitoring#1.1 v=proposed
why: host_log gains a coverage run read at RUST_LOG=trace with 0 needles under both builds, a new sink reading
@account n=13 entity=logging-monitoring#1.2 v=proposed
why: escher-session's stdout gains the coverage role's one line and its endings
@account n=14 entity=logging-monitoring#3.1 v=not_touched
why: the select-list host_log and mcp_log tests are unchanged; host_log's added test sits at 141-226
@account n=15 entity=logging-monitoring#6.1 v=not_touched
why: info_span! stays at 2 (entry 10) and the graph and signal_registry modules are unchanged
@account n=16 entity=logging-monitoring#7.1 v=not_touched
why: the actions verb, its span and actionable_nodes are unchanged; info_span! stays at 2
