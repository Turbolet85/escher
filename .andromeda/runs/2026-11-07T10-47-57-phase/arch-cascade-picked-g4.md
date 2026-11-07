@account n=1 entity=standard-contracts#1.1 v=carried
why: DioxusDocument's Document impl holds the poll the chunk edits, and the changed set and stable-id tree must read the same after a buffered render
@account n=2 entity=standard-contracts#2.2 v=not_in_scope
why: select_value is a read-only select reader the chunk neither changes nor names; the snapshot it feeds is re-checked by stand_* unchanged
@account n=3 entity=standard-contracts#2.4 v=not_in_scope
why: the select list's reader lives in blitz-dom's document.rs, which the chunk does not reach
@account n=4 entity=standard-contracts#2.5 v=not_in_scope
why: the select list's reconcile order in resolve and its removal clear are blitz-dom internals the replay leaves unchanged and the scope does not name
@account n=5 entity=standard-contracts#2.6 v=not_in_scope
why: the select readers and closed_select_of are untouched by the render path and named by no line of the scope
@account n=6 entity=standard-contracts#2.7 v=not_in_scope
why: the mutator's multiple-select record is a blitz-dom rule the in-order replay calls unchanged, with no select named in the scope
@account n=7 entity=standard-contracts#3.1 v=carried
why: the replayed buffer reaches the document only through doc.mutate()'s methods, the calls the new buffer module must drive
@account n=8 entity=standard-contracts#3.2 v=not_in_scope
why: a single select's expanded write is accessibility-builder behaviour the chunk does not reach
@account n=9 entity=standard-contracts#3.3 v=not_in_scope
why: the tree's focus fallback is a builder rule; the pins and the ported test read the focused node, not the tree's focus
@account n=10 entity=standard-contracts#3.4 v=not_in_scope
why: a native control's toggled write is builder behaviour the chunk does not reach
@account n=11 entity=standard-contracts#3.6 v=carried
why: the replay must call the mutator's attribute and child-list methods so restyle and the changed-set insert run as in the direct write, which incremental_oracle re-witnesses
@account n=12 entity=standard-contracts#4.1 v=carried
why: set_focus from a polled task must reach the engine's focus change, which the inverted pins and the ported test assert
@account n=13 entity=standard-contracts#5.1 v=carried
why: the scope names MountedData scroll_to as a call that now reaches the document, and stale_node_mapping, re-read here, witnesses stale-id panics
@account n=14 entity=standard-contracts#6.1 v=carried
why: the ported test calls get_client_rect, a reader of get_client_bounding_rect this contract names
@account n=15 entity=standard-contracts#6.4 v=not_in_scope
why: the hit walk's skip of dropped nodes is blitz-dom behaviour the chunk neither changes nor names
@account n=16 entity=standard-contracts#10.1 v=not_in_scope
why: the disabled reading is untouched by the render path
@account n=17 entity=standard-contracts#14.1 v=not_in_scope
why: the windowed shell's redraw transport is unchanged and read, not run; the replay requests redraws through the same mutator
@account n=18 entity=standard-contracts#15.1 v=carried
why: the inverted pins drive DioxusDocument::poll through Harness<DioxusDocument>'s pump in both layout modes
@account n=19 entity=standard-contracts#17.1 v=carried
why: the stand, named by the scope as a document that renders through the buffer, boots each task as a fresh VirtualDom on Harness<DioxusDocument>, and stand_* re-witness it
@account n=20 entity=standard-contracts#18.1 v=not_in_scope
why: the stand's font is unchanged and nothing the chunk re-pins is measured against it
@account n=21 entity=standard-contracts#19.1 v=not_in_scope
why: boot_cells is a stand boot the scope does not name
@account n=22 entity=standard-contracts#19.2 v=not_in_scope
why: boot_load's page is a stand boot the scope does not name
@account n=23 entity=standard-contracts#19.3 v=not_in_scope
why: LoadNet is a network stand-in the render path does not reach
@account n=24 entity=standard-contracts#20.1 v=not_in_scope
why: the telemetry sink's allowlist and scrub stay unchanged, and the buffer adds no log site
