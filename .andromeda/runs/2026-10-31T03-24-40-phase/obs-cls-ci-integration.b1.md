# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the WPT report's publication is in the task's not-needed list; the scope adds no CI job and reads no WPT score
ci-integration#1.2 · no · the wptscores.json publication is in the task's not-needed list; the chunk reads and changes no WPT score
ci-integration#1.3 · no · standing S7 · a leg's merged output holds no user data (§8), a convention the task lists
ci-integration#1.4 · no · standing S7 · the coverage artifact holds line counts and no user data (§8), a convention the task lists
ci-integration#1.5 · no · standing S7 · the agent-run raw output is unscrubbed and holds no user data (§8), a convention the task lists
ci-integration#1.6 · no · the session host keeps its own dir rules (0700, socket alone, cleared on stop or idle expiry); the scope names no session dir, socket or stop
ci-integration#1.7 · no · the scope names no --record launcher or recorded session; the recording rules are the launcher's
ci-integration#1.8 · no · standing S7 · the agent's check is raw by design and never printed (§8), a convention the task lists
ci-integration#1.9 · no · the scope runs no cold agent and touches no cold-agent.sh, so no kept check of a live run is reached
ci-integration#1.10 · no · the committed Counter checks are read by stand_replay and the freshness check, neither named by the scope
