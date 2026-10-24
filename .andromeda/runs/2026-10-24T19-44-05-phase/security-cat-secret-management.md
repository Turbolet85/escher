# security · Secret Management — category read

## Carried
- (.andromeda/security-plan-detail/secret-management.md:8) The `a11y` leg's widening (`.github/scripts/ci-leg.sh`, `ci.yml`'s "Accessibility (a11y) tests" job, `test_ci_workflows.py`'s pin) must leave CI secrets where they live: no secret, var or token is added to the a11y job or the leg script, which keeps reaching none.
  > - **Production (CI):** secrets live in GitHub Actions secrets and vars:
- (.andromeda/security-plan.md:288) The same rule stands broad in the body: the CI secrets are the listed signing material and tokens, passed only to the steps that use them, so a re-read's CI edit passes none of them to the a11y job.
  > **Production (CI):** secrets live in GitHub Actions secrets and vars — a macOS signing key and an Android keystore written from secrets for the build and removed in `always()` steps, the Apple certificate and keystore passwords passed from secrets, and the `WPT_GITHUB_TOKEN` and `GITHUB_TOKEN` tokens passed to the steps that use them.
- (.andromeda/security-plan.md:292) Every re-recorded row, new `stand_keyboard` row (the flight booker's select) and new a11y check must read no secret in source, so the census stays at zero.
  > **Never in code:** secret reads in source are observed absent in every slice that searched:
- (.andromeda/security-plan.md:294) The chunk adds no env read (its scope's Boundaries), so the environment-values list must stay exactly as written: no test the leg gains and no row the re-read adds reads an env var outside it.
  > **Environment values read (none secret-bearing):**
- (.andromeda/security-plan.md:301) `stand_act_select`, a candidate for the widened a11y leg, holds a session through `tests/session_common/`, whose `CARGO_TARGET_TMPDIR` read is one of the registered build-time reads; moving that target into the leg must keep it inside this registration, and no other env read may come with it.
  > `env!("CARGO_BIN_EXE_escher-session")` and `env!("CARGO_TARGET_TMPDIR")` at build time in the session checks — the host binary's path and the directory their state directories live under; the escher-driver library's code and the `escher-session` binary read no env var beyond `RUST_LOG` through `escher_telemetry::init`
- (.andromeda/security-plan.md:306) The secret set is closed as listed; fixtures, test rows and committed evidence the re-read writes (re-recorded Tab sequences, contrast rows, the chunk report) must hold none of it.
  > **What counts as secret (as observed):** CI signing material — the macOS signing key, the Android keystore and its passwords, the Apple certificate and its password — and the `WPT_GITHUB_TOKEN` and `GITHUB_TOKEN` tokens (see Storage above); on the operator host, the operator's Claude Code login the cold-agent live run and the 2026-10-10-mcp-surface live trial use, held by the `claude` CLI — no artifact, event or verdict carries it.

## Bindings
- **CI secrets ↔ Authentication & Authorization (the body)**: an edit to the a11y job in `ci.yml` keeps the job's `GITHUB_TOKEN` read-only like every other ci.yml job, and passes it no secret (.andromeda/security-plan-detail/secret-management.md:8) (.andromeda/security-plan.md:39)
- **CI secrets ↔ test-plan §CI Integration**: the a11y leg keeps its tier and takes no required status check (scope), and on the security side it also takes no secret (.andromeda/security-plan.md:288)
- **Environment values ↔ architecture §Occupied Resources**: a new env var is an Occupied Resources registration first; the chunk adds none, so the security plan's list and the arch registry both stay unchanged (.andromeda/security-plan.md:294) (.andromeda/security-plan.md:301)
