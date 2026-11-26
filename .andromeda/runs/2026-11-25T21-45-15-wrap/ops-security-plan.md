@row_add n=1 row=decision/canvas-answer-node-index area=threat-model-summary detector=none
name: Canvas answer carries a virtual node by its index
rule: Each command line of a canvas answer, whole or in parts, carries a virtual node only as the 1-based index of the canvas's virtual node whose region holds the command's centre, or none, a number alone, never the node's id or declared name.
why: Ratified.
guard: stand_circle_nodes the_full_answer_and_its_parts_carry_numbers_only_in_the_tag
basis: fct-yf6akjxu, fct-l3p3ykbb, fct-vufwaraf
because: The canvas answer now tags each command line with a virtual node's index, and canvas-content-crossing's rule cannot take that clause within its bound, so the crossing's new number stands as a row of its own.
@row_add n=2 row=decision/coverage-line-virtual-node-id area=threat-model-summary detector=none
name: Coverage line carries a virtual node by its id
rule: The coverage answer line lists a canvas's virtual node's click by its `{owner}///{key}` id, the canvas's id as its owner, never the node's declared name.
guard: stand_circle_nodes a_kept_check_lists_the_unclicked_circle_s_click
basis: fct-yf6akjxu, fct-l3p3ykbb
because: The coverage line now carries a virtual node's id, a carrier virtual-node-crossing and the virtual-node-form contract do not name and cannot take within their bounds, so the new carrier stands as a row of its own.
@row_none n=1 fact=F1 type=4
relied: Circle Drawer declares a circle as a virtual node only for an entry its data-circles parse takes, its id the node's key.
