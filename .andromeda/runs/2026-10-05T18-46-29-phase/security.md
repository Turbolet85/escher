# security extract

## Relevance
partial: the chunk adds no source surface, but its build and test run touches dependency-integrity rules (lockfile, pins) and writes evidence files that fall under the secret-handling rules.

## Constraints
- The baseline build must honour the lockfile. Dependency Security §Pinning and §Supply chain integrity require builds to pass `--locked`, and the recorded command form must carry the flag. Whether `ci.yml`'s test leg already passes `--locked` (the plan cites it only for flake.nix and publish-browser.yml) is a question for research.
- Git dependencies stay pinned by commit rev, per Dependency Security §Pinning. The run must leave `Cargo.toml` and `Cargo.lock` unchanged. A red result is never "fixed" by bumping or re-resolving a pin.
- No secret is read or written. Secret Management §Never in code requires source to read no secret, and §Environment values read lists the only env reads (`WPT_DIR`, `PAINT_TREE_BENCH_HTML`, `HOME`, `CARGO_MANIFEST_DIR`). The run sets nothing beyond these, and the host-facts record holds no env dump or credential.
- Audit tooling stays out of this chunk. Bootstrap phases (`dep-audit-tooling-install`, `dep-security-ci-gate`) and Dependency Security §CI integration record dependency auditing as absent. Installing it belongs to a later Foundation entry ("CI gate legs"), not this baseline.
- Any network access during the test run should be noted. API Security §Request timeout / §Request size limit record blitz-net as having no request timeout and no response-size cap. Whether any `blitz-tests` file fetches over the network (and so could hang or vary the wall-clock) is a question for research.

## Patterns to follow
- Use lockfile-verified builds (`--locked`) as the reproducible form, per Dependency Security §Supply chain integrity.
- Inherit dependency versions through `workspace = true`, including every blitz-tests dev-dependency, per Dependency Security §Pinning. The baseline reads these as-is.
- Keep host-only, non-secret environment reads, per Secret Management §Environment values read. `PAINT_TREE_BENCH_HTML` gates an ignored benchmark, so leave it unset for the baseline reading.

## Anti-patterns to avoid
- Do not run `cargo update`, change a pin on one side only, or drop `--locked` to get a build green (per Dependency Security §Pinning).
- Do not paste secrets, tokens or a full `env`/`printenv` dump into the chunk's evidence folder (per Secret Management §Never in code / §What counts as secret).

## Contract bindings
- security §Pinning `--locked` ↔ the tests/CI test-leg command form (test-plan §3, `.github/workflows/ci.yml`): the baseline command should match the CI leg's flags so the next entry ("Fork CI reached") reads against it.
- security Bootstrap `dep-audit-tooling-install` / `dep-security-ci-gate` ↔ the working route's "CI gate legs" entry, which owns the audit gate. This chunk only records that the gate is absent.

## Acceptance criteria contributions
- The recorded build command includes `--locked`, and `git diff --exit-code Cargo.lock Cargo.toml` is clean after the build and test run (per security-plan §Dependency Security, Pinning / Supply chain integrity).
- No dependency pin changed in the chunk's diff: `git diff` touches no `Cargo.toml`/`Cargo.lock` (per security-plan §Dependency Security, Pinning).
- The evidence folder `chunks/2026-10-05-as-built-baseline/` contains no secret values and no environment dump. A grep for `token|password|secret|API_KEY` over it finds no credential (per security-plan §Secret Management, Never in code).
