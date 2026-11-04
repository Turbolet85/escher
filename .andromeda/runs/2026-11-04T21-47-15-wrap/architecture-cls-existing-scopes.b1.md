@answer n=1 entity=existing-scopes#1.1 v=no
why: the session changed no file under packages/accesskit_xplat, so its module list and citations still hold.
@answer n=2 entity=existing-scopes#1.2 v=no
why: no file under packages/debug_timer changed, so the enable and dummy split still holds.
@answer n=3 entity=existing-scopes#1.3 v=no
why: no file under packages/blitz-traits changed, so the module list at lib.rs:4-9 still holds.
@answer n=4 entity=existing-scopes#1.4 v=no
why: no file under packages/blitz-dom changed, so its module list and the two cfg(test) child modules still hold.
@answer n=5 entity=existing-scopes#1.5 v=no
why: cssom.rs and resolved_style.rs are unchanged, so their claims still hold.
@answer n=6 entity=existing-scopes#1.6 v=no
why: form.rs, iframe.rs and query_selector.rs are unchanged, so their claims still hold.
@answer n=7 entity=existing-scopes#1.7 v=no
why: document.rs hit, focus, select and geometry sites are unchanged, the probe reads public API only, so the claims still hold.
@answer n=8 entity=existing-scopes#1.8 v=no
why: accessibility.rs and debug.rs are unchanged, so their claims still hold.
@answer n=9 entity=existing-scopes#1.9 v=no
why: the events submodules are unchanged, and the set_focus hazard at events.rs:295 lives in dioxus-native-dom, not here.
@answer n=10 entity=existing-scopes#1.10 v=no
why: scrolling.rs and selection.rs are unchanged, so their claims still hold.
@answer n=11 entity=existing-scopes#1.11 v=no
why: the Stylo integration files are unchanged, so their claims still hold.
@answer n=12 entity=existing-scopes#1.12 v=no
why: tree.rs, traversal.rs and url.rs are unchanged, so their claims still hold.
@answer n=13 entity=existing-scopes#1.13 v=no
why: the layout directory is unchanged, so its construct, damage, inline, list and table citations still hold.
@answer n=14 entity=existing-scopes#1.14 v=no
why: the node directory, including select_list and range, is unchanged, so its claims still hold.
@answer n=15 entity=existing-scopes#1.15 v=no
why: no file under packages/blitz-html changed, so its claims still hold.
@answer n=16 entity=existing-scopes#1.16 v=no
why: no file under packages/blitz-net changed, so its claims still hold.
@answer n=17 entity=existing-scopes#1.17 v=no
why: no file under packages/blitz-paint changed, so its render and select_list citations still hold.
@answer n=18 entity=existing-scopes#1.18 v=no
why: its integration tests under packages/blitz-vibey-script are unchanged, and the new probe lives in tests/blitz-tests, not here.
@answer n=19 entity=existing-scopes#1.19 v=no
why: no file under packages/blitz-shell changed, so its module list and unit test citation still hold.
@answer n=20 entity=existing-scopes#1.20 v=no
why: no file under packages/blitz changed, so its re-export citations still hold.
@answer n=21 entity=existing-scopes#1.21 v=no
why: no file under packages/stylo_taffy changed, so its module list still holds.
@answer n=22 entity=existing-scopes#1.22 v=no
why: packages/dioxus-native and its Cargo.toml:92 dependency are unchanged, and the pulldown-cmark bump in Cargo.lock is reached only through rdme, so the claim still holds.
@answer n=23 entity=existing-scopes#1.23 v=no
why: no file under packages/dioxus-native-dom changed, so its module list and the bridge_tests citation at dioxus_document.rs:838-840 still hold.
@answer n=24 entity=existing-scopes#1.24 v=no
why: no file under packages/blitz-test-harness changed, so its six module claims still hold.
@answer n=25 entity=existing-scopes#1.25 v=no
why: no file under packages/escher-telemetry changed, and the new probe installs no subscriber and prints nothing, so its format and init claims still hold.
