# security · standing conventions

- S1 · A check added for a click arm with no existing coverage takes no new dependency or git dependency, inherits workspace dependency versions, and builds locked. · dependency-security
  decision/color-crate-luminance-source · color crate as luminance source — The test harness's contrast reader takes its luminance from the color crate already in the graph through blitz-dom, adding no package to Cargo.lock.
  decision/engine-dependency-pins · Engine dependency pins — The pins stay: Stylo 0.22.0, Taffy the registry 0.15, Dioxus 0.7.3 and winit =0.31.0-beta.3.
  decision/locked-builds · Locked builds — Every build, the flake, the bundle and every CI cargo leg, passes --locked so the lockfile is verified. · except: examples/wasm_hello, a standalone workspace with no Cargo.lock
  decision/no-base64-crate · No base64 crate — No base64 crate is taken as a dependency; a shot's base64 is written by hand.
  decision/no-git-dependencies · No git dependencies in the root manifest — The root manifest holds no git dependency; taffy and parley are crates.io registry versions with checksums in the lockfile.
  decision/no-git-dependency · No git dependency — The workspace takes no git dependency.
  decision/test-harness-no-tracing-no-style · Test harness takes no tracing and no style — blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
  decision/workspace-inherited-dependency-versions · Workspace-inherited dependency versions — Dependency versions are inherited from the workspace with workspace = true rather than declared inline in each manifest.
