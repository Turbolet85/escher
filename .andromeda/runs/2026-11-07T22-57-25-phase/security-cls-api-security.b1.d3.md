@answer n=1 entity=api-security#1.0 v=yes
why: the chunk adds a verb and an MCP tool and must keep the session socket the one listener, with no served network API added
@answer n=2 entity=api-security#2.1 v=no
why: outbound per-host rate limiting in blitz-net, a fetch path the graph read never reaches since it fetches nothing
@answer n=3 entity=api-security#2.2 v=no
why: CORS and same-origin absence on outbound and document fetch paths the chunk does not touch
@answer n=4 entity=api-security#2.3 v=no
why: CSP absence on the outbound and sub-resource fetch paths the chunk does not touch
@answer n=5 entity=api-security#2.4 v=no
why: response body reads in blitz-net's outbound fetch, which the chunk does not change
@answer n=6 entity=api-security#2.5 v=no
why: outbound reqwest request timeout, not reached by a graph read that fetches nothing
@answer n=7 entity=api-security#2.6 v=yes
why: the new verb is an instance-level read that must carry no wait argument or result field, and its validation must read no clock
@answer n=8 entity=api-security#2.7 v=no
why: outbound request headers in blitz-net, a path the chunk does not reach
@answer n=9 entity=api-security#2.8 v=no
why: HTTP status mapping in blitz-net, a path the chunk does not reach
@answer n=10 entity=api-security#2.9 v=no
why: fetch cancellation through AbortSignal, a fetch path the graph read never reaches
@answer n=11 entity=api-security#2.10 v=no
why: sub-resource GETs built in blitz-dom's net module, which the chunk leaves unchanged
@answer n=12 entity=api-security#2.11 v=no
why: JS-facing fetch in blitz-vibey-script, which the UI graph does not touch
@answer n=13 entity=api-security#2.12 v=no
why: form submission in blitz-dom, carved out as not needed for this chunk
@answer n=14 entity=api-security#2.13 v=no
why: iframe sub-document navigation in blitz-dom, which the graph does not touch
@answer n=15 entity=api-security#2.14 v=no
why: the dioxus scheme's local asset serving in dioxus-native, which the chunk does not change
@answer n=16 entity=api-security#2.15 v=no
why: external navigation through webbrowser, the browser-app path carved out as not needed
@answer n=17 entity=api-security#2.16 v=no
why: JS evaluation routed to NoOpDocument in dioxus-native, which the graph read never invokes
@answer n=18 entity=api-security#2.17 v=no
why: plain reqwest GETs in example binaries, outside the chunk's code and carved out as not needed
@answer n=19 entity=api-security#2.18 v=no
why: outbound HTTP in the browser and rdme apps, which the chunk does not touch
@answer n=20 entity=api-security#2.19 v=no
why: CI GitHub API calls and their token, carved out as CI secrets and supply chain not needed
@answer n=21 entity=api-security#2.20 v=yes
why: the session socket carries the new verb's answer, so its owner-only trust boundary and its request and answer bounds and idle expiry bind
