@answer n=1 entity=api-security#1.0 v=no
why: the change is internal to the Dioxus render path and opens no listener, port or socket, so the served-API claim and its driver socket row stand
@answer n=2 entity=api-security#2.1 v=no
why: the change touches no blitz-net file, so the per-host semaphore at lib.rs:24 and 188-197 holds
@answer n=3 entity=api-security#2.2 v=no
why: no CORS or same-origin rule lives in a reached file, and the render path adds no request
@answer n=4 entity=api-security#2.3 v=no
why: no CSP rule lives in a reached file, and the render path adds no request
@answer n=5 entity=api-security#2.4 v=no
why: the response size read at blitz-net lib.rs:215-217 is in a file the change does not reach
@answer n=6 entity=api-security#2.5 v=no
why: the request timeout claim concerns reqwest in blitz-net, which the change does not reach
@answer n=7 entity=api-security#2.6 v=no
why: the settle, screenshot and driver files and the load-page witness are not reached, and the stand suite stays 237 passed unchanged so the load wait still holds
@answer n=8 entity=api-security#2.7 v=no
why: request header handling lives in blitz-net lib.rs:199-206, which the change does not reach
@answer n=9 entity=api-security#2.8 v=no
why: HTTP status mapping lives in blitz-net lib.rs:225-228, which the change does not reach
@answer n=10 entity=api-security#2.9 v=no
why: the AbortSignal rule lives in blitz-traits and blitz-dom config.rs, which the change does not reach
@answer n=11 entity=api-security#2.10 v=no
why: sub-resource GETs built by stamped_request in blitz-dom net.rs are not reached by the change
@answer n=12 entity=api-security#2.11 v=no
why: the JS-facing fetch lives in blitz-vibey-script, which the change does not reach
@answer n=13 entity=api-security#2.12 v=no
why: form submission lives in blitz-dom form.rs, which the change does not reach
@answer n=14 entity=api-security#2.13 v=no
why: iframe navigation routing lives in blitz-dom iframe.rs, which the change does not reach
@answer n=15 entity=api-security#2.14 v=no
why: the dioxus scheme's serve_asset lives in dioxus-native assets.rs, which the change does not reach
@answer n=16 entity=api-security#2.15 v=no
why: external navigation through webbrowser::open lives in dioxus-native link_handler.rs, which the change does not reach and whose mounted-event flush is unchanged
@answer n=17 entity=api-security#2.16 v=no
why: JS evaluation routing to NoOpDocument lives in dioxus-native contexts.rs, which the change does not reach
@answer n=18 entity=api-security#2.17 v=no
why: the examples' plain reqwest GETs live in examples/screenshot.rs and paint_bench.rs, which the change does not reach
@answer n=19 entity=api-security#2.18 v=no
why: outbound HTTP for the apps goes through their manifests and blitz-net, none of which the change edits or reaches
@answer n=20 entity=api-security#2.19 v=no
why: the CI API call path in the .github scripts and workflow is not moved, since no CI step or leg changed
@answer n=21 entity=api-security#2.20 v=no
why: the local session socket lives in escher-driver host.rs and wire.rs, which the change does not reach
