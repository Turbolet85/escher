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
