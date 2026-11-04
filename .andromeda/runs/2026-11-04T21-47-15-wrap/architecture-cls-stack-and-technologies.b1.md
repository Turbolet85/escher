@answer n=1 entity=stack-and-technologies#1.1 v=no
why: the Rust edition and rust-version pins live in Cargo.toml, which the session did not move
@answer n=2 entity=stack-and-technologies#1.2 v=no
why: the workspace version and in-repo crate pins live in Cargo.toml, which the session did not move
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: the Stylo pins live in Cargo.toml, unmoved, and no blitz-dom styling source changed
@answer n=4 entity=stack-and-technologies#1.4 v=no
why: the markup5ever, html5ever and xml5ever pins are Cargo.toml lines the session left alone
@answer n=5 entity=stack-and-technologies#1.5 v=no
why: taffy's resolved 0.15.0 still holds, since the lock change touched only pulldown-cmark
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: the parley, skrifa and ICU pins are unmoved, and the lock change touched only pulldown-cmark
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: the anyrender pins are unmoved and this entity names no zip advisory
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: the graphics crate pins are unmoved in Cargo.toml and no packages/ source changed
@answer n=9 entity=stack-and-technologies#1.9 v=no
why: the winit, keyboard-types and raw-window-handle pins are unmoved, and the lock change touched only pulldown-cmark
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: the accesskit pins are unmoved in Cargo.toml and the session left packages/ untouched
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: tokio's resolved 1.54.1 still holds, since the lock change touched only pulldown-cmark
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: the boa pins are unmoved in Cargo.toml and no script engine file changed
@answer n=13 entity=stack-and-technologies#1.13 v=no
why: the Dioxus requirement stays 0.7.3 with no Cargo.toml moved, and the new test only reads public API without changing the integration role
@answer n=14 entity=stack-and-technologies#1.14 v=no
why: the image, serde and png pins are unmoved and the lock change left the png and serde_json entries alone
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: the wasm-bindgen and web-sys pins are unmoved in Cargo.toml
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: the miscellaneous crate pins are unmoved and the session added no dependency
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: the rusqlite pins sit in apps/browser/persistence/Cargo.toml, which the session did not move
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: the browser app pins are unmoved and the lock's mimalloc entries stand
@answer n=19 entity=stack-and-technologies#1.19 v=yes mode=stale
why: the operator fix bumps the resolved pulldown-cmark from 0.13.5 to 0.13.6 in Cargo.lock, moving this entity's resolved version and its lock citation
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: the new target adds no dev-dependency and tests/blitz-tests/Cargo.toml is unmoved, so the dev-dependency list stands
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the dev host, flake and toolchain pins are unmoved and the session changed no toolchain
@answer n=22 entity=stack-and-technologies#1.22 v=no
why: the CI shape and the cargo-deny pin are unchanged, and the session moved no workflow
@answer n=23 entity=stack-and-technologies#1.23 v=no
why: deny.toml still holds per-ID ignores each with a reason with the added RUSTSEC-2026-0429 line, and the fmt, clippy and doc legs are unmoved
@answer n=24 entity=stack-and-technologies#2.1 v=no
why: the cognitive-complexity tool is a dev-host tool the session did not change
