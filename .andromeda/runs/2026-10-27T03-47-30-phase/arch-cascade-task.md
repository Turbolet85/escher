# Task — arch · 2026-10-27-readme-and-records

## The chunk
Rewrites the root README's status and current-state prose: 0.1.0 and 0.2.0 done and verified (0.2.0's stages, what it can do, its cold-agent result), 0.3.0 planned with its six epochs as stages, the "Later:" line restated to what is past 0.3.0, the name «Escher» capitalised in every prose mention.
Restates the capabilities statement's cold-agent paragraph to the 2026-10-25 live run (six apps, twelve tools, 66 calls, 0 wrong, a screenshot and a lint called) and sweeps the file for any other line naming the 2026-10-15 run as the latest.
Corrects records: two verified ledger refs re-pointed to the tests' current names; the design system's four stand rows (accent, success, cards, the stand's DejaVu Sans) moved out of the product surface into architecture and layout-templates, as an expected wrap amendment; the incubator's local exclude retired in favour of the ignore line already in `.gitignore`.
Changes no code, test, engine, driver, CI or schema; re-reads the capability-statement pin, which stays green.

## Reached files
- README.md · edits §Status, the "Later:" line, "What Escher is", the driver paragraph and "Try it"
- docs/capabilities.md · edits the cold-agent run paragraph (lines 288-291) and any other line naming the 2026-10-15 run as the latest
- escher-0.1.0/verification-matrix.json · edits the v010-17 verified ref
- escher-0.2.0/verification-matrix.json · edits the v020-05 verified ref
- .andromeda/design-system.md · edits the accent row, the success row, the cards item and the stand's DejaVu Sans item (moved out)
- .andromeda/architecture.md · edits the section that receives the moved stand rows (an expected wrap amendment)
- .andromeda/layout-templates.md · edits the section that receives the moved stand rows (an expected wrap amendment)
- .git/info/exclude · edits the two incubator lines at its end
- .gitignore · re-reads the incubator ignore line (line 39) through `git check-ignore -v`
- .github/scripts/test_capability_statement.py · re-reads the capability-statement pin (verb and cause names, five counts, the README's tool count)
- tests/blitz-tests/tests/stand_snapshot_state.rs · re-reads the test name `exactly_the_chosen_flight_type_reads_selected`
- tests/blitz-tests/tests/stand_lint.rs · re-reads the test name `the_stand_at_boot_reads_its_measured_findings`
- escher-0.3.0/working-route.md · re-reads its six epochs, the README's planned stages
- escher-0.3.0/intent.md · re-reads §Out of this version, the source of the "Later:" line
- .andromeda/security-plan.md · re-reads the 2026-10-25 run's recorded crossing to the model provider
- .andromeda/residuals.md · re-reads the two lines the chunk absorbs (lines 41 and 42)

## Needs
- The front page rule in Project Intent: the root README is escher's own page — what it is, its status, how the driver's command line and MCP surface are run, its Blitz lineage, licence and contact, a link to `docs/capabilities.md` — the name spelled «Escher» in prose and title, crate, binary and path names, URLs and the e-mail unchanged.
- The front page's one reader: the capability-statement pin reads the README's count of the driver commands an MCP client gets as tools; the README keeps "the twelve driver commands as tools".
- The front page at an Upstream sync resolves to escher's version — the README this chunk writes is the version a later sync keeps.
- The directory-structure line for `docs/`: `capabilities.md` is escher's capability statement beside upstream's three files; the chunk edits it in place and adds no file there.
- The driver session contract's counts the README and statement follow and do not move: twelve verbs, eight refusal causes, the schema stated once as `'static` data.
- The CI contracts' cold-agent pipe (`scripts/cold-agent.sh`) as the source of the run the statement describes, its contract in full in the test plan.
- The CI-scripts leg that runs the capability-statement pin: the pin must stay green after the README and statement edits.
- Outbound hosts: the model provider is reached by the spawned `claude` client on the operator-driven live cold-agent run only, never in CI — the statement's description of the 2026-10-25 run stays inside that.
- The headless stand contract as a landing site for the moved stand rows: the pinned 800x600 Light viewport and its one bundled font.
- The deployment-model line naming seven_guis' bundled DejaVuSans bytes, the crate-root `DEJAVU_SANS`, registered by the headless stand — a landing site for the stand's DejaVu Sans item.
- Existing Scopes' seven_guis scope as a landing site for the stand's accent and success colours and its cards.
- The README's named 0.2.0 capabilities — the visual lint, a session recorded as a replayable check, a large screen read in parts, a `select` driven by id, the load wait, the virtual clock — stated only as the capabilities statement already states them, no claim beyond a record.
- The recording decision's one exception, the two committed Counter checks under `tests/blitz-tests/checks/`, if the README names the replayable check.
- The `escher-session` command line (six served tasks, its `mcp` role, exits `0` · `1` · `2` · `3`) as the README's "Try it" describes it.
- The incubator ignore: the repository's own ignore file carries the `escher-0.3.0-incubator/` line; the `escher-0.2.0-incubator/` line is not touched.

## Standing conventions
- (none)

## Not needed
- The engine contracts — document core, the document's clock, events, scrolling, layout, nodes and widgets, the select and range-input models, keyboard activation — the README names capabilities by name only and changes no engine behaviour.
- The telemetry bootstrap and the logging and timing pattern: the chunk adds no log line, span or field.
- The driver session's wire, socket, MCP framing and bounds: no carrier, verb, argument or result field changes.
- Network ports and listeners, environment variables and the names registry: no listener, env read, crate or binary is added.
- Dependency pinning, the stack, build profiles, MSRV and default features: no manifest or lockfile changes.
- Release versioning and the workspace version: the README's 0.1.0 · 0.2.0 · 0.3.0 are route versions, and no Cargo version moves.
- The CI infrastructure, its jobs, caches and the stand-repeat leg: no workflow or leg changes.
- The upstream sync's merge base and the waiting Stylo, Taffy and Dioxus versions: the sync is another chunk.
- The conventions on formatting and lints, tests, unit-test placement, apps and examples and HTML fixtures: the chunk writes no code, test or fixture.
- The WPT runner, the browser app, rdme, the JavaScript runtime and the accessibility adapter: none is touched or described anew.
- The Dioxus DOM bridge's stable-id tiers and snapshot internals: the README restates no mechanism.
