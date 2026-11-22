@task_prose part=chunk
Closes or states the dependency audit's standing per-ID advisory ignores (sqlite, paste, memmap2, the zbus family, zip, ttf-parser): each fixed by a reachable version move, stated as a wait on a named release of another project, kept by the founder's word, or named as not closing.
Verifies skia's prebuilt archive: fetched once, its checksum recorded in the repository, CI and the local gates check the sum and point the build at the verified file through SKIA_BINARIES_URL, with no line of upstream code changed.
Acts on the cancelled-runner watch (a hosted macOS or iOS CI job cancelled with no runner and 0 steps) and finds why a CI=true fast leg on the dev host recompiles its units, fixing it or stating the cause as a limit.
Re-reads the zbus family's reach through the platform adapter chain (accesskit_unix under accesskit_xplat under blitz-shell) and its coupling to the AccessKit pin; writes the founder's forks into founder-forks.md and stops there.
@task_file n=1 path=deny.toml mark=edits
what: the dependency audit's configuration, its per-ID advisory ignore set — edits: the [advisories] ignore list
@task_file n=2 path=Cargo.toml mark=edits
what: the workspace manifest, its dependency version pins — edits: a dependency's version where an update reaches it
@task_file n=3 path=Cargo.lock mark=edits
what: the workspace lockfile, the resolved dependency graph including the AccessKit platform adapter chain
@task_file n=4 path=apps/browser/Cargo.toml mark=edits
what: the browser app's manifest, its features (accessibility off by default, avif, skia as a default) — edits: a dependency's version where an update reaches it
@task_file n=5 path=packages/browser-persistence/Cargo.toml mark=edits
what: the browser persistence package's manifest, its rusqlite pin — edits: the rusqlite pin where an update reaches it
@task_file n=6 path=.github/workflows/ci.yml mark=edits
what: the CI workflow, its jobs and legs, the a11y CI leg among them — edits: a skia checksum check step, the cancelled-runner handling of the hosted macOS or iOS job
@task_file n=7 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg script the CI jobs and the local gates run — edits: the skia verified-archive wiring, the CI=true dev-host recompile cause
@task_file n=8 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow checks, the workflow's pins and invariants — edits: the pins and invariants for the new skia step and the cancelled-runner change
@task_item n=1 part=needs
The zbus family advisories reach the build only through the platform adapter (accesskit_unix under accesskit_xplat under blitz-shell); they drop only when accesskit_unix moves to zbus 6, so any close is a version move of the AccessKit adapter chain, coupled to the AccessKit 0.25 pin, or a stated wait on that release.
@task_item n=2 part=needs
A version move of the AccessKit chain keeps AccessKit as the source of the accessibility tree and accesskit_xplat as the platform adapter; the adapter tree still equals the harness tree.
@task_item n=3 part=needs
A version move or feature change keeps the accessibility feature's gates as they stand: default-on where it is, off by default on the browser app, always enabled by the session library, and no platform adapter built into the stand binaries.
@task_item n=4 part=needs
The Dioxus accessibility feature is taken only by name: a dependency update reaching the Dioxus or blitz manifests does not rewire it.
@task_item n=5 part=needs
Every version move is a lockfile or manifest change under --locked with the audit, MSRV and lock legs green; no line of upstream code changes for skia.
@task_item n=6 part=needs
CI workflow edits (the skia checksum step, the cancelled-runner action) keep the a11y CI leg and the accessibility test leg in the slow tier running, and keep accessibility checks gating CI.
@task_item n=7 part=needs
The cancelled-runner watch is acted on: a retry, a continue-on-error, the job's removal from the required set or the founder's keep; a change to the required set must not drop an accessibility check from it.
@task_item n=8 part=needs
Skia's verified archive: a checksum file recorded in the repository and a fetch script per platform that builds skia; CI and the local gates check the sum and point the build at the verified file through SKIA_BINARIES_URL; the sum is updated by hand at each skia version move.
@task_item n=9 part=needs
The CI=true fast leg on the dev host builds its units under new hashes; the chunk makes it reuse the dev host's units or states the cause as a limit, the CI leg script's env reads among the suspects.
@task_item n=10 part=needs
The CI workflow checks' pins and invariants are updated for each new or changed step and stay green.
@task_item n=11 part=needs
Stand checks repeated three rounds stay as they are under any CI leg change.
@task_item n=12 part=needs
The founder's forks (what acting on the cancelled-runner watch is, an advisory kept by his word) are researched, costed and written into founder-forks.md in the run dir, recommended first and least-change named; skia's verified archive is ruled and is no fork.
@task_standing n=1
convention: each new CI step or job the chunk adds runs under the same CI gate the accessibility checks gate, and its pins are asserted in the CI workflow checks
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
Keyboard navigation (5-keyboard-navigation, keyboard-navigation): focus, Tab order, key activation, select list keys and range input keys; the chunk changes no input handling.
@task_item n=2 part=not_needed
Visual design verification (6-visual-design-verification, visual-design-verification): contrast thresholds, lint contrast findings, scrollbar and select list colours and motion; the chunk touches no paint or colour.
@task_item n=3 part=not_needed
Cognitive accessibility (8-cognitive-accessibility, cognitive-accessibility): invalid state signalling and the stand's shared tab order table; untouched by dependency and CI work.
@task_item n=4 part=not_needed
ARIA patterns and roles (4-aria-patterns-roles, aria-patterns-roles): role mapping, hidden content contracts and the single select's expanded state are engine reads the chunk does not edit.
@task_item n=5 part=not_needed
The driver's snapshot, actions answer, screenshot, refusal and snapshot text contracts in screen-reader support: the chunk adds no driver verb and changes no snapshot field.
@task_item n=6 part=not_needed
The obs log line format under the assertion harness overview: the chunk adds no log line and no a11y violation schema.
