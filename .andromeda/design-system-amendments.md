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
