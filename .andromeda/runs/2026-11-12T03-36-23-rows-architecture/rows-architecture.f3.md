@row_add n=1 row=decision/root-scroll-to-viewport area=established-decisions rec=1,2
name: Root element scroll applies to the viewport
rule: The root element's scroll applies to the viewport, and the root element does not clip its own overflow.
@row_add n=2 row=decision/document-clock area=established-decisions rec=3,4,5
name: The document's clock
rule: Every engine time read goes through the document's configured clock, wall time when it is unset and a virtual clock headlessly, a sub-document carrying its parent's clock, with every timing constant unchanged.
why: ratified; an API addition whose default keeps every windowed and embedder-built document's behaviour
@row_add n=3 row=decision/inert-honoured area=established-decisions rec=6,7
name: inert honoured by presence
rule: An element carrying `inert`, or under an inert ancestor, is not focusable, skipped by the Tab walk and autofocus, takes no hit so a hit passes through it, and leaves the accessibility tree and the snapshot.
why: ratified
@row_add n=4 row=decision/controlled-select-writes-selected area=established-decisions rec=6,8
name: Controlled select writes selected from its input event
rule: A controlled app writes `selected` from its `input` event's value, or a choice reverts.
why: Dioxus writes a dynamic `selected` again on each re-render
@row_add n=5 row=decision/renderer-suspended-before-window-drop area=established-decisions rec=9,10
name: Renderer suspended before its window drops
rule: The renderer is suspended before its window drops.
why: a GPU surface outliving its window segfaults on Wayland
@row_add n=6 row=decision/windows-dropped-before-loop-exit area=established-decisions rec=9,10
name: Windows dropped before the event loop exits
rule: Windows are dropped before the event loop exits.
why: winit issue 4135
@row_add n=7 row=decision/dom-wrapper-identity area=established-decisions rec=11,12
name: DOM wrappers cached per node
rule: DOM wrappers are cached per node, preserving `===` identity.
@row_add n=8 row=decision/browser-history-sqlite area=established-decisions rec=13,14,15
name: SQLite for browser history
rule: The browser's history persists in SQLite through bundled rusqlite with rusqlite_migration, in `history.sqlite3` in the project data dir, in memory where that fails and always on Android and iOS.
@row_add n=9 row=decision/session-over-unix-socket area=established-decisions rec=16,17
name: Driver session over a std-only Unix socket
rule: A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port.
why: a deliberate transport choice
@row_add n=10 row=decision/workspace-dependencies area=conventions rec=18,19
name: Dependencies declared once in the workspace
rule: Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`.
except: The WPT runner's direct `mimalloc` dependency.
@row_add n=11 row=decision/inherited-package-metadata area=conventions rec=18,19
name: Package metadata inherited from the workspace
rule: Package metadata is inherited with `*.workspace = true`.
@row_add n=12 row=decision/spec-url-citations area=conventions rec=20,21
name: Spec URLs cited at implementations
rule: Spec URLs are cited above or next to implementations.
