@row_add n=1 row=decision/virtual-node-answers-carry-id-alone area=pii-scrubbing-compliance fact=F1 detector=none
name: Virtual node crosses by its id alone
rule: The graph, actions and layout answers carry a virtual node only by its id, its canvas's id, the separator and its key, never its declared name.
why: Ratified as the virtual node's crossing.
guard: the_new_answers_carry_the_id_and_no_declared_name
basis: fct-wbnxhw6m, fct-llzzxhy4, fct-w4id572v
because: The graph, actions and layout answers now hold virtual nodes, and what of one crosses in them is a ratified decision no standing row of the section states.
@row_add n=2 row=decision/virtual-node-reads-open-no-span area=span-trace-coverage fact=F3 detector=D-obs-instrumentation
name: Virtual-node reads open no span
rule: Reading a canvas's virtual nodes in the graph, actions, layout, coverage and exploration opens no span and adds no span field or log site, each call riding its existing command span alone.
basis: fct-x6ohav4t, fct-r7v5dywp
because: The graph, actions, layout, coverage and exploration reads now reach virtual nodes with no instrumentation added, and the standing declared-node row cannot take these reads within its bound.
@row_none n=1 fact=F2 type=9
