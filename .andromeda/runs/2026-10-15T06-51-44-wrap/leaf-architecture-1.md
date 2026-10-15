group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/conventions.md:48   class: omission
  leaf-text: «`clippy::collapsible_if` allowed crate-wide in browser, blitz-dom, blitz-html, blitz-vibey-script; other allows are local, ideally with a reason.»
  source: .andromeda/architecture.md:105 «— then, only when that line is green and since 2026-10-14-quality-gates, upstream's own blitz-dom line `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --»
  replace-with: «`cargo fmt --all --check` and `cargo clippy --workspace --locked -- -D warnings` — then, only when that line is green and since 2026-10-14-quality-gates, upstream's own blitz-dom line `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings` — each run as its `.github/scripts/ci-leg.sh` leg; `clippy::collapsible_if` allowed crate-wide in browser, blitz-dom, blitz-html, blitz-vibey-script; other allows are local, ideally with a reason.»

- leaf: .claude/docs/services/blitz-dom.md:20   class: omission
  leaf-text: «Default features: svg, woff, accessibility, system-fonts, file-input, custom-widget, text-transform-icu.»
  source: .andromeda/architecture.md:105 «— then, only when that line is green and since 2026-10-14-quality-gates, upstream's own blitz-dom line `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --»
  replace-with: «Default features: svg, woff, accessibility, system-fonts, file-input, custom-widget, text-transform-icu. Since 2026-10-14-quality-gates the `clippy` leg lints this crate a second time, only when the workspace line is green, by upstream's own line `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings`.»

- leaf: .claude/docs/services/dioxus-native-dom.md:41   class: omission
  leaf-text: «the attribute half only — `stand_keyboard` pins that the engine honours neither a truthy nor a falsy `inert`).»
  source: .andromeda/architecture.md:67 «gated since 2026-10-14-quality-gates by per-crate line floors through `.github/scripts/coverage_floor.py` — escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %, read 98.34 · 84.25 · 91.69 % at that chunk»
  replace-with: «the attribute half only — `stand_keyboard` pins that the engine honours neither a truthy nor a falsy `inert`). Since 2026-10-14-quality-gates the `coverage` leg holds this crate's line coverage to a floor of 83 % through `.github/scripts/coverage_floor.py` (read 84.25 % at that chunk).»
