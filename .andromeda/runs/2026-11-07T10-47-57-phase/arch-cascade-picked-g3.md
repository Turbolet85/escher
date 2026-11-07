@account n=1 entity=conventions#4.1 v=carried
why: its "WARNING: DO NOT REORDER" block and TODO markers sit in mutation_writer.rs, the edited file the buffer replays into in order and keeps byte-identical to upstream
@account n=2 entity=conventions#6.1 v=carried
why: the task names its per-call-site tracing rule for the new buffer module, and the crate-local trace! macro lives in the edited lib.rs
@account n=3 entity=conventions#8.1 v=carried
why: the scope names what it rules over — the ported test's home in dioxus_document.rs's tests, the inverted pins in both layout modes with their precondition first, the probe file's module text re-said, an upstream regression test ported
