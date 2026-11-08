@answer n=1 entity=api-security#1.0 v=yes mode=stale
why: the session adds the actions tool, so the MCP tool count moves from thirteen to fourteen
@answer n=2 entity=api-security#2.1 v=no
why: the outbound per-host semaphore lives in blitz-net, which the session does not touch
@answer n=3 entity=api-security#2.2 v=no
why: the session adds no request, origin or policy code, so the observed absence of CORS and same-origin still holds
@answer n=4 entity=api-security#2.3 v=no
why: the session adds no content-security policy code, so the observed absence still holds
@answer n=5 entity=api-security#2.4 v=no
why: the whole-body response read in blitz-net is untouched by the actions change
@answer n=6 entity=api-security#2.5 v=no
why: the session adds no outbound request or timeout code
@answer n=7 entity=api-security#2.6 v=yes mode=stale
why: the instance verbs move from nine to ten with actions, which takes no argument and moves no time, so the nine count moved
@answer n=8 entity=api-security#2.7 v=no
why: request header handling in blitz-net is untouched by the session
@answer n=9 entity=api-security#2.8 v=no
why: the HTTP status mapping in blitz-net is untouched by the session
@answer n=10 entity=api-security#2.9 v=no
why: request cancellation in blitz-traits and blitz-dom is untouched by the session
@answer n=11 entity=api-security#2.10 v=no
why: the sub-resource GETs built by stamped_request in blitz-dom are untouched by the session
@answer n=12 entity=api-security#2.11 v=no
why: the JS-facing fetch in blitz-vibey-script is untouched by the session
@answer n=13 entity=api-security#2.12 v=no
why: form submission in blitz-dom is untouched by the session
@answer n=14 entity=api-security#2.13 v=no
why: iframe navigation in blitz-dom is untouched by the session
@answer n=15 entity=api-security#2.14 v=no
why: the dioxus scheme in dioxus-native assets is untouched, the session changing only dioxus-native-dom
@answer n=16 entity=api-security#2.15 v=no
why: external navigation through webbrowser is untouched by the session
@answer n=17 entity=api-security#2.16 v=no
why: the NoOpDocument evaluation path in dioxus-native is untouched, the session changing a different crate
@answer n=18 entity=api-security#2.17 v=no
why: the examples' plain reqwest GETs are untouched by the session
@answer n=19 entity=api-security#2.18 v=no
why: the apps' outbound HTTP through blitz-net is untouched by the session
@answer n=20 entity=api-security#2.19 v=no
why: the cited WPT scripts and workflow are untouched, and the session adds no GitHub API call
@answer n=21 entity=api-security#2.20 v=no
why: the socket's owner-only access, its bounds and the 1,048,576-byte answer bound all hold, the actions answer measuring 106,088 bytes on Cells, and the cited host.rs and wire.rs are not reached
@answer n=22 entity=api-security#2.21 v=yes mode=add-to
why: the residual res-7cc5bi4k is widened to own the unpaged actions answer beside the graph answer, F7, measured at 106,088 bytes on Cells
