# Classified — security-plan · api-security.b1

## Answers
api-security#1.0 · yes · add-to · F1: two test-only TCP loopback binds in blitz-net's tests (lib.rs:526 and :534) must be named beside the no-served-port claim
api-security#2.1 · yes · stale · blitz-net/src/lib.rs is a merge surface reached whole, so the cited :24 and :188-197 may have moved and the report does not show they hold
api-security#2.2 · no · the CORS, CSP and same-origin absence rests on searched slices that cite no reached file, and no session fact brings such text
api-security#2.3 · no · the CSP absence rests on the same searched slices that cite no reached file, and no session fact brings CSP or same-origin text
api-security#2.4 · yes · stale · blitz-net/src/lib.rs is a merge surface reached whole, so the cited :215-217 may have moved and the report does not show the claim holds
api-security#2.5 · no · the timeout absence rests on searched slices that cite no reached file, and the session adds no request timeout, LOAD_WAIT bounding only the harness wait
api-security#2.6 · yes · add-to · F2: a failed load now ends the harness and driver wait through CountedHandler's forward of NetHandler::error, which this row must carry beside its in-flight wait
api-security#2.7 · yes · stale · blitz-net/src/lib.rs is a merge surface reached whole, so the cited :199-206 may have moved and the report does not show the claim holds
api-security#2.8 · yes · stale · blitz-net/src/lib.rs is a merge surface reached whole, so the cited :225-228 may have moved and the report does not show the claim holds
api-security#2.9 · yes · stale · blitz-traits/src/net.rs is a merge surface reached whole (NetHandler::error added at :36-38), so the cited :62 and :77-80 may have moved and the report does not show they hold
api-security#2.10 · yes · stale · blitz-dom/src/net.rs is a merge surface reached whole, so the cited :29-35 may have moved and the report does not show the claim holds
api-security#2.11 · no · fetch.js and runtime/fetch.rs are not reached, and the bootstrap changes do not move their lines
api-security#2.12 · no · packages/blitz-dom/src/form.rs is not reached
api-security#2.13 · no · packages/blitz-dom/src/iframe.rs is not reached
api-security#2.14 · no · packages/dioxus-native/src/assets.rs is not reached; only dioxus-native-dom's dioxus_document.rs is
api-security#2.15 · no · packages/dioxus-native/src/link_handler.rs is not reached
api-security#2.16 · no · packages/dioxus-native/src/contexts.rs is not reached
api-security#2.17 · no · the cited example files are not reached and no manifest moved
api-security#2.18 · no · no manifest moved and the apps are not reached
api-security#2.19 · no · wpt-post-results.yml and wpt_diff_to_pr.py are not reached
api-security#2.20 · no · packages/escher-driver holds no diff and host.rs is not reached
