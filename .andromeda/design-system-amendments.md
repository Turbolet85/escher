# design-system — amendments

One entry per amendment to `design-system.md` (sidecar-contract.md §Entry form). Appended at wrap P2; rewritten only by the epoch-close consolidation.

## 2026-10-05-fork-ci-reached — file:line citations re-pointed after the CI files moved
**Section:** every section citing `ci.yml` or `publish-browser.yml` lines
**Change:** 4 citations re-pointed — `publish-browser.yml` from line 37 on +1 (the repository guard), `ci.yml` by a range map over the rewritten file (the ios/android matrix entries now 237-252); no claim text changed by the re-point.
**Why:** this chunk moved the cited lines.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-ci-gate-legs — file:line citations re-pointed after ci.yml moved
**Section:** every section citing `ci.yml` lines
**Change:** 2 citations re-pointed — the ios/android matrix entries 237-252 → 328-343, by a measured line map over ci.yml; no claim text changed by the re-point.
**Why:** this chunk moved the cited lines.
**Ref:** .andromeda/runs/2026-10-05T23-50-17-wrap/

## 2026-10-06-telemetry-bootstrap — file:line citations re-pointed after Cargo.toml moved
**Section:** every section citing `Cargo.toml` lines
**Change:** 2 citations re-pointed by the chunk's measured line shifts of the root manifest (a member, a workspace-dependency entry and `tracing-log` inserted).
**Why:** this chunk moved the cited lines.
**Ref:** .andromeda/runs/2026-10-06T01-33-33-wrap/

## 2026-10-06-headless-stand — the headless stand registers the bundled DejaVu Sans on native
**Section:** §Typography → Loading
**Change:** a new Loading bullet: the seven_guis headless stand registers the bundled DejaVu Sans for every generic with system fonts off on native — `build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`, the woff2 decoded by seven_guis' native `woff` feature. The WASM bullet stands.
**Why:** the headless stand chunk pins text measurement to one bundled face so layout is the same on the dev host and the CI runner.
**Ref:** .andromeda/runs/2026-10-06T02-44-36-wrap/

## 2026-10-06-upstream-sync-element-identity — file:line citations re-pointed after the upstream merge
**Section:** every section citing a merged upstream file's lines
**Change:** 17 `file:line` citations into the merged upstream files re-pointed by the merge's measured line map; no claim text changed by the re-point.
**Why:** the chunk merged upstream `main` at `23354585`; the merged files' lines moved.
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/

## 2026-10-06-stable-element-ids — citations re-pointed after the author-id inserts
**Section:** the sections citing `counter.rs` and `flight_booker.rs` (palette, typography, spacing, contrast rows)
**Change:** 15 `file:line` citations into `dioxus_document.rs`, `lib.rs` and the four lean-task files re-pointed by the chunk's measured line maps; no claim text changed — the task markup gained `id:` attributes only.
**Why:** the chunk inserted author `id:` lines into the lean-task files, moving the cited CSS lines.
**Ref:** .andromeda/runs/2026-10-06T09-35-44-wrap/

## 2026-10-06-accessibility-tree-identity — citations re-pointed
**Section:** citations into `flight_booker.rs`, blitz-dom `document.rs` and dioxus-native-dom `dioxus_document.rs`
**Change:** 13 citations re-pointed by the chunk's measured line map (5 `flight_booker.rs`, 7 `document.rs`, 1 `dioxus_document.rs`); no claim text changed — the flight booker's two date inputs gained `aria_label` lines only, and no token, colour, type or radius moved.
**Why:** the inserted attribute lines and the new trait method moved the cited lines.
**Ref:** .andromeda/runs/2026-10-06T12-55-34-wrap/

## 2026-10-06-id-stability-across-code-edits — seven_guis `app.rs` citations re-pointed
**Section:** §Color Palette → Core Colors · → Surface Scale · → Text Hierarchy · §Typography · §Spacing · §Border Radius — the sites citing `examples/seven_guis/src/app.rs`
**Change:** coordinates only — seven citations on six lines moved by nine lines (`173` → `182` three times, `169-195` → `178-204`, `181` → `190`, `218` → `227`, `259` → `268`). No token, value or claim changed.
**Why:** `app.rs` gained a `slug` field, seven slug values and one `id` attribute above the cited CSS; no style changed.
**Ref:** .andromeda/runs/2026-10-06T20-55-58-wrap/

## 2026-10-07-change-tracking-and-diff — blitz-dom `document.rs` citations re-pointed
**Section:** citations into blitz-dom `document.rs`
**Change:** coordinates only. 7 of 12 citations re-pointed by the chunk's measured line map — `:398-400` → `:400-402`, `:591-606` → `:593-608`, `:1761-1778` → `:1772-1789`, `:2028-2045` → `:2039-2056`, `:2078-2085` → `:2089-2096`, `:2088-2109` → `:2099-2120`, `:2156-2210` → `:2167-2221`; the 5 into blitz-shell `window.rs` keep their numbers. No token, value or claim changed.
**Why:** the chunk added the changed set's doc lines, its drain and its mark to `document.rs` and renders no UI.
**Ref:** .andromeda/runs/2026-10-07T01-31-03-wrap/

## 2026-10-07-settle-detection — the settle rule's one answer for the keeps-animating set
**Section:** §Motion → Animation runtime (the keeps-animating bullet and the animation-clock bullet)
**Change:** a harness settle answers for every member of the keeps-animating set the same way — none holds it open and none is waited on or advanced: it returns `Settled` with `animating` reading the document's animating flag; one member, a CSS animation, is exercised by a check, the others are answered by the same reading and not each exercised. Settle reads animation time only through the harness's controlled clock and never advances it. The harness-clock citation is re-pointed to the lines measured after the chunk (it was stale before it).
**Why:** the set's scrollbar-fade member reads the wall clock and a canvas reads animating for as long as it exists, so waiting on the flag would need a sleep; the caller holds the clock.
**Ref:** .andromeda/runs/2026-10-07T11-06-31-wrap/

## 2026-10-07-act-by-id — the driver's `advance` moves app time and leaves the animation clock alone
**Section:** §Motion → Animation runtime
**Change:** the harness-clock bullet gains one clause: the driver's `advance` moves an app's own time — through the step a session's caller hands it, on the stand whole Timer ticks — and leaves the harness's animation clock where it was, as do `click`, `type` and `press`; a driver click beside a running CSS animation returns settled without waiting on it. No style value, token or markup changes, and no cited line of this document moved.
**Why:** the chunk wired `advance`, and the two clocks must not be read as one: the app's time is the caller's to move, the animation clock is the harness's and moves only by `tick`. Measured on the stand's Timer (the clock reads the same before and after every call) and on a fixture with a running animation.
**Ref:** .andromeda/runs/2026-10-07T14-22-35-wrap/

## 2026-10-07-refusal-detection — an into-view scroll never animates a nested box
**Section:** §Motion
**Change:** a new bullet: `BaseDocument::scroll_into_view` writes every scrolling box that holds its target at once, innermost first, whatever behaviour was asked, and the requested behaviour — `Smooth` included — applies to the viewport alone; the driver's `scroll` is instant in every box and in the viewport, inside one settled step. The touch-fling citation is re-pointed (`scrolling.rs:724-747` → `:875-898`).
**Why:** the engine method was widened for every document, ratified by the founder (2026-10-07), his own choice relayed verbatim by the overseer and confirmed by the operator at this wrap's escalation. The document holds one scroll animation at a time, so smooth travel of a nested box was not built.
**Ref:** .andromeda/runs/2026-10-07T20-27-47-wrap/

## 2026-10-10-upstream-sync-agent-surfaces — the default link rule selects `a[href]`
**Section:** §Color Palette → Core Colors, the "blitz-dom default link" row
**Change:** the row's usage was "Default stylesheet link"; it now says the rule selects `a[href]` — an anchor with no `href` takes neither the colour nor the underline. The value, `rgb(0, 0, 238)`, is unchanged.
**Why:** upstream narrowed the default stylesheet's link rule from `a` to `a[href]`, as the HTML specification styles only links. No stand source holds an `a`, so no stand reading moves; no test is named for the rule.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/

## 2026-10-10-driver-cli — the driver command line's output form
**Section:** §Surface: cli → Component Patterns
**Change:** a new bullet: `escher-session` prints one line of JSON on stdout per command — the answer, the refusal or a session error's object, keyed by the driver schema's own words — and nothing else there; a usage error prints one fixed line on stderr with stdout empty, a session error its fixed message on stderr; no colour and no colour dependency, no ESC byte on any answer line, and no code asks whether a stream is a terminal, so the output is the same piped or not; the exit status carries the class, 0 accepted · 1 refused · 2 usage · 3 session error.
**Why:** the surface's reader is an agent, so its form is the schema's words, uncoloured (the founder's answers on the shape of stdout and on exit codes, 2026-10-10, relayed by the overseer). No detector covers an output form; the amendment was raised from the plan's reviewed list.
**Kept:** the WPT runner's colours and the surface's two NOT YET MEASURED sections stand as they were.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-scrolling-box-bounds-and-hit — the hit walk's edge rule
**Section:** §Depth Strategy → Engine depth behavior
**Change:** the hit-testing sentence, which stated the walk's order only (positive-z hoisted children, paint children in reverse, negative-z hoisted children), now also says where the walk stops: at the padding box of a node that clips by `overflow`, the root element excepted — a point outside that box reaches none of the node's hoisted children, paint children or inline content, as paint clips them, and the node itself is still answered inside its border box.
**Why:** the engine's hit was fixed to stop where paint clips by `overflow`; until then a hit reached content scrolled out of its box. The order inside the box is unchanged.
**Ref:** .andromeda/runs/2026-10-10T15-29-49-wrap/

## 2026-10-10-mcp-surface — the `mcp` role's stdout on the cli surface
**Section:** §Surface: cli → Component Patterns (the `escher-session` bullet)
**Change:** the `escher-session` bullet gains its `mcp` role — a surface an agent's client reads: newline-delimited JSON-RPC 2.0 protocol lines only on stdout (a reply per request that is not a notification; a tool result's text the screen text for `snapshot`, the JSON line otherwise), uncoloured, no ESC byte in any reply and still no terminal test; its end writes nothing there, an `Err` ending's fixed message on stderr with status `3`.
**Why:** a new mode of the same cli surface; the no-ESC reading is asserted on every reply by the shared `Speaker` and the no-terminal-test search counts 0. No token, colour or layout entered.
**Ref:** .andromeda/runs/2026-10-11T08-35-17-wrap/

## 2026-10-12-self-description — `escher-session help` and the exit-status table on the cli surface
**Section:** §Surface: cli → Component Patterns (the `escher-session` bullet)
**Change:** the exit status's four classes are now read from the schema's `EXIT_STATUSES` table, whose meanings `help` serves. `escher-session help` answers the tool's self-description as one uncoloured line of JSON on stdout, no ESC byte, status `0`, reaching no session. A session error's object carries its fixed `remedy`. In the `mcp` role, a `help` tool result's text is the self-description's line.
**Why:** the chunk added the `help` answer line to the cli surface; it keeps the surface's form — one line of JSON, uncoloured, no terminal test.
**Ref:** .andromeda/runs/2026-10-12T08-55-10-wrap/

## 2026-10-12-headless-screenshot — the shot's answer line, the MCP image block, no font refused
**Section:** §Typography → Loading · §Surface: cli → Component Patterns
**Change:**
- Loading: a driver `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted — text is never painted skipped; the stand's bundled font passes.
- cli: `escher-session screenshot [--id]` answers one uncoloured line `{"width","height","png"}` on stdout, no ESC byte, status `0`, the image as base64 text inside it; the `mcp` role answers a shot with two content blocks — an `image` block (`image/png`, base64) then a text block, the line without `png` — every reply still one line with no ESC byte.
**Why:** the surface stays one an agent reads; the image in band is the founder's fork 1, ratified by his own word (2026-10-12, by question dialog in the overseer session, relayed verbatim by the overseer; the wrap's answer card confirmed the same way, 2026-10-13).
**Ref:** .andromeda/runs/2026-10-12T20-34-17-wrap/

## 2026-10-13-upstream-sync-polish-and-ship — the default sheet arrives through the config
**Section:** §Surface: desktop-native → Tokens (platform-specific)
**Change:** the user-agent-sheet line: was "every document gets the blitz `DEFAULT_CSS` user-agent stylesheet" by `DioxusDocument::new`'s own call; now every document gets it once, through `DocumentConfig::default()` (`UserAgentStylesheet::default_sheet()`), the call removed by upstream #1189.
**Why:** upstream's mechanism, merged; the claim stands and only its path changed. The reconcile was confirmed by the founder (2026-10-13, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Подтвердить (Рекомендую)»; the English wording is the overseer's).
**Ref:** .andromeda/runs/2026-10-13T15-50-37-wrap/

## 2026-10-14-stand-requirement-sweep — the hit at an element's own box measured
**Section:** §Depth Strategy → Engine depth behavior (the hit-testing bullet)
**Change:** the bullet adds the measured answers at an element's own box — an `img` holding raster data answers the `img`, a block inside an `iframe`'s `srcdoc` sub-document answers the `iframe` (the parent's walk does not enter it, pinned as it reads), a filled text `input` answers the `input` — and a box both transformed and scrolled hit where its rows show, not through its scrolled-out content; both layout modes, both builds.
**Why:** measured at this chunk; widening the stop to a `contain: paint` box stays deferred on the founder's C4 «Отложить (Рекомендую)», by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label.
**Ref:** .andromeda/runs/2026-10-14T22-00-51-wrap/

## 2026-10-17-upstream-sync-engine-debts — the bump app leaves §Surface: cli
**Section:** §Surface: cli (Platform · Component Patterns)
**Change:** the Platform line no longer names the `bump` app among the binaries that print to the terminal, and the «bump prints "Bumped anyrender versions" or "Bumped blitz versions"» pattern is gone.
**Why:** upstream's `23b9b26a9` "Delete the bump crate", taken by the sync.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/
## 2026-10-18-engine-debts — the hit stops at a `contain: paint` box
**Section:** §Depth Strategy (Engine depth behavior, the hit-testing bullet)
**Change:** the hit stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` on a box paint applies it to — the root element excepted; was «a node that clips by `overflow`» alone.
**Why:** F5 A — PROVISIONAL, delegate operator; hit and paint agree. No design-system line states a client rect, so F8 moves nothing here.
**Ref:** .andromeda/runs/2026-10-18T04-30-47-wrap/
## 2026-10-18-upstream-sync-determinism-and-traces — §Iconography: the cursor over no box reads Default
**Section:** §Iconography (Cursors and favicons (engine), the cursor-source bullet)
**Change:** «Default otherwise» now covers a pointer over no box, so the shell keeps a visible arrow there; was: over no box `get_cursor` answered no cursor, which the shell hides. `cursor: none` still maps to no cursor, so the shell's `None` path stands (the `ShellProvider::set_cursor` bullet unchanged). Upstream's `hovering_below_content_reports_default_cursor` pins it.
**Why:** upstream's #1220 (`fb8d6c15e`), taken by the 2026-10-18 sync on F1 answered B by the operator as the founder's delegate — an engine behaviour change for every document, PROVISIONAL, in the founder's epoch-boundary discharge batch on the operator's direction (the operator, 2026-10-18); no crossing widens, the value reaching `ShellProvider::set_cursor` alone, as the operator agrees.
**Ref:** .andromeda/runs/2026-10-18T06-47-20-wrap/
## 2026-10-18-one-virtual-clock — §Motion: the harness drives animation from the documents' clock, PROVISIONAL
**Section:** §Motion (the settle-and-animating bullet; the animation-clock bullet)
**Change:**
- Settle: `animating` reads the document's animating flag, the scrollbar fade's membership read off the document's clock — after a driver `scroll` it reads true until 700 ms of that clock pass.
- Animation clock: was «the test harness drives animation from a controlled clock advanced by tick … the driver's `advance` moves an app's own time … and leaves that clock where it was»; now the harness drives animation from the `VirtualClock` its documents read — the overlay scrollbar fade, smooth scrolling and the fling read it headlessly — moved by `tick`, `advance_time` and the driver's `advance`, which moves it by exactly its `ms` and the app's own time through the session's step together; `click`, `type` and `press` leave it where it was. `FADE_DELAY`, `FADE_DURATION` and the smooth and fling constants are unchanged. PROVISIONAL.
**Why:** the founder's forks F1 B and F2 A, answered by the operator as the founder's delegate (delegate the operator, 2026-10-18), PROVISIONAL, for the founder's epoch-boundary batch (the operator, 2026-10-18, at the wrap resume).
**Ref:** .andromeda/runs/2026-10-18T08-45-11-wrap/
## 2026-10-18-engine-debts — the PROVISIONAL batch ratified by the founder (Epoch 2 close): three batches, 3 marks
**Section:** §Depth Strategy (the hit at a box paint clips) · §Motion (the harness drives animation from the documents' clock) · §Iconography (the cursor over no box)
**Change:** each PROVISIONAL mark now reads `ratified by the founder (2026-10-18)` (inside parentheses `ratified by the founder, 2026-10-18`), the clause unchanged: the stop at a computed `contain` that includes `paint` (engine-debts); `advance` moving the clock by exactly its `ms` and the app's own time together (one-virtual-clock); upstream's #1220 «taken on F1 answered B by the founder's delegate, ratified by the founder (2026-10-18)» (the cursor fix). Partly retires the PROVISIONAL clause of the earlier 2026-10-18-engine-debts, 2026-10-18-upstream-sync-determinism-and-traces and 2026-10-18-one-virtual-clock entries here; each one's other claims stand.
**Why:** the Provisional discharge at the epoch boundary, on the founder's own word: «Утвердить всё (Рекомендую)» on the engine-debts batch, «Утвердить (Рекомендую)» on the cursor fix F1 B, «Утвердить всё (Рекомендую)» on one-virtual-clock (the founder, 2026-10-18, by question dialog in the overseer session, relayed by the overseer with his chosen labels verbatim), discharged at this wrap on the operator's direction (the operator, 2026-10-18, at the gated wrap's resume). No token moves.
**Ref:** .andromeda/runs/2026-10-18T20-46-02-wrap/
## 2026-10-19-session-recording — the two recording session errors in the exit-3 class
**Section:** §Surface: cli → Component Patterns
**Change:** the exit-status class line now names the recording's `record-dir-not-private` and `recording-failed` among the session errors, status `3`, with no new exit; the four statuses are unchanged.
**Why:** this chunk's two new session errors, fixed text inside a ratified carrier (the playbook's «The program's own fixed text inside a ratified carrier»); the plan's expected amendment.
**Ref:** .andromeda/runs/2026-10-19T05-20-24-wrap/

## 2026-10-20-load-wait — the load wait moves no clock
**Section:** §Motion → Animation runtime
**Change:** a new bullet: the driver's load wait moves no clock — while an acting step waits on a load in flight, for at most `LOAD_WAIT` (5 s of wall time), the harness's `VirtualClock` stands still, so a timer or an animation does not run during the wait.
**Why:** this chunk's own wait (the plan's reviewed expected amendment); settle's own claim beside it stays true.
**Ref:** .andromeda/runs/2026-10-20T03-00-02-wrap/

## 2026-10-20-select-by-id — the user-agent select box
**Section:** §Surface: desktop-native → Tokens (platform-specific)
**Change:** a new bullet — in the user-agent sheet a `select` takes `input, textarea`'s border (`1px solid #999`), padding (`2px`) and background (`white`) and `display: inline-block`, and a single-choice select shows its chosen option alone (`select:not([multiple]) > option:checked { display: inline; }` beside `option { display: none; }`), so it lays out as a box holding its chosen option's label — engine defaults, no design token.
**Why:** this chunk's stylesheet change (the plan's expected amendment); the engine showing the chosen option's label is F2 «Engine shows its label», ratified by the founder (2026-10-20, «Утвердить все четыре (Рекомендую)», relayed verbatim by the overseer, chunk inputs#I3).
**Ref:** .andromeda/runs/2026-10-20T05-49-12-wrap/

## 2026-10-20-large-screen-in-parts — the paged `snapshot` answer line on the cli surface
**Section:** §Surface: cli §Component Patterns
**Change:** `escher-session snapshot [--part <part>]` answers one uncoloured line with no ESC byte, `{"text":…}` for a whole screen and `{"text":…,"part":n,"parts":m}` for a screen read in parts, status `0`. Over the `mcp` role a paged `snapshot` result is that JSON line (was the screen text alone).
**Why:** the paged answer the founder ratified, F1 «Аргумент part, части по умолчанию (Рекомендую)», by question dialog in the overseer session, relayed verbatim by the overseer (2026-10-20, chunk inputs#I2); his card confirm «Подтвердить все (Рекомендую)» (inputs#I5).
**Ref:** .andromeda/runs/2026-10-20T11-11-53-wrap/

## 2026-10-24-u55-door — design-system: split into its body and 8 category files
**Section:** (the whole master — the door)
**Change:** split into its body and 8 category files; the map .andromeda/design-system.map.tsv
**Why:** the operator's word: the founder's standing word of 2026-10-24 for the door on escher's masters, given after one review round, the three open rows taken in overseer1's wording from the master's own lines; relayed by overseer1.
**Ref:** $TMPDIR/claude-1000/-home-turbolet-dev-projects-additional-andromeda-worker/9a5434fe-fa07-4859-9671-f503cd8de21b/scratchpad/u55-door-2026-10-24/design-system/

## 2026-10-24-upstream-sync-polish-and-ship — the hit walk skips a dropped node
**Section:** §Depth Strategy (depth-strategy.md: the stacking and hit-testing order)
**Change:** the hit-testing walk's order line adds that it skips a node dropped since the last resolve, as the paint tree's walk does, pointing to architecture §Standard Contracts → Scrolling, selection, tree; its two `node.rs` cites re-pointed (`1568-1632`, `1534-1567`).
**Why:** an upstream engine change for every document, ratified by the founder's own word «Десять коммитов (Рекомендую)», relayed verbatim by the overseer (2026-10-24, chunk inputs#I4); the fact is architecture's, stated there once.
**Ref:** .andromeda/runs/2026-10-24T12-21-29-wrap/

## 2026-10-24-stand-requirement-sweep — the list-bullet font's third citation re-pointed
**Section:** §Typography (typography.md: List bullets)
**Change:** was `packages/blitz-dom/src/layout/construct.rs:1106-1112`; now `packages/blitz-dom/src/layout/list.rs:96-102` — the marker's text pushed into the bullet layout; the row's font stack and its other two citations unchanged.
**Why:** the citation sweep read the old range holding none of the row's claim (its row dispositioned «claim false»), and the marker's push into the bullet layout read at `list.rs:96-102`.
**Ref:** .andromeda/runs/2026-10-24T17-32-19-wrap/

## 2026-10-25-cold-agent-bar — the flight booker's select, the served load page and the scrollbar's band
**Section:** §Color Palette (color-palette.md: the stand accent row, a new seven_guis load page (stand) row, the Border Progression's new `flight-type` select field row) · §Border Radius (border-radius.md: the seven_guis row) · §Spacing (spacing.md) · §Typography (typography.md: the type table, the bundled font) · §Surface (surface.md: the select defaults group, a new `values · desktop-native-stand-load-page` group, the cli group) · §Motion (motion.md: the overlay scrollbar, the load wait) · the subject line (`desktop-native-stand-load-page`)
**Change:**
- Accent: white on `#3a5ce5` is the button text of FOUR accent controls — `counter-increment`, `flight-book`, `timer-reset`, `crud-create` — 14–16 px, weight 500 (was five, `flight-one-way` among them, weight 500–600).
- The flight booker's `flight-type` select (`.type-select`): `#1a1a1a` on `#ffffff` (17.40, 15px 400) inside `1px solid #c0c0c0`; focused, no outline and a `#3a5ce5` border, as the date inputs; radius 6px; padding 8px vertical, 10px horizontal; its type-table row 15px 400; it overrides the user-agent select defaults, its two options (`one-way flight`, `return flight`) showing the chosen one alone.
- The load page: `load-fetch` white on the accent at rest, hover `#2a4cd3`, active `#1f3fc1`; `load-done` `#1a1a1a` on `#f5f5f5`; no contrast row reads it, the page no lean task. Its surface group: two keyed elements, `load-fetch` and `load-done` (`display: none` until its stylesheet lands), classes `load-root` · `load-body` · `load-btn` (`:hover`, `:active`) · `load-done`, no CSS selecting by id.
- Cli: `escher-session` serves six tasks — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` — Cells and the load page in the lean four's `TaskShell`, the usage line's `apps:` and the MCP `start` tool's `enum` listing the six, an unknown task a usage ending.
- Typography: the bundled DejaVu Sans for every boot, `boot_cells` and `boot_load` among them.
- Motion: right after a scroll the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag and dispatches nothing, stays shown until an `advance` passes its fade, and a pointer left on the thumb keeps it shown — a driver `click` on a Cells cell in that band takes an `advance` past the fade first; the load page's `LoadNet` answers its one stylesheet after `LOAD_ANSWER_DELAY` (200 ms) from a thread of its own, so a `click` on `load-fetch` waits it out.
**Why:** the flight booker's mode moved from two `aria-pressed` buttons to one select and the stand serves Cells and a load page beside the lean four, measured at the chunk; a stand change, no engine crate moving and no token added — the load page reuses the stand accent.
**Ref:** .andromeda/runs/2026-10-25T17-25-04-wrap/

## 2026-10-27-readme-and-records — the stand's radii, card shadows and card padding leave for layout-templates, PROVISIONAL
**Section:** §Border Radius (border-radius.md: the seven_guis row) · §Depth Strategy (depth-strategy.md: the seven_guis cards item) · §Spacing (spacing.md: the seven_guis cards item)
**Change:**
- Border Radius: was a «seven_guis (untokenized)» row — cards 8px, buttons 6px, inputs 4px, task cards 6px, tags 3px, the flight booker's `flight-type` select 6px; now a «seven_guis stand» row holding no value: the stand's card, button, input, task-card, tag and flight-type select radii are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values.
- Depth Strategy: was «seven_guis cards use `0 2px 8px rgba(0, 0, 0, 0.08)`; the circle dialog uses 0.10»; now the stand's card and circle-dialog shadows are held in that layout-templates block, not as design-system values.
- Spacing: was «seven_guis cards pad 24px 32px with 16px gap; home pads 48px 32px 64px»; now the stand's card and Home padding is held in that layout-templates block, not as a design-system value.
**Why:** every seven_guis stand row leaves the product surface, its values as they stand (the report's Expected amendments 2 and 3, the move set's rows 7–9): a stand's own values are layout-templates', by the founder's rule of 2026-10-25 «The Apply benches». F2 «Every stand row (Recommended)» is the founder's delegate answer, given by the operator by question dialog at phase P4 (chunk inputs#I3), PROVISIONAL until the founder's own word, which supersedes it by rule. Each pointer went in with the removal it stands beside, atomically, on the operator's Q2 «Apply with their pairs (Recommended)» (the escher overseer, by question dialog at this wrap's P2 Validate, inputs#I8).
**Kept:** spacing's `flight-type` select padding (8px vertical, 10px horizontal) stands — one of the ten further stand lines the operator's Q1 «The nine rows only» (inputs#I8) left for the founder's word, F2 not widened past the plan's nine rows.
**Ref:** .andromeda/runs/2026-10-27T05-59-26-wrap/

## 2026-10-27-readme-and-records — the stand's colours leave the product surface, PROVISIONAL
**Section:** §Color Palette → Core Colors (design-system.md body; color-palette.md: the seven_guis accent and load page rows) · → Semantic Colors (color-palette.md: the seven_guis success row) · → Border Progression (color-palette.md: the flight-type select field row)
**Change:**
- Body, Core Colors: the seven_guis accent line (`#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`, white button text passing SC 1.4.3's 4.5 at rest, on hover and when active) removed; the subsection keeps its Detail pointer alone.
- Core Colors: was a «seven_guis accent» row (its shades, the old `#4a6cf7`, its `rgba(58, 92, 229, …)` uses, white text 5.47 · 6.83 · 8.27, the old 4.39) and a «seven_guis load page (stand)» row (`load-fetch` on the accent, `load-done` `#1a1a1a` on `#f5f5f5`); now one «seven_guis stand» row holding no value: the stand's accent (its load page's `load-fetch` included), its flight booker's success-message colours and its flight-type select's border are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values.
- Semantic: the «seven_guis success» row (`#ebf8ee` / `#68d391` / `#276749`) removed.
- Border Progression: the «Field — seven_guis flight-type select» row (`1px solid #c0c0c0`, focused `#3a5ce5`) removed.
- Was the four accent controls' white text, the select's colours and the load page's colours, per «2026-10-25-cold-agent-bar — the flight booker's select, the served load page and the scrollbar's band»; now held in layout-templates.
**Why:** the report's Expected amendment 2, the move set's rows 1–5, values as they stand, on F2 «Every stand row (Recommended)» — the founder's delegate answer, given by the operator by question dialog at phase P4 (chunk inputs#I3), PROVISIONAL — and the pointer row atomic with the removals on the operator's Q2 «Apply with their pairs (Recommended)» (inputs#I8).
**Kept:** the seven_guis invalid row, the page `#f5f5f5` (shared with todomvc) and text `#1a1a1a` rows, surface.md's select overrides and load page group with the subjects line's `desktop-native-stand-load-page`, and the body's WASM-pages line stand — among the ten further stand lines Q1 «The nine rows only» (inputs#I8) left for the founder's word.
**Supersedes:**
2026-10-13-stand-contrast-harness — the seven_guis accent measured under white button text
2026-10-14-stand-a11y-assertions — the stand's accent one step darker, SC 1.4.3 met
**Ref:** .andromeda/runs/2026-10-27T05-59-26-wrap/

## 2026-10-27-readme-and-records — the stand's font registration leaves for architecture's Headless stand, PROVISIONAL
**Section:** §Typography (typography.md: the `values · typography` list)
**Change:** was «The seven_guis headless stand registers the same bundled DejaVu Sans for every generic, with system fonts off, on native — `build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`, for every boot, Cells' (`boot_cells`) and the load page's (`boot_load`) among them, decoded by seven_guis' native `woff` feature», per «2026-10-06-headless-stand — the headless stand registers the bundled DejaVu Sans on native» and the Typography item of «2026-10-25-cold-agent-bar — the flight booker's select, the served load page and the scrollbar's band»; now an item says the stand's font registration is held in architecture §Standard Contracts → Headless stand, beside `font_ctx()`, not as a design-system typography value. The WASM builds' bundled-font item and the custom `FontContext` item stand.
**Why:** the report's Expected amendments 2 and 4, the move set's row 6: the stand's font is a contract of its headless boot, stated there once — on F2 «Every stand row (Recommended)», the founder's delegate answer given by the operator by question dialog at phase P4 (chunk inputs#I3), PROVISIONAL; the pointer atomic with the removal on the operator's Q2 «Apply with their pairs (Recommended)» (inputs#I8).
**Kept:** typography.md's two seven_guis type-table rows (base 14px with a 36px 700 home title; the `flight-type` select 15px 400) stand — among the ten further stand lines Q1 «The nine rows only» (inputs#I8) left for the founder's word.
**Ref:** .andromeda/runs/2026-10-27T05-59-26-wrap/

## 2026-10-27-upstream-sync-engine-debts — F2's stand-row move ratified by the founder's own word
**Section:** §Border Radius (border-radius.md) · §Depth Strategy (depth-strategy.md) · §Spacing (spacing.md) · §Color Palette (design-system.md body; color-palette.md) · §Typography (typography.md) — the move's pointer rows, no line changed
**Change:**
- No body line moves for it: the stand's radii, card shadows and card padding, its accent, success colours and select border, and its font registration stay held in layout-templates §Surface: desktop-native → Primary screens and architecture §Standard Contracts → Headless stand, each design-system pointer standing as these entries record it, now ratified, no longer PROVISIONAL: «2026-10-27-readme-and-records — the stand's radii, card shadows and card padding leave for layout-templates, PROVISIONAL» · «2026-10-27-readme-and-records — the stand's colours leave the product surface, PROVISIONAL» · «2026-10-27-readme-and-records — the stand's font registration leaves for architecture's Headless stand, PROVISIONAL».
**Why:** F2 «Every stand row (Recommended)», the founder's delegate answer and PROVISIONAL, is ratified by the founder's own word «Подтвердить (Рекомендую)», by phone, 2026-10-27, relayed for this wrap by the operator (inputs#I7), which supersedes it by rule; the operator's Q1 «Confirm (Recommended)» at this wrap's P2 Validate (inputs#I8) carries the ratification as one new entry per touched doc.
**Kept:** the three entries keep their words — a superseded provisional record is kept, never reworked, and their move still stands, so none is named under Supersedes; their Kept clauses are retired by the two move entries below.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the stand's colours, type and select padding leave design-system, the todomvc rows split
**Section:** §Color Palette (color-palette.md: Surface Scale's page row, Text Hierarchy, Semantic Colors) · §Spacing (spacing.md) · §Typography (typography.md: the type table) · §Surface: web-spa (design-system.md body: Platform)
**Change:**
- Surface Scale: was a «seven_guis / todomvc page» `#f5f5f5` row citing seven_guis' sources beside todomvc's CSS; now a «todomvc page» row citing todomvc's CSS alone, value and usage unchanged.
- Text Hierarchy: the «seven_guis text» `#1a1a1a` row removed.
- Semantic Colors: the «seven_guis invalid» row (`#e53e3e` / `#fff5f5` / `#c53030`) removed; the disabled checkbox/radio row stands.
- Spacing: the flight booker's `flight-type` select padding item (8px vertical, 10px horizontal) removed; the stand's card-and-Home pointer item stands.
- Typography: the «seven_guis» row (sans-serif, home title 700, base 14px, home title 36px) and the flight booker's `flight-type` select (`.type-select`) row (400, 15px) removed.
- Web-spa Platform: was «seven_guis and todomvc fill the window with a canvas on `#f5f5f5`», citing both pages; now «todomvc fills the window with a canvas on `#f5f5f5`», citing todomvc's alone.
- The values are now held in layout-templates' `values · desktop-native-screens` block, unchanged. Was kept, per the colours, radii and font registration PROVISIONAL entries of 2026-10-27-readme-and-records (the invalid, page and text rows and the WASM-pages line; the select padding; the two type rows); now gone from design-system.
**Why:** the founder's own word «Все десять, общие разделить (Рекомендую)», by phone, 2026-10-27, relayed for this wrap by the operator (inputs#I7): the stand-only lines go to layout-templates' desktop-native screens with values unchanged, and the two rows shared with todomvc keep todomvc and drop seven_guis. The text row lands as no new line, layout-templates already stating `#1a1a1a`, on the operator's Q2 «Fold into :16 (Recommended)» at this wrap's P2 Validate (inputs#I8).
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — citations follow the script runtime's split and the taken commits' line moves
**Section:** §Iconography (iconography.md: the cursor item; design-system.md body: Size grid) · §Motion (motion.md: the scroll-behaviour, animation-clock and `requestAnimationFrame` items) · §Surface: desktop-native (surface.md: the JS viewport, titlebar and window-title items; design-system.md body: Toolkit / Framework) · §Typography (typography.md: the `document.fonts` item)
**Change:**
- Re-pointed to blitz-vibey-script's split modules, claim text unchanged: scroll-behaviour parsing was `dom/element.rs`, now `dom/element/scrolling.rs`; `requestAnimationFrame` was `runtime.rs`, now `runtime/timers.rs`; the JS viewport sizes and `devicePixelRatio` were `runtime.rs` twice, now `runtime/bootstrap.rs` and `runtime/window.rs`; the `document.fonts` stub was `runtime.rs`, now `runtime/bootstrap/fonts.js`.
- Line ranges moved, path and claim unchanged: blitz-shell's `lib.rs` (the cursor's `None` handling, the titlebar provider), blitz-shell's `window.rs` (the animation clock, the window title, the winit toolkit line) and apps/browser's `tab.rs` (the 16x16 favicon).
**Why:** the take of eleven upstream commits, on the founder's own word «Одиннадцать коммитов (Рекомендую)», 2026-10-27, relayed verbatim by the overseer (inputs#I4): it split blitz-vibey-script's runtime and element bindings into the `runtime/` and `dom/element/` trees, a move removing nothing (`runtime.rs` keeps 151 lines), so the old paths read claim-false while each claim holds at its new site; and it added lines to blitz-shell's `lib.rs` and `window.rs` (the redraw route through the event loop) and to `tab.rs` (tab lenses surviving a props replacement).
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the flight-type select overrides and the load page's group leave design-system
**Section:** §Motion (motion.md: the `values · expression-motion` list) · §Surface: desktop-native (surface.md: Tokens; the `values · desktop-native-stand-load-page` group) · the subjects line (design-system.md)
**Change:**
- Motion: the stand load page's item (`LoadNet` answering its one stylesheet after `LOAD_ANSWER_DELAY`, 200 ms, from a thread of its own; `load-done` `display: none` until it lands, so a `click` on `load-fetch` waits it out) removed; the driver's load-wait item above it stands.
- Surface Tokens: the flight booker's `select#flight-type` overrides item (15px, `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10, on `:focus` a `#3a5ce5` border, its two options) removed; the engine's select-defaults item stands.
- Surface: the `desktop-native-stand-load-page` group (`boot_load` in a TaskShell titled "Load", keyed `load-fetch` and `load-done`, classes `load-root` · `load-body` · `load-btn` · `load-done`) removed whole with its markers.
- Subjects line: drops `desktop-native-stand-load-page`, every other subject in its order.
- The load-page facts fold into layout-templates' served-screens bullet and the select's into its mode-region bullet, values unchanged. Was kept, per «2026-10-27-readme-and-records — the stand's colours leave the product surface, PROVISIONAL» (the select overrides, the load-page group, the subject); now gone from design-system.
**Why:** the founder's own word «Все десять, общие разделить (Рекомендую)», by phone, 2026-10-27, relayed for this wrap by the operator (inputs#I7): the stand-only lines go to layout-templates' desktop-native screens with values unchanged, the subjects line dropping the emptied group.
**Kept:** the stand facts outside the ten — the five pointer rows to layout-templates (border radius, colour palette, depth, spacing, typography), surface's cli item naming the six served tasks, motion's observed absence of transitions in seven_guis styles and the driver's load-wait items in motion and the body — stay, settled, on the founder's own word «Оставить как есть (Рекомендую)», by phone, 2026-10-27, relayed by the operator at this wrap's P2 Validate (inputs#I8).
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/
