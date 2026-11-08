@answer n=1 entity=ci-integration#1.1 v=no
why: WPT report archive is the upstream-only wpt workflow, and the chunk touches no wpt code (not_needed item 2)
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed by the WPT run, which the chunk does not touch (not_needed item 2)
@answer n=3 entity=ci-integration#1.3 v=no
why: per-leg CI log artifact handling, and the chunk adds no CI job or artifact (not_needed item 5); its no-user-data claim is held by the chunk's own log and print rules, not by this row
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage lcov artifact's handling is CI artifact handling, which the chunk does not change (not_needed item 5), and no coverage floor is stated here
@answer n=5 entity=ci-integration#1.5 v=no
why: agent-run harness state, and the chunk does not change scripts/agent-run.sh (not_needed item 6)
@answer n=6 entity=ci-integration#1.6 v=no
why: the session state directory is created and held by host.rs, which the chunk does not reach; the graph read's answer returns over the socket and stdout and is never written into that directory
@answer n=7 entity=ci-integration#1.7 v=yes
why: task item 8 has the recording module write each answered call's answer, including the new outcome's, into the kept check at rest, so this row's list of what the recording directory holds and its masked-as-on-every-carrier rule bind
@answer n=8 entity=ci-integration#1.8 v=no
why: the cold-agent pipe's scripts and state are not changed; the server stderr is counted by span name, and the new MCP tool still opens the existing tool_call span, so that count does not move
@answer n=9 entity=ci-integration#1.9 v=yes
why: the new recorded step's answer lands in the kept checks under this live-run record directory, and the rule that kept checks are never printed or copied binds the chunk's recording code (task item 8)
@answer n=10 entity=ci-integration#1.10 v=yes
why: the committed Counter checks hold snapshot text and diffs that the new closed boolean snapshot field reshapes, and stand_replay and the freshness check read them
