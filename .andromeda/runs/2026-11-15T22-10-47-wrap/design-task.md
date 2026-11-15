@task_prose part=session
The session moved the layout engine Taffy from the crates.io registry version 0.15.1 to upstream Taffy's git source at rev e90751fd (package version 0.15.0), under the Taffy-only exception to the no-git-dependencies rule; it is now the workspace's one git source.
It took eight upstream blitz commits by cherry-pick, Taffy #1303 among them, and the founder ratified their layout changes for every document (not provisional).
The inline layout pass now hands Taffy a caller-defined float id (the box's own node id) when it places a floated box; no public symbol, crate or module was added or removed.
Measured: every test tally, the stand, stand-repeat, the a11y leg, msrv, audit, doc, build, counter and wasm legs stayed equal to the base, and CI was green 17/17 across desktop, mobile and wasm targets.
Left out by design for a later route entry: upstream's reversed scroll origins (Taffy #1372 on) and five later Taffy bumps, so scrolled-box bounds, hit-testing and absolute positioning keep their present behaviour.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace manifest, the dependency list that names the layout engine Taffy and its source
@task_file n=2 path=Cargo.lock mark=conflict
what: the lockfile pinning each dependency's version, source and checksum, the layout engine Taffy among them
@task_file n=3 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the engine's inline layout pass, which lays out inline text and places floated boxes beside it through the layout engine
@task_item n=1 part=touches
Any row that names the layout engine Taffy as a crates.io registry version, or a version 0.15 or 0.15.1 of it: the engine is now upstream Taffy git at e90751fd, package version 0.15.0.
@task_item n=2 part=touches
Any row whose cell holds that the engine has no git dependency, or a count of git sources: the count moved 0 to 1, Taffy alone, under its own exception.
@task_item n=3 part=touches
Any row on how floated boxes are placed in inline layout: float placement now passes a caller-defined float id per floated box (Taffy #1303).
@task_item n=4 part=touches
Indirectly, rows whose behaviour rests on the engine's block and inline layout of every document: the eight taken commits changed layout for every document, ratified, with all tallies unchanged; a row stating a layout result may need checking against it.
@task_item n=5 part=touches
Indirectly, rows on body margins and table cellspacing, cellpadding and valign presentational hints, through the changed layout engine that lays those boxes out.
@task_item n=6 part=touches
Indirectly, the snapshot and screenshot contracts that report box bounds, such as a row of an open select list and its bounds, through the changed layout engine that computes them.
@task_item n=7 part=touches
Indirectly, the decision that the open select list is placed inside the viewport and the logical viewport decision, through the layout engine they read sizes from.
@task_item n=8 part=touches
Rows on mobile targets built not tested and on the WASM build: the git Taffy builds on every target (wasm, android, ios legs green in CI), no behaviour of those rows moved.
@task_item n=9 part=touches
Any row on scroll origins, scrolled-box bounds or hit-testing inside scrolled boxes that anticipates an engine change: upstream's reversed scroll origins were deliberately not taken and wait for their own route entry.
@task_item n=1 part=untouched
Colour rows: the colour palette, engine colour constants, color-scheme recascade and select list colours; no colour moved.
@task_item n=2 part=untouched
Typography rows on fonts: font fallback, generic families, root-relative units, system fonts in tests and the bundled WASM font context; no font or text shaping change.
@task_item n=3 part=untouched
Motion rows: the virtual clock, driver advance, instant scrolls, smooth scroll-behavior, overlay scrollbar fade and select list motion.
@task_item n=4 part=untouched
Iconography rows: cursor icons, cursor resolution and the favicon.
@task_item n=5 part=untouched
The escher-session command line and mcp contracts: commands, exit statuses and codes, stdout and stderr lines, tasks, the actions, coverage and UI graph lines.
@task_item n=6 part=untouched
The WPT runner output and result colour contracts, the JS window contracts, the shell provider contracts and the select list event and accessibility contracts.
@task_item n=7 part=untouched
Paint and stacking rows: z-index paint level, stacking-context grouping, select list painted after the root and hit-testing stopping at a paint clip; paint order and clipping did not change.
