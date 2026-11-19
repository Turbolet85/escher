@row_add n=1 row=decision/target-sweep-writes-no-log area=ci-integration detector=D-obs-instrumentation
name: Build-directory sweep writes no log
rule: The build-directory sweep writes no log; its one stdout line of closed words and counts is its only record, printed as is by the fast leg.
why: No log by design: the line carries the outcome.
basis: fct-yez7fafg
because: The sweep is uninstrumented by design and obs-plan must say so; the proposer's row joined the env read, whose home is security-plan, so the no-log fact stands alone here.
