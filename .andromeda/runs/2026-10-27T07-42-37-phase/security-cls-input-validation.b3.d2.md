# Classified — security · input-validation.b3

## Answers
input-validation#1.51 · yes · construct.rs is a predicted conflict surface, so its `<ol start>` usize parse with one subtracted (construct.rs:485-491) binds whatever the hunks show
input-validation#1.52 · yes · element.rs is reached whole, so attr_parsed's None on parse failure (element.rs:500-503) and the disabled read at element.rs:732-734 bind through the take's merge of that file
input-validation#1.53 · yes · the Needs name disabled read by presence, a rule living in mutator.rs and the event files that a conflict resolution must leave unchanged, and element.rs (reached whole) carries its presence check, witnessed by stand_keyboard.rs and focusability_updates.rs
input-validation#1.54 · yes · autofocus sits at mutator.rs:1114-1126, a predicted conflict surface, so its any-value-but-"false" rule binds whatever the hunks show, and autofocus_attribute.rs runs in the fast leg the task says the chunk re-asserts
input-validation#1.55 · yes · dir_attribute.rs witnesses the case-insensitive match and the fast leg, which the task says the chunk re-asserts, runs that check, so the rule binds on its named witness
input-validation#1.56 · yes · canvas src's u64 parse sits at mutator.rs:1405-1416, a predicted conflict surface, so it binds whatever the hunks show
input-validation#1.57 · yes · the Needs' stable-id entry binds the collision-free id derivation in dioxus-native-dom (dioxus_document.rs, edited by 7fc9ee3af, no NodeId crossing) and the sink allowlist's drop of the id-carrying Stylo targets, which the format.rs re-read covers
input-validation#1.58 · yes · accessibility_label.rs witnesses the label reads, run by the fast leg the task says the chunk re-asserts, so the rule binds though accessibility.rs is not in the reached files
input-validation#1.59 · yes · accessibility_description.rs witnesses the description reads, run by the fast leg the task says the chunk re-asserts, so the rule binds though accessibility.rs is not in the reached files
input-validation#1.60 · yes · accessibility_merged_states.rs witnesses aria-expanded, aria-disabled and aria-level, run by the fast leg the task says the chunk re-asserts, so the rule binds though accessibility.rs is not in the reached files
input-validation#1.61 · yes · accessibility_states.rs witnesses the aria-selected, aria-pressed, aria-checked, aria-invalid and role="switch" readings, run by the fast leg the task says the chunk re-asserts, so the rule binds though accessibility.rs is not in the reached files
input-validation#1.62 · yes · the Needs name inert by presence, its rule sits in node.rs (a predicted conflict surface, lines 563-580 and 1437-1446), and inert_subtree.rs witnesses it
input-validation#1.63 · yes · the Needs name the range and select controls held within their rules, the range rules sit in mutator.rs (a predicted conflict surface, lines 361-367, 388-390 and 525-532) and the event files, and range_input_interaction.rs witnesses the default actions
input-validation#1.64 · yes · the Needs name Enter and Space activation, whose rules live in the event files (keyboard.rs and the button arm of pointer.rs's handle_click), and stand_keyboard.rs, a stand check the task re-asserts in both layout modes, witnesses them
