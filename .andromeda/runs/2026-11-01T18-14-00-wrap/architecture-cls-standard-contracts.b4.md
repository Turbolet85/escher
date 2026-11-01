# Classified — architecture · standard-contracts.b4

## Answers
standard-contracts#21.1 · no · the recording lives in escher-driver's recording.rs and session.rs, which hold no diff in this take, and its kept-check claims stand
standard-contracts#21.2 · no · replay.rs is outside the reached set and Check::replay's step comparison is unchanged
standard-contracts#21.3 · yes · add-to · F19: Session::lint's contrast reads None past a blended element through Harness::text_contrast (contrast.rs reached), beside the existing "None not measured"
standard-contracts#21.4 · yes · add-to · F15: CountedHandler forwards NetHandler::error, so a failed load ends the driver session's wait early instead of running to LOAD_WAIT; settle_waiting unedited
standard-contracts#22.1 · no · the select overlap readings rest on lint.rs and the stand_lint and stand_act_popup tests, none reached; select_popup's row-3 skip is another test
standard-contracts#22.2 · no · the load-page witness is a successful load, so the failed-load forward does not reach its load-done path; stand_load_page.rs and examples/seven_guis hold no diff
standard-contracts#23.1 · yes · stale · BOOTSTRAP_JS's concat moves from nine include_str! scripts to ten with url_search_params first, its bootstrap.rs citations move, and URLSearchParams joins the globals (F21)
standard-contracts#24.1 · no · document_may_load's fetch path (fetch.rs, runtime/fetch.rs, modules.rs, document.rs) is not reached and its file-URL rule holds; url_search_params adds no fetch
standard-contracts#25.1 · no · the CSS accessor claim lives in css.rs and bootstrap/style.js, neither reached; the new bootstrap script defines no CSSStyleDeclaration accessor, so installStyleProperties still holds
standard-contracts#26.1 · no · dioxus-native launch, config, hooks and contexts are outside the reached set, dioxus_document.rs belongs to dioxus-native-dom and is not cited, and blitz launch_url is unchanged
