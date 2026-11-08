@answer n=1 entity=surface#6.1 v=no
why: the macOS browser titlebar is in apps/browser/src/main.rs, which the session does not reach
@answer n=2 entity=surface#6.2 v=no
why: the transparent example in examples/transparent/src/app.rs is outside the session's change
@answer n=3 entity=surface#6.3 v=no
why: the window alpha mode in packages/dioxus-native/src/config.rs is not a file the session reaches
@answer n=4 entity=surface#7.1 v=no
why: the wasm32 canvas sizing in packages/dioxus-native/src/config.rs is not a file the session reaches
@answer n=5 entity=surface#7.2 v=no
why: the MountedData buffered render is claimed in dioxus_document.rs and mutation_buffer.rs, neither reached, and the session's dioxus-native-dom edits in actionable.rs and lib.rs are not what this claim states
@answer n=6 entity=surface#7.3 v=no
why: the scrollbars feature forward in packages/dioxus-native/Cargo.toml is not a file the session reaches
@answer n=7 entity=surface#7.4 v=no
why: the select list thumb is in blitz-paint select_list.rs and select_popup_painted.rs, neither reached, and the covered-option reading leaves the paint claim intact
@answer n=8 entity=surface#8.1 v=no
why: the mobile urlbar sizing in apps/browser/assets/browser.css is outside the session's change
@answer n=9 entity=surface#9.1 v=no
why: the mobile frame class in apps/browser/src/main.rs is outside the session's change
@answer n=10 entity=surface#10.1 v=no
why: the Android back button in apps/browser/src/main.rs is outside the session's change
@answer n=11 entity=surface#11.1 v=no
why: the mobile screenshot default name in apps/browser/src/capture.rs is outside the session's change
@answer n=12 entity=surface#12.1 v=no
why: the CI build-only note in .github/workflows/ci.yml is not a file the session reaches
@answer n=13 entity=surface#13.1 v=no
why: the WPT runner result colours are unchanged by the session
@answer n=14 entity=surface#13.2 v=no
why: the WPT runner bright black output is unchanged by the session
@answer n=15 entity=surface#13.3 v=no
why: the WPT runner OSC 8 hyperlinks are unchanged by the session
@answer n=16 entity=surface#14.1 v=no
why: the paint_bench example output is unchanged by the session and its file is not reached
@answer n=17 entity=surface#14.2 v=no
why: the screenshot example output in examples/screenshot.rs is unchanged, and the driver's screenshot answer is a separate line in surface#15.1
@answer n=18 entity=surface#15.1 v=yes mode=add-to
why: the new actions answer line (F1) joins the one-JSON-line-per-command answers this entity lists, and json.rs, which it cites, is a reached file
@answer n=19 entity=surface#15.2 v=no
why: the six served tasks and their usage and enum lists in examples/seven_guis/src/session_host.rs are not changed by the session
@answer n=20 entity=surface#16.1 v=yes mode=stale
why: its cited schema.rs lines 382-427 and cli_commands.rs lines 468-493 are reached by the change, and the graph answer's nodes field now sits beside the actions answer's nodes field of another kind
@answer n=21 entity=surface#17.1 v=yes mode=add-to
why: the unpaged actions answer (F2, 106,088 bytes for 678 nodes) joins the unpaged graph limit this entity states, owned by the same residual res-7cc5bi4k
@answer n=22 entity=surface#18.1 v=no
why: the WASM bundled font registration in examples/wasm_hello is not reached by the session
@answer n=23 entity=surface#19.1 v=no
why: the wasm_hello canvas in examples/wasm_hello/index.html and src/lib.rs is not reached by the session
