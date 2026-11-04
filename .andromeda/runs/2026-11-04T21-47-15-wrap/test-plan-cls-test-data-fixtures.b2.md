@answer n=1 entity=test-data-fixtures#3.24 v=no
why: the session touched no browser test-URL source (apps/browser) and no fixture the row names
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: the session changed no favicon fixture or apps/browser/src/favicon.rs
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the session changed no blitz-vibey-script fetch fixture or its tests/dom.rs
@answer n=4 entity=test-data-fixtures#3.27 v=no
why: LoadNet, the stand load boot and run stand (237 over 51 files) are unchanged, and the new probe boots through Harness::from_vdom, not a stand boot
@answer n=5 entity=test-data-fixtures#3.28 v=no
why: the session changed no render_blocking_stylesheet answer or its test file
@answer n=6 entity=test-data-fixtures#3.29 v=no
why: the session changed nothing under packages/, so the blitz-net loopback listener tests are unchanged
@answer n=7 entity=test-data-fixtures#4.1 v=yes mode=add-to
why: F5 adds dioxus_runtime_probe's one-reader fixture pages (toggled_listener, shared_state, hazard) in their own file, a fixture file this absence record must now carry
