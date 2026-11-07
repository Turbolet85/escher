@answer n=1 entity=api-security#1.0 v=yes
why: the chunk adds no listener, port or verb (task item 6), so its new code must keep the served-API inventory this entity records: one session socket and no TCP or UDP server.
@answer n=2 entity=api-security#2.1 v=no
why: the outbound per-host semaphore is blitz-net's; the chunk fetches nothing (task not_needed 5).
@answer n=3 entity=api-security#2.2 v=no
why: CORS and same-origin are observed absent and no cross-origin fetch path is in the chunk's scope.
@answer n=4 entity=api-security#2.3 v=no
why: CSP is observed absent and the chunk changes no document content policy.
@answer n=5 entity=api-security#2.4 v=no
why: the whole-body response read is a blitz-net path the buffered render never reaches.
@answer n=6 entity=api-security#2.5 v=no
why: the absent request timeout is a reqwest fetch-path fact; the chunk fetches nothing.
@answer n=7 entity=api-security#2.6 v=yes
why: the chunk re-runs the stand_* witnesses, which include stand_load_page, and this entity states the Loads settle and LOAD_WAIT behaviour that test witnesses.
@answer n=8 entity=api-security#2.7 v=no
why: request headers are set inside blitz-net, which the chunk does not touch.
@answer n=9 entity=api-security#2.8 v=no
why: HTTP status mapping is blitz-net's and the buffered render never reaches a response.
@answer n=10 entity=api-security#2.9 v=no
why: AbortSignal cancellation of document fetches is blitz-dom and blitz-traits code the chunk does not change.
@answer n=11 entity=api-security#2.10 v=no
why: sub-resource GETs built by stamped_request are blitz-dom net code outside the chunk.
@answer n=12 entity=api-security#2.11 v=no
why: the JS-facing fetch belongs to blitz-vibey-script, and the Dioxus bridge runs no document scripts (task not_needed 6).
@answer n=13 entity=api-security#2.12 v=no
why: form submission is untouched and the chunk changes no form path (task not_needed 13).
@answer n=14 entity=api-security#2.13 v=no
why: iframe navigation routing is blitz-dom's and the chunk changes no navigation.
@answer n=15 entity=api-security#2.14 v=no
why: the dioxus scheme is answered by dioxus-native's serve_asset, an asset path the buffered render does not reach.
@answer n=16 entity=api-security#2.15 v=no
why: external navigation through webbrowser is link handling, which the chunk's scope does not name.
@answer n=17 entity=api-security#2.16 v=no
why: the NoOpDocument eval path is not called by the chunk's code or tests, and the scope names no eval.
@answer n=18 entity=api-security#2.17 v=no
why: the example outbound GETs are outside the chunk, which edits no example.
@answer n=19 entity=api-security#2.18 v=no
why: apps/browser and rdme outbound HTTP are not reached (task not_needed 8 and 12).
@answer n=20 entity=api-security#2.19 v=no
why: the gh api CI calls live in workflows and scripts the chunk does not edit (task not_needed 10).
@answer n=21 entity=api-security#2.20 v=no
why: the driver session socket and its permission and size bounds are untouched (task not_needed 1), and the answers item 7 keeps are the unchanged driver's own output.
