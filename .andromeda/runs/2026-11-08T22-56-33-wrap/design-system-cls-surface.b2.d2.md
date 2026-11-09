@answer n=1 entity=surface#6.1 v=no
why: its claim is the browser app's macOS titlebar in apps/browser, which the session does not reach
@answer n=2 entity=surface#6.2 v=no
why: its claim is the transparent example's window in examples/transparent, which the session does not reach
@answer n=3 entity=surface#6.3 v=no
why: its claim is window alpha mode in packages/dioxus-native config, which the session does not reach
@answer n=4 entity=surface#7.1 v=no
why: its claim is the wasm canvas in packages/dioxus-native config, which the session does not reach
@answer n=5 entity=surface#7.2 v=yes mode=stale
why: the session reaches dioxus_document.rs, which this entity cites at lines 168-170 and 316-318, and the reached-handler write on the handle_event path may have moved them
@answer n=6 entity=surface#7.3 v=no
why: its claim is the scrollbars feature forward in packages/dioxus-native Cargo.toml, which the session does not reach
@answer n=7 entity=surface#7.4 v=no
why: its claim is the select list thumb in blitz-paint and its test, neither of which the session reaches
@answer n=8 entity=surface#8.1 v=no
why: its claim is the mobile urlbar CSS in apps/browser, which the session does not reach
@answer n=9 entity=surface#9.1 v=no
why: its claim is the mobile frame class in apps/browser, which the session does not reach
@answer n=10 entity=surface#10.1 v=no
why: its claim is the Android back button in apps/browser, which the session does not reach
@answer n=11 entity=surface#11.1 v=no
why: its claim is mobile screenshot naming in apps/browser capture, which the session does not reach
@answer n=12 entity=surface#12.1 v=no
why: its claim is the CI workflow's mobile build lines, which the session does not reach
@answer n=13 entity=surface#13.1 v=no
why: its claim is the WPT runner's result colours, and the session adds no colour and does not reach the runner
@answer n=14 entity=surface#13.2 v=no
why: its claim is the WPT runner's bright-black printing, and the session does not reach the runner
@answer n=15 entity=surface#13.3 v=no
why: its claim is the WPT runner's OSC 8 hyperlinks, and the session does not reach the runner
@answer n=16 entity=surface#14.1 v=no
why: its claim is the paint_bench example output, which the session does not reach and leaves unchanged
@answer n=17 entity=surface#14.2 v=no
why: its claim is the screenshot example output, which the session does not reach and leaves unchanged
@answer n=18 entity=surface#15.1 v=yes mode=add-to
why: the escher-session command line and its mcp role are the named agent-read surfaces, and the coverage role with F1 (one uncoloured JSON line) and F2 (a refusal holding no path or bytes) joins them
@answer n=19 entity=surface#15.2 v=yes mode=stale
why: the session reaches session_host.rs, and the usage line this entity quotes gains the coverage clause before its apps clause
@answer n=20 entity=surface#16.1 v=yes mode=stale
why: the coverage reader reads the UI graph at boot and after each replayed step, and cli_commands.rs, which this entity cites, is reached
@answer n=21 entity=surface#17.1 v=yes mode=stale
why: a kept check's actions steps now replay under test and the coverage role replays kept checks of any kind, and json.rs and cli_commands.rs, which this entity cites, are reached
@answer n=22 entity=surface#17.2 v=yes mode=stale
why: docs/capabilities.md, which this entity cites at lines 282-284, is reached by the three added limits and its lines may have moved
@answer n=23 entity=surface#18.1 v=yes mode=stale
why: docs/capabilities.md, which this entity cites for the limit at lines 293-296, is reached by the three added limits, though the unpaged graph reading holds
@answer n=24 entity=surface#19.1 v=yes mode=stale
why: docs/capabilities.md, which this entity cites for the limit at lines 297-299, is reached by the three added limits, though the unpaged actions reading holds
@answer n=25 entity=surface#20.1 v=no
why: its claim is WASM bundled fonts in examples/wasm_hello and dioxus-native config, neither reached by the session
