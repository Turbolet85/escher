# Classified — test-plan · ci-integration.b1

## Answers
ci-integration#1.1 · yes · stale · the a11y leg's 84 tests move to 85 with stand_keyboard's 14 → 15, and the stand-repeat glob's 48 stand_*.rs files become 50 with stand_cells_served and stand_load_page
ci-integration#2.1 · no · the cache keys and save-if settings live in .github/workflows/ci.yml, which the session does not reach, and it moves no Cargo.lock or job config
ci-integration#3.1 · no · the package-install steps live in ci.yml and apt-install.sh, neither reached, and the session adds no package install
ci-integration#4.1 · no · its feature claims sit in untouched Cargo manifests and packages/ (diff 0), no log line moved, and host_log's third test holds at host_log.rs:42-132 with the session's fourth test at 134-278 after it
ci-integration#5.1 · yes · stale · the a11y job's 84 tests (the reading at line 42) move to 85 with stand_keyboard's 14 → 15
ci-integration#6.1 · yes · stale · the a11y leg's 84 (line 65) and the workspace's 206 lines · 1105 passed (line 63) move to 85 and 208 · 1117 with the session's gate runs
ci-integration#7.1 · no · the coverage leg's command, its floors (97 · 83 · 90) and coverage_floor.py are unchanged, and the session moves only §10 readings this entity does not state
