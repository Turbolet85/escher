proposals:
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 77
    section: "## Stack and Technologies"
    change: "After the mutation tier's clause on the Code quality layer's Technology, add: and, since 2026-10-26-codebase-health, a cognitive-complexity reading run on the dev host alone, never in CI and no leg: `rust-code-analysis-cli` 0.0.25 (`~/.cargo/bin`, installed 2026-10-03), not lockfile-resolved, its `cognitive.sum` read per function against a ceiling of 15, a closure or nested `fn` counted in its parent (dev host, as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md); and on its Role line add: a host-run complexity reading"
    sidecar: "2026-10-26-codebase-health: Stack and Technologies › Code quality names the dev-host complexity reading's tool, rust-code-analysis-cli 0.0.25, no CI leg, not lockfile-resolved"
    rationale: "The chunk's acceptance (v030-01) and its first gate rest on `rust-code-analysis-cli … over 15`, a dev-host reading with no CI leg, the tool re-read at 0.0.25 and resolved by no lockfile (report Dev-tool versions, Harness / gate surface, Gates, Decisions & corrections on `cognitive.sum`); the Code quality layer names every other host-run tool (cargo-mutants, cargo-nextest) and not this one, so the stack the chunk used is wider than the stack the file allows"
    basis: escher-0.3.0/chunks/2026-10-26-codebase-health/report.md:110
note: "No resource, crate, dependency, retirement or public symbol landed (report Changes: Symbols, Crates, Dependencies, Retired all none), so D-arch-resources reads clean; the body's Stack line (.andromeda/architecture.md:19, «beside a host-run mutation tier and a Nix flake») would carry the complexity reading's broad line if the orchestrator applies the proposal — a body edit, not this file's; the complexity population (88 → 85) is deliberately left out, stated in no master per the report."
