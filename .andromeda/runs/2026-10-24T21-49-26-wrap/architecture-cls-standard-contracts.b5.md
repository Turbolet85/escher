# Classified — architecture · standard-contracts.b5

## Answers
standard-contracts#23.1 · no · no CLI source or binary changed, and its argv, verb and exit-status claims live in files outside the reached set
standard-contracts#23.2 · no · the multi_doc example and its MULTI_DOC_* reads live in examples/, outside the reached set, and the session changed none of them
standard-contracts#24.1 · no · the browser app's history, navigation and about-page code lives in apps/, outside the reached set, and the session changed none of it
standard-contracts#25.1 · no · the WPT runner's outputs, reports and page messages live in wpt/runner, outside the reached set, and the session changed none of it
standard-contracts#26.1 · yes · add-to · F1 and F2 join the a11y leg (fourteen targets, not six: ci-leg.sh:39) and F3 joins its pin (one assertIn per name in no order, test_ci_workflows.py:298-302); the stale count, qualifier and list change with them, and the citations past line 299 (:330, :381-495, the end of :103-499) move +3
