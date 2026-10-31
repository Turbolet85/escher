# Classified — security · api-security.b1

## Answers
api-security#1.0 · yes · the MCP results, `lint` findings and `screenshot` carriers the open list's data travels, and the no-listener fact the chunk must keep (no new port, socket or listener)
api-security#2.1 · no · outbound rate limit on blitz-net fetches; the list fetches nothing
api-security#2.2 · no · CORS absent on fetches; the list fetches nothing
api-security#2.3 · no · CSP absent on fetched content; the overlay paints document text and adds no fetch or CSP, so the absence does not move
api-security#2.4 · no · response body size on fetches; the list paints text already parsed
api-security#2.5 · no · request timeout on fetches; the chunk makes no fetch
api-security#2.6 · yes · the acting step's settle over a load in flight (`Session::act`, `LOAD_WAIT`) binds an option acted on through the driver, and `lint` and `screenshot` wait on none
api-security#2.7 · no · request headers of blitz-net fetches; the list sends none
api-security#2.8 · no · HTTP status mapping of fetches; the list makes none
api-security#2.9 · no · fetch cancellation through AbortSignal; the list starts no fetch
api-security#2.10 · no · sub-resource GETs through stamped_request; the list loads no sub-resource
api-security#2.11 · no · JS-facing fetch; the list adds no native API or script path
api-security#2.12 · no · form submission rules in form.rs; a select's choice submits nothing and the chunk leaves form.rs unchanged
api-security#2.13 · no · iframe sub-document navigation; the list navigates nothing
api-security#2.14 · no · `dioxus` scheme asset serving; the list paints no fetched asset
api-security#2.15 · no · external navigation through webbrowser; link Enter stays unchanged and the list opens no link
api-security#2.16 · no · JS evaluation goes to NoOpDocument; the list evaluates nothing
api-security#2.17 · no · example outbound GET; the scope edits no example
api-security#2.18 · no · app outbound HTTP through blitz-net; the chunk makes no fetch
api-security#2.19 · no · CI `gh api` calls and `GH_TOKEN`; the chunk touches no workflow job or secret
api-security#2.20 · yes · the session socket's request and reply bounds, and the 1,048,576-byte answer, carry the open list's snapshot, `execute` and `lint` answers the chunk changes
