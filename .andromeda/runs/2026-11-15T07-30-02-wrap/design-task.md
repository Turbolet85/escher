@task_prose part=session
The eleventh upstream take: upstream main read at ea07ee716, three commits cherry-picked as upstream wrote them, thirteen new commits left out, each with a measured reason.
One pick makes an @font-face rule's font-stretch (font-width) descriptor reach font matching, so a web font face is now registered under its declared width as well as its family, weight and style; the founder ratified it as a behaviour change for every document.
The layout engine Taffy moved from registry 0.15.0 to registry 0.15.1, a lock-only semver-compatible update that carries upstream's Taffy layout fixes for every document; the founder ratified it too.
The pinned web-platform-tests commit moved, and the WPT runner's timeout quarantine gained four tests, each with the reason dynamic-load-events.
The git Taffy take was measured green but left out under the no-git-dependency rule; the founder has since lifted that rule for Taffy alone, and a later chunk will measure the move to git Taffy.
No IPC method, verb, endpoint, env var, log field or span was added, and nothing was retired.
@task_file n=1 path=packages/blitz-dom/src/net.rs mark=merge
what: the engine's resource loading, including the @font-face rule reader that registers web font faces and their CSS descriptors (family, weight, style and now width) for font matching
@task_file n=2 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine's DOM document, where font-face overrides are applied when a web font is registered, together with its font-face override unit tests
@task_file n=3 path=wpt/WPT_COMMIT mark=merge
what: the pinned web-platform-tests commit that the WPT runner runs against
@task_file n=4 path=wpt/runner/timeout-quarantine.txt mark=merge
what: the WPT runner's timeout quarantine list of tests that are not run, each with a known reason
@task_file n=5 path=Cargo.lock
what: the workspace lockfile resolving every crate to a registry version and checksum, including the Taffy layout engine (now 0.15.1)
@task_item n=1 part=touches
Typography, font matching: a web font face declared by @font-face is now matched by its font-stretch (font-width) as well as its family, weight and style. A row that states how a family or face is chosen or falls back (unavailable families, no generic family) may need this beside it.
@task_item n=2 part=touches
Typography, the script runtime's document.fonts contract: the faces it reports come from the same @font-face registration, which now carries a width descriptor.
@task_item n=3 part=touches
Typography, indirect: test engine and tests built with system fonts. The font-face override unit tests changed to name the width field (always empty), and no test sets a non-empty width.
@task_item n=4 part=touches
Typography, indirect: screenshot text painting and the fonts-unavailable refusals of the screenshot verb. A face matched by width can paint different glyphs, but no refusal or contract value moved.
@task_item n=5 part=touches
Spacing, indirect through the Taffy 0.15.1 layout fixes for every document: rows on box placement and spacing (the open select list placed inside the viewport, table cellspacing and cellpadding hints, body margins).
@task_item n=6 part=touches
Depth strategy, indirect through the layout fixes: paint order and hit-testing rows read the boxes that layout places (stacking contexts, paint clips, the open select list answering hits first).
@task_item n=7 part=touches
Surface desktop-native, indirect through the layout fixes: geometry that the snapshot and screenshot verbs report, such as the bounds of a row in an open select list and the logical viewport.
@task_item n=8 part=touches
Surface desktop-native, WPT runner: four more tests sit in the timeout quarantine with the reason dynamic-load-events, and the pinned WPT commit moved. The runner's output and result-colour rows (TIMEOUT, SKIP) are near this but no colour or output shape changed.
@task_fact n=1 slug=typography
fact: An @font-face rule registers its face under the CSS-declared font-stretch (font-width) as well as its family, weight and style, so font matching selects a web font face by its declared width; the founder ratified this as a behaviour change for every document.
@task_item n=1 part=untouched
Color palette: engine colours, select list colour constants, color-scheme recascade and base colour are unchanged.
@task_item n=2 part=untouched
Motion and the virtual clock: animation, scroll behaviour, scrollbar fade timings and the driver's advance, click, press and type verbs are unchanged.
@task_item n=3 part=untouched
Iconography: cursor icons, cursor resolution and the favicon read are unchanged.
@task_item n=4 part=untouched
escher-session and the CLI surface: commands, the stdout and stderr shapes, exit statuses and codes, the mcp role, tasks, and the actions, coverage and UI graph lines are unchanged.
@task_item n=5 part=untouched
Shell provider window controls, the JS window object, MountedData calls during a render, and select list events and accessibility are unchanged.
@task_item n=6 part=untouched
Brand identity, border radius, the web-spa surface, anti-patterns, self-validation and the design decisions log: none of them is reached.
@task_item n=7 part=untouched
Absences of color tokens, a project type scale and reduced-motion handling still hold; the session adds no tokens, scale or motion handling.
