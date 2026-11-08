@answer n=1 entity=ci-integration#1.1 v=no
why: WPT report archive and dispatch run on upstream only; the chunk reaches no wpt workflow or WPT surface
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed and published upstream only; the chunk reads no WPT score
@answer n=3 entity=ci-integration#1.3 v=yes
why: the owed replay test's failure output lands in the leg log uploaded on failure, so its failure message must keep that log free of user data (step and differing key only)
@answer n=4 entity=ci-integration#1.4 v=no
why: the lcov coverage report is line counts only; the chunk sets no coverage floor and excludes no file
@answer n=5 entity=ci-integration#1.5 v=no
why: agent-run state is local, uploaded by no CI leg, and the chunk writes under target/tmp, not target/agent-run
@answer n=6 entity=ci-integration#1.6 v=yes
why: the owed test opens a fresh session whose caller-named state directory under target/tmp holds the socket alone, so the kept check must not be written into it
@answer n=7 entity=ci-integration#1.7 v=yes
why: the owed test's kept actions step is a recording: 0700 directory, 0600 check, answer kept whole, call content at rest, unix only
@answer n=8 entity=ci-integration#1.8 v=no
why: cold-agent pipe state is written by the cold-agent scripts, which the chunk does not change; its server.log span list is not this entity's rule
@answer n=9 entity=ci-integration#1.9 v=no
why: the rule covers the live cold-agent run's checks only; the owed test's kept check is written under target/tmp and not committed
@answer n=10 entity=ci-integration#1.10 v=no standing=S5
why: committed Counter checks hold no typed text, host path or secret, the committed-check convention; the chunk adds no committed check
