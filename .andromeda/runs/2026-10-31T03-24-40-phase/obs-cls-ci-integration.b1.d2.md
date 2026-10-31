# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the WPT report's Pages archive and dispatch are the WPT job's publication, out of scope (Not needed: the scope adds no CI job)
ci-integration#1.2 · no · wptscores.json is the published WPT score file, which the scope reads and changes none of (Not needed)
ci-integration#1.3 · no · the leg log and its failure upload belong to ci-leg.sh and the failure artifacts are Not needed; the chunk's checks print nothing (S1) and its engine adds no log site (S5)
ci-integration#1.4 · no · the coverage leg's lcov artifact is Not needed (the scope adds no CI job) and keyboard_compat.rs is not reached
ci-integration#1.5 · no · the agent-run state area under target/agent-run/ belongs to scripts/agent-run.sh, which the scope does not touch
ci-integration#1.6 · no · the session directory under target/tmp/ belongs to the session library and the checks it names, and no check the scope adds, edits or re-reads is named as a session user
ci-integration#1.7 · no · the recording directory belongs to the recorder and the checks that record; recording.rs and session.rs are not reached and the scope adds no recorded check
ci-integration#1.8 · no · the cold-agent pipe's state area belongs to scripts/cold-agent.sh, which the scope does not touch, and the scope runs no cold agent
ci-integration#1.9 · no · target/cold-agent/record/ is the cold-agent pipe's recording (Not needed), and the scope runs no cold agent
ci-integration#1.10 · no · the two committed Counter checks are read by stand_replay and the seven_guis freshness check, neither reached, and the scope names no Counter surface and adds no committed check
