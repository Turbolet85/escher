@row_add n=1 row=decision/component-alone-reads-as-in-app area=screen-reader-support fact=F1 detector=none
name: A component alone reads as inside its app
rule: A component rendered alone with given props at the stand's viewport and font reads the accessibility roles, names and states of its in-app rows, its ids those rows with the shell's positional prefix removed, and an equal UI graph; the isolated reading is in-process only, on no carrier.
why: The in-process-only reach was ratified.
guard: stand_component_alone
basis: fct-eqyk6e52, fct-xmlfpf6t, fct-ciwnzvsj
because: The chunk adds an isolated rendering of one component whose accessibility reading is held equal to the in-app one and kept off every carrier, a contract future accessibility checks may rely on.
@row_none n=1 fact=F2 type=4
relied: A check of the pinned 800 x 600 viewport in an isolated document reads the document's viewport, never the root element's bounds, since with no shell CSS the root element sizes to its content.
