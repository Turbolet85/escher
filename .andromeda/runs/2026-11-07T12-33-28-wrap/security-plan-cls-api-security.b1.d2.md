@answer n=1 entity=api-security#1.0 v=no
why: the session adds no bind, listen, socket or port; the Dioxus render-path change is in-process and the reached files hold no listener, so the no-served-API claim still holds
@answer n=2 entity=api-security#2.1 v=no
why: the outbound per-host semaphore lives in blitz-net, which the session did not reach
@answer n=3 entity=api-security#2.2 v=no
why: the session adds no CORS or same-origin code, and the reached dioxus-native-dom files do not hold the searched outbound-request rules
@answer n=4 entity=api-security#2.3 v=no
why: the session adds no CSP code or header, and the reached dioxus-native-dom files do not hold it
@answer n=5 entity=api-security#2.4 v=no
why: the response size handling lives in blitz-net, which the session did not reach, and no body cap was added
@answer n=6 entity=api-security#2.5 v=no
why: the request timeout is observed absent and the session adds none in blitz-net or the reached files
@answer n=7 entity=api-security#2.6 v=no
why: the settle and load-wait rules live in blitz-test-harness and the stand, which the session did not reach, and the Dioxus render-path change does not alter the load counter
@answer n=8 entity=api-security#2.7 v=no
why: request header forwarding lives in blitz-net, which the session did not reach
@answer n=9 entity=api-security#2.8 v=no
why: HTTP status mapping lives in blitz-net, which the session did not reach
@answer n=10 entity=api-security#2.9 v=no
why: fetch cancellation lives in blitz-traits and blitz-dom config, which the session did not reach, and the render-path change leaves the AbortSignal path alone
@answer n=11 entity=api-security#2.10 v=no
why: sub-resource GETs are built in blitz-dom net.rs, which the session did not reach
@answer n=12 entity=api-security#2.11 v=no
why: the JS-facing fetch lives in blitz-vibey-script, which the session did not reach
@answer n=13 entity=api-security#2.12 v=no
why: form submission lives in blitz-dom form.rs, which the session did not reach
@answer n=14 entity=api-security#2.13 v=no
why: iframe navigation routing lives in blitz-dom iframe.rs, which the session did not reach
@answer n=15 entity=api-security#2.14 v=no
why: the dioxus scheme handler lives in the dioxus-native crate's assets.rs, a different crate from the reached dioxus-native-dom, and that file is not changed
@answer n=16 entity=api-security#2.15 v=no
why: external navigation is handed to the system browser in dioxus-native link_handler.rs, which the session did not reach
@answer n=17 entity=api-security#2.16 v=no
why: evaluation routing to NoOpDocument lives in dioxus-native contexts.rs, which the session did not reach
@answer n=18 entity=api-security#2.17 v=no
why: the outbound GETs live in the examples screenshot.rs and paint_bench.rs, which the session did not reach
@answer n=19 entity=api-security#2.18 v=no
why: the app outbound HTTP lives in the browser and readme app manifests and code, and the session moved no app or manifest
@answer n=20 entity=api-security#2.19 v=no
why: the GitHub API calls live in the .github scripts and workflows, and no workflow or CI step changed
@answer n=21 entity=api-security#2.20 v=no
why: the local session socket lives in escher-driver, which the session did not reach
