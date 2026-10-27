# Classified — arch · established-decisions.b2

## Answers
established-decisions#21.1 · no · the JavaScript runtime is not touched or described anew (Not needed) and no blitz-vibey-script unit is reached
established-decisions#22.1 · no · no dioxus-native unit is reached and the README restates no DioxusDocument setup (UA sheet, hot reload, history, eval, custom widgets); the stand test re-reads are name-only
established-decisions#23.1 · no · the browser app is not touched or described anew (Not needed) and no apps/browser unit is reached
established-decisions#24.1 · no · the WPT runner is not touched or described anew (Not needed); its 800x600 render target is the runner's own, not the stand's pinned viewport
established-decisions#25.1 · yes · the README's driver paragraph and Try it describe how the command line and MCP surface are run (Needs: front page, Try it): the owner-only socket state directory, unix-only with Windows Unsupported, the idle expiry, and the mcp role as a stdio child with no listener
established-decisions#26.1 · yes · the README names the replayable check (Needs: named 0.2.0 capabilities), so the recording rule binds with its one exception, the two committed Counter checks under tests/blitz-tests/checks/
