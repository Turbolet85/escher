@row_add n=1 row=decision/declared-node-read-logs-nothing area=6-log-coverage detector=D-obs-instrumentation
name: The declared-node read logs nothing
rule: The in-process read of a canvas's declared virtual nodes, its validation and its answer have no log site, print, span or field; a refused declaration holds a closed reason and its index alone.
basis: fct-wkaf2evl
because: The chunk adds the declared-node read with no log site, print, span or field (Behaviour), the plan's obs-plan §6 entry, and no row of the area says it; it rests on no fork answer, so it stands unmarked.
