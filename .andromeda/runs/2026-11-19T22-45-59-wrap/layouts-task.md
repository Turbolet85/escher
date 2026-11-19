@task_prose part=session
The session merged upstream main (547d7514b) into the 0.4.0 branch as a two-parent merge, CI green 17 of 17 on the first attempt, every left-out upstream commit named with its reason.
It took upstream's event-queue model: every escher event path (pointer down and up, click, key up, select choice, row gesture, range move, generated text input) queues its events instead of taking a dispatch callback, and programmatic focus now fires blur, focusout, focus and focusin; the inert fix-up and the driver's select focus stay silent through two new silent focus calls.
The accessibility tree is no longer refreshed on poll: the shell window pushes it after resolve on redraw when the document's changed set is non-empty, then clears the set.
It fixed an upstream defect where an inline-block inside text no loaded font can shape was never placed: a private placeholder font family is registered in every font context, a given one included; proven to move no reading where a font exists.
It bumped Stylo 0.22 to 0.24, parley 0.12 to 0.13, skrifa 0.44 to 0.49, winit beta.3 to beta.4, svgtypes to 0.16.2, held Dioxus 0.7.3, moved the network layer onto rustls, and made Skia the default renderer of the browser app and the wpt runner.
It added upstream's devtools inspector crate (a CDP server on 127.0.0.1, opt-in by the devtools feature, no default build turns it on) and retired the parley code editor crate and the agents guide; script-dispatched events now read isTrusted false, a cancelled pointerup still clicks, and visibility collapse hides a node from the accessibility tree.
@task_file n=1 path=packages/blitz-dom/src/document.rs
what: the engine document: its focus state and focus calls, its changed-node set, its event queue, its font contexts and the fonts registered at boot
@task_file n=2 path=packages/blitz-dom/src/layout/inline/build.rs
what: the inline layout builder: how text runs and inline boxes (inline-blocks inside text) are pushed to the text shaper
@task_file n=3 path=packages/blitz-dom/src/lib.rs
what: the engine crate root: its module list and crate-wide constants, now the placeholder font family name
@task_file n=4 path=packages/blitz-dom/src/document_inline_box_placeholder_tests.rs
what: a new engine check that an inline-block in text no font can shape is placed once its own font loads, in both layout modes
@task_file n=5 path=packages/blitz-dom/src/events/pointer.rs
what: pointer down, up and click handling: focus on press, click synthesis, select and range gestures, scrollbar drag
@task_file n=6 path=packages/blitz-dom/src/events/keyboard.rs
what: keyboard event handling: key down and up, focus moves by keyboard, text input generation
@task_file n=7 path=packages/blitz-dom/src/events/mod.rs
what: the engine's event entry point: routes each UI event to its handler and applies default actions unless prevented
@task_file n=8 path=packages/blitz-dom/src/events/focus.rs
what: the old focus event generator (deleted; focus events now come from the document's focus calls)
@task_file n=9 path=packages/blitz-dom/src/events/wheel.rs mark=merge
what: wheel handling: gesture axis lock and scroll of the scroller under the pointer
@task_file n=10 path=packages/blitz-dom/src/scrolling.rs
what: scrolling: scroll into view, smooth scroll animation by the document clock, scroll chaining and overscroll behaviour
@task_file n=11 path=packages/blitz-dom/src/node/select_list.rs
what: the select element's open list: its rows, their bounds and the chosen option
@task_file n=12 path=packages/blitz-dom/src/node/node.rs
what: the engine node: its tree links, scroll container test and overscroll containment
@task_file n=13 path=packages/blitz-dom/src/accessibility.rs
what: the accessibility tree builder: names, roles and states from attributes, and which nodes are hidden from the tree
@task_file n=14 path=packages/blitz-dom/src/mutator.rs
what: the document mutator: DOM edits, the inert fix-up on flush, and mutation records for the devtools
@task_file n=15 path=packages/blitz-dom/src/stylo.rs
what: the style engine glue (deleted; split into a style module with device, DOM, prefs, presentational hints, SVG attributes, traversal and user-agent sheet)
@task_file n=16 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=merge
what: presentational hints: legacy HTML attributes and SVG presentation attributes turned into style, now hr and SVG attributes too
@task_file n=17 path=packages/blitz-dom/assets/default.css
what: the engine's default user-agent stylesheet
@task_file n=18 path=packages/blitz-dom/src/layout/construct.rs
what: layout tree construction: boxes built from the DOM, anonymous blocks and inline boxes
@task_file n=19 path=packages/blitz-paint/src/render.rs
what: the paint pass: node transforms, scrolled transforms, clipping of a node, SVG images and text masks
@task_file n=20 path=packages/blitz-paint/src/render/select_list.rs
what: paint of the open select list
@task_file n=21 path=packages/blitz-paint/src/text.rs
what: text paint: glyph runs, stroke, synthetic bold and text decorations
@task_file n=22 path=packages/blitz-shell/src/window.rs
what: the desktop shell window view: poll, redraw, the accessibility push after resolve, the safe-area content size
@task_file n=23 path=packages/blitz-shell/src/devtools.rs mark=merge
what: the shell's devtools: starting the CDP server from its port setting, opening the inspector, the element picker
@task_file n=24 path=packages/blitz-traits/src/shell.rs mark=merge
what: the ShellProvider trait the shell gives a document: window chrome controls, now also open devtools
@task_file n=25 path=packages/blitz-vibey-cdp-server/Cargo.toml mark=merge
what: the new devtools inspector crate: a Chrome DevTools Protocol server over a local TCP listener, opt-in by feature
@task_file n=26 path=packages/escher-driver/src/execute.rs
what: the driver's action executor: the actions an agent session runs on the stand, select focus among them
@task_file n=27 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus document: the stand app's DOM host, its event flush to Dioxus handlers
@task_file n=28 path=packages/dioxus-native/Cargo.toml
what: the Dioxus native launcher's build features: clipboard dropped from its default, devtools opt-in
@task_file n=29 path=examples/seven_guis/src/app.rs
what: the seven GUIs stand app: its task cards, element ids, keys and accent (kept as it stood; the merge's side not taken)
@task_file n=30 path=apps/browser/Cargo.toml
what: the browser app's build features: Skia now in its default, devtools left out
@task_file n=31 path=apps/browser/src/tab.rs
what: the browser app's tab: one page's document and its loading
@task_file n=32 path=packages/blitz-net/src/lib.rs
what: the network provider: fetches stamped with their document, its TLS now rustls
@task_file n=33 path=tests/blitz-tests/tests/svg_mask_attribute_fetch.rs
what: a new check that an SVG mask attribute URL is fetched stamped with its document, a fragment-only URL as a stamped data URL
@task_file n=34 path=tests/blitz-tests/tests/all.rs
what: the list of engine integration test targets
@task_file n=35 path=Cargo.toml
what: the workspace manifest: its members and pinned engine dependency versions
@task_file n=36 path=Cargo.lock
what: the workspace lock: the resolved dependency set
@task_file n=37 path=deny.toml
what: the dependency audit policy: the licence table and the advisory ignores
@task_file n=38 path=.github/workflows/ci.yml
what: the CI workflow: its legs and the Android job
@task_file n=39 path=wpt/runner/src/main.rs
what: the web platform test runner command: test selection by manifest and its renderer choice
@task_file n=40 path=packages/parley_code_editor/Cargo.toml
what: the parley code editor crate (deleted with the whole crate)
@task_item n=1 part=touches
decision/flight-type-select-boot-reading: the driver's select focus now goes through a silent focus call that queues no focus event; the boot reading of the flight type select's two options is to be re-read against it.
@task_item n=2 part=touches
decision/open-list-rows-own-bounds: the select list node, its paint and the select choice path (now queued through the event queue) changed; the rule over the open list's row bounds is to be re-read.
@task_item n=3 part=touches
decision/crud-list-keyboard-region: keyboard and pointer handling moved to the event queue and programmatic focus now fires blur, focusout, focus and focusin; a rule over one keyboard region and its focus moves is reached indirectly.
@task_item n=4 part=touches
decision/stand-a11y-by-attributes-alone: the accessibility tree is now pushed on redraw after resolve, built from the wrapping document, and a node with visibility collapse is hidden from it; the rule naming stand controls by attributes rides that builder.
@task_item n=5 part=touches
decision/scroll-into-view-moves-every-scroller: scrolling changed (smooth scroll reads the document clock, scroll chaining and overscroll behaviour per axis, a scroll container test on the node); the into-view rule over every holding scroller is reached indirectly.
@task_item n=6 part=touches
decision/visible-region-through-clipping-boxes: paint clipping, the scroll container test and the clipped-hit check were edited by the merge; the visible-region rule through clipping boxes is reached indirectly.
@task_item n=7 part=touches
decision/headless-stand-pinned-viewport: a placeholder font family is now registered in every font context, a given one included, and the text shaper moved to parley 0.13; readings held in both layout modes, so the pinned-viewport stand is reached only as a re-read.
@task_item n=8 part=touches
contract/escher-session-refusals/fonts-unavailable: the engine now registers its bullet font under a private placeholder family in a given font context too; a refusal that turns on fonts being unavailable is to be checked against it.
@task_item n=9 part=touches
decision/held-instance-reads-as-fresh-boot: the document's changed set is now cleared on redraw and events wait in a queue until flushed; a held instance reading as a fresh boot is reached indirectly.
@task_item n=10 part=touches
decision/stand-lints-clean-at-boot: the default user-agent stylesheet changed and presentational hints now cover hr and SVG presentation attributes; a lint-clean boot is reached indirectly.
@task_item n=11 part=touches
decision/every-stand-control-keyed and decision/stand-ids-as-attributes-alone: the stand app was kept as it stood through the merge, the Dioxus document re-applied over upstream's; the stand's keys and ids read the same, a re-read only.
@task_item n=12 part=touches
contract/stand-element-ids: the stand app kept as it stood; every id in the stable element ids list stands, a re-read only.
@task_item n=13 part=touches
decision/stand-accent-colour: the default user-agent stylesheet and select list paint changed; the stand accent colour is reached indirectly.
@task_item n=14 part=touches
decision/fixed-document-skeleton-main-mount: the Dioxus document gained a pending-event flush and the launcher dropped clipboard from its default; the document skeleton with its main mount is reached indirectly.
@task_item n=15 part=touches
decision/load-page-only-network: the network provider moved to rustls and an SVG mask attribute URL now reaches the fetch stamped with its document (a fragment-only one as a stamped data URL); the rule that the load page alone has a network provider is to be re-read.
@task_item n=16 part=touches
contract/shell-provider-window-chrome: the ShellProvider trait gained open devtools beside the six window chrome controls; something now belongs beside those items.
@task_item n=17 part=touches
decision/mcp-server-local-stdio: a second server now exists in the tree, the devtools CDP server on a TCP listener at 127.0.0.1, opt-in by feature, no authentication; a row stating the only server or the only listener is stale.
@task_item n=18 part=touches
contract/escher-session-actions/answer: the driver's actions run through the event queue and its select focus is silent; an action's answer is reached indirectly.
@task_item n=19 part=touches
contract/escher-session-snapshot/answer and contract/escher-session-layout/answer: the accessibility refresh site moved to redraw and inline-blocks in unshaped text are now placed; a snapshot or layout answer is reached indirectly.
@task_item n=20 part=touches
contract/escher-session-screenshot/answer and contract/escher-session-mcp/screenshot-result: text paint, clipping and masks were reworked by the merge; a screenshot answer is reached indirectly.
@task_item n=21 part=touches
contract/paint-bench-cli/backend: Skia is now built by default in the browser app and the wpt runner and tiny-skia was added as a renderer; a backend list a row holds is to be checked.
@task_item n=22 part=touches
decision/session-recording-replayable-check: events are now queued, script-dispatched events read isTrusted false and a cancelled pointerup still clicks; a recording replayed as a check is reached indirectly.
@task_item n=23 part=touches
Any row whose cell holds an engine version (Stylo 0.22, parley 0.12, skrifa 0.44, winit =0.31.0-beta.3, svgtypes 0.16.1) is stale: each now reads the merged value (0.24, 0.13, 0.49, =0.31.0-beta.4, 0.16.2); Dioxus stays 0.7.3.
@task_item n=24 part=touches
The names blitz_vibey_cdp_server, BLITZ_CDP_PORT and BLITZ_DEVTOOLS_FRONTEND_DIR are now taken; the name parley_code_editor is freed.
@task_fact n=1 slug=surface-desktop-native
fact: The browser app's default build paints through Skia and carries no devtools; the devtools inspector (a CDP server on 127.0.0.1) is opt-in by the devtools feature and opened from a toolbar item or the Chrome devtools shortcuts.
@task_fact n=2 slug=surface-desktop-native
fact: Programmatic focus fires blur, focusout, focus and focusin; the inert fix-up and the driver's select focus are the exceptions and fire none.
@task_item n=1 part=untouched
The agent-run and cold-agent command contracts (usage, exit codes, stdout, run log, record dir, Windows pass-through): their scripts did not change and the boot smoke stayed green.
@task_item n=2 part=untouched
The cold-agent decisions (the pipe builds nothing, the prompt names no id, the agent drives through the driver's MCP server): no change reached the cold-agent pipe.
@task_item n=3 part=untouched
The escher-session command line, help, usage, coverage role and exit codes 0 to 3: no argument parsing or exit path moved.
@task_item n=4 part=untouched
The escher-session MCP protocol revisions, tools list, result bound and end: the MCP server itself is unchanged.
@task_item n=5 part=untouched
The multi_doc command line (arguments, MULTI_DOC_FLAT, MULTI_DOC_N, MULTI_DOC_SEQ, stdout): no change reached it.
@task_item n=6 part=untouched
The escher-session host log service name and the recording files (check file, record dir not private, recording failed): untouched.
@task_item n=7 part=untouched
absence/arbitrary-index-html-templates: no document template capability was added.
@task_item n=8 part=untouched
The web-spa surface: no navigation, header, footer or page structure of a web app was reached.
