@account n=1 entity=input-validation#1.73 v=not_in_scope
why: the accessibility tree's focus and the inert clear live in blitz-dom's accessibility and mutator code the chunk neither edits nor re-measures; set_focus is re-asserted on the DOM focus, not the tree
@account n=2 entity=input-validation#1.74 v=not_in_scope
why: range input rules in blitz-dom are a component the chunk only paints through, its scope naming none of their values or keys
@account n=3 entity=input-validation#1.75 v=not_in_scope
why: Enter and Space activation is blitz-dom default-action code the chunk does not edit; the inverted pins assert a handler-spawned set_focus after a Tab, not keyboard activation
@account n=4 entity=input-validation#1.77 v=not_in_scope
why: single-select rules are blitz-dom code the chunk only renders through, unnamed by the scope
@account n=5 entity=input-validation#1.82 v=not_in_scope
why: multiple-select keys and clicks are blitz-dom code the chunk only renders through, unnamed by the scope
@account n=6 entity=input-validation#1.83 v=not_in_scope
why: optgroup reading is blitz-dom code the chunk only renders through, unnamed by the scope
@account n=7 entity=input-validation#1.88 v=carried
why: the buffer holds password and file-input values in process and the scope requires the snapshot's values to stay identical, so the mask must hold over the buffered render
@account n=8 entity=input-validation#1.92 v=not_in_scope
why: visible_region serves the driver's in_view, crop and lint, none of which the chunk edits or re-measures; get_client_rect does not read it
@account n=9 entity=input-validation#1.99 v=carried
why: the scope names scroll_to and scroll as MountedData calls now reachable during render, whose clamp and missing-node no-op must hold on the new path
@account n=10 entity=input-validation#1.106 v=carried
why: data-dioxus-id parsing is in dioxus_document.rs, the edited file, and the replay must keep it
@account n=11 entity=input-validation#1.107 v=carried
why: dangerous_inner_html is applied by MutationWriter, through which the buffer is replayed
@account n=12 entity=input-validation#1.108 v=carried
why: unsupported AttributeValue types are ignored by MutationWriter, through which the buffer is replayed
@account n=13 entity=input-validation#1.109 v=carried
why: the falsy boolean rule lives in mutation_writer.rs, kept byte-identical to upstream, and the replay must reach it on both attribute paths
