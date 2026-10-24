# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · the stand-repeat leg every sweep witness runs in (stand_*.rs selected by prefix, three rounds, no retry), which the sweep may widen; ci-leg.sh and test_ci_workflows.py are reached conflict surfaces
ci-integration#2.1 · yes · the stand-repeat job restores workspace-test with save-if false, which the flakiness budget binds ("the job restores the test cache without saving") for the sweep's stand checks
ci-integration#3.1 · no · the package-install bound lives in apt-install.sh and ci.yml's install steps, neither reached, and the task lists apt-install.sh and its bounds Not needed; its pins run in the ci-scripts leg whatever the sweep changes, a re-check every change owes
ci-integration#4.1 · yes · the runners resolve different engine features (tracing and writing-mode on only in the workspace build), so host_log and mcp_log count sink lines by target and must stay green under both builds
ci-integration#5.1 · no · a per-OS CI tally re-read after each run, not a rule the sweep's code or tests honour; its new checks move the counts, and that re-read is the re-check every change owes
ci-integration#6.1 · no · the dev-host cold and warm timings and counts are a measured reference the sweep neither reproduces nor changes; a re-check every change owes
ci-integration#7.1 · yes · the coverage leg's coverage_floor.py run over escher-driver 97, dioxus-native-dom 83 and blitz-test-harness 90, which must still pass at the chunk's end, with a blitz-dom floor possibly added; coverage_floor.py and test_coverage_floor.py are reached
