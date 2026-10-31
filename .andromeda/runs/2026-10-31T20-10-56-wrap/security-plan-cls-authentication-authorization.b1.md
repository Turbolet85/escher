# Classified — security-plan · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · the report's Untouched Authentication clause holds: no handshake, token or peer check moved and no new session error; no secret or env var was read or added
authentication-authorization#1.2 · no · the report's Untouched Authentication clause holds: no handshake, token or peer check moved and no new session error
authentication-authorization#1.3 · no · the report's Untouched Authentication clause holds: no handshake, token or peer check moved and no new session error
authentication-authorization#1.4 · no · the report's Untouched Authentication clause holds: no handshake, token or peer check moved and no new session error
authentication-authorization#1.5 · no · the report's Untouched Authentication clause holds: no handshake, token or peer check moved and no new session error
authentication-authorization#1.6 · no · the report's Untouched Authentication clause holds: no handshake, token or peer check moved and no new session error; no secret or env var was added
authentication-authorization#1.7 · no · the report's Untouched Authentication clause holds: no handshake, token or peer check moved and no new session error; no secret or env var was added
authentication-authorization#1.8 · no · the report's Untouched Authentication clause holds: no handshake, token or peer check moved and no new session error
authentication-authorization#1.9 · no · the report's Untouched Authentication clause holds: no handshake, token or peer check moved and no new session error
authentication-authorization#1.10 · no · the report's Untouched Authentication clause holds: no handshake, token or peer check moved and no new session error
authentication-authorization#1.11 · no · the report's Untouched Authentication clause holds: no handshake, token or peer check moved and no new session error
authentication-authorization#2.1 · no · the cookie store sits in packages/blitz-net, which the session does not reach
authentication-authorization#2.2 · no · wpt-post-results.yml is an unreached workflow file, and the report says no `.github/workflows` file changed
authentication-authorization#2.3 · no · wpt.yml is an unreached workflow file, and the report says no `.github/workflows` file changed
authentication-authorization#2.4 · no · publish-browser.yml is an unreached workflow file, and the report says no `.github/workflows` file changed
authentication-authorization#2.5 · yes · stale · test_ci_workflows.py is a reached file that gains the select_popup assertion at line 367, and the report re-reads only 186-192 on it, so the cited 201-205 may have moved; the ci.yml permissions themselves are unmoved
authentication-authorization#2.6 · no · packages/escher-driver/src/host.rs is not reached, and the report names no new local IPC change or session error
authentication-authorization#2.7 · no · packages/escher-driver/src/mcp.rs is not reached, and the new mcp_log test adds no change to the stdio server
