@task_prose part=session
The session added two unit tests to the browser crate for its screenshot capture, in a private test-only module, with no change to any line of product code.
The tests check that the capture's scene render returns the viewport size, 800 by 600 at scale 1.0, and the physical size at a HiDPI scale of 2.0, read as 1280 by 720.
They catch the four surviving mutants of the scene render's size from the 0.3.0 Epoch 6 mutation audit, both under hand controls and under a cargo-mutants run fired on the founder's word.
No public, crate-visible or IPC surface, dependency, config, layout, element id or command-line contract was added, changed or retired.
One mutant of the capture's anyrender scene step, outside the founder's word and unchanged by the session, was read missed and carried as a residual for the next version.
@task_file n=1 path=apps/browser/src/capture.rs
what: the browser's screenshot capture, which renders a document's scene at its viewport size or its physical size under the display scale; edited only to declare its cfg(test) module
@task_file n=2 path=apps/browser/src/capture_tests.rs
what: the unit tests of the screenshot capture's scene render size, at the viewport size and at a HiDPI physical size; a new file
@task_item n=1 part=touches
The headless stand pinned viewport decision: the capture's render at the viewport size is now held by a unit test at 800 by 600 at scale 1.0; the decision's rule is followed, not changed.
@task_item n=2 part=touches
The escher-session screenshot answer contract: the size of the captured image comes from the capture's scene render, whose viewport and HiDPI physical sizes are now tested; the contract item does not move.
@task_item n=3 part=touches
The escher-session MCP server screenshot result contract: the result's image is produced through the same capture render whose size is now held by tests; no cell of the item changes.
@task_item n=4 part=touches
The fonts-unavailable and off-screen refusals of escher-session, where they rest on the capture path: the capture render is unchanged in behaviour, so a picker should confirm no refusal moved.
@task_item n=5 part=touches
Any row that states how the display scale maps the viewport to physical pixels in a capture or screenshot: the session now tests the physical size returned at a scale of 2.0, read as 1280 by 720.
@task_item n=1 part=untouched
The stand stable element ids, every one: no id, attribute, role or state of the stand page was added, renamed or freed.
@task_item n=2 part=untouched
The ShellProvider window chrome controls: no window drag, maximize, minimize, close or decoration behaviour changed.
@task_item n=3 part=untouched
The stand page decisions on keyboard regions, select options, scroll into view, clipping boxes, accent colour, lints at boot and the document skeleton: no layout or stand behaviour changed.
@task_item n=4 part=untouched
The agent-run, cold-agent, multi_doc and paint_bench command-line contracts, and the escher-session exit codes, recording, coverage, help, graph, lint, layout and snapshot answers: no command line, exit code or answer shape moved.
@task_item n=5 part=untouched
The served tasks of escher-session and the load page's network provider: no task, page or provider was added or retired.
@task_item n=6 part=untouched
The absence of arbitrary index.html templates, and the mobile-native and web-spa surfaces: nothing of them was built or measured.
