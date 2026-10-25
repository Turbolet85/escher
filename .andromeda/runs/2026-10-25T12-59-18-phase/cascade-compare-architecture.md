# Cascade compare — architecture · phase · 2026-10-25-cold-agent-bar
E items 162 · held 51 · lost at classify 6 · lost at strong 0 · beside a placed fact - · cascade only 25 · body 36 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### conventions#2.1 · lost at classify · .andromeda/architecture-detail/conventions.md:15
- (.andromeda/architecture-detail/conventions.md:15) Any dependency the extended stand needs (a provider crate for a load in flight, say) must be declared once in the workspace table and consumed as `{ workspace = true }`, and the `seven_guis` path entry must keep its renderer feature.
  > dependencies declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }` (Cargo.toml:43; Cargo.toml:270-306; packages/blitz-traits/Cargo.toml:14-22; packages/blitz/Cargo.toml:24-36; packages/dioxus-native-dom/Cargo.toml:29-47; packages/dioxus-native/Cargo.toml:73-119; packages/stylo_taffy/Cargo.toml:15-18), except the WPT runner's direct `mimalloc = "0.1.48"` (wpt/runner/Cargo.toml:15); in-repo crates declared with `default-features = false` (Cargo.toml:47-58), except the `seven_guis` path entry, which keeps its default `hybrid` renderer feature because `dioxus-native` raises `compile_error!` with no renderer (Cargo.toml:62; packages/dioxus-native/src/dioxus_renderer.rs:27)

### design-philosophy#1.1 · lost at classify · .andromeda/architecture-detail/design-philosophy.md:11
- (.andromeda/architecture-detail/design-philosophy.md:11) The stand extension must be external code that drives the headless DOM through its `DioxusDocument` wrapper. Networking stays in its own crate, so the stand's load does not move networking into blitz-dom or the stand. The select model and the load wait already stand in the engine, so no engine change is made.
  > blitz-dom is a flexible headless DOM (`BaseDocument`) designed to be embedded in and "driven" by external code, with most users expected to use a wrapper (`HtmlDocument` from blitz-html, `DioxusDocument` from dioxus-native) (packages/blitz-dom/src/lib.rs:3-10); it holds the DOM tree, CSS parsing and resolution, layout and event handling, while html parsing (blitz-html), networking (blitz-net), rendering (blitz-paint) and windowing (blitz-shell) live in separate crates (packages/blitz-dom/src/lib.rs:12-14)

### inherited-defaults#1.1 · lost at classify · .andromeda/architecture-detail/inherited-defaults.md:8
- (.andromeda/architecture-detail/inherited-defaults.md:8) A load in flight on the stand may add a stand-in or a provider; whatever it adds lives in a crate that stays `publish = false` (the seven_guis example, blitz-tests or escher-driver), never in a published `packages/` library crate's version set.
  > - Publishability: the root package is virtual and not meant to be published (Cargo.toml:252-263); app and example crates, blitz-tests, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script (since 2026-10-13-upstream-sync-polish-and-ship) and wpt set `publish = false`

### project-intent#3.1 · lost at classify · .andromeda/architecture-detail/project-intent.md:21
- (.andromeda/architecture-detail/project-intent.md:21) The wrong-call bar the founder sets at P4 must be read against the baseline the front page records for 0.1.0, 43 cold-agent tool calls with 0 wrong. Any change to the page's status facts is the wrap's amendment, not a phase edit.
  > its six stages listed with three recorded facts (17 capabilities verified, 43 cold-agent tool calls with 0 wrong, 0 surviving mutants of 1,558)

### project-intent#3.1 · lost at classify · .andromeda/architecture-detail/project-intent.md:21
- (.andromeda/architecture-detail/project-intent.md:21) The README's tool count must stay true, so the chunk adds no driver verb and the cold agent sees exactly the twelve tools that `VERBS` holds. One test reads that count, and it must stay green.
  > the page saying twelve, the count `VERBS` holds), since 2026-10-12-self-description how to ask for `help` on both surfaces, a link to `docs/capabilities.md` as the statement of what Escher can do and its limits (README.md:48), the screenshot no longer named as still to come since 2026-10-13-upstream-sync-polish-and-ship, its Blitz lineage, the 0.1.0 epochs, the licence and an Author / Contact section, with the fork's `ci.yml` badge as its one image (README.md). No build or doc step reads it, and one test does — the capability-statement pin reads its count of the driver commands as tools (§Standard Contracts → CI contracts)

### project-intent#4.1 · lost at classify · .andromeda/architecture-detail/project-intent.md:27
- (.andromeda/architecture-detail/project-intent.md:27) The capability-statement pin must stay green. The chunk adds no verb, refusal cause, session error, node key or exit status, so the five pinned counts and names do not move. The statement's other claims are held by hand: the wrap re-reads the line about the earlier live run, and the phase does not edit it.
  > the CI-script pin `.github/scripts/test_capability_statement.py` holds its verb and cause names and its five counts — verbs, refusal causes, session errors, node keys, exit statuses — to the driver schema's tables (§Standard Contracts → CI contracts), and no check reads its other claims

### stack-and-technologies#1.11 · lost at classify · .andromeda/architecture-detail/stack-and-technologies.md:39
- (.andromeda/architecture-detail/stack-and-technologies.md:39) However the stand carries a load in flight (the P4 fork), it must add no networking crate or version beyond the workspace's pinned networking layer, which belongs to blitz-net.
  > url "2.5.0", http "1.1.0", data-url "0.3.1", tokio "1.42" (resolved 1.54.1, past RUSTSEC-2026-0386), reqwest "0.13", reqwest-middleware "0.5.1", http-cache-reqwest and http-cache "=1.0.0-alpha.6" (Cargo.toml:148-156); blitz-net uses reqwest with `charset`, `native-tls`, `form` (packages/blitz-net/Cargo.toml:20-27)

## Held

- conventions#1.1
- conventions#10.1
- conventions#8.1
- conventions#9.2
- cross-cutting-patterns#1.1
- cross-cutting-patterns#2.1
- cross-cutting-patterns#3.1
- cross-cutting-patterns#4.1
- cross-cutting-patterns#5.1
- design-philosophy#2.1
- design-philosophy#4.1
- design-philosophy#5.1
- established-decisions#10.1
- established-decisions#11.2
- established-decisions#11.3
- established-decisions#14.1
- established-decisions#15.3
- established-decisions#24.1
- established-decisions#25.1
- established-decisions#7.1
- existing-scopes#1.26
- existing-scopes#1.31
- existing-scopes#1.32
- existing-scopes#2.2
- inherited-defaults#2.1
- inherited-defaults#3.1
- occupied-resources#1.1
- occupied-resources#2.1
- occupied-resources#3.1
- occupied-resources#4.1
- occupied-resources#4.2
- occupied-resources#5.1
- occupied-resources#8.1
- stack-and-technologies#1.1
- stack-and-technologies#1.13
- stack-and-technologies#1.14
- stack-and-technologies#1.20
- stack-and-technologies#1.22
- stack-and-technologies#1.23
- standard-contracts#10.1
- standard-contracts#13.1
- standard-contracts#15.1
- standard-contracts#17.1
- standard-contracts#18.1
- standard-contracts#18.2
- standard-contracts#18.4
- standard-contracts#21.1
- standard-contracts#22.2
- standard-contracts#23.1
- standard-contracts#26.1
- standard-contracts#4.1

## Cascade only

- conventions#3.1
- conventions#5.1
- established-decisions#15.1
- established-decisions#15.2
- established-decisions#21.1
- established-decisions#5.1
- established-decisions#6.1
- existing-scopes#1.23
- existing-scopes#1.24
- existing-scopes#1.25
- existing-scopes#1.3
- occupied-resources#7.1
- stack-and-technologies#1.9
- standard-contracts#1.1
- standard-contracts#11.1
- standard-contracts#14.1
- standard-contracts#16.1
- standard-contracts#18.3
- standard-contracts#2.1
- standard-contracts#2.2
- standard-contracts#3.1
- standard-contracts#5.1
- standard-contracts#6.2
- standard-contracts#9.1
- standard-contracts#9.2
