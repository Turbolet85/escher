# Classified — security-plan · api-security.b1

## Answers

api-security#1.0 · yes · add-to · F1: two blitz-net tests bind TCP on loopback (packages/blitz-net/src/lib.rs:526 and :534), not served, so the no-port claim must carry that word
api-security#2.1 · yes · stale · packages/blitz-net/src/lib.rs is a merge surface, so the cited :24 and :188-197 concurrency-limit lines must be re-read on the merged file
api-security#2.2 · no · no F-fact adds CORS, same-origin or cross-origin text, so the observed-absent search still stands
api-security#2.3 · no · no F-fact adds CSP or content-security text, so the observed-absent search still stands
api-security#2.4 · yes · stale · packages/blitz-net/src/lib.rs is a merge surface, so the cited :215-217 whole-body read must be re-read on the merged file
api-security#2.5 · no · no F-fact adds timeout text, so the observed-absent search over the listed s08 files still stands
api-security#2.6 · yes · add-to · F2: a failed load now ends the harness and driver wait through CountedHandler's forward of NetHandler::error (settle.rs:121-125), while LOAD_WAIT still bounds an in-flight load
api-security#2.7 · yes · stale · packages/blitz-net/src/lib.rs is a merge surface, so the cited :199-206 header lines must be re-read on the merged file
api-security#2.8 · yes · stale · packages/blitz-net/src/lib.rs is a merge surface, so the cited :225-228 status mapping must be re-read on the merged file
api-security#2.9 · yes · stale · packages/blitz-traits/src/net.rs is a merge surface and gains NetHandler::error at :36-38, which moves the cited :62 and :77-80 lines
api-security#2.10 · yes · stale · packages/blitz-dom/src/net.rs is a merge surface, so the cited :29-35 stamped_request lines must be re-read on the merged file
api-security#2.11 · no · cites fetch.js and fetch.rs in blitz-vibey-script, neither reached; URLSearchParams adds no fetch path
api-security#2.12 · no · cites packages/blitz-dom/src/form.rs, not reached
api-security#2.13 · no · cites packages/blitz-dom/src/iframe.rs, not reached
api-security#2.14 · no · cites packages/dioxus-native/src/assets.rs, not reached (the reached dioxus-native-dom is a different crate)
api-security#2.15 · no · cites packages/dioxus-native/src/link_handler.rs, not reached
api-security#2.16 · no · cites packages/dioxus-native/src/contexts.rs, not reached
api-security#2.17 · no · cites examples/screenshot.rs and examples/paint_bench.rs, not reached
api-security#2.18 · no · cites apps/browser and apps/readme Cargo.toml, manifests byte-identical
api-security#2.19 · no · cites .github/scripts/wpt_diff_to_pr.py and .github/workflows/wpt-post-results.yml, neither reached
api-security#2.20 · no · cites packages/escher-driver, which holds no diff
