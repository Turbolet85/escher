@answer n=1 entity=ci-integration#1.1 v=no
why: WPT report archiving and dispatch runs only in the upstream wpt workflow, which the chunk's driver read does not touch
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed and published by the WPT run, which the chunk does not reach
@answer n=3 entity=ci-integration#1.3 v=no
why: describes the CI leg's merged build-output log and its artifact upload; the chunk adds no CI job or log writer, and the new checks' silence is the S1 rule
@answer n=4 entity=ci-integration#1.4 v=no
why: describes the coverage leg's lcov artifact and its keyboard_compat exclusion; the chunk adds no coverage floor or exclusion
@answer n=5 entity=ci-integration#1.5 v=no
why: agent-run harness state under target/agent-run, local and uploaded by no leg; the chunk does not touch agent-run.sh or its outputs
@answer n=6 entity=ci-integration#1.6 v=yes
why: the new read's carrier is a socket call into a session whose state directory must hold the socket file alone, and the new stand checks run sessions under target/tmp/ that this row records
@answer n=7 entity=ci-integration#1.7 v=yes
why: the recorder writes each answered read's accepted flag, answer digest and answer whole into the kept check, which this row states is the recording directory's content at rest
@answer n=8 entity=ci-integration#1.8 v=no
why: describes the cold-agent pipe's state directory and its scripts; the chunk changes no file there and adds no span name the pipe counts
@answer n=9 entity=ci-integration#1.9 v=no
why: the cold-agent live run's record directory and its cleanup belong to cold-agent.sh, which the chunk does not change; the kept-check writes are covered by #1.7
@answer n=10 entity=ci-integration#1.10 v=yes
why: the committed Counter checks are replayed by stand_replay with recorded answers and digests, and the chunk edits the verb table and the shared refusal path those answers rest on
