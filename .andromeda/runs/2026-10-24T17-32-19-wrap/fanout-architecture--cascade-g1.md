proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 77
    section: "## Stack and Technologies"
    change: "Re-point the stand-repeat leg's citation `.github/scripts/ci-leg.sh:48-65` so that its range ends five lines later, because the five lines added at 54-58 (the ten named `--test` words) fall inside the leg. The clause itself, the `stand-repeat` leg holding the stand checks to a zero-failure budget over three rounds, stays word for word, and so does `ci-leg.sh:21-25`."
    sidecar: "stack-and-technologies:77: the stand-repeat leg's ci-leg.sh citation now ends five lines later (54-58 added inside it, 2026-10-24-stand-requirement-sweep)."
    rationale: "The report's Schema / config section and its New text section show `.github/scripts/ci-leg.sh` gaining 5 lines at 54-58 inside `stand-repeat`, so the cited range 48-65 no longer covers the whole leg. Counts fact 1 rules this row's prose no-change because it names only the leg and its budget, so only the citation moves."
    basis: .andromeda/architecture-detail/stack-and-technologies.md:77
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 76
    section: "## Stack and Technologies"
    change: "Move the citations into `.github/scripts/ci-leg.sh` that lie past line 53 down five lines: `:80-85` (the `fast` gate) and `:87` (the `RUSTDOCFLAGS` export). Leave `:9`, `:20-25`, `:37` and `:38-47` unchanged, since they come before the lines added at 54-58. The prose is unchanged."
    sidecar: "stack-and-technologies:76: the fast gate's and the RUSTDOCFLAGS export's ci-leg.sh citations moved down five lines (54-58 added, 2026-10-24-stand-requirement-sweep)."
    rationale: "The report lists `.github/scripts/ci-leg.sh` as `added: 54-58`, 5 lines in one range, so every line of the reached file after 53 moved down five. The task's Touches section says the same of citations past line 53."
    basis: .andromeda/architecture-detail/stack-and-technologies.md:76
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 61
    section: "## Conventions"
    change: "Add `tests/blitz-tests/tests/fixtures/mod.rs:64-86` to the entity's citations: the `many_items` page and `ITEMS`, moved unchanged from `stand_snapshot_parts.rs` once `stand_act_spans` became their second reader. Re-read the `:87-130` citation (`HeldNet`) against the changed file, which gained 23 lines at 64-86, and point it where that range now stands. The rule's prose stays word for word."
    sidecar: "conventions:61: the shared fixture pages now cite many_items (fixtures/mod.rs:64-86, moved in from stand_snapshot_parts once it had a second reader), and the HeldNet citation was re-read past the 23 added lines (2026-10-24-stand-requirement-sweep)."
    rationale: "In the report's Symbols section, `fixtures/mod.rs` gains `pub const ITEMS` and `pub fn many_items` at 64-86, moved from `stand_snapshot_parts.rs` and now read by both it and `stand_act_spans`. That is the rule's own case of a page with more than one reader. Counts fact 10 rules conventions.md:61's prose no-change, so only the citations move."
    basis: .andromeda/architecture-detail/conventions.md:61
note: "conventions.md:61's `:87-130` citation starts on the line right after the added 64-86. It may already have been pointed at the changed file, so re-read it with `cites.py` rather than shifting it by 23 blind. The body lines architecture.md:123 and :315 carry no line citation and stand unchanged."
