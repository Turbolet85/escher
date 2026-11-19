@task_prose part=session
The session measured why the build directory grows over a working day: a lockfile move adds new unit hashes above the moved crate, while a source edit adds none.
It added a dev-host sweep of the build directory: a plan or run verb, a closed keep list of seven locked cargo runs, a non-blocking lock probe that makes it refuse while a build is in flight, and one stdout line of closed words and counts (no path), with exit 0 swept or planned, 1 failed, 2 usage, 3 refused.
The local fast leg now ends with that sweep after its five legs read green, on a dev host only (skipped when CI is set), and a refused or failed sweep never turns the leg red; the number of fast legs stays five.
Measured: an unmended replayed day grew the build directory by 57.90 GB, and a mended day shrank it by 2.04 GB. Measured limit: running the fast leg with CI=true rebuilds 42 packages under new hashes, by a mechanism not yet found; the project source reads no CI.
No Rust crate, manifest, lockfile, engine, app, example or test of the UI changed, and nothing was retired.
@task_file n=1 path=scripts/target-sweep.py
what: the dev-host build-directory sweep, which removes stale cargo units outside a closed keep list, refuses while a build holds the lock, and prints one line of closed words and counts
@task_file n=2 path=scripts/target-sweep.ps1
what: the PowerShell wrapper that passes its arguments to the build-directory sweep and returns its exit code
@task_file n=3 path=.github/scripts/test_target_sweep.py
what: the ci-scripts leg's contract tests for the build-directory sweep and for the fast leg owning it as its one caller
@task_file n=4 path=.github/scripts/ci-leg.sh
what: the CI leg runner, whose fast arm, the local pre-push gate, now ends with the build-directory sweep on a dev host
@task_item n=1 part=touches
Any row that states what the local fast leg or pre-push gate runs, or in what order: the fast leg now ends with a build-directory sweep after its five legs read green, skipped when CI is set, and its exit is discarded.
@task_item n=2 part=touches
Any row whose cell holds the number of fast legs or the fast leg's last step: the leg count stays five, but the sweep is now a step after the last leg.
@task_item n=3 part=touches
Indirect, through the sweep's keep list: rows on tests built with system fonts, or on the test engine built with system fonts. Test builds whose feature selections are on the keep list, such as the font-embolden select popup paint test, stay warm. Builds under any other feature selection lose their cached units after a green fast leg.
@task_item n=4 part=touches
Indirect: rows on the painted select list and on screenshots never skipping text. The select popup paint test with font embolden is now named in the sweep's closed keep list. Its behaviour is unchanged, but its build is now one the sweep keeps.
@task_item n=5 part=touches
An occupied name: CI is now an environment name the fast leg reads as a flag that carries no content. Any row listing the environment values the project reads is stale by omission.
@task_item n=6 part=touches
A measured limit: running with CI=true rebuilds the tree under new unit hashes (42 packages), by a mechanism not found. Any row stating that CI and local builds share one build, or that the engine source reads no CI flag, is touched.
@task_item n=1 part=untouched
Colour palette rows (engine colours, select list colours, color-scheme recascade, the base colour config): no colour value or rule changed.
@task_item n=2 part=untouched
Typography rows on font fallback, lang, root-relative units, font element hints and the bundled WASM font context: no font behaviour changed.
@task_item n=3 part=untouched
Spacing and depth rows (margins, cell spacing, z-index, stacking contexts, hit-testing at paint clips, select list placement and paint order): no layout or paint rule changed.
@task_item n=4 part=untouched
Motion rows on the virtual clock, driver advance, instant scroll, smooth scroll, overlay scrollbar timings and select list motion: no timing changed.
@task_item n=5 part=untouched
escher-session contracts (commands, exit statuses and exit codes, stdout, stderr, MCP role results, tasks, session error remedy): the sweep's own exit codes and stdout line belong to a separate script and do not change escher-session's.
@task_item n=6 part=untouched
The actions, coverage and UI graph lines, the snapshot and screenshot verbs, select accessibility and select list event contracts: no driver output changed.
@task_item n=7 part=untouched
WPT runner output and result colours, the JS window contracts, shell provider contracts, cursor and favicon iconography, and the web-spa canvas surface: none of these was reached.
