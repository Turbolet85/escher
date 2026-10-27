# Citation dispositions — security-plan

held .andromeda/security-plan.md:183 apps/browser/Cargo.toml:14 — refused at 2026-10-24T12-21-29-wrap, not touched since «default = […» → holds — stands (line 14's `default` list still holds `cookies` and `cache`)
held .andromeda/security-plan.md:331 packages/blitz-dom/src/layout/table.rs:535-539 — refused at 2026-10-13T15-50-37-wrap, not touched since «let Some(di…» → holds — stands (535-539 is the `let Some(display) … else` arm that logs at info under `tracing` and returns)
changed .andromeda/security-plan-detail/dependency-security.md:9 .andromeda/residuals.md:29 → ? — rewritten «2026-10-1…» → holds — re-pointed by hand to 29 (the zip-ignore residual, now `re-carried:0.3.0`, still target: next)
changed .andromeda/security-plan-detail/dependency-security.md:9 .andromeda/residuals.md:30 → ? — rewritten «2026-10-1…» → holds — re-pointed by hand to 30 (the zbus-family residual, now `re-carried:0.3.0`, still target: next)
held .andromeda/security-plan-detail/input-validation.md:201 tests/blitz-tests/tests/stand_snapshot.rs:1-5 — refused at 2026-10-25T17-25-04-wrap, not touched since «//! The sna…» → holds — stands (the module doc still states every snapshot node carries its stable element id, both layout modes)
