@row_set n=1 row=contract/agent-run-events/run-end cell=promise seen=1 fact=F2 detector=none
value: run.end {ts, selection, passed, failed, ignored, cargo_exit, outcome}, printed and appended; a run that ran 0 tests ends with outcome failed and exit 1, never a pass.
basis: fct-5wc27pqf
because: The chunk pins that an empty session run exits 1 with outcome failed, a promise of run.end's outcome the row does not yet state.
@row_none n=1 fact=F1 type=0
what: The session selection emits the existing run.start, test, run.end and status events with no new field and no captured output, which the standing agent-run-events contract rows and agent-run-no-captured-output already state; the selection word itself is held by test-plan's and security-plan's selection rows.
