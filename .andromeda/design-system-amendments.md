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

## 2026-10-13-stand-contrast-harness — the seven_guis accent measured under white button text
**Section:** §Color Palette → Core Colors (seven_guis accent row)
**Change:** the row adds the measured reading: `#4a6cf7` is the background of white `#ffffff` button text on the five accent controls (`counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create`; 14–16 px, weight 500–600, at boot), 4.39 against SC 1.4.3's 4.5 — a fail; every measured stand colour equals its declared value. The declared accent, hover and active values are unchanged.
**Why:** the stand contrast check measured the declared pairs; the palette states what its accent reads under the text the stand paints on it. Re-colouring is owned by the route entry "Stand a11y assertions", not by this row.
**Ref:** .andromeda/runs/2026-10-13T18-00-36-wrap/

## 2026-10-14-stand-a11y-assertions — the stand's accent one step darker, SC 1.4.3 met
**Section:** §Color Palette → Core Colors (the `seven_guis accent` row)
**Change:** the accent is `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1` in every task CSS that declares it, its non-text uses moving with it and its three `rgba` uses at `rgba(58, 92, 229, …)`, alpha kept; was `#4a6cf7` / `#3a5ce5` / `#2a4cd3`. White button text on the five accent controls reads 5.47, a pass (was 4.39, a fail); on the hover shade 6.83 and on the active shade 8.27, reached states of `stand_contrast`.
**Why:** the operator's F5, "One step darker, stand-wide", by question dialog — a stand change and no engine change, the hex literals staying where each task declares them (the token set is NOT YET MEASURED).
**Ref:** .andromeda/runs/2026-10-14T19-50-50-wrap/

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
