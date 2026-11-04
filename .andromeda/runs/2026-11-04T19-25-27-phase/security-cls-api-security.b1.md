@answer n=1 entity=api-security#1.0 v=yes
why: the probe adds no listener or port, and the driver socket stays the one listener, which the no-crossing item and the served-task run rely on
@answer n=2 entity=api-security#2.1 v=no
why: outbound per-host rate limiting is blitz-net's semaphore, and the probe fetches nothing remote
@answer n=3 entity=api-security#2.2 v=no
why: CORS and same-origin are observed absent, and the chunk makes no cross-origin fetch
@answer n=4 entity=api-security#2.3 v=no
why: CSP is observed absent, and the probe loads no remote content
@answer n=5 entity=api-security#2.4 v=no
why: the response body read belongs to blitz-net, and the probe fetches nothing remote
@answer n=6 entity=api-security#2.5 v=no
why: the reqwest request timeout covers outbound requests, and the chunk issues none
@answer n=7 entity=api-security#2.6 v=yes
why: the harness settle and its LOAD_WAIT bound are the headless stand's poll and settle path the probe runs on, named by the scope's settles that never wait on a load in flight
@answer n=8 entity=api-security#2.7 v=no
why: blitz-net request headers apply to outbound requests, and the probe sends none
@answer n=9 entity=api-security#2.8 v=no
why: HTTP status mapping in blitz-net applies to fetches, and the probe makes none
@answer n=10 entity=api-security#2.9 v=no
why: fetch cancellation via AbortSignal governs in-flight fetches, and the probe starts none
@answer n=11 entity=api-security#2.10 v=no
why: blitz-dom sub-resource GETs are not named by the scope, and the probe reads no remote subresource
@answer n=12 entity=api-security#2.11 v=no
why: the JS-facing fetch in blitz-vibey-script is excluded as not needed, since the probe runs Dioxus apps, not document scripts
@answer n=13 entity=api-security#2.12 v=no
why: form submission is excluded as not needed, since browser navigation surfaces are out of scope
@answer n=14 entity=api-security#2.13 v=no
why: iframe navigation in blitz-dom is not named by the scope, and the chunk's apps use no iframe
@answer n=15 entity=api-security#2.14 v=no
why: the dioxus scheme serves assets for URLs the chunk never requests, so the probe's code does not act on it
@answer n=16 entity=api-security#2.15 v=no
why: external navigation to the system browser is excluded as not needed
@answer n=17 entity=api-security#2.16 v=no
why: NoOpDocument eval is a call the probe does not make, since it reads handlers through the listener map
@answer n=18 entity=api-security#2.17 v=no
why: outbound reqwest in the upstream examples is excluded as not needed, and examples/seven_guis makes none
@answer n=19 entity=api-security#2.18 v=no
why: outbound HTTP in the browser and rdme apps is excluded as not needed
@answer n=20 entity=api-security#2.19 v=no
why: gh api CI calls and their token are excluded as not needed, since the wpt-post-results workflow is out of scope
@answer n=21 entity=api-security#2.20 v=yes
why: the served task runs through the driver session, so the 0700 state dir, 0600 socket and the request and answer bounds the probe's reads must fit bind it, as the scope's driver-session item names
