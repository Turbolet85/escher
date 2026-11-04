@account n=1 entity=test-strategy#9.1 v=proposed
why: its blitz-tests count reads 157 files and 156 targets where the report measures 158 and 157, dioxus_runtime_probe.rs the latest
@account n=2 entity=unit-test-strategy#20.4 v=not_touched
why: the new file's blocks as the report lists them by line are a struct, helpers and test functions, no in-file cfg(test) module, so the absence still holds
@account n=3 entity=e2e-test-strategy#2.1 v=proposed
why: the MountedData panic it records was re-measured at HEAD, pinned by dioxus_runtime_probe in both modes and its fix routed to the port chunk, and F2 belongs beside it
@account n=4 entity=test-data-fixtures#3.4 v=proposed
why: it is the row shape for one-reader pages kept in their check's own file, and F5 adds dioxus_runtime_probe's pages beside it
@account n=5 entity=test-data-fixtures#4.1 v=not_touched
why: its absence is over slice s08, while the new file sits under tests/blitz-tests (slice s12) and the two config lines moved are a version, a checksum and an advisory ID, no fixture or loader
@account n=6 entity=ci-integration#7.1 v=proposed
why: it records a red CI job and its local witness, and F7, the Dependency audit's red on CI#41911351081 and its local reproduction, belongs beside it
@account n=7 entity=ci-integration#14.1 v=not_touched
why: each tally is dated to a named CI run that still reads so, and the report reads no per-OS tally of CI#41914044942 to add
@account n=8 entity=ci-integration#15.1 v=proposed
why: the workspace test leg runs one more target and the report measures the fast leg at 9b3b29714 (221 test result lines ok, 0 FAILED), a re-count this history lacks
