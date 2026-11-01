### mocking-stubbing-discipline#2.1 · procedure · mocking · ## 8. Mocking & Stubbing Discipline · lines 40–40
- No font-metric test of `document.rs` skips: since 2026-10-14-quality-gates the two `hover_state_tests` assert their precondition through `#[track_caller] fn assert_text_has_size` — the container's height over 50 px — where they skipped with an `eprintln!` when text measured 0x0; the skip was the fixture's, not missing system fonts: built with the one-argument `qual_name!`, a no-namespace element the default sheet misses, both skipped under `--features system-fonts` too, and with the two-argument `qual_name!(…, html)` for `html`, `body` and both `div`s both assert and pass (`cargo test -p blitz-dom --locked --features system-fonts --lib hover_state_tests -- --nocapture`: exit 0, `2 passed`, no `skipping`) (packages/blitz-dom/src/document.rs:3634-3643; as measured at escher-0.1.0/chunks/2026-10-14-quality-gates/report.md); since 2026-10-18-upstream-sync-determinism-and-traces a third stands beside them, upstream's `hovering_below_content_reports_default_cursor`, built with the same two-argument `qual_name!` fixture, its assertion taking no precondition and no skip (`cargo test -p blitz-dom --locked --lib hover_state_tests`: `3 passed; 0 failed; 0 ignored`) (packages/blitz-dom/src/document.rs:3684-3692; as measured at escher-0.2.0/chunks/2026-10-18-upstream-sync-determinism-and-traces/report.md)
### mocking-stubbing-discipline#3.1 · absence · mocking · ## 8. Mocking & Stubbing Discipline · lines 46–46
- Mocking libraries in CI and build configuration · searched: `mock` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py
### mocking-stubbing-discipline#3.2 · absence · mocking · ## 8. Mocking & Stubbing Discipline · lines 47–47
- Mocking libraries in the apps · searched: `mockall|mock` (case-insensitive) over the 86 files of slice s04
### mocking-stubbing-discipline#3.3 · absence · mocking · ## 8. Mocking & Stubbing Discipline · lines 48–48
- Mocks or stubs in slice s03 · searched: `mock|Mock|stub|fake` over the 32 files of slice s03
### mocking-stubbing-discipline#3.4 · absence · mocking · ## 8. Mocking & Stubbing Discipline · lines 49–49
- Mocks or fixtures in slice s06 · searched: `mock|fixture` over the 17 files of slice s06
### mocking-stubbing-discipline#3.5 · absence · mocking · ## 8. Mocking & Stubbing Discipline · lines 50–50
- Mocks or fixtures in slice s07 · searched: `mock|fixture` over the 8 files of slice s07
### mocking-stubbing-discipline#3.6 · absence · mocking · ## 8. Mocking & Stubbing Discipline · lines 51–51
- Mocks, fakes or stubs in slice s08 · searched: `mock|fake|stub` over the 16 files of slice s08 (no match)
### mocking-stubbing-discipline#3.7 · absence · mocking · ## 8. Mocking & Stubbing Discipline · lines 52–52
- Mock or stub types in slice s09 · searched: `mock|Mock|stub|fake` over the 32 files of slice s09
### mocking-stubbing-discipline#3.8 · absence · mocking · ## 8. Mocking & Stubbing Discipline · lines 53–53
- Mocks, stubs or fakes in slice s11 · searched: `mock|stub|fake` over the 21 files of slice s11
