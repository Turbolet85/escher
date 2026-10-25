proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 54
    section: "## Conventions"
    change: "In the session_common sentence, after «since 2026-10-14-stand-requirement-sweep, the driver sweep `stand_requirement_sweep`», add «and, since 2026-10-25-cold-agent-bar, the two served-page checks `stand_cells_served` (Cells walked in parts through the driver) and `stand_load_page` (the load page's click waiting out its load)», each holding a `Session` and reading `mod session_common;` like the rest, citing (tests/blitz-tests/tests/stand_cells_served.rs:19-21; tests/blitz-tests/tests/stand_load_page.rs:19-23)"
    sidecar: "conv-tests: the session_common readers gain stand_cells_served and stand_load_page (2026-10-25-cold-agent-bar)"
    rationale: "The report's Changes list the two new files (Files → new; Symbols → stand_requirement_sweep's Subject::Served witnesses) and its New text shows each holding a Session through session_common — stand_cells_served.rs:19-21 «use session_common::{», :33-37 `Session::start(\"cells\", …)`; stand_load_page.rs:19-23 `Session::start(\"load\", …)`, :25-32 `session.run(&snapshot())` — so the conv-tests enumeration of session-holding checks that share session_common gains two members"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:13-14
    fact: F1
note: D-arch-resources and D-arch-decisions read clean for this category — no new library, crate, env var, socket or port (report Dependencies none, Crates none; LoadNet uses blitz-traits); no Retired item (flight-booker mode buttons, .type-btn CSS) and no moved fact or citation row (F1-F21) lands in conventions.md; its citations of tests/blitz-tests/tests/common/mod.rs:1-6 and session_common/mod.rs:1-7 sit above this chunk's added lines (25+, 305+) and stand; the line-54 enumeration was already partial before this chunk (later session-holding checks such as stand_lint, stand_replay, stand_load_wait are not named), which the orchestrator may prefer to settle by a generic wording instead.
