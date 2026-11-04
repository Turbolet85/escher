@answer n=1 entity=api-security#1.0 v=no
why: the session adds one test target with no listener, port or socket, so the test-only and no-served-listener claims still hold
@answer n=2 entity=api-security#2.1 v=no
why: the session changes no blitz-net per-host limit or outbound request path
@answer n=3 entity=api-security#2.2 v=no
why: the session adds or removes no cors or same-origin check and touches no request path
@answer n=4 entity=api-security#2.3 v=no
why: the session adds no content-security policy and changes no request path
@answer n=5 entity=api-security#2.4 v=no
why: blitz-net's whole-body read is unchanged and the lock moves only pulldown-cmark, not reqwest
@answer n=6 entity=api-security#2.5 v=no
why: no request timeout is added to reqwest or any fetch, and the lock moves no reqwest version
@answer n=7 entity=api-security#2.6 v=no
why: the harness settle, LOAD_WAIT and the driver command schema and its waits are unchanged, and the new test reads no clock
@answer n=8 entity=api-security#2.7 v=no
why: blitz-net's forwarded headers and User-Agent are unchanged
@answer n=9 entity=api-security#2.8 v=no
why: the HTTP status mapping in blitz-net is unchanged
@answer n=10 entity=api-security#2.9 v=no
why: AbortSignal handling in blitz-traits and blitz-dom is unchanged
@answer n=11 entity=api-security#2.10 v=no
why: stamped_request in blitz-dom is unchanged
@answer n=12 entity=api-security#2.11 v=no
why: the JS fetch bootstrap and its GET and HEAD gate are unchanged
@answer n=13 entity=api-security#2.12 v=no
why: form submission code in blitz-dom is unchanged
@answer n=14 entity=api-security#2.13 v=no
why: iframe navigation routing in blitz-dom is unchanged
@answer n=15 entity=api-security#2.14 v=no
why: the dioxus scheme handler in dioxus-native is unchanged and no dioxus-native source moves
@answer n=16 entity=api-security#2.15 v=no
why: the link handler's webbrowser open call is unchanged
@answer n=17 entity=api-security#2.16 v=no
why: the NoOpDocument evaluation path in dioxus-native contexts is unchanged
@answer n=18 entity=api-security#2.17 v=no
why: no example's reqwest GET moves and the lock change touches no reqwest
@answer n=19 entity=api-security#2.18 v=no
why: the apps Cargo.toml lines and the blitz-net outbound path are unchanged, and the lock moves only pulldown-cmark
@answer n=20 entity=api-security#2.19 v=no
why: no workflow or CI script moves, the audit change being deny.toml and the lock only
@answer n=21 entity=api-security#2.20 v=no
why: escher-driver's host socket, its permissions and its missing handshake are unchanged, with no driver code moved
