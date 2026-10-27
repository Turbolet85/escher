# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the WPT report's archive and dispatch are wpt.yml's upstream-only publish, a workflow the chunk neither changes nor runs
ci-integration#1.2 · no · wptscores.json is computed and published by wpt.yml on upstream only, outside the fast and doc legs the chunk re-runs
ci-integration#1.3 · yes · ci-leg.sh writes each leg's merged output to target/ci-logs/{leg}.log, which the fast and doc runs the chunk re-runs leave there, and the chunk's silent tests keep those logs free of user data
ci-integration#1.4 · no · the lcov report is the coverage leg's own upload, and the coverage leg is not one the chunk re-runs
ci-integration#1.5 · no · the agent-run harness state is agent-run.sh's, which the chunk neither runs nor touches
ci-integration#1.6 · no · the socket host's session state directories and the named session checks that make them are neither edited nor re-run by name in the chunk
ci-integration#1.7 · yes · a recorded check keeps a lint's findings whole as its answer, so the factored findings must keep their order and bytes, as the Needs' byte-for-byte written answer requires
ci-integration#1.8 · no · the cold-agent pipe's live-run state and server log, which the chunk does not run and whose rule is the pipe's own
ci-integration#1.9 · no · the cold-agent recording of the pipe's live run, which the chunk does not run and whose rule names no lint value
ci-integration#1.10 · no · the committed Counter checks hold ids, names, values, snapshot text, diffs and calls that the factoring does not change, and the chunk edits neither them nor the replay that reads them
