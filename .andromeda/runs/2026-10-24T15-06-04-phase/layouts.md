# layouts extract

## Relevance
partial — the chunk builds checks and fixture pages, and creates or moves no surface region. It touches layout only where its proofs read the stand's TaskShell mount, the CRUD list's nested scroller, the lint's boot readings and the `escher-session` command line and `mcp` role's stdout and stderr shape.

## Constraints
- The headless stand proof mounts one lean task (Counter, FlightBooker, Timer, Crud) in TaskShell through `task_in_shell` — `main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body` — at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans. A sweep check that boots the stand reads that mount and adds no other one (per layout-templates §Surface: desktop-native → Primary screens).
- Stable element ids on stand controls are author `id`s added as `id:` lines alone, with no class, style, wrapper or order moving and no CSS selecting by them. A gap the sweep fills on the stand must keep that shape, so that `unkeyed_actionable()` reads 0 on all seven tasks with the Circle Drawer dialog open included. Whether the code already reads so is research's question (per layout-templates §Surface: desktop-native → Primary screens).
- A held driver instance (the `escher-session` host role and the `mcp` role) must read as a fresh boot, with the TaskShell mount and the pinned viewport holding after commands in both layout modes. A sweep check through either role inherits that requirement (per layout-templates §Surface: desktop-native → Primary screens).
- On the headless stand the nested scroller is `crud-list`, which clips its rows. After twelve Creates the last row lies below the list's box with the viewport unscrolled, while `task-body` is not the box that clips it. C1's clipped-and-truncated reading, if proven on the stand rather than on a fixture, must be read against that box and not the viewport (per layout-templates §Surface: desktop-native → IA notes).
- The stand's layout is required to read clean to the visual lint: 0 findings at boot on each lean task in both layout modes, and `crud-person-14` reading `clipped` inside `crud-list` after twelve Creates. The sweep must not move stand markup, ids, classes or styles in a way that changes those readings (per layout-templates §Surface: desktop-native → IA notes).
- The `escher-session` command line writes one uncoloured line of JSON on stdout per command, and its `mcp` role writes protocol lines only on stdout, with no ESC byte. The stderr id-free measurement of `lint`, `snapshot --part` and `--record` therefore reads stderr alone. No ending writes back anything the caller typed, the `--record` path included (per layout-templates §Surface: cli → Primary screens).
- The harness viewport defaults to 800 × 600 at scale 1 in light mode. A minimal fixture page beside the stand for C1 (RTL or vertical text, a transformed box, a clipped or off-screen text node) lays out in that viewport unless the check pins another one (per layout-templates §Surface: desktop-native → IA notes).

## Patterns to follow
- Lean-task mount through `seven_guis::stand` / `task_in_shell`, the same boot reused by the host role and the `mcp` role, each adding no markup, id, class, style, wrapper or order (per layout-templates §Surface: desktop-native → Primary screens).
- 0.1.0's sweep pattern for a gap on the non-lean tasks: author `id`s on controls (`temp-*`, `circle-*`, `cell-{A..Z}{0..25}`, `cell-input`) added as `id:` lines alone (per layout-templates §Surface: desktop-native → Primary screens).
- Proving a clip on the stand through the existing nested scroller (`crud-list` after twelve Creates), with the pinned viewport scrolled only on an in-file fixture taller than it (per layout-templates §Surface: desktop-native → IA notes).
- Lint readings asserted at boot per lean task in both layout modes, as `stand_lint` states them (per layout-templates §Surface: desktop-native → IA notes).

## Anti-patterns to avoid
- Adding an element, class, style, wrapper or order change to stand markup in order to plant a case. A case the stand lacks belongs on a minimal fixture beside it (per layout-templates §Surface: desktop-native → Primary screens).
- Writing anything other than the answer line or protocol lines to stdout, or echoing caller-typed text (ids, `--text`, the `--record` path) in any ending, while instrumenting the stderr measurement (per layout-templates §Surface: cli → Primary screens).

## Contract bindings
- layouts ↔ a11y: the CRUD Tab sequence (`back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`) is stated in layout-templates §Surface: desktop-native → Primary screens. Its re-assertion belongs to «A11y re-read» (working-route.md:51), not this chunk. Any stand change the sweep makes must leave that order unmoved.
- layouts ↔ tests (test-plan §3 Session lifecycle): the `escher-session` host role and `mcp` role contract that the stderr measurement runs on is referenced from layout-templates §Surface: cli → Primary screens.
- layouts ↔ obs: the host's stderr lines carry `service.name=seven_guis`, so the id-free needle read binds to the obs-plan's sink rules (per layout-templates §Surface: cli → Primary screens).

## Acceptance criteria contributions
- (layouts) Every sweep check that boots a lean task reads the TaskShell mount at the pinned 800 × 600 viewport in both layout modes, and any stand gap filled moves no element, class, style, wrapper or order. `git diff` over `examples/seven_guis/src/tasks/` shows `id:` lines only, or nothing (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) After the chunk, each lean task still reads 0 lint findings at boot in both layout modes, and `crud-person-14` reads `clipped` inside `crud-list` after twelve Creates (per layout-templates §Surface: desktop-native → IA notes).
- (layouts) The stderr measurement runs of `lint`, `snapshot --part` and `--record` (host role and `mcp` role) leave each command's stdout as one uncoloured JSON line, or protocol lines only, with no ESC byte and nothing the caller typed written back (per layout-templates §Surface: cli → Primary screens).
