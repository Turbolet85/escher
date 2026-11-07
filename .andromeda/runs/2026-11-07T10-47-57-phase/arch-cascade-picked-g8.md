@account n=1 entity=existing-scopes#1.7 v=not_in_scope
why: locates blitz-dom's hit testing, focus and geometry code, which the chunk only calls unchanged through MountedData; it states no behaviour the chunk re-asserts
@account n=2 entity=existing-scopes#1.8 v=not_in_scope
why: blitz-dom's accessibility tree builder and debug printing, neither reached nor re-read by the render path's change
@account n=3 entity=existing-scopes#1.9 v=not_in_scope
why: lists blitz-dom's event submodules, which set_focus reaches unchanged; it states no behaviour the chunk re-asserts
@account n=4 entity=existing-scopes#1.10 v=not_in_scope
why: blitz-dom's scrolling and selection, used unchanged by MountedData's scroll calls; the scope asserts none of their behaviour
@account n=5 entity=existing-scopes#1.12 v=not_in_scope
why: blitz-dom's node storage and traversal, used unchanged by the replayed mutations
@account n=6 entity=existing-scopes#1.13 v=not_in_scope
why: blitz-dom's layout modules, which the chunk neither edits nor names; the layout identity it re-reads is witnessed by incremental_oracle
@account n=7 entity=existing-scopes#1.14 v=not_in_scope
why: blitz-dom's node module, which the chunk neither edits nor re-reads
@account n=8 entity=existing-scopes#1.19 v=not_in_scope
why: blitz-shell's modules, which the windowed launch drives unchanged, and the windowed path is read, not run
@account n=9 entity=existing-scopes#1.22 v=carried
why: dioxus-native depends on dioxus-native-dom, which is how the windowed launch takes the buffered render the scope names
@account n=10 entity=existing-scopes#1.23 v=carried
why: the module list of the crate whose lib.rs, dioxus_document.rs and mutation_writer.rs the chunk edits, and which gains the buffer module
@account n=11 entity=existing-scopes#1.25 v=not_in_scope
why: escher-telemetry's formatter and panic hook; nothing crosses and the buffer logs nothing, so the chunk does not reach it
@account n=12 entity=existing-scopes#1.31 v=carried
why: records stale_node_mapping and the stand_* checks the chunk re-reads as witnesses that behaviour is unchanged
@account n=13 entity=existing-scopes#2.1 v=carried
why: records stand_cells_served and stand_load_page, stand_* checks the chunk re-reads as witnesses
@account n=14 entity=existing-scopes#3.2 v=not_in_scope
why: the shared test modules' reader counts; the chunk adds no reader of them and inverts two tests in a file that reads none
@account n=15 entity=existing-scopes#4.2 v=carried
why: records dioxus_runtime_probe's ten tests and the two should_panic hazard pins the chunk inverts
