
## 2026-10-06-founder-rulings — OTel export out of escher 0.1.0; coverage-report upload ratified
**Section:** §3 Observability Harness Contract → Bootstrap phases (`otel-sdk-install`) · §9 CI Integration (`coverage-report`)
**Change:**
- `otel-sdk-install`: was "the opt-in export was deferred … (an egress and `OTEL_EXPORTER_OTLP_HEADERS` credential-path decision for the founder) and is carried to "Driver command spans""; now escher 0.1.0 ships no OTel export — no OTel crate, no egress, no credential path — with the export transport and the credential path both undecided, held in `.andromeda/residuals.md`. "Driver command spans" keeps the driver's spans and loses the export.
- `coverage-report` upload: no body text changes; now ratified, no longer PROVISIONAL per the 2026-10-05-ci-gate-legs entry.
**Why:** the founder chose option (c), no OTel export in escher 0.1.0, and ratified the upload, in their own word at the Epoch 2 boundary (the founder, 2026-10-06), superseding the delegate overseer's provisional deferral by rule.
**Ref:** .andromeda/runs/2026-10-06T16-16-18-wrap/
