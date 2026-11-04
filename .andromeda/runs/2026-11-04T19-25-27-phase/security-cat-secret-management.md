@carried n=1 label=security-plan fact=fct-p7pp73ko
why: The probe's test or example code (in tests/blitz-tests/, examples/seven_guis/ or packages/dioxus-native-dom/) must add no secret read to source, so the observed absence still holds after the chunk.
quote: **Never in code:** secret reads in source are observed absent in every slice that searched:
@carried n=2 label=security-plan fact=fct-pz4e3fuz
why: The probe must add no environment variable read, so the escher-driver library and the `escher-session` binary still read no env var beyond `RUST_LOG`; any env read in a probe check is limited to the build-time `env!` values already recorded for the session checks.
quote: `env!("CARGO_BIN_EXE_escher-session")` and `env!("CARGO_TARGET_TMPDIR")` at build time in the session checks — the host binary's path and the directory their state directories live under; the escher-driver library's code and the `escher-session` binary read no env var beyond `RUST_LOG` through `escher_telemetry::init`
@carried n=3 label=security-plan fact=fct-77bajbne
why: Any environment value a probe check reads must be non-secret and must be added to this list; the chunk's boundary allows no new env var, so the list must stay as it is.
quote: **Environment values read (none secret-bearing):**
@carried n=4 label=security-plan fact=fct-ipi52teh
why: The probe must not reach CI signing material, the CI tokens or the operator's Claude Code login, and its reading for the founder must carry none of them.
quote: **What counts as secret (as observed):** CI signing material — the macOS signing key, the Android keystore and its passwords, the Apple certificate and its password — and the `WPT_GITHUB_TOKEN` and `GITHUB_TOKEN` tokens (see Storage above); on the operator host, the operator's Claude Code login the cold-agent live run and the 2026-10-10-mcp-surface live trial use, held by the `claude` CLI — no artifact, event or verdict carries it.
@binding n=1
**Secret Management env reads ↔ the chunk's "Nothing crosses" boundary (scope)**: the probe adds no env var, so the recorded list of env values read stays the same and the escher-driver and `escher-session` reads stay limited to `RUST_LOG` (security-plan fct-pz4e3fuz) (security-plan fct-77bajbne)
@binding n=2
**What counts as secret ↔ Logging & Monitoring / Data Protection masking**: the probe's committed evidence and the founder's reading must hold no secret, and nothing the probe reads is logged (the scope's "values masked, never logged" rule) (security-plan fct-ipi52teh)
