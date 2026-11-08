@answer n=1 entity=input-validation#1.51 v=no
why: its rule lives in blitz-dom's HTML entity decoding, which the session does not reach
@answer n=2 entity=input-validation#1.52 v=no
why: the bgcolor parse is in blitz-dom's stylo.rs, which the session does not reach
@answer n=3 entity=input-validation#1.53 v=no
why: the dimension parser is in blitz-dom's stylo.rs and its tests, none reached by the session
@answer n=4 entity=input-validation#1.54 v=no
why: the SVG width and height rejection is in blitz-dom's stylo.rs, which the session does not reach
@answer n=5 entity=input-validation#1.55 v=no
why: the border and body margin parse is in blitz-dom's stylo.rs, which the session does not reach
@answer n=6 entity=input-validation#1.56 v=no
why: its numeric attribute fallbacks sit in blitz-dom layout files, none reached by the session
@answer n=7 entity=input-validation#1.57 v=no
why: the rowspan clamp is in blitz-dom's table layout, which the session does not reach
@answer n=8 entity=input-validation#1.58 v=no
why: the multiple-select size read and its tests are in blitz-dom and tests the session does not reach
@answer n=9 entity=input-validation#1.59 v=no
why: the ol start parse is in blitz-dom's construct.rs, which the session does not reach
@answer n=10 entity=input-validation#1.60 v=no
why: attr_parsed is in blitz-dom's element.rs, which the session does not reach
@answer n=11 entity=input-validation#1.61 v=no
why: the task says the rows naming unkeyed_actionable were re-read and found unmoved, and focusability still reads disabled through the shared rule
@answer n=12 entity=input-validation#1.62 v=no
why: the open attribute reading is in blitz-dom's element.rs and mutator.rs, which the session does not reach
@answer n=13 entity=input-validation#1.63 v=no
why: the autofocus reading is in blitz-dom's mutator.rs, which the session does not reach
@answer n=14 entity=input-validation#1.64 v=no
why: the dir match is in blitz-dom's own test file, which the session does not reach
@answer n=15 entity=input-validation#1.65 v=no
why: the canvas src parse is in blitz-dom's mutator.rs, which the session does not reach
@answer n=16 entity=input-validation#1.66 v=no
why: set_custom_widget is in blitz-dom's element.rs and mutator.rs, which the session does not reach
@answer n=17 entity=input-validation#1.67 v=yes mode=stale
why: VERBS moves from thirteen to fourteen, and the actions read (F1, F4) puts ids and owner ids onto the answer through element_ids and actionable_nodes, with execute.rs and actionable.rs reached
@answer n=18 entity=input-validation#1.68 v=no
why: the actions answer reads ids from the same element_ids walk, and no walk builds the /// form
@answer n=19 entity=input-validation#1.69 v=yes mode=stale
why: examples/seven_guis/tests/mcp_log.rs, which the session reaches whole, gained actions needles beside the cited 57-352 range, so its citation and its needle reading moved
@answer n=20 entity=input-validation#1.70 v=no
why: the aria-describedby builder is in blitz-dom's accessibility.rs and its tests, none reached by the session
@answer n=21 entity=input-validation#1.71 v=no
why: the aria-expanded, aria-disabled and aria-level reads are in blitz-dom's accessibility.rs and the snapshot, neither reached by the session
@answer n=22 entity=input-validation#1.72 v=no
why: the select open and aria-expanded steering is in blitz-dom's accessibility.rs and select_popup tests, none reached by the session
@answer n=23 entity=input-validation#1.73 v=yes mode=stale
why: json.rs, which the session reaches whole, gained the actions answer's writer, so the cited json.rs:159-165 range may have moved
@answer n=24 entity=input-validation#1.74 v=no
why: the native checked-state write is in blitz-dom's accessibility.rs, which the session does not reach
@answer n=25 entity=input-validation#1.75 v=no
why: the task says the rows naming unkeyed_actionable were re-read and found unmoved, and inert is read in blitz-dom, which the session does not reach
