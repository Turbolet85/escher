# security · Secret Management — category read

## Carried
- (.andromeda/security-plan.md:301) The list's changes to `escher-driver` (`execute`, `lint`, schema help) must leave the library and the `escher-session` binary reading no environment variable beyond `RUST_LOG`; any new read is named on a fork and registered in this list.
  > the escher-driver library's code and the `escher-session` binary read no env var beyond `RUST_LOG` through `escher_telemetry::init`
- (.andromeda/security-plan.md:292) The new overlay code in paint, hit testing, keys and the snapshot must add no secret read in source: the list's options, colours and keys take no credential, key or token from anywhere.
  > **Never in code:** secret reads in source are observed absent in every slice that searched:
- (.andromeda/security-plan-detail/secret-management.md:24) If the chunk's open-state work on the accessibility tree edits `packages/blitz-dom/src/accessibility.rs`, this census citation must move with the shifted line, and `password` there must still only map the input type to its role, with no new credential match.
  > the only `password` occurrence maps the input type `password` to `Role::PasswordInput` (packages/blitz-dom/src/accessibility.rs:543)

## Bindings
- **No env read beyond `RUST_LOG` ↔ body §Input Validation**: the driver's own rule that the crate reads no env var and the command line's that it reads none hold the same ground as this line, so a fork needing an env read moves all three, (.andromeda/security-plan.md:301) with (.andromeda/security-plan.md:79) and (.andromeda/security-plan.md:81).
- **The census's `password` citation ↔ body §Authentication & Authorization**: both record `password` in the accessibility role mapping as no credential, the census at the mapping and the body at its test fixture, so the two must keep the same reading, (.andromeda/security-plan-detail/secret-management.md:24) with (.andromeda/security-plan.md:45).
