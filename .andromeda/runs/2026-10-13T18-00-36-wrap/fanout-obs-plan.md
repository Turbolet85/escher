proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage'
    change: 'In the escher-driver command-span entry, extend the "Still observed absent" list: since 2026-10-13-stand-contrast-harness also blitz-test-harness`s contrast reader — `Harness::text_contrast` and the root re-exports `contrast_ratio` / `TextContrast` (the private sixth module `contrast`) — which opens no span, emits no event, reads no env var, mutates nothing and prints, panics and logs nothing; the crate still takes no `tracing` dependency (packages/blitz-test-harness/src/contrast.rs:13-23; packages/blitz-test-harness/src/contrast.rs:56-91).'
    sidecar: '§4 — 2026-10-13-stand-contrast-harness: Harness::text_contrast / contrast_ratio / TextContrast join the observed-absent list beside Harness::screenshot; blitz-test-harness still takes no tracing.'
    rationale: 'Report Changes → Symbols / APIs adds the reader (a test-harness reader, not an external surface or hot path; instrumentation n/a by §4 per Coverage of new surfaces); Expected amendments names "obs-plan §4 — the contrast reader opens no span", pattern obs-silent at obs-plan.md:106, carried by the gate census over contrast.rs (non-test part) reading 0 for println!/tracing::/.mutate(/std::env and the manifest census `grep -c tracing packages/blitz-test-harness/Cargo.toml` reading 0. No instrumentation is owed; the §4 record of what is uninstrumented must name it.'
    basis: '.andromeda/obs-plan.md:106'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§3 Observability Harness Contract → Logging stack (subscriber installation)'
    change: 'In the headless-stand entry, where it reads "for blitz-test-harness`s `Harness::screenshot` / `screenshot_rect`, which print, panic and log nothing, beside the settle loop", add `Harness::text_contrast` (with `contrast_ratio` and `TextContrast`) since 2026-10-13-stand-contrast-harness — installs no subscriber, reads no env var, prints, panics and logs nothing, reads and changes nothing in the document.'
    sidecar: '§3 Logging stack — 2026-10-13-stand-contrast-harness: the harness contrast reader joins Harness::screenshot in the silent harness listing.'
    rationale: 'Same retired claim as the §4 proposal (the silent harness surface listing): the report`s obs-silent pattern names obs-plan.md:70 beside :106 as the harness screenshot entries the reader joins (Expected amendments, obs-plan §4).'
    basis: '.andromeda/obs-plan.md:70'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-stack
    severity: warning
    section: '§3 Observability Harness Contract → Logging stack (subscriber installation)'
    change: 'In the headless-stand census of checks that install no subscriber, name `stand_contrast` since 2026-10-13-stand-contrast-harness — it installs no subscriber, reads no env var, prints nothing and binds no listener, measures every stand control in both layout modes, its readings into assertions only, and declares `mod common;` (tests/blitz-tests/tests/stand_contrast.rs).'
    sidecar: '§3 Logging stack — 2026-10-13-stand-contrast-harness: stand_contrast joins the silent census by name (0 subscriber · 0 env · 0 print · 0 bind).'
    rationale: 'Report Changes → Harness / gate surface adds the stand check target `stand_contrast`; Expected amendments names "obs-plan §3 — stand_contrast joins the silent census by name", census site obs-plan.md:70, carried by the gate entry over stand_contrast.rs reading 0 at exit 1. The check matches the §3 stack (no sink, no off-spec logger); the census must name it.'
    basis: '.andromeda/obs-plan.md:70'
  - detector: D-obs-stack
    severity: warning
    section: '§3 Observability Harness Contract → Logging stack (subscriber installation)'
    change: 'In the headless-stand entry, "driven in-process by the `stand_*` checks, 17 of which read the shared module `tests/blitz-tests/tests/common/mod.rs`" now reads 18 — `stand_contrast` the eighteenth, since 2026-10-13-stand-contrast-harness; the trailing provenance "the 17 readers … at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md" stays as history and gains "the eighteenth, `stand_contrast`, at escher-0.1.0/chunks/2026-10-13-stand-contrast-harness/report.md".'
    sidecar: '§3 Logging stack — 2026-10-13-stand-contrast-harness: mod common; readers 17 → 18.'
    rationale: 'Report Counts / qualifiers moved: `mod common;` readers 17 → 18 (`grep -l` over tests/blitz-tests/tests/*.rs reads 18, evidence/counts.md). The report`s test-totals sweep listed architecture.md:263 and test-plan.md:66/:100 but missed this obs-plan statement of the same count, worded "17 of which read the shared module" (no `mod common;` token), at the census line the report names.'
    basis: '.andromeda/obs-plan.md:70'
    dependent-of: D-obs-stack
note: 'D-obs-pii: no drift — the chunk adds no log, span, event or subscriber; TextContrast holds colours and font metrics only and the check`s messages name mode, task and row (report Coverage of new surfaces). The report`s test-totals sweep missed the obs-plan.md:70 "17 of which" count (proposal 4) — the same miss may exist in other masters worded without the `mod common;` token.'
