@row_set n=1 row=decision/reached-handler-record-without-log cell=rule seen=1 detector=none
value: Where recording of reached handlers is switched on, each reached handler's stable id and event name is written into a root-context record, with no log site and no `NodeId`.
basis: fct-xyhdhgxl
because: The record is no longer written ahead of the handler's dispatch but after it, so the row's timing clause is false; the shorter rule keeps the security promise and leaves the timing to architecture's decision/reached-record-before-dispatch.
