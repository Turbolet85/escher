@answer n=1 entity=api-security#1.0 v=yes mode=stale
why: the MCP tool count moved from twelve to thirteen with graph, and the session socket and MCP replies now carry a UI graph
@answer n=2 entity=api-security#2.1 v=no
why: the blitz-net outbound semaphore lives in blitz-net, which the change did not touch
@answer n=3 entity=api-security#2.2 v=no
why: the CORS and same-origin absence holds, since the change as described adds no fetch or cross-origin code to the reached driver and dioxus-native-dom files
@answer n=4 entity=api-security#2.3 v=no
why: the CSP absence holds, since the change as described adds no content-security code to the reached files
@answer n=5 entity=api-security#2.4 v=no
why: response bodies are read in blitz-net, which the change did not touch, and the unpaged graph answer is a driver reply carried by row api-security#2.20
@answer n=6 entity=api-security#2.5 v=no
why: the timeout absence holds, since the change as described adds no request or timeout to the reached driver and dioxus-native-dom files
@answer n=7 entity=api-security#2.6 v=yes mode=stale
why: the eight instance-level verbs moved to nine with graph as the ninth, a read that waits on no load, so the count is stale
@answer n=8 entity=api-security#2.7 v=no
why: request headers are set in blitz-net, which the change did not touch
@answer n=9 entity=api-security#2.8 v=no
why: HTTP status handling is in blitz-net, which the change did not touch
@answer n=10 entity=api-security#2.9 v=no
why: request cancellation lives in blitz-traits and blitz-dom config, and the change reaches neither
@answer n=11 entity=api-security#2.10 v=no
why: sub-resource GETs are built in blitz-dom net.rs, which the change did not touch
@answer n=12 entity=api-security#2.11 v=no
why: the JS-facing fetch lives in blitz-vibey-script, which the change did not touch
@answer n=13 entity=api-security#2.12 v=no
why: form submission lives in blitz-dom form.rs, which the change did not touch
@answer n=14 entity=api-security#2.13 v=no
why: iframe navigation lives in blitz-dom iframe.rs, which the change did not touch
@answer n=15 entity=api-security#2.14 v=no
why: the dioxus scheme serve_asset lives in dioxus-native assets.rs, which the change did not reach, its re-export sitting in lib.rs
@answer n=16 entity=api-security#2.15 v=no
why: external navigation lives in dioxus-native link_handler.rs, which the change did not reach
@answer n=17 entity=api-security#2.16 v=no
why: evaluation routing to NoOpDocument lives in dioxus-native contexts.rs, which the change did not reach, its re-export sitting in lib.rs
@answer n=18 entity=api-security#2.17 v=no
why: the example reqwest GETs sit in example files the change did not reach
@answer n=19 entity=api-security#2.18 v=no
why: the app outbound HTTP path goes through blitz-net and rdme, and neither changed
@answer n=20 entity=api-security#2.19 v=no
why: the gh api calls sit in CI scripts and workflows the change did not touch
@answer n=21 entity=api-security#2.20 v=yes mode=add-to
why: the session socket's answer bounds gain F7, the unpaged graph answer at 293,869 bytes within the 1,048,576-byte answer bound with its size as the stated limit
