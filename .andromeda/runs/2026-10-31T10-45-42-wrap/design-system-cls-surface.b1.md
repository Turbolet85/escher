# Classified — design-system · surface.b1

## Answers
surface#1.1 · no · the sheet is still added once through `DocumentConfig::default()` in `packages/blitz-dom/src/config.rs`, which the session does not change, and the past `DioxusDocument::new` note is fixed history
surface#1.2 · yes · add-to · the default sheet gained the `multiple` select's rows (F1), a chosen-row pair of `#c8c8c8` background and `#000000` text with no token (F2), the closed `optgroup`'s transparent layout (F3) and the no-popup, unpainted-label, unread-`size` limits (F4), while `packages/blitz-dom/assets/default.css` lines 77-78, 84-86 and 155-159 stand
surface#1.3 · no · the initial theme is read in `packages/blitz-shell/src/window.rs:181-182`, which the session does not change
surface#1.4 · no · the viewport defaults sit in `packages/blitz-traits/src/shell.rs`, which the session does not change
surface#1.5 · no · `packages/blitz-vibey-script/src/runtime/bootstrap.rs` and `runtime/window.rs` carry the `innerWidth` claim and are unchanged, and the session's runtime change is the synthesis retirement in `runtime/events.rs`
surface#2.1 · no · the macOS tabstrip padding and height sit in `apps/browser`, which the session does not change
surface#2.2 · no · `packages/blitz-shell/src/event.rs:28-32` sits before the 46-line test module appended at `:128-172`, and `packages/blitz-shell/src/lib.rs:146-160` is unchanged, so the provider's methods hold
surface#3.1 · no · the `opacity: 0.8` composite sits in `examples/custom_widget.rs`, which the session does not change
surface#4.1 · no · the devtools overlay claim sits in `packages/blitz-traits/src/devtools.rs:11-17`, which the session does not change
surface#5.1 · no · the title claim sits in `packages/blitz-shell/src/window.rs:201-207` and `apps/browser`, both unchanged, and the session names no title change
surface#5.2 · no · the readme title and Ctrl/Cmd+T binding sit in `apps/readme`, which the session does not change, and the `pulldown-cmark` lock move touches neither
surface#6.1 · no · the macOS titlebar setup sits in `apps/browser/src/main.rs:82-89`, which the session does not change
surface#6.2 · no · the transparent example in `examples/transparent/src/app.rs` is unchanged
surface#6.3 · no · the alpha-mode config at `packages/dioxus-native/src/config.rs:66-74` is unchanged
surface#7.1 · no · the wasm32 canvas config at `packages/dioxus-native/src/config.rs:22-34` is unchanged
surface#7.2 · no · the `scrollbars` forward at `packages/dioxus-native/Cargo.toml:49` is unchanged, and the `Cargo.lock` move is `pulldown-cmark` alone
surface#8.1 · no · the mobile urlbar sizing sits in `apps/browser/assets/browser.css`, which the session does not change
surface#9.1 · no · the `mobile` frame class sits in `apps/browser/src/main.rs`, which the session does not change
surface#10.1 · no · the Android back button sits in `apps/browser/src/main.rs:140-145`, which the session does not change
surface#11.1 · no · the mobile capture default sits in `apps/browser/src/capture.rs:109-110`, which the session does not change
surface#12.1 · yes · stale · the free-disk step of 6 lines at `.github/workflows/ci.yml:311-316` pushes this citation from `:359-374` down to `:365-380`
surface#13.1 · no · the WPT result colours sit in `wpt/runner/`, which the session does not change
surface#13.2 · no · the WPT runner's bright-black labels sit in `wpt/runner/`, which the session does not change
surface#13.3 · no · the WPT runner's OSC 8 hyperlinks sit in `wpt/runner/`, which the session does not change
surface#14.1 · no · the `paint_bench` output lines sit in `examples/paint_bench.rs`, which the session does not change
