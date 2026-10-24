# security · picked accounting · group 1 of 4

## Picked
threat-model-summary#3.1 · carried · host_log drives a hosted session over the owner-only socket, whose answers carry ids and names out while a `start`-spawned host's stderr is closed
threat-model-summary#4.1 · carried · mcp_log is the spawning client of the `mcp` role, whose stdout results carry ids and names while its stderr is the measured sink
authentication-authorization#2.5 · carried · test_ci_workflows.py is reached and its read-only `GITHUB_TOKEN` assertion must still hold when the leg's invariants change
authentication-authorization#2.6 · carried · the session checks' state directories keep the filesystem as the socket's only access control, no auth added
authentication-authorization#2.7 · carried · mcp_log reaches the server only as the process holding its stdio, binding nothing
