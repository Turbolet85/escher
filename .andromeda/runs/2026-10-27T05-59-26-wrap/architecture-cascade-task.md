# Task — architecture · 2026-10-27-readme-and-records

## The session
A documents-and-records chunk (v030-02 · v030-16): no source, test, CI script, manifest, lock file, crate, dependency or schema changed, and no `.andromeda/` master, category file or sidecar was edited by the chunk itself.
`README.md` changed: "What Escher is" names 0.2.0's additions; the driver paragraph names the six task slugs (`counter`, `flight-booker`, `timer`, `crud`, `cells`, `load`); "Try it" gains an `escher-session lint` line; a six-item «Since 0.2.0 the driver also does these» list follows the MCP paragraph; `## Status` now reads 0.1.0 done and verified, 0.2.0 done and verified (five epoch lines, 11 of 11 capabilities, 66 calls with 0 wrong under a 5 % bar, the version-end mutation reading), and 0.3.0 planned in six stages (one line per epoch of `escher-0.3.0/working-route.md`), and a "Later:" line restates `escher-0.3.0/intent.md` §Out of this version.
`docs/capabilities.md` restated in three places: the lint at boot (the flight booker reads one `overlap` of its `flight-type` select's two options), the host-sink part read (measured: 0 id and 0 name needles under both builds), and the run paragraph (the 2026-10-25 cold-agent live run: six apps, twelve tools, 66 calls, 0 wrong, client `claude` 2.1.308).
Measured: the GitHub front page renders the default branch `build/escher-0.1.0`, whose README has no `## Status`, so the README edit does not show there; the default-branch change is gated on the founder's own word (F3), and v030-02 carries a `gated` key and the working entry a `BLOCKED-ON`.
Retired: the local `.git/info/exclude` incubator lines; `.gitignore:39` (`escher-0.3.0-incubator/`) ignores the incubator, unchanged. Expected P2 amendments: the design-system's nine seven_guis stand rows leave the product surface (F2, PROVISIONAL), one of them, the stand's DejaVu Sans, landing in architecture §Standard Contracts → Headless stand.

## Reached files
- README.md
- docs/capabilities.md
- escher-0.3.0/verification-matrix.json
- escher-0.3.0/working-route.md
- .andromeda/master-route.md
- .andromeda/friction-log.ndjson
- escher-0.3.0/chunks/2026-10-27-readme-and-records/scope.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/research.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/plan.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/inputs/andromeda-inputs.json
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/records.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/counts.md
- escher-0.3.0/chunks/2026-10-27-readme-and-records/evidence/operator-pass.md
- .andromeda/runs/2026-10-27T03-47-30-phase/p5-controls.sh
- .andromeda/runs/2026-10-27T05-19-41-implement/
- .andromeda/runs/2026-10-27T05-59-26-wrap/
- .git/info/exclude

## Touches
- Project Intent → Front page clause (`project-intent.md:21`): its `## Status` clause says 0.2.0 planned in five stages, one line per epoch of `escher-0.2.0/working-route.md`; it now must say 0.1.0 and 0.2.0 done and verified and 0.3.0 planned in six stages, one line per epoch of `escher-0.3.0/working-route.md`, naming no marker.
- Front page clause, its list of what the page holds: "the 0.1.0 epochs" and the driver paragraph's contents move with the status clause. The paragraph now names six task slugs and a `lint` Try-it line, and the page adds the «Since 0.2.0» list and the "Later:" line.
- Front page clause: the premise «the root `README.md` is escher's own page» names no branch, but the page GitHub renders is the default branch `build/escher-0.1.0`'s README, so a README edit on a build branch does not reach it.
- Citations of `README.md:48`, the pinned phrase «the twelve driver commands as tools» and the `docs/capabilities.md` link, are now `README.md:49`, at `project-intent.md:21`, `project-intent.md:27` and `standard-contracts.md:279` (CI contracts, the capability-statement pin).
- The README's count of the driver commands as tools re-read and unmoved: «twelve» stands. The capability statement's five counts are also unmoved, and the pin `test_capability_statement.py` passes its 4 tests.
- Front page clause «no build or doc step reads it, one test does» re-read and unmoved: the capability-statement pin is still the one reader.
- The body's Front page pointer line (`architecture.md:293`) re-read: a pointer to the detail clause, carrying no status or branch wording of its own.
- Standard Contracts → Headless stand (`standard-contracts.md:120-121`, `font_ctx()` = `build_single_font_ctx(DEJAVU_SANS)`): it receives design-system §Typography's stand row 6, DejaVu Sans as the stand's font, beside `font_ctx()`. This is F2, PROVISIONAL, and the sidecar entry reads PROVISIONAL.
- Infrastructure Patterns → Deployment model: the line that names seven_guis' `DEJAVU_SANS` and the headless stand registering the same font is re-read against the same DejaVu move, so the stand's font is stated once and agrees with the Headless stand contract.
- The host-sink part read: the capability statement went from «unmeasured» to measured. Architecture's own sites (`occupied-resources.md:56`, `existing-scopes.md:111`) already state the measured reading (the host's `host_log`, 0 id and 0 name needles under both builds), so they read no-change.
- The lint at boot: the statement now says the flight booker reads one `overlap` (`flight-type-one-way` × `flight-type-return`). Any architecture text on the lint's stand readings must agree, and must not say each lean task reads 0 findings.
- The capability statement's measured cold-agent run moved from 2026-10-15 to 2026-10-25. Architecture holds no site that names the statement's run (the `cap-run` control did not fire). The occupied-resources outbound-hosts line, on the model provider reached on the operator-driven live run, still holds.
- Infrastructure Patterns → Directory structure `docs/` line: `capabilities.md` was edited in place and keeps its role as escher's capability statement beside upstream's three files.
- CI contracts: the CI scripts' test count read `Ran 108 tests` at implement and the final HEAD's CI run read 17/17 checks green. No CI leg, step or verdict shape moved.
- The incubator: the local exclude was retired and `.gitignore:39` holds, unchanged. No architecture site describes the incubator or `.git/info/exclude` (the control did not fire), so a classifier should find no site to change.

## New facts
- F1 · project-intent · The repository's default branch is `build/escher-0.1.0`, so the GitHub front page renders that branch's README, which has no `## Status`, not a build branch's. Changing the default branch is gated on the founder's own word (F3) and was not done.
- F2 · project-intent · The README's front page carries a six-item «Since 0.2.0 the driver also does these» list, each item worded as `docs/capabilities.md` §What it can do states it, and a "Later:" line restating `escher-0.3.0/intent.md` §Out of this version: opt-in OpenTelemetry export, the platform accessibility adapter witnessed with a real assistive technology, and the next MCP protocol revision.
- F3 · project-intent · The README's 0.2.0 status gives figures read from records: 11 of 11 capabilities verified; a fresh agent on the twelve MCP tools drove six apps in 66 calls with 0 wrong under a bar of at most 5 % wrong; and the version-end mutation reading, 53 of 53 caught in the DOM crate, the stand's run unscored with 12 of 15 unviable, and 0 of 5 caught in upstream's WPT runner.
- F4 · standard-contracts · The stand's typography row (DejaVu Sans registered as the stand's one bundled font) is stated in the Headless stand contract, not on design-system's product surface. It moved there from design-system §Typography on F2, PROVISIONAL until the founder's own word.

## Untouched
- Document core, events, scrolling, layout, nodes and widgets, provider traits, HTML and networking crates, paint and shell: no source or symbol changed.
- Driver session contract and dec-driver-session: twelve verbs, eight causes, wire `v3`, the recording, `Session::lint` and `LOAD_WAIT` unmoved. The README and the statement restate them; they define none.
- Dioxus DOM bridge, the snapshot, its text form and diff, `is_masked_input`: unmoved.
- Telemetry bootstrap and Logging and timing: no sink, target or field changed. The part-sink reading was already measured in architecture.
- Occupied Resources' listeners, environment variables, names and CI infrastructure: no resource added or retired.
- Stack and Technologies, Inherited Defaults, dec-dependency-pinning: no dependency, version or pin moved.
- The Upstream sync line and its merge base: no merge, cherry-pick or rebase in this chunk.
- Conventions (tests, fixtures, formatting, feature gating): no test or fixture changed.
- Existing Scopes' quotes of the stand accent `#3a5ce5` and the select's `#1a1a1a` on `#ffffff` text pair (`existing-scopes.md:113`), and the contract quote at `standard-contracts.md:128`: these state the accent or select as a contract or a test condition. They stay no-change; the design-system move takes design-system rows only.
- The CI/CD jobs, legs and cache rules in Infrastructure Patterns: no workflow or leg script changed.
- Cross-cutting Patterns (config, service wiring, event processing, invalidation): no engine or shell behaviour changed.
- Design Philosophy: no principle moved.
