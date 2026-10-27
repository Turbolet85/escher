# Classified — security · api-security.b1

## Answers
api-security#1.0 · yes · the README driver paragraph's MCP stdio boundary: the escher-session mcp role's twelve tools over its own stdin and stdout, no TCP or UDP port, the session socket the one listener
api-security#2.1 · no · blitz-net's per-host concurrency limit is outbound request code the chunk does not touch, and the scope names no fetch behaviour
api-security#2.2 · no · CORS is observed absent in blitz-net; the scope names no cross-origin behaviour and the chunk touches no fetch code
api-security#2.3 · no · CSP is observed absent; the scope names no content-security behaviour and the chunk touches no fetch code
api-security#2.4 · no · blitz-net reads responses whole with no size cap; outbound request controls are untouched and the prose states no size limit
api-security#2.5 · no · no request timeout is observed in blitz-net; the wait the README states is bounded by LOAD_WAIT through the settle (#2.6), not by this control
api-security#2.6 · yes · the README's wait claim: an acting command waits at most LOAD_WAIT (5 s), snapshot, screenshot and lint wait on none, and the settle's outcome names a class only
api-security#2.7 · no · caller-header forwarding and the User-Agent in blitz-net; outbound request code the chunk does not touch
api-security#2.8 · no · HTTP status mapping in blitz-net; outbound request code the chunk does not touch
api-security#2.9 · no · AbortSignal cancellation of in-flight fetches; the chunk touches no fetch code and the scope names no cancellation
api-security#2.10 · no · sub-resource GETs built by stamped_request in blitz-dom; the chunk touches no fetch code
api-security#2.11 · no · Boa's GET and HEAD fetch over ScriptFetcher; script execution is untouched by the chunk
api-security#2.12 · no · form submission's GET and POST rules; in-document navigation is untouched and the prose names no form
api-security#2.13 · no · sub-document navigation routed to the parent; in-document navigation rules are untouched and the prose names none
api-security#2.14 · no · the dioxus scheme's local asset reads in dioxus-native; the chunk touches no asset path and the prose names none
api-security#2.15 · no · external targets handed to webbrowser::open; in-document navigation is untouched and the prose names no link handling
api-security#2.16 · no · JS evaluation sent to NoOpDocument in dioxus-native; script execution is untouched and the prose states no evaluation
api-security#2.17 · no · the examples' plain reqwest GET; example argv and fetching are untouched and the prose names no example fetch
api-security#2.18 · no · the apps' outbound HTTP through blitz-net and rdme's reqwest; no app code changes and the prose names no app fetch
api-security#2.19 · no · CI's gh api calls and their secret wiring in the workflow files; the chunk changes no workflow and its prose names none
api-security#2.20 · yes · the README driver paragraph's session socket: a local Unix-domain socket, owner-only (state directory 0700, socket 0600), with no other authentication
