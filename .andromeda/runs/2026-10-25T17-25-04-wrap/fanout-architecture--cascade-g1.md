proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 73
    section: "## Stack and Technologies"
    change: "after «and at 2.1.296 (as measured at escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/operator-9-claude-version.txt)» add «and at 2.1.308 on 2026-10-25, the live run's `client_version` the same (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-run.md)»"
    sidecar: "stack-and-technologies: the cold-agent client `claude` read at 2.1.308 on 2026-10-25 beside its earlier readings (2026-10-25-cold-agent-bar)"
    rationale: "The report's Dev-tool versions bullet: `claude` read 2.1.308 on the dev host at 2026-10-25 (operator entry 22, `claude --version`), the live run's `client_version` the same; the gate `claude --version` green (2.1.308) — `evidence/live-run.md`. The line holds only the 2.1.288 and 2.1.296 readings."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:217
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 73
    section: "## Stack and Technologies"
    change: "re-pin the trailing citation `scripts/cold-agent.sh:557-567` (the judge's stdlib python3 and the `run` preconditions) to the lines that block now spans — `scripts/cold-agent.sh` gained 127 lines in 25 ranges, every one of them at new lines 3–577, ahead of it; the number read by the P2 citation sweep, not derived here"
    sidecar: "stack-and-technologies: the cold-agent pipe citation re-pinned after the pipe grew (2026-10-25-cold-agent-bar)"
    rationale: "The report's F14 lists stack-and-technologies.md:73 as a `citation` row (`cold-agent.sh:N`, the file grown by 127 lines in 25 ranges); `## New text, by line` lists the added ranges 3-7 … 571 · 577, all ahead of the cited block."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:169
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 76
    section: "## Established Decisions"
    change: "add after line 76 a bullet: «- A measured hazard of the overlay scrollbar, no engine change: right after a `scroll`, a main-button pointerdown at a point in the shown overlay scrollbar's band is taken as a thumb drag and dispatches nothing (packages/blitz-dom/src/events/pointer.rs, `hit_with_scrollbar` with `scrollbar_opacity > 0`); on the virtual clock the bar stays shown until an `advance` passes its fade, and a pointer left on the thumb keeps it shown — so a driver `click` inside a just-scrolled box needs an `advance` past the fade first, as `stand_cells_served` walks Cells to `Z25` with `advance 5000` (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)»"
    sidecar: "established-decisions: the shown overlay scrollbar takes a pointerdown right after a scroll as a thumb drag, held on the virtual clock until its fade (2026-10-25-cold-agent-bar)"
    rationale: "Task F8 (established-decisions); the report's Decisions & corrections 'Measured: right after `scroll`, the engine's overlay scrollbar takes a main-button pointerdown at a cell in its band as a thumb drag …', its Deviations (`stand_cells_served`'s walk takes `advance 5000` after the scroll) and F21 (the scrollbar-fade facts it rests on stand true). It sits beside the clock bullet, which names the overlay scrollbar fade read through `DocumentConfig.clock`."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:355
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 97
    section: "## Established Decisions"
    change: "re-pin the citation `tests/blitz-tests/tests/stand_keyboard.rs:677-718` (the disabled `summary` / `a href` activation pin) to the lines that test now spans — the file gained 76 lines in 6 ranges (7-15 · 45 · 208 · 214 · 220-221 · 249-310) and lost rows of its activation table ahead of it; the number read by the P2 citation sweep, not derived here"
    sidecar: "established-decisions: [DOM semantics]' stand_keyboard citation re-pinned after the flight-type select's keyboard test landed (2026-10-25-cold-agent-bar)"
    rationale: "stand_keyboard.rs is a reached file, re-pinned this chunk (F7: activation table 20 → 16 rows, `BUTTONS` 10 → 8, tests 14 → 15, the new `the_flight_type_select_moves_its_choice_by_its_arrows` at 249-309); `## New text, by line` lists every addition ahead of the cited 677-718, so its lines moved. The rule the citation pins (`disabled` read by presence on a form control alone) is unchanged — no `packages` path changed."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:576
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 54
    section: "## Conventions"
    change: "in the list of checks that hold a `Session` and keep what they share in `tests/blitz-tests/tests/session_common/mod.rs`, after «since 2026-10-14-stand-requirement-sweep, the driver sweep `stand_requirement_sweep`» add «and, since 2026-10-25-cold-agent-bar, the served-Cells walk `stand_cells_served` (tests/blitz-tests/tests/stand_cells_served.rs:19-21)»"
    sidecar: "conventions: stand_cells_served joins the Session-holding checks that read session_common (2026-10-25-cold-agent-bar)"
    rationale: "The report's Files name the new `tests/blitz-tests/tests/stand_cells_served.rs` (209 lines), which holds a `Session` (`Session::start(\"cells\", …)` at 35-36) and reads the shared module (`use session_common::{` at 19-21, `## New text, by line`); the entity's enumeration of the checks keeping their shared helpers there does not name it. Its two module citations (common/mod.rs:1-6, session_common/mod.rs:1-7) hold — the additions to those files begin at 25 and 305."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:549
note: the two citation re-pins (stack-and-technologies.md:73, established-decisions.md:97) carry no derived line number — the orchestrator takes the number from the P2 citation sweep; no new library, runtime, crate or locked-decision contradiction in this group (Dependencies none), so D-arch-decisions and D-arch-resources fire nothing here.
