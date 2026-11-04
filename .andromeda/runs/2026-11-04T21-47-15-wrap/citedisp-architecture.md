@disposition n=1
row: held fct-rekexd3u examples/screenshot.rs:41-51 — refused at 2026-10-10T01-56-50-wrap, not touched since «let file_cont…»
disposition: holds — stands (lines 41-51 still hold the example's `.unwrap()` chain on the file and reqwest arms)
@disposition n=2
row: stretched fct-b25i63xp deny.toml:1-29 → 1-30 — changed inside
disposition: holds (the root deny.toml still configures the cargo-deny advisories gate; the change inside is one added RUSTSEC-2026-0429 ignore)
@disposition n=3
row: held fct-m2wugvlj .github/workflows/publish-browser.yml:122-126 — refused at 2026-10-27T10-45-32-wrap, not touched since «- name: Ins…»
disposition: holds — stands (lines 122-126 are still the Install Dioxus CLI step that provides the dx the bundle step runs)
@disposition n=4
row: changed fct-kjoxjobc Cargo.lock:6505 → ? — rewritten «version = "0.13.5"»
disposition: claim false — raised at Validate (Cargo.lock:6505 now resolves pulldown-cmark 0.13.6 after 9b3b29714, not the stated 0.13.5)
