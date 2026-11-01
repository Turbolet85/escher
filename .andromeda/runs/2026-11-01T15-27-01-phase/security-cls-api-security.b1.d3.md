# Classified — security · api-security.b1

## Answers
api-security#1.0 · yes · the take must open no listener or port, so the searched-absent TcpListener/TcpStream/bind/listen check over the s13 files must still hold, with the session socket the one listener
api-security#2.1 · no · blitz-net's per-host semaphore is not reached and no scope item names outbound concurrency
api-security#2.2 · no · CORS/same-origin absence is observed over unreached slices and the scope names no CORS rule
api-security#2.3 · no · CSP absence is observed over an unreached slice and the scope names no CSP rule
api-security#2.4 · no · the whole-body read and absent size cap are blitz-net's own code, unreached, and no scope item names a body limit
api-security#2.5 · no · the absent request timeout is blitz-net's unchanged fact; the settle not hanging on a load is carried by api-security#2.6
api-security#2.6 · yes · the scope names the settle's not waiting on a load in flight on the f6edb9701 path, the acting step's LOAD_WAIT, and the stand_load_page witness
api-security#2.7 · no · blitz-net's User-Agent and Content-Type headers are unreached and no scope item names headers
api-security#2.8 · yes · the failed-load path f6edb9701 moves is "failed loads logged and dropped", and a non-success status is the failure it receives as ProviderError::HttpStatus
api-security#2.9 · no · AbortSignal cancellation is blitz-traits and blitz-dom config behaviour, unreached and not named
api-security#2.10 · yes · the new stylesheet fetch path must build its GETs through stamped_request, keeping the scope's GET/HEAD-only rule for new fetch paths
api-security#2.11 · yes · the scope's rule that a new script or fetch path adds no method past GET and HEAD is this JS-facing fetch guard
api-security#2.12 · no · form submission methods are in Not needed, as no commit the scope names moves them
api-security#2.13 · no · iframe navigation routing is blitz-dom iframe.rs, unreached and not named
api-security#2.14 · no · dioxus-scheme asset serving is in dioxus-native assets.rs, unreached and not named
api-security#2.15 · no · external navigation through webbrowser is in link_handler.rs, unreached and not named
api-security#2.16 · no · NoOpDocument JS evaluation is in contexts.rs, unreached and not named
api-security#2.17 · no · the example HTTP GETs are in examples the chunk does not reach or edit
api-security#2.18 · no · apps' http2 and reqwest features sit outside the reached root manifest and the scope names no app
api-security#2.19 · no · the gh api token use is in wpt scripts and wpt-post-results.yml, unreached; the ci.yml edit is the free-disk step only
api-security#2.20 · no · the session socket's modes, framing and idle expiry are Not needed, as the chunk edits no escher-driver code
