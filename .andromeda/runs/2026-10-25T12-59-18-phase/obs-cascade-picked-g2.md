# obs · picked accounting · group 2 of 2

## Picked
ci-integration#1.3 · carried · the re-pinned stand checks run in the `test` and `stand-repeat` legs, whose merged log is uploaded on failure and must hold no user data
ci-integration#1.7 · carried · the cold agent's recording-and-replay goal and a possible `--record` kept check as the acceptance artifact write a recording directory that stays local, owner-only and unlogged
ci-integration#1.8 · carried · the chunk edits `scripts/cold-agent.sh` and makes one live run whose state must stay local under `target/cold-agent/`, uploaded by no CI leg
ci-integration#1.9 · carried · the stand's `boot` / `options` edits must keep the two committed Counter checks replaying, and no check of the live run joins the committed set
