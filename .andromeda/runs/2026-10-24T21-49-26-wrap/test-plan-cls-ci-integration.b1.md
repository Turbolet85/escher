# Classified — test-plan · ci-integration.b1

## Answers
ci-integration#1.1 · yes · add-to · F1 (the founder's option b, 14 files and 84 tests) and F2 (each file's witness) join the a11y leg, whose six files became fourteen and whose test_ci_workflows.py:361-365 citation moved to 364-368
ci-integration#2.1 · no · ci.yml is byte-identical and no workflow file changed, so the a11y job's workspace-test restore with save-if false stands
ci-integration#3.1 · no · no install step or apt script changed and no leg calls apt-install.sh (the session edited only the a11y line's test words), so the a11y job's libfontconfig1-dev install stands as read
ci-integration#4.1 · no · the test leg still reads cargo test --workspace --locked (line 39 kept that form), the -p blitz-tests runner is unchanged, and the joining files ran green under the workspace build
ci-integration#5.1 · yes · add-to · F3 (a CI log spells the ANSI escape as the literal ^[, which a tally read must strip too) joins the readings, and the a11y job's 14 · 84 · 0 · 0 on CI#40134371519, the other jobs unmoved, joins the dated ones
ci-integration#6.1 · yes · stale · the local ci-leg.sh a11y reading on line 64 reads six files and 43 tests, now fourteen files and 84 tests
ci-integration#7.1 · no · the session replaced only line 39 of ci-leg.sh, so the coverage leg at lines 40-47 and the floor script keep their text and line numbers
