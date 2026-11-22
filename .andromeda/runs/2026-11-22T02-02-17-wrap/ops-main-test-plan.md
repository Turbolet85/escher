@row_add n=1 row=decision/hosted-runner-cancel-hand-rerun area=9-ci-integration detector=none
name: Hosted-runner cancel answered by a hand re-run
rule: A CI job cancelled with no runner and no step run is a limit of the hosted runners, answered by a hand re-run of the cancelled job; no automatic re-run is built.
why: An automatic re-run needs a write-scoped CI token, and the CI token stays read-only.
state: provisional
basis: fct-gyir65wd
because: The chunk retired the cancelled-runner watch by its rule and stated the no-runner cancel as a hosted-runner limit with the hand re-run, carried to test-plan §9, on an answer delegated for the founder and held provisional.
@row_add n=2 row=decision/ci-flag-rebuild-limit area=9-ci-integration detector=none
name: CI flag rebuild a stated limit
rule: A dev-host leg run with CI set rebuilds the workspace's own units beside the dev host's, a measured limit of Cargo under CI; the leg script does not export CI.
basis: fct-sods3b6s
because: The plan's test-plan §9 entry carries the CI=true rebuild as a measured limit with its override, and no proposer raised it.
