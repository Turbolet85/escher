@row_set n=1 row=decision/test-module-split-keeps-paths cell=rule seen=1 fact=F1 detector=none
value: A unit-test module moved to a file of its own is moved out whole with its test paths unchanged; an item a split module's child exposes to its parent or to the out-of-line tests is widened to pub(super) and no further, and the tests import by name what the parent no longer holds.
basis: fct-y7i4kmy3, fct-xrrknaow, fct-o6xhelkw, fct-jvsdhjqf
because: The answer writer's split widened each item its parent or its out-of-line tests reach to pub(super) at most and had the tests name the types they had reached through the parent's imports, a convention the row on splitting a test module says less than.
@row_add n=1 row=decision/runnerless-cancelled-job-rerun area=ci-integration fact=F2 detector=none
name: A CI job cancelled with no runner is re-run
rule: A CI job cancelled with no runner acquired is re-run, not fixed, and recorded before each re-run; a red on anything but a missing runner stops the pass.
basis: fct-xhvvlstk, fct-xks6mdiz, fct-ytgkenbi
because: A job cancelled with no runner recurred across two chunks and was re-run rather than fixed under a standing rule that a red of any other kind stops the pass, a CI handling decision no row of the CI section states.
