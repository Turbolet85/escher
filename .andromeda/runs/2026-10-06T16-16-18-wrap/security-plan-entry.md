
## 2026-10-06-founder-rulings — cold-agent crossings ratified; OTel export out of escher 0.1.0
**Section:** Input Validation → cold-agent MCP stub row · Secret Management → Development (operator host)
**Change:**
- Cold-agent pipe: the stub's input row and the operator-login clause were "PROVISIONAL, overseer under the founder's standing delegation, pending the founder's word"; now "ratified by the founder, 2026-10-06". The credential path (the operator's own Claude Code login, `apiKeySource` none, no API key, no env read, no CI secret) and the stdio IPC surface are unchanged.
- Opt-in OTel export: was deferred, provisional on the founder's word, per the 2026-10-06-telemetry-bootstrap entry; now ruled out of escher 0.1.0 — no egress and no `OTEL_EXPORTER_OTLP_HEADERS` credential path ship, and neither the transport nor the credential path is decided. No body text changes: the body never registered either surface.
**Why:** the founder's own word at the Epoch 2 boundary (the founder, 2026-10-06), superseding the delegate overseer's provisional answers by rule. The OTel decision is held as a cross-version residual (`.andromeda/residuals.md`); a later version that takes it up registers the egress and the credential path here first.
**Ref:** .andromeda/runs/2026-10-06T16-16-18-wrap/
