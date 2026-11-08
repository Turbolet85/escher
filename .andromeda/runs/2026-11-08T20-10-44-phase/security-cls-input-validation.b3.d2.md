@answer n=1 entity=input-validation#1.51 v=no
why: CSS declaration writes through set_style_property sit outside the coverage read, the bridge dispatch and the replay test
@answer n=2 entity=input-validation#1.52 v=no
why: the selectorText setter is not called or read by any chunk file
@answer n=3 entity=input-validation#1.53 v=no
why: query-string selector parsing is not reached, since the chunk reads the graph and the session's own handler set
@answer n=4 entity=input-validation#1.54 v=no
why: style element text decoding is not reached by any chunk file
@answer n=5 entity=input-validation#1.55 v=no
why: the bgcolor legacy attribute is not read by the graph, the actions verb or the replay
@answer n=6 entity=input-validation#1.56 v=no
why: legacy dimension parsing is not read by the chunk
@answer n=7 entity=input-validation#1.57 v=no
why: SVG width and height parsing is not read by the chunk
@answer n=8 entity=input-validation#1.58 v=no
why: border and body-margin parsing are not read by the chunk
@answer n=9 entity=input-validation#1.59 v=no
why: rows, cols, span and colspan layout reads feed no coverage list, actions answer or kept check
@answer n=10 entity=input-validation#1.60 v=no
why: rowspan clamping is layout only and reaches no answer the chunk reads or keeps
@answer n=11 entity=input-validation#1.61 v=no
why: a multiple select's size hint is a layout reading that reaches neither the coverage list nor the actions answer
@answer n=12 entity=input-validation#1.62 v=no
why: ol start parsing is not read by the chunk
@answer n=13 entity=input-validation#1.63 v=yes
why: the tabindex parse that returns None on failure decides focusability, which the actions answer the owed replay test keeps and compares reads
@answer n=14 entity=input-validation#1.64 v=yes
why: the presence reading of disabled decides focusability and clickability, and the actions answer the replay asserts reads focusability
@answer n=15 entity=input-validation#1.65 v=no
why: the open state reaches neither the actions answer nor the coverage list, and no chunk site touches details or dialog
@answer n=16 entity=input-validation#1.66 v=no
why: autofocus at mount is not an input to the coverage list, the actions answer or the replayed session
@answer n=17 entity=input-validation#1.67 v=no
why: dir attribute matching is not read by the chunk
@answer n=18 entity=input-validation#1.68 v=no
why: canvas src u64 parsing is not the canvas limit the chunk states, which concerns handlers inside one canvas listener
@answer n=19 entity=input-validation#1.69 v=no
why: custom widget attribute replay sits behind a feature the chunk does not touch
@answer n=20 entity=input-validation#1.70 v=yes
why: the coverage list keys on the graph's stable id and owner, so the id grammar's shape and its no-log rule bind the answer
@answer n=21 entity=input-validation#1.71 v=yes
why: the actions verb reads the actionable rule, and its ActionNode ids, owner ids and keyed flags are what the owed replay test asserts
@answer n=22 entity=input-validation#1.72 v=yes
why: the reserved virtual-node form is part of the stable-id grammar the coverage list keys on, and no served id may take it
@answer n=23 entity=input-validation#1.73 v=no
why: accessible names are read by neither the graph, the coverage answer nor the actions answer, and the chunk adds no name log site
@answer n=24 entity=input-validation#1.74 v=no
why: aria-describedby and aria-description reach only the platform accessibility adapter, which the chunk does not read
@answer n=25 entity=input-validation#1.75 v=no
why: aria-expanded, aria-disabled and aria-level move no reading the graph or the actions answer makes, since enabled keeps reading the disabled attribute by presence
