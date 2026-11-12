@row_add n=1 row=contract/wpt-runner-outputs/output-directory area=standard-contracts rec=1
boundary: WPT runner outputs
item: output directory
promise: The WPT runner writes its outputs to the directory `output` under the parent of its manifest directory, recreated on each run.
@row_add n=2 row=contract/wpt-runner-outputs/wpt-expectations-txt area=standard-contracts rec=1
boundary: WPT runner outputs
item: wpt_expectations.txt
promise: `wpt_expectations.txt` holds one line per test, with one character per subtest: `Y`, `N`, `T` or `.`.
@row_add n=3 row=contract/wpt-runner-outputs/wptreport-json area=standard-contracts rec=1
boundary: WPT runner outputs
item: wptreport.json
promise: `wptreport.json` is a `WptReport` whose run_info names product `blitz`, takes its revisions from git HEAD, and sets automation and headless true.
@row_add n=4 row=contract/wpt-runner-outputs/reftest-images area=standard-contracts rec=1
boundary: WPT runner outputs
item: reftest images
promise: A reftest writes `{test}-test.png`, `{test}-ref{suffix}.png` and `{test}-diff{suffix}.png`.
@row_add n=5 row=contract/wpt-page-messages/wpt-results area=standard-contracts rec=1
boundary: WPT page messages
item: wpt_results
promise: A WPT page reports its results with the message `wpt_results` sent through `__blitz_send_message`.
@row_add n=6 row=contract/wpt-page-messages/unsupported-feature area=standard-contracts rec=1
boundary: WPT page messages
item: unsupported_feature
promise: A WPT page reports an unsupported feature with the message `unsupported_feature` sent through `__blitz_send_message`.
@row_add n=7 row=contract/wpt-harness-codes/0 area=standard-contracts rec=1
boundary: WPT harness status codes
item: 0
promise: Harness code 0 means OK.
@row_add n=8 row=contract/wpt-harness-codes/1 area=standard-contracts rec=1
boundary: WPT harness status codes
item: 1
promise: Harness code 1 means ERROR.
@row_add n=9 row=contract/wpt-harness-codes/2 area=standard-contracts rec=1
boundary: WPT harness status codes
item: 2
promise: Harness code 2 means TIMEOUT.
@row_add n=10 row=contract/wpt-harness-codes/3 area=standard-contracts rec=1
boundary: WPT harness status codes
item: 3
promise: Harness code 3 means PRECONDITION_FAILED.
@row_add n=11 row=contract/wpt-subtest-codes/0 area=standard-contracts rec=1
boundary: WPT subtest status codes
item: 0
promise: Subtest code 0 means PASS.
@row_add n=12 row=contract/wpt-subtest-codes/1 area=standard-contracts rec=1
boundary: WPT subtest status codes
item: 1
promise: Subtest code 1 means FAIL.
@row_add n=13 row=contract/wpt-subtest-codes/2 area=standard-contracts rec=1
boundary: WPT subtest status codes
item: 2
promise: Subtest code 2 means TIMEOUT.
@row_add n=14 row=contract/wpt-subtest-codes/3 area=standard-contracts rec=1
boundary: WPT subtest status codes
item: 3
promise: Subtest code 3 means NOTRUN.
@row_add n=15 row=contract/wpt-subtest-codes/4 area=standard-contracts rec=1
boundary: WPT subtest status codes
item: 4
promise: Subtest code 4 means PRECONDITION_FAILED.
@row_add n=16 row=contract/wpt-script-fetcher/testharnessreport-js area=standard-contracts rec=1
boundary: WPT script fetcher
item: /resources/testharnessreport.js
promise: The WPT script fetcher serves its own custom `/resources/testharnessreport.js`.
@row_add n=17 row=contract/wpt-script-fetcher/testdriver-vendor-js area=standard-contracts rec=1
boundary: WPT script fetcher
item: /resources/testdriver-vendor.js
promise: The WPT script fetcher serves its own custom `/resources/testdriver-vendor.js`.
@row_add n=18 row=contract/wpt-script-fetcher/webidlparser-js area=standard-contracts rec=1
boundary: WPT script fetcher
item: WebIDLParser.js
promise: The WPT script fetcher rewrites `WebIDLParser.js`.
@row_add n=19 row=contract/wpt-attribute-tests/checklayout-attributes area=standard-contracts rec=1
boundary: WPT attribute tests
item: checkLayout attributes
promise: The WPT runner evaluates checkLayout attributes natively.
@row_add n=20 row=contract/wpt-reftests/meta-name-fuzzy area=standard-contracts rec=1
boundary: WPT reftests
item: <meta name=fuzzy>
promise: A reftest's `<meta name=fuzzy>` follows the fuzzy grammar, and a per-reference fuzzy spec wins.
@row_add n=21 row=contract/wpt-result-line/format area=standard-contracts rec=1
boundary: WPT runner result line
item: format
promise: Each WPT result line reads `STATUS (pass/total) name (Nms) KIND` followed by its flags.
@row_add n=22 row=contract/wpt-result-kinds/ref area=standard-contracts rec=1
boundary: WPT runner result kinds
item: REF
promise: REF is one of the closed set of test kinds a WPT result line shows.
@row_add n=23 row=contract/wpt-result-kinds/att area=standard-contracts rec=1
boundary: WPT runner result kinds
item: ATT
promise: ATT is one of the closed set of test kinds a WPT result line shows.
@row_add n=24 row=contract/wpt-result-kinds/cra area=standard-contracts rec=1
boundary: WPT runner result kinds
item: CRA
promise: CRA is one of the closed set of test kinds a WPT result line shows.
@row_add n=25 row=contract/wpt-result-kinds/har area=standard-contracts rec=1
boundary: WPT runner result kinds
item: HAR
promise: HAR is one of the closed set of test kinds a WPT result line shows.
@row_add n=26 row=contract/wpt-result-kinds/unk area=standard-contracts rec=1
boundary: WPT runner result kinds
item: UNK
promise: UNK is one of the closed set of test kinds a WPT result line shows.
@row_add n=27 row=contract/wpt-result-statuses/pass area=standard-contracts rec=1
boundary: WPT runner result statuses
item: PASS
promise: PASS is one of the closed set of statuses a WPT result line shows.
@row_add n=28 row=contract/wpt-result-statuses/fail area=standard-contracts rec=1
boundary: WPT runner result statuses
item: FAIL
promise: FAIL is one of the closed set of statuses a WPT result line shows.
@row_add n=29 row=contract/wpt-result-statuses/timeout area=standard-contracts rec=1
boundary: WPT runner result statuses
item: TIMEOUT
promise: TIMEOUT is one of the closed set of statuses a WPT result line shows.
@row_add n=30 row=contract/wpt-result-statuses/skip area=standard-contracts rec=1
boundary: WPT runner result statuses
item: SKIP
promise: SKIP is one of the closed set of statuses a WPT result line shows.
@row_add n=31 row=contract/wpt-result-statuses/crash area=standard-contracts rec=1
boundary: WPT runner result statuses
item: CRASH
promise: CRASH is one of the closed set of statuses a WPT result line shows.
@row_add n=32 row=contract/wpt-result-flags/letters area=standard-contracts rec=1
boundary: WPT runner result flags
item: flag letters
promise: The flags a WPT result line carries are drawn from the letters F, I, C, D, W, S, M and X.
@row_add n=33 row=contract/wpt-net-provider/data-urls area=standard-contracts rec=1
boundary: WPT runner network provider
item: data: URLs
promise: The WPT network provider decodes `data:` URLs itself.
@row_add n=34 row=contract/wpt-net-provider/other-paths area=standard-contracts rec=1
boundary: WPT runner network provider
item: other paths
promise: The WPT network provider resolves every path other than `data:` against the WPT base directory on disk.
@row_add n=35 row=contract/wpt-test-documents/base-url area=standard-contracts rec=1
boundary: WPT test documents
item: base URL
promise: WPT test documents use a base URL under `http://dummy.local`, with a dummy navigation provider and the HTML provider.
@row_add n=36 row=contract/wpt-diff-json-entry/test area=standard-contracts rec=2
boundary: wpt diff JSON entry
item: test
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `test`.
@row_add n=37 row=contract/wpt-diff-json-entry/kind area=standard-contracts rec=2
boundary: wpt diff JSON entry
item: kind
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `kind`.
@row_add n=38 row=contract/wpt-diff-json-entry/status area=standard-contracts rec=2
boundary: wpt diff JSON entry
item: status
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `status`.
@row_add n=39 row=contract/wpt-diff-json-entry/counts area=standard-contracts rec=2
boundary: wpt diff JSON entry
item: counts
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `counts`.
@row_add n=40 row=contract/wpt-diff-json-entry/before area=standard-contracts rec=2
boundary: wpt diff JSON entry
item: before
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `before`.
@row_add n=41 row=contract/wpt-diff-json-entry/after area=standard-contracts rec=2
boundary: wpt diff JSON entry
item: after
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `after`.
@row_add n=42 row=contract/wpt-diff-json-entry/counts-before area=standard-contracts rec=2
boundary: wpt diff JSON entry
item: counts_before
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `counts_before`.
@row_add n=43 row=contract/wpt-diff-json-entry/counts-after area=standard-contracts rec=2
boundary: wpt diff JSON entry
item: counts_after
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `counts_after`.
@row_add n=44 row=contract/pr-body-wpt-section/start-marker area=standard-contracts rec=2
boundary: PR body WPT results section
item: <!-- wpt-results-start -->
promise: The WPT results section of a PR body opens with `<!-- wpt-results-start -->`, and the section is replaced on re-runs.
@row_add n=45 row=contract/pr-body-wpt-section/end-marker area=standard-contracts rec=2
boundary: PR body WPT results section
item: <!-- wpt-results-end -->
promise: The WPT results section of a PR body closes with `<!-- wpt-results-end -->`, and the section is replaced on re-runs.
@row_add n=46 row=contract/wpt-diff-to-pr-cli/diff-file area=standard-contracts rec=2
boundary: wpt_diff_to_pr.py command line
item: diff_file
promise: The PR-results script takes the diff file as its positional argument `diff_file`.
@row_add n=47 row=contract/wpt-diff-to-pr-cli/repo area=standard-contracts rec=2
boundary: wpt_diff_to_pr.py command line
item: --repo
promise: The PR-results script accepts `--repo`.
@row_add n=48 row=contract/wpt-diff-to-pr-cli/pr area=standard-contracts rec=2
boundary: wpt_diff_to_pr.py command line
item: --pr
promise: The PR-results script accepts `--pr`.
@row_add n=49 row=contract/wpt-diff-to-pr-cli/run-url area=standard-contracts rec=2
boundary: wpt_diff_to_pr.py command line
item: --run-url
promise: The PR-results script accepts `--run-url`.
@row_add n=50 row=contract/wpt-diff-to-pr-cli/areas area=standard-contracts rec=2
boundary: wpt_diff_to_pr.py command line
item: --areas
promise: The PR-results script's `--areas` names the per-area subtest changes that upstream's area-changes script computes from the two reports.
@row_add n=51 row=contract/wpt-diff-to-pr-cli/dry-run area=standard-contracts rec=2
boundary: wpt_diff_to_pr.py command line
item: --dry-run
promise: The PR-results script accepts `--dry-run`.
@row_add n=52 row=contract/wpt-diff-to-pr-output/passing-statuses area=standard-contracts rec=2
boundary: wpt_diff_to_pr.py output
item: passing statuses
promise: The PR-results script counts both "PASS" and "OK" as passing.
@row_add n=53 row=contract/wpt-diff-to-pr-output/listing-cap area=standard-contracts rec=2
boundary: wpt_diff_to_pr.py output
item: listing cap
promise: The PR-results script caps its listing at 400 lines.
@row_add n=54 row=contract/wpt-diff-to-pr-output/step-summary area=standard-contracts rec=2
boundary: wpt_diff_to_pr.py output
item: GITHUB_STEP_SUMMARY
promise: The PR-results script appends its section to `GITHUB_STEP_SUMMARY`.
@row_add n=55 row=contract/wpt-diff-to-pr-output/pr-body area=standard-contracts rec=2
boundary: wpt_diff_to_pr.py output
item: PR body
promise: The PR-results script PATCHes the PR body through `gh api`.
@row_add n=56 row=contract/wpt-ci-artifacts/wpt-report-json-zst area=standard-contracts rec=2
boundary: WPT CI artifacts
item: wpt-report.json.zst
promise: The WPT workflow publishes the report as the artifact `wpt-report.json.zst`.
@row_add n=57 row=contract/wpt-ci-artifacts/wpt-diff area=standard-contracts rec=2
boundary: WPT CI artifacts
item: wpt-diff
promise: The WPT workflow publishes the diff as the artifact `wpt-diff`, which the post-results workflow reads.
@row_add n=58 row=contract/wpt-repository-dispatch/update-results area=standard-contracts rec=2
boundary: WPT repository dispatch
item: update-results
promise: After WPT on main, a `repository-dispatch` of `update-results` goes to `DioxusLabs/blitz-wpt-results`, only when the repository is `DioxusLabs/blitz`.
@row_add n=59 row=contract/ci-leg-legs/embolden area=standard-contracts rec=2,3
boundary: ci-leg.sh legs
item: embolden
promise: The `embolden` leg runs `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, has no CI job, and is the last step of `fast`.
@row_add n=60 row=contract/ci-leg-legs/fmt area=standard-contracts rec=2
boundary: ci-leg.sh legs
item: fmt
promise: The `fmt` leg runs `cargo fmt --all --check`.
@row_add n=61 row=contract/ci-leg-legs/clippy area=standard-contracts rec=2
boundary: ci-leg.sh legs
item: clippy
promise: The `clippy` leg runs the workspace clippy line and, only when that is green, upstream's own blitz-dom clippy line with `-D warnings`.
@row_add n=62 row=contract/ci-leg-legs/test area=standard-contracts rec=2
boundary: ci-leg.sh legs
item: test
promise: `test` is a leg of the CI leg script.
@row_add n=63 row=contract/ci-leg-legs/ci-scripts area=standard-contracts rec=2
boundary: ci-leg.sh legs
item: ci-scripts
promise: `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts.
@row_add n=64 row=contract/ci-leg-legs/build area=standard-contracts rec=2
boundary: ci-leg.sh legs
item: build
promise: `build` is a leg of the CI leg script.
@row_add n=65 row=contract/ci-leg-legs/msrv area=standard-contracts rec=2
boundary: ci-leg.sh legs
item: msrv
promise: The `msrv` leg runs `cargo +1.91 build`.
@row_add n=66 row=contract/ci-leg-legs/counter area=standard-contracts rec=2
boundary: ci-leg.sh legs
item: counter
promise: `counter` is a leg of the CI leg script.
@row_add n=67 row=contract/ci-leg-legs/wasm area=standard-contracts rec=2
boundary: ci-leg.sh legs
item: wasm
promise: `wasm` is a leg of the CI leg script.
@row_add n=68 row=contract/ci-leg-legs/doc area=standard-contracts rec=2
boundary: ci-leg.sh legs
item: doc
promise: The `doc` leg runs `cargo doc --workspace --no-deps`.
@row_add n=69 row=contract/ci-leg-legs/audit area=standard-contracts rec=2
boundary: ci-leg.sh legs
item: audit
promise: The `audit` leg runs `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace's graph.
@row_add n=70 row=contract/ci-leg-legs/a11y area=standard-contracts rec=2
boundary: ci-leg.sh legs
item: a11y
promise: The `a11y` leg runs fourteen named accessibility integration tests on one `cargo test --workspace --locked` line; other focus and accessibility tests stay in the `test` leg.
@row_add n=71 row=contract/ci-leg-legs/coverage area=standard-contracts rec=2
boundary: ci-leg.sh legs
item: coverage
promise: The `coverage` leg runs `cargo llvm-cov` to an lcov file and its per-file report, then the coverage-floor script, and exits with the floor's status.
@row_add n=72 row=contract/ci-leg-legs/stand-repeat area=standard-contracts rec=2
boundary: ci-leg.sh legs
item: stand-repeat
promise: The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
@row_add n=73 row=contract/ci-leg-legs/fast area=standard-contracts rec=2
boundary: ci-leg.sh legs
item: fast
promise: The `fast` leg runs fmt, clippy, test, ci-scripts and embolden in order, stopping at the first red: the local pre-push gate.
@row_add n=74 row=contract/ci-leg-script/leg-log area=standard-contracts rec=2
boundary: ci-leg.sh invocation
item: leg log
promise: The CI leg script tees the leg's merged output to `target/ci-logs/{leg}.log`, truncated at the leg's start.
@row_add n=75 row=contract/ci-leg-script/exit-status area=standard-contracts rec=2
boundary: ci-leg.sh invocation
item: exit status
promise: The CI leg script exits with the leg command's status.
@row_add n=76 row=contract/ci-leg-script/unknown-leg area=standard-contracts rec=2
boundary: ci-leg.sh invocation
item: unknown or missing leg
promise: On an unknown or missing leg the CI leg script prints the leg list to stderr and exits 2.
@row_add n=77 row=decision/cargo-legs-locked area=standard-contracts rec=2
name: Cargo legs run locked
rule: Every cargo leg of the CI leg script runs with `--locked`.
except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock`
@row_add n=78 row=decision/stand-repeat-zero-failure-budget area=standard-contracts rec=2
name: Stand-repeat zero-failure budget
rule: The stand-repeat leg ends on the first red command with its status: no retry, no `--ignored`, no environment variable; three green rounds are needed.
guard: test_stand_repeat_leg_runs_three_rounds
@row_add n=79 row=decision/coverage-excludes-keyboard-compat area=standard-contracts rec=2
name: Coverage excludes the keyboard compat bridge
rule: Both coverage measurement lines exclude dioxus-native-dom's crate-private keyboard compat module, the floors unchanged.
why: The module is upstream's temporary keyboard-types 0.7/0.8 bridge, excluded until escher takes upstream's retirement of it.
guard: test_coverage_leg_reaches_the_floor_on_each_crate
@row_add n=80 row=decision/a11y-leg-widens-without-new-job area=standard-contracts rec=2
name: Accessibility leg widens in place
rule: The a11y leg gains tests with no new job, no required status check and no workflow change.
guard: LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log
@row_add n=81 row=contract/ci-log-artifacts/ci-log area=standard-contracts rec=2
boundary: fork CI log artifacts
item: ci-log-{job id}
promise: Every CI leg job uploads, on failure only, its own leg log as `ci-log-{job id}` (matrix `ci-log-matrix-{platform}`), kept 7 days, ignoring a missing file.
@row_add n=82 row=contract/ci-log-artifacts/coverage-report area=standard-contracts rec=2
boundary: fork CI log artifacts
item: coverage-report
promise: The coverage job alone also uploads `coverage-report` on success, kept 7 days: the one fork-CI artifact outside the CI log directory.
@row_add n=83 row=decision/ci-contract-pinned area=standard-contracts rec=2
name: CI workflow contract pinned by tests
rule: The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg.
guard: test_ci_workflows.py
@row_add n=84 row=decision/bounded-package-install area=standard-contracts rec=2
name: Bounded package install
rule: Every fork CI package install goes through the one install script: up to 3 attempts, each update and install under a 120-second bound with a kill grace, 10 s pause between attempts.
guard: InstallScriptTest; test_o_package_installs_are_bounded
@row_add n=85 row=decision/package-install-repair-before-retry area=standard-contracts rec=2
name: Repair before a package install retry
rule: Every attempt after the first starts with a bounded `dpkg --configure -a` repair, and a failed repair fails that attempt.
why: A bound that fires inside an install can leave dpkg interrupted, which the next attempt's repair configures.
guard: InstallScriptTest
@row_add n=86 row=contract/apt-install-exit-codes/0 area=standard-contracts rec=2
boundary: apt-install.sh exit codes
item: 0
promise: An attempt completed.
@row_add n=87 row=contract/apt-install-exit-codes/1 area=standard-contracts rec=2
boundary: apt-install.sh exit codes
item: 1
promise: Every attempt failed; the bound's own 124 or 137 is never returned.
@row_add n=88 row=contract/apt-install-exit-codes/2 area=standard-contracts rec=2
boundary: apt-install.sh exit codes
item: 2
promise: Usage error, found before any command runs.
@row_add n=89 row=contract/apt-install-stderr/attempt-failed-line area=standard-contracts rec=2
boundary: apt-install.sh stderr
item: attempt failed line
promise: A failed call prints one stderr line `apt-install: attempt {n} of {N} failed in {repair|update|install} (exit {status})`.
@row_add n=90 row=contract/apt-install-arguments/package-name area=standard-contracts rec=2
boundary: apt-install.sh arguments
item: package name
promise: A package name must match `^[a-z0-9][a-z0-9+.-]+$`, and at least one package is named.
@row_add n=91 row=contract/apt-install-arguments/option-value area=standard-contracts rec=2
boundary: apt-install.sh arguments
item: option value
promise: An option value must match `^(0|[1-9][0-9]{0,5})$`, with `--attempts` and `--bound` at least 1.
@row_add n=92 row=contract/coverage-floor-output/crate-line area=standard-contracts rec=2
boundary: coverage_floor.py output
item: crate line
promise: The coverage-floor script prints one line per crate, `{dir} {hit}/{lines} = {pct:.2f} % (floor {f} %)`, naming no source path but the crate directory.
@row_add n=93 row=contract/coverage-floor-exit-codes/0 area=standard-contracts rec=2
boundary: coverage_floor.py exit codes
item: 0
promise: Every crate is at or above its floor, printed as `coverage floor: N of N crates at or above their floors`.
@row_add n=94 row=contract/coverage-floor-exit-codes/1 area=standard-contracts rec=2
boundary: coverage_floor.py exit codes
item: 1
promise: Some crates are under their floors, printed as `coverage floor: K of N crates under their floors`.
@row_add n=95 row=contract/coverage-floor-exit-codes/2 area=standard-contracts rec=2
boundary: coverage_floor.py exit codes
item: 2
promise: A crate has no lcov record (its directory named on stderr) or an argument is malformed (a usage line).
@row_add n=96 row=decision/coverage-floors area=standard-contracts rec=2
name: Per-crate coverage floors
rule: CI fails when line coverage falls under 97 % for escher-driver, 83 % for dioxus-native-dom or 90 % for blitz-test-harness.
guard: CoverageFloorTest
@row_add n=97 row=contract/agent-run-exit-codes/0 area=standard-contracts rec=2
boundary: agent-run.sh exit codes
item: 0
promise: Success.
@row_add n=98 row=contract/agent-run-exit-codes/1 area=standard-contracts rec=2
boundary: agent-run.sh exit codes
item: 1
promise: Failed, an empty run included.
@row_add n=99 row=contract/agent-run-exit-codes/2 area=standard-contracts rec=2
boundary: agent-run.sh exit codes
item: 2
promise: Usage error, checked before anything else.
@row_add n=100 row=contract/agent-run-exit-codes/3 area=standard-contracts rec=2
boundary: agent-run.sh exit codes
item: 3
promise: A precondition is unmet.
@row_add n=101 row=contract/agent-run-events/boot area=standard-contracts rec=2
boundary: agent-run.sh JSON-line events
item: boot
promise: `boot` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_add n=102 row=contract/agent-run-events/run-start area=standard-contracts rec=2
boundary: agent-run.sh JSON-line events
item: run.start
promise: `run.start` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_add n=103 row=contract/agent-run-events/test area=standard-contracts rec=2
boundary: agent-run.sh JSON-line events
item: test
promise: `test` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_add n=104 row=contract/agent-run-events/run-end area=standard-contracts rec=2
boundary: agent-run.sh JSON-line events
item: run.end
promise: `run.end` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_add n=105 row=contract/agent-run-events/status area=standard-contracts rec=2
boundary: agent-run.sh JSON-line events
item: status
promise: `status` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_add n=106 row=contract/agent-run-events/cleanup area=standard-contracts rec=2
boundary: agent-run.sh JSON-line events
item: cleanup
promise: `cleanup` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_add n=107 row=contract/agent-run-state/state-directory area=standard-contracts rec=2
boundary: agent-run.sh state
item: state directory
promise: The agent-run test pipe keeps its state in `target/agent-run/` only.
@row_add n=108 row=decision/windows-entries-pass-through area=standard-contracts rec=2
name: Windows script entries are pass-throughs
rule: The PowerShell entries of the agent-run and cold-agent pipes hold no logic and only pass through to the bash scripts.
@row_add n=109 row=contract/cold-agent-cli/run-task area=standard-contracts rec=2
boundary: cold-agent.sh command line
item: run <task>
promise: The cold-agent pipe's `run` takes the one task `stand`.
@row_add n=110 row=contract/cold-agent-exit-codes/0 area=standard-contracts rec=2
boundary: cold-agent.sh exit codes
item: 0
promise: Success.
@row_add n=111 row=contract/cold-agent-exit-codes/1 area=standard-contracts rec=2
boundary: cold-agent.sh exit codes
item: 1
promise: A `failed` verdict.
@row_add n=112 row=contract/cold-agent-exit-codes/2 area=standard-contracts rec=2
boundary: cold-agent.sh exit codes
item: 2
promise: Usage error, checked before any precondition.
@row_add n=113 row=contract/cold-agent-exit-codes/3 area=standard-contracts rec=2
boundary: cold-agent.sh exit codes
item: 3
promise: A precondition is unmet (`claude` or `python3` missing, or the escher-session binary absent), with one stderr line naming the build command, no verdict and no events file.
@row_add n=114 row=contract/cold-agent-events/run-start area=standard-contracts rec=2
boundary: cold-agent.sh JSON-line events
item: run.start
promise: `run.start` is one of the closed set of JSON-line events the cold-agent pipe emits.
@row_add n=115 row=contract/cold-agent-events/run-end area=standard-contracts rec=2
boundary: cold-agent.sh JSON-line events
item: run.end
promise: `run.end` is one of the closed set of JSON-line events the cold-agent pipe emits.
@row_add n=116 row=contract/cold-agent-events/status area=standard-contracts rec=2
boundary: cold-agent.sh JSON-line events
item: status
promise: `status` is one of the closed set of JSON-line events the cold-agent pipe emits.
@row_add n=117 row=contract/cold-agent-events/cleanup area=standard-contracts rec=2
boundary: cold-agent.sh JSON-line events
item: cleanup
promise: `cleanup` is one of the closed set of JSON-line events the cold-agent pipe emits.
@row_add n=118 row=contract/cold-agent-state/state-directory area=standard-contracts rec=2
boundary: cold-agent.sh state
item: state directory
promise: The cold-agent pipe keeps its state in `target/cold-agent/` only.
@row_add n=119 row=decision/cold-agent-isolated-session area=standard-contracts rec=2
name: Cold-agent run is one isolated session
rule: A cold-agent run starts one isolated non-interactive Claude session from a fresh temporary directory with a fixed prompt and only the escher server's tools, under a budget and time cap.
why: ratified
guard: test_cold_agent.py
@row_add n=120 row=contract/cold-agent-verdict/passed area=standard-contracts rec=2
boundary: cold-agent verdict.json
item: passed
promise: The verdict is `passed` only on positive evidence: client exit 0, isolation, matching tool-call counts, all six apps met, and the agent's final check replaying passing and failing on an ablated replay.
@row_add n=121 row=decision/blitz-tests-single-test-target area=standard-contracts rec=2
name: blitz-tests manifest declares one test table
rule: The blitz-tests package keeps test autodiscovery on and declares exactly one test table, `all`, with test set to false.
guard: BlitzTestsTargetsTest
@row_add n=122 row=decision/capability-statement-matches-schema area=standard-contracts rec=2
name: Capability statement held to the driver schema
rule: The capability statement and README name every verb and refusal cause and state in words the counts of verbs, refusal causes, session errors, node keys and exit statuses that the driver schema holds.
guard: CapabilityStatementTest
@row_add n=123 row=contract/root-crate-features/log-times area=standard-contracts rec=2
boundary: root crate features
item: log-times
promise: The feature `log-times` enables `log-frame-times` and `log-phase-times`, forwarding to dioxus-native.
@row_add n=124 row=decision/font-embolden-local-witness area=standard-contracts rec=3,5
name: Local witness for font-embolden paint
rule: The font-embolden feature is built and tested locally by the embolden step of the fast gate, with no CI job; macOS exercises the feature in CI.
why: Running the painted select-popup test under the feature on linux reproduces macOS's two readings.
guard: test_fast_leg_ends_on_the_font_embolden_witness
@row_add n=125 row=decision/only-stand-repeat-frees-disk area=standard-contracts rec=4
name: Only stand-repeat frees disk
rule: Only the stand-repeat CI job frees disk.
guard: test_only_stand_repeat_frees_disk
@row_add n=126 row=decision/cold-agent-six-apps area=standard-contracts rec=6
name: Cold-agent prompt names six apps
rule: The cold-agent prompt names six apps (counter, flight booker, timer, CRUD, cells and the load page) and their goals, naming no stand id.
@row_add n=127 row=contract/cold-agent-recording/live-server area=standard-contracts rec=6
boundary: cold-agent recording
item: live server child
promise: The cold-agent live server child records into the owner-only `target/cold-agent/record/`; its two replay children run without recording.
@row_add n=128 row=decision/cold-agent-wrong-call-bar area=standard-contracts rec=6,2
name: Cold-agent wrong-call bar
rule: A cold-agent run passes only when wrong calls times 100 is at most 5 times the tool calls.
@row_add n=129 row=contract/cold-agent-verdict-reasons/capability-unused area=standard-contracts rec=6
boundary: cold-agent verdict reasons
item: capability-unused
promise: The verdict fails with `capability-unused` unless `lint`, `screenshot` and `advance` are each used and a part is read.
@row_add n=130 row=contract/cold-agent-verdict-reasons/recording-not-kept area=standard-contracts rec=6
boundary: cold-agent verdict reasons
item: recording-not-kept
promise: The verdict fails with `recording-not-kept` unless one check is kept per accepted `start`.
@row_add n=131 row=contract/cold-agent-verdict-reasons/wrong-calls-over-bar area=standard-contracts rec=6
boundary: cold-agent verdict reasons
item: wrong-calls-over-bar
promise: The verdict fails with `wrong-calls-over-bar` when wrong calls exceed 5 % of tool calls; the three reasons come after `task-unmet` and before the check's.
@row_add n=132 row=contract/cold-agent-verdict/wrong-call-bar-pct area=standard-contracts rec=6
boundary: cold-agent verdict.json
item: wrong_call_bar_pct
promise: The verdict carries `wrong_call_bar_pct`, 5.
@row_add n=133 row=contract/cold-agent-verdict/lint-calls area=standard-contracts rec=6
boundary: cold-agent verdict.json
item: lint_calls
promise: The verdict carries `lint_calls`.
@row_add n=134 row=contract/cold-agent-verdict/screenshot-calls area=standard-contracts rec=6
boundary: cold-agent verdict.json
item: screenshot_calls
promise: The verdict carries `screenshot_calls`.
@row_add n=135 row=contract/cold-agent-verdict/advance-calls area=standard-contracts rec=6
boundary: cold-agent verdict.json
item: advance_calls
promise: The verdict carries `advance_calls`.
@row_add n=136 row=contract/cold-agent-verdict/part-reads area=standard-contracts rec=6
boundary: cold-agent verdict.json
item: part_reads
promise: The verdict carries `part_reads`.
@row_add n=137 row=contract/cold-agent-verdict/recorded-sessions area=standard-contracts rec=6
boundary: cold-agent verdict.json
item: recorded_sessions
promise: The verdict carries `recorded_sessions`.
@row_add n=138 row=contract/cold-agent-verdict/recording-kept area=standard-contracts rec=6
boundary: cold-agent verdict.json
item: recording_kept
promise: The verdict carries `recording_kept`.
