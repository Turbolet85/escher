# Security Rules

Universal security rules for escher. No `paths:` frontmatter — this file loads on every turn, so it stays lean.
Source: `.andromeda/security-plan.md` (security tier 0; application threat model NOT YET MEASURED).

## Secrets
- Secrets live only in GitHub Actions secrets/vars (macOS signing key, Android keystore + passwords, Apple certificate, `WPT_GITHUB_TOKEN`, `GITHUB_TOKEN`); signing material written to disk in CI is removed in an `always()` step — keep that pairing. The jobs that reach them (publish, WPT, post-results) run only in `DioxusLabs/blitz` — keep the `github.repository` guard; a ref-only `if:` re-arms them on the fork.
- Source reads no secret: env reads are `WPT_DIR`, `PAINT_TREE_BENCH_HTML`, `HOME`, `CARGO_MANIFEST_DIR` only. Never commit `.env*`.

## Untrusted input (remote content, scripts, CLI)
- Remote HTML/CSS/images/fonts/sub-documents are untrusted: keep the recursion caps (`MAX_SUBDOCUMENT_DEPTH = 10`, `MAX_IMPORT_DEPTH = 16`) and the stale-iframe-response discard by request id.
- `blitz-net` reads `file:` URLs from disk with no path restriction and has no response-size cap or request timeout — do not route agent- or user-supplied URLs through it without a decision recorded in the security plan.
- The default `ScriptFetcher` accepts only `file:` and `data:`; same-origin/CORS checks are absent and every runtime event has `isTrusted = true`.
- HTML is parsed with scripting disabled; invalid CSS declarations are dropped, not errors — keep parse failures as typed `Result`s, never panics, on new input paths.

## Surfaces
- No served API, listener or auth exists. A new socket, port, IPC endpoint or credential path is an arch §Occupied Resources + security-plan amendment first.

## Dependencies
- The dependency audit is cargo-deny (`bash .github/scripts/ci-leg.sh audit`, config `deny.toml`): an advisory it fires on is fixed by a semver-compatible update, or ignored by ID with a written reason — never a blanket allow, never `unmaintained`/`unsound` set to none. Its reach is cargo-deny's resolved graph, which misses the paste and memmap2 advisories in optional chains — still review new crates by hand.
- `Cargo.lock` is committed; git deps are pinned by `rev`; builds pass `--locked` — keep all three.

## Session Additions
_This section is owned by `/andromeda-wrap-session`. setup-project preserves content added here on re-run. See `section-markers.md` for the convention._
