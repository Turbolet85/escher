@answer n=1 entity=stack-and-technologies#1.1 v=no
why: the session changes no Rust edition, rust-version or workspace manifest line
@answer n=2 entity=stack-and-technologies#1.2 v=no
why: the workspace version and in-repo crate pins live in Cargo.toml, which the session did not move
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: the session touches no Stylo dependency or CSS engine code
@answer n=4 entity=stack-and-technologies#1.4 v=no
why: html5ever, markup5ever and xml5ever are untouched by a render-buffer change in the Dioxus bridge
@answer n=5 entity=stack-and-technologies#1.5 v=no
why: taffy and its stylo_taffy bridge are outside the session's reached files
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: parley, skrifa, ICU4X and fonts are outside the session's reached files
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: no anyrender or renderer dependency moved and no rendering code is reached
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: color, peniko, kurbo, wgpu and usvg versions are unchanged and no graphics code is reached
@answer n=9 entity=stack-and-technologies#1.9 v=no
why: winit, keyboard-types and keyboard_compat stand as the session's untouched item 3 records
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: accesskit versions and the accessibility tree builder are unchanged per the session's untouched item 10
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: no networking dependency or fetch path is reached
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: Boa and blitz-vibey-script are not touched
@answer n=13 entity=stack-and-technologies#1.13 v=no
why: Dioxus stays 0.7.3 and the row cites only manifest lines the session did not move, while the new buffer changes the bridge's internals rather than its role
@answer n=14 entity=stack-and-technologies#1.14 v=no
why: image, png and media crate versions are unchanged and no media code is reached
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: wasm-bindgen, web-sys and the browser example builds are not reached
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: rayon, atomic_refcell and the other misc crates are unchanged and Stylo's thread pool is not reached
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: rusqlite and the browser persistence crate are not touched
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: browser app dependencies and its UI code are not reached
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: the rdme crate and its markdown dependencies are untouched
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: blitz-tests dev-dependencies and the integration test files this row cites are unchanged, and the row states no probe count
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the flake, the rust-version pin and the dev-host toolchain are unchanged
@answer n=22 entity=stack-and-technologies#1.22 v=no
why: no CI job, step, workflow or action pin changed per the session's untouched item 4
@answer n=23 entity=stack-and-technologies#1.23 v=no
why: the gates, clippy and coverage floors are unchanged, and the session's measured 86.71 % is not a figure this row states
@answer n=24 entity=stack-and-technologies#2.1 v=no
why: the dev-host reading and its ceiling of 15 are unchanged, and the row names no function or file the session reached
