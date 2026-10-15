group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:14   class: omission
  leaf-text: «`counter` · `wasm` · `doc`; its merged output»
  source: .andromeda/architecture.md:141 «· `stand-repeat` (since 2026-10-14-quality-gates, not a fast leg: three rounds, each printing `stand-repeat: round R of 3` and running `cargo test -p blitz-tests --locked` with one `--test`»
  replace-with: «`counter` · `wasm` · `doc` · `stand-repeat` (since 2026-10-14-quality-gates, not a fast leg: three rounds of `cargo test -p blitz-tests --locked` over every `tests/blitz-tests/tests/stand_*.rs`, then seven_guis' `cli_commands` · `cli_flow` · `host_binary` · `host_log` · `host_timer` · `mcp_server` · `mcp_log`, the first red ending the leg — a zero-failure budget, no retry); its merged output»

- leaf: .claude/docs/commands.md:15   class: omission
  leaf-text: «up to 3 attempts, `apt-get update` then `install`, each call under a 120 s bound;»
  source: .andromeda/architecture.md:141 «the next attempt starts, since 2026-10-14-quality-gates, with a bounded repair — `sudo timeout --kill-after=10 120 dpkg --configure -a`, phase `repair`, a failed repair failing that attempt»
  replace-with: «up to 3 attempts, `apt-get update` then `install`, each call under a 120 s bound, and since 2026-10-14-quality-gates each retried attempt first runs a bounded repair, `dpkg --configure -a` (phase `repair`, a failed repair failing that attempt; the first attempt runs no `dpkg`);»

- leaf: .claude/docs/commands.md:52   class: omission
  leaf-text: «— the CI lint gate, the `clippy` leg (`just clippy` runs without `-D warnings`)»
  source: .andromeda/architecture.md:141 «legs `fmt` · `clippy` (the workspace line, then, only when green and since 2026-10-14-quality-gates, upstream's own `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility»
  replace-with: «— the CI lint gate, the `clippy` leg's first line; only when it is green, the leg then runs upstream's own `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings` (since 2026-10-14-quality-gates; `just clippy` runs without `-D warnings`)»

- leaf: .claude/docs/commands.md:53   class: stale
  leaf-text: «`ci-leg.sh audit` (cargo-deny advisories over `deny.toml`)»
  source: .andromeda/architecture.md:141 «`audit` (`cargo deny --locked --workspace --all-features check advisories licenses`, the whole workspace's graph, since 2026-10-14-quality-gates; `car»
  replace-with: «`ci-leg.sh audit` (`cargo deny --locked --workspace --all-features check advisories licenses` over `deny.toml` — the advisories and licences of the whole workspace's graph, since 2026-10-14-quality-gates)»

- leaf: .claude/docs/commands.md:53   class: omission
  leaf-text: «then the per-file table; needs the `llvm-tools` rustup component)»
  source: .andromeda/test-plan.md:343 «**Coverage floors:** since 2026-10-14-quality-gates the `coverage` leg ends with `.github/scripts/coverage_floor.py` over three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxu»
  replace-with: «then the per-file table, then, chained since 2026-10-14-quality-gates, `python3 .github/scripts/coverage_floor.py` over three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 % — the leg exiting with the floor's status; needs the `llvm-tools` rustup component)»

- leaf: .claude/docs/services/blitz-test-harness.md:34   class: omission
  leaf-text: «and runs in the `a11y` CI leg.»
  source: .andromeda/test-plan.md:343 «three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %»
  replace-with: «and runs in the `a11y` CI leg; since 2026-10-14-quality-gates the `stand-repeat` leg runs `stand_settle` and `stand_contrast` three rounds with a zero-failure budget, and the `coverage` leg holds this crate to a 90 % line floor (`packages/blitz-test-harness=90` through `coverage_floor.py`; read 91.69 % at that chunk).»
