@task_prose part=session
The session moved the layout engine dependency Taffy from the crates.io registry 0.15.1 to upstream's git source at rev e90751fd (package version 0.15.0), the workspace's one git source, under the Taffy-only exception.
It took eight upstream blitz commits by no-commit cherry-pick, the lock the only conflict in each, re-resolved; their layout changes for every document land ratified by the founder, not provisional.
The one code change: the inline layout pass now hands Taffy's float placement a caller-defined float id (Taffy #1303) built from the node id.
The later Taffy chain from #1372 (reversed scroll origins, its panic fix and five later bumps) was left out by design, waiting for its own route entry.
Measured: every test tally, the stand (294 passed, three of three repeat rounds green), the a11y leg, the windowed smoke and CI all equal to or as green as before; no public symbol, crate, schema, gate or harness surface moved.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace manifest — its dependency list, where the layout engine Taffy is pinned (now a git source at a fixed rev)
@task_file n=2 path=Cargo.lock mark=conflict
what: the workspace lockfile — the resolved package set and each package's source and checksum; the taffy package now from git with no checksum
@task_file n=3 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the DOM's inline layout pass — line layout of inline content and placement of floated boxes through Taffy, now passing each float its own id
@task_item n=1 part=touches
A rule that a document's layout (box placement, float placement, line layout) is the engine's own and unchanged across a dependency take: the eight upstream commits change layout for every document, ratified for this take.
@task_item n=2 part=touches
A row whose cell names the layout engine Taffy as a registry version or as 0.15.1: Taffy is now git 0.15.0 at e90751fd.
@task_item n=3 part=touches
Indirect: the stand's rows that read element bounds after layout (open list rows reading their own bounds, the visible region through clipping boxes) rest on the layout pass whose float placement changed; the stand stayed green, so no bound or value moved.
@task_item n=4 part=touches
Indirect: the headless stand's pinned viewport and the fixed document skeleton with its main mount are laid out by the changed engine; the stand passed unchanged at 294, so a row holding a stand count or viewport value is unmoved.
@task_item n=5 part=touches
Indirect: the into-view scroll that moves every holding scroller depends on scroll origins; the reversed-scroll-origin change of Taffy #1372 is deliberately not taken here and waits for its own entry, so this row stands as written.
@task_item n=6 part=touches
Indirect: the escher-session host log's service.name contract was re-proven by the windowed smoke (seven_guis logged its service.name); no value moved.
@task_item n=1 part=untouched
The stand stable element ids and every stand-element-ids contract item: no id was added, renamed or freed.
@task_item n=2 part=untouched
The ShellProvider window chrome controls: no window chrome call moved.
@task_item n=3 part=untouched
The cli surface's contracts — agent-run.sh, cold-agent.sh, escher-session commands, exit codes, MCP server, recording, refusals, served tasks, multi_doc and paint_bench command lines: no command line, exit code, tool or answer shape changed.
@task_item n=4 part=untouched
The stand's accessibility, keying, lint and accent-colour decisions: no attribute, key, lint or colour moved, and the a11y leg stayed green at 115.
@task_item n=5 part=untouched
The absence of arbitrary index.html templates for a document: no template capability was added.
@task_item n=6 part=untouched
The mobile-native and web-spa surfaces and the decisions log: not yet measured or holding no intent, and the session reached none of them.
