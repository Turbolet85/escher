# Classified — security · api-security.b1

## Answers
api-security#1.0 · yes · the one listener and the twelve MCP tools with their 65,536-byte request and 1,048,576-byte result bounds; the chunk edits `execute` and `lint` and the schema help that the MCP `help` tool answers
api-security#2.1 · no · outbound per-host fetch limit; the list fetches nothing
api-security#2.2 · no · CORS observed absent; the chunk adds no fetch or cross-origin path
api-security#2.3 · no · CSP observed absent; the overlay adds no document policy
api-security#2.4 · no · outbound response body read; the list fetches nothing
api-security#2.5 · no · request timeout observed absent; outbound only
api-security#2.6 · yes · acting driver commands settle on a load in flight within LOAD_WAIT, while `lint` and `screenshot` wait on none and move no time; the new option acts and the edited `lint` and `execute` must keep this
api-security#2.7 · no · outbound request headers; the list sends no request
api-security#2.8 · no · outbound HTTP status handling; the list sends no request
api-security#2.9 · no · outbound fetch cancellation; the list fetches nothing
api-security#2.10 · no · sub-resource GETs by stamped_request; the overlay paints the parsed document and fetches nothing
api-security#2.11 · no · JS-facing fetch; the list adds no script path or native API
api-security#2.12 · no · form submission in form.rs is unchanged and the scope names no submit behaviour; the choice only sets the select's value
api-security#2.13 · no · iframe navigation routing; the overlay navigates nothing
api-security#2.14 · no · dioxus scheme asset serving; the list requests no asset
api-security#2.15 · yes · the scope names Enter on links, which carries the webbrowser::open external hand-off, so the new key branches beside it must leave that path as built
api-security#2.16 · no · JS evaluation routed to NoOpDocument; the chunk evaluates nothing
api-security#2.17 · no · outbound GET in the examples; the scope edits no example and no examples' arguments
api-security#2.18 · no · outbound HTTP in apps through blitz-net and reqwest; the list adds none
api-security#2.19 · no · CI `gh api` calls and their token; no CI job or secret is touched
api-security#2.20 · yes · the local socket's request line, answer size and idle expiry bounds; the snapshot's option bounds, the lint findings and the new driver acts travel these replies
