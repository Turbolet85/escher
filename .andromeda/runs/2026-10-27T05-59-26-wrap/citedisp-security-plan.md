# Citation dispositions — security-plan

held .andromeda/security-plan.md:183 apps/browser/Cargo.toml:14 — refused at 2026-10-24T12-21-29-wrap, not touched since «default = […» → holds — stands (line 14 is the `default` feature list, holding `cookies` and `cache`)
held .andromeda/security-plan.md:331 packages/blitz-dom/src/layout/table.rs:535-539 — refused at 2026-10-13T15-50-37-wrap, not touched since «let Some(di…» → holds — stands (535-539 is the `let Some(display) … else` arm: a tracing-gated `info!` and a return, no panic)
held .andromeda/security-plan-detail/dependency-security.md:9 .andromeda/residuals.md:29 — refused at 2026-10-27T02-06-43-wrap, not touched since «2026-10-16-…» → holds — stands (line 29 is the zip RUSTSEC-2026-0367 · 0368 · 0369 ignores line, target: next)
held .andromeda/security-plan-detail/dependency-security.md:9 .andromeda/residuals.md:30 — refused at 2026-10-27T02-06-43-wrap, not touched since «2026-10-17-…» → holds — stands (line 30 is the zbus-family RUSTSEC-2026-0370 · 0371 · 0372 ignores line, target: next)
held .andromeda/security-plan-detail/input-validation.md:201 tests/blitz-tests/tests/stand_snapshot.rs:1-5 — refused at 2026-10-25T17-25-04-wrap, not touched since «//! The sna…» → holds — stands (1-5 is the stand snapshot check's module doc, the snapshot the record is fed from)
