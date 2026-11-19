@task_prose part=chunk
Measures, from a cold build directory, what grows it: which gate entries, CI legs, the agent-run harness, the cold-agent pipe and the dev-host mutation tier write which artifacts, and which edits mint a new metadata hash for which units, read from cargo's own output.
Names the cause from that measurement as a share per mechanism, each a number: re-hash per low-crate edit, differing feature sets or package selections, profile or flag differences, per-file integration-test binaries, debuginfo weight.
Applies one mend the measurement points at (one test binary per leg, a stale-hash sweep, less debuginfo for test binaries, or another), with every check still green.
Re-measures a working day's growth of the build directory after the mend and states it as a number of gigabytes, read before and after on the dev host.
Changes no test assertion; a test moved or merged keeps its assertions and its two layout modes.
@task_file n=1 path=Cargo.toml mark=edits
what: the workspace manifest and its build profiles — edits: the [profile.*] tables (dev debuginfo level)
@task_file n=2 path=.cargo/config.toml mark=edits
what: a cargo build configuration for the workspace (none stands today; a mend may add it)
@task_file n=3 path=tests/blitz-tests/Cargo.toml mark=edits
what: the integration-test crate's manifest — edits: its test targets list
@task_file n=4 path=tests/blitz-tests/tests mark=edits
what: the integration-test crate's per-file test binaries (185 files), which a one-binary-per-leg mend may merge or move
@task_file n=5 path=examples/seven_guis/tests mark=edits
what: the seven-GUIs example's integration tests (8 files), which a one-binary-per-leg mend may merge or move
@task_file n=6 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg runner and each leg's cargo invocation — edits: the legs' cargo invocations (fast, a11y, stand-repeat, embolden, coverage, doc, build)
@task_file n=7 path=.github/workflows/ci.yml mark=edits
what: the CI workflow and its jobs, whose leg membership stays as stated
@task_file n=8 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI-pipeline pin check over the workflow and the legs' invocations — edits: the pins a changed leg invocation moves
@task_file n=9 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the check pinning the integration-test crate's test targets — edits: the pinned target list
@task_file n=10 path=scripts/agent-run.sh mark=edits
what: the agent-run harness and its cargo invocation
@task_file n=11 path=.config/nextest.toml mark=edits
what: the nextest configuration — edits: the mutants profile of the dev-host-only mutation tier
@task_file n=12 path=CLAUDE.md mark=edits
what: the project's key commands, as a mend changes them — edits: the key commands list
@task_file n=13 path=.claude/docs/commands.md mark=edits
what: the key commands reference, as a mend changes them — edits: the key commands (wrap's leaves)
@task_item n=1 part=needs
A measurement of the build directory's growth per cargo invocation of the dev host's ordinary work, each step read by size before and after from a cold start, and each unit's hash read from cargo's output, not inferred.
@task_item n=2 part=needs
The cause named as a numeric share per mechanism: low-crate edit re-hash, feature-set or package-selection differences between invocations, profile or flag differences, per-file integration-test binaries, debuginfo weight.
@task_item n=3 part=needs
A mend applied to the build's profiles, test-binary layout, leg invocations or a stale-hash sweep, keeping every check green; a sweep touches the build directory alone.
@task_item n=4 part=needs
A working day's build-directory growth after the mend stated in gigabytes, read before and after on the dev host, manual method.
@task_item n=5 part=needs
Restructure invariant: integration tests moved or merged into fewer binaries keep every assertion and both layout modes, held by the same tests running green on every leg.
@task_item n=6 part=needs
Test binaries stay built with system fonts: any feature or package-selection change to the test legs must keep the test engine's system-font build, so a unified feature set does not drop or swap the font source tests rely on.
@task_item n=7 part=needs
The embolden leg's font-embolden feature on the paint crate stays applied to the test crate's run; a mend that unifies feature sets must not lose or leak it into legs that do not ask for it.
@task_item n=8 part=needs
Mobile targets stay built, not tested: the build leg's target builds are measured but a mend leaves them build-only.
@task_item n=9 part=needs
The committed lockfile and the locked flag on every build, test and CI leg stay; a change to a leg's cargo invocation goes through the CI-pipeline pin checks.
@task_item n=10 part=needs
The mutation tier stays dev-host-only, never a CI leg; its nextest profile may change only within that tier.
@task_item n=11 part=needs
No new network, socket, environment read or secret in any script, sweep or harness the chunk changes.
@task_item n=12 part=needs
The agent-run harness and the cold-agent pipe keep their sessions and task outcomes as they are; only their build cost is measured or mended.
@task_standing n=1
convention: every test engine or test binary the chunk builds, merges or reconfigures is built with system fonts, not bundled ones
home: typography
@task_item n=1 part=not_needed
Color palette rows: engine colours, select list colours and color-scheme recascade are rendering behaviour no build-directory change reaches.
@task_item n=2 part=not_needed
Spacing and depth-strategy rows: margins, paint order, stacking contexts and hit-testing are engine behaviour the build layout does not change.
@task_item n=3 part=not_needed
Motion rows: the virtual clock, scroll and animation timing are runtime behaviour, untouched by profiles or test-binary layout.
@task_item n=4 part=not_needed
Iconography rows: cursor and favicon resolution are runtime behaviour the chunk does not touch.
@task_item n=5 part=not_needed
escher-session command, exit-status, stdout, stderr and mcp contracts: the chunk changes no session output, only how its binaries are built.
@task_item n=6 part=not_needed
Select list events, accessibility and snapshot contracts: widget behaviour the chunk keeps asserted unchanged, not behaviour it changes.
@task_item n=7 part=not_needed
WPT runner output and result colour contracts: the WPT runner's terminal output is not touched.
@task_item n=8 part=not_needed
JS window, shell provider and dioxus-native configuration contracts: runtime surfaces outside the build.
@task_item n=9 part=not_needed
Web-spa surface: WASM page rendering into a canvas is not changed by the dev host's build directory.
