@task_prose part=chunk
Measures from a cold build directory what grows it: which gate entries, CI legs (fast, a11y, stand-repeat, embolden, coverage, doc, build), the agent-run harness, the cold-agent pipe and the dev-host-only mutation tier write which artifacts, and which edits mint a new metadata hash for which units, read from cargo's own output.
Names the cause as numbers: the share of re-hash per low-crate edit, differing feature sets or package selections, profile or flag differences, per-file integration-test binaries and debuginfo weight.
Re-measures the overseer's figures (773 GB in deps, 193 test binaries of about 145 MB in 30-65 stale hashes) as hypotheses, not premises.
Applies one mend the measurement points at (one test binary per leg, a stale-hash sweep at a wrap or boundary, less debuginfo for test binaries, or another), with every check still green.
States a working day's growth after the mend in gigabytes, read before and after on the dev host (method manual).
@task_file n=1 path=Cargo.toml mark=edits
what: the workspace manifest and its build profiles — edits: the [profile.*] tables (dev today line-tables-only debuginfo)
@task_file n=2 path=.cargo/config.toml mark=edits
what: cargo's build configuration for the workspace (none stands today; added only if the mend needs it)
@task_file n=3 path=tests/blitz-tests/Cargo.toml mark=edits
what: the integration-test crate's manifest — edits: its test targets and features
@task_file n=4 path=tests/blitz-tests/tests mark=edits
what: the integration-test crate's test files, one test binary per file today (185 files, the stand and a11y tests among them) — edits: their layout, if the mend merges them into one binary per leg
@task_file n=5 path=examples/seven_guis/tests mark=edits
what: the seven_guis example's integration tests (8 files) — edits: their layout, if the mend merges them
@task_file n=6 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg runner, run locally too — edits: a leg's cargo invocation (fast, a11y, stand-repeat, embolden, coverage, doc, build), its package selection and features
@task_file n=7 path=.github/workflows/ci.yml mark=edits
what: the CI workflow and its legs — edits: a leg's cargo invocation, its membership kept
@task_file n=8 path=.github/scripts/test_ci_workflows.py mark=edits
what: the check that pins the CI legs, their --locked flag and membership — edits: the pins a changed invocation moves
@task_file n=9 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the check that pins the integration-test crate's targets — edits: the pinned target list if the layout changes
@task_file n=10 path=scripts/agent-run.sh mark=edits
what: the agent-run harness — edits: its cargo invocation, if the mend aligns it
@task_file n=11 path=.config/nextest.toml mark=edits
what: the test runner configuration — edits: the mutants profile of the dev-host-only mutation tier
@task_file n=12 path=CLAUDE.md mark=edits
what: the project's key commands — edits: the commands a mend changes
@task_file n=13 path=.claude/docs/commands.md mark=edits
what: the key commands reference — edits: the commands a mend changes
@task_item n=1 part=needs
The restructure's invariant: if the mend moves or merges the integration-test files into fewer binaries, every test keeps its assertions and its two layout modes, and the accessibility assertions (Rust tests over AccessKit), the stand contrast and focus-order tables and the stand expectation tables still run as before; the existing a11y, stand-repeat and fast legs hold it, all green.
@task_item n=2 part=needs
The a11y CI leg and the accessibility test leg in the slow tier: its cargo invocation (cargo test --workspace today) is one of the invocations measured; if the mend changes its package selection, features or test targets, the leg still runs every accessibility assertion and still gates CI.
@task_item n=3 part=needs
Accessibility checks gate CI: a mend that changes test binaries or leg invocations keeps the accessibility checks gating, and the CI legs' membership stays as stated.
@task_item n=4 part=needs
Stand checks repeated three rounds: the stand-repeat leg (cargo test -p blitz-tests) is measured for its own hash; a mend that merges binaries or aligns its feature set keeps the three repeated rounds.
@task_item n=5 part=needs
Feature unification across invocations: if the mend aligns feature sets between gate entries and legs to stop a second hash, the accessibility feature stays default-on, the session library still always enables accessibility, the browser accessibility feature stays off by default, the stand binaries still build no platform adapter, and the snapshot readers and screen-reader integration stay behind the accessibility feature.
@task_item n=6 part=needs
The accessibility tier and reach stay as stated: a build-directory mend changes no accessibility capability and the reach statement is not edited.
@task_item n=7 part=needs
The measurement: each step's growth read by size before and after from a cold build directory, each unit's hash read from cargo's fingerprint, message-format output or deps names, never inferred; the full workspace build's footprint read first; where coverage and the mutation tier write and whether they count toward the total read too.
@task_item n=8 part=needs
The day's growth after the mend stated as a number of gigabytes for the capability v040-01, read before and after on the dev host, every check green.
@task_item n=9 part=needs
Committed lockfile and --locked on every build, test and CI leg stay; a changed leg invocation is a CI-pipeline change that goes through the CI workflow pins and the test-target pins.
@task_item n=10 part=needs
A stale-hash sweep, if chosen, touches the build directory alone: no new network, socket, environment read or secret; the mutation tier stays dev-host-only, never a CI leg.
@task_item n=11 part=needs
Less debuginfo for test binaries, if chosen, still lets a failing accessibility or stand test report where it failed (line tables), and changes no test outcome.
@task_standing n=1
convention: each accessibility check a moved or merged test file carries stays a Rust test over AccessKit, asserted in CI
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: each CI leg or job changed keeps the accessibility checks gating CI and the stand checks repeated
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
Keyboard navigation rows (focus, Tab order, keyboard activation, range input and select keys, the Cells grid and CRUD list contracts): the chunk changes no input or focus behaviour.
@task_item n=2 part=not_needed
ARIA patterns and roles rows (role mapping, hidden-content exclusion, single select expanded state): the chunk changes no role or tree content.
@task_item n=3 part=not_needed
Visual design verification rows (contrast thresholds and helper, lint findings, select list colours and motion, scrollbars, hit-testing, zoom and scheme): the chunk changes no paint, style or lint.
@task_item n=4 part=not_needed
The ARIA state and property read contracts and the accessibility tree state contracts of the A11y Strategy area, and its changed-set and tree-refresh decisions: no engine behaviour changes.
@task_item n=5 part=not_needed
The snapshot, snapshot text, snapshot diff, actions answer, driver refusal, screenshot, MCP result, node id and accessible-name contracts of Screen Reader Support: no driver, snapshot or naming behaviour changes.
@task_item n=6 part=not_needed
Cognitive accessibility rows on invalid-state presentation: no control styling or state changes.
@task_item n=7 part=not_needed
The Dioxus document reached-handler record and UI graph state registration contracts: no handler or graph state changes.
@task_item n=8 part=not_needed
The SLO invariants, anti-patterns and decisions log areas: they hold no recorded intent.
