# Classified — test-plan · mocking-stubbing-discipline.b2

## Answers
mocking-stubbing-discipline#3.1 · yes · stale · test_ci_workflows.py is reached and lies in the searched .github/scripts/*.py scope, so the no-mocking reading cannot be shown to hold on the changed file.
mocking-stubbing-discipline#3.2 · no · no reached file lies under apps/ (apps/browser is untouched), so the apps search is not reached.
mocking-stubbing-discipline#3.3 · yes · stale · the reached blitz-tests checks (select_popup with its hand-written stub, plus the three new checks) lie in the searched test scope, so the slice's file count and no-match reading can move.
mocking-stubbing-discipline#3.4 · yes · stale · the session moves fixture readers (`mod fixtures;` from 28 to 31 in checks), which the mock|fixture search over this slice reads.
mocking-stubbing-discipline#3.5 · yes · stale · reached seven_guis tests (cli_commands, host_log, mcp_log) and common/mod.rs, which gains option_needles (F12), lie in the mock|fixture search scope.
mocking-stubbing-discipline#3.6 · yes · stale · the reached test checks and the new hand-written CountingShell stub lie in the mock|fake|stub scope, so the (no match) reading cannot be shown to hold.
mocking-stubbing-discipline#3.7 · yes · add-to · F13: select_popup holds its own hand-written CountingShell, a ShellProvider stub counting request_redraw, which joins the mock or stub types list.
mocking-stubbing-discipline#3.8 · yes · stale · the reached test checks and the new hand-written CountingShell stub lie in the mock|stub|fake search scope, so the 21-file reading can move.
