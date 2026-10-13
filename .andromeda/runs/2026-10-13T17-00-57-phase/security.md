# security extract

## Relevance
partial. The chunk is an in-process, read-only test harness on the offline stand. It opens no surface, so security applies only to what it must not add: a crossing, an env read, a log field, an unreviewed dependency or a panic path.

## Constraints
- The harness opens no listener, socket, port or IPC endpoint. Per security-plan §API Security, the driver session's Unix-domain socket is the one listener, and the `mcp` role binds nothing. A new one is a registration in arch §Occupied Resources and an amendment to the security plan, never a silent add. The measurement runs in process on `seven_guis::stand::boot` and sends nothing across a process boundary.
- The harness reads no environment variable beyond those security-plan §Secret Management lists under "Environment values read (none secret-bearing)". It reads no secret, and the per-pair record holds none.
- No URL reaches blitz-net, which reads `file:` URLs with no path restriction and has no size cap or timeout (per security-plan §API Security, the "Request size limit" and "Request timeout" rows). Per security-plan §Input Validation, the `@font-face` source row records that "escher routes no agent- or user-supplied URL through this path" and that the stand boots offline on its bundled font. The harness keeps that boot and adds no fetch.
- A per-pair result must not travel through a `tracing` event or span field:
  - Per security-plan §Input Validation (the `id` row), escher's own code writes the stable id to no log.
  - Per security-plan §Input Validation (the `aria-label` · `<label for>` row), escher's own code logs no name.
  - Per security-plan §Logging & Monitoring, escher's sink admits fixed fields only, and new user-content log fields are not added.
  - A record printed by a test or written as committed evidence is test output, not a log line. Whether the chunk emits any record through `tracing` at all is research's question.
- The node walk resolves every `NodeId` through `get`, never by index. Per security-plan §Error Handling (Graceful degradation), a stale node id resolves to `None` through `get`.
- The walk does not call the engine's panicking accessors on nodes that lack the field. Per security-plan §Error Handling (Panic paths):
  - `layout_style()` panics on a node with no computed styles.
  - `layout_data` and `guard` panic on node kinds that lack the field.

  The colour and background walk up the ancestor chain to the canvas crosses text, anonymous and document nodes. Whether the chosen reader already guards this is research's question.
- A new crate is reviewed by hand, and the audit leg reads `advisories ok` (per security-plan §Dependency Security). A colour or WCAG crate is the likely candidate. Builds keep `--locked` (per security-plan §Dependency Security, Pinning). The audit's reach is cargo-deny's resolved graph only, so hand review is still owed for each crate the lock gains.

## Patterns to follow
- Masked paint on the raster source. Per security-plan §Input Validation (the password and file `input` row), blitz-paint paints a password `input` masked in every document. If P4 picks `Harness::screenshot_rect`, the raster carries no typed password text. The record then holds colours, a ratio, a font size and weight and the stable id, and no control value or accessible name.
- Typed, non-panicking failure. Per security-plan §Error Handling (Graceful degradation), query APIs return `Result<Option<NodeId>>` and a missing element is `None`. A control the harness cannot resolve, or one that paints no text (`timer-duration`), is recorded as such and is not a panic.
- The harness's selectors are test-authored constants. Per security-plan §Input Validation (the "Test harness queries" row), the harness query helper panics with "invalid selector" on a selector that fails to parse. Pass it no runtime-built or external selector text.

## Anti-patterns to avoid
- Do not widen an engine crate's surface in a way that crosses a boundary. Making blitz-paint's `contrast_ratio` public is an API visibility change, not a crossing. Pairing it with any new carrier (stdout of a binary, a socket, an MCP result) would be a crossing question (per security-plan §API Security).
- Do not add a `println!`, an `eprintln!` or an unconditional log in an engine crate. A harness home in `blitz-test-harness` must not carry one. Engine crates log through `tracing` behind their `tracing` feature (per security-plan §Logging & Monitoring, "Log format and backends").

## Contract bindings
- security §Logging & Monitoring ↔ obs-plan: the new code adds no record to escher's sink, so it needs no new allowlist or scrub entry.
- security §Dependency Security ↔ test-plan CI legs: if the chunk adds a crate, the `audit` leg (`bash .github/scripts/ci-leg.sh audit`) is the gate.

## Acceptance criteria contributions
- If a package enters `Cargo.lock`, the audit leg reads `advisories ok` with `deny.toml` unchanged, and each such package is reviewed by hand (per security-plan §Dependency Security).
- Over the chunk's added lines, a census of `std::env|env::var|env!\(|UnixListener|TcpListener|bind\(` finds 0 hits outside test-build `env!("CARGO_MANIFEST_DIR")` (per security-plan §Secret Management, with security-plan §API Security for the listener half).
- The chunk's added lines hold no `tracing` event or span field carrying a stable id, accessible name or control value (per security-plan §Input Validation, the `id` row).
- No added harness code indexes the slab by `NodeId` or calls `layout_style()`, `layout_data` or `guard` on a node that may lack styles. The check is a read of the diff, and the walk runs on all 18 stand control rows without panic in both layout modes (per security-plan §Error Handling).
