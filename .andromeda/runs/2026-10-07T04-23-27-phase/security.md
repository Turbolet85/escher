# security extract

## Relevance
partial — a measured no-op sync adds no input class, crossing, dependency, secret read or log emission; the security domain binds only through the dependency/supply-chain gates the chunk's "fork CI green" proof rests on, the unchanged-tree boundary, and the hygiene of the evidence it commits.

## Constraints
- The chunk's "no dependency or lock change" boundary is a security boundary too: security-plan §Dependency Security (Pinning) requires `Cargo.lock` committed, git dependencies pinned by `rev`, and `--locked` on every cargo leg. The chunk's commits must touch none of `Cargo.lock`, any `Cargo.toml`, or `deny.toml`. Whether the tree at /implement is in fact unchanged there is research's / implement's measurement, not this extract's.
- The dependency audit is part of the proof, not beside it: security-plan §Dependency Security (CI integration) requires ci.yml's `audit` job ("Dependency audit") to run `bash .github/scripts/ci-leg.sh audit` on every push, in the slow tier behind the four fast jobs. A "fork CI green on HEAD" verdict therefore has to include that job's conclusion; the scope's two local legs (`fast`, `doc`) are not the audit leg. Whether the CI run read at /implement shows the audit job concluded (not pending, not skipped) is research's question — the scope records the verdict as not yet available at take-up.
- The advisory state moves independently of the tree: security-plan §Dependency Security (Audit tool) describes the audit as reading the RustSec advisory database on every push, and the one standing ignore as "a bounded deferral the audit leg re-reads on every push". A zero-delta tree can therefore turn red on audit. A red audit cannot be fixed inside this chunk's boundary (the plan's remedy is a semver-compatible lockfile update or a per-ID ignore with a written reason — both are tree changes), so it is a halt for the operator, the same shape as the scope's "upstream moved" halt.
- The ignore set stays as the plan states it: security-plan §Dependency Security (Audit tool) requires per-ID ignores only, each with a written reason, no blanket allow, `unmaintained` and `unsound` at their defaults. This chunk adds, removes and rewords no ignore.
- The fork's CI trust posture is untouched: security-plan §Authentication & Authorization (RBAC / permissions rows) requires ci.yml's workflow-level `permissions: contents: read` with no job-level grant, and the `github.repository == 'DioxusLabs/blitz'` guard on the publish job; §Secret Management (Storage) ties the signing material and tokens to those guarded jobs. With nothing merged, no `.github/workflows/*` file changes. This is also why a moved upstream is a halt and never a silent merge: an upstream merge can carry workflow edits that have to be read against those guards.
- Committed evidence carries no credential: security-plan §Secret Management (What counts as secret; Never in code) requires that no artifact, event or verdict holds a secret. The measurement record (the `ls-remote` sha, the merge-base read, CI run ids and job conclusions, leg verdicts) must hold no token, auth header or credential-file content.
- Raw gate output is not evidence to commit: security-plan §Logging & Monitoring (Log format and backends) describes `target/ci-logs/` raw cargo output as unscrubbed and gitignored under `target/`. The chunk's evidence cites the leg verdict and counts; it does not copy the raw log into the tree.

## Patterns to follow
- Run each gate through the leg script, never a hand-built cargo line — security-plan §Supply chain integrity (Lockfile verification) lists every leg command as passing `--locked`; the leg script is what carries it.
- Record a measurement with its provenance, as the plan's own audit evidence does — security-plan §Dependency Security (Audit tool) cites its audit reading with the host and the date the advisory database was fetched. The no-op sync's record should carry the same: the sha read, when it was read, and the run id each verdict comes from.
- Treat the audit's reach as cargo-deny's resolved graph, not the whole lockfile — security-plan §Dependency Security (Audit tool) names two advisories in optional chains that never reach the gate. A green audit job is a statement about that graph only; the record should not word it as "no advisories in the lockfile".
- Keep the coupled-pin reading of the standing ignore — security-plan §Dependency Security (Audit tool) ties RUSTSEC-2026-0192 to the coupled winit pin, which is why it is deferred and not bumped. It stays deferred here.

## Anti-patterns to avoid
- Making a red audit green by a blanket allow, or by setting `unmaintained` / `unsound` to none — banned by security-plan §Dependency Security (Audit tool).
- A lone bump of one side of a coupled pin (the winit pin behind the standing ignore) to clear an advisory inside a chunk scoped to no dependency change — security-plan §Dependency Security (Audit tool) names that bump as the unreachable fix.
- Reading a pending or partial CI run as green for the security gate — security-plan §Dependency Security (CI integration) places the audit job in the slow tier, so it concludes after the fast jobs a partial read would show.

(security-plan §Security Anti-Patterns reads `NO RECORDED INTENT`; nothing is cited from it. The three above are drawn from §Dependency Security.)

## Contract bindings
- **CI security gate ↔ tests (CI integration):** the audit job is one job of the single ci.yml workflow whose verdict the chunk reads as its proof; the test plan's CI section owns the workflow's shape, security owns that job's meaning (security-plan §Dependency Security, CI integration).
- **Coupled pins ↔ architecture:** the standing advisory ignore depends on the coupled winit pin the architecture records; "coupled pins untouched" in the scope and "ignore unchanged" here are the same fact read from two domains (security-plan §Dependency Security, Audit tool).
- **Evidence hygiene ↔ obs:** no new log emission, so no new scrub obligation; the only tie is that raw leg logs stay under `target/` (security-plan §Logging & Monitoring).

## Acceptance criteria contributions
- The chunk's commits change no `Cargo.lock`, no `Cargo.toml`, no `deny.toml` and no file under `.github/workflows/` — a path-limited diff from the take-up HEAD to the chunk's last commit is empty (per security-plan §Dependency Security, Pinning · §Supply chain integrity, Lockfile verification).
- The CI run cited as the green verdict for HEAD shows the `audit` ("Dependency audit") job with conclusion success — concluded, not in progress and not skipped; if it is red, the chunk halts for the operator rather than editing the lockfile or `deny.toml` (per security-plan §Dependency Security, CI integration · Audit tool).
- `deny.toml`'s advisory ignores are the same set before and after the chunk, with no blanket allow and `unmaintained` / `unsound` at their defaults (per security-plan §Dependency Security, Audit tool).
- The committed measurement record and evidence hold no token, auth header or credential-file content, and no copy of a raw `target/ci-logs/` log (per security-plan §Secret Management, What counts as secret · §Logging & Monitoring, Log format and backends).
