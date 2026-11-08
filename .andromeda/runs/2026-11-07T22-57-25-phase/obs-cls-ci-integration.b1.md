@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT report archive is the wpt workflow's upstream-only output, and the chunk does not touch wpt or CI artifacts (not_needed item 5)
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed by the WPT run, which the chunk does not touch
@answer n=3 entity=ci-integration#1.3 v=no
why: the per-leg CI log is CI artifact handling, and the chunk adds no CI job or artifact (not_needed item 5)
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage lcov artifact is CI artifact handling, which the chunk does not change (not_needed item 5)
@answer n=5 entity=ci-integration#1.5 v=no
why: the agent-run harness state is written by scripts/agent-run.sh, which the chunk does not change (not_needed item 6)
@answer n=6 entity=ci-integration#1.6 v=no
why: the session state directory holds only the socket, and the chunk writes no file into it since the answers go over the socket and stdout
@answer n=7 entity=ci-integration#1.7 v=yes
why: the recording module writes the new outcome's answer into the kept check, which this row says holds the answer whole, masked, at rest (task item 8)
@answer n=8 entity=ci-integration#1.8 v=no
why: the cold-agent pipe state is written by scripts/cold-agent.sh, which the chunk does not change, and the chunk adds no span to the server.log counts (S4 and item 6)
@answer n=9 entity=ci-integration#1.9 v=yes
why: the live run's escher-session mcp --record child keeps the new verb's answer as a kept check that is never printed or copied, a rule the recording module the chunk edits must honour
@answer n=10 entity=ci-integration#1.10 v=yes
why: the committed Counter checks hold ids, names, values and snapshot text that stand_replay and the freshness check read, and the chunk moves the stand's signals and the snapshot field they carry
