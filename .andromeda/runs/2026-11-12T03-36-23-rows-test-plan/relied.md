fct-bloeybfr@1 · Snapshot-style assertions rely on the harness's DOM string being a stable one-node-per-line tree serialization.
fct-vu43sciq@1 · Harness input reaches the document through its real event-dispatch pipeline without a window, while recorded dispatch bypasses document-specific handling such as Dioxus forwarding.
fct-px3qbelc@1 · Harness documents always use HtmlProvider, and only a fetching net_provider is wrapped in the load counter that settle reads.
fct-4qfz5352@1 · Documents the harness builds read the harness's virtual clock, while a wrapped document keeps the clock its own config gave it.
fct-pmze6kz2@1 · A red job's later targets go unread on that run, since cargo stops at the first failing target.
fct-sys2coyr@1 · The buffered render's key-handler pins discriminate on Dioxus 0.7 because a task an event handler spawns while dirtying a scope is first polled inside `render_immediate`.
fct-cd63wvqs@1 · The harness, the session and the binary drive the select list headlessly because the list paints through `paint_scene` alone, with no shell or renderer code of its own.
fct-ome33psz@1 · A check that needs a load in flight uses the harness's `click`, which waits on nothing, while a driver `click` waits out `LOAD_WAIT`.
fct-qbzx34mx@2 · The stand load page's checks rely on its stand-in network answering the fixed URL with fixed bytes after a fixed delay, every other stand boot having no network provider.
fct-a24rqe7v@2 · The graph and handler-coverage checks rely on the shared graph drives driving each stand task's conditional writes once, so no stand write reads a flag.
fct-romvomei@1 · A key press on a focused control whose anonymous block was dropped does not panic, focus stays and the key still acts.
fct-4q3czvm2@2 · A task polled during render acts on the document as the previous render left it, so a node created in that same render cannot yet be focused.
fct-4qy7sqin@1 · The UI graph reads a declared write unconfirmed once its handler's event passed through its element and no run wrote the state, so a conditional write reads unconfirmed after runs that skipped it.
fct-n4zvuqt2@1 · The `:open` selector matches a details or dialog element carrying `open`, set at construction and kept on each write or removal of the attribute.
fct-j4q5jgse@1 · A failed load ends the harness and driver settle wait, so a failed stylesheet does not leave the document not settled on loads for good.
fct-vwiy3q7s@1 · The lint's overlap reading skips any entry inside a select's box, closed or open, so the stand's lint reads no overlap on select options and open-list rows.
