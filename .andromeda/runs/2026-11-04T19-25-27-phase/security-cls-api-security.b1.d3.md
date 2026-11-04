@answer n=1 entity=api-security#1.0 v=yes
why: the served task runs through the driver socket, and needs items 3 and 9 keep its one-listener rule and the twelve-verb schema untouched.
@answer n=2 entity=api-security#2.1 v=no
why: blitz-net's per-host outbound limit; the probe fetches nothing remote (not-needed item 2).
@answer n=3 entity=api-security#2.2 v=no
why: CORS and same-origin are observed absent for outbound and cross-origin paths; the probe makes no cross-origin request.
@answer n=4 entity=api-security#2.3 v=no
why: CSP is observed absent; the probe loads no remote page content and runs no document script.
@answer n=5 entity=api-security#2.4 v=no
why: outbound response bodies are read whole; the probe fetches nothing remote.
@answer n=6 entity=api-security#2.5 v=no
why: reqwest's request timeout is observed absent; the probe fetches nothing, and the stand's load page is answered in process by LoadNet, not by reqwest.
@answer n=7 entity=api-security#2.6 v=yes
why: the settle never waits on a load in flight and the driver's acting step waits at most LOAD_WAIT; needs item 8 names settles that never wait on a load, and the stand's load page is the wait witness the probe runs on.
@answer n=8 entity=api-security#2.7 v=no
why: outbound request headers; the probe sends no request.
@answer n=9 entity=api-security#2.8 v=no
why: HTTP status mapping for outbound fetches; the probe reads no HTTP status.
@answer n=10 entity=api-security#2.9 v=no
why: AbortSignal cancellation of document fetches; the scope names no fetch the probe cancels.
@answer n=11 entity=api-security#2.10 v=no
why: blitz-dom's sub-resource GETs; the probe loads no sub-resources (not-needed item 2).
@answer n=12 entity=api-security#2.11 v=no
why: the JS-facing fetch in blitz-vibey-script; document script execution is not-needed item 3.
@answer n=13 entity=api-security#2.12 v=no
why: form submission; the probe submits no form (not-needed item 8).
@answer n=14 entity=api-security#2.13 v=no
why: iframe navigation routing; the probe renders no iframe and navigates nothing.
@answer n=15 entity=api-security#2.14 v=no
why: the dioxus scheme's local asset serving; the scope names no asset path the probe reads.
@answer n=16 entity=api-security#2.15 v=no
why: external navigation through webbrowser::open; the probe follows no link (not-needed item 8).
@answer n=17 entity=api-security#2.16 v=no
why: JS evaluation answered by NoOpDocument; the probe evaluates no script and reads no eval result.
@answer n=18 entity=api-security#2.17 v=no
why: outbound HTTP in the screenshot and paint_bench examples; the chunk touches examples/seven_guis only.
@answer n=19 entity=api-security#2.18 v=no
why: outbound HTTP in the browser and rdme apps; not-needed items 1 and 6 drop them.
@answer n=20 entity=api-security#2.19 v=no
why: GitHub API calls in the wpt-post-results CI workflow; not-needed item 4 drops that workflow.
@answer n=21 entity=api-security#2.20 v=yes
why: the driver session's socket (0600), its state directory (0700), its bounds and idle expiry are what needs item 9 names for a served task run through the driver session.
