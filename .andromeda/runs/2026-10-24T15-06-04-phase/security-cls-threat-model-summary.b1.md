# Classified — security · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · no · remote web content is on the task's Not-needed list and the fixtures run offline; its iframe and @import caps and stale-response discard sit in no reached file
threat-model-summary#2.1 · no · script execution is on the task's Not-needed list; the chunk adds no JavaScript and no reached file carries the script fetcher, runtime or isTrusted
threat-model-summary#3.1 · yes · host_log drives a hosted session over the socket, so the 0700 state directory, 0600 socket, socket-path length bound, idle expiry and one-connection rule bind; the wire grammar stays excluded
threat-model-summary#4.1 · yes · mcp_log drives the mcp role, so its stdio-only boundary (no listener), request and result bounds, fixed protocol errors with no data and the two admitted id forms bind; the cold-agent carriage stays excluded
