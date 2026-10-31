# Classified — security · input-validation.b3

## Answers
input-validation#1.51 · no · the rowspan clamp sits in layout/table.rs, a component the chunk only uses, and no named check reads a table span
input-validation#1.52 · no · the ol start parse sits in layout/construct.rs, a component the chunk only uses, and no named check reads a list start
input-validation#1.53 · no · attr_parsed sits in node/element.rs, which the chunk does not reach, and no named reader of the chunk parses a number through it
input-validation#1.54 · yes · disabled read by presence gates Enter and Space activation in events/ and the Tab walk the stand_keyboard rows measure, and snapshot.rs, reached whole, reads enabled from the same presence
input-validation#1.55 · yes · the new :open check on details and dialog rules over this presence read through set_attribute and clear_attribute, and it is the sidecar entry the task names for the open read
input-validation#1.56 · no · autofocus sits in mutator.rs, which the chunk only re-reads for the clear_attribute name, and no unit the chunk edits or checks reads an autofocus value
input-validation#1.57 · no · the case-insensitive dir match is in no reached file or named check, and no scope item reads text direction
input-validation#1.58 · no · the canvas src parse sits in mutator.rs, which the chunk only re-reads for the clear_attribute name, and no canvas path is in scope
input-validation#1.59 · yes · the stable element id's rules decide the ids that lint's overlaps findings and the driver's stale and not-found lookups carry, and the scope names flight-type-one-way and flight-type-return
input-validation#1.60 · yes · the aria-labelledby and label builder in accessibility.rs, an upstream-owned merge surface, binds whatever hunk the chunk shows, and snapshot.rs, reached whole, resolves the same names into SnapshotNode.name
input-validation#1.61 · yes · the aria-describedby builder in accessibility.rs, an upstream-owned merge surface, binds whatever hunk the chunk shows, and snapshot.rs, reached whole, must keep carrying no description
input-validation#1.62 · yes · the expanded, disabled and level writes in accessibility.rs, an upstream-owned merge surface, bind whatever hunk the chunk shows, and snapshot.rs, reached whole, keeps enabled on the disabled presence read
input-validation#1.63 · yes · the native checkbox and radio toggled write that the chunk edits is the rule this entity states, and the snapshot's checked and pressed readings leave as closed words
input-validation#1.64 · yes · Node::is_inert and its ancestor presence walk in node.rs, reached whole, are the rule the inert focus fixup and the inert_subtree pin rest on
input-validation#1.65 · yes · the range default actions in events/keyboard.rs and pointer.rs, reached whole, bind the Enter and Space activation fix, and NodeState.value in snapshot.rs, reached whole, binds for the same reason
input-validation#1.66 · yes · the Enter and Space activation arms and the button click focus in events/, reached whole, are the keyboard-activation debt's own code, held by the stand_keyboard rows the chunk edits
