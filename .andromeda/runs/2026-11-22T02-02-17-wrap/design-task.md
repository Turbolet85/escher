@task_prose part=session
The session paid dependency and CI debts and changed no Rust source, no crate and no manifest; the engine, the driver, the examples and their rendering are byte-unchanged.
Skia's prebuilt archive (skia-bindings 0.157.0, the same version as before) is now fetched once per platform, checked against a committed sha256 record of five targets, and handed to skia-bindings' build as a local file template; the seven skia-building CI legs and the CI matrix rows build from it.
A failed read or an unrecorded key falls back to a full source build of skia with no panic; a witness step in the CI matrix reads the build output by content and goes red when a row's build did not read the verified archive. The planned force-download flag was disproved by measurement and never set.
The yanked async-executor 1.14.0 moved to 1.14.1 in the lock alone; the accessibility adapter probe on the stand still reads 0 after the move.
The advisory ignores keep the same six IDs, each reason now naming the release it waits on; the zip ignores are gone, and the cancelled-runner watch was retired by its rule.
Measured: on Cargo 1.100 (the CI runners) a build script's output sits under a different build-directory layout than on Cargo 1.99 (the dev host); final CI green 17/17.
@task_file n=1 path=.github/skia-archives.toml
what: the committed record of skia's prebuilt archives, one tag and a key and sha256 for each of the five built targets (linux desktop, macOS desktop, iOS, windows desktop, android)
@task_file n=2 path=.github/scripts/skia_archive.py
what: the build-time check that fetches skia's prebuilt archive for one target, verifies its sum against the record and prints the local URL template the skia build reads
@task_file n=3 path=.github/scripts/test_skia_archive.py
what: the unit tests of the skia archive check, run in the CI scripts leg
@task_file n=4 path=.github/scripts/ci-leg.sh
what: the CI leg runner shared by local and CI runs, the jobs that run clippy, the tests, the build, the MSRV, the docs, the coverage and the a11y stand, now pointing each skia-building leg at the verified archive
@task_file n=5 path=.github/scripts/test_ci_workflows.py
what: the CI scripts leg's pins on the workflow and the leg runner, now pinning the verified-archive steps and the witness
@task_file n=6 path=.github/workflows/ci.yml
what: the CI workflow, whose matrix job builds and tests the desktop rows and builds the mobile targets (android, iOS), now with a verified skia archive step and a witness step
@task_file n=7 path=deny.toml
what: the dependency audit's advisory ignores, each with its reason
@task_file n=8 path=Cargo.lock
what: the locked dependency graph of the workspace
@task_item n=1 part=touches
decision/mobile-targets-built-not-tested: the mobile targets are still built, not tested, but each android and iOS matrix row now builds skia from the verified prebuilt archive and a witness step reads red when that build did not read it; a row stating how the mobile builds get skia, or what turns a mobile row red, may be stale.
@task_item n=2 part=touches
decision/test-engine-with-system-fonts and decision/tests-built-with-system-fonts: the test leg that builds the test engine is one of the skia-building legs now pointed at the verified archive; the fonts choice is not changed, but a row describing how the test build is produced sits beside this.
@task_item n=3 part=touches
Rows on screenshot painting (decision/screenshot-never-skips-text, contract/driver-screenshot/fonts-unavailable, contract/driver-screenshot-refusals/fonts-unavailable, contract/screenshot/select-list-row): indirectly, through the skia renderer's prebuilt binary now coming from a verified archive of the same skia version; no painting behaviour changed, so a picker should confirm no row names how skia is obtained.
@task_item n=4 part=touches
decision/select-list-engine-overlay and the select-accessibility contracts: indirectly, through the a11y stand leg (a skia-building leg now under the verified archive) and the async-executor lock move, after which the accessibility adapter probe still reads 0; no accessibility tree output moved.
@task_item n=5 part=touches
surface-desktop-native area overview and decisions on desktop targets: the windows, macOS and linux desktop matrix rows now carry the verified skia archive step and the witness step; a row stating the desktop rows' build steps or their failure signal would sit beside this.
@task_item n=1 part=untouched
Colour palette, typography, spacing, depth, border radius and iconography rows: no colour, font, spacing, paint order, hit-testing, cursor or favicon behaviour changed; no engine or integration source moved.
@task_item n=2 part=untouched
Motion rows (the virtual clock, driver advance, scroll behaviour, overlay scrollbar timings, select list motion): no timing or scrolling code changed.
@task_item n=3 part=untouched
The escher-session commands, exit statuses, exit codes, stdout, stderr, mcp role, tasks, actions line, coverage line and UI graph line contracts: the driver and its output are byte-unchanged.
@task_item n=4 part=untouched
The select list events, the snapshot's select list row bounds and the JS window, shell provider, MountedData and dioxus-native configuration contracts: no runtime API changed.
@task_item n=5 part=untouched
The WPT runner output and result colour contracts, and the web-spa surface and its canvas sizing: neither the WPT runner nor the WASM pages changed.
@task_item n=6 part=untouched
The absences (engine colour tokens, a project type scale, reduced-motion handling): no capability was added or removed.
