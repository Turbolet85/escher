# Classified — security · api-security.b1

## Answers
api-security#1.0 · yes · the README's driver paragraph names the MCP stdio child and the session socket as the only listeners, with no TCP or UDP port
api-security#2.1 · no · blitz-net's per-host outbound concurrency is not named by the chunk, and the Not-needed list leaves outbound request controls untouched
api-security#2.2 · no · the absent CORS check is an outbound request control the chunk neither names nor re-measures, and the Not-needed list leaves it untouched
api-security#2.3 · no · the absent CSP is an outbound request control the chunk neither names nor re-measures, and the Not-needed list leaves it untouched
api-security#2.4 · no · blitz-net reads a response body whole with no size cap, an outbound request control the Not-needed list leaves untouched
api-security#2.5 · no · the absent request timeout is blitz-net's outbound control, and the chunk names only LOAD_WAIT, which bounds the wait without this timeout
api-security#2.6 · yes · the README names an acting command's wait as at most LOAD_WAIT (5 s) and snapshot, screenshot and lint as waiting on no load
api-security#2.7 · no · blitz-net's request headers are an outbound path the chunk does not reach
api-security#2.8 · no · blitz-net's HTTP status mapping is an outbound path the chunk does not reach
api-security#2.9 · no · AbortSignal cancellation of document fetches is a net path the chunk does not reach
api-security#2.10 · no · blitz-dom's stamped sub-resource GETs are not named by the chunk's README or statement prose
api-security#2.11 · no · the Boa JS-facing fetch is untouched, as the Not-needed list leaves script execution and the default ScriptFetcher untouched
api-security#2.12 · no · form submission's GET and POST rules are in-document navigation the Not-needed list leaves untouched
api-security#2.13 · no · iframe navigation routing is in-document navigation the Not-needed list leaves untouched
api-security#2.14 · no · the dioxus scheme's local asset serving in dioxus-native is not named by the chunk
api-security#2.15 · no · external links handed to the system browser in dioxus-native are not named by the chunk
api-security#2.16 · no · JS evaluation routed to NoOpDocument in dioxus-native is not named by the chunk
api-security#2.17 · no · the examples' plain GET is outside the chunk's Try it, which shows escher-session, and the example argv handling is untouched
api-security#2.18 · no · the apps' outbound HTTP through blitz-net and rdme's reqwest is untouched, as the chunk changes no app code
api-security#2.19 · no · the CI gh api calls and their workflow token are untouched, as the chunk changes no workflow
api-security#2.20 · yes · the README's driver paragraph states the session socket owner-only by file mode (directory 0700, socket 0600), with no other authentication and no TCP or UDP port
