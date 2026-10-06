# security extract

## Relevance
partial — a measured no-op upstream sync (no merge, no source, dependency, lock or workflow edit) touches security only through the gates it must pass on the unchanged tree and through the guard that a non-empty upstream move is a halt, not a silent merge.

## Constraints
- The dependency audit leg must pass on the unchanged tree: `bash .github/scripts/ci-leg.sh audit` (cargo-deny, `--locked`, per-ID ignores only, no blanket allow, `unmaintained`/`unsound` at defaults) (per security-plan §Dependency Security, Audit tool + CI integration). The one standing ignore (RUSTSEC-2026-0192) is re-read by that leg; whether the lock still triggers any other advisory is the audit leg's answer, not this extract's.
- `Cargo.lock` stays committed and unchanged, git deps stay pinned by `rev`, and every cargo leg keeps `--locked`; a no-op sync must leave all three untouched (per security-plan §Dependency Security, Pinning + §Supply chain integrity, Lockfile verification).
- Upstream content is an external supply-chain channel: the ruling covers upstream 0 commits past the merge base only. If upstream has moved by /implement, the new commits are unreviewed third-party code and a non-empty merge is a different chunk shape, so a halt for the operator, with no security review implied by this chunk (per security-plan §Dependency Security, Supply chain integrity; scope §What it builds).
- If upstream ever moves and is merged by a later chunk, the repository guards (`github.repository == 'DioxusLabs/blitz'` on publish, WPT and post-results), the `always()` signing-material removal, ci.yml's workflow-level `permissions: contents: read` and the 40-hex SHA-pinned `uses:` must be re-asserted by `test_ci_workflows.py`; this chunk carries no workflow delta, so those are expected unchanged (per security-plan §Threat Model Summary, CI workflow vector; §Authentication & Authorization, CI token rows; §Dependency Security, CI tooling).
- No new network port, socket, listener, env var or credential path enters (the scope claims none); source env reads stay the list in the plan, and `RUST_LOG` stays the only telemetry-filter read (per security-plan §Secret Management, Environment values read; §API Security, no served API).
- No secret, token or host-path material may enter the committed evidence or report for this chunk (the measurement is a sha and a gate verdict only) (per security-plan §Secret Management, What counts as secret; §Logging & Monitoring, agent-run/cold-agent log notes).

## Patterns to follow
- Record the audit leg's verdict from its own log (`target/ci-logs/audit.log`) and the fork CI run id, as the prior chunk's audit evidence does (per security-plan §Dependency Security, audit reach note citing `escher-0.1.0/chunks/2026-10-05-ci-gate-legs/evidence/audit.md`).
- Read fork CI with `-R Turbolet85/escher`, never a bare `gh run` (the checkout carries an `upstream` remote), so the green verdict is the fork's (per project CLAUDE.md Session Learnings; per security-plan §Dependency Security, CI integration).
- Take the upstream measurement from `git ls-remote upstream refs/heads/main` and `git merge-base`, read-only, with no fetch-then-merge step that could write the tree (per scope §What it builds).

## Anti-patterns to avoid
- A silent or "just in case" merge of a moved `upstream/main` under the no-op ruling; the ruling covers 0 ahead only (per security-plan §Dependency Security, Supply chain integrity).
- Loosening `deny.toml` (a new blanket allow, `unmaintained`/`unsound` set to none) or editing `Cargo.lock` to make the audit green on a chunk that is meant to change neither (per security-plan §Dependency Security, Audit tool).
- Citing the audit leg as covering the whole lockfile: its reach is cargo-deny's resolved graph, which misses the `paste` and `memmap2` advisories in optional chains (per security-plan §Dependency Security, Audit tool).

## Contract bindings
- CI security gate binds to tests §CI Integration: the audit job runs in ci.yml's slow tier behind the fast jobs, so the chunk's "CI green 16/16" verdict includes it (per security-plan §Dependency Security, CI integration).
- Dependency pins bind to the architecture's coupled-pin rule (CLAUDE.md Critical Warnings): untouched by a no-op sync.

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh audit` passes on the unchanged tree and its log is cited; no new advisory ignore, no `deny.toml` change (per security-plan §Dependency Security, Audit tool)
- `git diff` of the chunk shows no change to `Cargo.lock`, any `Cargo.toml`, `deny.toml` or `.github/workflows/*`, and builds still pass `--locked` (per security-plan §Dependency Security, Pinning + §Supply chain integrity)
- Re-measured at /implement, `upstream/main` is still `23354585` and an ancestor of HEAD; any other read halts for the operator instead of merging (per security-plan §Dependency Security, Supply chain integrity; scope §What it builds)
- The chunk's committed evidence holds no credential, auth header, token or unmasked host path (per security-plan §Secret Management, What counts as secret; §Logging & Monitoring)
