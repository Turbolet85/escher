# Apply — architecture

## Verdicts
- fanout-architecture--cascade.md 1 · applied · .andromeda/architecture-detail/project-intent.md:21#1, .andromeda/architecture-detail/project-intent.md:21#2, .andromeda/architecture-detail/project-intent.md:21#3
- fanout-architecture--cascade.md 2 · applied · .andromeda/architecture-detail/project-intent.md:23+#1
- fanout-architecture--cascade.md 3 · applied · .andromeda/architecture-detail/project-intent.md:23+#2
- fanout-architecture--cascade.md 4 · applied · .andromeda/architecture-detail/project-intent.md:23+#3
- fanout-architecture--cascade.md 5 · applied · .andromeda/architecture-detail/standard-contracts.md:123+#1

## Edits
- line: .andromeda/architecture-detail/project-intent.md:21
  old: «what escher is and, since 2026-10-16-handler-splits-and-readme, its state under `## Status`: 0.1.0 done and verified, its six stages listed with three recorded facts (17 capabilities verified, 43 cold-agent tool calls with 0 wrong, 0 surviving mutants of 1,558), and 0.2.0 planned in five stages, one line per epoch of `escher-0.2.0/working-route.md`, where it read built vs planned;»
  new: «what escher is, naming 0.2.0's additions (a large screen in parts, a `select`, `lint` and its six defects, the one virtual clock, the bounded load wait, a session recorded as a check that replays), and, since 2026-10-16-handler-splits-and-readme, its state under `## Status`: 0.1.0 done and verified, its six stages listed with three recorded facts (17 capabilities verified, 43 cold-agent tool calls with 0 wrong, 0 surviving mutants of 1,558), 0.2.0 done and verified, its five stages listed with three recorded facts (below), and 0.3.0 planned in six stages, one line per epoch of `escher-0.3.0/working-route.md`, naming no marker;»
- line: .andromeda/architecture-detail/project-intent.md:21
  old: «the four exit statuses, `stop`)»
  new: «the four exit statuses, `stop`; the six served task slugs `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`, and a Try it line `target/debug/escher-session lint --session demo-session`)»
- line: .andromeda/architecture-detail/project-intent.md:21
  old: «its Blitz lineage, the 0.1.0 epochs, the licence»
  new: «its Blitz lineage, the 0.1.0 and 0.2.0 epochs, the licence»
- after: .andromeda/architecture-detail/project-intent.md:23
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · intent -->»
  add: «»
  add: «- The front page GitHub renders is the root `README.md` of the repository's default branch, `build/escher-0.3.0`, whose README reads the three status headings (`gh repo view Turbolet85/escher --json defaultBranchRef`; `gh api repos/Turbolet85/escher/readme` — as measured at escher-0.3.0/chunks/2026-10-27-readme-and-records/report.md §Addendum)»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/project-intent.md:23
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · intent -->»
  add: «»
  add: «- After the MCP paragraph the front page lists, under «Since 0.2.0 the driver also does these», six things the driver does — `lint`, `snapshot --part`, `type` on a `select`, the acting commands' load wait, `advance` on the one virtual clock and `start --record` — each worded as `docs/capabilities.md` §What it can do states it; after the 0.3.0 stages a «Later:» line restates `escher-0.3.0/intent.md` §Out of this version: opt-in OpenTelemetry export, the platform accessibility adapter witnessed with a real assistive technology, and the next MCP protocol revision (README.md:51-58; README.md:96)»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/project-intent.md:23
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · intent -->»
  add: «»
  add: «- The front page's 0.2.0 section gives three figures read from its records: 11 of 11 capabilities verified; a fresh agent given only the twelve MCP tools drove six apps — the four lean tasks, Cells and the load page — in 66 calls with 0 wrong, under a bar of at most 5 % wrong; and the version-end mutation reading, 53 of 53 caught in the DOM crate, the stand's run unscored with 12 of its 15 mutants unviable, and 0 of 5 caught in upstream's WPT runner (README.md:85 — as measured at escher-0.3.0/chunks/2026-10-27-readme-and-records/report.md, from `matrix.py coverage --dir escher-0.2.0` and `live-verdict.json`)»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/standard-contracts.md:123
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · contract-headless-stand -->»
  add: «»
  add: «- The stand's typography is DejaVu Sans, its one bundled font, stated in this contract and not on design-system's product surface: `font_ctx()` registers it for every generic family with system fonts off, on native; every stand boot holds it through `HarnessOptions.font_ctx`, `boot_cells` and `boot_load` included; and seven_guis' native `woff` feature decodes it (examples/seven_guis/src/stand.rs:59-61; examples/seven_guis/src/stand.rs:73; examples/seven_guis/src/lib.rs:7; examples/seven_guis/Cargo.toml:31).»
  add: «- OPEN — the stand's typography stated here rather than on design-system's product surface · owner: F2, the stand rows' move off design-system's product surface»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "arch-01"
fixed = '0.2.0 planned in five stages'
