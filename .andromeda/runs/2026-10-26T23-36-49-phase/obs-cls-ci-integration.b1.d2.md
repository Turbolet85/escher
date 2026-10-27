# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the WPT report archive and its dispatch are wpt.yml's upstream-only jobs, which the chunk neither runs nor re-reads
ci-integration#1.2 · no · wptscores.json is computed and published by wpt.yml's upstream-only run, which the chunk does not reach
ci-integration#1.3 · yes · ci-leg.sh writes the fast and doc legs the chunk re-runs to target/ci-logs/{leg}.log unscrubbed, where its edited tests' output lands, so those tests must print no user data
ci-integration#1.4 · no · the coverage leg's lcov and its success-only artifact belong to the coverage leg, which the chunk neither runs nor re-reads
ci-integration#1.5 · no · target/agent-run/ is written only by scripts/agent-run.sh, which the chunk neither runs nor touches
ci-integration#1.6 · no · the session state directories are made by host.rs and the session checks, which the chunk neither edits nor re-reads, and no edited unit creates one
ci-integration#1.7 · yes · the recording directory keeps a lint's findings at rest as its answer kept whole, so the refactor must keep each finding's closed ids, words and numbers and their order, as the Needs' recorded-check clause requires
ci-integration#1.8 · no · the cold-agent pipe's files and its server.log span counts come from cold-agent.sh's live run, which the chunk neither runs nor touches, and the chunk adds no span
ci-integration#1.9 · no · the live run's kept checks under target/cold-agent/record/ come only from cold-agent.sh's live run, which the chunk does not run
ci-integration#1.10 · no · the two committed Counter checks hold Counter ids, names, values, snapshot text, diffs and calls, none a lint answer, and no edited unit produces them, so stand_replay is unaffected
