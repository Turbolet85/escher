# Classified — security · api-security.b1

## Answers
api-security#1.0 · yes · the MCP help tables and lint findings that the chunk's schema, refusal and lint.rs edits move leave under the 1,048,576-byte tool-result bound, and lint keeps its no-argument, no-wait contract
api-security#2.1 · no · blitz-net's per-host concurrency limit on outbound fetches; the chunk changes no fetching and the load page is answered in process by LoadNet
api-security#2.2 · no · CORS and same-origin are observed absent and stay so; outbound request controls are not reached (Not needed)
api-security#2.3 · no · CSP is observed absent and stays so; outbound request controls are not reached (Not needed)
api-security#2.4 · no · blitz-net reads successful response bodies whole; net provider and resource intake change nothing (Not needed)
api-security#2.5 · no · the absent request timeout is reqwest's on blitz-net's fetch path, which the load page does not use; the chunk's load waits end at LOAD_WAIT under #2.6
api-security#2.6 · yes · the load page's Enter and Space rows on load-fetch press through Session::act, which waits on a load in flight for at most LOAD_WAIT, and lint's no-argument, no-wait contract binds the overlaps edit
api-security#2.7 · no · blitz-net forwards caller headers and sets User-Agent on outbound requests; not reached (Not needed)
api-security#2.8 · no · blitz-net maps non-success statuses to ProviderError::HttpStatus; fetch outcomes are not reached
api-security#2.9 · no · AbortSignal cancellation of document fetches sits in blitz-traits and blitz-dom config.rs, neither reached; no debt changes fetching
api-security#2.10 · no · stamped_request in blitz-dom net.rs is not a reached file; sub-resource GETs are not changed
api-security#2.11 · no · the JS-facing fetch belongs to blitz-vibey-script; the debts lie in the engine and driver, not the script runtime (Not needed)
api-security#2.12 · no · form submission schemes in form.rs stay as they are (Not needed); the dialog here is a form method value, not the element's :open check
api-security#2.13 · no · iframe sub-document navigation routing in iframe.rs is not reached; in-document navigation stays as it is (Not needed)
api-security#2.14 · no · the dioxus scheme serves local assets through serve_asset in dioxus-native, not reached; resource intake stays as it is (Not needed)
api-security#2.15 · no · webbrowser::open routing in dioxus-native's link_handler is not reached; external-browser routing stays as it is (Not needed)
api-security#2.16 · no · JS evaluation goes to NoOpDocument in dioxus-native contexts, not reached; the debts lie in the engine and driver (Not needed)
api-security#2.17 · no · the examples' plain reqwest GETs are not reached; no debt touches examples or outbound fetching
api-security#2.18 · no · app outbound HTTP through blitz-net and reqwest; the browser app and rdme are outside the six debts (Not needed)
api-security#2.19 · no · CI's gh api calls and workflow token; the chunk adds no CI job and touches no release path (Not needed)
api-security#2.20 · yes · the session socket's 1,048,576-byte answer bound, with oversize over it, binds the lint findings and the off-screen and covered refusals the chunk moves, which leave by the socket under the reply bounds
