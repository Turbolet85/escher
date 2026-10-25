# security · Secret Management — category read

## Carried
- (.andromeda/security-plan-detail/secret-management.md:14) The statement's "what an MCP client carries onward" part names the cold-agent pipe as the measured instance, so wherever it says how that pipe reaches the model provider it must say what this line says: through the operator's own login held by the `claude` CLI, with no API key and no env read by the pipe.
  > the cold-agent pipe's `claude -p` session reaches the model provider through the operator's own Claude Code claude.ai login, held by the `claude` CLI outside the repository — no API key (the session reports `apiKeySource: "none"`), no env read by the pipe
- (.andromeda/security-plan-detail/secret-management.md:14) The statement must not claim CI reaches the model provider or holds a secret for it. Only the shim contract tests run in CI.
  > and no CI secret (CI runs only the shim contract tests)
- (.andromeda/security-plan-detail/secret-management.md:15) The statement is committed text, in README.md or a tracked document, and may cite the live-run evidence. Like that evidence, it must hold no key, auth header, credential-file name, token field or e-mail address.
  > the committed live evidence of both runs holds no key, auth header, credential-file name, token field or e-mail address
- (.andromeda/security-plan.md:290) The development-secret line the statement must match if it describes the live run or the MCP live trial: no new secret source and no auth library.
  > both reach the model provider through the operator's own Claude Code login, held by the `claude` CLI outside the repository — no API key, no env read by the pipe, no CI secret, no new secret source and no auth library; the committed live evidence holds no key, auth header, credential-file name, token field or e-mail address.
- (.andromeda/security-plan.md:301) If the statement says what the driver or its binary reads from the environment, it must match this value: `RUST_LOG` alone, through the telemetry bootstrap.
  > the escher-driver library's code and the `escher-session` binary read no env var beyond `RUST_LOG` through `escher_telemetry::init`
- (.andromeda/security-plan.md:300) If the statement names `RUST_LOG`, it must describe it as a filter that is not secret and defaults to `warn`.
  > `RUST_LOG`, the tracing filter directives escher-telemetry's `EnvFilter` reads, defaulting to `warn` when unset or unparsable — not secret
- (.andromeda/security-plan.md:306) The operator's login counts as a secret. The statement is an artifact and must not carry it.
  > on the operator host, the operator's Claude Code login the cold-agent live run and the 2026-10-10-mcp-surface live trial use, held by the `claude` CLI — no artifact, event or verdict carries it.
- (.andromeda/security-plan.md:308) A limits list must not state as measured what is still unmeasured, including where the login is stored at rest and whether CI scans for secrets.
  > development secret storage (beyond the fact that the cold-agent run's login is held by the `claude` CLI; its at-rest location is unmeasured), secret scanning in CI, rotation cadence and access auditing: no slice gathered them.

## Bindings
- **Cold-agent login (secret) ↔ the body's Threat Model Summary (MCP stdio vector)**: the statement's "what an MCP client carries onward" part rests on two lines. One says what a client does with the content is the client's, the model provider included (.andromeda/security-plan.md:27). The other says that in the measured instance the client reaches that provider through the operator's login and no API key (.andromeda/security-plan-detail/secret-management.md:14). Both must agree in the statement.
- **Committed evidence holds no credential ↔ the body's Data Protection (committed checks)**: the statement may cite committed material. The committed Counter checks hold no secret (.andromeda/security-plan.md:187), and the committed live evidence holds no key, header, credential-file name, token or e-mail (.andromeda/security-plan-detail/secret-management.md:15). The statement keeps both clean.
- **Env values read ↔ the body's Input Validation (escher-session CLI, MCP server)**: the driver's env read stated at (.andromeda/security-plan.md:301) agrees with the command line reading no environment variable (.andromeda/security-plan.md:81) and the MCP server's lack of any env read (.andromeda/security-plan.md:83). If the statement states one, it states them consistently.
