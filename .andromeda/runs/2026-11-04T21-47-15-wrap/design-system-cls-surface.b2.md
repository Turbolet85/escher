@answer n=1 entity=surface#6.1 v=no
why: the macOS titlebar lives in apps/browser/src/main.rs, which the session did not reach, and the lock move touches no windowing crate
@answer n=2 entity=surface#6.2 v=no
why: examples/transparent/src/app.rs is unchanged and the window and shell close claim does not depend on the moved lock lines
@answer n=3 entity=surface#6.3 v=no
why: packages/dioxus-native/src/config.rs is unchanged and Dioxus stays at the 0.7.3 requirement, so the alpha mode claim holds
@answer n=4 entity=surface#7.1 v=no
why: the wasm32 canvas code in packages/dioxus-native/src/config.rs is unchanged and no lock line for that surface moved
@answer n=5 entity=surface#7.2 v=no
why: packages/dioxus-native/Cargo.toml is not reached and the scrollbars feature forward it names is unchanged
@answer n=6 entity=surface#7.3 v=no
why: the cited select_popup_painted.rs and blitz-paint select_list.rs are unchanged and the new probe target does not alter the painted thumb claim
@answer n=7 entity=surface#8.1 v=no
why: apps/browser/assets/browser.css is unchanged and the mobile urlbar sizing does not depend on the moved lock lines
@answer n=8 entity=surface#9.1 v=no
why: the mobile frame class in apps/browser/src/main.rs and apps/browser/assets/browser.css are unchanged
@answer n=9 entity=surface#10.1 v=no
why: the Android back navigation in apps/browser/src/main.rs is unchanged and no lock line for it moved
@answer n=10 entity=surface#11.1 v=no
why: apps/browser/src/capture.rs is unchanged and zip 2.4.2 reaches only the deserialize path, so the default file name claim holds
@answer n=11 entity=surface#12.1 v=no
why: .github/workflows/ci.yml is unchanged and the session adds no CI target, so the iOS and Android not tested in CI claim holds
@answer n=12 entity=surface#13.1 v=no
why: wpt/runner/src/main.rs is unchanged and the lock move touches only the pulldown-cmark version and its checksum
@answer n=13 entity=surface#13.2 v=no
why: the bright black print lines in wpt/runner/src/main.rs are unchanged
@answer n=14 entity=surface#13.3 v=no
why: the OSC 8 hyperlink code in wpt/runner/src/main.rs is unchanged and the supports_hyperlinks dependency is not in the lock move
@answer n=15 entity=surface#14.1 v=no
why: examples/paint_bench.rs is unchanged and the lines it prints are not moved
@answer n=16 entity=surface#14.2 v=no
why: examples/screenshot.rs is unchanged and the lines it prints are not moved
@answer n=17 entity=surface#15.1 v=no
why: the escher-session CLI surface is unchanged, with VERBS twelve, CAUSES eight and SessionError fifteen unmoved
@answer n=18 entity=surface#15.2 v=no
why: examples/seven_guis/src/session_host.rs is unchanged and the six task list stands
@answer n=19 entity=surface#16.1 v=no
why: the DejaVu font registration in examples/wasm_hello/src/lib.rs and the on-WASM font context in config.rs are unchanged
@answer n=20 entity=surface#17.1 v=no
why: examples/wasm_hello/index.html and examples/wasm_hello/src/lib.rs canvas code are unchanged
