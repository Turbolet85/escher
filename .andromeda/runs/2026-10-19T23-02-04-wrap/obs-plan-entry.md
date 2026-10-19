
## 2026-10-19-shared-fixtures — the fixtures module in the silence census
**Section:** §3 Observability Harness Contract → Logging stack (`absence · logs` group)
**Change:** the stand checks' third shared module, `tests/blitz-tests/tests/fixtures/mod.rs` (the fixture boot and hold, three pages and the held network stand-in `HeldNet`), is silent: no print, `tracing` call, subscriber, env read, file, socket, listener or `blitz_net`, the census reading 0. Its 19 readers, now including `stand_mcp_masked`, `stand_boot` and `font_face_sniffed_source`, hold the 4 print or sink sites of `stand_act_spans`' re-run children, none added, and the blitz-tests env-read set is unchanged.
**Why:** this chunk's own test-only module: the playbook's «Accurate this-chunk addition» and the plan's reviewed expected amendment. The «their shared modules» clause of the stand line already covers all three modules, so it is left as written.
**Ref:** .andromeda/runs/2026-10-19T23-02-04-wrap/
