@task_prose part=session
The session is the eleventh upstream take. It cherry-picked 3 of the 16 new upstream commits (pin moved to ea07ee716) and left 13 out, so 112 are now left out, each with a measured reason.
One picked commit reads the @font-face font-stretch (font-width) descriptor into font matching as a new width override, so every document's text can match a different face width. The founder ratified this.
The taffy layout engine moved 0.15.0 to 0.15.1 from the crates.io registry, a lock-only semver-compatible update. It carries upstream's Taffy-main layout fixes except #1303 (the caller-defined float id), which changes layout for every document. The founder ratified this.
The pinned WPT commit moved, and the WPT timeout quarantine gained four entries with reason dynamic-load-events.
The git-Taffy take was measured and left out. The founder then allowed an exception for Taffy alone to the no-git-dependency rule; a later chunk owes the move to git Taffy.
No CLI, MCP verb, stand element id, window chrome control or test count changed: fast 1461/0/13, run stand 294/0/8 and run all 825/0/13 all passed, the same as before.
@task_file n=1 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine's document, which applies a web font's @font-face overrides (family, weight, style, now width) when it registers the font for matching
@task_file n=2 path=packages/blitz-dom/src/net.rs mark=merge
what: the document's network provider and the fetch of @font-face rules, which reads each rule's descriptors into the font overrides
@task_file n=3 path=wpt/WPT_COMMIT mark=merge
what: the pinned WPT commit that the conformance suite runs against
@task_file n=4 path=wpt/runner/timeout-quarantine.txt mark=merge
what: the WPT runner's timeout quarantine list, one test path and a known reason per entry
@task_file n=5 path=Cargo.lock
what: the workspace lockfile, which resolves taffy (the layout engine) to a crates.io registry version with its checksum
@task_item n=1 part=touches
Indirect, through the layout engine: taffy 0.15.1's layout fixes change the box geometry of every document. Any row that rests on laid-out bounds can be affected: open list rows reading their own bounds, the visible region found through clipping boxes, and into-view scroll moving every scroller that holds the target. The stand runs stayed green with no count moved.
@task_item n=2 part=touches
Indirect, through the layout engine and font matching: the headless stand's pinned viewport and the fixed document skeleton with its main mount now lay out under taffy 0.15.1 and the new font-width matching. A row that holds a pinned size or a layout value read from it should be checked.
@task_item n=3 part=touches
Indirect, through font matching: an @font-face rule's font-stretch descriptor now reaches matching, so the escher-session screenshot and snapshot answers, and any geometry they report, can differ for pages that declare font-stretch. The fonts-unavailable refusal is a rule about font availability, so check it next to this.
@task_item n=4 part=touches
Indirect, through the network provider: the load page is the only page with a network provider, and that provider's @font-face fetch now reads one more descriptor (width). No new request, endpoint or log site was added.
@task_item n=5 part=touches
Indirect, through layout: the flight type select reading its two options at boot, and the held instance reading as a fresh boot, both rest on boot-time layout, which now runs under taffy 0.15.1. Their stand checks passed unchanged.
@task_item n=6 part=touches
Indirect, through the engine version: a row that names the layout engine's version (Taffy 0.15 from the registry) is still true at 0.15.1. A row that names the resolved version 0.15.0, or says no git source can ever be used, is now stale given the Taffy-only exception.
@task_item n=1 part=untouched
Stand stable element ids: no id was added, renamed or removed.
@task_item n=2 part=untouched
ShellProvider window chrome controls (drag, maximize, minimize, close, decorations): no window chrome control moved.
@task_item n=3 part=untouched
The agent-run.sh and cold-agent.sh test and run pipe contracts (usage, exit codes, stdout, record and log paths, Windows pass-through), and the cold-agent rules: no script or exit code changed.
@task_item n=4 part=untouched
The escher-session command line, exit codes, help, lint, graph, coverage role, recording, served tasks, host log service.name and the MCP server's tools, protocol revisions, result bound and stdio: no verb, field, bound or code changed.
@task_item n=5 part=untouched
The multi_doc and paint_bench command lines: no argument, environment variable or output changed.
@task_item n=6 part=untouched
Stand accent colour, keying every stand control, attribute-only stand ids and accessibility, the CRUD list as one keyboard region, and lint-clean boot: no stand markup, colour or keyboard behaviour changed.
@task_item n=7 part=untouched
The mobile-native and web-spa surfaces, the absence of arbitrary index.html templates, and the Decisions Log: the session adds no surface, template or recorded intent.
