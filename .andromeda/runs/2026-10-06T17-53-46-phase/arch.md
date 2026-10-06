# arch extract

## Relevance
partial — a measured no-op upstream sync with zero source delta: arch binds only through the sync-pin record, the unchanged-resource and dependency-pin invariants, and the CI gate legs.

## Constraints
- The sync baseline is recorded in the architecture's own §Project Intent "Upstream sync" line (upstream `main` last merged at `23354585…`, the next sync's merge base). That line requires the next sync to start from that sha; whether the chunk's measurement (merge-base = upstream/main, 0 ahead) still matches it at /implement is research's question, not something this plan attests (per .andromeda/architecture.md §Project Intent).
- No new crate, port, listener, env var, filesystem path, thread or outbound host may land. A no-op sync with no source edit must leave every §Occupied Resources entry byte-identical, and a new resource there is an arch registration, never a silent add. The plan records no network listener, and `RUST_LOG`, `WPT_DIR` and `PAINT_TREE_BENCH_HTML` as the only env reads (per .andromeda/architecture.md §Occupied Resources, §Inherited Defaults "API style").
- Coupled dependency pins are locked: markup5ever/html5ever/xml5ever with the stylo web_atoms version, skrifa with parley and vello, svgtypes with usvg, taffy and parley as git deps pinned by `rev`, winit exactly `=0.31.0-beta.3`. The chunk must not touch `Cargo.toml` or `Cargo.lock`. Were upstream to move, a bump of one side alone would violate this (per .andromeda/architecture.md §Established Decisions "Dependency pinning", §Inherited Defaults "Framework").
- The gate legs are the ones CI runs. Each linux job runs `bash .github/scripts/ci-leg.sh {leg}`, the same command the dev host runs, with `--locked` on every cargo leg but `examples/wasm_hello`. The plan names fmt, clippy `-D warnings`, test, ci-scripts, and docs (`cargo doc --workspace --no-deps --locked`), plus audit, a11y, coverage and the matrix as CI jobs. The local gate verdict on an unchanged tree is read through those legs (per .andromeda/architecture.md §Infrastructure Patterns "CI/CD", §Inherited Defaults "Code quality").
- CI triggers on push to `build/**` with per-ref concurrency and cancel-in-progress. The fork's own CI is the proof surface for "our logic survived". The WPT, post-results and publish jobs run only on `DioxusLabs/blitz`, so a fork green run does not exercise them (per .andromeda/architecture.md §Infrastructure Patterns "CI/CD", "Deployment model").
- If upstream has moved by /implement, the plan's merge discipline applies. Upstream README changes resolve to escher's `README.md`. Pin discipline and additivity are arch concerns for that merge. A non-empty merge is a different chunk shape and arch's §Project Intent sync line would then need amending at wrap (per .andromeda/architecture.md §Project Intent "Front page", "Upstream sync").

## Patterns to follow
- Record the upstream pin as a measured fact with its method (merge-base against `upstream/main`, `git ls-remote`), the same way §Occupied Resources entries carry "as measured at {evidence path}" provenance (per .andromeda/architecture.md §Occupied Resources).
- Take every gate verdict from `bash .github/scripts/ci-leg.sh {leg}`, never an ad-hoc cargo invocation. Logs land in `target/ci-logs/{leg}.log`, the only path the CI failure artifacts upload (per .andromeda/architecture.md §Occupied Resources "Filesystem", §Infrastructure Patterns "CI/CD").
- Keep the one-line `Upstream sync:` anchor in §Project Intent as the single authority for the next merge base. Change it only when a merge actually lands. A no-op sync leaves it as it stands (per .andromeda/architecture.md §Project Intent).

## Anti-patterns to avoid
- No silent merge of a later upstream sha and no merge commit under a "no-op" label. If upstream moved, halt for the operator (per .andromeda/architecture.md §Project Intent "Upstream sync", §Established Decisions "Dependency pinning").
- No lockfile or manifest churn. A `Cargo.lock` change re-mints every GitHub Actions cache key and evicts against the 10 GB budget, so even an incidental `cargo update` is a cost, not only a pin risk (per .andromeda/architecture.md §Occupied Resources "CI infrastructure", §Established Decisions "Dependency pinning").
- No adopting of upstream APIs and no new workspace crate, port or env var as part of this chunk (per .andromeda/architecture.md §Occupied Resources "Names", "Environment variables").

## Contract bindings
- arch ↔ tests/CI: the proof of survival is the fork CI run (16 checks) plus the local `ci-leg.sh fast` and `doc` legs. The legs themselves are defined by §Infrastructure Patterns "CI/CD". Their pass criteria are tests-domain content.
- arch ↔ security: the unchanged-pins and no-new-resource invariants overlap the dependency audit (cargo-deny leg `audit`). Its verdict is security's to read, not arch's.

## Acceptance criteria contributions
- The upstream pin re-measured at /implement equals the sha recorded in the architecture's `Upstream sync:` line, and that sha is an ancestor of HEAD. If it differs, the chunk halts for the operator (per .andromeda/architecture.md §Project Intent).
- The tree diff of the chunk touches no `Cargo.toml`, `Cargo.lock`, source file or workflow, so §Occupied Resources and the pin set in §Established Decisions "Dependency pinning" are unchanged (per .andromeda/architecture.md §Occupied Resources, §Established Decisions).
- `bash .github/scripts/ci-leg.sh fast` and `bash .github/scripts/ci-leg.sh doc` both pass on the unchanged tree, and the fork's CI on HEAD is green (per .andromeda/architecture.md §Infrastructure Patterns "CI/CD", §Inherited Defaults "Code quality").
