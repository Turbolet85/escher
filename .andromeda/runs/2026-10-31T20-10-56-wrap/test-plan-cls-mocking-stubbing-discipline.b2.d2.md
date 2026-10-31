# Classified — test-plan · mocking-stubbing-discipline.b2

## Answers
mocking-stubbing-discipline#3.1 · yes · stale · reached `.github/scripts/test_ci_workflows.py` sits in the searched `.github/scripts/*.py` set (its assertion moves at :367), and the report does not show the mock search holds on it
mocking-stubbing-discipline#3.2 · no · the apps slice holds no reached file (apps/browser untouched), and no mocking library, dependency or manifest moved
mocking-stubbing-discipline#3.3 · yes · stale · the reached blitz-tests checks now hold a hand-written test double (F13) that can fall in this stub search's scope, and the report does not show the search holds on them
mocking-stubbing-discipline#3.4 · yes · stale · the reached checks now declare `mod fixtures;` (28 to 31), which matches this fixture term in a slice whose membership the report does not show is clear
mocking-stubbing-discipline#3.5 · yes · stale · the reached checks now declare `mod fixtures;` (28 to 31), which can fall in this fixture term's scope, and the report does not show the search holds
mocking-stubbing-discipline#3.6 · yes · stale · the reached test double and the new `mod fixtures;` readers can fall in this no-match search's scope, and the report does not show it still reads no match
mocking-stubbing-discipline#3.7 · yes · add-to · F13: `select_popup`'s hand-written `CountingShell`, a `ShellProvider` counting `request_redraw`, joins this record of mock or stub types
mocking-stubbing-discipline#3.8 · yes · stale · the reached test double can fall in this mock, stub or fake search's scope, and the report does not show it still reads no match
