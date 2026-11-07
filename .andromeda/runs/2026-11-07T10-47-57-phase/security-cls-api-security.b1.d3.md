@answer n=1 entity=api-security#1.0 v=yes
why: the chunk adds no listener, port or served route, and the scope names listeners and ports as not added, so the served-API inventory binds what the change must keep
@answer n=2 entity=api-security#2.1 v=no
why: per-host outbound concurrency in blitz-net, and the buffered render fetches nothing, so the chunk neither uses nor changes it
@answer n=3 entity=api-security#2.2 v=no
why: CORS and same-origin absence concerns cross-origin fetches, which the buffered render does not perform
@answer n=4 entity=api-security#2.3 v=no
why: CSP absence concerns document fetches, which the chunk does not reach
@answer n=5 entity=api-security#2.4 v=no
why: response body reads in blitz-net are off the Dioxus render path, which fetches nothing
@answer n=6 entity=api-security#2.5 v=no
why: request timeout on reqwest fetches, untouched by a render path that fetches nothing
@answer n=7 entity=api-security#2.6 v=no
why: harness settle and the driver's LOAD_WAIT are untouched, and the scope names no settle call, verb or value that the probe tests read
@answer n=8 entity=api-security#2.7 v=no
why: request headers on blitz-net fetches, which the chunk does not send
@answer n=9 entity=api-security#2.8 v=no
why: HTTP status mapping in blitz-net, not reached by the in-process render
@answer n=10 entity=api-security#2.9 v=no
why: AbortSignal cancellation of document fetches, which the buffered render does not start
@answer n=11 entity=api-security#2.10 v=no
why: sub-resource GET construction in blitz-dom net, untouched and unused by the chunk
@answer n=12 entity=api-security#2.11 v=no
why: the JS-facing fetch over ScriptFetcher, and the scope states the bridge runs no document scripts
@answer n=13 entity=api-security#2.12 v=no
why: form submission paths in blitz-dom, which the scope states the chunk does not change
@answer n=14 entity=api-security#2.13 v=no
why: iframe navigation routing in blitz-dom, not touched by the render port
@answer n=15 entity=api-security#2.14 v=no
why: the dioxus scheme asset server in dioxus-native assets.rs serves requests the chunk does not reach
@answer n=16 entity=api-security#2.15 v=no
why: external navigation through webbrowser in the link handler, which the chunk does not change
@answer n=17 entity=api-security#2.16 v=no
why: JS evaluation routed to NoOpDocument in contexts.rs, which the chunk neither uses nor changes and whose behaviour the scope does not name
@answer n=18 entity=api-security#2.17 v=no
why: example outbound reqwest GETs, outside the bridge and untouched by the chunk
@answer n=19 entity=api-security#2.18 v=no
why: outbound HTTP in the browser and rdme apps, and the scope states apps/browser is not reached
@answer n=20 entity=api-security#2.19 v=no
why: CI gh api calls and their GH_TOKEN wiring, and the scope states no workflow is edited
@answer n=21 entity=api-security#2.20 v=no
why: the driver session socket's bounds and permissions, which the scope states the chunk does not touch by adding no carrier or listener
