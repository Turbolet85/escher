## 2026-10-27-readme-and-records — the README's pinned line and the paint-order citation re-pointed
**Section:** §Established Decisions (established-decisions.md: [Layout]) · §Project Intent (project-intent.md: Front page, Capability statement) · §Standard Contracts (standard-contracts.md: CI contracts → the capability-statement pin)
**Change:**
- [Layout]: the citation of the z-index-auto paint level and the hoisted stacking contexts, was `paint_tree.rs:420-462`; now `:420-461`.
- Three `README.md` citations re-pointed, was `README.md:48`; now `README.md:49` — the Front page's and the Capability statement's link to `docs/capabilities.md`, and the capability-statement pin's README phrase.
- No claim text changed by the re-points.
**Why:** the paint-tree file holds 461 lines, `position_to_order` and `node_to_paint_order` ending at 461 — a citation held unchanged since an earlier sweep, re-pointed by hand at this pass's citation sweep; the chunk's new README text moved the pinned phrase «the twelve driver commands as tools» and the `docs/capabilities.md` link from line 48 to line 49.
**Ref:** .andromeda/runs/2026-10-27T05-59-26-wrap/

## 2026-10-27-readme-and-records — the front page: 0.2.0 done and verified, 0.3.0 planned
**Section:** §Project Intent (project-intent.md: the Front page clause; two new `values · intent` detail blocks after it — the «Since 0.2.0» list with the «Later:» line, and the 0.2.0 figures)
**Change:**
- Front page: was «what escher is and … its state under `## Status`: 0.1.0 done and verified … and 0.2.0 planned in five stages, one line per epoch of `escher-0.2.0/working-route.md`, where it read built vs planned»; now what escher is, naming 0.2.0's additions (a large screen in parts, a `select`, `lint` and its six defects, the one virtual clock, the bounded load wait, a session recorded as a check that replays), and under `## Status` 0.1.0 done and verified with its three facts as they stood, 0.2.0 done and verified with its five stages and three recorded facts, and 0.3.0 planned in six stages, one line per epoch of `escher-0.3.0/working-route.md`, naming no marker.
- The command-line clause adds the six served task slugs `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` and a Try it line `target/debug/escher-session lint --session demo-session`; was «the 0.1.0 epochs»; now «the 0.1.0 and 0.2.0 epochs».
- New: after the MCP paragraph, «Since 0.2.0 the driver also does these», six items — `lint`, `snapshot --part`, `type` on a `select`, the acting commands' load wait, `advance` on the one virtual clock, `start --record` — each worded as `docs/capabilities.md` §What it can do states it; after the 0.3.0 stages a «Later:» line restating `escher-0.3.0/intent.md` §Out of this version: opt-in OpenTelemetry export, the platform accessibility adapter witnessed with a real assistive technology, the next MCP protocol revision.
- New: the 0.2.0 section's three figures — 11 of 11 capabilities verified; a fresh agent given only the twelve MCP tools drove six apps (the four lean tasks, Cells, the load page) in 66 calls with 0 wrong, under a bar of at most 5 % wrong; the version-end mutation reading, 53 of 53 caught in the DOM crate, the stand's run unscored with 12 of its 15 mutants unviable, 0 of 5 caught in upstream's WPT runner.
**Why:** the chunk rewrote the README for the new version (v030-02), its 0.2.0 figures read from 0.2.0's own records — its matrix coverage and the cold-agent live verdict. The figures block and the clause's restatement went in together, atomically, on the operator's dialog answer at this wrap's P2 halt (the operator's word, the founder asleep): the block contradicted the clause until both landed.
**Ref:** .andromeda/runs/2026-10-27T05-59-26-wrap/

## 2026-10-27-readme-and-records — the default branch GitHub renders the front page from
**Section:** §Project Intent (project-intent.md: a new `values · intent` detail block after the Front page clause)
**Change:**
- New: the front page GitHub renders is the root `README.md` of the repository's default branch, `build/escher-0.3.0`, whose README reads the three status headings — read by `gh repo view Turbolet85/escher --json defaultBranchRef` and `gh api repos/Turbolet85/escher/readme`.
**Why:** the report's Addendum: the default branch was switched to `build/escher-0.3.0` on the founder's own word (2026-10-27, relayed by overseer1, who ran the switch on his further word), read back by the escher overseer at 06:16:24Z with the 0.1.0, 0.2.0 and 0.3.0 status headings. Until then it was `build/escher-0.1.0`, whose README held no `## Status`, so the front page did not show a build branch's README edit; the plan's expected fact that the page renders `build/escher-0.1.0`, gated at phase on his word, is superseded by the switch.
**Ref:** .andromeda/runs/2026-10-27T05-59-26-wrap/

## 2026-10-27-readme-and-records — the stand's typography in the Headless stand contract (PROVISIONAL)
**Section:** §Standard Contracts (standard-contracts.md: Headless stand, a new `values · contract-headless-stand` detail block)
**Change:**
- New: the stand's typography is DejaVu Sans, its one bundled font, stated in this contract and not on design-system's product surface — `font_ctx()` registers it for every generic family with system fonts off, on native; every stand boot holds it through `HarnessOptions.font_ctx`, `boot_cells` and `boot_load` included; seven_guis' native `woff` feature decodes it.
- New: an OPEN line — the typography stated here rather than on design-system's product surface, owned by F2, the stand rows' move off design-system's product surface.
**Why:** the stand's DejaVu Sans row left design-system §Typography in this pass under F2 (every stand row moves), a delegate answer given in the founder's stead by the operator's question dialog at phase P4 (2026-10-27) — PROVISIONAL until the founder's own word, which supersedes it by rule; the font's values unchanged by the move.
**Ref:** .andromeda/runs/2026-10-27T05-59-26-wrap/
