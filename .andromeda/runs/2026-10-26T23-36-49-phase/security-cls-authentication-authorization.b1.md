# Classified — security · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · a credential-word search over 32 s03 files whose only hit, a commented-out `token` in tests/stylo_usage.rs, is a file the chunk does not reach; the chunk adds no credential code to find
authentication-authorization#1.2 · no · a login, auth or session search over 86 s04 files; the chunk adds no login or auth code to find
authentication-authorization#1.3 · no · a login, credential or permission search over 15 s05 files; the chunk adds no such code to find
authentication-authorization#1.4 · no · an auth, login, session or credential search over 17 s06 files; the chunk adds no such code to find
authentication-authorization#1.5 · no · an auth, session, login or permission search over 8 s07 files; the chunk adds no such code to find
authentication-authorization#1.6 · no · a credential-word search over 16 s08 files; the chunk adds no credential code to find
authentication-authorization#1.7 · no · a credential-word search over 32 s09 files; the chunk adds no credential code to find
authentication-authorization#1.8 · no · a credential-word search over 32 s10 files; the chunk adds no credential code to find
authentication-authorization#1.9 · no · a credential-word search over 21 s11 files; the chunk adds no credential code to find
authentication-authorization#1.10 · no · a credential-word search over 61 s12 files; the chunk adds no credential code to find
authentication-authorization#1.11 · no · a credential-word search over 12 s13 files; the chunk adds no credential code to find
authentication-authorization#2.1 · no · the cookie store sits in blitz-net's reqwest client, which the chunk does not reach; the Not-needed list names blitz-net's fetches and the browser's cookies
authentication-authorization#2.2 · no · the post-results workflow's token grants; the Not-needed list names the post-results workflow and the chunk changes no workflow
authentication-authorization#2.3 · no · the wpt.yml token grants; the Not-needed list names the WPT workflows and the chunk changes no workflow
authentication-authorization#2.4 · no · the publish job's repository guard and its environment; the Not-needed list names the upstream-only publish and its guards, and the chunk changes no workflow
authentication-authorization#2.5 · yes · ci.yml's read-only workflow token: the fast leg the chunk re-runs runs the CI scripts, and test_ci_workflows.py, which this entity names, asserts the grant (test_ci_workflows.py:201-205), so the chunk must keep ci.yml's permissions as they are
authentication-authorization#2.6 · no · the session socket's owner-only directory and file modes and its unchecked answering pid; the Not-needed list names the socket's modes and the chunk touches no host, client or wire code
authentication-authorization#2.7 · no · the stdio MCP server's no-handshake, no-token, no-peer-check model; the chunk changes no stdio, framing or listener path, and the Not-needed list names the MCP framing
