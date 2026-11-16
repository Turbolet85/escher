@row_set n=1 row=decision/no-screen-content-in-logs cell=rule seen=2 detector=none
value: No driver call or in-process read puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer, a layout answer, a coverage check and a step's caused text reach no log line at any level.
basis: fct-v77omrom, fct-xz256kkb
because: A step's caused now carries panic payloads, error text and scrubbed fields to the caller, and the founder check reads the sink's lines at trace holding none of it, so the row's list of what reaches no log must name caused.
