# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the WPT archive and its repository guards live in wpt.yml, which the session does not reach
ci-integration#1.2 · no · wptscores.json is computed and published by wpt.yml, which the session does not reach
ci-integration#1.3 · no · ci-leg.sh's line count is unchanged and its line 39 (the a11y arm) is its only edit, so the log write at 88-89 holds for every leg; ci.yml is byte-identical, so the failure-only upload holds
ci-integration#1.4 · no · ci-leg.sh's line count is unchanged and its line 39 is its only edit, so the coverage arm at 40-47 holds; the coverage tally 187 · 1102 · 0 · 12 is unmoved and no source changed
ci-integration#1.5 · no · the agent-run harness was run unchanged and scripts/agent-run.sh is not reached, so its target/agent-run files and their cleanup hold
ci-integration#1.6 · no · none of the eight joining files is one of the named session checks, and the a11y leg uploads only target/ci-logs/, never target/tmp/
ci-integration#1.7 · no · none of the eight joining files is a recording check, and no CI leg uploads the recording directory
ci-integration#1.8 · no · the cold-agent pipe is not run or changed, and scripts/cold-agent.sh is not reached
ci-integration#1.9 · no · stand_replay and seven_guis' freshness check read the committed checks, neither is among the eight joining files, and no CI leg uploads them
