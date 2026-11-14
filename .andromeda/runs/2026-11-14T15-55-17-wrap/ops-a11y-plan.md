@row_add n=1 row=decision/exploration-reads-snapshot-and-actions area=screen-reader-support fact=F1 detector=none
name: Exploration reads only the snapshot and the actions listing
rule: Auto-exploration reads only the snapshot and the actions listing, explores only the target verbs whose result holds no image field, chosen by the field's kind so a screenshot is never explored, and its report holds closed words, indices and calls, never screen text, a NodeId or a value.
basis: fct-xetejwtu
because: The chunk adds an in-process exploration that reads the snapshot and actions only, drops verbs whose result field is a Png kind and reports closed words, indices and calls only, a rule future readers of the snapshot must respect and no row of the section states.
@row_none n=1 fact=F2 type=4
relied: Exploration counts two screens with the same snapshot text form as one screen, so what the text form writes decides which screens it tells apart.
