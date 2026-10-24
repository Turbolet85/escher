# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the WPT report archive and dispatch are upstream-only and the chunk touches no WPT workflow (task Not needed)
ci-integration#1.2 · no · wptscores.json is WPT score publishing, upstream only, and no WPT target is a 0.2.0 witness (task Not needed)
ci-integration#1.3 · yes · a failing leg's merged log upload, kept 7 days with no user data, and the stand-repeat leg's selection run through ci-leg.sh (task Needs)
ci-integration#1.4 · yes · the coverage leg's floors, possibly raised or extended to blitz-dom, and its lcov report upload on success, kept 7 days with no user data (task Needs)
ci-integration#1.5 · yes · the sweep's checks run through the agent-run harness, so their test events name file and test only and their failure text in run.log holds no id, name or value (task Needs)
ci-integration#1.6 · yes · a spawned host's session state directory holds only session.sock, with no log, event or screen content, as host_log and the session checks need (task Needs)
ci-integration#1.7 · yes · the recording directory and its kept check hold call content at rest, so the --record leg of the stderr measurement reads 0 recording needles (task Needs)
ci-integration#1.8 · no · the cold-agent pipe's state, its log, verdict and server.log span counting belong to v020-11, a later entry out of this chunk (task Not needed)
ci-integration#1.9 · yes · the committed Counter checks that stand_replay replays hold no typed text, host path or secret, binding any sweep witness that names them (task Needs)
