# arch extract

## Relevance
relevant: the chunk moves (or declines to move) a locked dependency pin, the recorded upstream-sync base and the engine's layout code, all governed by the arch plan.

## Constraints
- The coupled pins (html5ever family with stylo web_atoms, skrifa with parley/vello, svgtypes with usvg, winit exact beta) move only as a set. The plan records taffy as a registry version, "0.15", with no git dependency left in the manifest. A take carrying upstream's git Taffy `rev` reverses this locked decision, and the record must be amended for it, not silently diverged from (per architecture §Established Decisions → [Dependency pinning]).
- The Layout row fixes taffy at registry "0.15" with its eight features (std, flexbox, flexbox_balance, grid, block_layout, content_size, calc, detailed_layout_info). Any Taffy move must keep that feature set or state the change in the row (per architecture §Stack and Technologies).
- Incremental and non-incremental layout run one pipeline and must produce identical layouts after any mutation sequence (`incremental_oracle`). A Taffy-main float-placement change (`place_floated_box` in `packages/blitz-dom/src/layout/inline.rs`) falls under this rule in both modes (per architecture §Design Philosophy → One incremental pipeline, safe identities).
- The sink drops every record whose target is outside `ENGINE_TARGET_PREFIXES` / `ESCHER_TARGET_PREFIXES`, and engine tracing is optional and gated per call site. Any `tracing`/`log` call site the upstream delta brings into an engine crate must follow that shape. Whether the delta adds one is research's question (per architecture §Cross-cutting Patterns → Logging and timing).
- The registered resources stay closed: one Unix-domain socket listener and no port, the listed env vars only (`RUST_LOG`, `WPT_DIR`, `PAINT_TREE_BENCH_HTML`, test-only `TMPDIR`, CI-script vars) and the listed crate names. A new one is a registration, not a silent add (per architecture §Occupied Resources).
- The MSRV job runs only `cargo build` at Rust 1.91. Whether a git Taffy builds on 1.91 is that job's reading (per architecture §Established Decisions → [MSRV]); upstream also makes an effort not to raise MSRV (per architecture §Design Philosophy → Radically modular, embeddable engine).
- The `Upstream sync:` line records `23b9b26a9` as the next sync's merge base, with the four Taffy-main commits waiting for it. It must move to the take's measured pin and merge commit, or record which commits are left again (per architecture §Project Intent).

## Patterns to follow
- Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`. A Taffy line change lands in the root manifest only (per architecture §Conventions → Manifests).
- Every gate runs as `bash .github/scripts/ci-leg.sh {leg}`, with `--locked` on every cargo leg but `examples/wasm_hello`. The same command runs on the dev host and in CI (per architecture §Infrastructure Patterns → CI/CD).
- The code-quality gates are fmt, workspace clippy `-D warnings`, upstream's blitz-dom clippy line, rustdoc `-D warnings` over every library crate, and the cargo-deny audit of advisories and licences over the whole workspace graph (per architecture §Inherited Defaults → Code quality).
- The seven PROVISIONAL engine changes are flagged upstreamable against `23b9b26a9`. Their by-file merge surface is owned by this entry's CARRY and is re-read against the take, not changed (per architecture §Established Decisions → [Scrolling and input], detail `dec-scrolling-and-input`).

## Anti-patterns to avoid
- Bumping one side of a coupled pin alone, or hand-fixing a pin set that does not resolve as a set (per architecture §Established Decisions → [Dependency pinning]).
- An unconditional print or ungated `tracing` call site in an engine crate (per architecture §Conventions → Feature gating).
- Landing a new crate, env var, listener or port without an §Occupied Resources registration (per architecture §Occupied Resources).

## Contract bindings
- arch ↔ tests: `BlitzShellProvider` implements `ShellProvider` for the cursor, among other things (per architecture §Standard Contracts → Paint and shell). blitz-tests takes `cursor-icon` as a dev-dependency for upstream's `pointer_leave` check (per architecture §Stack and Technologies, Testing row). Two things are research's question: whether any escher reader (snapshot, driver, stand, harness) consumes `get_cursor`, and whether `pointer_leave` covers `fb8d6c15e`'s `Default`.
- arch ↔ obs: the sink allowlist and per-call-site gating (per architecture §Cross-cutting Patterns → Logging and timing) are what the entry's "sink allowlist re-checked" reads the upstream delta against.
- arch ↔ security: the no-git-dependency decision (per architecture §Established Decisions → [Dependency pinning]) is mirrored by the security rules' Dependencies section. The cargo-deny audit covers advisories and licences only (per architecture §Inherited Defaults → Code quality), so the rule against a git source rests on the decision, not the tool.
- arch ↔ tests (incremental oracle): layout equality across both modes (per architecture §Design Philosophy → One incremental pipeline, safe identities) binds the stand checks that run `for incremental in [false, true]`.

## Acceptance criteria contributions
- (arch) The root manifest still holds taffy as registry "0.15" with no git dependency. If not, the [Dependency pinning] decision and the Layout row are amended at the wrap, naming the git `rev`, the reversal and its delegate (PROVISIONAL) answer (per architecture §Established Decisions → [Dependency pinning]).
- (arch) The `Upstream sync:` line names the measured upstream sha taken and the merge or cherry-pick commit, or records a measured no-op, and lists the commits left for the next sync (per architecture §Project Intent).
- (arch) No new workspace crate, env var, listener or port exists after the take (per architecture §Occupied Resources).
- (arch) The MSRV build (Rust 1.91, `cargo build`) is green on the take's pushed CI run (per architecture §Established Decisions → [MSRV]).
