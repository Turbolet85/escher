@answer n=1 entity=input-validation#1.51 v=no
why: its CSS declaration rule lives in blitz-dom files the session does not reach
@answer n=2 entity=input-validation#1.52 v=no
why: its selector rule lives in blitz-dom cssom.rs, which the session does not reach
@answer n=3 entity=input-validation#1.53 v=no
why: its query-string rule lives in blitz-dom query_selector.rs, which the session does not reach
@answer n=4 entity=input-validation#1.54 v=no
why: its style-element rule lives in blitz-dom document.rs, which the session does not reach
@answer n=5 entity=input-validation#1.55 v=no
why: its bgcolor rule lives in blitz-dom stylo.rs, which the session does not reach
@answer n=6 entity=input-validation#1.56 v=no
why: its dimension rule lives in blitz-dom stylo.rs, which the session does not reach
@answer n=7 entity=input-validation#1.57 v=no
why: its SVG width and height rule lives in blitz-dom stylo.rs, which the session does not reach
@answer n=8 entity=input-validation#1.58 v=no
why: its border and body margin rule lives in blitz-dom stylo.rs, which the session does not reach
@answer n=9 entity=input-validation#1.59 v=no
why: its markup numeric rule lives in blitz-dom layout files, which the session does not reach
@answer n=10 entity=input-validation#1.60 v=no
why: its rowspan clamp lives in blitz-dom layout/table.rs, which the session does not reach
@answer n=11 entity=input-validation#1.61 v=no
why: its multiple-select size rule lives in blitz-dom and select_popup.rs, none of them reached by the session
@answer n=12 entity=input-validation#1.62 v=no
why: its ol start rule lives in blitz-dom layout/construct.rs, which the session does not reach
@answer n=13 entity=input-validation#1.63 v=no
why: its attr_parsed rule lives in blitz-dom element.rs, which the session does not reach
@answer n=14 entity=input-validation#1.64 v=no
why: its disabled presence rule lives in blitz-dom and snapshot.rs with unreached tests, none changed by the session
@answer n=15 entity=input-validation#1.65 v=no
why: its open attribute rule lives in blitz-dom and open_state.rs, none of them reached by the session
@answer n=16 entity=input-validation#1.66 v=no
why: its autofocus rule lives in blitz-dom mutator.rs and an unreached test, which the session does not reach
@answer n=17 entity=input-validation#1.67 v=no
why: its dir attribute rule lives in an unreached test and blitz-dom, which the session does not reach
@answer n=18 entity=input-validation#1.68 v=no
why: its canvas src rule lives in blitz-dom mutator.rs, which the session does not reach
@answer n=19 entity=input-validation#1.69 v=no
why: its custom widget rule lives in blitz-dom element.rs and mutator.rs, which the session does not reach
@answer n=20 entity=input-validation#1.70 v=yes mode=add-to
why: the coverage answer line adds an id carrier and the Dioxus bridge reached record (F2) adds an in-process id holder, both joining this list, and element_id.rs is reached whole
@answer n=21 entity=input-validation#1.71 v=no
why: its actionable rule reads listeners in actionable.rs, which the session does not reach, and the reached record and coverage role read no listener
@answer n=22 entity=input-validation#1.72 v=yes mode=stale
why: its virtual-node id form rule lives in element_id.rs, a file the session reached whole, and its claim is not shown to hold on that file
@answer n=23 entity=input-validation#1.73 v=yes mode=stale
why: host_log.rs is reached and gains a sixth test reading the coverage run at trace, moving the host log measurement this claim cites
@answer n=24 entity=input-validation#1.74 v=no
why: its aria-describedby and aria-description rule lives in blitz-dom accessibility.rs, which the session does not reach
@answer n=25 entity=input-validation#1.75 v=no
why: its aria-expanded, aria-disabled and aria-level rule lives in blitz-dom accessibility.rs and snapshot.rs, neither reached by the session
