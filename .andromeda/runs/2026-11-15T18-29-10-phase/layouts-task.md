@task_prose part=chunk
Moves the workspace's taffy dependency from the crates.io registry 0.15.1 to upstream's git pin a3997d55 of dioxuslabs/taffy, so the tree carries Taffy #1303 (the caller-defined float id) and the Taffy content of upstream's left-out chain.
Takes the blitz side of that chain as upstream wrote it: the inline layout's place_floated_box call gains a new first argument, the caller's taffy node id, plus the code edits of the nine Bump Taffy commits; no other upstream pick.
Cargo.lock moves with it: one taffy package from a git source in place of the registry one, every other package only as the move forces.
Before any plan, research re-measures the move in a trial worktree: whether the workspace builds, which gates and tests change (fast, clippy, audit, msrv at 1.91, doc, build, the incremental oracle, the stand checks, the WPT runner's build), the lock delta, and the standalone wasm example's own taffy resolution.
If the measurement says the move cannot be taken cleanly, or a fork widens a boundary or needs a master rule lifted beyond the Taffy-only exception, the phase stops with the measurement or the fork.
@task_file n=1 path=Cargo.toml mark=edits
what: the workspace's root manifest, its dependency table and the taffy dependency's source — edits: the taffy dependency entry (registry 0.15 to git rev a3997d55)
@task_file n=2 path=Cargo.lock mark=edits
what: the workspace lock every --locked build, test and CI leg reads — edits: the taffy package entry and any package the move forces
@task_file n=3 path=packages/blitz-dom/src/layout/inline/layout.rs mark=edits
what: the inline layout of the DOM's layout engine, where floated boxes are placed through Taffy's block context — edits: the place_floated_box call, which gains the caller's taffy node id as its new first argument
@task_file n=4 path=deny.toml mark=re-reads
what: the dependency audit's advisories and licences checks, kept as they are and re-run against the git Taffy
@task_file n=5 path=.github/workflows/ci.yml mark=re-reads
what: the CI gates the move is measured against: fast, clippy, audit, msrv at rust 1.91, doc and build legs, all under --locked
@task_file n=6 path=examples/wasm_hello/Cargo.toml mark=re-reads
what: the wasm example, a standalone workspace with its own lock not under the root's --locked, whose taffy resolution is read on its own
@task_item n=1 part=needs
The layout engine's Taffy version changes for every document, remote pages and the stand pages included: every laid-out box, bound and position the stands report may move, and the measurement names which checks change.
@task_item n=2 part=needs
The stand checks are among the gates re-measured: each stand task page laid out by the new Taffy must still pass its checks, or the change is named in the measurement.
@task_item n=3 part=needs
The headless stand's pinned viewport is the size the stand checks lay out at during the measurement; the move must not change that viewport, only what Taffy lays out inside it.
@task_item n=4 part=needs
Geometry the stand reads from layout — open list rows reading their own bounds, the visible region taken through clipping boxes, into-view scroll moving every holding scroller — is layout-derived and may shift under the new Taffy; those checks are re-run, not rebuilt.
@task_item n=5 part=needs
Float placement behaviour changes: Taffy #1303 makes the float id caller-defined, and the inline layout passes the DOM node's taffy node id as place_floated_box's first argument.
@task_item n=6 part=needs
Each layout behaviour change the chain brings is a fork the measurement must name, not settle: whether the founder's Taffy answer stands as the ruling for those changes or each needs its own ratification.
@task_item n=7 part=needs
The no-git-dependency rule binds the move: taffy alone may take a git source; any second git source the move brings (a Taffy dependency from git, a patch section) is a fork for the forks file and stops the phase.
@task_item n=8 part=needs
--locked stays on every build, test and CI leg, so the lock change must be committed with the manifest change and stay minimal.
@task_item n=9 part=needs
MSRV stays rust 1.91: a Taffy pin needing a newer toolchain is a stop.
@task_item n=10 part=needs
The dependency audit keeps its advisories and licences checks and must pass with the git Taffy source.
@task_item n=11 part=needs
The incremental oracle invariant holds under the new Taffy: incremental relayout must still match a full layout.
@task_item n=12 part=needs
The wasm legs: the wasm example resolves taffy through its own lock, read on its own; a wasm leg the move breaks is a stop.
@task_item n=13 part=needs
The WPT runner must still build against the moved Taffy.
@task_item n=14 part=needs
Stop rule: a red gate or leg with no in-scope fix, an MSRV above 1.91, a broken wasm leg, or a lock move beyond Taffy that a master rule refuses ends the phase with the measurement, planning nothing around it.
@task_item n=1 part=not_needed
Window chrome controls of the shell provider (drag, maximise, minimise, close, decorations): the move changes no shell provider call.
@task_item n=2 part=not_needed
Stand stable element ids and the rule that every stand control is keyed with ids as attributes alone: no element id is added, renamed or removed.
@task_item n=3 part=not_needed
Stand accessibility attributes, the stand accent colour, the flight type select's boot reading and the CRUD list as one keyboard region: no stand markup, style value or keyboard handling changes.
@task_item n=4 part=not_needed
The cold-agent pipe and agent-run test script contracts (usage, exit codes, stdout, record directory, Windows pass-through): no script or its arguments change.
@task_item n=5 part=not_needed
The escher-session command line, its MCP server protocol, tools list, exit codes, refusals and recording format: the session driver's surface is untouched by a dependency move.
@task_item n=6 part=not_needed
The multi_doc and paint_bench command lines (arguments, environment variables, backend, stdout): no bench or its interface changes.
@task_item n=7 part=not_needed
The absence of arbitrary index.html templates and the fixed document skeleton with main mount: no document template or mount changes.
@task_item n=8 part=not_needed
Network provider placement on the load page and session roles sharing the stand boot: no provider or boot path changes.
