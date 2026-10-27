proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/project-intent.md
    near: 21
    section: "## Project Intent"
    change: "Front page clause: «what escher is» becomes «what Escher is, naming 0.2.0's additions (a large screen in parts, a `select`, `lint` and its six defects, the one virtual clock, the bounded load wait, a session recorded as a check that replays)»; «and 0.2.0 planned in five stages, one line per epoch of `escher-0.2.0/working-route.md`, where it read built vs planned» becomes «0.2.0 done and verified since 2026-10-27-readme-and-records, its five stages listed with its recorded facts, and 0.3.0 planned in six stages, one line per epoch of `escher-0.3.0/working-route.md`, naming no marker»; the driver's command-line clause adds that the page names the six served task slugs (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`) and its Try it lines include `target/debug/escher-session lint --session demo-session`; «the 0.1.0 epochs» becomes «the 0.1.0 and 0.2.0 epochs»; the 0.1.0 facts, the «Escher» spelling, the lineage, licence, Author / Contact, the badge and the reader sentence unchanged"
    sidecar: "2026-10-27-readme-and-records: Front page status restated — 0.1.0 and 0.2.0 done and verified, 0.3.0 planned in six stages naming no marker; the page's 0.2.0 additions, six task slugs and lint Try-it line recorded"
    rationale: "Report Changes → Files (README.md): `## Status` replaces `### 0.2.0 — planned, in five stages` with `### 0.2.0 — done and verified` and adds `### 0.3.0 — planned, in six stages`; What Escher is names 0.2.0's additions; the driver paragraph names the six slugs and Try it gains the lint line; Counts pattern `readme-status` 1 to change at this clause; Expected amendments 1"
    basis: "escher-0.3.0/chunks/2026-10-27-readme-and-records/report.md:14"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/project-intent.md
    near: 21
    section: "## Project Intent"
    change: "Front page clause, after «the root `README.md` is escher's own page, not upstream Blitz's» add «— the page GitHub renders for the repository from its default branch, `build/escher-0.3.0` since the founder's switch on his word «Да, переключить» (2026-10-27), read back at 06:16:24Z by `gh repo view Turbolet85/escher --json defaultBranchRef` and `gh api repos/Turbolet85/escher/readme` (the README's three status headings); until then the default branch was `build/escher-0.1.0`, whose README held no `## Status`, so a build branch's README edit did not show there»"
    sidecar: "2026-10-27-readme-and-records: Front page states the default branch GitHub renders — build/escher-0.3.0 since the founder's switch («Да, переключить», read back 06:16:24Z), build/escher-0.1.0 before"
    rationale: "Report Spec claims disproved 2 (pattern `front-page`, 1 to change at this clause) and Expected amendments 1, as superseded by the Addendum (inputs#I6): the default branch is `build/escher-0.3.0` since the founder's switch, read back at 06:16:24Z — the task's F1 wording (`build/escher-0.1.0`, the switch not done) is the pre-addendum reading, kept here only as the before"
    basis: "escher-0.3.0/chunks/2026-10-27-readme-and-records/report.md:321"
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/project-intent.md
    near: 21
    section: "## Project Intent"
    change: "Front page clause, after the MCP clause add «since 2026-10-27-readme-and-records a six-item «Since 0.2.0 the driver also does these» list after the MCP paragraph, each item worded as `docs/capabilities.md` §What it can do states it, and a «Later:» line restating `escher-0.3.0/intent.md` §Out of this version — opt-in OpenTelemetry export, the platform accessibility adapter witnessed with a real assistive technology, the next MCP protocol revision»"
    sidecar: "2026-10-27-readme-and-records: Front page records the «Since 0.2.0» list (worded from the capability statement) and the Later line (from 0.3.0's out-of-version list)"
    rationale: "Report Changes → Files (README.md): a six-item «Since 0.2.0 the driver also does these» list follows the MCP paragraph, each item as `docs/capabilities.md` §What it can do states it; «Later:» restated from `escher-0.3.0/intent.md` §Out of this version"
    basis: "escher-0.3.0/chunks/2026-10-27-readme-and-records/report.md:18"
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/project-intent.md
    near: 21
    section: "## Project Intent"
    change: "Front page clause, beside the 0.1.0 facts add «0.2.0's section gives figures read from its records: 11 of 11 capabilities verified; a fresh agent on the twelve MCP tools drove six apps in 66 calls with 0 wrong under a bar of at most 5 % wrong; and the version-end mutation reading, 53 of 53 caught in the DOM crate, the stand's run unscored with 12 of 15 unviable, 0 of 5 caught in upstream's WPT runner»"
    sidecar: "2026-10-27-readme-and-records: Front page records 0.2.0's figures — 11/11 capabilities, 66 calls 0 wrong under the 5 % bar, the version-end mutation reading"
    rationale: "Report Changes → Files (README.md): `### 0.2.0 — done and verified` gives five epoch lines, then 11 of 11 capabilities, 66 calls with 0 wrong under at most 5 %, and the mutation reading; Outcome v030-02 reads them from `matrix.py coverage` and `live-verdict.json`"
    basis: "escher-0.3.0/chunks/2026-10-27-readme-and-records/report.md:20"
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 120
    section: "## Standard Contracts"
    change: "Headless stand contract, after «`font_ctx()` = `build_single_font_ctx(DEJAVU_SANS)`, system fonts off» add «— DejaVu Sans, registered as the stand's one bundled font, is the stand's typography, stated in this contract and not on design-system's product surface: it moved here from design-system §Typography's stand row at 2026-10-27-readme-and-records on F2 «Every stand row (Recommended)», the founder's delegate answer, PROVISIONAL until the founder's own word»"
    sidecar: "2026-10-27-readme-and-records: Headless stand receives the stand's DejaVu Sans typography row from design-system §Typography — F2, PROVISIONAL until the founder's own word"
    rationale: "Report Expected amendments 4 (design-system row 6 lands beside `font_ctx()`, F2 PROVISIONAL) and Decisions & corrections F2 (every sidecar entry carrying it reads PROVISIONAL); the body's Deployment model line and Headless stand broad line already agree (seven_guis' `DEJAVU_SANS`, one bundled font), so no body edit"
    basis: "escher-0.3.0/chunks/2026-10-27-readme-and-records/report.md:178"
    fact: F4
note: "The task's F1 (default branch `build/escher-0.1.0`, switch not done) is superseded by the report's Addendum (inputs#I6): the F1 proposal states `build/escher-0.3.0` since the founder's switch; the README.md:49 citations at project-intent 21 and 27 and standard-contracts 279 are the citation sweep's and drew no proposal; no new resource, dependency or decision, so D-arch-resources and D-arch-decisions fire nothing."
