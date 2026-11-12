@row_none n=1 rec=1 type=4
@row_none n=2 rec=2 type=5
@row_none n=3 rec=3 type=5
@row_none n=4 rec=4 type=5
@row_none n=5 rec=5 type=5
@row_none n=6 rec=6 type=5
@row_none n=7 rec=7 type=4
relied: Every document gets the blitz default user-agent stylesheet once, through the default document config.
@row_none n=8 rec=8 type=4
relied: A single-choice select lays out as a box holding its chosen option's label, from engine defaults with no design token.
@row_none n=9 rec=9 type=4
@row_none n=10 rec=10 type=4
@row_none n=11 rec=11 type=4
@row_none n=12 rec=12 type=4
@row_none n=13 rec=13 type=4
relied: A single select's list opens on a click, Enter, or a Space with no typed text standing.
@row_add n=1 row=decision/select-list-engine-overlay area=surface rec=14
name: Select list as an engine overlay
rule: On desktop-native a single select's open list is an engine overlay that reaches the screen through the scene paint alone, not a second OS window and not rows as boxes.
@row_add n=2 row=contract/select-list-events/open area=surface rec=15
boundary: A single select's list events
item: open
promise: The list opens on its chosen option with no input and no change event.
@row_add n=3 row=contract/select-list-events/highlight-move area=surface rec=15
boundary: A single select's list events
item: highlight move
promise: Keys and the pointer move the highlight over enabled rows and choose nothing.
@row_add n=4 row=contract/select-list-events/choose area=surface rec=16
boundary: A single select's list events
item: choose
promise: Enter, Space, Tab in either direction or a click on a row chooses the row with one input event then one change event.
@row_add n=5 row=contract/select-list-events/choose-disabled-row area=surface rec=16
boundary: A single select's list events
item: choose a disabled row
promise: Choosing a disabled row changes nothing and leaves the list open.
@row_add n=6 row=contract/select-list-events/choose-chosen-option area=surface rec=16
boundary: A single select's list events
item: choose the chosen option
promise: Choosing the option already chosen fires nothing.
@row_add n=7 row=contract/select-list-events/close area=surface rec=17
boundary: A single select's list events
item: close
promise: Escape, a second click, a click outside, a wheel elsewhere and the page scrolling each close the list with no input and no change event.
@row_add n=8 row=contract/select-list-events/close-on-select-change area=surface rec=17
boundary: A single select's list events
item: close on its select's change
promise: Removing, disabling, hiding or making its select inert closes the list.
@row_none n=14 rec=18 type=4
relied: A disabled, inert or hidden select, a select with no row to show and a multiple select open no list.
@row_none n=15 rec=19 type=4
relied: A focused select whose container lost its box keeps focus and takes a key with no panic.
@row_none n=16 rec=20 type=4
@row_add n=9 row=contract/select-accessibility/highlighted-row area=surface rec=21
boundary: The snapshot and accessibility tree of a select
item: highlighted row
promise: The highlighted row of an open list is in no snapshot and no tree.
@row_add n=10 row=contract/select-accessibility/open-select area=surface rec=21
boundary: The snapshot and accessibility tree of a select
item: open select
promise: The tree reads an open select as ComboBox and expanded.
@row_add n=11 row=contract/select-accessibility/chosen-option area=surface rec=21
boundary: The snapshot and accessibility tree of a select
item: chosen option
promise: The tree reads a select's chosen option as selected.
@row_add n=12 row=contract/select-accessibility/open-attribute area=surface rec=21
boundary: The snapshot and accessibility tree of a select
item: open attribute
promise: An author's open attribute steers nothing on a select.
@row_add n=13 row=contract/snapshot/select-list-row-bounds area=surface rec=22
boundary: The snapshot
item: a row of an open select list
promise: A row of an open list carries its own bounds in the snapshot.
@row_add n=14 row=contract/screenshot/select-list-row area=surface rec=22
boundary: The screenshot verb
item: a row of an open select list
promise: A screenshot of a row of an open list crops to the row and leaves the list open.
@row_none n=17 rec=23 type=4
@row_none n=18 rec=24 type=4
relied: The logical viewport size is the physical window size divided by hidpi scale times zoom.
@row_add n=15 row=contract/js-window/inner-width-height area=surface rec=25
boundary: The JS window object
item: innerWidth / innerHeight
promise: innerWidth and innerHeight are the window size divided by the scale.
@row_add n=16 row=contract/js-window/outer-width-height area=surface rec=25
boundary: The JS window object
item: outerWidth / outerHeight
promise: outerWidth and outerHeight alias innerWidth and innerHeight.
@row_add n=17 row=contract/js-window/device-pixel-ratio area=surface rec=25
boundary: The JS window object
item: devicePixelRatio
promise: devicePixelRatio is the scale.
@row_none n=19 rec=26 type=4
@row_add n=18 row=contract/shell-provider/minimize area=surface rec=27
boundary: The shell provider
item: minimize
promise: The shell provider exposes minimize for custom titlebars.
@row_add n=19 row=contract/shell-provider/maximize area=surface rec=27
boundary: The shell provider
item: maximize
promise: The shell provider exposes maximize for custom titlebars.
@row_add n=20 row=contract/shell-provider/decorations-toggle area=surface rec=27
boundary: The shell provider
item: decorations toggle
promise: The shell provider exposes a decorations toggle for custom titlebars.
@row_add n=21 row=contract/shell-provider/window-drag area=surface rec=27
boundary: The shell provider
item: window drag
promise: The shell provider exposes window drag for custom titlebars.
@row_none n=20 rec=28 type=5
@row_none n=21 rec=29 type=4
@row_none n=22 rec=30 type=4
relied: The shell sets the window title from the document's title node when it was set before the window existed.
@row_none n=23 rec=31 type=4
@row_none n=24 rec=32 type=4
@row_none n=25 rec=33 type=5
@row_add n=22 row=contract/dioxus-native-config/alpha-mode area=surface rec=34
boundary: The dioxus-native config
item: window compositing alpha mode
promise: The window compositing alpha mode is configurable, for example for transparent windows; the renderer ignores unsupported modes.
@row_none n=26 rec=35 type=4
@row_add n=23 row=decision/buffered-render-replay area=surface rec=36
name: Render buffered, then replayed under one borrow
rule: On every Dioxus document, windowed and headless alike, a render is recorded into a mutation buffer with the document unborrowed, then replayed under one borrow.
why: so code the vdom runs during a render, a component body or a polled task can call MountedData's methods with no RefCell already borrowed panic
@row_add n=24 row=contract/mounteddata-during-render/document-state area=surface rec=36
boundary: MountedData calls during a render
item: document state
promise: A MountedData call made during a render acts on the document as the previous render left it, so a node created in that same render is not yet in it.
@row_add n=25 row=contract/mounteddata-during-render/set-focus-to area=surface rec=36
boundary: MountedData calls during a render
item: set_focus_to
promise: set_focus_to returns false for a node created in the same render.
@row_none n=27 rec=37 type=6
@row_none n=28 rec=38 type=4
@row_none n=29 rec=39 type=4
@row_none n=30 rec=40 type=4
@row_none n=31 rec=41 type=4
@row_none n=32 rec=42 type=4
@row_add n=26 row=decision/mobile-targets-built-not-tested area=surface rec=43
name: Mobile targets built, not tested
rule: CI builds the iOS and Android targets but runs no tests on them.
@row_add n=27 row=contract/wpt-runner-result-colours/pass area=surface rec=44
boundary: WPT runner result colours
item: PASS
promise: PASS prints green.
@row_add n=28 row=contract/wpt-runner-result-colours/fail-some-passing area=surface rec=44
boundary: WPT runner result colours
item: FAIL with some passing subtests
promise: FAIL with some passing subtests prints yellow.
@row_add n=29 row=contract/wpt-runner-result-colours/fail area=surface rec=44
boundary: WPT runner result colours
item: FAIL
promise: FAIL prints red.
@row_add n=30 row=contract/wpt-runner-result-colours/timeout area=surface rec=44
boundary: WPT runner result colours
item: TIMEOUT
promise: TIMEOUT prints bright red.
@row_add n=31 row=contract/wpt-runner-result-colours/skip area=surface rec=44
boundary: WPT runner result colours
item: SKIP
promise: SKIP prints bright black.
@row_add n=32 row=contract/wpt-runner-result-colours/crash area=surface rec=44
boundary: WPT runner result colours
item: CRASH
promise: CRASH prints bright magenta.
@row_add n=33 row=decision/wpt-runner-secondary-text-bright-black area=surface rec=45
name: WPT runner secondary text in bright black
rule: The WPT runner prints test kind, flag markers and summary section headings in bright black.
@row_add n=34 row=contract/wpt-runner-output/test-name-hyperlink area=surface rec=46
boundary: WPT runner output
item: test name
promise: A test name is wrapped in an OSC 8 hyperlink to https://wpt.live/{name} only when stdout supports hyperlinks (a terminal, honouring FORCE_HYPERLINK).
@row_none n=33 rec=47 type=4
@row_none n=34 rec=48 type=4
@row_add n=35 row=decision/escher-session-agent-read-surface area=surface rec=49
name: escher-session is read by an agent, not a person
rule: An escher-session command prints one line of JSON on stdout, the answer, the refusal or a session error's object keyed by the driver schema's own words, and nothing else there.
@row_add n=36 row=decision/escher-session-no-colour area=surface rec=49
name: No colour and no terminal test on escher-session
rule: escher-session uses no colour and no colour dependency, writes no ESC byte on any answer line or MCP reply, and never asks whether a stream is a terminal, so its output is the same piped or not.
guard: the seven_guis checks' shared reader and the shared Speaker assert no ESC byte on every line
@row_add n=37 row=contract/escher-session-stderr/usage-error area=surface rec=49
boundary: escher-session stderr
item: usage error
promise: A usage error prints one fixed line on stderr with stdout empty.
@row_add n=38 row=contract/escher-session-stderr/session-error area=surface rec=49
boundary: escher-session stderr
item: session error
promise: A session error prints its fixed message on stderr.
@row_add n=39 row=contract/escher-session-exit-status/0 area=surface rec=49,54
boundary: escher-session exit status
item: 0
promise: Accepted.
@row_add n=40 row=contract/escher-session-exit-status/1 area=surface rec=49,54
boundary: escher-session exit status
item: 1
promise: Refused.
@row_add n=41 row=contract/escher-session-exit-status/2 area=surface rec=49,54
boundary: escher-session exit status
item: 2
promise: Usage.
@row_add n=42 row=contract/escher-session-exit-status/3 area=surface rec=49,54
boundary: escher-session exit status
item: 3
promise: Session error, the recording's record-dir-not-private and recording-failed among them, with no exit of their own.
@row_add n=43 row=decision/exit-statuses-from-schema-table area=surface rec=49
name: Exit statuses read from the schema's table
rule: The exit statuses are read from the driver schema's EXIT_STATUSES table, whose meanings help serves.
@row_add n=44 row=contract/escher-session-commands/help area=surface rec=49
boundary: escher-session commands
item: help
promise: help answers the tool's self-description as one uncoloured line of JSON on stdout with no ESC byte, status 0, reaching no session.
@row_add n=45 row=contract/escher-session-session-error/remedy area=surface rec=49
boundary: escher-session session error object
item: remedy
promise: A session error's object carries its fixed remedy.
@row_add n=46 row=contract/escher-session-commands/screenshot area=surface rec=49
boundary: escher-session commands
item: screenshot [--id]
promise: screenshot answers one uncoloured line {"width":…,"height":…,"png":"<base64>"} on stdout with no ESC byte, status 0, the image as base64 text inside the line.
@row_add n=47 row=contract/escher-session-commands/snapshot area=surface rec=49
boundary: escher-session commands
item: snapshot [--part <part>]
promise: snapshot answers one uncoloured line with no ESC byte, status 0: {"text":…} for a whole screen and {"text":…,"part":n,"parts":m} for a screen read in parts.
@row_add n=48 row=contract/escher-session-mcp/stdout area=surface rec=49
boundary: escher-session mcp role
item: stdout
promise: stdout holds newline-delimited JSON-RPC 2.0 protocol lines only, a reply per request that is not a notification, every reply one uncoloured line with no ESC byte; its end writes nothing there.
@row_add n=49 row=contract/escher-session-mcp/snapshot-result area=surface rec=49
boundary: escher-session mcp role
item: snapshot tool result
promise: A snapshot's tool result text is the screen text, or its JSON line {"text","part","parts"} for a screen read in parts.
@row_add n=50 row=contract/escher-session-mcp/help-result area=surface rec=49
boundary: escher-session mcp role
item: help tool result
promise: help's tool result is the self-description's line.
@row_add n=51 row=contract/escher-session-mcp/screenshot-result area=surface rec=49
boundary: escher-session mcp role
item: screenshot tool result
promise: A screenshot answers two content blocks: an image block (mimeType image/png, the PNG as base64) then a text block holding the shot's line without png.
@row_add n=52 row=contract/escher-session-mcp/other-results area=surface rec=49
boundary: escher-session mcp role
item: other tool results
promise: Any other tool result's text is the command's JSON line.
@row_add n=53 row=contract/escher-session-mcp/err-ending area=surface rec=49
boundary: escher-session mcp role
item: Err ending
promise: An Err ending's fixed message goes to stderr with status 3.
@row_add n=54 row=contract/escher-session-tasks/counter area=surface rec=50
boundary: escher-session tasks
item: counter
promise: counter is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_add n=55 row=contract/escher-session-tasks/flight-booker area=surface rec=50
boundary: escher-session tasks
item: flight-booker
promise: flight-booker is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_add n=56 row=contract/escher-session-tasks/timer area=surface rec=50
boundary: escher-session tasks
item: timer
promise: timer is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_add n=57 row=contract/escher-session-tasks/crud area=surface rec=50
boundary: escher-session tasks
item: crud
promise: crud is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_add n=58 row=contract/escher-session-tasks/cells area=surface rec=50
boundary: escher-session tasks
item: cells
promise: cells is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_add n=59 row=contract/escher-session-tasks/load area=surface rec=50
boundary: escher-session tasks
item: load
promise: load is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_add n=60 row=contract/escher-session-tasks/unknown-task area=surface rec=50
boundary: escher-session tasks
item: unknown task
promise: An unknown task is a usage ending.
@row_add n=61 row=contract/escher-session-commands/graph-arguments area=surface rec=51
boundary: escher-session commands
item: graph arguments
promise: graph takes no argument; an argument is refused malformed with nothing run.
@row_add n=62 row=contract/escher-session-commands/graph-answer area=surface rec=51
boundary: escher-session commands
item: graph answer
promise: graph answers the screen's whole UI graph as one uncoloured line of JSON {"nodes":[…],"states":[…]} on stdout, status 0, the same on the session socket's call and in a kept check; over the mcp role the tool result is the text block of that line.
@row_add n=63 row=contract/ui-graph-line/content area=surface rec=51
boundary: The UI graph line
item: content
promise: The line carries ids, owner ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, and booleans only, never a value or typed text.
@row_add n=64 row=contract/escher-session-commands/actions-arguments area=surface rec=52
boundary: escher-session commands
item: actions arguments
promise: actions takes no argument; an argument is refused malformed.
@row_add n=65 row=contract/escher-session-commands/actions-answer area=surface rec=52
boundary: escher-session commands
item: actions answer
promise: actions answers one uncoloured line of JSON {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]} on stdout, status 0, the same on the session socket's call, over the mcp role and in a kept check; owner and cause left out when absent.
@row_add n=66 row=contract/actions-line/nodes area=surface rec=52
boundary: The actions line
item: nodes
promise: Every actionable node is listed in snapshot pre-order: focusable, of an interactive role, holding a current listener for an event other than mounted, or owned by another node, each with the id-taking instance verbs and the cause each would be refused with.
@row_add n=67 row=contract/actions-line/content area=surface rec=52
boundary: The actions line
item: content
promise: The line carries ids, owner ids, the verb table's words, closed cause words and booleans only, never a value, typed text or NodeId.
@row_add n=68 row=decision/actions-share-the-verbs-checks area=surface rec=53
name: actions decided by the verbs' own checks
rule: The actions answer reports whether each call would be accepted, not what it would do, deciding a node's verbs by the same validation and per-verb target check every id-taking verb meets before it acts.
why: so a listed cause is the cause the call returns
@row_add n=69 row=contract/actions-line/stale area=surface rec=53
boundary: The actions line
item: stale
promise: A node that resolves to no element reads stale.
@row_add n=70 row=contract/escher-session-commands/coverage-answer area=surface rec=54
boundary: escher-session commands
item: coverage answer
promise: coverage replays each named kept check of the app on a fresh session and answers one uncoloured line {"unexercised":[{"id","owner","event"}],"checks":n,"held":m} on stdout, status 0: the handlers no given check exercised, read at boot and after each step.
@row_add n=71 row=contract/coverage-line/content area=surface rec=54
boundary: The coverage line
item: content
promise: The line carries ids, owner ids, event names and two counts only, owner left out when absent.
@row_add n=72 row=contract/escher-session-commands/coverage-arguments area=surface rec=55
boundary: escher-session commands
item: coverage arguments
promise: coverage's one shape is coverage <app> <check>..., an app the binary boots then one or more check paths, with no --session and no --record; any other shape is a usage ending, status 2.
@row_add n=73 row=contract/coverage-check-files/read-bound area=surface rec=55
boundary: coverage check files
item: read bound
promise: Each check path is read bounded at 1,048,576 + 1 bytes and parsed strictly.
@row_add n=74 row=contract/coverage-check-files/refusal area=surface rec=55
boundary: coverage check files
item: refusal
promise: A file that cannot be read, one past the bound, one that is no check and a check of another app are each refused malformed, status 1, the refusal holding nothing of the path or the file.
