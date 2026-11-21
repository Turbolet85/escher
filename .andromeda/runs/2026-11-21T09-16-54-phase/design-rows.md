# design-system · the rows that bind 2026-11-21-engine-debts — picked 12 of 139 (4 by every draw) · rule any · pin 2822

## typography · ## Typography
contract/script-runtime/document-fonts · script runtime DOM · document.fonts — `document.fonts` is a stub FontFaceSet in which all fonts report as loaded.

## surface · ## Surface: desktop-native · twin surface-desktop-native
decision/boxless-focused-select-keeps-focus · Focused select with no box keeps focus and keys — A focused select whose container lost its box keeps focus and takes a key with no panic. · why: pinned by a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes, which presses a key after hiding the select's container
decision/buffered-render-replay · Render buffered, then replayed under one borrow — On every Dioxus document, windowed and headless alike, a render is recorded into a mutation buffer with the document unborrowed, then replayed under one borrow. · why: so code the vdom runs during a render, a component body or a polled task can call MountedData's methods with no RefCell already borrowed panic
contract/mounteddata-during-render/document-state · MountedData calls during a render · document state — A MountedData call made during a render acts on the document as the previous render left it, so a node created in that same render is not yet in it.
contract/mounteddata-during-render/set-focus-to · MountedData calls during a render · set_focus_to — set_focus_to returns false for a node created in the same render.
decision/actions-share-the-verbs-checks · actions decided by the verbs' own checks — The actions answer reports whether each call would be accepted, not what it would do, deciding a node's verbs by the same validation and per-verb target check every id-taking verb meets before it acts. · why: so a listed cause is the cause the call returns
decision/default-ua-sheet-via-default-config · Default user-agent sheet through the default config — Every document gets the blitz default user-agent stylesheet once, through the default document config. · why: pinned by default_config_uses_the_default_sheet; the test harness and Dioxus documents take the sheet only through DocumentConfig default
decision/selects-that-open-no-list · Selects that open no list — A disabled, inert or hidden select, a select with no row to show and a multiple select open no list. · why: pinned by a_disabled_inert_hidden_singular_or_empty_select_opens_nothing in the select_popup tests
contract/js-window/device-pixel-ratio · The JS window object · devicePixelRatio — devicePixelRatio is the scale.
contract/js-window/inner-width-height · The JS window object · innerWidth / innerHeight — innerWidth and innerHeight are the window size divided by the scale.
contract/js-window/outer-width-height · The JS window object · outerWidth / outerHeight — outerWidth and outerHeight alias innerWidth and innerHeight.
contract/select-list-events/close-on-select-change · A single select's list events · close on its select's change — Removing, disabling, hiding or making its select inert closes the list.

# design-system · its absences, names and overviews, whole

## brand-identity · ## Brand Identity
> NO RECORDED INTENT

## color-palette · ## Color Palette
Border Progression › > NOT YET MEASURED — a project-wide UI colour token set: the examples slice (s01) and the DOM-node slice (s08) recorded the project's own colour tokens as out of slice, and the default user-agent stylesheet's full contents were out of slice for s05.
absence/engine-color-tokens — Color tokens or CSS custom properties in the engine and integration crates

## typography · ## Typography
> NOT YET MEASURED — a project-wide type scale: the examples slice (s01) recorded the project's own type scale as out of slice, and the WPT runner slice (s13) recorded typography as out of slice.
absence/project-type-scale — A project type scale or font tokens

## spacing · ## Spacing
> NOT YET MEASURED — a project-wide spacing scale or base unit: the examples, engine, test and WPT slices (s01, s05, s07, s10, s11, s12, s13) recorded a spacing scale as out of slice.

## depth-strategy · ## Depth Strategy
> NO RECORDED INTENT ¶ > NOT YET MEASURED — a project-wide elevation scale: the examples, engine-resolve, layout and WPT slices (s01, s05, s07, s13) recorded it as out of slice.

## border-radius · ## Border Radius
> NO RECORDED INTENT ¶ > NOT YET MEASURED — a project-wide radius scale: the examples, test and WPT slices (s01, s12, s13) recorded radius tokens as out of slice.

## motion-calibrated-to-expression-level-unrecorded · ## Motion (calibrated to expression level `unrecorded`) · twin motion
> NO RECORDED INTENT
absence/reduced-motion-handling — Reduced-motion handling

## surface-cli · ## Surface: cli
Navigation Pattern › > NOT YET MEASURED — no slice recorded argument parsing or command structure for the CLI binaries. ¶ Platform-Specific Notes › > NOT YET MEASURED — no slice recorded terminal-width handling or platform differences for the CLI binaries.

## surface-web-spa · ## Surface: web-spa
Navigation Pattern › > NOT YET MEASURED — no slice recorded routing or navigation for the WASM pages. ¶ Platform-Specific Notes › > NOT YET MEASURED — no slice recorded browser-specific limitations for the WASM pages.
overview/surface-web-spa — The web-spa surface is WASM pages rendering into a canvas: on wasm32 the dioxus-native surface is a canvas appended to the page body and sized from host CSS, so a page such as todomvc fills the window with that canvas.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
> NO RECORDED INTENT

## self-validation-protocol · ## Self-Validation Protocol
> NO RECORDED INTENT

## design-decisions-log · ## Design Decisions Log
> NO RECORDED INTENT
