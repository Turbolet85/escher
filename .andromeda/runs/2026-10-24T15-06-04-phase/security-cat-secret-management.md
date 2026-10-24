# security · Secret Management — category read

## Carried
- (.andromeda/security-plan.md:292) Every check, fixture page and table the sweep adds (the requirement record, the C1 lint and `text_extent` fixtures, the stderr measurement's needles) must read no secret in source.
  > **Never in code:** secret reads in source are observed absent in every slice that searched:
- (.andromeda/security-plan.md:294) The environment values a new or widened check reads must stay inside the recorded, non-secret list; a value outside it is a registration, not a silent add.
  > **Environment values read (none secret-bearing):**
- (.andromeda/security-plan.md:301) The stderr measurement of the new verbs (`lint`, `snapshot`'s `part`, `--record`) through the `escher-session` host and its `mcp` role, beside or inside `host_log` and `mcp_log`, must reach the binary and its state directories only through these build-time values, and the escher-driver library and the `escher-session` binary must read no env var beyond `RUST_LOG`.
  > `env!("CARGO_BIN_EXE_escher-session")` and `env!("CARGO_TARGET_TMPDIR")` at build time in the session checks — the host binary's path and the directory their state directories live under; the escher-driver library's code and the `escher-session` binary read no env var beyond `RUST_LOG` through `escher_telemetry::init` (examples/seven_guis/tests/host_binary.rs:16; examples/seven_guis/tests/host_log.rs:18; examples/seven_guis/tests/common/mod.rs:16-18; tests/blitz-tests/tests/session_common/mod.rs:331)
- (.andromeda/security-plan.md:300) The measurement at `trace` sets the filter through `RUST_LOG` alone, the one env var the sink reads, which carries no secret.
  > `RUST_LOG`, the tracing filter directives escher-telemetry's `EnvFilter` reads, defaulting to `warn` when unset or unparsable — not secret (packages/escher-telemetry/src/lib.rs:144)
- (.andromeda/security-plan.md:295) A new escher-driver unit test (a lint reading asserted in `lint.rs`, or one added to hold the `escher-driver` coverage floor) that reaches `target/tmp` must do so through `CARGO_MANIFEST_DIR` at build time, as the recorded unit tests do.
  > `env!("CARGO_MANIFEST_DIR")` at build time (examples/screenshot.rs:172), in a test (packages/blitz-vibey-script/tests/preact.rs:13), since 2026-10-19-session-recording in escher-driver's unit tests, which reach the workspace `target/tmp` through it (packages/escher-driver/src/mcp_tests.rs:633-634)
- (.andromeda/security-plan.md:302) An env read that only a test makes must be registered as test-only before it lands, as `TMPDIR` was; the same holds for any read the sweep's checks add.
  > test code only, not secret (packages/escher-driver/src/client.rs:263-308; since 2026-10-14-quality-gates, registered as test-only on the operator's answer at that chunk's wrap, 2026-10-15)
- (.andromeda/security-plan.md:306) The chunk's committed evidence (the sweep record, the stderr captures and the chunk report) must carry none of the secrets listed here; the sweep runs no cold-agent live run (v020-11 is out of scope), so the operator's login is not reached.
  > **What counts as secret (as observed):** CI signing material — the macOS signing key, the Android keystore and its passwords, the Apple certificate and its password — and the `WPT_GITHUB_TOKEN` and `GITHUB_TOKEN` tokens (see Storage above); on the operator host, the operator's Claude Code login the cold-agent live run and the 2026-10-10-mcp-surface live trial use, held by the `claude` CLI — no artifact, event or verdict carries it.

## Bindings
- **Env values read ↔ Logging & Monitoring (body):** the `trace` measurement of the new verbs' stderr is driven by `RUST_LOG`, the sink's one env read, and the sink's reach is escher's own allowlist subscriber; both sides bind the "new verbs' stderr id-free" measurement (.andromeda/security-plan.md:300) (.andromeda/security-plan.md:301)
- **Env values read ↔ architecture §Occupied Resources:** a new env var a check reads, test-only included, is registered in the architecture's resource registry and in this list before it lands (.andromeda/security-plan.md:302)
- **What counts as secret ↔ test plan (committed evidence, the cold-agent pipe):** the sweep's evidence holds no secret; the cold-agent live run and its login stay with «Cold-agent bar», not this chunk (.andromeda/security-plan.md:306)
