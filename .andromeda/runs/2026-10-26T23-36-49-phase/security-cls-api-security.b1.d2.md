# Classified — security · api-security.b1

## Answers
api-security#1.0 · yes · the listing test and every helper the lint walk gains bind no listener; the MCP role is stdio-only and the session socket stays the one listener
api-security#2.1 · no · blitz-net's per-host outbound semaphore; the chunk makes no outbound request
api-security#2.2 · no · CORS and same-origin, observed absent on the fetch paths; the chunk reaches no fetch
api-security#2.3 · no · CSP, observed absent on the fetch paths; the chunk adds no page load
api-security#2.4 · no · blitz-net reads response bodies whole; the chunk reads no response
api-security#2.5 · no · reqwest request timeout, observed absent; the chunk makes no request and waits on no load
api-security#2.6 · yes · the lint verb runs no step, settles nothing and moves no time, which the refactored `findings` must keep (the scope makes lint read-only and leaves the held instance unchanged)
api-security#2.7 · no · blitz-net's forwarded request headers; the chunk makes no request
api-security#2.8 · no · blitz-net's HTTP status mapping; the chunk receives no response
api-security#2.9 · no · AbortSignal fetch cancellation in blitz-traits and blitz-dom; the chunk starts no fetch
api-security#2.10 · no · sub-resource GETs built in blitz-dom's net.rs; the chunk loads no sub-resource
api-security#2.11 · no · the JS-facing fetch over ScriptFetcher; script execution is outside the chunk's scope
api-security#2.12 · no · form submission in blitz-dom's form.rs; the chunk touches no form path
api-security#2.13 · no · iframe navigation routing in blitz-dom; the chunk touches no sub-document
api-security#2.14 · no · the `dioxus` scheme's local asset serving in dioxus-native; the chunk touches no asset path
api-security#2.15 · no · external navigation through webbrowser in dioxus-native; the chunk opens no link
api-security#2.16 · no · dioxus-native script evaluation routed to NoOpDocument; the chunk evaluates no script
api-security#2.17 · no · the root examples' plain reqwest GETs; the chunk touches no example
api-security#2.18 · no · the browser and rdme apps' outbound HTTP; the chunk touches no app
api-security#2.19 · no · gh api calls with the workflow token in the WPT scripts; the chunk makes no GitHub call and the WPT and post-results workflows are out of scope
api-security#2.20 · no · the session socket's directory, wire, answer bounds and idle expiry; no host, client or wire code is touched and the lint's written form stays the same
