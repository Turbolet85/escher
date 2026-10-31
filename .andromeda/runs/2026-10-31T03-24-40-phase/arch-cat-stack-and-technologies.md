# arch · Stack and Technologies — category read

## Carried
- (.andromeda/architecture-detail/stack-and-technologies.md:9) Every fix and check must compile at this edition and hold the MSRV, using no language or library feature newer than it.
  > Rust Cargo workspace, resolver "2", edition "2024", rust-version "1.91.0"
- (.andromeda/architecture-detail/stack-and-technologies.md:15) The Stylo family must stay at these versions: the entry bumps no dependency, so a fix, and the `:open` check of CARRY 2, is written against them.
  > stylo, stylo_traits, stylo_atoms, stylo_static_prefs, stylo_dom "0.22.0"; selectors "0.41.0"; cssparser "0.38.0"
- (.andromeda/architecture-detail/stack-and-technologies.md:21) Taffy must stay the registry version; a scrolling-box or select-bounds fix is made in escher's code, never by moving this pin or returning to a git dependency.
  > taffy "0.15" from the crates.io registry (resolved 0.15.0; a git dependency at a pinned rev until 2026-10-13-upstream-sync-polish-and-ship)
- (.andromeda/architecture-detail/stack-and-technologies.md:33) A served-page keyboard row or fixture that glob-imports the Dioxus prelude must name the workspace's crate by this path, or it builds key events against the wrong version.
  > keyboard-types "0.8" (lock 0.8.3) since the 2026-10-17 upstream sync, while Dioxus 0.7.3 carries its own keyboard-types 0.7, so two versions stand in the graph and a module that glob-imports `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`
- (.andromeda/architecture-detail/stack-and-technologies.md:34) CARRY 2's check of the windowed shell's redraw route must read that route on this stack, the winit event loop, with no second loop or windowing dependency added for it.
  > Window shell on winit's `ApplicationHandler` and `EventLoop` (packages/blitz-shell/src/application.rs:112-187; packages/blitz-shell/src/lib.rs:54-67)
- (.andromeda/architecture-detail/stack-and-technologies.md:36) A native-checkedness fix must write its state through this AccessKit version, its `toggled` a closed value of this crate, with no adapter version moved.
  > accesskit "0.25" (Cargo.toml:139-146); accesskit_xplat with accesskit_windows 0.35.0, accesskit_macos 0.27.0, accesskit_unix 0.24.0, accesskit_android 0.9.0 per target (packages/accesskit_xplat/Cargo.toml:16-35)
- (.andromeda/architecture-detail/stack-and-technologies.md:66) The stand, fixture and served-page checks must be built from this dev-dependency set; a new one is a change this file records.
  > blitz-tests dev-dependencies: blitz-test-harness, escher-telemetry, blitz crates, dioxus-native-dom, seven_guis (the headless stand, tests/blitz-tests/Cargo.toml:22), escher-driver (the driver session, tests/blitz-tests/Cargo.toml:23), anyrender, anyrender_vello_cpu, dioxus, test-that, usvg, tracing, tracing-log
- (.andromeda/architecture-detail/stack-and-technologies.md:72) A new check target in blitz-tests must keep the `ci-scripts` leg's manifest-reading test green.
  > among them upstream's `wpt_area_changes.py` with its test file and escher's `test_blitz_tests_targets.py`, which reads a manifest with the stdlib `tomllib`
- (.andromeda/architecture-detail/stack-and-technologies.md:76) Every code change must pass these gates through the one leg runner, the `fast` leg being the local gate before a push.
  > `cargo fmt --all --check` and `cargo clippy --workspace --locked -- -D warnings`, run as the `fmt` and `clippy` legs of `.github/scripts/ci-leg.sh` (.github/scripts/ci-leg.sh:20-25; .github/workflows/ci.yml:155; .github/workflows/ci.yml:177), with `bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → ci-scripts, stopping at the first red) the local pre-push gate
- (.andromeda/architecture-detail/stack-and-technologies.md:76) Any doc comment a fix touches, the stated limits' text included, must pass the rustdoc gate.
  > applied by the docs job's `doc` leg, `cargo doc --workspace --no-deps --locked`, which documents every workspace library crate
- (.andromeda/architecture-detail/stack-and-technologies.md:76) A driver change in `lint.rs`'s `overlaps` and any harness or bridge change must hold these per-crate floors.
  > gated since 2026-10-14-quality-gates by per-crate line floors through `.github/scripts/coverage_floor.py` — escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %, read 98.34 · 84.25 · 91.69 % at that chunk
- (.andromeda/architecture-detail/stack-and-technologies.md:77) An engine fix in blitz-dom must also pass upstream's feature-set clippy line, and a stand check added or moved must hold the zero-failure budget.
  > since the same chunk the `clippy` leg runs a second line only when the workspace line is green, upstream's own `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings`, and the `stand-repeat` leg holds the stand checks to a zero-failure budget over three rounds
- (.andromeda/architecture-detail/stack-and-technologies.md:77) A mutation reading of changed code must run on the dev host with this tool pair and must add no CI leg.
  > a mutation tier run on the dev host alone, never in CI: cargo-mutants 27.1.0, the tier's tool, with `--test-tool nextest` and `--baseline=skip`, and cargo-nextest 0.9.146
- (.andromeda/architecture-detail/stack-and-technologies.md:86) A function a fix adds or grows must read within this ceiling on the dev host.
  > a cognitive-complexity reading on the dev host alone, never in CI — `rust-code-analysis-cli` 0.0.25 in `~/.cargo/bin`, resolved by no lockfile and run by no CI leg, reading each function against the ceiling of 15
- (.andromeda/architecture.md:11) Every engine fix must keep incremental and non-incremental layout identical.
  > both layout modes produce identical layouts
- (.andromeda/architecture.md:29) The pinning rules must stand unmoved through the entry.
  > taffy and parley are registry versions, so no git dependency is left in the manifest; winit is pinned exactly to a beta
- (.andromeda/architecture.md:56) A transformed-scrolling-box fix must hold these scrolling and hit rules, the two fixes already taken on the code it touches.
  > `scroll_into_view` writes every scrolling box on the chain, nested ones instantly, so it never animates a nested box; a box's client rect never subtracts its own scroll offset, and a hit stops at the padding box of every box paint clips.
- (.andromeda/architecture.md:58) A fix the founder takes must be recorded as a direct engine change for every document, in the same form as these, flagged upstreamable with nothing sent upstream.
  > Those two fixes, the accessibility builder's state writes and the range input, keyboard activation and focus rules are direct engine changes for every document, each flagged upstreamable with no upstream pull request open.
- (.andromeda/architecture.md:62) The select debts must change this statement only as the founder's word per fork says, and any fix must keep the one choice path and its silence on a choice of the chosen option.
  > The select model: every choice that changes the chosen option goes through the one path `apply_select_choice`, and a choice of the chosen option fires nothing; no popup is painted, an option is not clicked, and a `multiple` select or an `optgroup`'s options are not modelled.
- (.andromeda/architecture.md:72) A focus-fixup fix must keep every other `inert` reading and the `:focus-within` rule.
  > `inert` is honoured by its presence on the element or an ancestor — not focusable, not hit, out of the accessibility tree and the snapshot; focus keeps `:focus-within` on its ancestors
- (.andromeda/architecture.md:74) This limit must be removed if the founder takes the fix and must stand with its residual line if he does not.
  > A known limit: HTML's focus fixup is not built, so a node that holds focus when it becomes inert keeps it until focus moves, its fix owned by a `.andromeda/residuals.md` line.
- (.andromeda/architecture.md:108) A fix to focus fixup, checkedness or a select rule must cite its spec at the implementation.
  > spec URLs are cited above or next to implementations
- (.andromeda/architecture.md:121) Every proving check and CARRY 2 check must take this form, both layout modes included, and the fixture must first show the condition.
  > scenarios run in both layout modes; stand checks share helpers through modules under `tests/` that are no test target; tests first assert the fixture produces the condition; blitz-tests keeps one test target per file
- (.andromeda/architecture.md:137) A checkedness write must join the builder's state writes, its attribute read once, and the platform refresh rule must hold.
  > `build_accessibility_tree` returns an AccessKit `TreeUpdate`, naming nodes from text children, `aria-labelledby`, `aria-label`, a bound `<label>` and option labels, and writing state attributes each read once; blitz-shell refreshes the platform tree when a poll took a non-empty changed set.
- (.andromeda/architecture.md:139) This limit must be removed if the founder takes the fix and must stand with its residual line if he does not.
  > the accessibility tree states no native checkbox's or radio's checked state, its fix owned by a `.andromeda/residuals.md` line
- (.andromeda/architecture.md:141) A focus fixup must fire the stated focus order, a `change` on a checkbox, radio or text input must be taken only on the founder's word, and a served-page keyboard row must read this activation route.
  > a focus change as `blur`, `focusout`, `focus`, `focusin`; a range input's default actions dispatch `input`; Enter and a released Space dispatch a Click the driver queues, so the app's handlers run before the default action; the engine dispatches `change` only after `input` on a select's choice.
- (.andromeda/architecture.md:143) A `visible_region` fix must keep the reader read-only and must not read what is seen from a hit.
  > `visible_region` is the read-only reader of what can be seen, and seeing is never read from a hit
- (.andromeda/architecture.md:145) This measured limit must be rewritten if the founder takes the fix, under a translate and any scale P3 measures in reach, and must stand if he does not.
  > `get_client_bounding_rect` reads an element where its transforms move it, while `visible_region` reads a scrolling box under a transform where it stands untransformed (a measured limit)
- (.andromeda/architecture.md:159) A served-page contrast row must read through this reader, which moves nothing.
  > `text_contrast` reads a WCAG contrast and changes nothing
- (.andromeda/architecture.md:163) The served-page rows must run against these boots, the load page's `load-fetch` Button under its stand-in fetch.
  > `seven_guis::stand` boots the four lean tasks and, beside them, Cells (`boot_cells`, no `LeanTask`) and a load page (`boot_load`) in a pinned 800x600 Light viewport with one bundled font, every boot offline but the load page's, whose one stylesheet fetch the in-process stand-in `LoadNet` answers after `LOAD_ANSWER_DELAY` (200 ms)
- (.andromeda/architecture.md:167) No debt's fix may change these counts without coming to the founder as a boundary fork.
  > twelve verbs, eight closed causes, no refusal holding what a call supplied
- (.andromeda/architecture.md:171) The `overlap` fix must keep lint a read of the screen.
  > `Session::lint` reads the screen and changes nothing
- (.andromeda/architecture.md:183) The select-option fork's bounds fix must keep the options as the select's children in the snapshot.
  > a select's `input` and `change` reach the app as form data, and its options are its children in the snapshot.
- (.andromeda/architecture.md:187) The served-page rows must cover the two served tasks that lack them through this command line, adding no served task.
  > `escher-session <verb> [arguments] --session <dir>`, the driver's command line over six served stand tasks — the four lean ones, Cells and the load page
- (.andromeda/architecture.md:271) A focus-fixup fix that clears focus must request its redraw by this rule.
  > redraws are requested after mutations, loads, hover, device changes and a focus set or cleared
- (.andromeda/architecture.md:275) A focus-fixup or checkedness fix must reach the platform tree through this rule, and a new write site of the changed set must be stated against this count.
  > the changed set is written at four kinds of site and by no node creation, hover, scroll, resize, load, animation or layout; the shell rebuilds the platform tree only when a poll took a non-empty set.
- (.andromeda/architecture.md:287) The entry must move none of these versions, and a transformed-scrolling-box fix must be planned knowing the upstream commit left out at this reading stays Epoch 3's to take.
  > upstream `main` last read at `ebacd68bd`, twenty-four of the forty commits past the merge base taken by no-commit cherry-picks with the merge base unmoved and sixteen left out (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/lock.md); git Stylo 0.23-pre, the six Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version, so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3".
- (.andromeda/architecture.md:293) A stated limit a fix moves must move in this statement too, with whatever pins it.
  > a link to `docs/capabilities.md`, the statement of what Escher can do and its limits

## Bindings
- **Pins unmoved ↔ scope's coupled-pins line and the body's pinning and intent lines**: the entry bumps no dependency, so the category's values and the body's pinning rule and upstream reading hold together, (.andromeda/architecture-detail/stack-and-technologies.md:15), (.andromeda/architecture-detail/stack-and-technologies.md:21), (.andromeda/architecture.md:29), (.andromeda/architecture.md:287)
- **Code-quality gates ↔ test plan**: the legs, floors and stand budget here are the checks the test plan's rows must pass; the mutation recipe the category names lives in test-plan §10, (.andromeda/architecture-detail/stack-and-technologies.md:76), (.andromeda/architecture-detail/stack-and-technologies.md:77)
- **AccessKit ↔ a11y plan, security rules and Standard Contracts**: a checkedness write crosses to the platform adapter as a closed value on this AccessKit version, and the builder's write rule and the stated limit move with it, (.andromeda/architecture-detail/stack-and-technologies.md:36), (.andromeda/architecture.md:137), (.andromeda/architecture.md:139)
- **keyboard-types ↔ a11y plan**: the served-page keyboard rows for SC 2.1.1 build their key events through the workspace's crate and read the activation route the Events contract states, (.andromeda/architecture-detail/stack-and-technologies.md:33), (.andromeda/architecture.md:141)
- **Testing dev-dependencies ↔ Conventions' test rules**: the check targets take this dependency set and the one-target-per-file, both-modes form, and the manifest test holds them, (.andromeda/architecture-detail/stack-and-technologies.md:66), (.andromeda/architecture-detail/stack-and-technologies.md:72), (.andromeda/architecture.md:121)
- **Stated limits ↔ capability statement and driver schema**: a fix that moves `off-screen` or `covered` moves the limit in the master, the capability statement and the schema text together, (.andromeda/architecture.md:145), (.andromeda/architecture.md:62), (.andromeda/architecture.md:293)
