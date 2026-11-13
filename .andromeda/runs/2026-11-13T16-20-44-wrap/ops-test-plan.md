@row_set n=1 row=decision/ci-actions-pinned-to-sha cell=rule seen=1 detector=none
value: Every action every CI workflow uses is pinned to a commit SHA.
basis: fct-uluruybu
because: The pin check widened from the CI workflow to every workflow, all four now pinned, so a rule naming the CI workflow alone reads stale.
@row_set n=2 row=decision/actions-pinned-to-commit-sha cell=guard seen=1 detector=none
value: WorkflowPinTest
basis: fct-spbatvsg
because: The pin assertions and the toolchain-naming assertion moved out of CiWorkflowTest into the new WorkflowPinTest, which reads every workflow, so the row's guard names the wrong test.
@row_set n=3 row=decision/layout-tests-in-both-layout-modes cell=except seen=1 fact=F1 detector=none
value: Upstream integration targets taken in a sync are kept as written, and those that run one layout mode stand as named exceptions.
basis: fct-kq6gb6jt
because: Six of the seven upstream targets taken run one layout mode and were recorded as exceptions, not edited, so the both-modes rule now carries that named exception.
@row_set n=4 row=decision/mutation-controlled-tests cell=except seen=1 fact=F4 detector=none
value: A control of a CI scripts change takes a narrower entry or a hand control, not the fast leg, whose socket path in the control tool's clone passes the 94-byte state-dir bound.
basis: fct-e6o2igse
because: The fast leg reads red in place in the control tool's clone for a CI scripts change, so the red-before-green convention gains this named exception.
@row_add n=1 row=decision/ci-verdict-latest-attempt area=ci-integration fact=F3 detector=none
name: CI verdict read from the latest attempt
rule: A CI job cancelled for runner capacity with no step run reads red; it is re-run, and the latest attempt's verdict is the one read.
basis: fct-ay63ynwk
because: A runner-capacity cancellation read red until re-run to green at attempt 3, and the verdict is now read from the latest attempt, a convention no row of the CI section states.
@row_none n=1 fact=F2 type=5
what: Two named pixel frame tests and what each pins: an inventory of tests the code holds; the both-modes and CPU-buffer pixel-test decisions already stand.
