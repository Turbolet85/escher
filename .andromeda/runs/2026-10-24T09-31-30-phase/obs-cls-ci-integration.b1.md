# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the task's Not-needed excludes WPT score publishing to Pages, which runs on upstream only and whose conformance numbers are out of scope
ci-integration#1.2 · no · the task's Not-needed excludes WPT score publishing, and wptscores.json reaches Pages on upstream only
ci-integration#1.3 · yes · the audit fix reads a failed leg's merged log, the only CI artifact it can produce, and Needs holds that log free of user data
ci-integration#1.4 · no · the task's Not-needed excludes the coverage report artifact, which the sync does not change
ci-integration#1.5 · no · the task's Not-needed excludes the agent-run harness log, and the chunk runs no agent-run pipeline
ci-integration#1.6 · no · the task's Not-needed excludes the local session state directories, which the chunk neither writes nor reads
ci-integration#1.7 · no · the task's Not-needed excludes the recording directory and its kept-check recordings, which the chunk neither writes nor reads
ci-integration#1.8 · no · the task's Not-needed excludes the cold-agent pipe state, and the chunk runs no cold-agent pipe
ci-integration#1.9 · no · the task's Not-needed excludes the two committed Counter checks, which the chunk neither writes nor reads
