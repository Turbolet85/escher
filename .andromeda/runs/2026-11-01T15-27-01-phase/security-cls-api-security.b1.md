# Classified — security · api-security.b1

## Answers
api-security#1.0 · yes · the chunk opens no listener or port of ours, so the local session socket stays the only listener and no served network API is added
api-security#2.1 · no · blitz-net's per-host concurrency limit is not in the reached files or the scope, and the chunk changes no outbound request path
api-security#2.2 · no · the CORS and same-origin absence is not named by the scope, which adds no cross-origin path
api-security#2.3 · no · the CSP absence is not named by the scope
api-security#2.4 · no · the response body size cap is not named by the scope, and the stylesheet UTF-8 rule does not reach body size
api-security#2.5 · no · the absent request timeout is not named by the scope, and the settle rule over the new path is carried by 2.6
api-security#2.6 · yes · the settle never waits on a load in flight, which f6edb9701's new stylesheet path must honour, and the LOAD_WAIT bound the stand load page re-checks
api-security#2.7 · no · caller-forwarded headers and User-Agent are not named by the scope
api-security#2.8 · yes · a non-success status on the new stylesheet path arrives as HttpStatus, the failed load the scope says is logged and dropped
api-security#2.9 · no · AbortSignal cancellation of in-flight fetches is not named by the scope
api-security#2.10 · yes · a new sub-resource fetch path built by stamped_request must stay GET-only, the verb the scope allows for any new fetch path
api-security#2.11 · yes · the JS-facing fetch stays GET and HEAD only under the ScriptFetcher, with the default fetcher's file: and data: narrowing the scope names for bootstrap.rs and events.rs
api-security#2.12 · no · form submission is excluded by the task's Not needed list
api-security#2.13 · no · iframe navigation routing is not named by the scope
api-security#2.14 · no · the dioxus scheme's serve_asset in dioxus-native's assets.rs is not in the reached files or named by the scope
api-security#2.15 · no · external navigation through webbrowser is not named by the scope
api-security#2.16 · no · NoOpDocument JS evaluation in dioxus-native is not named by the scope
api-security#2.17 · no · the example HTTP calls are not in the reached files
api-security#2.18 · no · the blitz-net and reqwest features sit in the apps/browser and apps/readme manifests, which are not reached, and rdme's markdown path is excluded
api-security#2.19 · no · the gh api calls sit in wpt-post-results.yml and wpt_diff_to_pr.py, which are not reached
api-security#2.20 · no · the chunk edits no escher-driver code and adds no listener, so the socket's modes, bounds and expiry are Not needed
