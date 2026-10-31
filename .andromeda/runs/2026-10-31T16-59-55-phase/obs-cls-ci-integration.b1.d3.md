# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the WPT report and its Pages and dispatch publication are upstream-only in wpt.yml, which the chunk does not touch
ci-integration#1.2 · no · wptscores.json is computed and published upstream only, and the WPT scores' publication is untouched
ci-integration#1.3 · yes · the per-leg log is the job's log that keeps the free-disk step's output the `Stand checks repeated` watch requires, and its ci-log artifact on failure
ci-integration#1.4 · no · the coverage lcov artifact is not the free-disk step's output and no scope line names coverage
ci-integration#1.5 · no · agent-run.sh's state area is unchanged and a new test appears through it unchanged
ci-integration#1.6 · no · the session directory holds the socket file alone and the chunk adds no check that names a session directory under target/tmp/
ci-integration#1.7 · yes · the recording directory holds each recorded answer at rest, and the moved snapshot changes recorded answers
ci-integration#1.8 · yes · transcript.jsonl's carriage of the flight booker's select options is a PII row the open list may move, and server.log is counted by span name
ci-integration#1.9 · no · the live-run record directory is removed by cold-agent cleanup and the chunk commits no live-run check
ci-integration#1.10 · yes · the two committed Counter checks may be re-recorded when the moved snapshot changes an answer and must still hold no typed text, host path or secret
