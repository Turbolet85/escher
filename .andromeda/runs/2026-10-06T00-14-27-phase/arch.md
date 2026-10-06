# arch extract

## Relevance
relevant — the chunk adds a workspace unit, process-wide state (global subscriber, panic hook), env switches, an optional outbound host and new dependencies, all of which arch governs for placement, registration and manifest discipline.

## Constraints
- **Resource registration before landing.** A new crate name, each new env var (filter level, OTel opt-in, endpoint), the OTel endpoint as an outbound host, and the global subscriber/panic hook as process-wide state must each be registered. Per architecture §Occupied Resources → Names / Environment variables / Outbound hosts / Process-wide state and threads. §Occupied Resources → Network ports and listeners requires the workspace to stay at "none", so the exporter may only make outbound connections and must never bind a listener. The service identity is read at compile time from the binary, which adds no env read. Whether the code already reads `RUST_LOG` through an `EnvFilter` is research's question; if it does not, it is a new registration.
- **Manifest discipline.** Per architecture §Conventions → Manifests, each new dependency (`opentelemetry*`, `tracing-opentelemetry`, any `tracing-subscriber` features) is declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`. Package metadata is inherited with `*.workspace = true`, and in-repo crates are referenced with `default-features = false`. The crate's tracing stack must reuse the workspace `tracing "0.1.40"` / `tracing-subscriber "0.3"` versions, per §Stack and Technologies → Parallelism and misc.
- **Coupled pins.** Per architecture §Established Decisions → Dependency pinning, coupled pins must never move one side alone. The OTel transport stack must also resolve against the workspace's existing `tokio "1.42"`, `reqwest "0.13"` and `http "1.1.0"` (§Stack and Technologies → Networking and async). Builds stay `--locked` with `Cargo.lock` committed (§Infrastructure Patterns → CI/CD).
- **Optional capabilities are Cargo features.** Per architecture §Inherited Defaults → Optional capabilities and §Cross-cutting Patterns → Config management, optional capabilities are compile-time Cargo features forwarded across crates. OTel export therefore belongs behind an off-by-default feature, separate from the runtime opt-in switch. Engine call sites keep the `#[cfg(feature = "tracing")]` + no-op-fallback form (§Conventions → Feature gating).
- **Embeddable-engine boundary.** Per architecture §Design Philosophy ("Radically modular, embeddable engine"; "Embedder services are injected"), engine library crates (blitz-dom, blitz-net, blitz-paint, …) are embedded and driven by external code. Installing a global subscriber or panic hook is therefore an embedder (binary) act and belongs in an escher-owned unit, not in an engine crate. Library crates live under `packages/` per §Infrastructure Patterns → Directory structure.
- **Done-gate legs.** Per architecture §Infrastructure Patterns → CI/CD and §Stack and Technologies → Code quality:
  - Rustdoc `-D warnings` covers every workspace library crate (`doc` leg), so the new crate's public items need `///` and its modules `//!` (§Conventions → Documentation).
  - Every new dependency must pass the `audit` leg (cargo-deny over `deny.toml`).
  - `fmt`, `clippy -D warnings` and `test` apply as usual.
- **Current wiring the bootstrap replaces.** Per architecture §Cross-cutting Patterns → Logging and timing, the as-built record is:
  - `tracing_subscriber::fmt::init()` runs only under each app's `tracing` feature, and the record lists browser, rdme and todomvc.
  - The WPT runner uses `log` + `env_logger`.
  - The record does not list seven_guis. Whether `seven_guis_native` installs any subscriber today is research's question.

## Patterns to follow
- **Panic hook chaining.** The WPT runner installs a process-wide panic hook that stashes the message, location and backtrace (architecture §Infrastructure Patterns → WPT runner execution). It is the in-repo precedent for a process-wide hook. The new hook must chain to the previous hook rather than replace it.
- **Native vs wasm split.** seven_guis has a wasm32 entry that uses `#[wasm_bindgen(start)]` + `console_error_panic_hook` (architecture §Infrastructure Patterns → Deployment model). `tracing-wasm` is the workspace's wasm tracing dependency (§Stack and Technologies → WASM). The bootstrap targets the native binary and must leave the wasm entry building, since CI builds seven_guis as a wasm cdylib (§Infrastructure Patterns → CI/CD).
- **Runtime context.** dioxus-native enters a tokio multi-thread runtime with `net` off wasm32, but only inside `launch` (architecture §Infrastructure Patterns → Launch paths). A bootstrap called before `launch` therefore runs with no ambient tokio runtime, and an OTel batch exporter must not assume one.
- **Typed errors.** Errors are enums with `Display` and `From` per wrapped error (architecture §Conventions → Error handling; §Cross-cutting Patterns → Error handling). An init that can fail, such as an exporter build, returns a typed `Result`, not a panic.
- **Test placement.** Unit tests sit in a trailing `#[cfg(test)] mod tests`. Integration tests are one file per behaviour in `tests/blitz-tests`, opening with a `//!` doc that names the behaviour (architecture §Conventions → Tests).

## Anti-patterns to avoid
- An engine library crate installing a global subscriber or panic hook, or taking an unconditional `tracing`/OTel dependency (architecture §Design Philosophy; §Conventions → Feature gating).
- A dependency declared outside `[workspace.dependencies]`, or a transitive bump that moves one side of a coupled pin (architecture §Conventions → Manifests; §Established Decisions → Dependency pinning).
- Any of these landing silently without an §Occupied Resources entry: a new crate, an env var, an outbound host, or any listener (architecture §Occupied Resources).

## Contract bindings
- **arch ↔ security.** The OTel endpoint is a new outbound host (architecture §Occupied Resources → Outbound hosts) and pairs with a security-plan amendment, as the scope notes. New crates also bind to the security rules' cargo-deny + hand-review discipline through the `audit` leg (§Infrastructure Patterns → CI/CD).
- **arch ↔ obs.** Arch owns where the bootstrap lives and its resource registration. obs-plan owns what it emits: identity, scrub field list, sinks. The as-built logging row (architecture §Cross-cutting Patterns → Logging and timing) is amended alongside obs-plan §3/§8 at wrap.
- **arch ↔ tests.** The new crate joins the workspace `test` and `doc` legs (architecture §Standard Contracts → CI contracts). Arch records no Driver CLI / MCP stdout contract yet (§Standard Contracts), so the "stdout reserved" rule comes from the working route, not from arch.

## Acceptance criteria contributions
- (arch) The bootstrap lives in one escher-owned unit, not in any engine crate. A new crate under `packages/` is a workspace member whose dependencies are all `{ workspace = true }` entries of `[workspace.dependencies]` (per architecture §Conventions → Manifests; §Design Philosophy).
- (arch) In the default build (OTel feature off), the `seven_guis_native` dependency graph contains no `opentelemetry*` crate, and the workspace still binds no port or listener (per architecture §Inherited Defaults → Optional capabilities; §Occupied Resources → Network ports and listeners).
- (arch) Every new crate name, env var, outbound host and piece of process-wide state the chunk lands is named for registration in §Occupied Resources at wrap (per architecture §Occupied Resources).
- (arch) The following all pass with `Cargo.lock` committed and no coupled pin moved:
  - `bash .github/scripts/ci-leg.sh fast`
  - `doc` and `audit` legs
  - the seven_guis wasm cdylib build

  (per architecture §Infrastructure Patterns → CI/CD; §Established Decisions → Dependency pinning)
