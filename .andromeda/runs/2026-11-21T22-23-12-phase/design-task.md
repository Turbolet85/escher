@task_prose part=chunk
Closes the dependency audit's standing per-ID ignores (libsqlite3-sys, paste, memmap2, the zbus family, zip, ttf-parser) each by a reachable version update, a stated wait on a named release, the founder's keep, or a named non-closure.
Verifies skia's prebuilt archive: fetched once, its checksum recorded in the repository, CI and the local gates check the sum and point the build at the verified file; no line of upstream code changes.
Acts on the cancelled-runner watch (the hosted macOS/iOS CI job cancelled with no runner) and finds or states as a limit why a CI-flagged fast leg recompiles on the dev host.
Re-reads the Epoch 2 boundary audit to claim codebase health, with the oversize files, over-ceiling functions and duplicated test pairs kept by the founder's word.
@task_file n=1 path=deny.toml mark=edits
what: the dependency audit's configuration — edits: the advisories ignore list
@task_file n=2 path=Cargo.toml mark=edits
what: the workspace manifest and its dependency version pins
@task_file n=3 path=Cargo.lock mark=edits
what: the locked dependency graph, moved only under the locked build with the audit, MSRV and lock legs green
@task_file n=4 path=apps/browser mark=edits
what: the browser app, whose skia default, avif image feature and capture serialization reach the advisories, where an update reaches it
@task_file n=5 path=packages/browser-persistence mark=edits
what: the browser persistence package and its SQLite binding pin, where an update reaches it
@task_file n=6 path=.github/workflows/ci.yml mark=edits
what: the CI workflow — edits: the jobs that build skia, the new checksum-verified skia fetch step, the hosted macOS and iOS jobs under the cancelled-runner watch
@task_file n=7 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg script the local gates and CI run — edits: the skia archive fetch and checksum check, the fast leg's CI-flagged build
@task_file n=8 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow tests — edits: the pins and invariants
@task_item n=1 part=needs
Skia stays the default renderer backend for the browser app and the WPT runner; only where its prebuilt archive comes from changes, so what those apps paint and screenshot stays as it is.
@task_item n=2 part=needs
Mobile targets are built in CI, not tested: whatever acting on the cancelled no-runner macOS/iOS job becomes (a retry, a tolerated failure, removal from the required set, or a keep), the mobile targets stay built.
@task_item n=3 part=needs
A version move of the AccessKit Linux adapter off the old zbus family, where one is reachable, must keep the accessibility tree a select exposes (open select, chosen option, highlighted row, open attribute) as it is.
@task_item n=4 part=needs
A version move of the image, avif encoder or capture serialization crates, where one is reachable, must keep the screenshot verb's output and the capture path's reads as they are.
@task_item n=5 part=needs
A move of the font parser pinned through the windowing beta, should the chunk take one, must keep text laid out and screenshots painted with text and the system fonts tests are built with.
@task_item n=6 part=needs
A move of the SQLite binding behind browser persistence, should the chunk take one, changes no behaviour a surface of the browser app shows.
@task_item n=7 part=needs
No new socket, port, listener, credential path or secret; the skia fetch is a supply-chain surface the security plan's Dependencies section registers, and the checksum file is no secret.
@task_item n=8 part=needs
Founder forks are written, researched and costed, to the run dir's founder-forks file and the phase stops there; skia's verified archive is already ruled and is no fork.
@task_item n=1 part=not_needed
Color palette rows: engine colours, select list colours and color-scheme recascade are untouched by a dependency or CI change.
@task_item n=2 part=not_needed
Spacing and depth-strategy rows: margins, cell spacing, list placement, paint order, stacking and hit-testing are no part of this chunk.
@task_item n=3 part=not_needed
Motion rows in both motion areas: the virtual clock, animation, scroll behaviour and scrollbar fade timings are not reached.
@task_item n=4 part=not_needed
Iconography rows: cursor icons, cursor resolution and favicons are not reached.
@task_item n=5 part=not_needed
The cli surface's escher-session exit codes and stdout lines: the chunk changes no command line tool.
@task_item n=6 part=not_needed
The web-spa surface: the WASM canvas pages are not built or changed by this chunk.
@task_item n=7 part=not_needed
Brand identity, border radius, anti-patterns, self-validation and design decisions log areas: they hold no rows a dependency or CI change reaches.
