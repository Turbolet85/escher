# facts-s01 · HEAD 0f60502ea724ef703b34220b3847bb3483f70b5f · 24 files

## architecture §Design Philosophy
- The root manifest describes its crate as "Top level crate for Blitz" with keywords dom, ui, gui, react, wasm (Cargo.toml:243-245)
- The root `[package]` is commented as a "virtual package" not meant to be published, existing so `cargo run --example XYZ` works (Cargo.toml:236-247)
- The CI file states Blitz only guarantees "latest stable", keeps an MSRV check to advertise its MSRV, and makes an effort not to increase MSRV in patch versions (.github/workflows/ci.yml:20-22)
- A captured GitHub profile fixture shows the repository description "A radically modular HTML/CSS rendering engine" (examples/assets/github_profile_reduced2.html:93-95)

## architecture §Stack and Technologies
- Rust Cargo workspace, resolver "2", edition "2024", rust-version "1.91.0" (Cargo.toml:1; Cargo.toml:31; Cargo.toml:39-40)
- Workspace package version "0.3.0-beta.2", license "MIT OR Apache-2.0", category "gui" (Cargo.toml:33-38)
- In-repo crates pinned to `=0.3.0-beta.2`: blitz, blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy; dioxus-native and dioxus-native-dom at "0.7.0"; debug_timer "0.1.2"; accesskit_xplat "0.2" (Cargo.toml:43-58)
- CSS engine from Servo: stylo, stylo_traits, stylo_atoms, stylo_static_prefs, stylo_dom at "0.22.0"; selectors "0.41.0"; cssparser "0.38.0" (Cargo.toml:60-67)
- HTML/XML parsing: markup5ever "0.40.0", html5ever "0.40.1", xml5ever "0.40.0" (Cargo.toml:69-72)
- Other Servo crates: euclid "0.22", atomic_refcell "0.1.13", app_units "0.7.5", smallvec "1", thin-vec "0.2" (Cargo.toml:74-79)
- Dioxus crates at "0.7.3" (dioxus, dioxus-core, dioxus-html, dioxus-hooks, dioxus-signals, dioxus-stores, dioxus-asset-resolver, manganis, dioxus-document, dioxus-history, dioxus-devtools, dioxus-cli-config, dioxus-core-macro) (Cargo.toml:81-98)
- Layout: taffy from git `https://github.com/DioxusLabs/taffy` at a pinned rev with features std, flexbox, flexbox_balance, grid, block_layout, content_size, calc, detailed_layout_info (Cargo.toml:100-110)
- Text: parley from git `https://github.com/linebender/parley` at a pinned rev; skrifa "0.44" (Cargo.toml:111-114)
- Rendering: anyrender "0.14.0", anyrender_serialize "0.8.0", anyrender_skia "0.12.0", anyrender_vello "0.15.0", anyrender_vello_cpu "0.18.0", anyrender_vello_hybrid "0.11.0", anyrender_svg "0.15.0", wgpu_context "0.10.0" (Cargo.toml:116-124)
- Graphics: color "0.3", peniko "0.6.0", kurbo "0.13.1", wgpu "30", usvg "0.48.1", svgtypes "0.16.1" (Cargo.toml:126-133)
- Windowing and input: raw-window-handle "0.6.0", winit "=0.31.0-beta.3", accesskit "0.25", arboard "3.4.1", rfd "0.17.1", keyboard-types "0.7", cursor-icon "1" (Cargo.toml:135-142)
- IO and networking: url "2.5.0", http "1.1.0", data-url "0.3.1", tokio "1.42", reqwest "0.13", reqwest-middleware "0.5.1", http-cache-reqwest and http-cache "=1.0.0-alpha.6" (Cargo.toml:144-152)
- Media: image "0.25.6", wuff "0.2", html-escape "0.2.13", percent-encoding "2.3.1", png "0.18", serde "1" (Cargo.toml:154-160)
- WASM: wasm-bindgen "0.2", wasm-bindgen-futures "0.4", tracing-wasm "0.2.1", web-sys "0.3.98", web-time "1", console_error_panic_hook "0.1" (Cargo.toml:166-172)
- JavaScript engine Boa: boa_engine, boa_runtime, boa_gc "0.22" (Cargo.toml:174-177)
- Misc: rustc-hash, bytes, slotmap, tracing "0.1.40", tracing-subscriber "0.3", futures-util, futures-intrusive, pollster, smol_str, bitflags, bytemuck, rayon "1", test-that "0.5.2", thread_local (Cargo.toml:179-193)
- Root dev-dependencies add env_logger "0.11", vello "0.11", vello_cpu/vello_gpu/vello_common "0.3" (Cargo.toml:282-287)
- Nix flake inputs: nixpkgs-unstable, flake-parts, nix-systems/default, oxalica/rust-overlay, ipetkov/crane (flake.nix:2-9)
- Nix toolchain is rust-bin stable "1.90.0" with rust-src, rust-analyzer, clippy, commented "Keep in sync with `rust-version` in Cargo.toml" (flake.nix:30-37)
- Build inputs: openssl (via reqwest in blitz-net), fontconfig (via parley/fontique) on Linux, apple-sdk and libiconv on Darwin; pkg-config and python3 native, python3 "required at build time to generate code for `stylo`" (flake.nix:55-73)
- CI helper scripts are Python 3 (.github/scripts/wpt_diff_to_pr.py:1; .github/scripts/test_wpt_diff_to_pr.py:1)

## architecture §Established Decisions
- markup5ever, html5ever and xml5ever versions are commented "needs to match stylo web_atoms version" (Cargo.toml:70-72)
- skrifa is commented "Should match parley and vello versions"; svgtypes "Should match usvg's version" (Cargo.toml:112-114; Cargo.toml:133)
- taffy and parley are consumed as git dependencies pinned by `rev` (Cargo.toml:101; Cargo.toml:111)
- winit is pinned exactly to a beta, `=0.31.0-beta.3` (Cargo.toml:137)
- Commented-out `[patch.crates-io]` blocks point anyrender, vello, stylo, taffy and parley at sibling local checkouts (Cargo.toml:292-321)
- Cargo profiles: `profile` (release + debug), `production` (opt-level 3, lto, codegen-units 1, strip, no incremental), `p2` (opt-level 2), `small` (opt-level "s", panic abort), `small-panic` (unwind), `tiny` (opt-level "z") with fearless_simd, vello_cpu, taffy, harfrust, skrifa kept at opt-level 3 (Cargo.toml:195-234)
- The MSRV job runs only `cargo build`, not `cargo test`, to avoid requiring dev-dependencies to build with the MSRV (.github/workflows/ci.yml:24-25)
- publish-browser.yml states main is always a prepatch until 1.0 and the version in git is one minor bump ahead of the actual release (.github/workflows/publish-browser.yml:21-26)
- The flake builds deps and crate in one derivation (`cargoArtifacts = null`), commenting that the workspace root is a virtual manifest with no root `[package]` (flake.nix:96-99); the root manifest does declare a `[package]` named blitz-examples (Cargo.toml:238-239)
- The flake disables checks in package builds, "to avoid building deps for them" (flake.nix:103)
- `cross` is installed from a git rev because the latest release does not work with recent Rust when targeting Android (.github/workflows/ci.yml:187-191)

## architecture §Conventions
- Formatting is enforced with `cargo fmt --all --check` (.github/workflows/ci.yml:87-96)
- Lints are enforced with `cargo clippy --workspace -- -D warnings` (.github/workflows/ci.yml:98-109)
- Rustdoc warnings are errors via `RUSTDOCFLAGS: "-D warnings"` (.github/workflows/ci.yml:14-15; .github/workflows/wpt.yml:13-14)
- Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }` (Cargo.toml:42; Cargo.toml:254-290)
- In-repo crates are declared with `default-features = false` at the workspace level (Cargo.toml:45-56)
- Dependency groups in the workspace manifest are separated by comment headers (Servo, HTML5ever, DioxusLabs, Taffy + Parley + Fontations, AnyRender, Windowing & Input, IO & Networking, Media & Decoding, WASM, Boa) (Cargo.toml:60-179)
- The CI Python test file uses `unittest` with one `TestCase` class per function under test (.github/scripts/test_wpt_diff_to_pr.py:4; .github/scripts/test_wpt_diff_to_pr.py:51-88)

## architecture §Standard Contracts
- wpt_diff_to_pr.py consumes `wpt diff --format json` entries with keys `test`, `kind` ("added" / "removed" / changed), `status`, `counts`, `before`, `after`, `counts_before`, `counts_after` (.github/scripts/wpt_diff_to_pr.py:2; .github/scripts/wpt_diff_to_pr.py:24-42)
- The rendered PR section is delimited by `<!-- wpt-results-start -->` and `<!-- wpt-results-end -->`; re-runs replace the section between markers, else append (.github/scripts/wpt_diff_to_pr.py:4-5; .github/scripts/wpt_diff_to_pr.py:14-15; .github/scripts/wpt_diff_to_pr.py:164-172)
- The script's CLI takes `diff_file`, `--repo` (default `GITHUB_REPOSITORY`), `--pr` (default `PR_NUMBER`), `--run-url` (default `RUN_URL`), `--dry-run` (.github/scripts/wpt_diff_to_pr.py:186-191)
- Statuses "PASS" and "OK" count as passing; the diff listing is capped at 400 lines (.github/scripts/wpt_diff_to_pr.py:17-18; .github/scripts/wpt_diff_to_pr.py:143-151)
- The script appends its section to `GITHUB_STEP_SUMMARY` when set, and PATCHes the PR body through `gh api` otherwise prints in dry-run (.github/scripts/wpt_diff_to_pr.py:199-214)
- WPT workflow artifacts: `wpt-report.json.zst` and `wpt-diff` (wptdiff.txt, wptdiff.json); the post-results workflow downloads `wpt-diff` by run id (.github/workflows/wpt.yml:59-63; .github/workflows/wpt.yml:86-93; .github/workflows/wpt-post-results.yml:22-27)
- A `repository-dispatch` with event-type `update-results` is sent to `DioxusLabs/blitz-wpt-results` after WPT on main (.github/workflows/wpt.yml:106-118)
- Cargo features on the root crate: `log-times` = `log-frame-times` + `log-phase-times`, forwarding to dioxus-native (Cargo.toml:249-252)

## architecture §Occupied Resources
- WPT results are published to GitHub Pages and the main-branch report is fetched from `https://dioxuslabs.github.io/blitz/wptreport.json` (.github/workflows/wpt.yml:75-76; .github/workflows/wpt.yml:94-105)
- `sites/gh-pages` is the Pages staging directory; `sites` is excluded from the Cargo workspace (.github/workflows/wpt.yml:64-68; Cargo.toml:30)
- WPT tests are cloned into `./wpt/tests` (`WPT_DIR`) and output goes to `./wpt/output` (.github/workflows/wpt.yml:16; .github/workflows/wpt.yml:51-58)
- GitHub environments used: "Signed Builds" and "WPT" (.github/workflows/publish-browser.yml:40; .github/workflows/wpt.yml:111)
- The WPT job runs on runner label `warp-ubuntu-latest-arm64-16x` with rust-cache provider "warpbuild" (.github/workflows/wpt.yml:26; .github/workflows/wpt.yml:31-37)
- Bundle outputs live under `./target/dx/blitz/...` per platform (.github/workflows/publish-browser.yml:51-88)
- observed absent — network ports or local hosts · searched: `localhost|port` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix

## architecture §Infrastructure Patterns
- CI runs on pull_request and on push to `main` and `v0.*`, with per-ref concurrency and cancel-in-progress (.github/workflows/ci.yml:3-12)
- CI jobs: MSRV build (Rust 1.91), default build, default test, counter build, wasm examples build, rustfmt, clippy, CI-script tests, docs, cross-platform matrix (.github/workflows/ci.yml:26-221)
- Ubuntu jobs install `libfontconfig1-dev` and rewrite `opt-level = 2` to `0` in Cargo.toml before building (.github/workflows/ci.yml:34-35; .github/workflows/ci.yml:44-45)
- The matrix tests on windows, macos and linux (`--all --tests`) and only builds for ios and android, android through `cross` (.github/workflows/ci.yml:126-174)
- Cross.toml pre-build adds `$CROSS_DEB_ARCH` and installs `python3` for that arch (Cross.toml:1-6)
- `Swatinem/rust-cache@v2` saves only on `refs/heads/main` (.github/workflows/ci.yml:201-206; .github/workflows/publish-browser.yml:99-103; .github/workflows/wpt.yml:31-37)
- wasm_hello is described as a standalone workspace; seven_guis and todomvc build as wasm cdylib with `--no-default-features --features hybrid` (.github/workflows/ci.yml:68-85)
- Publish Browser runs on push to `main` and `ci-test/*` and on workflow_dispatch, matrix of Windows x86_64/aarch64, macOS x86_64/aarch64, Linux x86_64/aarch64, Android aarch64 (.github/workflows/publish-browser.yml:2-7; .github/workflows/publish-browser.yml:42-88)
- Publish uses Rust "1.96.1", dioxus-cli "0.7.8", and `dx bundle --package browser --release --profile production --locked` (.github/workflows/publish-browser.yml:93-97; .github/workflows/publish-browser.yml:121-125; .github/workflows/publish-browser.yml:153-154)
- Publish limits `CARGO_BUILD_JOBS: 2` "to debug possible OOM" (.github/workflows/publish-browser.yml:31-32)
- Bundles are uploaded unzipped with `actions/upload-artifact@v7` (.github/workflows/publish-browser.yml:175-181)
- WPT job: 15-minute timeout, builds and runs `cargo run -rp wpt css svg`, compresses with zstd, computes scores with `wpt` cli "0.0.14", diffs against main, deploys Pages on main (.github/workflows/wpt.yml:27; .github/workflows/wpt.yml:53-105)
- Nix flake exposes `packages.browser` (binary `blitz`) as default and a `blitz-dev` devShell (flake.nix:117-138)
- Linux packages are wrapped with `LD_LIBRARY_PATH` for dlopen'd runtime libs (wayland, libxkbcommon, libGL, vulkan-loader, X11 libs) for winit + wgpu (flake.nix:40-53; flake.nix:105-112)

## architecture §Cross-cutting Patterns
- Frame and phase time logging are opt-in Cargo features (`log-frame-times`, `log-phase-times`) (Cargo.toml:249-252)
- `tracing` and `tracing-subscriber` are workspace dependencies (Cargo.toml:183-184)
- out of slice — runtime cross-cutting code (error types, logging setup, config loading) lives in crates not in this slice

## architecture §Project Intent
- Homepage and repository are `https://github.com/dioxuslabs/blitz` (Cargo.toml:36-37)
- The flake names `apps/browser` "The example browser app", whose binary is `blitz` (flake.nix:117-121)
- An example page states it shows "the built-in cursor styles supported by blitz" (examples/assets/cursor.html:54)

## architecture §Existing Scopes
- Workspace members: packages accesskit_xplat, debug_timer, blitz-traits, blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz, stylo_taffy, dioxus-native, dioxus-native-dom, blitz-test-harness; apps browser, browser/persistence, readme, bump; wpt/runner; tests/blitz-tests; examples counter, transparent, seven_guis, todomvc, wgpu_texture, wasm_hello (Cargo.toml:2-29)
- examples/assets holds standalone HTML pages exercising CSS features: clip-path shapes, border-style values, cursor styles, filters, animations/transitions, floats, border radii (examples/assets/clip-path.html:193; examples/assets/border-styles.html:100; examples/assets/cursor.html:53; examples/assets/filters.html:24-83; examples/assets/animated_layout.html:7; examples/assets/float-width.html:9-18; examples/assets/border.html:12-61)
- examples/assets also holds reduced captures of real pages (Google, BBC News, GitHub profile, docs.rs header) (examples/assets/bottom_only.html:6; examples/assets/bbc_reduced.html:6; examples/assets/github_profile_reduced2.html:39; examples/assets/docsrs_header.html:4-11)

## security-plan §Threat Model Summary
- The post-results workflow runs only for successful `pull_request`-triggered WPT runs and checks out scripts from the default branch, a step named "Checkout trusted scripts" (.github/workflows/wpt-post-results.yml:3-6; .github/workflows/wpt-post-results.yml:15-21)
- out of slice — application-level threat model

## security-plan §Authentication & Authorization
- wpt-post-results.yml grants `pull-requests: write`, `actions: read`, `contents: read` (.github/workflows/wpt-post-results.yml:8-11)
- wpt.yml grants `contents: read`, `pages: write`, `id-token: write` (.github/workflows/wpt.yml:18-21)
- The publish job grants `contents: write` and uses environment "Signed Builds" only on main or `ci-test` branches (.github/workflows/publish-browser.yml:37-40)
- observed absent — a workflow-level permissions block in ci.yml · searched: `permissions` over .github/workflows/ci.yml
- out of slice — application authentication/authorization

## security-plan §Input Validation
- The PR lookup selects only open PRs whose head sha equals the workflow run's head sha, and skips when none is found (.github/workflows/wpt-post-results.yml:36-42)
- `splice` replaces only when both markers exist with end after start (.github/scripts/wpt_diff_to_pr.py:164-172)
- out of slice — HTML/CSS/URL input handling in the engine crates

## security-plan §Data Protection
- out of slice — no data storage or encryption code in this slice

## security-plan §API Security
- GitHub API calls go through the `gh api` CLI with `GH_TOKEN` from the workflow `token` secret (.github/scripts/wpt_diff_to_pr.py:175-182; .github/workflows/wpt-post-results.yml:29-49)
- out of slice — no served API in this slice

## security-plan §Dependency Security
- observed absent — dependency audit tooling · searched: `audit|deny|dependabot|cargo-vet` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py
- Git dependencies are pinned by commit rev (Cargo.toml:101; Cargo.toml:111)
- Builds pass `--locked` in the flake and in `dx bundle` (flake.nix:100; .github/workflows/publish-browser.yml:154)
- `cross` is installed from a pinned git rev; dioxus-cli is pinned to 0.7.8; wpt cli to 0.0.14 (.github/workflows/ci.yml:191; .github/workflows/publish-browser.yml:125; .github/workflows/wpt.yml:70)
- `awalsh128/cache-apt-pkgs-action` is referenced at `@latest` (.github/workflows/ci.yml:210; .github/workflows/publish-browser.yml:147; .github/workflows/wpt.yml:47)

## security-plan §Secret Management
- A macOS signing key is written from a secret to the path in `vars.APPLE_API_KEY_PATH` and removed in an `always()` step (.github/workflows/publish-browser.yml:105-107; .github/workflows/publish-browser.yml:167-169)
- An Android keystore is base64-decoded from a secret to `apps/browser/keystore.jks`, `password` fields (`jks_password`, `key_password`) are appended to Dioxus.toml from a secret, and the keystore is removed in an `always()` step (.github/workflows/publish-browser.yml:109-119; .github/workflows/publish-browser.yml:171-173)
- Apple certificate and certificate `password` are passed as env from secrets; API issuer, key id and key path come from `vars` (.github/workflows/publish-browser.yml:155-160)
- A `token` field is read from secret `WPT_GITHUB_TOKEN` for the cross-repo dispatch (.github/workflows/wpt.yml:113-116)
- The workflow `token` secret `GITHUB_TOKEN` is passed as `github-token` and `GH_TOKEN` (.github/workflows/wpt-post-results.yml:28; .github/workflows/wpt-post-results.yml:32; .github/workflows/wpt-post-results.yml:46)

## security-plan §Error Handling
- `gh_api` runs `subprocess.run(..., check=True)`, raising on non-zero exit (.github/scripts/wpt_diff_to_pr.py:175-182)
- Matrix jobs set `fail-fast: false` (.github/workflows/ci.yml:131-132; .github/workflows/publish-browser.yml:42-43)
- out of slice — engine error handling

## security-plan §Logging & Monitoring
- Publish sets `CARGO_LOG: info` and runs `dx bundle --verbose --trace` (.github/workflows/publish-browser.yml:33; .github/workflows/publish-browser.yml:154)
- out of slice — runtime security logging

## design-system §Color Palette
- An example fixture defines a 25-colour block palette commented "curated harmonious palette" (e.g. #e63946, #2a9d8f, #264653, #8338ec, #3a86ff) (examples/assets/clip-path.html:35-60)
- An example fixture uses border colours #d6336c, #1c7ed6, #2f9e44, #f59f00 and dark #1a1a1a on background #fffbe6 / #f5f5f5 (examples/assets/border-styles.html:7; examples/assets/border-styles.html:27-29; examples/assets/border-styles.html:77; examples/assets/border-styles.html:82)
- A reduced GitHub fixture sets custom property `--borderColor-default: green` and consumes it via `var()` (examples/assets/github_profile_reduced.html:6-13)
- The BBC fixture uses relative colour syntax `color(from #202224 srgb r g b / …)` (examples/assets/bbc_reduced.html:13-16)
- out of slice — the project's own UI colour tokens

## design-system §Typography
- Example fixtures use `font-family: sans-serif` for body and `monospace` for code labels (examples/assets/cursor.html:6; examples/assets/cursor.html:46; examples/assets/border-styles.html:6; examples/assets/border-styles.html:38)
- The Google fixture sets 14px `arial,sans-serif` with colour #202124 (examples/assets/bottom_only.html:8)
- out of slice — the project's own type scale

## design-system §Spacing
- Example fixtures use grid/flex gaps of 24px and 8px and 24px body padding (examples/assets/border-styles.html:9; examples/assets/border-styles.html:17; examples/assets/cursor.html:9; examples/assets/cursor.html:23)
- out of slice — the project's own spacing scale

## design-system §Depth Strategy
- A fixture exercises `filter: drop-shadow(...)` and `backdrop-filter: blur(10px)` (examples/assets/filters.html:37; examples/assets/filters.html:80)
- A captured GitHub fixture uses `z-index: 2147483647` on a fixed progress bar (examples/assets/github_profile_reduced2.html:3-10)
- out of slice — the project's own elevation scale

## design-system §Border Radius
- Fixtures exercise uniform, percentage, per-corner and elliptical radii (examples/assets/border.html:16-20; examples/assets/border-styles.html:65-73; examples/assets/cursor.html:34)
- out of slice — the project's own radius scale

## design-system §Motion
- Fixtures exercise `@keyframes` animations (big-small 2s alternate, gradient-animation 20s, spin 4s linear) (examples/assets/animated_layout.html:21; examples/assets/animated_layout.html:28-43; examples/assets/animation.html:32-38)
- Fixtures exercise transitions on transform and filter with `:hover`/`:active` nesting (examples/assets/animation.html:17-21; examples/assets/animation.html:46-51; examples/assets/filters.html:16-17)
- The BBC fixture contains `prefers-reduced-motion` media queries (examples/assets/bbc_reduced.html:10; examples/assets/bbc_reduced.html:11; examples/assets/bbc_reduced.html:17)

## design-system §Iconography
- A fixture enumerates 36 CSS cursor values as "the built-in cursor styles supported by blitz" (examples/assets/cursor.html:54-91)
- The captured GitHub fixture uses inline SVG octicons (examples/assets/github_profile_reduced2.html:65-67)
- out of slice — the project's own icon set

## design-system §Surface: desktop-native
- The flake's default package is the browser app whose binary is `blitz`, wrapped with winit/wgpu runtime libraries on Linux (flake.nix:40-53; flake.nix:117-121)
- Desktop bundles are produced for Windows (NSIS .exe), macOS (.dmg) and Linux (.AppImage) on x86_64 and aarch64 (.github/workflows/publish-browser.yml:46-81)
- out of slice — the desktop UI's own visual design

## design-system §Surface: mobile-native
- An Android aarch64 APK is bundled with `--android --package-types apk --no-default-features --features android-defaults` (.github/workflows/publish-browser.yml:82-88)
- CI builds (does not test) for `aarch64-apple-ios` and `aarch64-linux-android` (.github/workflows/ci.yml:159-174)
- out of slice — the mobile UI's own visual design

## layout-templates §Surface: desktop-native
- out of slice — the browser app's window/page layout code is not in this slice

## layout-templates §Surface: mobile-native
- out of slice — mobile layout code is not in this slice

## test-plan §Test Scope Summary
- `cargo test --workspace` runs on ubuntu with default features (.github/workflows/ci.yml:48-56)
- The matrix runs `test --all --tests` on windows, macos, linux and `build --all` on ios and android (.github/workflows/ci.yml:135-174; .github/workflows/ci.yml:219-221)
- Web Platform Tests run for `css` and `svg` (.github/workflows/wpt.yml:55-56)
- CI Python scripts are unit-tested with `python3 -m unittest discover -s .github/scripts` (.github/workflows/ci.yml:111-116)

## test-plan §Test Strategy
- MSRV is verified by build only, not test (.github/workflows/ci.yml:20-25)
- WPT results are diffed against the main-branch report and summarised per test as gained/lost subtests (.github/workflows/wpt.yml:75-85; .github/scripts/wpt_diff_to_pr.py:116-161)

## test-plan §Test Harness Contract
- The WPT runner is invoked as `cargo build -rp wpt` then `cargo run -rp wpt css svg`, producing `./wpt/output/wptreport.json` (.github/workflows/wpt.yml:53-58)
- The `wpt` cli provides `calc-scores` and `diff --format json` (.github/workflows/wpt.yml:69-80)
- The test file states it runs with `python3 -m unittest discover .github/scripts` (.github/scripts/test_wpt_diff_to_pr.py:2)

## test-plan §Unit Test Strategy
- test_wpt_diff_to_pr.py covers `format_lines` ordering/alignment/markers, `render` headline counts and empty diff, and `splice` idempotent replacement (.github/scripts/test_wpt_diff_to_pr.py:51-88)
- out of slice — Rust unit tests in the crates

## test-plan §Integration Test Strategy
- `tests/blitz-tests` is a workspace member (Cargo.toml:22)
- out of slice — its contents

## test-plan §E2E Test Strategy
- WPT reftests are run with a Thai font installed because some reftests depend on Thai glyph widths (.github/workflows/wpt.yml:43-50)
- Successful PR WPT runs have their results posted into the PR description (.github/workflows/wpt-post-results.yml:43-49)

## test-plan §Test Data & Fixtures
- WPT tests are cloned at the commit in `./wpt/WPT_COMMIT` (.github/workflows/wpt.yml:51-52)
- The script test uses an inline `ENTRIES` list covering changed, added and removed entries (.github/scripts/test_wpt_diff_to_pr.py:8-48)
- examples/assets holds static HTML pages for rendering CSS features and reduced real-world pages (examples/assets/clip-path.html:193; examples/assets/bbc_reduced.html:6; examples/assets/bottom_only.html:6)

## test-plan §Mocking & Stubbing Discipline
- The script's `--dry-run` prints instead of calling the GitHub API, and tests call `render` with `run_url=None` (.github/scripts/wpt_diff_to_pr.py:204-206; .github/scripts/test_wpt_diff_to_pr.py:67)
- observed absent — mocking libraries · searched: `mock` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py

## test-plan §CI Integration
- Tests run in GitHub Actions on PRs and pushes to main/v0.* (.github/workflows/ci.yml:3-8)
- WPT runs on PRs and pushes to main, publishes a step summary on PRs, and uploads the diff artifact (.github/workflows/wpt.yml:3-7; .github/workflows/wpt.yml:81-93)
- observed absent — coverage tooling · searched: `coverage|tarpaulin|llvm-cov|codecov` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py

## obs-plan §Obs Scope Summary
- Workspace dependencies include tracing, tracing-subscriber, tracing-wasm and console_error_panic_hook; the root dev-dependencies add env_logger (Cargo.toml:169; Cargo.toml:172; Cargo.toml:183-184; Cargo.toml:282)
- `packages/debug_timer` is a workspace member (Cargo.toml:4; Cargo.toml:55)

## obs-plan §Telemetry Strategy
- observed absent — telemetry backends · searched: `sentry|opentelemetry|otel|prometheus|metrics` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py

## obs-plan §Observability Harness Contract
- out of slice — no observability harness code in this slice

## obs-plan §Span / Trace Coverage
- out of slice — span instrumentation lives in crates not in this slice

## obs-plan §Metric Coverage
- Frame and phase timing logs are feature-gated (`log-frame-times`, `log-phase-times`, umbrella `log-times`) (Cargo.toml:249-252)
- WPT scores are computed into `wptscores.json` and published to Pages (.github/workflows/wpt.yml:71-74)

## obs-plan §Log Coverage
- Publish builds log at `CARGO_LOG: info` with `--verbose --trace` (.github/workflows/publish-browser.yml:33; .github/workflows/publish-browser.yml:154)
- out of slice — runtime log statements

## obs-plan §Error Capture & Reporting
- `console_error_panic_hook` is a workspace dependency (Cargo.toml:172)
- out of slice — panic/error capture code

## obs-plan §PII Scrubbing & Compliance
- out of slice — no PII handling in this slice

## obs-plan §CI Integration
- The WPT report and scores are archived to GitHub Pages and dispatched to `DioxusLabs/blitz-wpt-results` on main (.github/workflows/wpt.yml:94-118)

## a11y-plan §A11y Scope Summary
- accesskit "0.25" and the in-repo `accesskit_xplat` package are workspace dependencies/members (Cargo.toml:3; Cargo.toml:56; Cargo.toml:138)

## a11y-plan §A11y Strategy
- out of slice — accessibility tree code lives in crates not in this slice

## a11y-plan §A11y Assertion Harness Contract
- out of slice — no a11y assertion harness in this slice

## a11y-plan §ARIA Patterns & Roles
- The captured GitHub fixtures carry `aria-hidden`, `role="tooltip"`, `role="img"` with `aria-label`, and `aria-labelledby` (examples/assets/github_profile_reduced2.html:65; examples/assets/github_profile_reduced2.html:68; examples/assets/github_profile_reduced2.html:78)
- The Google fixture carries `role="contentinfo"` (examples/assets/bottom_only.html:14)

## a11y-plan §Keyboard Navigation
- The docs.rs fixture uses `tabindex="-1"` on toolbar containers (examples/assets/docsrs_header.html:13; examples/assets/docsrs_header.html:16)
- out of slice — keyboard focus handling code

## a11y-plan §Visual Design Verification
- The BBC fixture contains `prefers-reduced-motion` media queries (examples/assets/bbc_reduced.html:10; examples/assets/bbc_reduced.html:11; examples/assets/bbc_reduced.html:17)
- out of slice — contrast or visual a11y checks

## a11y-plan §Screen Reader Support
- The captured GitHub fixture uses an `sr-only` tooltip element (examples/assets/github_profile_reduced2.html:68)
- out of slice — screen reader integration code

## a11y-plan §Cognitive Accessibility
- out of slice — nothing in this slice addresses cognitive accessibility

## a11y-plan §CI Integration
- observed absent — accessibility checks in CI · searched: `a11y|accessib|axe` over .github/workflows/*.yml
