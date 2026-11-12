@row_none n=1 rec=1 type=S
@row_none n=2 rec=2 type=S
@row_add n=1 row=decision/wpt-reftests-thai-font area=6-e2e-test-strategy rec=3
name: WPT reftests run with a Thai font
rule: WPT reftests are run with a Thai font installed.
why: some reftests depend on Thai glyph widths
@row_add n=2 row=decision/wpt-upstream-only area=6-e2e-test-strategy rec=3,55
name: WPT workflow on upstream only
rule: The WPT workflow, its runs and its posting of results, runs only on the upstream DioxusLabs/blitz repository, its jobs repository-guarded.
@row_none n=3 rec=4 type=5
@row_none n=4 rec=5 type=5
@row_add n=3 row=decision/unscrolled-containers-no-scrollbar-thumbs area=6-e2e-test-strategy rec=6
name: Never-scrolled containers paint no scrollbar thumbs
rule: A container that has never been scrolled paints no scrollbar thumbs.
why: keeps scrollbar thumbs out of static reftest screenshots
@row_add n=4 row=decision/cli-e2e-through-own-binary area=6-e2e-test-strategy rec=7
name: Command-line E2E through the package's own binary
rule: Each escher-session driver command is tested as a process of the package's own binary against a host its start command spawned.
@row_add n=5 row=decision/checks-byte-equal-to-recording area=6-e2e-test-strategy rec=7,22
name: Committed checks byte-equal to the binary's recording
rule: The committed checks are recorded by escher-session and read byte-equal to what the binary records.
@row_add n=6 row=decision/checks-hold-no-screenshot-step area=6-e2e-test-strategy rec=7,22
name: No screenshot step in committed checks
rule: The scripts behind the committed checks hold no screenshot step.
why: a shot's PNG bytes differ by OS
@row_add n=7 row=decision/mcp-tested-as-child-and-in-process area=6-e2e-test-strategy rec=8
name: MCP role tested as a child and in process
rule: The binary's mcp role is tested as a child over piped stdin through the shared Speaker and in process in both layout modes.
@row_add n=8 row=decision/real-mcp-client-outside-ci area=6-e2e-test-strategy rec=8
name: Real MCP client only in the operator pass
rule: A real MCP client drives escher-session mcp in the operator pass, not in CI.
@row_none n=5 rec=9 type=S
@row_add n=9 row=decision/example-fixtures-are-manual area=6-e2e-test-strategy rec=10,11,15
name: Example fixtures carry no automated assertion
rule: Example fixture pages state their click-through instructions and expected outcomes in text instead of automated assertions.
@row_none n=6 rec=12 type=5
@row_none n=7 rec=13 type=5
@row_none n=8 rec=14 type=S
@row_add n=10 row=decision/ui-driven-through-session-binary area=6-e2e-test-strategy rec=15
name: App UI driven through the session binary
rule: No test in the apps launches a window; the UI is driven through the escher-session binary.
@row_none n=9 rec=16 type=S
@row_none n=10 rec=17 type=S
@row_add n=11 row=decision/wpt-pinned-commit area=7-test-data-fixtures rec=18
name: WPT pinned to a recorded commit
rule: WPT tests are cloned at the commit recorded in the repository's WPT commit file.
@row_add n=12 row=contract/wpt-runner-env/wpt-dir area=7-test-data-fixtures rec=18
boundary: WPT runner environment
item: WPT_DIR
promise: The runner reads its test inputs from the WPT checkout at WPT_DIR.
@row_none n=11 rec=19 type=5
@row_none n=12 rec=20 type=5
@row_none n=13 rec=21 type=5
@row_add n=13 row=decision/checks-hold-no-private-data area=7-test-data-fixtures rec=22
name: Committed checks hold no private data
rule: The committed checks hold no typed text, host path or secret.
@row_add n=14 row=decision/shared-fixture-pages area=7-test-data-fixtures rec=23
name: Shared fixture pages stated once
rule: A fixture page that more than one check reads is stated once as an rsx component and booted with the stand's options.
except: a page with one reader stays in its check's file
@row_none n=14 rec=24 type=S
@row_add n=15 row=decision/test-data-in-code area=7-test-data-fixtures rec=25
name: Test data stated in code
rule: Test data is stated in code: inline HTML, literals and byte arrays, directly injected images, fixed viewports, embedded fonts and pages generated in the test.
except: one external benchmark page supplied through PAINT_TREE_BENCH_HTML
@row_none n=15 rec=26 type=S
@row_none n=16 rec=27 type=5
@row_none n=17 rec=28 type=5
@row_none n=18 rec=29 type=5
@row_none n=19 rec=30 type=5
@row_none n=20 rec=31 type=4
relied: Harness documents always use HtmlProvider, and only a fetching net_provider is wrapped in the load counter that settle reads.
@row_none n=21 rec=32 type=4
relied: Documents the harness builds read the harness's virtual clock, while a wrapped document keeps the clock its own config gave it.
@row_add n=16 row=decision/stand-boots-offline area=7-test-data-fixtures rec=33,39
name: Stand boots offline
rule: Every headless stand boot runs with no network provider.
guard: the_stand_options_stay_offline
except: the stand load page's boot, which supplies the self-answering LoadNet stand-in
@row_none n=22 rec=34 type=S
@row_none n=23 rec=35 type=7
@row_none n=24 rec=36 type=S
@row_none n=25 rec=37 type=S
@row_add n=17 row=decision/hand-written-stand-ins area=8-mocking-stubbing-discipline rec=38,45
name: Hand-written stand-ins, no mocking library
rule: Test doubles are hand-written stand-ins; no mocking library is used in CI, the build configuration or the apps.
@row_add n=18 row=decision/network-stand-ins-counted area=8-mocking-stubbing-discipline rec=39
name: Network stand-ins keep their loads counted
rule: Network stand-ins do not override is_noop, so the harness counts their loads.
@row_add n=19 row=decision/stand-timer-ticks area=8-mocking-stubbing-discipline rec=40
name: Stand timer driven by TimerTicks
rule: Every headless-stand boot provides TimerTicks in place of the timer's real delay.
@row_add n=20 row=decision/settle-never-advances-time area=8-mocking-stubbing-discipline rec=40
name: Settle never advances time
rule: Settle neither advances the timer nor waits on a tick not yet delivered.
@row_add n=21 row=contract/driver-advance/advance area=8-mocking-stubbing-discipline rec=40
boundary: the driver's advance command
item: advance
promise: Moves the session's clock by exactly ms inside one settled step.
@row_add n=22 row=contract/driver-advance/advanced-ms area=8-mocking-stubbing-discipline rec=40
boundary: the driver's advance command
item: advanced_ms
promise: Is exactly ms, the clock's move, not what the Timer's counter gained.
@row_none n=26 rec=41 type=S
@row_add n=23 row=decision/font-metric-tests-never-skip area=8-mocking-stubbing-discipline rec=42,43
name: Font-metric tests never skip
rule: Font-metric tests take no skip path and assert their precondition instead.
@row_add n=24 row=decision/stand-pinned-real-font area=8-mocking-stubbing-discipline rec=43
name: Stand checks keep a real pinned font
rule: The headless-stand checks use the bundled DejaVu Sans font, decoded through the native woff feature, with system fonts off.
@row_none n=27 rec=44 type=S
@row_none n=28 rec=46 type=S
@row_add n=25 row=decision/ci-on-github-actions area=9-ci-integration rec=47
name: CI on GitHub Actions
rule: GitHub Actions is what runs the project's CI.
@row_add n=26 row=decision/ci-trigger-branches area=9-ci-integration rec=47
name: CI trigger branches
rule: Tests run on pull requests and on pushes to main, v0.* and build/** branches, escher's build branch included.
@row_none n=29 rec=48 type=S
@row_add n=27 row=decision/ci-legs-host-reproducible area=9-ci-integration rec=49
name: Host-reproducible CI legs
rule: Each linux CI job runs one leg of the shared CI leg script, and the dev host runs the same script.
why: every linux leg is host-reproducible
@row_add n=28 row=decision/cargo-legs-locked area=9-ci-integration rec=49
name: Cargo legs run locked
rule: Every cargo leg of CI runs with --locked.
except: fmt, and examples/wasm_hello, which has no Cargo.lock
@row_add n=29 row=decision/stand-repeat-three-rounds area=9-ci-integration rec=49
name: Stand checks repeated three rounds
rule: The stand-repeat leg runs the stand checks and the named process checks for three rounds, the first red command ending the leg with its status, with no retry, no --ignored and no env var.
@row_add n=30 row=decision/ci-fast-slow-split area=9-ci-integration rec=50
name: Fast/slow CI split
rule: fmt, clippy, test-features-default and ci-scripts run first with no dependencies, and every other job, the platform matrix included, needs all four.
@row_add n=31 row=decision/ci-rust-cache area=9-ci-integration rec=51
name: SHA-pinned Rust cache
rule: Every compiling CI job uses the SHA-pinned Swatinem/rust-cache.
except: coverage
@row_add n=32 row=decision/ci-cache-save-branches area=9-ci-integration rec=51
name: Cache saved on main and build branches
rule: CI caches are saved only on main and build/* branches.
@row_add n=33 row=decision/shared-test-cache area=9-ci-integration rec=51
name: Shared workspace test cache
rule: The test job saves the cache under shared-key workspace-test, which the a11y and stand-repeat jobs restore without saving.
@row_add n=34 row=contract/ci-artifacts/leg-log area=9-ci-integration rec=52
boundary: CI logs and artifacts
item: target/ci-logs/{leg}.log
promise: Holds each leg's merged output, truncated at the leg's start.
@row_add n=35 row=contract/ci-artifacts/ci-log-job-id area=9-ci-integration rec=52
boundary: CI logs and artifacts
item: ci-log-{job id}
promise: The leg's log, uploaded only on failure and kept 7 days.
@row_add n=36 row=contract/ci-artifacts/coverage-report area=9-ci-integration rec=52
boundary: CI logs and artifacts
item: coverage-report
promise: Uploaded by the coverage job alone from target/coverage/ on success, kept 7 days.
@row_add n=37 row=decision/package-installs-bounded-retried area=9-ci-integration rec=53
name: Bounded, retried package installs
rule: Every package install of the CI workflow is bounded and retried: up to 3 attempts under a 120 s bound, each retry opening with a bounded dpkg repair.
why: the bound answers a package-install hang
@row_add n=38 row=contract/apt-install-exit-status/range area=9-ci-integration rec=53
boundary: apt-install script exit status
item: 0, 1 or 2
promise: The script exits 0, 1 or 2 and never with the bound's 124 or 137.
@row_add n=39 row=decision/local-pre-push-gate area=9-ci-integration rec=54
name: Local pre-push gate
rule: The fast leg runs fmt, clippy, test, ci-scripts and embolden in that order, stopping at the first red, as the local pre-push gate.
@row_add n=40 row=decision/embolden-leg-local-only area=9-ci-integration rec=54
name: Embolden witness leg without a CI job
rule: The font-embolden witness leg runs locally and has no CI job of its own.
why: macOS exercises the feature in CI
