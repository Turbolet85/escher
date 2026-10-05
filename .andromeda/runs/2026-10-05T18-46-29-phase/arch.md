# arch extract

## Relevance
partial: the chunk touches no source crate. Arch applies through the toolchain, the coupled dependency pins, the build environment, the CI build form and the workspace placement of `tests/blitz-tests`.

## Constraints
- Toolchain baseline: per arch §Stack and Technologies (Language / runtime row) and §Inherited Defaults → Framework, the workspace is edition "2024" with rust-version "1.91.0". Per arch §Stack and Technologies → Build environment, the Nix flake pins rust-bin stable "1.90.0" under a "keep in sync with `rust-version`" comment. The host record must name the rustc/cargo actually used, measured against 1.91.0. The flake/MSRV mismatch is a fact to record, not to fix here.
- Coupled pins are frozen: per arch §Established Decisions [Dependency pinning], html5ever/markup5ever/xml5ever must match stylo web_atoms, skrifa must match parley/vello, svgtypes must match usvg, taffy and parley are git deps pinned by `rev`, and winit is pinned exactly to `=0.31.0-beta.3`. The baseline reads `Cargo.toml`/`Cargo.lock` and must leave both unchanged.
- Build-time system inputs: per arch §Stack and Technologies → Build environment, the build needs openssl, fontconfig (Linux), pkg-config and python3, which is "required at build time to generate code for `stylo`". Per arch §Infrastructure Patterns → CI/CD, Ubuntu CI installs `libfontconfig1-dev`. The host-facts record names this Arch host's equivalent packages. Whether each one is present is research's question.
- CI build form differs from a plain local build: per arch §Infrastructure Patterns → CI/CD, the Ubuntu jobs rewrite `opt-level = 2` to `0` before building, and the Linux matrix leg tests with `--all --tests`. Per arch §Established Decisions [MSRV], the MSRV job runs only `cargo build`. Research must settle whether the local baseline mirrors the rewrite and must record which form was run, because the wall-clock is only comparable to the next entry ("Fork CI reached") if the form is stated.
- Build profile: per arch §Established Decisions [Build profiles], the workspace defines several profiles, and the default dev profile is not `production`/`p2`. The wall-clock figure must state the profile it was taken under.
- Workspace placement: per arch §Existing Scopes (blitz-tests row) and §Infrastructure Patterns → Directory structure, the suite is the `tests/blitz-tests` member. Per arch §Inherited Defaults → Publishability it is `publish = false`. The chunk's evidence goes in `escher-0.1.0/chunks/…`, outside every workspace member.
- No new occupied resource: per arch §Occupied Resources (Network ports and listeners: none; Environment variables; Names), the run adds no port, env var, crate or binary. `PAINT_TREE_BENCH_HTML` is an existing optional env var that the `paint_tree_bench` test reads (§Occupied Resources → Environment variables), so its set/unset state belongs in the host record. The test base URLs `http://example.com/` / `https://example.com/` are registered outbound hosts (§Occupied Resources → Outbound hosts). Whether any blitz-tests test actually reaches the network, which affects reproducibility, is research's question.

## Patterns to follow
- Headless execution: per arch §Design Philosophy → "Headless, measurable, testable", the harness needs no window, GPU or compositor, so the suite should run on this host without a display session. Whether every blitz-tests file actually holds to that is research's question.
- Phase-split timing: per arch §Cross-cutting Patterns → Logging and timing, `paint_bench` separates phases and runs warmup iterations, and `screenshot` times each phase. Follow the same discipline for the baseline: separate build and test figures, each labelled cold or warm with a stated basis.
- Both layout modes: per arch §Conventions → Tests and §Design Philosophy → "One incremental pipeline, safe identities", pipeline tests run `for incremental in [false, true]` (the incremental_oracle invariant). Run the suite as-is. A failure in only one mode is recorded as such.
- One file per behaviour: per arch §Conventions → Tests, each integration file opens with a `//!` doc that names its behaviour. A red is recorded against its file name and that doc's behaviour.

## Anti-patterns to avoid
- Turning a red green by bumping one side of a coupled pin, or by unpinning a git `rev` (arch §Established Decisions [Dependency pinning]).
- Committing a measurement-time edit to `Cargo.toml` or `Cargo.lock`, such as CI's `opt-level` rewrite (arch §Infrastructure Patterns → CI/CD). The baseline reads the manifests and does not change them.
- Adding a measurement crate, script, env var or default-feature change to get a figure (arch §Occupied Resources → Names / Environment variables; §Established Decisions [Default features]). That tooling belongs to later Foundation entries.

## Contract bindings
- arch ↔ security: the frozen pins and git `rev`s (arch §Established Decisions [Dependency pinning]) bind to the security rule that builds pass `--locked` and that `Cargo.lock` is committed.
- arch ↔ tests: the suite's composition and harness API (arch §Existing Scopes → blitz-tests; §Standard Contracts → Test harness) bind to the test plan, which owns the suite command and how a red is classified.
- arch ↔ obs: `log-frame-times`/`log-phase-times` and `debug_timer` `enable` are opt-in features (arch §Cross-cutting Patterns → Logging and timing; §Occupied Resources → Names). Taking the baseline with default features keeps the wall-clock free of timing instrumentation. Obs owns any phase-timing figure.

## Acceptance criteria contributions
- The workspace build (`cargo build --workspace --locked`, final form confirmed by research) exits 0 on this host, and `git diff --quiet -- Cargo.toml Cargo.lock` holds afterwards (per arch §Established Decisions [Dependency pinning]).
- The host-facts record names the rustc/cargo versions used against rust-version "1.91.0" / edition "2024", notes the flake's "1.90.0" pin, and names the host packages standing in for python3, fontconfig, openssl and pkg-config (per arch §Stack and Technologies → Build environment).
- The wall-clock record states the build profile and whether CI's `opt-level = 2 → 0` rewrite was applied, so the figure can be compared with the fork-CI entry (per arch §Infrastructure Patterns → CI/CD; §Established Decisions [Build profiles]).
- The run adds no workspace member, env var, port or binary: `git status` shows changes only under the chunk folder and run dirs (per arch §Occupied Resources).
