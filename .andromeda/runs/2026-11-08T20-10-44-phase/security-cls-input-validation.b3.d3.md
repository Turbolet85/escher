@answer n=1 entity=input-validation#1.51 v=no
why: CSS declaration setters are not reached; the coverage read takes handlers and ids only.
@answer n=2 entity=input-validation#1.52 v=no
why: the selectorText setter is not reached; the chunk writes no selector.
@answer n=3 entity=input-validation#1.53 v=no
why: query-string parsing feeds neither the coverage answer nor the owed replay; the chunk runs no selector query.
@answer n=4 entity=input-validation#1.54 v=no
why: style element text decoding is outside the handler record and the actions answer.
@answer n=5 entity=input-validation#1.55 v=no
why: the bgcolor legacy attribute is not read by the chunk's code or tests.
@answer n=6 entity=input-validation#1.56 v=no
why: legacy HTML dimension parsing is not reached by the chunk.
@answer n=7 entity=input-validation#1.57 v=no
why: SVG width and height parsing is not reached by the chunk.
@answer n=8 entity=input-validation#1.58 v=no
why: border and body margin integer parsing is not reached by the chunk.
@answer n=9 entity=input-validation#1.59 v=no
why: markup numeric fallbacks for table, textarea and replaced elements are layout code the chunk does not touch.
@answer n=10 entity=input-validation#1.60 v=no
why: the rowspan clamp is table layout the chunk does not touch.
@answer n=11 entity=input-validation#1.61 v=no
why: the multiple-select size hint is layout the chunk reads neither in the coverage answer nor in the actions answer.
@answer n=12 entity=input-validation#1.62 v=no
why: ol start parsing is not reached by the chunk.
@answer n=13 entity=input-validation#1.63 v=no
why: attr_parsed is an attribute parse helper the chunk does not call, and the scope names no attribute it parses.
@answer n=14 entity=input-validation#1.64 v=yes
why: the presence reading of disabled sets focusability, which the actionable rule of 1.71 reads and the owed replay re-asserts through the actions answer.
@answer n=15 entity=input-validation#1.65 v=no
why: the open state of details and dialog is not named by the scope and feeds no handler record or actions answer.
@answer n=16 entity=input-validation#1.66 v=no
why: autofocus on present attributes is not named by the scope and feeds no handler record.
@answer n=17 entity=input-validation#1.67 v=no
why: the dir attribute match is not reached by the chunk.
@answer n=18 entity=input-validation#1.68 v=no
why: the canvas src u64 parse is not reached; the canvas limit concerns listeners, not src.
@answer n=19 entity=input-validation#1.69 v=no
why: custom widget attribute replay is behind a feature the chunk does not touch and reaches no widget.
@answer n=20 entity=input-validation#1.70 v=yes
why: the stable element id grammar, with its author key and owner path, is the key the coverage list and the graph's owner are built on.
@answer n=21 entity=input-validation#1.71 v=yes
why: the actionable rule and actionable_nodes list the graph's ids, owners and keyed flags over the listener set that the coverage read counts.
@answer n=22 entity=input-validation#1.72 v=yes
why: the reserved virtual-node form {owner}///{key} is a stable-id shape the coverage list keyed on graph ids and owners must keep, and 0.4.0 extends the count to virtual nodes.
@answer n=23 entity=input-validation#1.73 v=no
why: accessible names reach neither the coverage answer nor the actions answer, and the owed test's kept actions answer holds no name.
@answer n=24 entity=input-validation#1.74 v=no
why: aria-describedby and aria-description reach only the platform accessibility adapter, which the chunk does not change.
@answer n=25 entity=input-validation#1.75 v=no
why: aria-expanded, aria-disabled and aria-level move no actionable, snapshot or handler reading the chunk uses.
