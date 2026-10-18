# security extract

## Relevance
partial — the chunk adds no input surface, carrier or listener; it bears on dependency pinning and the audit gate, which the Taffy fork decides, and on re-checking the log sink against upstream's delta.

## Constraints
- Per security-plan §Dependency Security (Pinning), the root manifest must hold no git dependency: Taffy is pinned as the registry "0.15" with a checksum in `Cargo.lock`. Merging `fb8d6c15e` brings in upstream's git `rev` Taffy, which reverses that rule. It is the founder's fork, taken here as a delegate answer marked PROVISIONAL (scope inputs#I1). Whichever way it goes, the Pinning bullet must be restated to match the take.
- Per security-plan §Dependency Security (Pinning, the 2026-10-10 / 2026-10-13 / 2026-10-17 sync bullets), a sync takes upstream's dependency set whole with no `cargo update`. The lock's `name@version` set must equal upstream's at the pin, with the two `escher-*` entries set aside. Every package that enters the lock must be reviewed by hand for licence, repository, build script, proc-macro flag and direct dependent. Whether a git Taffy at `ea07c4c3` brings in or moves any package is research's question.
- Per security-plan §Dependency Security (Audit tool), `cargo deny --locked --workspace --all-features check advisories licenses` must pass over the whole workspace's graph. An advisory may be ignored only by ID with a written reason: no blanket allow, and `unmaintained` / `unsound` stay at their defaults. The eight standing ignores are re-read on the merged lock.
- Per security-plan §Supply chain integrity (Lockfile verification), every ci.yml cargo leg must keep building `--locked` on the merged tree, and `Cargo.lock` stays committed.
- Per security-plan §Logging & Monitoring (Log format and backends), escher's sink admits engine targets (safe fields only) and `escher_*` targets (content-named fields redacted), and drops every other target whole at every level. Each new `tracing` / `log` call site, target or print in upstream's delta must be read against that allowlist. Whether the two blitz-dom hunks add any is research's question.
- Per security-plan §Secret Management (Environment values read), source must read no env var beyond the listed set. Upstream's delta must add none, and whether it does is research's question.
- Per security-plan §Input Validation (the `@font-face` source row and the `aria-describedby` · `aria-description` row), a new read of untrusted page input, or a new exit for one, that arrives by upstream merge is a boundary widening, ratified only as upstream's behaviour on the founder's word. Whether `fb8d6c15e`'s `get_cursor` change reads or carries out anything new is research's question, and if it does it goes to `founder-forks.md`.

## Patterns to follow
- Record the take's dependency delta the way earlier syncs did. Per security-plan §Dependency Security (Pinning), the last one is measured at `escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/evidence/lock.md`: packages entering, leaving and moving, `name@version` set equality, and the audit leg's `advisories ok` / `licenses ok` with the `deny.toml` delta.
- Handle external decay the way security-plan §Dependency Security (Audit tool) records the zip and zbus-family ignores. If an advisory turns red over an unchanged lock, ignore it by ID with its reason and its reach, owned by a `.andromeda/residuals.md` line, citing the red run and the green one.
- Re-check the sink with the standing capture checks named in security-plan §Logging & Monitoring: `stand_act_spans`, `host_log` and `mcp_log`, whose typed-sentinel, id, name and base64 needles read 0 at `RUST_LOG=trace` under both builds. A changed line count is re-measured and explained, as the 2026-10-14-stand-requirement-sweep re-read did.
- A ratified merge widening is recorded the way security-plan §Input Validation's `@font-face` row records one: as upstream's behaviour, with the ratifying word and the check that covers it.

## Anti-patterns to avoid
- Never silence a red audit with a blanket allow or by setting `unmaintained` / `unsound` to none (per security-plan §Dependency Security, Audit tool).
- Never let a merged log site print a URL, attribute value, typed text, id or name from a target the sink admits (per security-plan §Logging & Monitoring).

## Contract bindings
- Dependency audit ↔ tests/CI: ci.yml's `audit` job runs `bash .github/scripts/ci-leg.sh audit` on every push (security-plan §Dependency Security, CI integration). The fork CI green the chunk requires includes it. `deny.toml` has no `[sources]` table (scope), so a git Taffy is caught only by our rule, never by the audit tool.
- Sink allowlist ↔ obs-plan sink records and the tests' capture checks (`host_log`, `mcp_log`, `stand_act_spans`) (security-plan §Logging & Monitoring).
- A new env var, crate, port or listener ↔ arch §Occupied Resources: registered before it lands (security-plan §Secret Management, Environment values read). Scope expects none.

## Acceptance criteria contributions
- (security) `bash .github/scripts/ci-leg.sh audit` reads `advisories ok` and `licenses ok` on the merged lock. Any `deny.toml` change is a per-ID ignore with a written reason, and nothing else (per security-plan §Dependency Security).
- (security) The root manifest holds no git dependency, or the delegate's PROVISIONAL reversal is recorded and the Pinning bullet restated with the git rev (per security-plan §Dependency Security, Pinning).
- (security) The take's evidence records the lock's `name@version` set against upstream's at the pin, `escher-*` set aside, and a hand review of every package that enters (per security-plan §Dependency Security, Pinning).
- (security) Upstream's delta is read and counted for `tracing` / `log` / print / env-read lines, recorded as a number. `stand_act_spans`, `host_log` and `mcp_log` stay green at 0 needles (per security-plan §Logging & Monitoring; §Secret Management).
