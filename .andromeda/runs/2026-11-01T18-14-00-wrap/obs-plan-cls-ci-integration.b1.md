# Classified — obs-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · wpt.yml is not in the reached files, and the session's ci.yml edits (the free-disk step left, the embolden leg) touch no WPT dispatch
ci-integration#1.2 · no · wpt.yml is not reached, and wptscores.json's computation and Pages publish are unchanged by the session's ci.yml and ci-leg.sh edits
ci-integration#1.3 · yes · stale · ci-leg.sh gained lines at 8-9 and 28-29, moving the :92-93 citation, the free-disk step left ci.yml, moving :414-423, and embolden, last in FAST_LEGS, adds its output to the fast leg's merged log
ci-integration#1.4 · yes · stale · ci-leg.sh gained lines at 8-9 and 28-29, moving the lcov writer citation at :42-49
ci-integration#1.5 · no · scripts/agent-run.sh is unedited, and the ci.yml change removes a free-disk step and adds no upload, so no CI leg uploads the agent-run state
ci-integration#1.6 · no · the new key_press test is a plain engine test, not one of the named session checks, escher-driver has no diff, and ci.yml adds no upload of target/tmp/
ci-integration#1.7 · no · the recording writers in escher-driver are unedited (no diff in that package), and the session adds no recorded check
ci-integration#1.8 · no · the cold-agent scripts are not among the changed files, and embolden and fast's cargo step are not the cold-agent shim tests, so CI still runs only those for the pipe
ci-integration#1.9 · no · the cold-agent recorder and escher-driver's recording code are unedited, and no committed check is added, so the two Counter checks stay the only committed ones
ci-integration#1.10 · no · the two committed Counter checks and record_checks.sh are not reached, and the session adds no committed check
