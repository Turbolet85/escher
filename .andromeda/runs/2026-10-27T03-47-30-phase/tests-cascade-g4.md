# tests · cascade read · group 4 of 4

## Carried
- (.andromeda/test-plan-detail/ci-integration.md:8) The rewritten README and capabilities statement must leave the ci-scripts leg green — it runs `test_capability_statement.py`, the pin the acceptance says stays green — and the chunk is done only when `ci-leg.sh fast` and `ci-leg.sh doc` pass on the changed tree, run through the same script CI's jobs run.
  > each of the thirteen linux ci.yml jobs (twelve until 2026-10-14-quality-gates) runs `bash .github/scripts/ci-leg.sh {leg}` — fmt, clippy (the workspace line, then, only when green and since 2026-10-14-quality-gates, upstream's blitz-dom line `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings`), test, ci-scripts, build, msrv (`cargo +1.91 build`), counter, wasm, doc (`cargo doc --workspace --no-deps`)
  (.andromeda/test-plan.md:377)
  > - **Legs:** each linux ci.yml job runs `bash .github/scripts/ci-leg.sh {leg}`, every cargo leg `--locked` but `examples/wasm_hello` (no `Cargo.lock`); the dev host runs the same script, so every linux leg is host-reproducible
  (.andromeda/test-plan.md:387)
  > - **Local pre-push gate:** `bash .github/scripts/ci-leg.sh fast` — fmt → clippy → test → ci-scripts, stopping at the first red (.github/scripts/ci-leg.sh:80-85)
- (.andromeda/test-plan-detail/ci-integration.md:32) Any README sentence about the screenshot must state a shot as byte-identical across processes on one host only, never across OSes, and the DejaVu Sans item moved out of the design system must state the stand's font in its new home as the test plan does — bundled, system fonts off, measuring the same on any runner and under the package-alone build.
  > `stand_screenshot_repeat` reads byte-identical shots under the per-package and the workspace build on one host — the shot's bytes differ across OSes (CI#39225375577, macOS aarch64 against linux x86_64) —
  (.andromeda/test-plan.md:390)
  > except the headless-stand checks, which use only the bundled DejaVu Sans with system fonts off and so measure the same on any runner, in the `-p blitz-tests` package-alone build included (examples/seven_guis/src/stand.rs:59-61)

## Bindings
- **ci-integration legs ↔ the body's local gate**: the ci-scripts leg named among the jobs' legs (.andromeda/test-plan-detail/ci-integration.md:8) is the fast gate's last step (.andromeda/test-plan.md:387), so the capability pin's green is read in the same `ci-leg.sh fast` run the chunk's done gate is.
- **ci-integration engine features by runner ↔ the body's fonts line**: the shot's one-host byte identity under both builds (.andromeda/test-plan-detail/ci-integration.md:32) rests on the stand's bundled font with system fonts off (.andromeda/test-plan.md:390), the fact the moved DejaVu Sans item carries into architecture or layout-templates.
