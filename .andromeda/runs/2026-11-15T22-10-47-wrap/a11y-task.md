@task_prose part=session
The session moved the Taffy layout engine from the crates.io registry release 0.15.1 to upstream's git source at rev e90751fd (package 0.15.0), the workspace's one git dependency, under the Taffy-only exception.
It took eight upstream blitz commits by cherry-pick, the lockfile the only conflict in each, and with them Taffy #1303: the engine's inline layout pass now hands Taffy a caller-defined float id when it places a floated box.
The eight commits' layout changes reach every document's float placement; the founder ratified them for this take (F2), on the condition that the gates, the stand, the a11y leg and CI are green, and all held.
Every test tally stayed equal to the base: the workspace tests, the embolden check, the a11y leg (14 binaries, 115 tests) and the stand (294 passed), with the stand repeated three of three rounds green.
Upstream's later Taffy chain from #1372 (reversed scroll origins and its five later bumps) was left out by design and waits for its own route entry.
No accessibility code, ARIA reading, accessibility tree builder, snapshot reader or contrast check was edited; no capability was added or retired.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace manifest, which pins the engine dependencies; its Taffy layout engine line moved from a registry version to upstream's git source at a fixed rev
@task_file n=2 path=Cargo.lock mark=conflict
what: the workspace lockfile, the resolved dependency set with each source and checksum; the Taffy package now resolves from git with no checksum line, the conflict of each pick resolved ours and re-resolved
@task_file n=3 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the engine DOM's inline layout pass, where floated boxes are placed through Taffy; its one float placement call now passes the node's own id as the float id
@task_item n=1 part=touches
Indirect, through float placement in the inline layout pass: rows whose readings rest on laid-out boxes — the snapshot's bounds of a closed select's options, an open list's rows and a multiple select's rows and groups — read layout that the eight commits changed for every document; the stand and a11y leg stayed green, so no reading moved.
@task_item n=2 part=touches
Indirect, through layout: the lint's contrast finding and its bounds, and stand contrast measured from computed styles and asserted per SC 1.4.3, all run over the re-laid documents; the stand reads no lint finding at boot still held.
@task_item n=3 part=touches
Indirect, through layout and hit-testing: the synthetic click at a node's centre, the driver click refusal for a covered node and an inert element taking no hit depend on box positions from the changed layout engine.
@task_item n=4 part=touches
Indirect, through the inline layout pass: hidpi or zoom change reshaping text and open select list rows taking one height rest on inline layout that the taken commits changed.
@task_item n=5 part=touches
The a11y CI leg, the accessibility test leg in the slow tier and the stand checks repeated three rounds served as part of the founder's ratification condition for this upstream layout take (gates, stand, a11y leg and CI green); each ran green, their tallies unmoved.
@task_item n=6 part=touches
Accessibility checks gate CI: the pushed head's CI run passed 17 of 17 checks with this change, the a11y leg among them; no threshold or bound a row holds moved.
@task_item n=7 part=untouched
ARIA role mapping, role attribute precedence and HTML-AAM native roles: no role reading changed.
@task_item n=8 part=untouched
ARIA state and property reads (checked, pressed, selected, invalid, expanded, disabled, describedby, labelledby, level) and the accessible name precedence: untouched.
@task_item n=9 part=untouched
Keyboard navigation: focusability, Tab order, keyboard activation, range input keys and the select list's open and closed key contracts, and the seven_guis Cells and CRUD key contracts: no key or focus behaviour changed.
@task_item n=10 part=untouched
The snapshot text form, its state tokens, the driver schema, the actions answer, handler coverage, the UI graph and the MCP results: no field or wire form changed.
@task_item n=11 part=untouched
Password masking, hidden and inert content exclusion from the tree, and the platform adapter's creation order and event routing: untouched.
@task_item n=12 part=untouched
The WCAG contrast ratio helper, its required ratios and thresholds, the stand accent colour and the select list's colour constants: no value changed.
@task_item n=13 part=untouched
Cognitive accessibility and the obs log format the harness contract's overview describes: untouched.
