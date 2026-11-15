@task_prose part=session
The eleventh upstream take: upstream main read to ea07ee716, three upstream commits cherry-picked as upstream wrote them, thirteen new commits left out (112 left out in all), each with a measured reason.
The picks: blitz-dom now carries the @font-face font-stretch (font-width) descriptor into font matching as a new public width field on its font-face overrides; the WPT pinned commit moved; the WPT timeout quarantine gained four entries, each with reason dynamic-load-events.
taffy moved 0.15.0 to 0.15.1 from the crates.io registry, a lock-only semver-compatible update that brings 13 of the 14 Taffy-main commits; no manifest moved, no package entered or left, and the lock holds no git source.
The git-Taffy take was measured green and then left out under the no-git-dependency rule. The founder has since ruled an exception for Taffy alone to that rule, and a later chunk will move Taffy to git. The founder ratified both behaviour changes for every document (font-stretch read, taffy 0.15.1 layout fixes).
Every leg was green with no tally moved: fast 1461 passed, 0 failed, 13 ignored; stand 294/0/8; all 825/0/13; a11y, doc, msrv and audit green; fork CI 17/17, tallies +0 -0 in every job on linux, windows and macos.
On the founder's word, rdme and blitz (upstream demo shells that hold no tests) are stated outside the mutation tier's scope.
@task_file n=1 path=packages/blitz-dom/src/net.rs mark=merge
what: blitz-dom's net module, which fetches and registers @font-face rules and holds the font-face overrides carrier (now with a width field read from font-stretch)
@task_file n=2 path=packages/blitz-dom/src/document.rs mark=merge
what: blitz-dom's document, which applies font-face overrides at font registration and holds the inline font_face_override_tests unit module and its fixture
@task_file n=3 path=wpt/WPT_COMMIT mark=merge
what: the recorded WPT commit the conformance runner is pinned to
@task_file n=4 path=wpt/runner/timeout-quarantine.txt mark=merge
what: the WPT runner's timeout quarantine list, one test path and its known reason a line, skipped unless run-quarantined
@task_file n=5 path=Cargo.lock
what: the workspace lockfile the locked cargo legs build from, with registry sources and checksums (taffy now 0.15.1)
@task_item n=1 part=touches
WPT pinned to a recorded commit: the pinned commit moved to bbb30d0f; the rule names the file, not the commit, so it should still hold. A picker checks any row that quotes the old commit 07e827e4.
@task_item n=2 part=touches
WPT runner CLI run-quarantined and the WPT status TIMEOUT: four tests joined the timeout quarantine with the known reason dynamic-load-events, and no path repeats.
@task_item n=3 part=touches
WPT conformance runner, WPT suites run in CI, WPT run on the host on the fork, and WPT conformance being report-only: the suite set they run moved with the new pin and the four newly quarantined tests.
@task_item n=4 part=touches
WPT harness timeout multiplier: the four new entries are timeouts, quarantined rather than met by a larger multiplier.
@task_item n=5 part=touches
Inline unit test modules and blitz-dom fixtures built without the HTML parser: the blitz-dom font-face override fixture now names width None, and no test exercises a non-None width.
@task_item n=6 part=touches
Font-metric tests never skip, pre-existing font tests skip without a font, and font-dependent assertions on system fonts: font matching now also reads the @font-face width descriptor, a behaviour change for every document.
@task_item n=7 part=touches
Layout assertions in both layout modes, stand checks run in both layout modes, and the incremental layout differential oracle: taffy 0.15.1's layout fixes reach every document, and both modes and the oracle stayed green with no count moved.
@task_item n=8 part=touches
Local baseline for CI comparison and tally moves attributed by test name: the take's tallies read +0 -0 in every job against the base run.
@task_item n=9 part=touches
Dependency audit (advisories and licenses): taffy's resolved version and checksum moved, and the audit read advisories ok, licenses ok.
@task_item n=10 part=touches
Minimum supported Rust version and MSRV verified by build only: taffy 0.15.1 declares rust-version 1.71, and the msrv leg stayed green.
@task_item n=11 part=touches
Cargo legs run locked: the lockfile moved by a lock-only update (taffy 0.15.0 to 0.15.1), and no manifest moved.
@task_item n=12 part=touches
Clippy over the workspace and upstream blitz-dom: upstream blitz-dom gained a public field and two construction sites, and clippy stayed green.
@task_item n=13 part=touches
Upstream tests kept as written or left out: this take's upstream tests rule over the picks. No upstream test came with the picks, and upstream added no width test.
@task_item n=14 part=touches
Mutation testing on the dev host, mutation runs on the dev host only, mutation baseline read per unit, and mutation baseline skipped: rdme and blitz are now stated outside the mutation tier's scope (no tests, baseline-test-failure skips), while churn, sizes and the other collectors keep both.
@task_item n=15 part=touches
Per-crate line coverage floors and coverage floor per crate: blitz-dom gained a font-stretch read path with no non-None test. A picker flags only a row whose floor cell holds a blitz-dom value.
@task_item n=16 part=touches
blitz-dom builds requests through one stamping constructor: blitz-dom's net module changed, though only its font-face descriptor read, not how it builds requests.
@task_fact n=1 slug=10-quality-gates-coverage-targets
fact: The mutation tier does not run rdme (apps/readme) or blitz (packages/blitz): both are upstream demo shells holding no tests, so no test is owed them for the mutation tier; churn, sizes and the other collectors keep both.
@task_item n=1 part=untouched
The session lifecycle, session wire, session host, session errors and session state directory contracts: no socket, verb, wire field or error changed.
@task_item n=2 part=untouched
The escher-session command line, its exit codes, stderr and stdout, the MCP role and its tool results: no surface was added or moved.
@task_item n=3 part=untouched
The agent-run verbs, selections, events, exit codes and state: the five-command contract is unchanged, and it ran green as written.
@task_item n=4 part=untouched
Cold-agent runs, their verdict, events, isolation and exit codes: not reached.
@task_item n=5 part=untouched
Harness construction, core, input, inspection, screenshot, contrast and settle contracts: no harness API changed.
@task_item n=6 part=untouched
Driver refusal causes, driver verbs, the driver JSON answers, snapshot nodes, lint defects and handler coverage: not reached.
@task_item n=7 part=untouched
CI workflow shape: actions pinned to SHAs, read-only permissions, no secrets, cache and trigger branches, the fast/slow split, and the apt-install and coverage-floor script exit codes. No workflow or CI script changed.
@task_item n=8 part=untouched
Stand stand-ins, the virtual clock, advance and timer ticks, recorded checks and committed check content: no stand, fixture page or recorded check changed.
