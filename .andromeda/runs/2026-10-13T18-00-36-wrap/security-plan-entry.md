
## 2026-10-13-stand-contrast-harness — blitz-test-harness takes `color`, reviewed
**Section:** §Dependency Security (Pinning)
**Change:** a Pinning bullet: since this chunk blitz-test-harness takes `color = { workspace = true }` (the workspace's "0.3", resolved 0.3.3, already in the graph through blitz-dom), the luminance source of its contrast reader — reviewed at the chunk: no package enters `Cargo.lock` (its one changed line `"color",` inside the harness's dependency list), the audit leg reads `advisories ok` with `deny.toml` unchanged, the harness takes no `tracing` and no `style`, and blitz-tests gains no dependency; no ban list exists.
**Why:** every dependency an escher crate takes is recorded as a reviewed Pinning bullet; no boundary moved — the crate was already in both graphs.
**Ref:** .andromeda/runs/2026-10-13T18-00-36-wrap/
