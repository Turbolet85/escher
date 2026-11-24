@row_add n=1 row=absence/keyboard-drag-and-drop area=1-a11y-scope-summary fact=F1 detector=D-a11y-surface
capability: A keyboard or assistive-technology route to drag and drop; a drag reaches a document only as OS pointer drop input
basis: fct-pr3l6fxd
because: The merge brings the OS drop input through the shell with a11y n/a and no keyboard or assistive-technology route, so the scope must state that drag and drop is pointer-only and unchecked.
@row_add n=2 row=decision/adapter-probe-on-sync-lock-move area=9-ci-integration fact=F3 detector=none
name: Adapter probe on every upstream-sync lock move
rule: Each upstream sync that moves the lock re-reads the stand binary's dependency tree, which must still hold neither accesskit_xplat nor accesskit_winit after the take.
basis: fct-dgtampaw
because: The plan's a11y-plan §CI Integration amendment carries the adapter probe on the moved lock (fct-pwiivh4c), a gate no standing row of the section states.
@row_none n=1 fact=F2 type=7
what: The two windowed root examples' unchecked accessibility is an absence inside single example files, held by the report's coverage line and the build leg that compiles them.
