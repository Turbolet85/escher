# Classified — security · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · its only hit is a commented sketch in tests/stylo_usage.rs, a file the chunk does not reach
authentication-authorization#1.2 · no · the chunk adds no auth or login handling, its driver session being the local socket of #2.6
authentication-authorization#1.3 · no · the chunk adds no login, credential, permission or authentication handling, the operator's login staying outside the repo
authentication-authorization#1.4 · no · the chunk adds no authentication, authorization, login or credential handling, its driver session being the local socket of #2.6
authentication-authorization#1.5 · no · the chunk adds no auth, login or permission handling, its driver session being the local socket of #2.6
authentication-authorization#1.6 · no · its hits are html5ever's TokenizerOpts and a commented DOMTokenList, not auth code the chunk adds to
authentication-authorization#1.7 · no · the sweep reads no such term, and the chunk adds no token, password, secret, API key, authorization or cookie handling
authentication-authorization#1.8 · no · its hits are a hyperlink URL component and an author-stylesheets comment, not auth code the chunk adds to
authentication-authorization#1.9 · no · its only match is the word authors, not auth code the chunk adds to
authentication-authorization#1.10 · no · the sweep reads no such term, and the chunk adds no token, password, secret, API key, authorization or cookie handling
authentication-authorization#1.11 · no · the sweep reads no such term, and the chunk adds no authorization, cookie, x-api-key, password, secret or token handling
authentication-authorization#2.1 · no · the reqwest cookie store is blitz-net's HTTP path, which the offline stand does not reach, and cookies are untouched
authentication-authorization#2.2 · no · the chunk changes no workflow, and the WPT post-results workflow's grants are outside its reach
authentication-authorization#2.3 · no · the chunk changes no workflow, and wpt.yml's grants are outside its reach
authentication-authorization#2.4 · no · the publish job's environment and secrets are release surface the chunk does not reach, and it adds no CI secret
authentication-authorization#2.5 · no · the chunk changes no CI workflow or test_ci_workflows.py, so the read-only workflow token stays as asserted
authentication-authorization#2.6 · yes · the chunk's new start task and flight.sh drive this socket, so its owner-only 0700 and 0600 modes and the missing handshake bind session_host.rs and flight.sh
authentication-authorization#2.7 · yes · the cold-agent pipe spawns fresh escher-session mcp children and its live run's tools ride this stdio boundary, so scripts/cold-agent.sh and test_cold_agent.py honour it
