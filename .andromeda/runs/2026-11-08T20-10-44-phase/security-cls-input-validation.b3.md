@answer n=1 entity=input-validation#1.51 v=no
why: CSS declaration setters are not reached by the coverage read or its replay test
@answer n=2 entity=input-validation#1.52 v=no
why: selectorText setter is not reached by the chunk
@answer n=3 entity=input-validation#1.53 v=no
why: query-string selector parsing is not reached; the chunk parses no selector
@answer n=4 entity=input-validation#1.54 v=no
why: style element text entity decoding is not reached
@answer n=5 entity=input-validation#1.55 v=no
why: legacy bgcolor parsing is not reached; the chunk reads no presentational attribute
@answer n=6 entity=input-validation#1.56 v=no
why: legacy dimension parsing is not reached; the chunk reads no layout attribute
@answer n=7 entity=input-validation#1.57 v=no
why: SVG width and height parsing is not reached
@answer n=8 entity=input-validation#1.58 v=no
why: border and body margin parsing is not reached
@answer n=9 entity=input-validation#1.59 v=no
why: markup numeric attribute defaults are not reached; the chunk parses no numeric attribute
@answer n=10 entity=input-validation#1.60 v=no
why: rowspan clamping in table layout is not reached
@answer n=11 entity=input-validation#1.61 v=no
why: the multiple-select size hint is an engine layout rule the chunk does not touch
@answer n=12 entity=input-validation#1.62 v=no
why: ol start parsing is not reached
@answer n=13 entity=input-validation#1.63 v=no
why: attr_parsed is not read by the chunk
@answer n=14 entity=input-validation#1.64 v=no
why: the disabled presence reading (focusability, clickable, activation) is not read by the coverage answer, which carries no state and runs no step
@answer n=15 entity=input-validation#1.65 v=no
why: the open state of details and dialog is not reached
@answer n=16 entity=input-validation#1.66 v=no
why: autofocus is not reached
@answer n=17 entity=input-validation#1.67 v=no
why: the dir attribute is not reached
@answer n=18 entity=input-validation#1.68 v=no
why: canvas src u64 parsing is not reached
@answer n=19 entity=input-validation#1.69 v=no
why: custom widget attribute replay is not reached
@answer n=20 entity=input-validation#1.70 v=yes
why: the coverage list keys on the graph's stable id with its owner, so the id grammar (author key, no duplicate or slash rule, no id written to a log) binds
@answer n=21 entity=input-validation#1.71 v=yes
why: the coverage counts DOM listeners on agent-visible nodes, one per (node, event), which is the listener reading the actionable rule takes (current listeners other than mounted)
@answer n=22 entity=input-validation#1.72 v=yes
why: the coverage keys on graph node ids, and the reserved {owner}///{key} virtual-node form is part of the id grammar the coverage ids must keep against the 0.4.0 virtual-node extension
@answer n=23 entity=input-validation#1.73 v=no standing=S3
why: the no-name-in-log rule is the escher_ log-line convention, and the coverage answer carries no accessible name
@answer n=24 entity=input-validation#1.74 v=no
why: aria-describedby and aria-description reach only the platform adapter, and the chunk adds no description read
@answer n=25 entity=input-validation#1.75 v=no
why: aria-expanded, aria-disabled and aria-level closed parses reach only the adapter, and the snapshot's enabled reading is untouched
