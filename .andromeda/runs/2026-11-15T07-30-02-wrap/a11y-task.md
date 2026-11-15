@task_prose part=session
The eleventh upstream take: upstream/main read at ea07ee716, 3 commits cherry-picked and 13 more left out, each with a measured reason, so 112 are left out in all.
Engine behaviour change for every document, ratified by the founder: an @font-face rule's font-stretch (font-width) descriptor now reaches font matching beside family, weight and style.
Engine behaviour change for every document, ratified: the Taffy layout crate moves 0.15.0 to 0.15.1 from the crates.io registry, a lock-only move that brings 13 of upstream's 14 Taffy-main layout fixes. No package enters or leaves, and no manifest moves.
The WPT pinned commit moves, and the WPT timeout quarantine gains four dynamic-load-events entries.
The founder lifted the no-git-dependency rule for the Taffy crate alone. A later chunk takes git Taffy. No source of this chunk changes because of it.
Every gate is green, the a11y leg included. Test counts are unchanged on every OS, and no log site, endpoint, verb or carrier field was added.
@task_file n=1 path=packages/blitz-dom/src/net.rs mark=merge
what: the engine's resource loading: it reads @font-face rules and their descriptors (family, weight, style and now width) into the font overrides that font matching uses
@task_file n=2 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine document: where it registers @font-face fonts with their overrides, and the unit fixture of those overrides
@task_file n=3 path=wpt/WPT_COMMIT mark=merge
what: the pinned WPT commit that the conformance runner checks out
@task_file n=4 path=wpt/runner/timeout-quarantine.txt mark=merge
what: the WPT runner's timeout quarantine list: test paths, each with a known reason
@task_file n=5 path=Cargo.lock
what: the workspace lockfile: resolved crate versions and checksums, with the Taffy layout crate now at registry 0.15.1
@task_item n=1 part=touches
Indirect: the Taffy layout fixes change box layout for every document. Check any row whose rule or cell rests on laid-out bounds, such as snapshot node, row or option bounds, a lint finding's bounds, or a synthetic click at a node's centre.
@task_item n=2 part=touches
Indirect: the Taffy layout fixes may move heights or positions inside the engine's open select overlay list. Check rows on its row height and row bounds.
@task_item n=3 part=touches
Indirect: font matching now reads the font-stretch width of an @font-face rule. Check rows on text shaping or reshaping, such as a hidpi or zoom change reshaping text, and rows on text measured from computed styles.
@task_item n=4 part=touches
Indirect: the stand's contrast, focus-order and lint-clean-at-boot checks run over the newly laid-out documents. They read green with no count moved. A row is touched only where its cell holds a value or bound that this layout could shift.
@task_item n=5 part=touches
Indirect: rows that treat the accessibility tree, the platform adapter or the accessibility feature as part of the engine dependency set. The accessibility crates did not move, and the only crate that moved is Taffy, a lock-only semver-compatible move.
@task_item n=6 part=touches
Indirect: the a11y CI leg and the stand checks repeated three rounds both ran green on the take. Check rows that hold a pass count or a threshold for that leg.
@task_item n=1 part=untouched
ARIA role mapping, ARIA state and property reads, and hidden or inert content: no role, state, name or hidden rule was read or changed.
@task_item n=2 part=untouched
Keyboard navigation: Tab order, keyboard activation, range input keys, the select list keys and focus rules are unchanged.
@task_item n=3 part=untouched
The snapshot model, its text form, the actions answer, driver refusals, the UI graph and the MCP results: no field, token, verb or carrier changed.
@task_item n=4 part=untouched
The accessibility feature defaults, AccessKit as the tree, accesskit_xplat as the adapter, and the stand's lack of a platform adapter: no feature flag or accessibility crate moved.
@task_item n=5 part=untouched
WCAG contrast thresholds, the contrast formula, the stand accent colour and the select list colours: no colour or ratio constant changed.
@task_item n=6 part=untouched
Cognitive accessibility, the invalid-state rules and the shared tab-order table: nothing changed here.
