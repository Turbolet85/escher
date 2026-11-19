@task_prose part=session
The session measured why the local build directory grows: a source edit adds no new cargo hash, a lockfile move adds one per unit above the moved crate; an unmended replayed working day grew it by 57.90 GB, a mended one shrank it by 2.04 GB.
It added a dev-host build-directory sweep that removes stale cargo units under target/debug outside a closed keep list of seven locked cargo runs, refuses while a build holds the cargo lock, reads no environment value of its own and prints one line of closed words and counts.
The local fast leg now ends, after its five legs read green and only when CI is unset or empty, with that sweep; a refused or failed sweep never reddens it, and the leg count stays five.
It measured, as an open limit, that running with CI=true rebuilds about 48 GB of the tree under new hashes; the mechanism is not found.
No Rust crate, manifest, lockfile, stand app, test of the engine or accessibility code changed, and the pushed HEAD's CI read green 17/17.
@task_file n=1 path=scripts/target-sweep.py
what: the new dev-host build-directory sweep — removes stale cargo build units under target/debug outside its keep list, holds the cargo build lock, prints one closed-word stdout line; a build-hygiene tool, no CI leg and no accessibility check
@task_file n=2 path=scripts/target-sweep.ps1
what: the new Windows-shell wrapper that passes its arguments to the build-directory sweep and returns its exit code
@task_file n=3 path=.github/scripts/test_target_sweep.py
what: the new ci-scripts leg unittest file pinning the build-directory sweep and the fast leg's ownership of it (16 cases)
@task_file n=4 path=.github/scripts/ci-leg.sh
what: the CI leg runner that runs each named CI leg locally and in CI; its fast arm edited to run the build-directory sweep after a green verdict on a dev host, skipped under CI
@task_item n=1 part=touches
A row that states what the local fast leg or the local pre-push gate runs, or what follows its green verdict, through the CI leg runner: fast now ends with the build-directory sweep on a dev host, skipped under CI, never reddening the leg; the number of fast legs stays five.
@task_item n=2 part=touches
The a11y CI leg and the accessibility test leg in the slow tier, indirectly through the CI leg runner that runs them: check only whether either row names the fast leg's steps, the leg order or the runner's environment reads; the a11y and slow-tier legs themselves did not change.
@task_item n=3 part=touches
Accessibility checks gating CI, indirectly: the session reads a gate line of the fast leg differently on a dev host (a sweep line printed after green) than under CI (no sweep); a row stating what a green local run versus a green CI run proves may need that beside it.
@task_item n=4 part=touches
A row naming an environment value the CI leg runner reads: the runner's fast arm now reads CI, a flag carrying no content, and CI=true is measured to rebuild the tree under new cargo hashes.
@task_item n=5 part=touches
A row that names the painted select-list test built with the font-embolden feature or a clippy build of the engine with accessibility or other default features off: those selections are now on the sweep's closed keep list, so their build units are kept warm; no behaviour of them changed.
@task_item n=6 part=touches
Stand checks repeated three rounds and any row assuming a warm local build between runs, indirectly: units outside the sweep's keep list are removed after each green local fast leg, so a selection not on that list may rebuild cold on its next run; the checks themselves did not change.
@task_item n=1 part=untouched
Accessibility tree, role mapping, ARIA property and state reads, hidden and inert content: no engine or accessibility code changed.
@task_item n=2 part=untouched
Keyboard navigation, focus, activation, select and range input contracts and the seven_guis grid and list contracts: no stand app or input handling changed.
@task_item n=3 part=untouched
Visual design verification, contrast thresholds, the lint's contrast finding and the stand accent colour: no paint, style or lint code changed.
@task_item n=4 part=untouched
Screen reader support, the snapshot, the actions answer, the UI graph and the driver's refusals and screenshots: no driver or snapshot code changed.
@task_item n=5 part=untouched
Accessibility scope, tier, the accessibility feature defaults and the platform adapter choice: no feature flag, manifest or dependency changed.
@task_item n=6 part=untouched
The a11y assertion harness contract and the obs log format its overview cites: the sweep writes no log and adds no violation output.
@task_item n=7 part=untouched
Cognitive accessibility and the stand's shared tab-order table: nothing in them is reached.
