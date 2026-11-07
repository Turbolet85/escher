@answer n=1 entity=input-validation#1.51 v=no
why: the SVG width and height reject rule lives in blitz-dom stylo.rs, which the session's render-path change does not reach
@answer n=2 entity=input-validation#1.52 v=no
why: the border and body margin unsigned-integer parse lives in blitz-dom stylo.rs, which the session does not reach
@answer n=3 entity=input-validation#1.53 v=no
why: the markup numeric fallbacks live in blitz-dom layout code, which the session does not reach
@answer n=4 entity=input-validation#1.54 v=no
why: the rowspan clamp lives in blitz-dom table.rs, which the session does not reach
@answer n=5 entity=input-validation#1.55 v=no
why: the multiple-select size hint lives in blitz-dom select_list.rs and stylo.rs, which the session does not reach
@answer n=6 entity=input-validation#1.56 v=no
why: the ol start parse lives in blitz-dom construct.rs, which the session does not reach
@answer n=7 entity=input-validation#1.57 v=no
why: attr_parsed lives in blitz-dom element.rs, which the session does not reach
@answer n=8 entity=input-validation#1.58 v=no
why: the disabled presence reading and its readers live in blitz-dom element.rs and events and in dioxus-native-dom snapshot.rs and actionable.rs, none changed by the buffered render, which replays the same mutations in order
@answer n=9 entity=input-validation#1.59 v=no
why: the open rule lives in blitz-dom element.rs and mutator.rs, which the session does not reach
@answer n=10 entity=input-validation#1.60 v=no
why: the autofocus rule lives in blitz-dom mutator.rs, which the session does not reach, and the replay still calls that same mutator path
@answer n=11 entity=input-validation#1.61 v=no
why: the dir attribute match lives in blitz-dom, which the session does not reach
@answer n=12 entity=input-validation#1.62 v=no
why: the canvas src parse lives in blitz-dom mutator.rs, which the session does not reach
@answer n=13 entity=input-validation#1.63 v=no
why: the set_custom_widget attribute replay lives in blitz-dom element.rs and mutator.rs, which the session does not reach, and the widget still receives the same attribute calls through the replay
@answer n=14 entity=input-validation#1.64 v=no
why: the id derivation lives in dioxus-native-dom element_id.rs and its readers in snapshot.rs and actionable.rs, none reached, and the session changes no id derivation while the buffer replays the same mutations in order
@answer n=15 entity=input-validation#1.65 v=no
why: the aria-label and label-for name rules live in blitz-dom accessibility.rs, which the session does not reach
@answer n=16 entity=input-validation#1.66 v=no
why: the aria-description and aria-describedby builder lives in blitz-dom accessibility.rs, which the session does not reach
@answer n=17 entity=input-validation#1.67 v=no
why: the aria-expanded, aria-disabled and aria-level parses live in blitz-dom accessibility.rs, which the session does not reach
@answer n=18 entity=input-validation#1.68 v=no
why: the select steering rule for open and aria-expanded lives in blitz-dom accessibility.rs and select_popup, which the session does not reach
@answer n=19 entity=input-validation#1.69 v=no
why: the control-state readings live in blitz-dom accessibility.rs and dioxus-native-dom snapshot.rs, which the session does not reach
@answer n=20 entity=input-validation#1.70 v=no
why: the native checked reading lives in blitz-dom accessibility.rs, which the session does not reach
@answer n=21 entity=input-validation#1.71 v=no
why: the inert rule lives in blitz-dom node.rs, accessibility.rs and mutator.rs, which the session does not reach
@answer n=22 entity=input-validation#1.72 v=no
why: select_list_allowed lives in blitz-dom document.rs, which the session does not reach
@answer n=23 entity=input-validation#1.73 v=no
why: the inert focus clear and the tree focus rule live in blitz-dom accessibility.rs and mutator.rs, which the session does not reach
@answer n=24 entity=input-validation#1.74 v=no
why: the range value, bounds and default actions live in blitz-dom range.rs, keyboard.rs, pointer.rs, mutator.rs and form.rs, none reached by the session
@answer n=25 entity=input-validation#1.75 v=no
why: the Enter and Space activation arms live in blitz-dom keyboard.rs and events, which the session does not reach
