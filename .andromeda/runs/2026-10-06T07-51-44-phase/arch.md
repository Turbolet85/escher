# arch extract

## Relevance
relevant — the merge moves coupled dependency pins, rewrites the workspace root manifest and lockfile, and must keep the workspace gates, the occupied-resource registry and the escher-owned contracts as the plan specifies.

## Constraints
- Coupled pins move together or not at all. Per arch §Established Decisions [Dependency pinning], the markup5ever/html5ever/xml5ever versions must match stylo's web_atoms, skrifa must match the parley and vello versions, and svgtypes must match usvg. taffy and parley are git dependencies pinned by `rev`, and winit is pinned exactly to `=0.31.0-beta.3`. Per arch §Inherited Defaults (Framework), Stylo 0.22.0, anyrender 0.14.0, Dioxus 0.7.3 and the winit beta are part of the inherited set. Upstream's taffy and parley rev moves enter as one set. Whether upstream's parley rev `e41dfea5` still resolves skrifa within the 0.44 line shared with vello in Cargo.lock is research's question.
- Two rules govern the workspace root manifest. Per arch §Conventions (Manifests), dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`, and in-repo crates are declared `default-features = false`, except the `seven_guis` path entry, which keeps its default renderer feature. Per arch §Occupied Resources (Names), `escher-telemetry` (workspace member plus path entry) and `seven_guis` (path entry, dev-dependency of blitz-tests) are registered names. The `Cargo.toml` overlap resolution must keep both.
- Gates hold over the merged tree. Per arch §Stack and Technologies (Code quality) and §Inherited Defaults (Code quality), fmt and `clippy --workspace --locked -D warnings` must pass, rustdoc `-D warnings` is enforced over every workspace library crate through `cargo doc --workspace --no-deps --locked`, and the cargo-deny advisory audit runs over the root `deny.toml`. Per arch §Infrastructure Patterns (CI/CD), every cargo leg runs `--locked` except `examples/wasm_hello`. Whether upstream's new rustdoc in `node.rs`/`document.rs` passes the `doc` leg is research's question.
- One incremental pipeline. Per arch §Design Philosophy ("One incremental pipeline, safe identities"), incremental and non-incremental layout must stay identical after any sequence of mutations, the paint tree holds topology only, and public APIs use the versioned `NodeId`. Upstream's Taffy, align-content, abspos self-alignment and inline-fragment changes all land inside that pipeline.
- No silent resource registration. Per arch §Occupied Resources (Network ports and listeners · Environment variables · Names), there is no listener, the env-var set is closed, and the crate, binary and feature names are registered. A merge that brought in a new crate, env var, port or feature name would be an §Occupied Resources amendment, not a carry. Whether upstream's 11 commits add any is research's question.
- The escher-owned contracts survive as specified. Per arch §Standard Contracts, these contracts must read the same after the merge: Test harness (the eight `HarnessOptions` fields and the inspection/input API), Headless stand (`seven_guis::stand`), Telemetry bootstrap (`escher_telemetry::init`), the Dioxus DOM bridge (the falsy `checked`/`disabled` clear) and the CI contracts (`ci-leg.sh` legs, `agent-run.sh`, `cold-agent.sh`). Upstream's additions to the Document core and Nodes surfaces (a `children` accessor, `Node::inline_fragment_boxes`) arrive as upstream-owned API and are not adopted here.
- The fork's CI guards stay put. Per arch §Infrastructure Patterns (CI/CD), the WPT, post-results and publish workflows run only on `DioxusLabs/blitz` (a repository guard on every job), and CI triggers on `build/**` pushes. Upstream's WPT pin bump (`wpt/WPT_COMMIT`) must not re-arm those jobs on the fork.

## Patterns to follow
- Resolve manifest overlap in place. Keep one `[workspace.dependencies]` declaration per crate, grouped under comment headers, and keep in-repo entries `default-features = false` (except `seven_guis`), per arch §Conventions (Manifests).
- Prove green through the leg runner. The dev host and CI run the same `bash .github/scripts/ci-leg.sh {leg}`: `fast` for fmt → clippy → test → ci-scripts, then `doc` and `audit`. Logs go to `target/ci-logs/{leg}.log`, per arch §Standard Contracts (CI contracts) and §Infrastructure Patterns (CI/CD).
- Any strictly required rustdoc fix must stay minimal and follow arch §Conventions (Documentation): `//!` module docs, `///` on public items, and intra-doc links that resolve. Record it as non-additive.
- Any engine-side fix that touches tracing gates it per call site with `#[cfg(feature = "tracing")]` and a `#[cfg(not(feature = "tracing"))]` no-op fallback, per arch §Conventions (Feature gating).

## Anti-patterns to avoid
- Do not bump or hand-edit one side of a coupled pin (for example, nudging skrifa or vello to make parley resolve). Do not regenerate Cargo.lock beyond what the merge needs, and do not drop `--locked`. All three are banned by arch §Established Decisions [Dependency pinning] and §Inherited Defaults (Code quality).
- Do not add a crate, env var, port, listener or feature name without registering it in §Occupied Resources, per arch §Occupied Resources.
- Do not weaken or remove a `github.repository` guard on the upstream-only workflows, per arch §Infrastructure Patterns (CI/CD).

## Contract bindings
- The pins in arch §Established Decisions [Dependency pinning] bind to security §Dependencies: new taffy and parley git revs enter the cargo-deny graph (the `audit` leg), and `Cargo.lock` stays committed and `rev`-pinned.
- arch §Standard Contracts (Test harness · Headless stand) binds to tests: the `stand_*` checks, the `agent-run.sh`/`cold-agent.sh` contract tests in the `ci-scripts` leg, and `incremental_oracle`.
- arch §Standard Contracts (Telemetry bootstrap) and §Conventions (Feature gating) bind to obs: the `telemetry_*` tests, and upstream tracing call sites carried gated per call site.
- arch §Standard Contracts (Dioxus DOM bridge, falsy `disabled`) binds to a11y focusability through `dioxus_falsy_disabled` and the `a11y` leg.

## Acceptance criteria contributions
- After the merge, `Cargo.toml` still lists `escher-telemetry` as a workspace member, with `escher-telemetry` and `seven_guis` as `[workspace.dependencies]` path entries. The diff adds no crate, env var, port, listener or feature name (per arch §Occupied Resources).
- taffy and parley stay git dependencies pinned by `rev` at upstream's revs, taken together. skrifa in `Cargo.lock` matches the version parley and vello resolve. The html5ever family is unchanged against stylo's web_atoms. `cargo build --workspace --locked` exits 0 (per arch §Established Decisions [Dependency pinning]).
- `bash .github/scripts/ci-leg.sh fast`, `bash .github/scripts/ci-leg.sh doc` and `bash .github/scripts/ci-leg.sh audit` each exit 0 on the merge commit (per arch §Stack and Technologies, Code quality, and §Infrastructure Patterns, CI/CD).
- `incremental_oracle` passes after the merge (per arch §Design Philosophy, "One incremental pipeline, safe identities").
