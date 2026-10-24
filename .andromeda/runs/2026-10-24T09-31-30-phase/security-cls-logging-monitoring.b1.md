# Classified — security · logging-monitoring.b1

## Answers
logging-monitoring#1.1 · yes · the sink's target allowlist and span rule, which the sync re-checks against every new tracing or log call site in its delta
logging-monitoring#1.2 · yes · the stdout rule, since the delta adds one println! in examples/multi_doc.rs, which must be read as an example's output and not become an escher stdout carrier
logging-monitoring#2.1 · no · the cold-agent pipe's run files, transcript and committed-copy masking, which no sync step changes or runs (its scripts are not touched and no live run is made)
