@version key=escher-0.4.0 source=intent id_scheme=v040-NN
# escher 0.4.0 — requirements

*Source: `escher-0.4.0-incubator/intent-0.4.0.md` §Findings and §Principles.*
- Ids are version-scoped (`v040-NN`), continuing 0.3.0's `v030-NN` scheme.
- Every capability is verified headless, on the stand (modified for the canvas) or on a minimal fixture beside it
  (intent §Principles).
- Residuals taken in: the 22 rows of the store's `open_residuals` and the 9 lines 0.3.0's requirements carried. All
  22 open rows are absorbed below, each by the intent's finding 4. Six of the nine carried lines are absorbed on the
  founder's F1 «Взять все шесть (Recommended)» (2026-11-19, relayed by the overseer). The three the intent names out
  are re-carried (§Carried residuals). The 0.3.0 ledger holds no `deferred` entry.

- v040-01 · Build directory held — the build directory's growth cause is measured and mended, and a working day's growth after the mend is stated as a number (per intent §1)
- v040-02 · Upstream sync — each epoch opens with a two-parent merge of upstream's new work, our checks and CI proving our logic survived, each commit left out named with its reason (per intent §2; residuals res-7qgqygtj, res-xpkmxnvl, res-xchmbfjr, res-vuvpa25q, res-q4s3p5ga, res-kk4tzmi5, res-derughab, res-ipcah32k)
- v040-03 · The version on the GitHub page — the README states 0.3.0 closed, in the form the founder chooses, and says what 0.4.0 builds; the records' category-file citation backlog is read through or kept by the founder's word (per intent §3; residual res-xe7pyput, on the founder's F1)
- v040-04 · Select list and selection debts — the select list's highlighted-row contrast, a larger authored font, the highlight's reading, a single select's `size` and Cells stating its selected cell, each fixed or kept by the founder's word (per intent §4; residuals res-tym5wois, res-2iu7qn2e, res-caouqg3t, res-ysz67imk; res-wvatzhpe on the founder's F1)
- v040-05 · Driver debts — `press` with Ctrl, the UI graph read in parts by its ruled form, and handler coverage under a stopped bubbling dispatch, each fixed or kept by the founder's word (per intent §4; residuals res-hfvjj77l, res-7cc5bi4k, res-4dr3wt62)
- v040-06 · Engine debts — a focused control that no longer shows, a task polled during a render, a page script's unseeded randomness and the growing error-boundary registry, each fixed or kept by the founder's word (per intent §4; residuals res-6k56erh6, res-433ttjb2, res-jqbadec2, res-k2zggoqx)
- v040-07 · Dependency and CI debts — the advisory ignores (the browser's database crate, paste, memmap2, the zbus family, zip) and the cancelled-runner watch, each fixed, retired or kept by the founder's word; one that waits on another project's release is stated as such, and one that does not close is named (per intent §4; residuals res-j4x7iiqc, res-q2tllspz; res-kdhnfj5p, res-uqt2yd2y, res-p73cueyv, res-h7acz56c on the founder's F1)
- v040-08 · Codebase health — the Epoch 6 code audit's oversize files, functions over the complexity ceiling, rising stand-test setup duplication and the browser's surviving mutants each brought under its bar or kept by the founder's word (per intent §1, §4)
- v040-09 · A canvas painted headless — what an app paints into a canvas from Rust appears in a headless screenshot, its text from the bundled fonts, its cross-host byte equality measured and required or given a tolerance by the founder's word; Circle Drawer on a real canvas (per intent §5, §10)
- v040-10 · The canvas read as data — a driver verb returns a canvas's draw commands as data, deterministic, hardware-independent and diffable, grouped by virtual node, at a summary and a full level, with lint-like checks inside the canvas; what it may carry to a provider is the founder's fork (per intent §6)
- v040-11 · Virtual nodes inside a canvas — an app declares nodes inside a canvas with an author key, role, name and region, and the driver treats them as elements across the snapshot, diff, click, refusals, UI graph, actions, coverage, layout and exploration; Circle Drawer's circles are virtual nodes (per intent §7, §10)
- v040-12 · A `drag` verb — `drag` moves from an element or a virtual node to another or by an offset through interpolated moves, settled and refused like every action, recorded and replayed, listed where it applies; it drags the select list's thumb (per intent §8; residual res-ammkrycp)
- v040-13 · Pan and zoom under a transform — a target inside a panned and zoomed container is read, hit and acted on where it shows; zoom is its own verb or a stated limit by the founder's word; the surface's speed is measured at a stated node count and held as a performance budget, a check reading red past it (per intent §9; the founder's F6 «Сделать бюджетом»)
- v040-14 · The node-graph stand app — a new stand app of a few nodes with ports and curved wires in a canvas, its nodes dragged, a wire pulled port to port and the surface panned and zoomed, each with its stand checks (per intent §10)
- v040-15 · Cold-agent run — 0.4.0 ends with one cold-agent run on the stand, the two canvas samples among its apps, the canvas read as data, a virtual node acted on and `drag` used at least once, judged at 5 % wrong calls; its budget and count the founder's word (per intent §11; covers v040-09…v040-14 end to end)

## Carried residuals (NOT 0.4.0 capabilities — deliberately unnumbered, no matrix entry)
Each line below was dispositioned at this route's Phase A. All three are named in intent §Out of this version. The six
other lines 0.3.0 carried are absorbed above (v040-03, v040-04, v040-07) on the founder's F1. Each stays visible here
and outside the matrix until a version asks for it.
- **Opt-in OTel export** (`res-fzitzzw2`). Named out by the intent.
- **MCP revision `2026-07-28`** (`res-wpbdioya`). Named out by the intent.
- **The stand's platform accessibility adapter and its windowed AT witness** (`res-llkgqygv`). Named out by the intent.

@epoch number=1 as=e1
Foundation

@chunk in=@e1 as=c1
Build directory held — the growth's cause measured, then mended; a working day's growth after the mend stated as a number (v040-01)

@epoch number=2 as=e2
Debts of 0.1.0–0.3.0

@chunk in=@e2 as=c2
Upstream sync ahead of the debts — two-parent merge; left-out commits named; CDP listener's security amendment first; lock, residuals re-read; CI, a11y leg, audit green (v040-02)

@chunk in=@e2 as=c3
README and records — 0.3.0 closed in the founder's chosen form, what 0.4.0 builds; the category-file citation backlog read through or kept (v040-03)

@chunk in=@e2 as=c4
Stand-test setup shared — the stand checks' repeated setup in one place, duplication back under its earlier reading, every check unchanged (v040-08)

@chunk in=@e2 as=c5
Codebase health — the code audit's oversize files and functions over the ceiling, each under its bar or kept by the founder's word (v040-08)

@chunk in=@e2 as=c6
Browser's surviving mutants — the four at the capture module caught, or kept by the founder's word (v040-08)

@chunk in=@e2 as=c7
Select list and selection debts — highlighted-row contrast shared with a multiple's chosen row, larger font, highlight's reading, single `size`, Cells' selected cell; founder's forks (v040-04)

@chunk in=@e2 as=c8
Driver debts — `press` with Ctrl, `graph --part N` as ruled, coverage under a stopped bubbling dispatch; each the founder's fork (v040-05)

@chunk in=@e2 as=c9
Engine debts — a hidden focused control, a render-polled task, page-script `Math.random`, the growing boundary registry; each the founder's fork (v040-06)

@chunk in=@e2 as=c10
Dependency and CI debts — advisories (sqlite, paste, memmap2, zbus, zip) closed or stated as waits; the cancelled-runner watch retired or kept (v040-07)

@epoch number=3 as=e3
The canvas painted

@chunk in=@e3 as=c11
Upstream sync ahead of the canvas — two-parent merge; left-out commits named with reasons; CI, a11y leg and audit green (v040-02)

@chunk in=@e3 as=c12
Canvas in a headless shot — what an app paints from Rust appears in a harness shot, measured on the stand first (v040-09)

@chunk in=@e3 as=c13
Canvas placement and text — the widget placed at its content box (upstream issue 1578), canvas text from the bundled fonts (v040-09)

@chunk in=@e3 as=c14
Canvas shot across hosts — byte equality on Linux, macOS, Windows measured; required (amending the no-shot cross-OS rule) or toleranced, by the founder's word (v040-09)

@chunk in=@e3 as=c15
Circle Drawer on a canvas — the first sample, circles painted from Rust; skeleton, keys, boot lint clean; checks in both layout modes, cross-OS digests (v040-09)

@epoch number=4 as=e4
Inside the canvas

@chunk in=@e4 as=c16
Upstream sync ahead of inside the canvas — two-parent merge; left-out commits named with reasons; CI, a11y leg and audit green (v040-02)

@chunk in=@e4 as=c17
Canvas draw commands — a driver verb answering a canvas's commands, summary and full levels, deterministic, diffable; size measured; every carrier's crossing the founder's fork (v040-10)

@chunk in=@e4 as=c18
Canvas lint — lint-like checks inside a canvas, a label overflowing its region among them (v040-10)

@chunk in=@e4 as=c19
Virtual nodes declared — author key, role, non-empty name, region inside a canvas, reserved id form; declaration's form and names' crossing the founder's fork (v040-11)

@chunk in=@e4 as=c20
Virtual nodes as elements — in the accessibility tree and from it the snapshot and diff; taking `click`; refused covered, off-screen, disabled or stale (v040-11)

@chunk in=@e4 as=c21
Virtual nodes in every read — the UI graph, actions, handler coverage, layout explained and exploration (v040-11)

@chunk in=@e4 as=c22
Circle Drawer's circles as virtual nodes — each circle a node, its draw commands grouped by it, with stand checks (v040-10, v040-11)

@chunk in=@e4 as=c23
Canvas keyboard path — Circle Drawer's circles reached and acted on by keyboard; engine, snapshot and tree focus agree; Tab order in the shared table (v040-11)

@epoch number=5 as=e5
Drag, pan and zoom

@chunk in=@e5 as=c24
Upstream sync ahead of drag, pan and zoom — two-parent merge; left-out commits named with reasons; CI, a11y leg and audit green (v040-02)

@chunk in=@e5 as=c25
The drag verb — element, virtual node or offset; interpolated moves; settled and refused like every action; recorded, replayed, in actions; a widening fork (v040-12)

@chunk in=@e5 as=c26
Select list thumb dragged — the list's thumb moved by `drag`, a long list scrolled by it (v040-12)

@chunk in=@e5 as=c27
Pan and zoom — transformed targets read, hit, acted on where shown; zoom a widening-fork verb or stated limit; speed budget at stated node count (v040-13)

@chunk in=@e5 as=c28
Node-graph stand app — nodes with ports, canvas wires; dragged, wired, panned, zoomed, with a keyboard path or stated limit; keyed, lint-clean, both layout modes (v040-14)

@epoch number=6 as=e6
Polish & ship

@chunk in=@e6 as=c29
Upstream sync ahead of polish and ship — two-parent merge; left-out commits named with reasons; CI, a11y leg and audit green (v040-02)

@chunk in=@e6 as=c30
Stand requirement sweep — 0.4.0's capabilities proven inside stand-repeat; floors, masks, stand layout and the log scrub at trace held over every new verb (intent §Principles)

@chunk in=@e6 as=c31
A11y re-read — SC 2.1.1, 2.4.3, 1.4.3 on both canvas samples' keyboard paths, virtual-node names and roles, in the `a11y` leg and stand-repeat (a11y-plan §9)

@chunk in=@e6 as=c32
Capability statement — what 0.4.0 adds and its stated limits, span set and accessibility reach by hand, the README's 0.4.0 done (v040-03 reach)

@chunk in=@e6 as=c33
Cold-agent bar — the pipe, its reply check and verb counts widened to canvas apps and new verbs; fresh-agent goals over canvas, virtual nodes, `drag` (v040-15)

@chunk in=@e6 as=c34
Cold-agent live run — one fresh agent on the stand, the two canvas samples among its apps, judged at 5 % wrong calls (v040-15)

@capability key=v040-01
title: Build directory held
requirement: The build directory's growth cause is measured and mended, and a working day's growth after the mend is stated as a number
observed_gap: 2026-11-19 06:11Z at bc7661240 (the overseer's ES142): target/debug/deps held 773 GB, written at 83 to 162 GB a day since 2026-11-13; the weight is 193 integration-test binaries of about 145 MB each, each standing in 30 to 65 stale hashes; during the night's mutation tier the host fell to 58 GB free of 930, and a cargo clean removed 896 GiB. The cause (a low-crate edit re-hashing every test binary while cargo collects nothing; feature sets differing between gate entries possibly multiplying it) is a hypothesis, not measured.
method: manual
acceptance: The cause of the growth is measured and named; after the mend, a working day of ordinary gate runs on the dev host grows the build directory by a stated number of gigabytes, read before and after, and every check stays green.

@capability key=v040-02
title: Upstream sync
requirement: Each epoch opens with a two-parent merge of upstream's new work, our checks and CI proving our logic survived, each commit left out named with its reason
observed_gap: 2026-11-19 06:44Z after a fetch: upstream main at 5b8e7a268, 209 commits not in the build branch, the ones earlier syncs left out on purpose among them (residual res-7qgqygtj: 102 at e5305a4c1, each with its reason — the git AnyRender pins, Dioxus 0.8 alpha, the CDP TCP listener res-ipcah32k); upstream's status page lists position sticky as in progress.
method: integration
acceptance: At each epoch's first entry, upstream's new work is merged with two parents; the workspace checks, the a11y leg and the audit read green on CI; each upstream commit left out is named with its reason; the merge-surface residuals are re-read, and the CDP listener's security amendment stands before any take that brings it.

@capability key=v040-03
title: The version on the GitHub page
requirement: The README states 0.3.0 closed, in the form the founder chooses, and says what 0.4.0 builds; the records' category-file citation backlog is read through or kept by the founder's word
observed_gap: The README's 0.3.0 heading reads «built, its cold-agent run passed» (the overseer's interim heading) while 0.1.0 and 0.2.0 read «done and verified»; the version-end audit of 0.3.0 is recorded (ledger tx 2303). The citation backlog F334 (res-xe7pyput) holds three hand-read sites, five held rows and two unresolved rows.
method: manual
acceptance: The README on the build branch states 0.3.0 closed in the form the founder chose and names what 0.4.0 builds; the citation backlog's remaining rows are each read through or kept as a residual by the founder's word.

@capability key=v040-04
title: Select list and selection debts
requirement: The select list's highlighted-row contrast, a larger authored font, the highlight's reading, a single select's size and Cells stating its selected cell are each fixed or kept by the founder's word
observed_gap: A highlighted row's contrast is unread by the lint (res-tym5wois); a larger authored font clips in the list's one row height (res-2iu7qn2e); the highlight is in no snapshot and no tree (res-caouqg3t); a single select's size stays unread (res-ysz67imk); Cells' cells stay keyed and carry no selected reading (res-wvatzhpe).
method: integration
acceptance: Each of the five debts reaches the founder as a fork; each one fixed is proven headless on the stand or a fixture in both layout modes, and each one kept stands as a residual with his reason.

@capability key=v040-05
title: Driver debts
requirement: press with Ctrl, the UI graph read in parts by its ruled form, and handler coverage under a stopped bubbling dispatch are each fixed or kept by the founder's word
observed_gap: press carries no Ctrl, so a non-adjacent multiple set cannot be built through the driver (res-hfvjj77l); the graph answer is not paged, 293,869 bytes on Cells at boot, its form ruled as graph --part N (res-7cc5bi4k); handler coverage counts every ancestor listener a bubbling dispatch passed (res-4dr3wt62).
method: integration
acceptance: Each of the three debts reaches the founder as a fork; the graph is read in parts as the founder ruled, every part within the answer bound; each debt fixed is proven on the stand through the driver's carriers, and each one kept stands as a residual with his reason.

@capability key=v040-06
title: Engine debts
requirement: A focused control that no longer shows, a task polled during a render, a page script's unseeded randomness and the growing error-boundary registry are each fixed or kept by the founder's word
observed_gap: A focused control hidden by attribute, style or CSS keeps focus and takes keys (res-6k56erh6); a task polled during a Dioxus render acts on the previous render's document (res-433ttjb2); a page script's Math.random is the host's, not the run's seed (res-jqbadec2); the error-boundary registry only grows for a document's life (res-k2zggoqx).
method: integration
acceptance: Each of the four debts reaches the founder as a fork; each one fixed is proven headless on the stand or a fixture in both layout modes, and each one kept stands as a residual with his reason, a wait on another project's release named as such.

@capability key=v040-07
title: Dependency and CI debts
requirement: The advisory ignores (the browser's database crate, paste, memmap2, the zbus family, zip) and the cancelled-runner watch are each fixed, retired or kept by the founder's word; one that waits on another project's release is stated as such, and one that does not close is named
observed_gap: The audit ignores RUSTSEC-2026-0459 until the browser moves to rusqlite 0.41 or later (res-j4x7iiqc), the paste advisory (res-kdhnfj5p), the zbus-family advisories (res-p73cueyv) and the zip advisories (res-h7acz56c) by ID; memmap2's unsound advisory is kept out by the unsound default (res-uqt2yd2y); a hosted macOS or iOS job cancelled with no runner is watched, 1 of 3 green runs so far (res-q2tllspz).
method: integration
acceptance: The audit leg reads green; each advisory is closed by a reachable update or stated as a wait on a named release, and any that does not close is named; the cancelled-runner watch is retired by its rule or kept by the founder's word.

@capability key=v040-08
title: Codebase health
requirement: The Epoch 6 code audit's oversize files, functions over the complexity ceiling, rising stand-test setup duplication and the browser's surviving mutants are each brought under their bar or kept by the founder's word
observed_gap: The Epoch 6 boundary audit reads the engine's document module at 3,367 lines, 25 files over 800 lines and 88 functions over the complexity ceiling; duplication rising for two boundaries (3.80, 3.81, 3.89 %), all of it stand tests repeating each other's setup; four surviving mutants in the browser's capture module.
method: integration
acceptance: The next boundary audit reads the stand tests' duplication back under its earlier reading and each oversize file and over-ceiling function named under its bar or kept by the founder's word; the browser's four mutants read caught or kept; every check stays green.

@capability key=v040-09
title: A canvas painted headless
requirement: What an app paints into a canvas from Rust appears in a headless screenshot, its text from the bundled fonts, its cross-host byte equality measured and required or given a tolerance by the founder's word; Circle Drawer moves onto a real canvas
observed_gap: An app paints through the widget trait, appended behind a feature the headless harness drops unless another crate unifies it, so whether a headless shot paints a widget at all is unmeasured; the CPU backend paints custom and resource paints transparent with no warning; a widget's paint gets the border-box size placed at the content-box origin (upstream #1578); no stand app holds a canvas, Circle Drawer's canvas being an ordinary element.
method: integration
acceptance: On the stand, a shot of Circle Drawer shows the circles its app painted from Rust and canvas text drawn from the bundled fonts, in both layout modes; the canvas shot's bytes are measured on CI's Linux, macOS and Windows hosts and held equal or within the tolerance the founder set.

@capability key=v040-10
title: The canvas read as data
requirement: A driver verb returns a canvas's draw commands as data, deterministic, hardware-independent and diffable, grouped by virtual node, at a summary and a full level, with lint-like checks inside the canvas; what it may carry to a provider is the founder's fork
observed_gap: The renderer records a scene as serializable commands (fill, stroke, glyph run, layers, clips) and a scene serializer is in the workspace, used by the browser alone; no driver verb reads a canvas's content.
method: integration
acceptance: On the stand, the verb answers Circle Drawer's draw commands at a summary and a full level, the same answer on two runs and on every CI host, a diff naming exactly what one action changed, the commands grouped under the virtual node that covers them; a canvas lint names a label overflowing its region on a planted fixture and reads clean on the stand; a dump's size on the stand is stated; the crossing the founder ruled holds on every carrier.

@capability key=v040-11
title: Virtual nodes inside a canvas
requirement: An app declares nodes inside a canvas with an author key, role, name and region, and the driver treats them as elements across the snapshot, diff, click, refusals, UI graph, actions, coverage, layout and exploration; Circle Drawer's circles are virtual nodes
observed_gap: A snapshot node needs an author id and a DOM element, so an agent sees a canvas as one opaque element; the UI graph reserves no id form for a non-DOM node; the layout verb reserves an app-canvas placing and the widget trait a commented accessibility tree.
method: integration
acceptance: On the stand, each Circle Drawer circle reads as a node with its key, role, name and region in the accessibility tree and the snapshot; a click on one acts on its circle and its diff names it; a covered, off-screen, disabled or removed one is refused with that cause; the UI graph, actions, handler coverage, layout explanation and exploration each list it; the circles are reached and acted on by keyboard.

@capability key=v040-12
title: A drag verb
requirement: drag moves from an element or a virtual node to another or by an offset through interpolated moves, settled and refused like every action, recorded and replayed, listed where it applies; it drags the select list's thumb
observed_gap: The driver has fifteen verbs, none drag; the harness holds a drag and the mouse primitives; a drag in one jump silently fails in apps that listen to moves; the select list's thumb is painted and not draggable (res-ammkrycp).
method: integration
acceptance: On the stand, drag moves an element, a virtual node and an offset through several moves on every carrier, settles, and is refused with a cause when its source or destination cannot take it; a recorded check holding a drag replays; the actions answer lists drag where it applies; the select list's thumb dragged scrolls a long list.

@capability key=v040-13
title: Pan and zoom under a transform
requirement: A target inside a panned and zoomed container is read, hit and acted on where it shows; zoom is its own verb or a stated limit by the founder's word; the surface's speed is measured at a stated node count and held as a performance budget, a check reading red past it
observed_gap: The visible-region reader is exact under a translate and a scale and reads a bounding box under a rotation, and was not proven on a pan/zoom container; the driver's scroll scrolls and nothing zooms; speed with hundreds of positioned nodes is unmeasured; the test plan sets no performance budget.
method: integration
acceptance: On the stand's pan/zoom surface, a target inside the panned and zoomed container reads its bounds where it shows and takes a click there; zoom is a verb or a stated limit as the founder ruled; the surface's speed at the stated node count is measured, and a check holding it to its budget reads red on a slowdown past it.

@capability key=v040-14
title: The node-graph stand app
requirement: A new stand app of a few nodes with ports and curved wires in a canvas, its nodes dragged, a wire pulled port to port and the surface panned and zoomed, each with its stand checks
observed_gap: The stand holds eight apps (counter, temperature converter, flight booker, timer, CRUD, circle drawer, cells, order), none with a canvas.
method: integration
acceptance: The stand serves a node-graph app of three or four nodes with ports and curved wires in a canvas; through the driver a node is dragged, a wire is pulled port to port and the surface is panned and zoomed, each read back in the snapshot and the draw commands, in both layout modes, each with a keyboard path or a stated limit.

@capability key=v040-15
title: Cold-agent run
requirement: 0.4.0 ends with one cold-agent run on the stand, the two canvas samples among its apps, the canvas read as data, a virtual node acted on and drag used at least once, judged at 5 % wrong calls; its budget and count the founder's word
observed_gap: 0.2.0's run read 66 calls, 0 wrong, $0.82; 0.3.0's read 76 calls, 0 wrong, seven apps, $1.03, under the 5 % bar.
method: dynamic-external
acceptance: One fresh agent's run on the stand, within the budget and count the founder set, meets both canvas samples, reads a canvas as data, acts on a virtual node and drags at least once, and its wrong calls stay at or under 5 %; the run is stated as measured, with its reach.
