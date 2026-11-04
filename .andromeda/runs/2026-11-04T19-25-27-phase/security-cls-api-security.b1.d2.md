@answer n=1 entity=api-security#1.0 v=yes
why: the served task may run through the driver session, and task item 3 bars any new listener, verb or socket, so the no-listener topology and the twelve-verb MCP surface bind the probe's code
@answer n=2 entity=api-security#2.1 v=no
why: outbound per-host concurrency in blitz-net, which the probe never reaches because it fetches nothing remote
@answer n=3 entity=api-security#2.2 v=no
why: CORS and same-origin absence concerns cross-origin web requests, which the probe does not make
@answer n=4 entity=api-security#2.3 v=no
why: CSP absence concerns the remote web content boundary, which is not needed by the chunk
@answer n=5 entity=api-security#2.4 v=no
why: the response body read in blitz-net, which the probe never reaches because it fetches nothing remote
@answer n=6 entity=api-security#2.5 v=no
why: the absent reqwest request timeout sits on a fetch path the probe never reaches, and the harness's own load bound is carried by entity 2.6
@answer n=7 entity=api-security#2.6 v=yes
why: the harness settle and the driver's acting step never wait on a load in flight, which the chunk's poll(None) and settles use directly (task item 8)
@answer n=8 entity=api-security#2.7 v=no
why: blitz-net request header forwarding, which the probe does not reach
@answer n=9 entity=api-security#2.8 v=no
why: blitz-net HTTP status mapping, which the probe does not reach
@answer n=10 entity=api-security#2.9 v=no
why: AbortSignal cancellation of document fetches, and the probe starts no fetch
@answer n=11 entity=api-security#2.10 v=no
why: sub-resource GETs stamped in blitz-dom, which the probe does not reach because it fetches nothing remote
@answer n=12 entity=api-security#2.11 v=no
why: JS-facing fetch over the ScriptFetcher, which is not needed because the probe runs Dioxus apps, not document scripts
@answer n=13 entity=api-security#2.12 v=no
why: form submission and its method and scheme rules are browser navigation surfaces the chunk does not need
@answer n=14 entity=api-security#2.13 v=no
why: iframe sub-document navigation, which the probe's scope does not name
@answer n=15 entity=api-security#2.14 v=no
why: the dioxus scheme's local asset serving, which the probe's scope does not name
@answer n=16 entity=api-security#2.15 v=no
why: external link handing to the system browser is browser navigation, which the chunk does not need
@answer n=17 entity=api-security#2.16 v=no
why: JS evaluation routed to NoOpDocument in dioxus-native, which the probe does not reach
@answer n=18 entity=api-security#2.17 v=no
why: outbound reqwest in the upstream examples screenshot and paint_bench, which are not the reached examples/seven_guis and are not needed
@answer n=19 entity=api-security#2.18 v=no
why: outbound HTTP in the browser and rdme apps, which the chunk does not reach
@answer n=20 entity=api-security#2.19 v=no
why: CI gh api calls with the token secret in the WPT workflow, which the task lists as not needed
@answer n=21 entity=api-security#2.20 v=yes
why: when the served task runs through the driver session, the 0700 and 0600 socket rules, the path parameter and the byte bounds bind the probe (task items 9 and 3)
