# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · WPT report archive is upstream-only and out of scope (Not needed: WPT report and wptscores.json)
ci-integration#1.2 · no · wptscores.json is computed and published upstream only, WPT out of scope (Not needed)
ci-integration#1.3 · yes · the scope names the handling of raw logs in target/ci-logs/ (unscrubbed, uploaded only as a leg's merged log on failure) that the per-OS tally read draws on, and ci-leg.sh's log write for the re-read legs
ci-integration#1.4 · no · standing S4 · no artifact beyond the fork's two (§9 CI Integration); the coverage-report artifact is not reached and the chunk adds no artifact
ci-integration#1.5 · no · agent-run harness state (target/agent-run/) is Not needed; the chunk does not change scripts/agent-run.sh
ci-integration#1.6 · no · session state directory is Not needed (recording and session directories); the re-read checks' trace reads do not rule over the directory
ci-integration#1.7 · no · recording directory is Not needed (the recording module's kept checks, recording and session directories)
ci-integration#1.8 · no · cold-agent pipe state is Not needed; the chunk runs no cold-agent bar and changes nothing in that pipe
ci-integration#1.9 · no · cold-agent recording is Not needed; the pipe is unchanged and no kept check is committed
ci-integration#1.10 · no · committed Counter checks are read by stand_replay and seven_guis' freshness check, neither reached nor re-read; the recording module's kept checks are Not needed
