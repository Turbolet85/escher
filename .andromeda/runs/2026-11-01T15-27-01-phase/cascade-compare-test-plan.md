# Cascade compare — test-plan · phase · 2026-11-01-upstream-sync-ui-graph
E items 230 · held 82 · lost at classify 10 · lost at strong 0 · beside a placed fact - · cascade only 24 · listed 4 · body 63 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#3.1 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:22
- (.andromeda/test-plan-detail/ci-integration.md:22) The shared cache wiring stays as stated, and an edit that shortens ci.yml at the matrix free-disk step corrects the citation here that stands below that step.
  > `Swatinem/rust-cache` (SHA-pinned, `# v2`) on every compiling job but `coverage`, saved on `main` and `build/*`; the test job saves under `shared-key: workspace-test`, which the `a11y` job and, since 2026-10-14-quality-gates, the `stand-repeat` job restore with `save-if: false` — the `a11y` job's an exact hit in CI run 37386253475 (.github/workflows/ci.yml:43-45; .github/workflows/ci.yml:86-89; .github/workflows/ci.yml:256-259; .github/workflows/ci.yml:300-329; .github/workflows/ci.yml:400-405)

### ci-integration#3.1 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:22
- (.andromeda/test-plan-detail/ci-integration.md:22) A take that moves the lock re-keys the cache, so its run's wall time is read as a cold run against this upstream-merge precedent, not as a regression.
  > the 2026-10-10 upstream merge's run, CI#38013740580 on `9462a7e4`, 16 of 16 jobs green at its first attempt, took 1100 s wall, a cold run, against 641 s for CI#38010081458 on the chunk start `09f479b8`

### e2e-test-strategy#4.2 · lost at classify · .andromeda/test-plan-detail/e2e-test-strategy.md:32
- (.andromeda/test-plan-detail/e2e-test-strategy.md:32) The take's end-to-end proof must run through the binary, `host_log` among the standing captures, and must state plainly that no windowed launch proves it.
  > still no test launches a window; since 2026-10-10-driver-cli the UI is driven through that binary — `cli_commands`, `cli_flow`, `host_timer` and `host_log` run driver commands against a host it serves (Drivers per surface, below)

### integration-test-strategy#4.2 · lost at classify · .andromeda/test-plan-detail/integration-test-strategy.md:32
- (.andromeda/test-plan-detail/integration-test-strategy.md:32) The take's CI read must have the Windows job settled green, since these arms have no other proof, and the per-OS tally re-read must account for them.
  > the non-unix arms, which return `Unsupported`, are compiled and run by the fork's windows CI leg only

### mocking-stubbing-discipline#1.2 · lost at classify · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:11
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:11) A shape that changes `ShellProvider` (the winit beta.4 shape brings its clipboard API) must leave this fake still recording `set_ime_enabled`, adapted and not removed.
  > `RecordingShell` implements `ShellProvider` and records `set_ime_enabled` calls

### mocking-stubbing-discipline#1.4 · lost at classify · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:13
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:13) Under a shape that moves winit, this stand-in must keep reading exactly one wake; a change in the count is explained, never loosened.
  > a test-local `CountedWakes` implements `EventLoopProxyProvider`, read by `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once` for a loop woken once

### mocking-stubbing-discipline#1.10 · lost at classify · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:19
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19) Settle's unit tests must keep these two stand-ins and stay green over whatever the take does to `poll` and to render-blocking loads.
  > a `Scripted` `Document` whose `poll` answers from a script and counts its calls, and a `Held` `NetProvider` that keeps each handler until the test takes it

### mocking-stubbing-discipline#1.17 · lost at classify · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:26
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:26) Where the take carries `a26c6ddeb` (`CustomWidget` attributes replayed), these stubs must still implement `Widget` and `custom_widget_layout` stay green, the row following any change to them.
  > `Probe`/`SizedProbe` stub widgets implement `Widget`

### mocking-stubbing-discipline#2.1 · lost at classify · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:40
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:40) These font-metric tests must keep asserting their precondition with no skip path after the take, `document.rs` being engine code the delta can reach.
  > No font-metric test of `document.rs` skips: since 2026-10-14-quality-gates the two `hover_state_tests` assert their precondition through `#[track_caller] fn assert_text_has_size` — the container's height over 50 px — where they skipped with an `eprintln!` when text measured 0x0

### mocking-stubbing-discipline#2.1 · lost at classify · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:40
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:40) This count and command are the reading the take re-runs; a moved count is explained and recorded.
  > since 2026-10-18-upstream-sync-determinism-and-traces a third stands beside them, upstream's `hovering_below_content_reports_default_cursor`, built with the same two-argument `qual_name!` fixture, its assertion taking no precondition and no skip (`cargo test -p blitz-dom --locked --lib hover_state_tests`: `3 passed; 0 failed; 0 ignored`)

### quality-gates-coverage-targets#5.1 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:34
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:34) The ceiling must not be read as a gate on this chunk: engine functions the take brings or a conflict resolution edits are not held to 15 by any leg, and the take must not split upstream's functions to meet it.
  > the Epoch 5 code audit's cognitive-complexity ceiling of 15 is read on the dev host alone with rust-code-analysis-cli 0.0.25, a host tool no lockfile resolves, test functions counted — no CI leg runs it and it is no ceiling on every function

### unit-test-strategy#14.1 · lost at classify · .andromeda/test-plan-detail/unit-test-strategy.md:100
- (.andromeda/test-plan-detail/unit-test-strategy.md:100) A take must keep a select's submitted entries as these two tests pin them, part of the select model the resolutions leave unchanged.
  > `form.rs`'s test module pins a select's submitted entries by two tests — `a_multiple_select_submits_its_set_less_its_disabled_rows`, a `multiple` set less its disabled rows, and `a_single_select_submits_its_choice_through_a_group`, a single choice through a group — and these use neither `.unwrap()` nor `.expect(` (packages/blitz-dom/src/form.rs:558-592; packages/blitz-dom/src/form.rs:594-617)

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### ci-integration#10.1 · listed (lost at classify) · .andromeda/test-plan-detail/ci-integration.md:64
- (.andromeda/test-plan-detail/ci-integration.md:64) An edit beside the matrix's install step leaves this step timeout and the bounded-install invariant unchanged.
  > The matrix job's `Install apt deps` step, a third-party action, carries `timeout-minutes: 10`, the file's one step timeout.

### ci-integration#10.1 · listed (lost at classify) · .andromeda/test-plan-detail/ci-integration.md:65
- (.andromeda/test-plan-detail/ci-integration.md:65) An edit that shortens ci.yml at lines 392-394 corrects the citation here that stands below it, and an edit to `ci-leg.sh` keeps no leg calling the install script.
  > No leg of `ci-leg.sh` calls the script and the dev host never runs it against a package manager (.github/scripts/apt-install.sh:47-50; .github/scripts/apt-install.sh:56-65; .github/scripts/apt-install.sh:67-73; .github/workflows/ci.yml:46; .github/workflows/ci.yml:191; .github/workflows/ci.yml:409)

### mocking-stubbing-discipline#3.1 · listed (lost at strong) · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:46
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:46) No mocking library enters with the take: P3's reading of the manifests and the lock per shape repeats this search, and a crate that matches goes to the hand review the scope asks of new crates.
  > Mocking libraries in CI and build configuration · searched: `mock` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py

### test-data-fixtures#3.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-data-fixtures.md:22
- (.andromeda/test-plan-detail/test-data-fixtures.md:22) The incremental/non-incremental oracle is a named proof of the take and its inline fixture pages must read green unedited; upstream's new and edited test files under a take seed their pages the same way, or the row is restated, and the row's cites follow any file the take moves.
  > inline HTML string constants or `format!`-built pages | (tests/blitz-tests/tests/incremental_oracle.rs:371-420; tests/blitz-tests/tests/dir_attribute.rs:12-29)

### test-strategy#7.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-strategy.md:46
- (.andromeda/test-plan-detail/test-strategy.md:46) A conflict resolution or re-measure must add no skip path to a headless-stand check, which keeps asserting under the bundled font.
  > the headless-stand checks have no skip path — they assert unconditionally under the bundled DejaVu Sans `seven_guis::stand::font_ctx()` registers with system fonts off, passed through `HarnessOptions.font_ctx`

## Held

- ci-integration#1.1
- ci-integration#11.1
- ci-integration#12.1
- ci-integration#13.1
- ci-integration#14.1
- ci-integration#2.1
- ci-integration#4.1
- ci-integration#5.1
- ci-integration#6.1
- ci-integration#7.1
- ci-integration#8.1
- ci-integration#9.1
- e2e-test-strategy#1.1
- e2e-test-strategy#1.2
- e2e-test-strategy#1.3
- e2e-test-strategy#2.1
- e2e-test-strategy#3.1
- integration-test-strategy#1.1
- integration-test-strategy#10.1
- integration-test-strategy#2.1
- integration-test-strategy#3.1
- integration-test-strategy#4.1
- integration-test-strategy#4.3
- integration-test-strategy#5.1
- integration-test-strategy#6.1
- integration-test-strategy#7.1
- integration-test-strategy#8.1
- integration-test-strategy#9.1
- mocking-stubbing-discipline#1.1
- mocking-stubbing-discipline#1.12
- mocking-stubbing-discipline#1.20
- mocking-stubbing-discipline#1.3
- mocking-stubbing-discipline#1.6
- mocking-stubbing-discipline#1.7
- mocking-stubbing-discipline#1.8
- mocking-stubbing-discipline#1.9
- quality-gates-coverage-targets#2.1
- quality-gates-coverage-targets#3.1
- test-data-fixtures#2.1
- test-data-fixtures#3.15
- test-data-fixtures#3.16
- test-data-fixtures#3.2
- test-data-fixtures#3.24
- test-data-fixtures#3.3
- test-data-fixtures#3.4
- test-data-fixtures#3.5
- test-scope-summary#1.1
- test-scope-summary#10.1
- test-scope-summary#14.1
- test-scope-summary#16.1
- test-scope-summary#17.1
- test-scope-summary#17.3
- test-scope-summary#19.1
- test-scope-summary#2.1
- test-scope-summary#2.4
- test-scope-summary#20.1
- test-scope-summary#21.1
- test-scope-summary#24.1
- test-scope-summary#28.1
- test-scope-summary#29.1
- test-scope-summary#3.1
- test-scope-summary#34.1
- test-scope-summary#35.1
- test-scope-summary#5.1
- test-scope-summary#7.1
- test-scope-summary#9.1
- test-scope-summary#9.2
- test-scope-summary#9.4
- test-strategy#1.1
- test-strategy#2.1
- test-strategy#3.1
- test-strategy#5.1
- test-strategy#6.1
- test-strategy#9.1
- unit-test-strategy#1.1
- unit-test-strategy#11.1
- unit-test-strategy#12.1
- unit-test-strategy#13.1
- unit-test-strategy#16.3
- unit-test-strategy#3.1
- unit-test-strategy#7.1
- unit-test-strategy#8.1

## Cascade only

- test-data-fixtures#3.23
- test-data-fixtures#3.8
- test-scope-summary#11.1
- test-scope-summary#12.1
- test-scope-summary#13.1
- test-scope-summary#15.1
- test-scope-summary#17.2
- test-scope-summary#2.3
- test-scope-summary#22.1
- test-scope-summary#23.1
- test-scope-summary#25.1
- test-scope-summary#26.1
- test-scope-summary#27.1
- test-scope-summary#30.1
- test-scope-summary#31.1
- test-scope-summary#33.1
- test-scope-summary#34.2
- test-scope-summary#34.4
- test-scope-summary#36.1
- test-scope-summary#4.1
- test-scope-summary#9.3
- test-strategy#10.1
- test-strategy#4.1
- unit-test-strategy#16.4
