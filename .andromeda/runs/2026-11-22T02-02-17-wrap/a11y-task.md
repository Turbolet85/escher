@task_prose part=session
The session paid dependency and CI debts with no Rust source change: the yanked async-executor 1.14.0 moved to 1.14.1 in the lock, and no crate was added.
Every CI leg that builds skia (clippy, test, build, msrv, doc, coverage and a11y) now first fetches skia's prebuilt archive for its host, checks it against a committed sha256 record and points the build at the local copy; a refused archive ends the leg before any cargo run.
The fast leg's closing target sweep now runs under the same verified archive, and the per-platform test matrix gained a verify step and a witness step that reads red when a row's skia build did not read its verified archive.
The dependency audit's six advisory ignores stayed the same, each with a reworded reason naming the release it waits on, and the zip ignores are gone.
Measured: after the lock move, the stand's dependency graph still holds no accesskit platform adapter (the probe reads 0), and CI read green 17 of 17 checks on the final head.
A plan claim was disproved and dropped: forcing skia's binary download does not panic on a wrong key in a crates.io build, so a missing or wrong archive falls back to a full source build, which the witness then reads red.
@task_file n=1 path=.github/skia-archives.toml
what: the committed record of skia's prebuilt archives, with the release tag and one key and sha256 per built target
@task_file n=2 path=.github/scripts/skia_archive.py
what: the build-time job that fetches and verifies skia's prebuilt archive for a target and prints the archive URL for the build
@task_file n=3 path=.github/scripts/test_skia_archive.py
what: the unit tests of the verified-archive job, run in the ci-scripts leg
@task_file n=4 path=.github/scripts/ci-leg.sh
what: the CI leg runner, the script that runs each named leg (the a11y leg among them) and the fast leg's target sweep
@task_file n=5 path=.github/scripts/test_ci_workflows.py
what: the ci-scripts leg's pins over the CI workflow and the leg runner
@task_file n=6 path=.github/workflows/ci.yml
what: the CI workflow, its legs and tiers and the per-platform test matrix
@task_file n=7 path=deny.toml
what: the dependency audit's policy and its advisory ignore list with each ignore's reason
@task_file n=8 path=Cargo.lock
what: the workspace dependency lock, the resolved dependency graph of the stand binaries
@task_item n=1 part=touches
The a11y CI leg and the accessibility test leg in the slow tier: the a11y leg now verifies skia's prebuilt archive before it runs any cargo, and a refused archive ends the leg with no test run.
@task_item n=2 part=touches
Accessibility checks gating CI: the gate now also fails when the a11y leg's skia archive is refused, before any accessibility assertion runs.
@task_item n=3 part=touches
Stand checks repeated three rounds: the rounds run inside the a11y leg, which now builds skia from the verified archive; the rounds themselves are unchanged.
@task_item n=4 part=touches
No platform adapter on the stand binaries: the dependency lock moved (async-executor 1.14.0 to 1.14.1); the stand's adapter probe was re-read and still reads 0.
@task_item n=5 part=touches
The absence of a platform accessibility adapter in any stand binary: re-measured after the lock move, still true.
@task_item n=6 part=touches
accesskit_xplat as the platform adapter: its dependency graph's lock entry for async-executor moved to a non-yanked release; no feature or adapter choice changed.
@task_item n=7 part=touches
Accessibility tier 0 and accessibility reach at tier 0: the reach stands, measured by the same adapter probe after the lock move.
@task_fact n=1 slug=9-ci-integration
fact: The a11y CI leg builds skia only from a prebuilt archive verified against a committed sha256 record, and a refused archive ends the leg before any test runs.
@task_item n=1 part=untouched
ARIA role mapping, ARIA state and property reads and accessible name precedence: no Rust source changed.
@task_item n=2 part=untouched
Keyboard navigation contracts (select lists, range input, cells grid, CRUD list, keyboard activation, focus rules): no behaviour changed.
@task_item n=3 part=untouched
Visual design verification: contrast thresholds, the lint's contrast finding, select list colours and the stand accent colour are unchanged.
@task_item n=4 part=untouched
Screen reader support: the snapshot, the snapshot text form, the actions answer, driver refusals and the accessibility tree contracts are unchanged.
@task_item n=5 part=untouched
Cognitive accessibility and the invalid-state rules are unchanged.
@task_item n=6 part=untouched
The accessibility feature's defaults and the dioxus accessibility feature: no manifest changed, every manifest byte-equal.
