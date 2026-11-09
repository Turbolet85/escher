@answer n=1 entity=input-validation#1.51 v=no
why: its CSS declaration rule lives in blitz-dom, which the session does not reach, and no engine reader changed
@answer n=2 entity=input-validation#1.52 v=no
why: its selectorText rule lives in blitz-dom cssom, which the session does not reach, and no engine reader changed
@answer n=3 entity=input-validation#1.53 v=no
why: its query-string rule lives in blitz-dom query_selector, which the session does not reach, and no engine reader changed
@answer n=4 entity=input-validation#1.54 v=no
why: its style-element text rule lives in blitz-dom document, which the session does not reach, and no engine reader changed
@answer n=5 entity=input-validation#1.55 v=no
why: its bgcolor rule lives in blitz-dom stylo, which the session does not reach, and no engine reader changed
@answer n=6 entity=input-validation#1.56 v=no
why: its dimension-value rule lives in blitz-dom stylo, which the session does not reach, and no engine reader changed
@answer n=7 entity=input-validation#1.57 v=no
why: its SVG width and height rule lives in blitz-dom stylo, which the session does not reach, and no engine reader changed
@answer n=8 entity=input-validation#1.58 v=no
why: its border and body-margin rule lives in blitz-dom stylo, which the session does not reach, and no engine reader changed
@answer n=9 entity=input-validation#1.59 v=no
why: its markup numeric rule lives in blitz-dom layout, which the session does not reach, and no engine reader changed
@answer n=10 entity=input-validation#1.60 v=no
why: its rowspan clamp lives in blitz-dom layout table, which the session does not reach, and no engine reader changed
@answer n=11 entity=input-validation#1.61 v=no
why: its multiple-select size rule lives in blitz-dom and the select_popup tests, none reached by the session, and no engine reader changed
@answer n=12 entity=input-validation#1.62 v=no
why: its ol start rule lives in blitz-dom layout construct, which the session does not reach, and no engine reader changed
@answer n=13 entity=input-validation#1.63 v=no
why: its attr_parsed rule lives in blitz-dom element, which the session does not reach, and no engine reader changed
@answer n=14 entity=input-validation#1.64 v=no
why: its disabled presence rule lives in blitz-dom and its tests in blitz-tests, none reached by the session, and no engine reader changed
@answer n=15 entity=input-validation#1.65 v=no
why: its open-state rule lives in blitz-dom element and mutator, which the session does not reach, and no engine reader changed
@answer n=16 entity=input-validation#1.66 v=no
why: its autofocus rule lives in blitz-dom mutator, which the session does not reach, and no engine reader changed
@answer n=17 entity=input-validation#1.67 v=no
why: its dir rule lives in blitz-dom and its test in blitz-tests, none reached by the session, and no engine reader changed
@answer n=18 entity=input-validation#1.68 v=no
why: its canvas src rule lives in blitz-dom mutator, which the session does not reach, and no engine reader changed
@answer n=19 entity=input-validation#1.69 v=no
why: its custom widget attribute replay lives in blitz-dom element and mutator, which the session does not reach, and no engine reader changed
@answer n=20 entity=input-validation#1.70 v=yes mode=add-to
why: F2 the reached record holds each handled node's stable id beside the session's id record, and the coverage line adds ids as a new exit, so this id holders and exits list gains them, with element_id.rs and dioxus_document_tests.rs reached
@answer n=21 entity=input-validation#1.71 v=no
why: its actionable rule lives in actionable.rs, which the session does not reach, and neither the reached record nor the coverage role reads actionability
@answer n=22 entity=input-validation#1.72 v=yes mode=stale
why: element_id.rs, where the virtual-node form is defined, is a reached file, and the new coverage and reached-record id carriers can move its no-served-form claim, which no report shows still holds
@answer n=23 entity=input-validation#1.73 v=no
why: its accessible-name rule lives in blitz-dom and the snapshot files, none reached by the session, and the coverage line prints ids and counts only, so no name crosses
@answer n=24 entity=input-validation#1.74 v=no
why: its description rule lives in blitz-dom accessibility, which the session does not reach, and the session adds no description exit
@answer n=25 entity=input-validation#1.75 v=no
why: its closed-value rule lives in blitz-dom accessibility, which the session does not reach, and the session moves no aria read
