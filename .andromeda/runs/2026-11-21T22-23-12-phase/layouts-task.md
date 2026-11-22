@task_prose part=chunk
Closes the dependency audit's standing per-ID ignores (sqlite, paste, memmap2, zbus family, zip, and reads the ttf-parser ignore whole) by a reachable version update under --locked, or states each as a wait on a named release of another project.
Builds skia's verified prebuilt archive: fetched once, its checksum recorded in the repository, CI and the local gates check the sum and point the build at the verified file through SKIA_BINARIES_URL, with a CI step and a fetch script per platform; no line of upstream code changes.
Acts on the cancelled-runner watch (a hosted macOS or iOS CI job cancelled with no runner and 0 steps) and finds why CI=true recompiles on the dev host, fixing it or stating the cause as a limit.
Claims the codebase-health capability against the epoch boundary audit's reading; the default-branch line of the workflow doc is the operator's landing slot, never a code step.
@task_file n=1 path=deny.toml mark=edits
what: the dependency audit's configuration, its advisory ignore set — edits: the [advisories] ignore list
@task_file n=2 path=Cargo.toml mark=edits
what: the workspace manifest, its dependency version pins — edits: the pins a version move reaches
@task_file n=3 path=Cargo.lock mark=edits
what: the workspace lock file, the resolved dependency graph built under --locked
@task_file n=4 path=apps/browser/Cargo.toml mark=edits
what: the browser app's manifest, its dependencies and features (capture serialization, avif, skia default) — edits: the dependency pins an update reaches
@task_file n=5 path=packages/browser-persistence/Cargo.toml mark=edits
what: the browser persistence package's manifest, its sqlite binding and migration pins — edits: the rusqlite pin, where an update reaches it
@task_file n=6 path=.github/workflows/ci.yml mark=edits
what: the CI workflow, its jobs per platform including the hosted macOS and iOS jobs and the skia-building jobs — edits: a skia archive fetch-and-verify step, the cancelled-runner handling of the iOS job
@task_file n=7 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg runner shared by CI and the local gates (the fast leg and the others) — edits: the skia archive check and SKIA_BINARIES_URL wiring, the CI-variable read behind the dev-host recompile
@task_file n=8 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow tests, the pins and invariants the workflow must keep — edits: the pins and invariants the new skia step and the iOS job change
@task_item n=1 part=needs
The dependency audit's advisory ignore set: each ignored advisory (RUSTSEC-2026-0459, 2024-0436, 2026-0186, the zbus family 0370-0372, the zip family 0367-0369 and 0429, 2026-0192) removed by an update or kept with a stated wait on a named release.
@task_item n=2 part=needs
A dependency version move is a lock and manifest change under --locked, with the audit, MSRV and lock legs green.
@task_item n=3 part=needs
Skia stays a default build dependency of the browser app and the WPT runner; its prebuilt archive's checksum is recorded in the repository and updated by hand at each skia version move.
@task_item n=4 part=needs
CI and the local gates verify the archive's checksum and point the skia build at the verified file through SKIA_BINARIES_URL, so a failed fetch never falls back silently to a source build.
@task_item n=5 part=needs
No line of upstream code changes for skia or for any dependency fix.
@task_item n=6 part=needs
The cancelled-runner watch on the hosted macOS and iOS CI jobs: its recurrence acted on (a retry, continue-on-error, removal from the required set, or the founder's keep).
@task_item n=7 part=needs
The CI=true recompile on the dev host: the fast leg run with CI set either reuses the dev host's build units or its cause is stated as a limit.
@task_item n=8 part=needs
The CI workflow tests' pins and invariants hold every changed or added CI step and job.
@task_item n=9 part=needs
No new socket, port, listener, credential path or secret; the skia fetch is a supply-chain surface registered in the security plan's Dependencies section, with each advisory's standing.
@task_item n=10 part=needs
The browser app's capture serialization keeps reading archive entries by name and never extracting them, through whatever zip version an update brings.
@task_item n=11 part=needs
The browser persistence package's sqlite storage keeps its migrations working where the rusqlite move reaches it.
@task_item n=12 part=needs
The accessibility tree reaches the platform through the shell's AccessKit adapter; a zbus move stays coupled to the AccessKit pin.
@task_item n=13 part=needs
Founder forks are written, researched and costed into founder-forks.md in the run dir and the phase stops there; the verified skia archive is already ruled and is no fork.
@task_item n=1 part=not_needed
The stand's stable element ids: the chunk adds, renames or removes no stand element.
@task_item n=2 part=not_needed
The stand's page decisions (document skeleton, accent colour, keyed controls, attribute-only names and ids, pinned headless viewport, the task pages' controls): the chunk builds no page and changes no stand layout.
@task_item n=3 part=not_needed
The escher-session command's contracts (actions, exit codes, refusals, snapshot, screenshot, layout, lint, graph, coverage, recording, served tasks, MCP server): the chunk changes no session verb, answer or refusal.
@task_item n=4 part=not_needed
The cold-agent pipe and agent-run test contracts and their decisions: the chunk changes neither pipe's usage, output or exit codes.
@task_item n=5 part=not_needed
The multi_doc and paint_bench command lines: the chunk changes no benchmark argument or output.
@task_item n=6 part=not_needed
The scroll, clipping and open-list bounds decisions: the chunk does no layout or canvas work.
@task_item n=7 part=not_needed
The mobile-native and web-spa surfaces: unmeasured, and the chunk builds no screen on either.
@task_item n=8 part=not_needed
Arbitrary index.html templates for a document: the chunk adds no document template.
