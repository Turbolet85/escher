# security · picked accounting · group 1 of 6

## Picked
threat-model-summary#1.1 · carried · its sub-document and never-styled-node rules live in `node/element.rs` and `document.rs`, both merge surfaces of the take (`53e9d81c6`, `0980cce5f`, `ebacd68bd`)
threat-model-summary#2.1 · carried · the script-execution boundary moves under the take: `runtime.rs` is a conflict surface split by `e6f4615ec`, and `cee6b8731` narrows the fetcher's `file:` rule
threat-model-summary#3.1 · not in scope · the driver session socket and its wire are untouched by the take (no escher-driver code or carrier moves); its standing checks are a re-check every change owes
threat-model-summary#4.1 · not in scope · the MCP stdio server and the cold-agent carriages are untouched, and the chunk runs no live agent session; a re-check every change owes
threat-model-summary#5.1 · not in scope · `docs/capabilities.md` is not reached and the MCP crossing it states does not move
authentication-authorization#2.4 · carried · `publish-browser.yml` is reached by `a7ea9631b` · `b5b163450`, and its repository guard on the publish job must hold after the take
authentication-authorization#2.5 · not in scope · `ci.yml` is not reached by the take; its read-only `permissions` and `test_ci_workflows.py`'s assertion are a re-check every change owes in the fast leg
