# Classified — security · api-security.b1

## Answers
api-security#1.0 · yes · the session socket is the one listener, which the host-log and MCP checks drive, so no TCP port or new listener may land unregistered
api-security#2.1 · no · blitz-net's per-host semaphore is outbound request limiting (Not needed), and the fixtures run offline
api-security#2.2 · no · CORS and same-origin are observed absent and belong to script fetching (Not needed); the chunk adds no fetch
api-security#2.3 · no · CSP is observed absent and is outbound networking (Not needed); the chunk adds no served page
api-security#2.4 · no · the whole-body response read is blitz-net's outbound code (Not needed: request limits), and the fixtures run offline
api-security#2.5 · no · the absent reqwest timeout is blitz-net's outbound limit (Not needed); the held loads it could hang are bounded by LOAD_WAIT, which #2.6 carries
api-security#2.6 · yes · settle and LOAD_WAIT never wait on a load in flight, which stand_load_wait and the held network stand-in test under stand-repeat
api-security#2.7 · no · forwarded request headers and User-Agent are blitz-net's outbound request building (Not needed)
api-security#2.8 · no · HTTP status mapping is blitz-net's outbound response handling (Not needed), and the fixtures run offline
api-security#2.9 · no · AbortSignal cancellation is blitz-traits and blitz-dom's fetch cancel, not reached; the sweep's held load is bounded by LOAD_WAIT, not cancelled
api-security#2.10 · no · stamped sub-resource GETs are blitz-dom's request builder, not reached, and no sweep need names a sub-resource request
api-security#2.11 · no · the JS-facing fetch is script execution through ScriptFetcher (Not needed)
api-security#2.12 · no · form submission is blitz-dom's form.rs, not reached, and no sweep need names a form submit
api-security#2.13 · no · sub-document navigation is blitz-dom's iframe.rs, not reached, and no sweep need names an iframe
api-security#2.14 · no · serve_asset is the windowed dioxus-native asset path, not reached; the headless stand boots offline with its bundled font
api-security#2.15 · no · webbrowser::open is the windowed dioxus-native link handler, not reached, and external-browser navigation is Not needed
api-security#2.16 · no · evaluation through NoOpDocument is dioxus-native, not reached, and script execution is Not needed
api-security#2.17 · no · the example CLIs' plain reqwest GET is Not needed, and no sweep need reaches an example's HTTP
api-security#2.18 · no · the browser and rdme apps' outbound HTTP is Not needed
api-security#2.19 · no · its gh api calls sit in wpt_diff_to_pr.py and wpt-post-results.yml, neither reached, and no secret-bearing job is touched (Not needed)
api-security#2.20 · yes · the session socket's 0700 state directory and 0600 socket, its path-length bound, one connection at a time and the idle expiry, which the host-log measurement drives over it
