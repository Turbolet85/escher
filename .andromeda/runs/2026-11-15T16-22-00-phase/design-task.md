@task_prose part=chunk
Proves the browser app's page-URL hand-off with the browser's own tests: the document config every browser document loads with must keep its network provider and its navigation provider, the one that hands the page URL to the file: navigation gate.
The new tests fail when that config is replaced by a default, when its network provider is dropped, or when its navigation provider is dropped, which kills the three mutation survivors the Epoch 4 code audit measured.
Through the config's own navigation provider, the tests re-assert that a remote page's navigation to a file: URL loads nothing and a file: page's navigation to one does.
Writes a reason beside the two browser expect_used allows that have none: the favicon test module's and the tab web view's. The browser's behaviour does not change.
@task_file n=1 path=apps/browser/src/document_loader.rs mark=edits
what: the browser's document loader, which builds the document config each browser document loads with (its network provider and its navigation provider, which carries the page URL) — edits: tests of make_doc_config
@task_file n=2 path=apps/browser/src/favicon.rs mark=edits
what: the browser's favicon reader, with its test module — edits: the test module's expect_used allow, which gets its written reason
@task_file n=3 path=apps/browser/src/tab.rs mark=edits
what: the browser tab and its web view component — edits: the TabWebView expect_used allow, which gets its written reason
@task_item n=1 part=needs
The document config hand-off: every browser document loads with a config that carries a network provider and a navigation provider built for that document from its history and page URL. New tests prove that neither provider can be dropped and that the config cannot be swapped for the default config.
@task_item n=2 part=needs
The file: navigation gate, re-asserted and not changed: through the config's navigation provider, a remote page's navigation to a file: URL loads nothing, and a file: page's navigation to a file: URL loads.
@task_item n=3 part=needs
The page URL a document's navigation provider reports is the value the navigation gate reads to decide whether a document may load. The tests read it as the gate reads it.
@task_item n=4 part=needs
Indirect, through the shell provider: building the document config reads the shell provider from the Dioxus context, so a test of it runs inside a Dioxus runtime (a virtual DOM) that provides a shell provider. P3 settles the harness shape.
@task_item n=5 part=needs
The favicon reader's test module: its expect_used allow gets a written reason (a comment line, or the allow's reason). Reading a favicon does not change.
@task_item n=6 part=needs
The browser tab's web view component: its expect_used allow gets a written reason. The tab's other allows, and the history's, already stand beside a comment and stay as they are.
@task_item n=7 part=needs
No browser behaviour changes: the hand-off and the gate stay as the remote-pages-local-files chunk built them, and the existing browser history tests still hold them. This chunk adds only tests and justification text.
@task_item n=8 part=needs
How the change is measured: a cargo-mutants run on the document loader, on the dev host once it is free, reads whether the three survivors are killed. The mutation tier is not a CI leg.
@task_item n=9 part=needs
The new tests run in the workspace test leg, so CI gates them. P3 confirms this against the CI leg script and the workspace members.
@task_item n=10 part=needs
Boundary: the browser app only. The chunk adds no engine crate change, no driver change, and no carrier, verb, field, log site or span.
@task_item n=1 part=not_needed
Colour rows (engine colours, colour-scheme recascade, select list colours, base colour, colour tokens): the chunk paints nothing and sets no colour.
@task_item n=2 part=not_needed
Typography rows (font faces, font context, system-font test builds, root-relative units, text locale, screenshot text): the new tests render and lay out no text.
@task_item n=3 part=not_needed
Spacing and depth-strategy rows (margins, table spacing hints, z-index paint levels, stacking contexts, hit-testing at paint clips): the chunk changes no layout, paint or hit-testing.
@task_item n=4 part=not_needed
The two motion areas (virtual clock, driver advance, scroll behaviour, overlay scrollbar fades, select list motion, reduced motion): the chunk changes no animation or scrolling and touches no driver verb.
@task_item n=5 part=not_needed
escher-session commands, exit statuses or codes, stdout, stderr, the mcp role, its tasks, and the actions, coverage and UI graph lines: the chunk adds no verb, command, field or output line.
@task_item n=6 part=not_needed
Select element rows (the list overlay, its open gestures, its events, its accessibility tree, its snapshot and screenshot rows): the browser change touches no select control.
@task_item n=7 part=not_needed
WPT runner output and result colour rows: the chunk touches nothing in the WPT runner.
@task_item n=8 part=not_needed
JS window contracts (devicePixelRatio, inner and outer sizes), the logical viewport and the web-spa WASM canvas surface: the chunk changes no viewport and builds nothing for wasm32.
@task_item n=9 part=not_needed
Cursor icon rows (the cursor-icon crate, the cursor resolution order): the chunk changes no cursor.
