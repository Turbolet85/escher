# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the WPT report archive and its dispatch sit in wpt.yml, which the take does not reach; WPT_COMMIT moves only the pinned revision, and the WPT run is repository-guarded off the fork
ci-integration#1.2 · no · wptscores.json is computed and published by the upstream-only WPT run in wpt.yml, which the take does not reach
ci-integration#1.3 · no · the log write and its failure-only upload are not edited by the take, and the task's Not needed names a failed leg's merged log
ci-integration#1.4 · no · the coverage report is the coverage leg's upload, outside the four legs the task re-reads, and the task's Not needed names the coverage report
ci-integration#1.5 · no · the agent-run harness state is not reached by the take, and the task's Not needed names the agent-run harness log and its events
ci-integration#1.6 · no · escher-driver's host creates the session state directory and the take does not edit it, and the capture checks re-measure stderr needles, not the directory
ci-integration#1.7 · yes · its no-log-or-event-line clause, which host_log and mcp_log re-measure at trace under both builds as 0 recording needles and unchanged tool_call and command span counts; its at-rest content and directory modes are Not needed
ci-integration#1.8 · no · the cold-agent pipe's state is not reached, and the task's Not needed names the cold-agent pipe's log, the chunk running no cold-agent pass
ci-integration#1.9 · no · the cold-agent recording directory is written only by the live cold-agent pass, which the chunk does not run
ci-integration#1.10 · yes · the take's reached engine files (dioxus_document.rs, node.rs, scrolling.rs, document.rs) can move the Counter ids, names, values, snapshot text and diffs these checks hold, which stand_replay and seven_guis' freshness check read in the fast leg the chunk re-runs
