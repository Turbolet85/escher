@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT workflow lines it cites are not in the change's reach, and the session changes no workflow outside the cold-agent test
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed by the WPT workflow, which the session does not reach
@answer n=3 entity=ci-integration#1.3 v=no
why: ci-leg.sh and ci.yml are not reached, so each leg's merged log path and upload rule stand and CI stays green on 2ce2820b
@answer n=4 entity=ci-integration#1.4 v=no
why: ci-leg.sh and ci.yml are not reached, and the new dioxus-native-dom modules join the lcov report without any change to its stated text
@answer n=5 entity=ci-integration#1.5 v=no
why: agent-run.sh is not reached, and the session moves only a run's tally, not the harness state files or their fields
@answer n=6 entity=ci-integration#1.6 v=yes mode=stale
why: host_log.rs and the seven_guis common module gained lines and session_common gained graph and graphed, so the cited lines host_log.rs:421, common/mod.rs:17-25 and session_common/mod.rs:348-356 can be off
@answer n=7 entity=ci-integration#1.7 v=no
why: the recording module is unedited and keeps every verb's answer whole, and a graph answer adds only ids and names with no values, already within the at-rest list
@answer n=8 entity=ci-integration#1.8 v=yes mode=stale
why: scripts/cold-agent.sh is reached and gained graph in its VERBS and a fake-server answer, so its cited lines 17, 555-588 and 593-596 can move, though the state-file claim itself holds
@answer n=9 entity=ci-integration#1.9 v=yes mode=stale
why: scripts/cold-agent.sh is reached and gained the VERBS and fake-server edits, so its cited lines 257-271 and 570-577 can move, though the committed-checks claim holds
@answer n=10 entity=ci-integration#1.10 v=no
why: the task states the CI artifact text naming the two committed checks still holds after the re-record, with no typed text, host path or secret
