proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 76
    section: "## Stack and Technologies"
    change: The parenthetical after `bash .github/scripts/ci-leg.sh fast` reads (fmt → clippy → test → ci-scripts → embolden, stopping at the first red), the rest of the line unedited.
    sidecar: Code quality row, the `fast` leg's order gains its last step `embolden`, the font-embolden witness, as the chunk's `ci-leg.sh` holds it.
    rationale: The chunk adds the leg `embolden` last in `FAST_LEGS`, and the report rules this line changed for both its fast-leg hits, one claim in one parenthetical (report.md:41, report.md:64).
    basis: .github/scripts/ci-leg.sh:8-9
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 22
    section: "## Stack and Technologies"
    change: The Role clause ends «with no `pub use` of it», the words «the comment at lib.rs:96-97 heading none» gone, the rest of the line unedited.
    sidecar: Layout row, the orphan «Re-export taffy» comment is retired, removed from blitz-dom's `lib.rs` by the chunk, so the row no longer names it.
    rationale: The report's Retired bullet names the orphan comment as item 5 and its Removed line takes the two-line comment out, ruling this line changed (report.md:26, report.md:33, report.md:55).
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:26
note: D-arch-decisions and D-arch-resources hold for this category, Dependencies none (report.md:30); line 73's free-disk step of `Stand checks repeated` stands, as report.md:43 rules.
