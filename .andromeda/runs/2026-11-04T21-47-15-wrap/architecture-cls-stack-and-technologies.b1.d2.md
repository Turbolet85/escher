@answer n=1 entity=stack-and-technologies#1.1 v=no
why: edition and rust-version sit in manifests the session did not move, and the lock bump is pulldown-cmark alone.
@answer n=2 entity=stack-and-technologies#1.2 v=no
why: the workspace version and in-repo pins are Cargo.toml lines the session did not move.
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: the Stylo and selectors pins are Cargo.toml lines the session did not move, and the lock bump is pulldown-cmark alone.
@answer n=4 entity=stack-and-technologies#1.4 v=no
why: the markup5ever, html5ever and xml5ever pins are Cargo.toml lines the session did not move.
@answer n=5 entity=stack-and-technologies#1.5 v=no
why: taffy's resolved 0.15.0 and its Cargo.toml features hold, the lock bump being pulldown-cmark alone.
@answer n=6 entity=stack-and-technologies#1.6 v=no
why: the parley, skrifa and ICU pins are Cargo.toml and blitz-dom lines the session did not touch.
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: the anyrender pins are Cargo.toml lines the session did not move, and zip is not named in this entity.
@answer n=8 entity=stack-and-technologies#1.8 v=no
why: the color, peniko, kurbo, wgpu and usvg pins are Cargo.toml lines the session did not move.
@answer n=9 entity=stack-and-technologies#1.9 v=no
why: the winit and keyboard-types pins hold, and the lock's keyboard-types 0.8.3 is outside the pulldown-cmark bump.
@answer n=10 entity=stack-and-technologies#1.10 v=no
why: the accesskit versions are Cargo.toml and accesskit_xplat lines the session did not move.
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: tokio's resolved 1.54.1 and the reqwest pins hold, the lock bump being pulldown-cmark alone.
@answer n=12 entity=stack-and-technologies#1.12 v=no
why: the Boa pins are Cargo.toml lines and blitz-vibey-script is unchanged by the session.
@answer n=13 entity=stack-and-technologies#1.13 v=no
why: the Dioxus 0.7.3 requirement and its Cargo.toml citations stay, the session adding a test target and moving no pin.
@answer n=14 entity=stack-and-technologies#1.14 v=no
why: the image, png and serde_json resolutions hold, the lock bump being pulldown-cmark alone.
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: the wasm-bindgen and web-sys pins are Cargo.toml lines the session did not move.
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: the parallelism and misc crate pins are Cargo.toml lines the session did not move.
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: the rusqlite pins sit in the browser persistence manifest, which the session did not touch.
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: the mimalloc lock 0.1.52 holds, the lock bump being pulldown-cmark alone, and the browser extras are unchanged.
@answer n=19 entity=stack-and-technologies#1.19 v=yes mode=contradict
why: the lock bump moved pulldown-cmark from the resolved 0.13.5 to 0.13.6 fixing RUSTSEC-2026-0433, so the resolution claim and its Cargo.lock:6505 citation no longer hold.
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: the session adds no blitz-tests dev-dependency or manifest line, the probe uses dioxus already listed, and the role names no count.
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the flake pin, the rust-bin 1.91.0 pin and the dev-host toolchain are not moved by a lock bump or a new test file.
@answer n=22 entity=stack-and-technologies#1.22 v=no
why: no workflow, job, leg, script or CI tool version moved, and this entity makes no audit-status claim.
@answer n=23 entity=stack-and-technologies#1.23 v=no
why: deny.toml is named only generically with per-ID reasons, and the report gives the added zip ignore its reason, so every per-ID ignore still carries one.
@answer n=24 entity=stack-and-technologies#2.1 v=no
why: the rust-code-analysis-cli host tool and its ceiling of 15 are not touched by the new test file or the lock bump.
