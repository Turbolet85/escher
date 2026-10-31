# security · Secret Management — category read

## Carried
- (.andromeda/security-plan-detail/secret-management.md:33) Where a fix moves the `off-screen` or `covered` limit text and with it `docs/capabilities.md` or its pin `.github/scripts/test_capability_statement.py`, the moved text and the pin must still hold no host path, API key, auth header or bearer value and no e-mail domain, and the pin must read no environment variable; this measured line is re-read after the move.
  > searched: `/home/|api[_-]?key|authorization:|bearer ` (case-insensitive) over `README.md`, `docs/capabilities.md` and `.github/scripts/test_capability_statement.py` (0 hits, as measured at escher-0.3.0/chunks/2026-10-27-readme-and-records/report.md), and two e-mail domains (case-insensitive) over `docs/capabilities.md` and the pin (0 hits); the pin reads no environment variable
- (.andromeda/security-plan-detail/secret-management.md:24) A native-checkedness fix in upstream's `accessibility.rs` must leave `password` there as a role mapping and no credential, and if the edit shifts the mapping's line, this citation moves with it at the wrap.
  > searched: `env::var|secret|token|authorization|cookie|x-api-key` over the 15 s05 files; the only `password` occurrence maps the input type `password` to `Role::PasswordInput` (packages/blitz-dom/src/accessibility.rs:519)
- (.andromeda/security-plan.md:301) A driver change for the select-option overlap in `lint.rs`, or for any limit's schema text, must keep the driver library and the `escher-session` binary free of any env read beyond `RUST_LOG`.
  > the escher-driver library's code and the `escher-session` binary read no env var beyond `RUST_LOG` through `escher_telemetry::init`
- (.andromeda/security-plan.md:292) Every engine or driver fix and every new check (the served pages' rows, CARRY 2's three checks) must read no secret in source.
  > **Never in code:** secret reads in source are observed absent in every slice that searched:
- (.andromeda/security-plan.md:294) Any environment value a fix, fixture or check of this chunk would read must be non-secret and recorded in this list, or it does not ship.
  > **Environment values read (none secret-bearing):**

## Bindings
- **Driver env read ↔ §Input Validation, driver command schema**: both sides require the driver to read no env var beyond telemetry's `RUST_LOG`, so a `lint.rs` or schema-text change answers to both, (.andromeda/security-plan.md:301) and (.andromeda/security-plan.md:79)
- **Password role mapping ↔ §Input Validation, masked values**: a checkedness fix on the same adapter path must leave the mapping a role and a password's value `MASKED_VALUE` on every carrier, (.andromeda/security-plan-detail/secret-management.md:24) and (.andromeda/security-plan.md:139)
- **Environment registry ↔ test plan, new checks**: the served pages' rows and CARRY 2's checks are the test plan's to place, and any env value they read is registered here, (.andromeda/security-plan.md:294)
