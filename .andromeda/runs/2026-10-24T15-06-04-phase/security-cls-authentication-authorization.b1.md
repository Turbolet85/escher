# Classified — security · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · dropped as a re-check every change owes, since its only hits are a commented-out token in tests/stylo_usage.rs, a file the chunk does not reach, and the chunk adds no credential path
authentication-authorization#1.2 · no · dropped as a re-check every change owes, since the chunk adds no login, authentication or auth-session path (the driver session is not one) and the task names authentication as none existing and none added
authentication-authorization#1.3 · no · dropped as a re-check every change owes, since the chunk adds no authorization or credential layer and its 0700 and 0600 file-mode checks are the filesystem access that #2.6 governs
authentication-authorization#1.4 · no · dropped as a re-check every change owes, since the chunk adds no authentication, login, auth-session or credential path, which the task names as none existing and none added
authentication-authorization#1.5 · no · dropped as a re-check every change owes, since the chunk adds no auth, session-auth, login or permission layer and its file-mode checks are the filesystem access that #2.6 governs
authentication-authorization#1.6 · no · dropped as a re-check every change owes, since the password_caret_click witness the chunk brings into the sweep is a masked form control rather than a credential path, and no real credential may enter a fixture
authentication-authorization#1.7 · no · dropped as a re-check every change owes, since the chunk adds no credential, token, cookie or API-key path and blitz-net's cookie store lies outside it (see #2.1)
authentication-authorization#1.8 · no · dropped as a re-check every change owes, since its hits are a URL component and a comment, and the chunk adds no credential or authorization path
authentication-authorization#1.9 · no · dropped as a re-check every change owes, since its only hit is the word authors and the chunk adds no credential or authorization path
authentication-authorization#1.10 · no · dropped as a re-check every change owes, since the chunk adds no token, password, secret, API-key, authorization or cookie path
authentication-authorization#1.11 · no · dropped as a re-check every change owes, since the chunk adds no authorization, cookie, API-key, password, secret or token path
authentication-authorization#2.1 · no · the cookie store belongs to blitz-net's reqwest client, which the chunk never reaches (its fixtures run offline on the held network stand-in), and the task's Not-needed list names cookies
authentication-authorization#2.2 · no · the wpt-post-results grants sit in a workflow the task's Not-needed list excludes, and the chunk touches no secret-bearing job
authentication-authorization#2.3 · no · the wpt.yml grants sit in a workflow the task's Not-needed list excludes, and the chunk touches no secret-bearing job
authentication-authorization#2.4 · no · the publish job's repository guard and environment live in publish-browser.yml, which the chunk does not reach and the Not-needed list excludes
authentication-authorization#2.5 · yes · test_ci_workflows.py, which the chunk edits, asserts ci.yml's workflow-level read-only token (lines 201-205), so any ci.yml job the leg or the coverage pin changes must keep its token read-only
authentication-authorization#2.6 · yes · the session checks and the host-log measurement drive a hosted session over the socket, so the 0700 state directory, the 0600 socket, the state-dir-too-long bound and the idle expiry bind, with access left to the filesystem
authentication-authorization#2.7 · yes · the mcp_log check and the mcp-role stderr measurement drive the server over its own stdio, so the trust model (only the spawning client, no listener) binds how the checks spawn it and no listener may be added
