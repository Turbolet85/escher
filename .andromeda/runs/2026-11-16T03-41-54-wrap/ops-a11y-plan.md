@row_add n=1 row=decision/layout-read-changes-nothing area=screen-reader-support fact=F1 detector=none
name: The layout read changes nothing
rule: The in-process layout read runs no step, pump, tick or settle, and leaves the snapshot, both focus readings, the diff and the clock as they were, an open list open and delivered timer ticks unapplied.
guard: the_read_changes_nothing, the_read_runs_no_pass_that_would_apply_pending_ticks
basis: fct-5qpmkurf
because: The chunk adds an in-process layout read stated to change nothing, a promise beside the actions verb's and the screenshot's that no standing row of the area says of this read.
