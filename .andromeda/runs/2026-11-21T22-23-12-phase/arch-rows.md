# architecture · the rows that bind 2026-11-21-dependency-and-ci-debts — picked 113 of 1541 (70 by every draw) · rule any · pin 2957

## design-philosophy · ## Design Philosophy
decision/best-effort-browser-persistence · Browser persistence is best-effort — Browser persistence writes are best-effort and may silently no-op. · why: history is UX state, not load-bearing on the browser
decision/browser-state-best-effort · Reference browser state is best-effort — The reference browser keeps its own state best-effort.
decision/latest-stable-toolchain · Latest stable Rust only — CI guarantees only the latest stable Rust, keeping an MSRV check and making an effort not to raise the MSRV in a patch version.
decision/msrv-policy · Latest stable guaranteed, MSRV advertised — Only the latest stable Rust is guaranteed; an MSRV check advertises the MSRV, and effort is made not to raise it in patch versions.
decision/sync-runtime-agnostic-persistence · Synchronous, runtime-agnostic persistence — Persistence methods are synchronous and runtime-agnostic; callers keep disk work off latency-sensitive threads.
decision/accesskit-adapter-without-winit · AccessKit adapter without Winit — accesskit_xplat is the cross-platform AccessKit adapter, used without depending on Winit.
decision/history-fold-consecutive-visits · Only consecutive same-URL visits fold — Only consecutive visits to the same URL fold into the head history entry.
decision/paint-to-paint-scene · Paint emits commands to a paint scene — Painting pushes drawing commands into a paint scene whose implementation decides whether to rasterize, emit SVG or PDF, or serialize, and painting assumes styles and layout are already resolved. · except: an open select list's row text is laid out at paint, only the rows in the scrollport

## stack-and-technologies · ## Stack and Technologies
decision/accesskit-accessibility-tree · AccessKit for the accessibility tree — The accessibility tree is built with accesskit, its platform adapters taken per target through accesskit_xplat.
decision/anyrender-rendering-abstraction · anyrender as the rendering abstraction — Painting goes through the anyrender abstraction, with its vello, vello-cpu, vello-gpu, skia and svg backends and its serializer.
decision/cognitive-complexity-ceiling · Cognitive complexity ceiling — A cognitive-complexity reading on the dev host alone, never in CI, reads each function against a ceiling of 15 with rust-code-analysis-cli.
decision/dependency-and-licence-audit · Dependency and licence audit — cargo deny checks the advisories and licences of the whole workspace's graph over the root deny.toml, each per-ID ignore carrying a reason.
decision/image-raster-decoding · image for raster decoding — Raster images are decoded with the image crate, with an SVG fallback.
decision/local-pre-push-gate · Local pre-push gate — The fast leg run — fmt, clippy, test, ci-scripts, embolden, stopping at the first red — is the local pre-push gate; on a dev host, after a green verdict and skipped under CI, it ends by running the target sweep, whose refusal or failure never reddens it.
decision/nix-flake-dev-shell · Nix flake dev shell — A Nix flake provides the reproducible dev shell and packages.
decision/one-apt-install-script · One package-install script — One bash install script is the only statement of fork CI's package install, retrying with a bounded dpkg repair, and adds no package and no action. · except: The test matrix installs its one package through a SHA-pinned apt cache action under its step timeout, and the Android job sets up its NDK through a SHA-pinned setup-ndk action.
decision/publish-workflow-upstream-only · Browser publish workflow upstream-only — The browser publish workflow is upstream-only and guarded by repository.
decision/rust-cache-on-compiling-jobs · Rust cache on compiling jobs — Every compiling CI job uses Swatinem/rust-cache. · except: the coverage job
decision/rust-cargo-workspace · Rust Cargo workspace — Rust is the implementation language of every crate, in one Cargo workspace on resolver 2, edition 2024 and rust-version 1.91.0. · except: wgpu_texture and debug_timer stay on edition 2021.
decision/window-renderer-chosen-at-compile-time · Window renderer chosen at compile time — dioxus-native chooses its window renderer at compile time among vello, vello-cpu-base, skia, skia-raster-base and vello-gpu, the GPU window renderer being anyrender_vello_gpu, while blitz uses the vello window renderer.
decision/cargo-deny-and-llvm-cov-tools · CI-installed audit and coverage tools — CI installs pinned cargo-deny for the dependency audit and pinned cargo-llvm-cov for the coverage report.
decision/flake-rust-pin-matches-rust-version · Flake Rust pin matches rust-version — The flake's pinned Rust toolchain matches the workspace rust-version. · why: the flake's own comment says to keep them in sync
decision/github-actions-ci · GitHub Actions as CI — Build, test, lint, WPT and browser bundling run as GitHub Actions workflows.
decision/usvg-svg-parsing · usvg for SVG parsing — SVG is parsed with usvg and svgtypes.

## established-decisions · ## Established Decisions
decision/accesskit-xplat-async-features · accesskit_xplat async features on Unix — On Unix, accesskit_xplat forbids enabling neither or both of `async-io` and `tokio`.
decision/append-only-migrations · Append-only history migrations — The browser history's migrations are append-only.
decision/browser-history-migrations-append-only · Browser history migrations append-only — Browser history migrations are append-only, their index stamped into the database's user version.
decision/browser-history-read-sync-at-start · Initial browser history read synchronously — The browser's initial history read is synchronous. · why: so suggestions have entries on first render
decision/browser-history-sqlite · SQLite for browser history — The browser's history persists in SQLite through bundled rusqlite with rusqlite_migration, in `history.sqlite3` in the project data dir, in memory where that fails and always on Android and iOS.
decision/dev-line-tables-debuginfo · Line-tables-only debuginfo for dev and test — The dev profile uses line-tables-only debuginfo, one level for the dev host and CI, inherited by the test profile. · why: cuts debuginfo size and the cold build-and-test baseline sharply
decision/disk-writes-off-runtime · Disk writes on blocking threads — The browser's disk writes go through tokio `spawn_blocking`.
decision/history-entry-cap · History capped at 1000 entries — The browser keeps at most 1000 history entries.
decision/lockstep-package-versions · Blitz packages versioned together — Blitz packages are versioned together through the one workspace version and the in-repo `=` pins.
decision/msrv-job-build-only · MSRV job builds only — The MSRV job runs only `cargo build`. · why: to avoid requiring dev-dependencies to build with the MSRV
decision/nix-single-derivation · Nix flake builds in one derivation — The Nix flake builds dependencies and crate in one derivation.
decision/no-git-dependencies · No git dependencies — Every dependency in the manifest is a crates.io registry version, with no git source. · except: The taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy still passes the lock, audit and MSRV legs.
decision/one-debuginfo-level · One debuginfo level for dev host and CI — The `dev` profile sets line-tables-only debuginfo, one debuginfo level for the dev host and CI, and the `test` profile inherits it.
decision/per-crate-default-features · Default features fixed per crate — Each crate fixes its default feature set in its manifest.
decision/winit-exact-beta-pin · winit pinned exactly to a beta — winit is pinned exactly to a beta version, 0.31.0-beta.4.
decision/dioxus-native-requires-renderer · dioxus-native requires a renderer feature — dioxus-native fails to compile when no renderer feature is enabled.
decision/driver-builds-no-platform-adapter · Driver graph holds no platform adapter — Binaries built through the driver compile dioxus-native-dom's accessibility modules yet build no platform adapter: accesskit_xplat and accesskit_winit are in neither graph.
decision/favicon-success-by-decoding · Favicon success inferred by decoding — A favicon fetch's success is inferred by decoding it. · why: the net layer surfaces no status or Content-Type
decision/inline-svg-reparsed · Inline SVG re-parsed into an image — Inline `<svg>` is re-parsed from its outer HTML into an image and rebuilt when its subtree's style changes. · guard: inline_svg_serialize, inline_svg_restyle
decision/mcp-stdio-crossing · Session calls over the MCP role's stdio — A session's calls also cross from an MCP client over the escher-session mcp role's stdio, a child of that client local to the invoking user, holding its session in its own process with no listener. · why: A boundary widening, ratified.
decision/renderer-required · A renderer is required in dioxus-native — `dioxus-native` raises a compile error when no renderer is enabled.
decision/session-over-unix-socket · Driver session over a std-only Unix socket — A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port. · why: a deliberate transport choice
decision/workspace-lockstep-versions · Blitz packages versioned together — Blitz packages are versioned together through the one workspace version and exact in-repo pins.
decision/wpt-default-cpu-renderer · WPT runner defaults to CPU rendering — The WPT runner defaults to the CPU renderer.

## conventions · ## Conventions
decision/workspace-dependencies · Dependencies declared once in the workspace — Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`. · except: The WPT runner's direct `mimalloc` dependency.
decision/cargo-feature-gating · Optional functionality behind cargo features — Optional functionality is gated by cargo features.
decision/known-gap-markers · Known gaps marked TODO or FIXME — Known gaps are marked with `TODO` or `FIXME`.
decision/todo-fixme-gap-markers · Known gaps marked TODO/FIXME — Known gaps are marked with `TODO`/`FIXME`.

## standard-contracts · ## Standard Contracts
decision/bounded-package-install · Bounded package install — Every fork CI package install goes through the one install script: up to 3 attempts, each update and install under the timeout bound BOUND=120 with the kill grace GRACE=10, pausing 10 s when another attempt remains. · guard: InstallScriptTest; test_o_package_installs_are_bounded
decision/bounded-retried-package-install · Bounded, retried package install — Every CI package install goes through one bounded, retried install script; no ci.yml line calls apt-get.
decision/ci-contract-pinned · CI workflow contract pinned by tests — The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg. · guard: test_ci_workflows.py
decision/one-ci-leg-runner · One CI leg runner — Every linux CI job runs its leg through one bash leg runner, the same command the dev host runs. · except: The cross-platform test matrix, its linux android entry included, runs cargo or cross directly.
decision/package-install-repair-before-retry · Repair before a package install retry — Every attempt after the first starts with a bounded `dpkg --configure -a` repair, and a failed repair fails that attempt. · why: A bound that fires inside an install can leave dpkg interrupted, which the next attempt's repair configures. · guard: InstallScriptTest
decision/upstream-only-workflows · Upstream-only workflows — The WPT, WPT post-results and publish workflows, and the Signed Builds and WPT environments, run only on DioxusLabs/blitz; the fork builds no bundle.
contract/browser-history/history-entries-table · Browser history · history_entries table — Columns id INTEGER PK AUTOINCREMENT, url TEXT NOT NULL, title TEXT NOT NULL, favicon_url TEXT, visited_at INTEGER NOT NULL, indexed on visited_at DESC and url.
contract/browser-history/history-entry · Browser history · HistoryEntry — Carries id, url, title, favicon_url and visited_at.
contract/browser-history/history-store · Browser history · HistoryStore — Offers open, load_recent, record_visit, clear and set_favicon_by_url.
contract/ci-leg-legs/audit · ci-leg.sh legs · audit — The `audit` leg runs `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace's graph.
contract/ci-leg-legs/ci-scripts · ci-leg.sh legs · ci-scripts — `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts.
contract/ci-leg-legs/fast · ci-leg.sh legs · fast — The `fast` leg runs fmt, clippy, test, ci-scripts and embolden in order, stopping at the first red: the local pre-push gate. After a green verdict, with `CI` unset or empty, it runs the target sweep, its line printed and its exit discarded; a red leg calls no sweep.
contract/ci-leg-legs/msrv · ci-leg.sh legs · msrv — The `msrv` leg runs `cargo +1.91 build`.
contract/ci-leg-script/exit-status · ci-leg.sh invocation · exit status — The CI leg script exits with the leg command's status.
contract/ci-leg-script/leg-log · ci-leg.sh invocation · leg log — The CI leg script tees the leg's merged output to `target/ci-logs/{leg}.log`, truncated at the leg's start.
contract/ci-leg-script/unknown-leg · ci-leg.sh invocation · unknown or missing leg — On an unknown or missing leg the CI leg script prints the leg list to stderr and exits 2.
contract/wpt-repository-dispatch/update-results · WPT repository dispatch · update-results — After WPT on main, a `repository-dispatch` of `update-results` goes to `DioxusLabs/blitz-wpt-results`, only when the repository is `DioxusLabs/blitz`.
decision/capability-statement-matches-schema · Capability statement held to the driver schema — The capability statement and README name every verb and refusal cause and state in words the counts of verbs, refusal causes, session errors, node keys and exit statuses that the driver schema holds. · guard: CapabilityStatementTest
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of the CI leg script runs with `--locked`. · except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock`
decision/font-embolden-local-witness · Local witness for font-embolden paint — The font-embolden feature is built and tested locally by the embolden step of the fast gate, with no CI job; macOS exercises the feature in CI. · why: Running the painted select-popup test under the feature on linux reproduces macOS's two readings. · guard: test_fast_leg_ends_on_the_font_embolden_witness
contract/accessibility-tree/build-accessibility-tree · blitz-dom accessibility tree · build_accessibility_tree — build_accessibility_tree returns an AccessKit TreeUpdate.
contract/accesskit-xplat/adapter · accesskit_xplat adapter · Adapter — Adapter offers with_split_handlers, with_combined_handler, update_if_active, set_focus and set_window_bounds.
contract/accesskit-xplat/platform-adapter · accesskit_xplat adapter · platform adapter — Each platform adapter exposes new, update_if_active, set_focus and set_window_bounds.
contract/accesskit-xplat/window-event · accesskit_xplat adapter · WindowEvent — WindowEvent carries InitialTreeRequested, ActionRequested and AccessibilityDeactivated.
contract/apt-install-arguments/option-value · apt-install.sh arguments · option value — An option value must match `^(0|[1-9][0-9]{0,5})$`, with `--attempts` and `--bound` at least 1.
contract/apt-install-exit-codes/0 · apt-install.sh exit codes · 0 — An attempt completed.
contract/apt-install-exit-codes/1 · apt-install.sh exit codes · 1 — Every attempt failed; the bound's own 124 or 137 is never returned.
contract/apt-install-exit-codes/2 · apt-install.sh exit codes · 2 — Usage error, found before any command runs.
contract/apt-install-stderr/attempt-failed-line · apt-install.sh stderr · attempt failed line — A failed call prints one stderr line `apt-install: attempt {n} of {N} failed in {repair|update|install} (exit {status})`.
contract/ci-leg-legs/a11y · ci-leg.sh legs · a11y — The `a11y` leg runs named accessibility integration tests on one `cargo test --workspace --locked` line; other focus and accessibility tests stay in the `test` leg.
contract/ci-leg-legs/build · ci-leg.sh legs · build — `build` is a leg of the CI leg script.
contract/ci-leg-legs/clippy · ci-leg.sh legs · clippy — The `clippy` leg runs the workspace clippy line and, only when that is green, upstream's own blitz-dom clippy line with `-D warnings`.
contract/ci-leg-legs/coverage · ci-leg.sh legs · coverage — The `coverage` leg runs `cargo llvm-cov` to an lcov file and its per-file report, then the coverage-floor script, and exits with the floor's status.
contract/ci-leg-legs/doc · ci-leg.sh legs · doc — The `doc` leg runs `cargo doc --workspace --no-deps`.
contract/ci-leg-legs/test · ci-leg.sh legs · test — `test` is a leg of the CI leg script.
contract/ci-log-artifacts/ci-log · fork CI log artifacts · ci-log-{job id} — Every CI leg job uploads, on failure only, its own leg log as `ci-log-{job id}` (matrix `ci-log-matrix-{platform}`), kept 7 days, ignoring a missing file.
contract/net-handler/bytes · NetHandler · bytes — Receives the resolved URL and the fetched bytes, handled per resource kind.

## occupied-resources · ## Occupied Resources
decision/one-local-unix-listener · One local Unix-domain listener — The one local listener is the driver session's Unix-domain socket, owner-only in an owner-only state directory, unix only; no second served listener stands. · except: The opt-in devtools feature's CDP server, a TCP listener on 127.0.0.1, which no default feature set turns on.
decision/session-socket-one-listener · Session socket the one listener — The driver session's Unix-domain socket is the only listener served: mode 0600 inside a caller-named state directory created 0700, with no second listener and no network port. · why: A boundary widening, ratified. · except: The opt-in devtools feature's CDP server, a TCP listener on 127.0.0.1, which no default feature set turns on.
decision/test-cache-restore-only · Shared test cache restored, never saved — The a11y and stand-repeat jobs restore the test job's workspace-test cache and never save it.
decision/upstream-ci-resources-upstream-only · Upstream CI resources reached only from upstream — Every job that uses the Signed Builds and WPT environments or the warpbuild WPT runner carries a DioxusLabs/blitz repository guard in its if, so no fork ref reaches them.
decision/run-state-areas-local-only · Agent-run and cold-agent state areas local only — The agent-run and cold-agent state areas under target/ are recreated by their run, removed by cleanup with nothing else under target/ touched, local only and uploaded by no CI job; the dev-host target sweep removes stale build units of the debug build directory alone and touches no state area.

## infrastructure-patterns · ## Infrastructure Patterns
decision/capabilities-doc-beside-upstream · Capability statement beside upstream docs — escher's capability statement sits in a file of its own under docs, beside upstream's three files. · why: an upstream change under docs never conflicts with it
decision/cargo-deny-advisory-gate · cargo-deny advisory gate — A cargo-deny advisory gate, configured by the root deny.toml, holds the dependencies.
decision/ci-cache-save-refs · CI cache save refs — ci.yml caches save only on main and build/* refs; publish-browser's and wpt's caches save only on main.
decision/ci-concurrency · Per-ref CI concurrency — CI runs with per-ref concurrency and cancels a run in progress.
decision/ci-fast-slow-split · CI fast/slow split — fmt, clippy, test-features-default and ci-scripts carry no needs, and every other CI job, the matrix included, needs all four.
decision/ci-read-only-permissions · Read-only CI permissions — CI runs under a workflow-level contents: read permission with no job-level grant.
decision/linux-tested-by-fast-leg · Linux tested by the fast leg — Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.
decision/locked-cargo-legs · Locked cargo legs — Every cargo leg of CI runs with --locked. · except: fmt, and examples/wasm_hello.
decision/nix-flake · Nix flake — A Nix flake exposes the browser package as default and a blitz-dev devShell, wrapping Linux packages so the dlopen'd graphics and windowing libraries are found.
decision/unrewritten-profiles · Build profiles unrewritten — CI jobs build the profiles unrewritten, with no opt-level rewrite step.
decision/workspace-shared-dependencies · Shared workspace dependencies — The Cargo workspace declares its dependencies once in a shared workspace dependency table.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/runtime-inputs · Runtime inputs — Runtime inputs are CLI arguments and environment variables.

## project-intent · ## Project Intent
decision/capability-statement · Capability statement — The capability statement is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, its a11y leg file count and stand-repeat named-target count to the CI leg runner, and no check reads its other claims. · guard: .github/scripts/test_capability_statement.py
contract/capability-statement/sections · The capability statement · sections — Five sections in order: What it can do, Stated limits, Accessibility reach, What an MCP client carries onward, Where each claim is proven.

## inherited-defaults · ## Inherited Defaults
decision/cargo-deny-audit · Workspace dependency audit — A cargo-deny audit covers the whole workspace's advisories and licences.
decision/ci-quality-gates · CI quality gates — Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget. · guard: the CI legs of .github/scripts/ci-leg.sh
decision/driver-session-unix-socket · One local Unix-domain socket — The one local listener is a Unix-domain socket, the driver session's, carrying its lifecycle and its calls. · except: The opt-in devtools feature's CDP server, a TCP listener on 127.0.0.1, which no default feature set turns on.
decision/browser-publish-upstream-only · Browser publish job is upstream only — The browser bundle publish job is repository-guarded to DioxusLabs/blitz and does not run on the escher fork.
decision/fmt-and-clippy-gates · Format and clippy gates — `cargo fmt --all --check` and workspace clippy with warnings denied run, then, only when that line is green, upstream's blitz-dom clippy line, each as its CI leg script leg in CI and on the dev host.

# architecture · its absences, names and overviews, whole

## design-philosophy · ## Design Philosophy
absence/browsing-history-in-engine — Browsing history in the engine; every link is unvisited

## stack-and-technologies · ## Stack and Technologies
overview/stack-and-technologies — The stack is a Rust 2024 Cargo workspace (rust-version 1.91.0) on Servo Stylo, html5ever and xml5ever, Taffy, Parley with ICU4X, anyrender over Vello and Skia, winit, AccessKit, Boa, Dioxus 0.7, tokio and reqwest, with rusqlite in the browser app and Trunk for wasm; GitHub Actions CI runs fmt, clippy, rustdoc, cargo-deny and coverage gates, beside a host-run mutation tier and a Nix flake.

## established-decisions · ## Established Decisions
absence/3d-transforms — 3D transforms (a z-axis rotate is applied as planar)
absence/anchor-positioning — Anchor positioning
absence/anonymous-table-wrappers — Anonymous table wrapper boxes
absence/background-clip-text — background-clip: text
absence/container-queries — Container queries
absence/css-offset-in-transforms — CSS `offset` in transforms
absence/dioxus-native-js-eval — JS eval from a Dioxus native app
absence/grid-masonry — Grid masonry
absence/grid-subgrid — Grid subgrid
absence/in-repo-release-tool — An in-repo release tool
absence/non-string-generated-content — Pseudo-element content other than strings
absence/non-string-pseudo-content — Non-string pseudo-element `content`
absence/shadow-dom — Shadow DOM
absence/visited-link-styling — Visited-link styling

## standard-contracts · ## Standard Contracts
absence/component-alone-on-carriers — A component alone on the driver's carriers: no verb, argument, MCP tool, socket call or kept check reaches it
absence/exploration-follows-navigation — Exploration that follows navigation to another document
absence/exploration-on-carriers — Exploration on the driver's carriers: no verb, role, argument, MCP tool, session socket call or kept check reaches it
absence/fixture-state-on-carriers — A fixture state on the driver's carriers: the command line, MCP, the session socket or a kept check
absence/script-same-origin-cors — Same-origin and CORS checks on script loads
absence/settle-verb-on-socket — A settle verb or a busy-source reply of its own on the session socket

## occupied-resources · ## Occupied Resources
absence/served-network-port — A served network port in a default build: no TCP or UDP port any binary serves with its default features
name/binary/blitz — blitz
name/binary/escher-session — escher-session
name/binary/seven-guis-native — seven_guis_native
name/binary/todomvc-native — todomvc_native
name/crate/accesskit-xplat — accesskit_xplat
name/crate/blitz — blitz
name/crate/blitz-dom — blitz-dom
name/crate/blitz-traits — blitz-traits
name/crate/blitz-vibey-cdp-server — blitz-vibey-cdp-server
name/crate/blitz-vibey-script — blitz-vibey-script
name/crate/browser — browser
name/crate/browser-persistence — browser-persistence
name/crate/counter — counter
name/crate/debug-timer — debug_timer
name/crate/escher-driver — escher-driver
name/crate/escher-telemetry — escher-telemetry
name/crate/rdme — rdme
name/crate/seven-guis — seven_guis
name/crate/todomvc — todomvc
name/crate/transparent — transparent
name/crate/wasm-hello — wasm_hello
name/crate/wgpu-texture — wgpu_texture
name/other/blitz-cdp-port — BLITZ_CDP_PORT
name/other/blitz-devtools-frontend-dir — BLITZ_DEVTOOLS_FRONTEND_DIR
name/other/blitz-target — blitz-target
name/other/blitz-user-agent — Mozilla/5.0 (compatible; Blitz)
name/other/cargo-target-tmpdir — CARGO_TARGET_TMPDIR
name/other/ci — CI
name/other/cold-agent-ps1 — cold-agent.ps1
name/other/cold-agent-sh — cold-agent.sh
name/other/com-dioxuslabs-blitz — com.dioxuslabs.blitz
name/other/dioxus-scheme — dioxus://index.html
name/other/mcp-escher — mcp__escher
name/other/multi-doc-flat — MULTI_DOC_FLAT
name/other/multi-doc-n — MULTI_DOC_N
name/other/multi-doc-seq — MULTI_DOC_SEQ
name/other/paint-tree-bench-html — PAINT_TREE_BENCH_HTML
name/other/rust-log — RUST_LOG
name/other/target-sweep-ps1 — target-sweep.ps1
name/other/target-sweep-py — target-sweep.py
name/other/tmpdir — TMPDIR
name/other/wpt-dir — WPT_DIR
name/server/escher — escher

## project-intent · ## Project Intent
overview/project-intent — escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.

## inherited-defaults · ## Inherited Defaults
absence/network-listener — A served network (TCP or UDP) listener in any default feature set
