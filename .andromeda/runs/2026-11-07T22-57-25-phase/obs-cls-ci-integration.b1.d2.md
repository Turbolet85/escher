@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT report archive runs only in the upstream wpt workflow, which the chunk does not touch.
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed by the WPT run, which the chunk does not touch.
@answer n=3 entity=ci-integration#1.3 v=no
why: the per-leg CI log is ci-leg.sh build output; the chunk adds no CI leg, job or artifact.
@answer n=4 entity=ci-integration#1.4 v=no
why: the coverage lcov is the coverage leg's artifact; the chunk adds no artifact and names no coverage floor here.
@answer n=5 entity=ci-integration#1.5 v=no
why: agent-run harness state is local and gitignored, and the chunk does not change scripts/agent-run.sh.
@answer n=6 entity=ci-integration#1.6 v=no
why: the session state directory holds only the socket file, and the chunk's new verb writes nothing into it.
@answer n=7 entity=ci-integration#1.7 v=yes
why: the recording module writes each answered call into the kept check, so the new outcome's answer lands there at rest, masked as on every carrier.
@answer n=8 entity=ci-integration#1.8 v=no
why: cold-agent pipe state belongs to cold-agent.sh, which the chunk does not change, and the chunk adds no span name that the server.log count reads.
@answer n=9 entity=ci-integration#1.9 v=no
why: the cold-agent live recording directory's placement, cleanup and never-printed rule belong to cold-agent.sh; the recording write itself is covered by n=7.
@answer n=10 entity=ci-integration#1.10 v=yes
why: the committed Counter checks hold snapshot text and diffs that stand_replay and the freshness check replay, and the chunk's new snapshot field and moved NODE_FIELDS can change that text.
