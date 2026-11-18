# obs · standing conventions

- S1 · a change to the cold-agent pipe lands with its contract test, the verdict's fields and reasons closed words and counts · 6-log-coverage · log-coverage
- S2 · harness JSON lines are encoded by python's json module, and a log check's failure names a kind and a count · 3-observability-harness-contract
  decision/harness-json-via-python-json · Harness JSON lines encoded by python's json module — The agent-run and cold-agent harnesses write JSON lines encoded by python3's stdlib json module embedded in the script, never assembled in bash.
  decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote.
