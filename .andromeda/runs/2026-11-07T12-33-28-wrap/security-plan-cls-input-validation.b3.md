@answer n=1 entity=input-validation#1.51 v=no
why: the SVG width and height rules live in blitz-dom's stylo.rs and layout code, which the session's dioxus-native-dom render-path change does not reach
@answer n=2 entity=input-validation#1.52 v=no
why: the border and body margin parsing lives in blitz-dom's stylo.rs, which the session does not reach
@answer n=3 entity=input-validation#1.53 v=no
why: the markup numeric attribute parsing lives in blitz-dom's layout code, which the session does not reach
@answer n=4 entity=input-validation#1.54 v=no
why: the rowspan clamp lives in blitz-dom's table.rs, which the session does not reach
@answer n=5 entity=input-validation#1.55 v=no
why: the multiple-select size reading lives in blitz-dom's select_list.rs and stylo.rs, which the session does not reach
@answer n=6 entity=input-validation#1.56 v=no
why: the ol start parse lives in blitz-dom's construct.rs, which the session does not reach
@answer n=7 entity=input-validation#1.57 v=no
why: attr_parsed lives in blitz-dom's element.rs, which the session does not reach and which gains no parse path
@answer n=8 entity=input-validation#1.58 v=no
why: the disabled presence reading lives in blitz-dom's element.rs and events, which the session does not reach, and Dioxus mutations still replay through the same MutationWriter
@answer n=9 entity=input-validation#1.59 v=no
why: the open reading on details and dialog lives in blitz-dom's element.rs and mutator.rs, which the session does not reach
@answer n=10 entity=input-validation#1.60 v=no
why: the autofocus attribute handling lives in blitz-dom's mutator.rs, which the session does not reach
@answer n=11 entity=input-validation#1.61 v=no
why: the dir attribute matching lives in blitz-dom, which the session does not reach
@answer n=12 entity=input-validation#1.62 v=no
why: the canvas src parse lives in blitz-dom's mutator.rs, which the session does not reach
@answer n=13 entity=input-validation#1.63 v=no
why: set_custom_widget's attribute replay lives in blitz-dom's element.rs and mutator.rs, which the session does not reach
@answer n=14 entity=input-validation#1.64 v=no
why: the author-key and component-path id derivation lives in dioxus-native-dom's element_id.rs and the other files it cites, none of them reached, the replayed DOM is the same, and the render buffer keeps attribute values in process and writes no log, DOM or vdom, so the claim holds
@answer n=15 entity=input-validation#1.65 v=no
why: the aria-label and label-for reading lives in blitz-dom's accessibility.rs and document.rs and in dioxus-native-dom's snapshot.rs, none of them reached
@answer n=16 entity=input-validation#1.66 v=no
why: the aria-describedby and aria-description reading lives in blitz-dom's accessibility.rs, which the session does not reach
@answer n=17 entity=input-validation#1.67 v=no
why: the aria-expanded, aria-disabled and aria-level reading lives in blitz-dom's accessibility.rs and dioxus-native-dom's snapshot.rs, neither reached
@answer n=18 entity=input-validation#1.68 v=no
why: the select open and aria-expanded behaviour lives in blitz-dom's accessibility.rs and document flow, which the session does not reach
@answer n=19 entity=input-validation#1.69 v=no
why: the control-state reading lives in blitz-dom's accessibility.rs and dioxus-native-dom's snapshot.rs, neither reached
@answer n=20 entity=input-validation#1.70 v=no
why: the native checked-state reading lives in blitz-dom's accessibility.rs, which the session does not reach
@answer n=21 entity=input-validation#1.71 v=no
why: the inert reading and focus clear live in blitz-dom's node.rs, mutator.rs and accessibility.rs, none reached
@answer n=22 entity=input-validation#1.72 v=no
why: the select list gating lives in blitz-dom's document.rs, which the session does not reach
@answer n=23 entity=input-validation#1.73 v=no
why: the tree focus and inert focus clear live in blitz-dom's accessibility.rs and mutator.rs, which the session does not reach and whose focus semantics it does not change
@answer n=24 entity=input-validation#1.74 v=no
why: the range input rules live in blitz-dom's range.rs, keyboard.rs, pointer.rs and mutator.rs, none reached, and replayed Dioxus value writes keep the same sanitize through the same MutationWriter
@answer n=25 entity=input-validation#1.75 v=no
why: the Enter and Space activation and button click focus live in blitz-dom's keyboard.rs and pointer.rs, which the session does not reach
