proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 63
    section: "## Stack and Technologies"
    change: "On the Technology line, after `pulldown-cmark 0.13` add: locked at 0.13.5 in `Cargo.lock` alone since 2026-10-31-engine-debts, RUSTSEC-2026-0414's fix, reached through `rdme`, which never sets `ENABLE_WIKILINKS`, so no reader was exposed (apps/readme/src/markdown/pulldown_cmark.rs:7-12)"
    sidecar: "stack-and-technologies, Markdown app (rdme): `pulldown-cmark` locked at 0.13.5 for RUSTSEC-2026-0414, reached through `rdme`, no reader exposed."
    rationale: "Report, Changes, Dependencies: `pulldown-cmark` 0.13.4 to 0.13.5 in `Cargo.lock` alone, RUSTSEC-2026-0414's fix, reached through `rdme`, which never sets `ENABLE_WIKILINKS`; semver-compatible, no coupled pin, `deny.toml` unmoved. The row's requirement 0.13 stands, and the locked version and advisory are the new fact."
    basis: Cargo.lock:6505
    fact: F26
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 73
    section: "## Stack and Technologies"
    change: "In the actions list, read `dtolnay/rust-toolchain` (`# master`, each step naming its `toolchain`), `taiki-e/install-action` (`# v2`) and `jlumbroso/free-disk-space`, the free-disk step of the `Stand checks repeated` job since 2026-10-31-engine-debts (.github/workflows/ci.yml:311-316); the rest of the line stands, the 40-hex pin claim included"
    sidecar: "stack-and-technologies, CI/CD: `jlumbroso/free-disk-space` joins the row's list of ci.yml actions, for the `Stand checks repeated` job."
    rationale: "Report, Harness / gate surface: `Stand checks repeated` gains `jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be` at ci.yml:311-316, and `free-disk`'s control is unfired, so no master states the step. The row enumerates the workflow's actions and omits this one; the Coverage bullet reads it pinned, so the pin claim still holds."
    basis: .github/workflows/ci.yml:311
note: "Not proposed here, each another category's: F23 and F24, the free-disk step's parameters, unproven cure and the skipped older step, are occupied-resources'; F19, the default sheet's chosen-row pair, Chromium's inactive selected pair and so a named-engine value beside design-philosophy#3.1, is established-decisions'."
