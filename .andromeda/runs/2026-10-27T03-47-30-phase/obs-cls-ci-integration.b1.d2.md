# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the upstream-only WPT report is a CI telemetry artifact the task's Not needed list drops, and the chunk changes no CI file
ci-integration#1.2 · no · wptscores.json is an upstream-only CI telemetry artifact the Not needed list drops, and the chunk names no WPT score
ci-integration#1.3 · no · the per-leg CI log is a failed leg's merged build output, which the Not needed list drops and which is not escher telemetry
ci-integration#1.4 · no · the coverage report is a CI telemetry artifact the Not needed list drops, and the chunk measures no coverage
ci-integration#1.5 · no · the agent-run harness state is the harness log (events and run.log) the Not needed list drops, and the chunk runs and changes no harness
ci-integration#1.6 · no · the session state directory and its socket belong to the session library and its checks, which the chunk does not read, and the chunk's Needs name the tool count, the telemetry readings and the recorded check, not the directory
ci-integration#1.7 · yes · the README's 0.2.0 recorded-check line must agree with this rule that a kept check is a file at rest, owner-only and never logged, with no log or event line
ci-integration#1.8 · yes · the cited 2026-10-25 verdict counts come from verdict.json and the tool_call lines of server.log, and transcript.jsonl, check.json and client.log must stay unprinted, uncopied and uncommitted
ci-integration#1.9 · yes · the cited counts and booleans, recorded_sessions and recording_kept among them, come from the pipe's private state, whose live child's kept checks under target/cold-agent/record/ are never printed, copied or committed
ci-integration#1.10 · no · the chunk names neither committed Counter check and reads neither stand_replay nor the freshness check, and its never-logged reading is already held by #1.7
