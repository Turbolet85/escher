# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the WPT archive and its upstream-only dispatch sit in wpt.yml, which is not a reached file, and the session moved no CI step, leg, artifact or retention
ci-integration#1.2 · no · wptscores.json is computed and published in wpt.yml, which is not a reached file, and the session moved no CI step, leg, artifact or retention
ci-integration#1.3 · no · the per-leg log is written by ci-leg.sh and uploaded from ci.yml, neither a reached file, and the green CI run leaves the failure-only upload rule as it was
ci-integration#1.4 · no · the coverage lcov and its success-only upload sit in ci-leg.sh and ci.yml, neither a reached file, and no CI step, leg, artifact or retention moved
ci-integration#1.5 · no · scripts/agent-run.sh is not a reached file and no harness script or verdict shape moved, the retired incubator exclude being no log, event or telemetry state
ci-integration#1.6 · no · host.rs and the session checks are not reached, no session source or check changed, and the directory's socket-only state holds
ci-integration#1.7 · no · recording.rs and session.rs are not reached, the session changed no recorder, and the call content kept at rest stays as it was
ci-integration#1.8 · no · scripts/cold-agent.sh is unchanged and the session only quotes the 2026-10-25 run's figures, adding, renaming and retiring no verdict field
ci-integration#1.9 · no · scripts/cold-agent.sh sets the live run's record directory and is unchanged, and the session quotes that run's counts without moving the directory or its replay children
ci-integration#1.10 · no · the two committed Counter checks and record_checks.sh are not reached, the session changed no test or check, and the local state rule is unmoved
