# design-system · the rows 2026-11-14-remote-pages-local-files touches — picked 2 of 136 · rule any · pin 1036

## iconography · ## Iconography
decision/cursor-icon-crate · cursor-icon crate for cursor icons — Cursor icons are cursor-icon crate values, CSS cursor keywords map one-to-one to them, and cursor: none maps to no cursor. · r1
decision/cursor-resolution · Cursor resolution order — The cursor is taken from the CSS cursor keyword, else Text for text inputs, Pointer inside links, Text over selectable text, and Default otherwise, a pointer over no box included, so the shell keeps a visible arrow there. · why: Taken from upstream's #1220, ratified. · r1
decision/favicon-from-first-icon-link · Favicon read from the first icon link — The favicon read returns the href of the first link element whose rel contains icon. · why: the browser app's document loader takes favicon_url as its favicon candidate, which the tab strip and history show · r1 · add-to · F1
contract/shell-provider/set-cursor · ShellProvider · set_cursor — Takes an optional CursorIcon; None hides the cursor and resets it to Default. · r1

## surface-desktop-native · ## Surface: desktop-native · twin surface

## surface · ## Surface: desktop-native · twin surface-desktop-native
decision/actions-share-the-verbs-checks · actions decided by the verbs' own checks — The actions answer reports whether each call would be accepted, not what it would do, deciding a node's verbs by the same validation and per-verb target check every id-taking verb meets before it acts. · why: so a listed cause is the cause the call returns · r1
decision/boxless-focused-select-keeps-focus · Focused select with no box keeps focus and keys — A focused select whose container lost its box keeps focus and takes a key with no panic. · why: pinned by a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes, which presses a key after hiding the select's container · r1
decision/buffered-render-replay · Render buffered, then replayed under one borrow — On every Dioxus document, windowed and headless alike, a render is recorded into a mutation buffer with the document unborrowed, then replayed under one borrow. · why: so code the vdom runs during a render, a component body or a polled task can call MountedData's methods with no RefCell already borrowed panic · r1
decision/default-ua-sheet-via-default-config · Default user-agent sheet through the default config — Every document gets the blitz default user-agent stylesheet once, through the default document config. · why: pinned by default_config_uses_the_default_sheet; the test harness and Dioxus documents take the sheet only through DocumentConfig default · r1 · stale
decision/escher-session-agent-read-surface · escher-session is read by an agent, not a person — An escher-session command prints one line of JSON on stdout, the answer, the refusal or a session error's object keyed by the driver schema's own words, and nothing else there. · r1
decision/escher-session-no-colour · No colour and no terminal test on escher-session — escher-session uses no colour and no colour dependency, writes no ESC byte on any answer line or MCP reply, and never asks whether a stream is a terminal, so its output is the same piped or not. · guard: the seven_guis checks' shared reader and the shared Speaker assert no ESC byte on every line · r1
decision/exit-statuses-from-schema-table · Exit statuses read from the schema's table — The exit statuses are read from the driver schema's EXIT_STATUSES table, whose meanings help serves. · r1
decision/logical-viewport-from-scale · Logical viewport is physical size over total scale — The logical viewport size is the physical window size divided by hidpi scale times zoom. · why: the stylo device is built from logical_size; pinned by resize_and_zoom_coalesce in the device_coalescing tests · r1
decision/mobile-targets-built-not-tested · Mobile targets built, not tested — CI builds the iOS and Android targets but runs no tests on them. · r1
decision/select-list-engine-overlay · Select list as an engine overlay — On desktop-native a single select's open list is an engine overlay that reaches the screen through the scene paint alone, not a second OS window and not rows as boxes. · r1
decision/selects-that-open-no-list · Selects that open no list — A disabled, inert or hidden select, a select with no row to show and a multiple select open no list. · why: pinned by a_disabled_inert_hidden_singular_or_empty_select_opens_nothing in the select_popup tests · r1
decision/single-select-box-shows-chosen-label · Single select box shows its chosen option — A single-choice select lays out as a box holding its chosen option's label, from engine defaults with no design token. · why: pinned by a_select_lays_out_as_a_box_holding_its_chosen_option in the select_interaction tests · r1
decision/single-select-open-gestures · Click, Enter or Space opens a single select — A single select's list opens on a click, Enter, or a Space with no typed text standing. · why: pinned by a_click_enter_and_space_open_the_list_on_its_chosen_option_and_fire_nothing and enter_and_space_open_the_list_and_a_released_space_does_nothing_more · r1
decision/wpt-runner-secondary-text-bright-black · WPT runner secondary text in bright black — The WPT runner prints test kind, flag markers and summary section headings in bright black. · r1
contract/actions-line/content · The actions line · content — The line carries ids, owner ids, the verb table's words, closed cause words and booleans only, never a value, typed text or NodeId. · r1
contract/actions-line/nodes · The actions line · nodes — Every actionable node is listed in snapshot pre-order: focusable, of an interactive role, holding a current listener for an event other than mounted, or owned by another node, each with the id-taking instance verbs and the cause each would be refused with. · r1
contract/actions-line/stale · The actions line · stale — A node that resolves to no element reads stale. · r1
contract/coverage-check-files/read-bound · coverage check files · read bound — Each check path is read bounded at 1,048,576 + 1 bytes and parsed strictly. · r1
contract/coverage-check-files/refusal · coverage check files · refusal — A file that cannot be read, one past the bound, one that is no check and a check of another app are each refused malformed, status 1, the refusal holding nothing of the path or the file. · r1
contract/coverage-line/content · The coverage line · content — The line carries ids, owner ids, event names and two counts only, owner left out when absent. · r1
contract/dioxus-native-config/alpha-mode · The dioxus-native config · window compositing alpha mode — The window compositing alpha mode is configurable, for example for transparent windows; the renderer ignores unsupported modes. · r1
contract/escher-session-commands/actions-answer · escher-session commands · actions answer — actions answers one uncoloured line of JSON {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]} on stdout, status 0, the same on the session socket's call, over the mcp role and in a kept check; owner and cause left out when absent. · r1
contract/escher-session-commands/actions-arguments · escher-session commands · actions arguments — actions takes no argument; an argument is refused malformed. · r1
contract/escher-session-commands/coverage-answer · escher-session commands · coverage answer — coverage replays each named kept check of the app on a fresh session and answers one uncoloured line {"unexercised":[{"id","owner","event"}],"checks":n,"held":m} on stdout, status 0: the handlers no given check exercised, read at boot and after each step. · r1
contract/escher-session-commands/coverage-arguments · escher-session commands · coverage arguments — coverage's one shape is coverage <app> <check>..., an app the binary boots then one or more check paths, with no --session and no --record; any other shape is a usage ending, status 2. · r1
contract/escher-session-commands/graph-answer · escher-session commands · graph answer — graph answers the screen's whole UI graph as one uncoloured line of JSON {"nodes":[…],"states":[…]} on stdout, status 0, the same on the session socket's call and in a kept check; over the mcp role the tool result is the text block of that line. · r1
contract/escher-session-commands/graph-arguments · escher-session commands · graph arguments — graph takes no argument; an argument is refused malformed with nothing run. · r1
contract/escher-session-commands/help · escher-session commands · help — help answers the tool's self-description as one uncoloured line of JSON on stdout with no ESC byte, status 0, reaching no session. · r1
contract/escher-session-commands/screenshot · escher-session commands · screenshot [--id] — screenshot answers one uncoloured line {"width":…,"height":…,"png":"<base64>"} on stdout with no ESC byte, status 0, the image as base64 text inside the line. · r1
contract/escher-session-commands/snapshot · escher-session commands · snapshot [--part <part>] — snapshot answers one uncoloured line with no ESC byte, status 0: {"text":…} for a whole screen and {"text":…,"part":n,"parts":m} for a screen read in parts. · r1
contract/escher-session-exit-status/0 · escher-session exit status · 0 — Accepted. · r1
contract/escher-session-exit-status/1 · escher-session exit status · 1 — Refused. · r1
contract/escher-session-exit-status/2 · escher-session exit status · 2 — Usage. · r1
contract/escher-session-exit-status/3 · escher-session exit status · 3 — Session error, the recording's record-dir-not-private and recording-failed among them, with no exit of their own. · r1
contract/escher-session-mcp/err-ending · escher-session mcp role · Err ending — An Err ending's fixed message goes to stderr with status 3. · r1
contract/escher-session-mcp/help-result · escher-session mcp role · help tool result — help's tool result is the self-description's line. · r1
contract/escher-session-mcp/other-results · escher-session mcp role · other tool results — Any other tool result's text is the command's JSON line. · r1
contract/escher-session-mcp/screenshot-result · escher-session mcp role · screenshot tool result — A screenshot answers two content blocks: an image block (mimeType image/png, the PNG as base64) then a text block holding the shot's line without png. · r1
contract/escher-session-mcp/snapshot-result · escher-session mcp role · snapshot tool result — A snapshot's tool result text is the screen text, or its JSON line {"text","part","parts"} for a screen read in parts. · r1
contract/escher-session-mcp/stdout · escher-session mcp role · stdout — stdout holds newline-delimited JSON-RPC 2.0 protocol lines only, a reply per request that is not a notification, every reply one uncoloured line with no ESC byte; its end writes nothing there. · r1
contract/escher-session-session-error/remedy · escher-session session error object · remedy — A session error's object carries its fixed remedy. · r1
contract/escher-session-stderr/session-error · escher-session stderr · session error — A session error prints its fixed message on stderr. · r1
contract/escher-session-stderr/usage-error · escher-session stderr · usage error — A usage error prints one fixed line on stderr with stdout empty. · r1
contract/escher-session-tasks/cells · escher-session tasks · cells — cells is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum. · r1
contract/escher-session-tasks/counter · escher-session tasks · counter — counter is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum. · r1
contract/escher-session-tasks/crud · escher-session tasks · crud — crud is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum. · r1
contract/escher-session-tasks/flight-booker · escher-session tasks · flight-booker — flight-booker is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum. · r1
contract/escher-session-tasks/load · escher-session tasks · load — load is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum. · r1
contract/escher-session-tasks/timer · escher-session tasks · timer — timer is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum. · r1
contract/escher-session-tasks/unknown-task · escher-session tasks · unknown task — An unknown task is a usage ending. · r1
contract/js-window/device-pixel-ratio · The JS window object · devicePixelRatio — devicePixelRatio is the scale. · r1
contract/js-window/inner-width-height · The JS window object · innerWidth / innerHeight — innerWidth and innerHeight are the window size divided by the scale. · r1
contract/js-window/outer-width-height · The JS window object · outerWidth / outerHeight — outerWidth and outerHeight alias innerWidth and innerHeight. · r1
contract/mounteddata-during-render/document-state · MountedData calls during a render · document state — A MountedData call made during a render acts on the document as the previous render left it, so a node created in that same render is not yet in it. · r1
contract/mounteddata-during-render/set-focus-to · MountedData calls during a render · set_focus_to — set_focus_to returns false for a node created in the same render. · r1
contract/native-config/wasm-font-context · dioxus-native configuration · font context on WASM — On WASM a font context with bundled fonts must be provided; `build_single_font_ctx` serves where one font is bundled. · r1
contract/screenshot/select-list-row · The screenshot verb · a row of an open select list — A screenshot of a row of an open list crops to the row and leaves the list open. · r1
contract/select-accessibility/chosen-option · The snapshot and accessibility tree of a select · chosen option — The tree reads a select's chosen option as selected. · r1
contract/select-accessibility/highlighted-row · The snapshot and accessibility tree of a select · highlighted row — The highlighted row of an open list is in no snapshot and no tree. · r1
contract/select-accessibility/open-attribute · The snapshot and accessibility tree of a select · open attribute — An author's open attribute steers nothing on a select. · r1
contract/select-accessibility/open-select · The snapshot and accessibility tree of a select · open select — The tree reads an open select as ComboBox and expanded. · r1
contract/select-list-events/choose · A single select's list events · choose — Enter, Space, Tab in either direction or a click on a row chooses the row with one input event then one change event. · r1
contract/select-list-events/choose-chosen-option · A single select's list events · choose the chosen option — Choosing the option already chosen fires nothing. · r1
contract/select-list-events/choose-disabled-row · A single select's list events · choose a disabled row — Choosing a disabled row changes nothing and leaves the list open. · r1
contract/select-list-events/close · A single select's list events · close — Escape, a second click, a click outside, a wheel elsewhere and the page scrolling each close the list with no input and no change event. · r1
contract/select-list-events/close-on-select-change · A single select's list events · close on its select's change — Removing, disabling, hiding or making its select inert closes the list. · r1
contract/select-list-events/highlight-move · A single select's list events · highlight move — Keys and the pointer move the highlight over enabled rows and choose nothing. · r1
contract/select-list-events/open · A single select's list events · open — The list opens on its chosen option with no input and no change event. · r1
contract/shell-provider/decorations-toggle · The shell provider · decorations toggle — The shell provider exposes a decorations toggle for custom titlebars. · r1
contract/shell-provider/maximize · The shell provider · maximize — The shell provider exposes maximize for custom titlebars. · r1
contract/shell-provider/minimize · The shell provider · minimize — The shell provider exposes minimize for custom titlebars. · r1
contract/shell-provider/window-drag · The shell provider · window drag — The shell provider exposes window drag for custom titlebars. · r1
contract/snapshot/select-list-row-bounds · The snapshot · a row of an open select list — A row of an open list carries its own bounds in the snapshot. · r1
contract/ui-graph-line/content · The UI graph line · content — The line carries ids, owner ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, and booleans only, never a value or typed text. · r1
contract/wpt-runner-output/test-name-hyperlink · WPT runner output · test name — A test name is wrapped in an OSC 8 hyperlink to https://wpt.live/{name} only when stdout supports hyperlinks (a terminal, honouring FORCE_HYPERLINK). · r1
contract/wpt-runner-result-colours/crash · WPT runner result colours · CRASH — CRASH prints bright magenta. · r1
contract/wpt-runner-result-colours/fail · WPT runner result colours · FAIL — FAIL prints red. · r1
contract/wpt-runner-result-colours/fail-some-passing · WPT runner result colours · FAIL with some passing subtests — FAIL with some passing subtests prints yellow. · r1
contract/wpt-runner-result-colours/pass · WPT runner result colours · PASS — PASS prints green. · r1
contract/wpt-runner-result-colours/skip · WPT runner result colours · SKIP — SKIP prints bright black. · r1
contract/wpt-runner-result-colours/timeout · WPT runner result colours · TIMEOUT — TIMEOUT prints bright red. · r1
