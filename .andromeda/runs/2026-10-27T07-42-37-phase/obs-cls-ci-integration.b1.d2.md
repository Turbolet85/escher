# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · wpt.yml's archive dispatch is upstream-only and repository-guarded off on the fork, and the take moves only the pinned WPT revision
ci-integration#1.2 · no · wptscores.json is the upstream-only wpt.yml job's output, and the WPT scores are Not needed (the take adds no exporter, counter or metric)
ci-integration#1.3 · no · a failed leg's merged-log upload is Not needed, the take adds no upload, and ci-leg.sh is only re-read
ci-integration#1.4 · no · the coverage report upload is Not needed, the take adds no upload, and the coverage leg's lines sit outside the four legs re-read in ci-leg.sh
ci-integration#1.5 · no · the agent-run harness log and its events are Not needed, and the chunk runs no agent-run pass
ci-integration#1.6 · no · the session directories under CI's local state are Not needed, and escher-driver and seven_guis' host are not edited
ci-integration#1.7 · no · a kept check's call content at rest, the recording directory included, is Not needed, and escher-driver and the host are not edited
ci-integration#1.8 · no · the cold-agent pipe's log, verdict and server-log counting are Not needed, and the chunk runs no cold-agent pass
ci-integration#1.9 · no · the cold-agent recording is Not needed, and the chunk runs no cold-agent pass
ci-integration#1.10 · yes · stand_replay and seven_guis' freshness check read these two committed Counter checks, which the fast leg and fork CI re-run on the synced engine, so a take that moves a Counter reading moves them
