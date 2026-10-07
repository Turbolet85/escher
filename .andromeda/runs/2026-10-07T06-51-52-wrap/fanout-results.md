# Fan-out results — 2026-10-07-driver-session (wrap run 2026-10-07T06-51-52-wrap)

Seven doc-agents, one parallel batch; 15 detectors over 7 docs (arch 2 · security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2). Each return below is the agent's own text, taken from its transcript, entity-decoded where the probe found entities. Dispositions follow in §Validate.

## architecture

verdict: 17 proposal(s) · parsed · entities 0 found, 0 after decode · 8 comment line(s) around the YAML (the agent's notes, kept below, not proposals)

```yaml
# drift-detector: architecture · chunk 2026-10-07-driver-session
# Baseline read: /home/turbolet/dev/projects/escher/.andromeda/architecture.md (264 lines); `escher.driver|escher-session|session\.sock` = 0 hits, so every new resource is unregistered.
# D-arch-decisions: no third-party crate added or bumped (report Dependencies: only `std`), no locked decision contradicted; one proposal records the new cross-process pattern.
proposals:
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Network ports and listeners"
    change: >-
      Replace "none." with: network ports — none (no TCP or UDP port); listeners — one, a Unix-domain socket `{state_dir}/session.sock`, mode `0600`, inside a `0700` state directory the caller names, bound by `escher_driver::serve` at the tree's one listener site (packages/escher-driver/src/host.rs:81), unix only (non-unix: `SessionError::Unsupported`), carrying the lifecycle wire only (`hello v1` / `stop v1`; no element id, accessible name, control value, snapshot text or diff) — the founder's choice, 2026-10-07; keep the existing "Observed absent" searches as the as-adopted baseline they describe.
    sidecar: "2026-10-07-driver-session — Occupied Resources: first listener registered (Unix-domain `session.sock`, 0600 in a 0700 caller-named dir, escher-driver host.rs); still no TCP/UDP port."
    rationale: "Report Changes → Symbols/APIs: 'the workspace's first listener: one Unix-domain socket'; Counts: 'listener sites in the tree: 0 → 1'; Expected amendments names arch:144 as the site; boundary widening per inputs#I1."
    basis: ".andromeda/architecture.md:144 · packages/escher-driver/src/host.rs:81,95 · packages/escher-driver/src/wire.rs:10"

  - detector: D-arch-resources
    severity: warning
    section: "§Inherited Defaults → API style"
    change: >-
      Replace "no network listener (see Occupied Resources)" with "no network (TCP/UDP) listener; one local Unix-domain socket, the driver session's lifecycle socket bound by `escher_driver::serve` (see Occupied Resources)".
    sidecar: "2026-10-07-driver-session — Inherited Defaults/API style: 'no network listener' scoped to TCP/UDP; the driver session's local socket named."
    rationale: "Same claim as arch:144 restated; the report notes arch:218 stays true 'only if network is read strictly — a local socket is not a network listener'. Left as written it reads as 'no listener at all'."
    basis: ".andromeda/architecture.md:218"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Process-wide state and threads (the re-exec clause)"
    change: >-
      "two blitz-tests integration binaries each re-execute their own test binary … test-only, a fixed argv, no socket or port" becomes four: keep the two older ones (`telemetry_stdout_silent`, `stand_id_persistence`) as "no socket or port", and add `stand_session_lifecycle` (child `session_host`) and `stand_session_quiet` (child `session_host_with_typed_text`), each re-running `std::env::current_exe()` with `--ignored --exact {child} --nocapture` and stdin null as a session host that binds `session.sock` in its `target/tmp/` state directory and serves until stopped, every wait bounded and a guard killing and reaping a host a failing check still holds; plus `host_binary` (seven_guis) spawning the built `escher-session` binary via `CARGO_BIN_EXE_escher-session`.
    sidecar: "2026-10-07-driver-session — Occupied Resources/process state: re-exec children 2 → 4; the two new ones host a session socket; `host_binary` spawns `escher-session`."
    rationale: "Report Counts: 're-exec children in blitz-tests: 2 → 4 (session_host, session_host_with_typed_text, both #[ignore], each serving until stopped)'; Harness/gate surface describes the spawn, bounds and guard. The clause's 'no socket or port' is the retired no-listener claim restated and is now true of the two older children only."
    basis: ".andromeda/architecture.md:148 · tests/blitz-tests/tests/session_common/mod.rs:82-85 · tests/blitz-tests/tests/stand_session_lifecycle.rs:99-100 · tests/blitz-tests/tests/stand_session_quiet.rs:91-92 · examples/seven_guis/tests/host_binary.rs:11"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Process-wide state and threads (the telemetry-install clause)"
    change: >-
      "in `seven_guis_native`, `escher_telemetry::init` installs …" becomes: in both binaries of the `seven_guis` package — `seven_guis_native` (examples/seven_guis/src/main.rs:6-9) and the headless session host `escher-session` (examples/seven_guis/src/session_host.rs:20, its second install site) — with the rest of the clause unchanged; add that `escher-session` is a long-lived host process holding one `Session` and one listener on its main thread, spawning no thread, started by `escher_driver::start` (which spawns the caller's `std::process::Command`) and ended by `stop`.
    sidecar: "2026-10-07-driver-session — Occupied Resources/process state: `escher_telemetry::init` has a second install site (`escher-session`); the session host process registered."
    rationale: "Report Symbols/APIs: binary `escher-session` 'installs escher_telemetry::init … first (session_host.rs:20)'; remaining-caller facts: 'escher_telemetry::init gains its second install site'."
    basis: ".andromeda/architecture.md:148 · examples/seven_guis/src/session_host.rs:20"

  - detector: D-arch-resources
    severity: warning
    section: "§Cross-cutting Patterns → Logging and timing"
    change: >-
      "escher binaries install `escher_telemetry::init(…)` at start — today `seven_guis_native`, … before `dioxus_native::launch`" becomes: today two, both of the `seven_guis` package and so both stamping `service.name=seven_guis` — `seven_guis_native` before `dioxus_native::launch` (examples/seven_guis/src/main.rs:4-10) and `escher-session` first thing in its native `main` (examples/seven_guis/src/session_host.rs:20) — each reporting an `Err` with `eprintln!` and continuing.
    sidecar: "2026-10-07-driver-session — Logging and timing: second sink-installing binary (`escher-session`), same `service.name=seven_guis`."
    rationale: "Same single-install-site claim as the arch:148 telemetry clause, restated with 'today seven_guis_native'; report: 'its stderr identity is service.name=seven_guis — the same as the windowed seven_guis_native binary's'."
    basis: ".andromeda/architecture.md:190 · examples/seven_guis/src/session_host.rs:20-22"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Filesystem"
    change: >-
      Add: the driver session's state directory — a path the caller names (argv for `escher-session`; never an env read or a cwd-relative default), created `0700` by `escher_driver::serve` when absent (its parent must exist), refused with `StateDirNotPrivate` when present and open to group or other, holding `session.sock` alone while a session is up, and removed at `stop` (the socket file, then a non-recursive `remove_dir`; nothing else deleted); the checks' state directories under `env!("CARGO_TARGET_TMPDIR")` = `target/tmp/` — `ss-life`, `ss-quiet` (blitz-tests) and `hb-serve`, `hb-refuse` (seven_guis) — each removed at `stop`, local only, no CI workflow changed.
    sidecar: "2026-10-07-driver-session — Occupied Resources/Filesystem: session state directory (0700, `session.sock` only, removed at stop) and the four check dirs under `target/tmp/`."
    rationale: "Report Symbols/APIs (`serve`: state dir creation, modes, removal), Schema/config ('session.sock alone while a session is up; nothing after stop'), Harness/gate surface (the four `{CARGO_TARGET_TMPDIR}` directories)."
    basis: ".andromeda/architecture.md:147 · packages/escher-driver/src/host.rs:37-53 · tests/blitz-tests/tests/session_common/mod.rs:71 · examples/seven_guis/tests/host_binary.rs:15"

  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Names"
    change: >-
      Register crate `escher-driver` — a `publish = false` library with no features and no binary, workspace member and `[workspace.dependencies]` path entry (packages/escher-driver/Cargo.toml:2-4; Cargo.toml:18; Cargo.toml:62), consumed by seven_guis on the native target and as a blitz-tests dev-dependency (tests/blitz-tests/Cargo.toml:23); binaries become `blitz`, `seven_guis_native`, `escher-session`, `todomvc_native`, the second a `[[bin]]` of seven_guis (examples/seven_guis/Cargo.toml:45-47); seven_guis' native-target dependencies read escher-telemetry, blitz-test-harness, blitz-traits and escher-driver with `dioxus-native` `system-fonts` + `woff` at `examples/seven_guis/Cargo.toml:29-35` (was `:29-34`); re-point this bullet's root cites — escher-telemetry `Cargo.toml:17; Cargo.toml:61` (was `:60`), seven_guis `Cargo.toml:63` (was `:61`).
    sidecar: "2026-10-07-driver-session — Names: crate `escher-driver`, binary `escher-session`, seven_guis native deps `:29-35`; root manifest cites re-pointed (+1/+2)."
    rationale: "Report Crates/modules: 'added workspace member escher-driver … root Cargo.toml:18 member line, :62 path entry'; 'the seven_guis package now has two binaries'; Counts: native block ':29-35 (was :29-34) and now lists five dependencies'."
    basis: ".andromeda/architecture.md:152 · Cargo.toml:18,61-63 · packages/escher-driver/Cargo.toml:2-4 · examples/seven_guis/Cargo.toml:29-35,45-47"

  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver) [new bullet, after Telemetry bootstrap (escher-telemetry)]"
    change: >-
      New bullet: `escher_driver` re-exports `Session`, `SessionError`, `serve`, `attach`, `stop`, `start`, `Hello`, `Started` (lib.rs:29-32; modules private, `#![deny(missing_docs)]`); `Session::start(label, boot: impl FnOnce() -> Harness<DioxusDocument>) -> Result<Session, SessionError>` validates the label first (1-32 bytes of `a-z0-9-`, else `InvalidLabel` with the boot not run) then runs the caller's boot once on the calling thread, with `label()`, `harness()`, `harness_mut()` — app-agnostic, naming no app or stand type; `serve(state_dir, session)` binds `{state_dir}/session.sock` and serves one connection at a time on the calling thread with 2 s read and write bounds; `attach(state_dir) -> Hello { pid, label, served }`, `stop(state_dir)` (polls until the directory is gone, 5 s bound), `start(state_dir, host: Command, ready: Duration) -> Started { child, hello }` (attach first, spawn, poll every 25 ms); `SessionError` is exactly `AlreadyRunning · NoSession · Dead · HostExited(Option<i32>) · Timeout · Protocol · InvalidLabel · StateDirTooLong · StateDirNotPrivate · Unsupported · Io(std::io::ErrorKind)`, `Display` + `Error`, fixed messages interpolating no path, label, id or value; the wire is crate-private and versioned `v1` — one ASCII `\n`-terminated request line (≤ 64 bytes: `hello v1`, `stop v1`) and one reply line (≤ 128 bytes: `ok v1 pid={pid} label={label} served={n}` · `ok v1 stopping` · `refused version` · `refused malformed`), carrying no element id, accessible name, control value, snapshot text or diff; non-unix: `serve`/`attach`/`stop`/`start` return `Unsupported` while an in-process `Session` works everywhere; not built: verbs, settle, act-by-id, CLI JSON, MCP tool, idle expiry.
    sidecar: "2026-10-07-driver-session — Standard Contracts: new 'Driver session (escher-driver)' bullet (Session, serve/attach/stop/start, SessionError, the v1 lifecycle wire)."
    rationale: "Report Symbols/APIs lists every symbol and the wire grammar; Expected amendments: 'arch §Standard Contracts — a session bullet — carried … No site exists yet (0 hits)'."
    basis: "packages/escher-driver/src/lib.rs:29-32 · report.md:15-19 (session.rs:23,35 · error.rs:10 · host.rs:20 · client.rs:11,22,37,48,62 · wire.rs:16,20)"

  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → CLIs"
    change: >-
      Add: `escher-session <task> <state-dir>` (binary of seven_guis, examples/seven_guis/src/session_host.rs; an empty `main` on wasm) — a closed argv of exactly two arguments, the task one of `counter` · `flight-booker` · `timer` · `crud`; it installs the telemetry sink first, boots through `seven_guis::stand` only (`stand::options(true)`), hands the harness to `Session::start(slug, …)` and runs `serve`; exit `0` after `serve` returns `Ok`, `1` with the error's `Display` on stderr, `2` with a usage line on stderr for any other argv having booted nothing and created no state; nothing on stdout on any path; no env var read beyond `RUST_LOG` through `escher_telemetry::init`.
    sidecar: "2026-10-07-driver-session — CLIs: `escher-session <task> <state-dir>` host entry, exit codes 0/1/2, stderr only."
    rationale: "Report Symbols/APIs: 'binary escher-session of the seven_guis package … argv is closed: exactly <task> <state-dir> … Exit codes 0/1/2 … writes nothing to stdout'; Schema/config lists the exit codes."
    basis: "examples/seven_guis/src/session_host.rs:13-30 · examples/seven_guis/Cargo.toml:45-47 · .andromeda/architecture.md:136"

  - detector: D-arch-resources
    severity: warning
    section: "§Inherited Defaults → Publishability"
    change: >-
      Add escher-driver to the `publish = false` list with its cite (packages/escher-driver/Cargo.toml:4) and to the exclusion "the published `packages/` library crates (not blitz-test-harness, escher-telemetry or escher-driver)"; re-point the root-package cite `Cargo.toml:243-254` → `Cargo.toml:245-256`.
    sidecar: "2026-10-07-driver-session — Publishability: escher-driver is `publish = false` and outside bump's published set."
    rationale: "Report Crates/modules: 'added workspace member escher-driver (library only … publish = false, every package field *.workspace = true)'; Expected amendments names §Inherited Defaults → Publishability."
    basis: ".andromeda/architecture.md:221 · packages/escher-driver/Cargo.toml:4 · Cargo.toml:245-256"

  - detector: D-arch-resources
    severity: warning
    section: "§Infrastructure Patterns → Directory structure"
    change: >-
      Add `escher-driver` to the `packages/` line of the tree (after `escher-telemetry`) and re-point the heading's cites to `Cargo.toml:2-31; Cargo.toml:32; Cargo.toml:245-256` (members list, `exclude`, the virtual package).
    sidecar: "2026-10-07-driver-session — Directory structure: `packages/escher-driver` added; root manifest cites re-pointed."
    rationale: "Report Counts: 'workspace members: +1 (packages/escher-driver, the second escher-owned crate under packages/)'; the tree at arch:162-165 lists every packages/ member and omits it."
    basis: ".andromeda/architecture.md:158-165 · Cargo.toml:2-32"

  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes (new row: escher-driver)"
    change: >-
      New row after escher-telemetry: `escher-driver | packages/escher-driver | The driver's session — one headless app instance held by one process with a start / attach / stop lifecycle: private modules session (`Session`, the label rule), host (`serve`, the one listener), client (`attach`, `stop`, `start`), wire (the crate-private v1 lifecycle grammar) and error (`SessionError`), re-exported at the crate root; depends on blitz-test-harness and dioxus-native-dom only, no feature named, and on no app (packages/escher-driver/src/lib.rs:22-32; packages/escher-driver/Cargo.toml:13-15)`.
    sidecar: "2026-10-07-driver-session — Existing Scopes: `escher-driver` row added."
    rationale: "Report Crates/modules: 'Modules client, error, host, session, wire (private; re-exports at lib.rs:29-32)'; 'escher-driver → blitz-test-harness, escher-driver → dioxus-native-dom … no feature named'; Outcome: 'the library does not depend on the example'."
    basis: "packages/escher-driver/src/lib.rs:22-32 · packages/escher-driver/Cargo.toml:13-15"

  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → seven_guis"
    change: >-
      Row adds: a second binary `escher-session` (`src/session_host.rs`, the headless session host, examples/seven_guis/Cargo.toml:45-47) and the crate's first integration-test target `tests/host_binary.rs`; the native-dependency clause becomes "native dependencies escher-telemetry, blitz-test-harness, blitz-traits and escher-driver, `dioxus-native` with `system-fonts` + `woff` (examples/seven_guis/Cargo.toml:29-35)".
    sidecar: "2026-10-07-driver-session — Existing Scopes/seven_guis: `escher-session` binary, `tests/host_binary.rs`, native deps `:29-35` incl. escher-driver."
    rationale: "Report Files ('the crate's first tests/ directory'), Crates/modules ('two binaries … and one integration-test target'), Counts ('only the two :29-34 sites (architecture.md:152, :260) cite a moved range and a changed claim')."
    basis: ".andromeda/architecture.md:260 · examples/seven_guis/Cargo.toml:29-35,45-47"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Existing Scopes → blitz-tests"
    change: >-
      Row adds, after the stand snapshot-state clause: the driver session over the stand — state, ids and fresh-boot equality held across commands on one `Session`, the socket lifecycle and each edge, and nothing of the screen leaving the session path (tests/blitz-tests/tests/stand_session_state.rs · stand_session_ids.rs · stand_session_fresh.rs · stand_session_lifecycle.rs · stand_session_quiet.rs), with a second shared module `tests/session_common/mod.rs` — no test target and no test in it — read through `mod session_common;` by four of them; and "read through `mod common;` by six stand checks" becomes eleven (the five session checks declare it too).
    sidecar: "2026-10-07-driver-session — Existing Scopes/blitz-tests: five `stand_session_*` checks and the `session_common` module; `mod common;` readers 6 → 11."
    rationale: "Report Files: the five new stand checks 'and their shared module tests/blitz-tests/tests/session_common/mod.rs (145; no test target and no test in it)'. The reader count 6 → 11 is NOT stated in the report — measured here (`grep -l '^mod common;' tests/blitz-tests/tests/*.rs` → 11; `mod session_common;` in fresh, ids, lifecycle, quiet); orchestrator to re-derive before applying that clause."
    basis: ".andromeda/architecture.md:259 · tests/blitz-tests/tests/session_common/mod.rs:1-6 · tests/blitz-tests/tests/stand_session_state.rs:11"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Conventions → Tests"
    change: >-
      After "the seven_guis stand checks state the tables and helpers they share once, in the module `tests/blitz-tests/tests/common/mod.rs` …" add: the session checks keep what only they share in a second such module, `tests/blitz-tests/tests/session_common/mod.rs`, read by `mod session_common;` — also a subdirectory module and no test target (tests/blitz-tests/tests/session_common/mod.rs:1-6).
    sidecar: "2026-10-07-driver-session — Conventions/Tests: second shared stand-check module `session_common` named."
    rationale: "Same single-shared-module claim as the blitz-tests scope row (arch:259), restated in Conventions; report Scope record: session_common 'exists because tests/common/ is frozen by the plan's preservation gate'."
    basis: ".andromeda/architecture.md:114 · tests/blitz-tests/tests/session_common/mod.rs:1-6"
    dependent-of: D-arch-resources

  - detector: D-arch-resources
    severity: warning
    section: "§Stack and Technologies → Testing"
    change: >-
      blitz-tests dev-dependencies list adds escher-driver (the driver session, tests/blitz-tests/Cargo.toml:23) after seven_guis (still `:22`); the row's manifest ranges shift by the one inserted line — `tests/blitz-tests/Cargo.toml:14-24; :26-28; :30-39` (were `:14-23; :25-27; :29-38`).
    sidecar: "2026-10-07-driver-session — Stack/Testing: blitz-tests dev-dependency `escher-driver` (:23); manifest ranges +1."
    rationale: "Report Crates/modules: 'blitz-tests → escher-driver (dev-dependency, tests/blitz-tests/Cargo.toml:23)'; Files: 'tests/blitz-tests/Cargo.toml (+1 line, :23)'; Expected amendments names §Stack and Technologies → Testing."
    basis: ".andromeda/architecture.md:64 · tests/blitz-tests/Cargo.toml:22-25"

  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Driver session] (new keyed entry)"
    change: >-
      New entry: **[Driver session]** the session crosses processes on a std-only Unix-domain socket in an owner-only state directory (`0700` directory, `0600` socket), lifecycle messages only — no third-party crate, no TCP or UDP port, and nothing of the screen on the wire (the founder's choice, 2026-10-07); the library takes the app's boot from its caller and depends on no app (`escher-driver` names no `seven_guis` type; the one edge runs `seven_guis → escher-driver`); no idle expiry; names no `dioxus-native-dom` feature, so it enables no accessibility code; Windows has no `std` local socket, so the cross-process lifecycle is `SessionError::Unsupported` there in 0.1.0 behind `cfg(unix)` gates, while an in-process `Session` works on every target (packages/escher-driver/src/lib.rs:12-18; packages/escher-driver/Cargo.toml:13-15).
    sidecar: "2026-10-07-driver-session — Established Decisions: [Driver session] — std-only Unix-domain socket, owner-only state dir, lifecycle-only wire, caller-supplied boot, unix-only in 0.1.0 (founder's choice, inputs#I1/I2)."
    rationale: "No library or runtime outside §Stack entered (report Dependencies: 'no third-party crate added or bumped … only std') and no locked decision is contradicted; but the chunk lands a pattern neither §Stack nor §Established Decisions names — the workspace's first cross-process transport and first `cfg(unix)` gates in escher-owned code (report Counts) — chosen by the founder (inputs#I1: 'a std-only Unix-domain socket in an owner-only state directory, lifecycle messages only'; inputs#I2: caller-supplied boot, no seven_guis dependency, no idle expiry). Recorded so later driver chunks are checked against it. If §Established Decisions is treated as a no-new-entry log, drop this one; the Standard Contracts bullet carries the facts."
    basis: "report.md:20,25,41-42 · packages/escher-driver/src/lib.rs:12-18"

# Outside both detectors (not proposed; for the orchestrator's other passes):
# - Root `Cargo.toml` citation re-point: architecture cites it at 65 sites (measured with the report's regex); 62 move — rule: line < 18 unchanged, 18..60 → +1, ≥ 61 → +2; `2-30` → `2-31`, `44-60` → `45-61`, `243-254` → `245-256`. Only `Cargo.toml:1` (×2) and `Cargo.toml:17` (×1) stand. The proposals above re-point only the cites inside the bullets they touch.
# - PROVISIONAL marks at architecture.md:120, 121, 134, 194 (founder's ratification, report Decisions & corrections) are not in the report's Changes and belong to neither detector.
# - `examples/seven_guis/src/session_host.rs` sets no `windows_subsystem` attribute (0 hits), unlike the binaries §Conventions → Apps and examples cites; the report does not state it, so no proposal.
# - Checked and left standing: arch:134 "no log, event, socket, file or command carries the snapshot/diff … no driver, CLI or MCP command exposes it yet" (the wire carries none of it; no verb built); arch:130 headless-stand contract (stand.rs unchanged); arch:139 agent-run "state in `target/agent-run/` only" (true of the script; the socket belongs to the library and its checks); arch:151 Environment variables (report: no new env var).
```

## security-plan

verdict: 20 proposal(s) · parsed · entities 0 found, 0 after decode · 16 comment line(s) around the YAML (the agent's notes, kept below, not proposals)

```yaml
proposals:
  # ---- D-security-input: the new external-input surfaces have no §Input Validation rows ----
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation (new table row: Driver session socket — lifecycle wire)"
    change: >-
      Add a row — Boundary "Driver session socket (escher-driver lifecycle wire)" · What "Request line from a local peer over `{state_dir}/session.sock`" · How: one request line and one reply line per connection, ASCII, `\n`-terminated; a request is parsed against a closed grammar (`hello v1`, `stop v1`) and bounded at 64 bytes (`MAX_REQUEST_BYTES`), a reply at 128 (`MAX_REPLY_BYTES`); a known word with another `v{digits}` answers `refused version`, anything else — an over-long line included — `refused malformed`; a refusal or a broken connection changes no state and a connection that ends before a line terminator gets no reply; one connection at a time with a 2 s read and write bound each; no request or reply carries an element id, an accessible name, a control value, snapshot text or a diff — only pid, the label and a served count; unix only, `Unsupported` elsewhere (packages/escher-driver/src/wire.rs:16-20; packages/escher-driver/src/wire.rs:62; packages/escher-driver/src/host.rs:128-130) — as measured at escher-0.1.0/chunks/2026-10-07-driver-session/report.md.
    sidecar: "2026-10-07-driver-session: §Input Validation gains the driver-session lifecycle-wire row (closed `hello v1` / `stop v1` grammar, 64-byte request bound, 2 s I/O bound, nothing of the screen on the wire)."
    rationale: >-
      Report Changes → Symbols / APIs adds the workspace's first listener and its wire (report.md:19-20); Coverage of new surfaces reads validation ✓ for it (report.md:71), so the mechanism is present in code, but security-plan has no row for the boundary (search `session\.sock|escher-driver|escher-session|UnixListener` over security-plan.md: 0 hits). Expected amendment "security-plan §Input Validation — rows for the session wire, the label and the `escher-session` argv — carried" (report.md:59). A boundary widening on the founder's choice (inputs#I1, report.md:41).
    basis: packages/escher-driver/src/wire.rs:16
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation (new table row: Driver session state directory)"
    change: >-
      Add a row — Boundary "Driver session state directory" · What "Caller-named directory path and its mode" · How: `serve` creates it mode `0700` when absent (the parent must exist) and refuses a present one whose mode grants group or other any bit (`StateDirNotPrivate`); the socket file is set `0600`; a path `std` refuses as too long is `StateDirTooLong`; on `AddrInUse` it connects once — answered → `AlreadyRunning`, `ConnectionRefused` → removes that one stale socket file and binds again; `stop` removes `session.sock` then the directory non-recursively and deletes nothing else; the directory is a parameter (argv for the binary), never an env read or a cwd-relative default (packages/escher-driver/src/host.rs:63-73; packages/escher-driver/src/host.rs:81-95; packages/escher-driver/src/host.rs:52-53) — as measured at escher-0.1.0/chunks/2026-10-07-driver-session/report.md.
    sidecar: "2026-10-07-driver-session: §Input Validation gains the session state-directory row (0700 directory, 0600 socket, group/other-open directory refused, stale socket handling, non-recursive removal)."
    rationale: >-
      Report Symbols / APIs `serve` (report.md:17) and Coverage "the session state directory → validation ✓" (report.md:74); no row in security-plan records the mode checks that are the socket's only access control.
    basis: packages/escher-driver/src/host.rs:63
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation (new table row: Driver session label)"
    change: >-
      Add a row — Boundary "Driver session label (`Session::start`)" · What "Caller-chosen label" · How: 1 to 32 bytes of `a-z`, `0-9`, `-` (`valid_label`), checked before the boot closure runs; an invalid label returns `InvalidLabel` with nothing booted; the label is never text read from the document and is the only caller string the wire echoes (packages/escher-driver/src/session.rs:11; packages/escher-driver/src/session.rs:35) — as measured at escher-0.1.0/chunks/2026-10-07-driver-session/report.md.
    sidecar: "2026-10-07-driver-session: §Input Validation gains the session-label row (1-32 bytes of `a-z0-9-`, refused before boot)."
    rationale: >-
      Report Symbols / APIs `escher_driver::Session` (report.md:15) and Coverage "the session label → validation ✓" (report.md:73); expected amendment report.md:59 names the label row; security-plan has none.
    basis: packages/escher-driver/src/session.rs:11
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation (new table row: CLI arguments (escher-session))"
    change: >-
      Add a row — Boundary "CLI arguments (escher-session)" · What "Task and state directory" · How: argv is closed — exactly `<task> <state-dir>`, the task one of `counter`, `flight-booker`, `timer`, `crud` from a four-entry table; any other argv prints a usage line on stderr and exits 2 having booted nothing and created no state; the state directory then passes `serve`'s mode checks; exit `0` after `serve` returns `Ok`, `1` with the error's fixed-string `Display` (no path, label, id or value interpolated) on stderr; nothing is written to stdout on any path (examples/seven_guis/src/session_host.rs:13-18; examples/seven_guis/src/session_host.rs:24-32) — as measured at escher-0.1.0/chunks/2026-10-07-driver-session/report.md.
    sidecar: "2026-10-07-driver-session: §Input Validation gains the `escher-session` argv row (closed two-argument form, four-task table, exit 2 before any boot or state)."
    rationale: >-
      Report Symbols / APIs binary `escher-session` (report.md:21) and Coverage "the `escher-session` binary's argv → validation ✓" (report.md:72); expected amendment report.md:59 names the argv row; security-plan has none.
    basis: examples/seven_guis/src/session_host.rs:24
    dependent-of: D-security-input

  # ---- the claim the listener retires: "no served surface / no listener" ----
  - detector: D-security-input
    severity: escalate
    section: "§API Security (lead sentence, security-plan.md:185)"
    change: >-
      The lead should now say: no served network API exists — no TCP or UDP port — and the two slice searches (`TcpListener|listen\(` over s10, `TcpListener|TcpStream|bind\(|listen\(` over s13) still stand for those files; one local listener exists since 2026-10-07-driver-session: a Unix-domain socket `{state_dir}/session.sock`, constructed at one site (packages/escher-driver/src/host.rs:81 — the census `UnixListener|TcpListener|UdpSocket` over packages, apps, examples, tests, wpt prints that one path, where before it printed nothing), carrying lifecycle messages only (`hello`, `stop`); Windows has no listener (`Unsupported`).
    sidecar: "2026-10-07-driver-session: §API Security lead scoped — no network listener, one local Unix-domain session socket (the workspace's first listener, one site)."
    rationale: >-
      security-plan.md:185 reads "No served API surface exists … server routes or listeners are observed absent"; report Changes: "the workspace's first listener: one Unix-domain socket" and "listener sites in the tree: 0 → 1" (report.md:20, :34). The sentence's two searches do not name `UnixListener`, so its literal scope holds, but its headline is retired. Expected amendment "security-plan §API Security · §Threat Model Summary — carried" (report.md:58).
    basis: packages/escher-driver/src/host.rs:81
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§API Security (controls table, new row: Local session socket)"
    change: >-
      Add a control row — "Local session socket (escher-driver)" · reachable by the owning user only (state directory `0700`, socket file `0600`, a group- or other-open directory refused); no handshake secret, token or peer-credential check; one connection at a time, 2 s read and write bound, request line bounded at 64 bytes and reply at 128; `stop` removes the socket and the directory; `start` does not check that the answering pid is the child it spawned (known, left as built) (packages/escher-driver/src/host.rs:63-73; packages/escher-driver/src/host.rs:95; packages/escher-driver/src/host.rs:128-130) · Stack reference `std::os::unix::net::UnixListener`.
    sidecar: "2026-10-07-driver-session: §API Security controls table gains the local session-socket row (owner-only modes, bounded line and I/O, no peer authentication)."
    rationale: >-
      Expected amendment report.md:58 asks §API Security to state "what it is, who reaches it … what is validated on it, what `stop` removes"; the table lists outbound and in-document controls only. The pid caveat is the report's own "Known and left as built" (report.md:85).
    basis: packages/escher-driver/src/host.rs:95
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Attack surface (new vector: local IPC — driver session socket)"
    change: >-
      Add a vector "local IPC (driver session socket)" — Entry point: `escher_driver::serve` binds one Unix-domain socket `{state_dir}/session.sock` in a caller-named directory, hosted by the `escher-session` binary of `seven_guis` (argv `<task> <state-dir>`) and by the session checks; no TCP or UDP port; unix only. Trust boundary: the owning user, by the `0700` directory and `0600` socket file — no other authentication; the wire is lifecycle only (`hello`, `stop`) and carries no element id, accessible name, control value, snapshot text or diff; `scripts/agent-run.sh` itself still starts no daemon and binds nothing; per-input checks are listed under Input Validation.
    sidecar: "2026-10-07-driver-session: §Threat Model Summary attack surface gains the local-IPC vector (owner-only Unix-domain session socket, lifecycle messages only) — the founder's boundary choice."
    rationale: >-
      The attack-surface list (security-plan.md:7-22) names remote content, script, CLI, markdown, WPT and CI vectors and no local IPC; the chunk adds the first listener (report.md:20) and the report's expected amendment names §Threat Model Summary (report.md:58), "the same boundary widening and answer (inputs#I1)".
    basis: packages/escher-driver/src/host.rs:20
    dependent-of: D-security-input

  # ---- the new host binary's coverage line reads PII raw at debug/trace: the "logged nowhere" claim ----
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → Markup attributes · `aria-label` · `<label for>` (accessible names) row (security-plan.md:108)"
    change: >-
      Replace "the names are logged nowhere" with the measured scope: escher's own code logs no name, and a host that installs escher's sink is quiet at the default level and at `RUST_LOG=info` (0 ids, 0 names measured on `escher-session`, CRUD task); at `RUST_LOG=trace` an accessible name prints on that host's stderr through the third-party target `dioxus_core::diff::node`, outside the sink's engine allowlist (a fixture row's name ×3 measured); typed text is not measured (no command types into the held instance yet) and the windowed `seven_guis_native` is not measured by level; the remedy — the sink dropping records from targets outside its allowlist — is owned by the route entry added next — as measured at escher-0.1.0/chunks/2026-10-07-driver-session/report.md.
    sidecar: "2026-10-07-driver-session: accessible-names row — \"logged nowhere\" scoped to the measurement (quiet at default and info; names on a sink-installing host's stderr at trace through `dioxus_core::diff::node`; typed text and the windowed stand unmeasured)."
    rationale: >-
      Report Changes → Spec claims disproved by measurement 2 (report.md:48): "`security-plan.md:108` … 'the names are logged nowhere' — measured false for a host that installs escher's sink at `RUST_LOG=trace`"; Coverage of the new `escher-session` surface reads "raw✗ at `debug` (stable ids) and `trace` (ids and accessible names)" (report.md:72). The founder's ruling scopes the security wording to what was measured (report.md:90).
    basis: escher-0.1.0/chunks/2026-10-07-driver-session/report.md:48
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → Markup attributes · `id` (stable element id) row — the clause \"the id is computed on demand and written to no log, DOM or vdom\" (security-plan.md:107)"
    change: >-
      The clause should now say: the id is computed on demand and escher's own code writes it to no log, DOM or vdom; an author-key id is the element's HTML `id`, and that value prints on the stderr of a host that installs escher's sink at `RUST_LOG=debug` and `trace` through Stylo's own records (`style::traversal`, `style::sharing`, `style::style_resolver`, `selectors::matching` via the `log` bridge — `crud-surname` ×12 at debug, ×20 at trace on `escher-session`), 0 ids at the default level and at info — as measured at escher-0.1.0/chunks/2026-10-07-driver-session/report.md.
    sidecar: "2026-10-07-driver-session: `id` row — \"written to no log\" scoped to escher's own code; author-key ids (HTML `id` values) print on a sink-installing host's stderr at debug and trace through Stylo's records."
    rationale: >-
      Report disproved claim 2 (report.md:48): the row "holds for the id computation … but an author-key id is the element's HTML `id`, and that value is on the sink's stderr at `debug` and `trace` through Stylo's own records — the row does not say so."
    basis: escher-0.1.0/chunks/2026-10-07-driver-session/report.md:48
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → `id` row — first restatement, \"so the platform adapter stays the id's only exit\" (after the `Snapshot::to_text` clause, security-plan.md:107)"
    change: >-
      Should now say: so the platform adapter stays the only exit of the id as escher computes and carries it — the same value, as an author-key element's HTML `id`, also reaches the stderr of a sink-installing host at `RUST_LOG=debug` and `trace` (the clause above).
    sidecar: "2026-10-07-driver-session: `id` row — \"the id's only exit\" (snapshot-text clause) scoped to the computed id; the stderr reach at debug/trace noted."
    rationale: >-
      Same claim as the "written to no log" clause, restated; left unscoped it contradicts the measured stderr reach (report.md:46-48).
    basis: /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:107
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → `id` row — second restatement, \"so the platform adapter still stays the id's only exit\" (after the `unkeyed_actionable` clause, security-plan.md:107)"
    change: >-
      Should now say: so the platform adapter still stays the only exit of the id as escher computes and carries it (the stderr reach of author-key `id` values at debug and trace is the clause above).
    sidecar: "2026-10-07-driver-session: `id` row — \"the id's only exit\" (`unkeyed_actionable` clause) scoped to the computed id."
    rationale: Second occurrence of the retired "only exit" claim in the same row (report.md:48).
    basis: /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:107
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → `id` row — third restatement, \"so the platform adapter still stays the id's only exit\" (after the `Snapshot::diff` clause, security-plan.md:107)"
    change: >-
      Should now say: so the platform adapter still stays the only exit of the id as escher computes and carries it (the stderr reach of author-key `id` values at debug and trace is the clause above).
    sidecar: "2026-10-07-driver-session: `id` row — \"the id's only exit\" (`Snapshot::diff` clause) scoped to the computed id."
    rationale: Third occurrence of the retired "only exit" claim in the same row (report.md:48).
    basis: /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:107
    dependent-of: D-security-input
  - detector: D-security-input
    severity: warning
    section: "§Logging & Monitoring → Log format and backends — escher's own sink, the \"Its reach is that sink only\" list (security-plan.md:366)"
    change: >-
      Add to the past-the-scrub list: a record from a target outside the engine allowlist (`ENGINE_TARGET_PREFIXES`) prints its message and fields as written — on `escher-session` (CRUD task) `warn` 0 lines, `info` 1 line with 0 ids and 0 names, `debug` 1165 lines carrying stable ids, `trace` 1501 lines carrying ids and accessible names, through `style::traversal`, `style::sharing`, `style::style_resolver`, `selectors::matching` (the `log` bridge) and `dioxus_core::diff::node`; stdout 0 bytes at every level; typed text not measured, the windowed `seven_guis_native` not measured by level — as measured at escher-0.1.0/chunks/2026-10-07-driver-session/report.md.
    sidecar: "2026-10-07-driver-session: §Logging & Monitoring past-the-scrub list gains the non-allowlisted-target class (ids at debug, names at trace on a sink-installing host)."
    rationale: >-
      Report disproved claim 3 (report.md:49): the list at `security-plan.md:366` is "incomplete, not false: neither names the class measured here"; the scrub statements themselves are accurate as written and stay.
    basis: escher-0.1.0/chunks/2026-10-07-driver-session/report.md:49
    dependent-of: D-security-input
  - detector: D-security-input
    severity: warning
    section: "§Logging & Monitoring → Log format and backends — escher's own sink, the install clause \"`seven_guis_native` installs `escher_telemetry::init`\" (security-plan.md:366)"
    change: >-
      Should now say: both binaries of the `seven_guis` package install `escher_telemetry::init` — `seven_guis_native` (examples/seven_guis/src/main.rs:6-9) and the headless `escher-session` host, first thing in `main`, an `Err` `eprintln!`ed and the process continuing (examples/seven_guis/src/session_host.rs:20-22) — under the same `service.name=seven_guis`.
    sidecar: "2026-10-07-driver-session: escher's sink has a second install site, the `escher-session` host binary (same `service.name=seven_guis`)."
    rationale: >-
      Report Symbols / APIs: "`escher_telemetry::init` gains its second install site (`session_host.rs:20`; the first is `examples/seven_guis/src/main.rs:6`)" (report.md:23); the line names one installer.
    basis: examples/seven_guis/src/session_host.rs:20
    dependent-of: D-security-input
  - detector: D-security-input
    severity: warning
    section: "§Logging & Monitoring → Log format and backends — Stdout output bullet, \"`seven_guis_native` additionally logs through escher's stderr sink above\" (security-plan.md:367)"
    change: >-
      Should now say: `seven_guis_native` and the `escher-session` host additionally log through escher's stderr sink above; `escher-session` writes nothing to stdout on any path — its usage line and its error `Display` go to stderr through `eprintln!` (examples/seven_guis/src/session_host.rs:30; examples/seven_guis/src/session_host.rs:50).
    sidecar: "2026-10-07-driver-session: Stdout-output bullet names the `escher-session` host as a second sink-logging example binary (stdout 0 bytes)."
    rationale: >-
      Same actor claim as the install clause, restated; report.md:21 ("It writes nothing to stdout on any path") and the measured "stdout 0 bytes at every level" (report.md:46).
    basis: examples/seven_guis/src/session_host.rs:30
    dependent-of: D-security-input
  - detector: D-security-input
    severity: warning
    section: "§Bootstrap phases → logging-redaction-wire (security-plan.md:248)"
    change: >-
      Should now say: discharged for engine-target records and content-named fields in escher's own log sink, installed by `seven_guis_native` and `escher-session`; still open for (a) records from targets outside the sink's engine allowlist, which print unredacted — stable ids at `RUST_LOG=debug`, accessible names at `trace` — the route entry added next owns it, and (b) the upstream apps' `fmt::init()` stdout subscribers and the WPT runner's `env_logger`, which stay unscrubbed.
    sidecar: "2026-10-07-driver-session: logging-redaction-wire phase re-scoped — discharged for engine targets and content-named fields only; non-allowlisted third-party targets recorded as open, owned by the next route entry; second installer named."
    rationale: >-
      The bullet restates the scrub's reach ("content-named fields of any target in `seven_guis_native`") and calls the phase discharged for escher's sink; report disproved claims 2-3 (report.md:48-49) and the founder's ruling half (2) — "a route entry NEXT … the telemetry sink drops records from targets outside its allowlist at every level" (report.md:90) — leave a measured remainder; the bullet also names one installer (report.md:23).
    basis: /home/turbolet/dev/projects/escher/.andromeda/security-plan.md:248
    dependent-of: D-security-input

  # ---- D-security-auth: no off-spec library or secret source; two record gaps ----
  - detector: D-security-auth
    severity: warning
    section: "§Authentication & Authorization (requirements table, new row; the \"Auth approach: none observed\" lead stays)"
    change: >-
      Add a row — Requirement "Local IPC access (driver session socket)" · Implementation: no authentication handshake, token, secret or peer-credential check; access is the filesystem's — the state directory is created `0700` and a present one open to group or other is refused (`StateDirNotPrivate`), the socket file is set `0600`, so only the owning user reaches it; `start` does not check that the answering pid is the child it spawned (packages/escher-driver/src/host.rs:63-73; packages/escher-driver/src/host.rs:95) · Stack reference "`std` file modes (unix)".
    sidecar: "2026-10-07-driver-session: §Authentication & Authorization table gains the session-socket access row (owner-only file modes; no handshake, token or peer check)."
    rationale: >-
      The chunk touches a session and its access control (report.md:17-20). No auth or crypto library, token or secret entered — "Only `std` is used for the socket, the process spawn and the permissions" (report.md:25), no env var read beyond `RUST_LOG` (report.md:22) — so the flow matches "Auth approach: none observed"; the mismatch is that the section's table records permission grants (CI tokens) and omits the one access control this chunk adds. Record gap, not an off-spec mechanism.
    basis: packages/escher-driver/src/host.rs:70
  - detector: D-security-auth
    severity: warning
    section: "§Secret Management → Environment values read (none secret-bearing) (security-plan.md:278-283)"
    change: >-
      Add a bullet: compile-time `env!("CARGO_BIN_EXE_escher-session")` and `env!("CARGO_TARGET_TMPDIR")` in the session checks — the host binary's path and the per-target temp directory the state directories live under (examples/seven_guis/tests/host_binary.rs:11; examples/seven_guis/tests/host_binary.rs:15; tests/blitz-tests/tests/session_common/mod.rs:71); the `escher-driver` library and the `escher-session` binary read no env var beyond `RUST_LOG` through `escher_telemetry::init` — the state directory is a parameter (argv), never an env read.
    sidecar: "2026-10-07-driver-session: Environment values read gains the two compile-time cargo values the session checks use; the library and host binary add no runtime env read."
    rationale: >-
      Report Counts: "`CARGO_BIN_EXE_*` / `CARGO_TARGET_TMPDIR` uses: 0 → `host_binary.rs` and `session_common/mod.rs`" (report.md:34) and "No new env var" at runtime (report.md:22); the list already records the build-time `env!("CARGO_MANIFEST_DIR")`, so it is the home for these. No secret source added.
    basis: examples/seven_guis/tests/host_binary.rs:11
    dependent-of: D-security-auth

  # ---- D-security-deps: no dependency added, bumped or banned; cited lines moved ----
  - detector: D-security-deps
    severity: warning
    section: "§Dependency Security → Pinning — \"Git dependencies are pinned by commit rev (Cargo.toml:104; Cargo.toml:114)\" (security-plan.md:217)"
    change: >-
      Re-point the two root-manifest citations to (Cargo.toml:106; Cargo.toml:116) — the taffy and parley `git … rev` lines, each moved +2 by the chunk's inserts at root `Cargo.toml:18` and `:62`; the claim is unchanged.
    sidecar: "2026-10-07-driver-session: §Dependency Security git-pin citations re-pointed Cargo.toml:104→106, :114→116 (two inserted lines above; no dependency fact changed)."
    rationale: >-
      Report Dependencies: "no third-party crate added or bumped" (report.md:25) — the invariant holds, nothing banned or unvetted. Report Counts: root `Cargo.toml` lines at old `:61` or below moved +2 and security-plan cites root `Cargo.toml` at 2 sites (report.md:32); expected amendment "security-plan §Dependency Security — citation re-point — carried … no dependency fact changed" (report.md:60). Measured: taffy at Cargo.toml:106 (base :104), parley at :116 (base :114).
    basis: Cargo.toml:106
  - detector: D-security-deps
    severity: warning
    section: "§Dependency Security → Pinning — \"every dev-dependency of blitz-tests (tests/blitz-tests/Cargo.toml:15-38)\" (security-plan.md:219)"
    change: >-
      Re-point the range to (tests/blitz-tests/Cargo.toml:15-40): the dev-dependency block now ends at `:40`, the chunk having added `escher-driver = { workspace = true }` at `:23`; the claim — every dev-dependency inherits `workspace = true` — still holds with the new edge.
    sidecar: "2026-10-07-driver-session: blitz-tests dev-dependency range re-pointed to Cargo.toml:15-40 (+1 line, `escher-driver` at :23, workspace-inherited)."
    rationale: >-
      Report Files: `tests/blitz-tests/Cargo.toml` (+1 line, `:23`) and the new edge `blitz-tests → escher-driver` (dev-dependency) (report.md:12, :24). Measured: the block runs :15-40 now; it ran :15-39 at the base `7d9f351d`, so the cited `:15-38` was already one line short before this chunk.
    basis: tests/blitz-tests/Cargo.toml:23
    dependent-of: D-security-deps

# Notes for the orchestrator (outside the three detectors, no proposal made):
# - PROVISIONAL marks: 10 hits on security-plan.md:107-109. Their removal and the ratification record are a
#   Decisions & corrections direction (report.md:91), not a Changes fact, and no security-plan detector covers
#   them. They sit in the same three rows the :107 and :108 proposals above edit — apply in one pass per row.
# - Rows :107-:109 also say "no log, event, socket or file carries" the snapshot, its text or a diff and
#   "no driver, CLI or MCP command exposes" them yet. Left unproposed: the report keeps them true — the wire
#   carries pid, label and served count only (report.md:19) and no verb set, CLI JSON or MCP tool was built
#   (report.md:44).
# - D-security-deps: the invariant itself is not violated; both entries are citation re-points only.
# - Sweep for the retired "no listener" claim in security-plan.md: one hit, :185 (the `socket` at :370 is the
#   cold-agent client's own path, unrelated).
```

## design-system

verdict: 0 proposal(s) · parsed · entities 0 found, 0 after decode · 2 comment line(s) around the YAML (the agent's notes, kept below, not proposals)

```yaml
proposals: []
# D-design-tokens: no drift. All five Coverage lines in the report (report.md:71-75) carry `tokens n/a`; none reads `hardcoded✗`. The chunk rendered no new UI element (report.md:75 "tokens n/a (no UI rendered)"; report.md:68 "the session renders no new markup and uses no colour or type token").
# Not verified by me: the report's claim (report.md:33, :68) that design-system.md:128's citation `examples/seven_guis/Cargo.toml:31` did not move — taken from the report, not re-measured against the manifest.
```

## layout-templates

verdict: 2 proposal(s) · parsed · entities 0 found, 0 after decode · 6 comment line(s) around the YAML (the agent's notes, kept below, not proposals)

```yaml
proposals:
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: cli → Primary screens"
    change: "Add a bullet after the `scripts/cold-agent.sh` entry: `escher-session` — the session host binary of the `seven_guis` package: usage `escher-session <counter|flight-booker|timer|crud> <state-dir>`, argv closed at exactly two arguments with the task one of the stand's four lean-task slugs; any other argv prints that usage line to stderr and exits 2 having booted nothing and created no state; exit 0 after the session is stopped, exit 1 with the error's fixed-string message on stderr; it writes nothing to stdout on any path, and its stderr log lines carry `service.name=seven_guis`; unix only in 0.1.0 (examples/seven_guis/src/session_host.rs:13-18; examples/seven_guis/src/session_host.rs:29-32; examples/seven_guis/src/session_host.rs:47-53; examples/seven_guis/Cargo.toml:45-47)"
    sidecar: "2026-10-07-driver-session: §Surface: cli → Primary screens gains the `escher-session <task> <state-dir>` host entry (closed argv, exit codes 0 · 1 · 2, empty stdout) — the chunk's new command-line surface."
    rationale: "Report Changes → Symbols / APIs adds binary `escher-session` with a closed argv `<task> <state-dir>`, a usage line on stderr and exit codes 0 · 1 · 2, and Coverage of new surfaces lists 'the `escher-session` binary's argv' as a new external surface; Expected amendments names this exact site ('layout-templates §Surface: cli → Primary screens — the `escher-session <task> <state-dir>` host entry — carried'). The document has no entry for it: `escher-session`, `escher-driver` and `session.sock` occur 0 times in layout-templates.md (the only 'session' hit is :72, the cold-agent transcript). An undocumented surface is drift."
    basis: "examples/seven_guis/src/session_host.rs:30 (the usage line `usage: escher-session <counter|flight-booker|timer|crud> <state-dir>`, followed by `ExitCode::from(2)` at :31); insertion site .andromeda/layout-templates.md:72 (last bullet of the list that opens at :69)"
  - detector: D-layout-surface
    severity: warning
    section: "§Surface: desktop-native → Primary screens"
    change: "In the `seven_guis Home and TaskShell` bullet, after the headless-stand clause (the `task_in_shell` mount at the pinned 800 × 600, scale 1.0, Light viewport), add: the `escher-session` host is a third consumer of that same mount — it boots one lean task through `seven_guis::stand` only (`stand::boot` / `stand::boot_timer` with `stand::options(true)`) and holds the instance across commands, adding no markup, id, class, style, wrapper or order; a held instance reads as a fresh boot and the TaskShell mount and the pinned viewport hold after commands, in both layout modes (examples/seven_guis/src/session_host.rs:36-43; tests/blitz-tests/tests/stand_session_fresh.rs:40; tests/blitz-tests/tests/stand_session_fresh.rs:85-109)"
    sidecar: "2026-10-07-driver-session: §Surface: desktop-native → Primary screens names the `escher-session` host as a third consumer of the TaskShell mount (after the windowed app and the headless stand checks); no markup or viewport value changed."
    rationale: "Report Expected amendments: 'layout-templates … §Surface: desktop-native → Primary screens — the session as a third consumer of the TaskShell mount — carried: Symbols / APIs (the binary's argv) and Coverage. Sites: layout-templates.md:67 and :5.' Symbols / APIs states the binary 'boots through `seven_guis::stand` only'; Coverage lists 'the held instance' as a new surface with 'a11y ✓ unchanged surface, read not altered … no markup, role, name or focusability changed · tokens n/a (no UI rendered)'; Outcome: '(design) (layouts) a held instance reads as a fresh boot; mount and viewport hold — met: `stand_session_fresh` 2 passed'. The seven_guis bullet names two consumers of the mount (the windowed app, the headless stand) and not the session. This is a second site of the same new surface, not a restatement of a retired claim — the existing sentence 'the headless stand skips Home and mounts one lean task … in TaskShell' stays true and is extended, not replaced — so it carries no `dependent-of`."
    basis: ".andromeda/layout-templates.md:10 (the seven_guis bullet; the headless-stand clause cites examples/seven_guis/src/stand.rs:43-69); examples/seven_guis/src/session_host.rs:38 and :42 (the two `stand::` boot calls)"

# Notes for the orchestrator (not proposals)
# - No claim in layout-templates is retired by this chunk, so the duplicate-occurrence sweep found nothing to add: grep for `escher-session|escher.driver|session` over the document hits only :72 (cold-agent's "raw session transcript"), which is unrelated and stays as written.
# - The report cites the cli site as `layout-templates.md:67`; :67 is the `### Primary screens` heading, the bullets run :69-72.
# - Not proposed: the `> NOT YET MEASURED` note at :65 ("did not reach any further CLI help or argument parser beyond the `paint_bench` usage line") already sits beside the agent-run and cold-agent entries; it is baseline prose about a past reading, not a fact this chunk's Changes moved.
# - layout-templates has 0 root-`Cargo.toml` citations and 0 `examples/seven_guis/Cargo.toml` citations (report Counts), so the line-shift re-point touches nothing here. The first proposal introduces one new citation, `examples/seven_guis/Cargo.toml:45-47`, measured against the current file.
# - Line numbers in both `change` texts were read from the current working tree (session_host.rs is 57 lines, matching the report).
```

## test-plan

verdict: 19 proposal(s) · parsed · entities 0 found, 0 after decode · 5 comment line(s) around the YAML (the agent's notes, kept below, not proposals)

```yaml
# test-plan drift pass — chunk 2026-10-07-driver-session
# D-tests-framework: NO DRIFT — every new test runs under cargo test / libtest (`cargo test -p escher-driver --locked --lib`, `-p seven_guis --locked --test host_binary`, `-p blitz-tests --locked --test stand_session_*`, report.md:118-124); report.md:25 lists no third-party crate added or bumped; `scripts/agent-run.sh` and `.github/` unchanged (report.md:37).
# D-tests-coverage: every new surface carries unit and/or integration tests (report.md:71-75) with ONE gap the report names itself (report.md:45, :47, :110) — first proposal below; the rest bring test-plan's coverage statements up to the tests the chunk added.
# D-tests-obs-harness: the 5-command script, its status shape and both log formats are unchanged (report.md:26, :37), but three §3 statements now read wider than what is true, and the obs-plan half (obs-plan.md:67, :83, :322) is expected to move — the proposals below are test-plan's half and are one-sided unless obs-plan §3 lands its own.
# Not proposed: `.claude/docs/tests-summary.md:28` (a distillation); test-plan.md:281 (`examples/seven_guis/Cargo.toml:31`, lines 1-34 byte-identical, report.md:33); test-plan.md:116 (cold-agent pipe, untouched); test-plan.md:139 (`Ran 64 tests`, unchanged, report.md:31).
proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Boundaries covered (new bullet: session host quietness)"
    change: >-
      Add a bullet stating what `stand_session_quiet` establishes and what it does not: its host is the re-run test binary (`#[ignore]` child `session_host_with_typed_text`), which installs no log sink, so its `RUST_LOG=trace` is inert — it proves the session library and the harness path write nothing of the screen to stdout, stderr or the state directory (listing = `session.sock` alone) and that nothing on that path reads `RUST_LOG`; it does NOT cover a host that installs escher's sink — the `escher-session` binary measured on CRUD reads 0 ids / 0 names at `warn` and `info`, stable ids at `debug`, ids and accessible names at `trace` through third-party targets outside the sink's allowlist; typed text is not measured (no command can type into the binary's instance yet) and the windowed stand is not measured by level; the remaining check is owed by the route entry added next (the sink dropping records from targets outside its allowlist), as measured at escher-0.1.0/chunks/2026-10-07-driver-session/evidence/host-stderr-by-level.md.
    sidecar: "2026-10-07-driver-session — §5: recorded that `stand_session_quiet` covers the sink-less host only; a sink-installing host at debug/trace is untested and owed to the next route entry (founder's ruling, «Честно записать + чанк следом»)."
    rationale: >-
      The one new path with no adequate test at its tier: report.md:45 (Insufficient fixes — the check is green and does not establish its criterion for a host that logs), report.md:47 (disproved claim 1), report.md:110 (criterion "met as the check is written, UNMET as a statement about a host that installs the sink"), report.md:90 (the ruling: scope the wording to the measurement, remedy in the next chunk). Sweep hazard report.md:97 says a zero over a sink-less child passes vacuously.
    basis: "tests/blitz-tests/tests/stand_session_quiet.rs:29-30 (the check), :90-92 (the #[ignore] host child); report.md:45-47"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope (new bullet: escher-driver)"
    change: >-
      Add a bullet `escher-driver`: 13 inline unit tests in three files — 3 in error.rs (every `SessionError` message non-empty and holding no path, an exit code not in the message, the `Io` message adding the kind), 2 in session.rs (a label of the closed set valid; a label outside it refused before the boot runs), 8 in wire.rs (each request and each reply round-trips, a known word with another version refused by version, anything else and non-text bytes refused as malformed, a reply outside the protocol not parsing, a line read up to its terminator and no further, an over-long and an unterminated line named); `host.rs` and `client.rs` hold no unit test — the socket, the spawn and each lifecycle edge are covered in `stand_session_lifecycle` and `host_binary`; its doc-test line and the `escher-session` binary's unit-test target read 0 — as measured at escher-0.1.0/chunks/2026-10-07-driver-session/report.md.
    sidecar: "2026-10-07-driver-session — §1: added the `escher-driver` coverage bullet (13 unit tests: error 3 · session 2 · wire 8)."
    rationale: "New workspace member with 13 unit tests (report.md:24, :30, :112, :118); §1 lists every escher-owned crate's tests (escher-telemetry at test-plan.md:20) and has no line for this one."
    basis: "packages/escher-driver/src/error.rs:69-103; packages/escher-driver/src/session.rs:68-76; packages/escher-driver/src/wire.rs:151-291; report.md:118"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests (test-plan.md:25)"
    change: >-
      Replace "one shared module that is no target, `tests/common/mod.rs`" with two shared modules that are no target — `tests/common/mod.rs` and `tests/session_common/mod.rs` (what the session checks share) — and extend the stand coverage with the held session: state, focus and timer time read by a later command on one `Session` in both layout modes (`stand_session_state`, 3), a control's stable id equal to its node's `author_id` after a command and a CRUD row keeping its id across commands (`stand_session_ids`, 2), a held instance reading as a fresh boot with mount and viewport holding (`stand_session_fresh`, 2), start · attach · stop and each edge over the Unix-domain socket with the `0700` directory and `0600` socket modes (`stand_session_lifecycle`, 1 + the `#[ignore]` host child `session_host`), and nothing of the screen written by the sink-less session path (`stand_session_quiet`, 1 + the `#[ignore]` host child `session_host_with_typed_text`; its limit in §5); add the five files to the citation list.
    sidecar: "2026-10-07-driver-session — §1 blitz-tests bullet: five `stand_session_*` checks (+9 tests, +2 ignored children) and the second shared module `session_common/mod.rs` added."
    rationale: "report.md:11 (five new stand checks and `session_common/mod.rs`, 'no test target and no test in it'), report.md:29 (+3/+2/+2/+1/+1), report.md:34 (re-exec children 2 → 4), report.md:105-108. The bullet's 'one shared module' is retired by the new module."
    basis: "test-plan.md:25; tests/blitz-tests/tests/session_common/mod.rs:1-6; tests/blitz-tests/tests/stand_session_state.rs:56-133; stand_session_ids.rs:52-100; stand_session_fresh.rs:39-85; stand_session_lifecycle.rs:39-100; stand_session_quiet.rs:29-92"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern (test-plan.md:63)"
    change: >-
      Restate "one shared module that is no target — `tests/common/mod.rs`, read through `mod common;` by six `stand_*.rs` checks" as two shared modules that are no target: `tests/common/mod.rs`, read through `mod common;` by eleven `stand_*.rs` checks (the six named plus the five `stand_session_*`), and `tests/session_common/mod.rs`, read through `mod session_common;` by `stand_session_fresh`, `_ids`, `_lifecycle` and `_quiet`; add that `examples/seven_guis` now holds a `tests/` directory with one integration-test target, `host_binary.rs`.
    sidecar: "2026-10-07-driver-session — §2 Directory pattern: second shared module `session_common/mod.rs`; `mod common;` readers six → eleven; `examples/seven_guis/tests/` named."
    rationale: "Second occurrence of the retired 'one shared module' claim (report.md:11, :87); the reader counts are measured from the new files the report lists, not stated in the report — re-derive before applying."
    basis: "test-plan.md:63; `mod common;` at tests/blitz-tests/tests/stand_session_fresh.rs:11, stand_session_ids.rs:12, stand_session_lifecycle.rs:17, stand_session_quiet.rs:16, stand_session_state.rs:11; `mod session_common;` at stand_session_fresh.rs:12, stand_session_ids.rs:13, stand_session_lifecycle.rs:18, stand_session_quiet.rs:17"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (stand checks) (test-plan.md:97)"
    change: >-
      Scope "`tests/common/mod.rs` is the one statement of what the stand checks share" to the tables and helpers it holds, extend its reader list with the five `stand_session_*` checks, and add a sibling bullet for `tests/session_common/mod.rs` — no test and no target; for a session held in process `slug(task)`, `hold(task, incremental) -> (Session, TimerTicks)` and `change(task, &mut session, &ticks)`; for a session held by a process of its own `state_dir(name)` under `CARGO_TARGET_TMPDIR`, `clear`, `host_command(child)` (this test binary re-run on one `#[ignore]` child with `--ignored --exact {child} --nocapture`), the bounds `READY` 60 s and `EXIT` 10 s, `SOCKET_FILE`, and the guard `Host` that kills and reaps a host a failing check still holds; it exists beside `common/` because that module was frozen by the chunk's preservation gate.
    sidecar: "2026-10-07-driver-session — §3 Crate-local test helpers: `session_common/mod.rs` bullet added; `common/mod.rs` 'the one statement' scoped and its readers extended."
    rationale: "Third occurrence of the retired single-shared-module claim, worded as 'the one statement of what the stand checks share' (report.md:11, :37, :87)."
    basis: "test-plan.md:97; tests/blitz-tests/tests/session_common/mod.rs:1-6, :22, :33, :46, :60, :63, :66, :70, :75, :81, :90"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → apps (rdme, bump, examples, accesskit_xplat) (test-plan.md:10)"
    change: >-
      Keep the slice-s04 search as its past reading and add what holds since: `examples/seven_guis` carries one integration-test target, `tests/host_binary.rs` (2 tests — `the_binary_serves_and_stops`: the `escher-session` binary installs the sink first, writes nothing to stdout, stamps `service.name=seven_guis`, exits 0 after `stop` leaving no state directory; `an_unknown_task_is_refused_before_anything_boots`: exit 2, nothing booted, no state created), run as `cargo test -p seven_guis --locked --test host_binary`; the other examples and the apps named stay without tests.
    sidecar: "2026-10-07-driver-session — §1: the 'examples: no tests' reading qualified — `examples/seven_guis/tests/host_binary.rs` (2) is the stand crate's first integration test."
    rationale: "report.md:10 (`host_binary.rs` — 'the crate's first `tests/` directory'), report.md:24 (one integration-test target), report.md:72, :111, :119. Slice s04 includes `examples/seven_guis`, so the bullet's 'no tests' no longer holds for it."
    basis: "test-plan.md:10; examples/seven_guis/tests/host_binary.rs:37-38, :102-103; .andromeda/runs/2026-10-05T19-08-04-adopt/slices.md:82 (s04 units name examples/seven_guis)"
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Observed absent (test-plan.md:178)"
    change: "Qualify 'Integration tests outside `src` in the apps … every hit sits inside a `#[cfg(test)] mod tests` in `src`' as the reading at that search, and add: since 2026-10-07-driver-session `examples/seven_guis/tests/host_binary.rs` is an integration test outside `src` in that slice."
    sidecar: "2026-10-07-driver-session — §5 Observed absent: the slice-s04 'no integration tests outside src' reading qualified by `examples/seven_guis/tests/host_binary.rs`."
    rationale: "Same retired claim as test-plan.md:10, restated over the same 86 files of slice s04 (report.md:10, :24)."
    basis: "test-plan.md:178; examples/seven_guis/tests/host_binary.rs:1-121"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§6 E2E Test Strategy → Observed absent (test-plan.md:201)"
    change: "Qualify 'End-to-end or UI-driving tests in the apps … no test launches a window or document' as the reading at that search, and add: since 2026-10-07-driver-session `host_binary` launches the `escher-session` binary, which boots a lean task headless and serves its lifecycle; no test launches a window and none drives the UI through that binary (no verb beyond `hello` and `stop` exists)."
    sidecar: "2026-10-07-driver-session — §6 Observed absent: 'no test launches a window or document' qualified — `host_binary` launches a headless session host; still no windowed or UI-driving test."
    rationale: "Third statement of the slice-s04 absence, worded as a launch claim; report.md:21 (the binary boots through `seven_guis::stand`), report.md:44 (no verb set, no act-by-id built), report.md:111."
    basis: "test-plan.md:201; examples/seven_guis/tests/host_binary.rs:37-38"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover (new bullet: escher-driver)"
    change: >-
      Add a bullet `escher-driver`: wire.rs tests pin the closed lifecycle grammar — `hello v1` / `stop v1` and the four replies round-trip, another `v{digits}` is `refused version`, anything else (non-text bytes, a request line over 64 bytes, an unterminated line) is malformed, a reply outside the protocol does not parse, a line is read to its terminator and no further; session.rs tests pin the label rule (1-32 bytes of `a-z0-9-`) and that an invalid label returns `InvalidLabel` without the boot closure running; error.rs tests pin that every `SessionError` message is a non-empty fixed string holding no path, exit code or value, the `Io` arm adding only the kind's name (spelled "input or output", so a no-`/` assertion holds).
    sidecar: "2026-10-07-driver-session — §4: added the `escher-driver` unit-coverage bullet (wire grammar, label rule, error messages)."
    rationale: "report.md:16, :19, :71, :73 (unit tests in wire.rs, the label's unit tests), report.md:96 (the 'I/O' sweep hazard), report.md:112 (13 passed)."
    basis: "packages/escher-driver/src/wire.rs:151-291; packages/escher-driver/src/session.rs:68-76; packages/escher-driver/src/error.rs:69-103"
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Boundaries covered (new bullets: held session; session host ↔ lifecycle socket)"
    change: >-
      Add two bullets. Session ↔ held instance: one `escher_driver::Session` booted through `seven_guis::stand` holds state, focus, ids, CRUD rows and timer time across commands and reads as a fresh boot, in both layout modes (`stand_session_state`, `stand_session_ids`, `stand_session_fresh`). Session host ↔ lifecycle socket (unix only, `cfg(unix)`-gated): `start` spawns a host, `attach` reads `Hello {pid, label, served}`, `stop` leaves no state directory, and each edge — `AlreadyRunning`, `NoSession`, `Dead` with the stale socket replaced, refusals changing nothing, the `0700` directory and `0600` socket — is asserted over a real socket against this test binary re-run on an `#[ignore]` child (`stand_session_lifecycle`) and against the real `escher-session` binary through `CARGO_BIN_EXE_escher-session` (`host_binary`); non-unix arms return `Unsupported`, witnessed only by the fork's CI windows leg (CI#37580856074).
    sidecar: "2026-10-07-driver-session — §5: added the held-session and session-host-over-socket boundaries."
    rationale: "report.md:62 (expected amendment 'test-plan §1 · §5 — carried'), report.md:71-75 (Coverage of new surfaces), report.md:34 (`cfg(unix)`, `CARGO_BIN_EXE_*` uses 0 → present), report.md:39 (windows the only witness of the non-unix arms)."
    basis: "tests/blitz-tests/tests/stand_session_lifecycle.rs:39-40; examples/seven_guis/tests/host_binary.rs:37-38; report.md:17-18, :71"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test levels observed (new bullet: process-lifecycle checks)"
    change: >-
      Add a level bullet: process-lifecycle checks start and stop their own session host — the test binary re-run on one `#[ignore]` child (`--ignored --exact {child} --nocapture`) or the package's own binary through `CARGO_BIN_EXE_*` — with every wait bounded, a guard that kills and reaps a host a failing check still holds, state under `CARGO_TARGET_TMPDIR` removed at `stop`, and the child's streams sent to null so its libtest lines never reach the parent's output; they are `cfg(unix)`-gated; the two `#[ignore]` host children serve until stopped, so a bare `--ignored` run of blitz-tests blocks on them; a check that captures a sink-installing host at `debug` or below must drain while it runs or capture to a file (about 1.1 MB of stderr at `trace` against a 64 KiB / 16 KiB pipe).
    sidecar: "2026-10-07-driver-session — §2: added the process-lifecycle check level (re-exec host children, bounded waits, null streams, `cfg(unix)`, the `--ignored` blocking hazard)."
    rationale: "report.md:34, :37 (the process checks' shape), report.md:83 (streams to null — inherited lines would be counted by agent-run.sh as extra tests), report.md:85 (the `#[ignore]` hosts block under `--ignored`), report.md:98 (pipe-fill hazard). §2 lists two blitz-tests styles (test-plan.md:47) and neither names a spawned host."
    basis: "test-plan.md:47; tests/blitz-tests/tests/session_common/mod.rs:60-90; tests/blitz-tests/tests/stand_session_lifecycle.rs:98-100; tests/blitz-tests/tests/stand_session_quiet.rs:90-92"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Proof (test-plan.md:112)"
    change: "Append the link: re-counted at 2026-10-07-driver-session: `run stand` 72 `ok` stand events (+3 `stand_session_state`, +2 `stand_session_ids`, +2 `stand_session_fresh`, +1 `stand_session_lifecycle`, +1 `stand_session_quiet`, all picked up by the `stand_` prefix with no script change; `session_common/` is a directory and is not selected), `run.end` passed 72 · failed 0 · ignored 3 (+2 `#[ignore]` host children) — as measured at escher-0.1.0/chunks/2026-10-07-driver-session/report.md."
    sidecar: "2026-10-07-driver-session — §3 Proof: `run stand` re-count 63 → 72 `ok`, ignored 1 → 3."
    rationale: "report.md:29 (63 → 72, ignored 1 → 3, stated for test-plan.md:112), report.md:37, :114, :135."
    basis: "test-plan.md:112; report.md:29; escher-0.1.0/chunks/2026-10-07-driver-session/evidence/gate-readings.md"
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Local baseline (`cargo test --workspace` re-count chain, test-plan.md:315)"
    change: "Append the link: re-counted at 2026-10-07-driver-session: 140 result lines, 572 passed · 0 failed · 7 ignored (+13 `escher-driver` unit tests in a new lib result line, +1 line its doc-tests at 0, +1 line the `escher-session` binary's unit-test target at 0, +2 `host_binary` in a new line, +9 in the five `stand_session_*` lines; +2 ignored host children), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` runs at /implement and on its pre-CI commit 91484eb0 (escher-0.1.0/chunks/2026-10-07-driver-session/report.md)."
    sidecar: "2026-10-07-driver-session — §9 Local baseline: workspace re-count 131 · 548 · 0 · 5 → 140 · 572 · 0 · 7."
    rationale: "report.md:30 (the re-count and its breakdown, stated for test-plan.md:315), report.md:115, :143."
    basis: "test-plan.md:315; report.md:30; target/ci-logs/test.log as summed in escher-0.1.0/chunks/2026-10-07-driver-session/evidence/operator-pass.md"
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Boundaries covered → tests/blitz-tests crate (test-plan.md:164)"
    change: "Re-point the root-manifest citation `(Cargo.toml:23)` to `(Cargo.toml:24)` — the `tests/blitz-tests` member line moved +1; `tests/blitz-tests/Cargo.toml:13` did not move (the chunk's one added line there is `:23`)."
    sidecar: "2026-10-07-driver-session — §5: root `Cargo.toml:23` → `:24` (member line shifted by the `escher-driver` insert)."
    rationale: "A citation re-point, not an invariant violation — carried here because report.md:32 counts test-plan's 2 root-`Cargo.toml` sites and report.md:57, :63 carry the re-point; every line at old `:18` or below moved +1."
    basis: "Cargo.toml:24 (`\"tests/blitz-tests\",`); test-plan.md:164"
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Local baseline (test-plan.md:312)"
    change: "Re-point `(Cargo.toml:199-200)` to `(Cargo.toml:201-202)` — `[profile.dev]` and `debug = \"line-tables-only\"` moved +2."
    sidecar: "2026-10-07-driver-session — §9: root `Cargo.toml:199-200` → `:201-202` (shifted by the two `escher-driver` inserts)."
    rationale: "Second of test-plan's 2 root-`Cargo.toml` sites (report.md:32): every line at old `:61` or below moved +2."
    basis: "Cargo.toml:201-202; test-plan.md:312"
    dependent-of: D-tests-coverage
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Invocation (test-plan.md:103)"
    change: >-
      Scope the premise sentence to the script: `agent-run.sh` itself starts no daemon, writes no PID file, has no status endpoint and binds no port or socket, and reads no environment variable of its own; the stand checks it runs boot in process, bar two under `run stand` — `stand_session_lifecycle` and `stand_session_quiet` — which start and stop their own session host process and its Unix-domain socket (`{state_dir}/session.sock`, the workspace's one listener site, `packages/escher-driver/src/host.rs`), a surface of the session library and its checks (§3 → Session lifecycle), not of the contract.
    sidecar: "2026-10-07-driver-session — §3 Agent-run Invocation: 'nothing is started, polled or listened on' scoped to `agent-run.sh`; two session checks under `run stand` spawn a host and bind a socket."
    rationale: "report.md:61 (expected amendment: the premise at test-plan.md:103 'to be read as scoped to that contract now that a library socket exists'), report.md:37 ('The agent-run contract itself still starts no daemon and binds nothing — the socket belongs to the session library and to the checks'), report.md:20, :34 (listener sites 0 → 1). As written, 'The stand is an in-process library boot, so nothing is started … or listened on' is false of a `run stand` that executes the two process checks."
    basis: "test-plan.md:103; report.md:37; packages/escher-driver/src/host.rs:20"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → State (test-plan.md:111)"
    change: "Keep `target/agent-run/{status.json, events.jsonl, run.log}` as the script's only state and add: during `run stand` the two session process checks hold `target/tmp/ss-life` and `target/tmp/ss-quiet` (`{CARGO_TARGET_TMPDIR}`, each a `0700` directory holding `session.sock` alone, removed at `stop`); `cleanup` does not touch them and nothing remains after a green run."
    sidecar: "2026-10-07-driver-session — §3 Agent-run State: the session checks' transient state directories under `target/tmp/` named beside the script's own state."
    rationale: "The same nothing-else premise restated as '… only' (test-plan.md:111); report.md:37 (state directories `ss-life`, `ss-quiet`, each removed at `stop`), report.md:65 (obs-plan §9 at obs-plan.md:322 gains the session state directory as harness state — test-plan's State line must agree), report.md:148 (`target/tmp/` empty after the runs)."
    basis: "test-plan.md:111; tests/blitz-tests/tests/session_common/mod.rs:70; report.md:37"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Stand log format (test-plan.md:99)"
    change: >-
      Name both binaries of the `seven_guis` package: `seven_guis_native` and `escher-session` each install the same sink first and write the same one-line stderr format under `service.name=seven_guis` (stdout empty on every path of `escher-session`), the second exercised by `host_binary`; and scope "with escher-telemetry's allowlist scrub applied" to what was measured — the scrub covers engine targets and content-named fields; on `escher-session` (CRUD) stderr holds 0 stable ids and 0 accessible names at the default level and at `info`, while records from third-party targets outside the allowlist print unredacted, carrying ids at `debug` and ids and names at `trace`; typed text and the windowed binary by level are not measured (obs-plan §3, §8).
    sidecar: "2026-10-07-driver-session — §3 Stand log format: second sink-installing binary `escher-session` named; the scrub statement scoped to the measured levels."
    rationale: "report.md:21, :23 (`escher_telemetry::init` gains its second install site, same `service.name`), report.md:64 (obs-plan §1 · §3 at obs-plan.md:67 gains the second binary — a one-sided change unless test-plan §3 names it too), report.md:46-49 and :90 (the ruling: scope the wording to the measurement). The log format itself is unchanged (report.md:26, `packages/escher-telemetry` untouched)."
    basis: "test-plan.md:99; examples/seven_guis/src/session_host.rs:20; escher-0.1.0/chunks/2026-10-07-driver-session/evidence/host-stderr-by-level.md; obs-plan.md:67"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → Session lifecycle (new keyed contract: a row in .andromeda/registries/test-plan-contracts.toml + a file under registries/contracts/test-plan/)"
    change: >-
      Create the key stating the session surface the checks drive: `Session::start(label, boot)` (label 1-32 bytes of `a-z0-9-`, validated before the caller's boot runs once); `serve(state_dir, session)`; `start(state_dir, host, ready)` · `attach(state_dir) -> Hello {pid, label, served}` · `stop(state_dir)`; the wire `hello v1` / `stop v1` → `ok v1 pid= label= served=` · `ok v1 stopping` · `refused version` · `refused malformed`, request line ≤ 64 bytes, reply ≤ 128, carrying no element id, name, value, snapshot text or diff; state = a caller-named `0700` directory holding `session.sock` (`0600`) alone, nothing after `stop`; the host binary `escher-session <task> <state-dir>` (task `counter` · `flight-booker` · `timer` · `crud`), exit `0` · `1` · `2`, stdout empty; unix only (`Unsupported` elsewhere); `SessionError`'s eleven variants; and the check obligations — a check starts and stops its own host, bounds every wait, guards with kill-and-reap, nulls the child's streams, and proves quietness only against a host that installs the sink (owed; §5). It is not part of the 5-command agent-run contract, which starts and binds nothing.
    sidecar: "2026-10-07-driver-session — §3: new keyed contract 'Session lifecycle' (API, wire v1, state directory, host binary argv and exit codes, check obligations)."
    rationale: "report.md:61 (expected amendment: 'a session-lifecycle contract key — the key is new'), report.md:15-21 (Symbols / APIs), report.md:26 (Schema / config: wire grammar, label rule, state layout, exit codes), report.md:55 (0 hits for `escher.driver|Session::start|session\\.sock` in every master and key file). The existing key file (bootstrap-phases, coverage-tooling-install) is untouched by this chunk."
    basis: "packages/escher-driver/src/session.rs:23-59; host.rs:20; client.rs:11-62; wire.rs:16-20; error.rs:10; examples/seven_guis/src/session_host.rs:13-20; .andromeda/registries/test-plan-contracts.toml (one row today)"
```

## obs-plan

verdict: 17 proposal(s) · parsed · entities 0 found, 0 after decode · 23 comment line(s) around the YAML (the agent's notes, kept below, not proposals)

```yaml
# obs-plan drift pass — chunk 2026-10-07-driver-session
# Read: report.md (whole), obs-plan.md (whole, 346 lines), the one keyed-contract file
# (.andromeda/registries/contracts/obs-plan/bootstrap-phases-derive-for-route-setup-project.md), directives.md.
# Line numbers below are obs-plan.md body lines as read at this pass; root Cargo.toml lines were measured on the working tree.
#
# Detector verdicts
#   D-obs-pii             VIOLATED — 1 primary + 2 dependents. The escalation is already ruled on by the founder
#                         (directives.md item 5; report Decisions & corrections): scope the wording to the measurement,
#                         remedy is a route entry ahead of Settle detection. Proposals below only scope wording.
#   D-obs-stack           The library used is on-spec (escher_telemetry::init, no third-party crate added — report
#                         Dependencies). What drifted is §3's statement of WHO installs the sink: 1 primary + 7 dependents,
#                         plus the §9 state row and 4 citation re-point lines the report lists as carried for obs-plan.
#   D-obs-instrumentation NOT strictly violated — §4–§6 record no requirement (all "observed absent"; §10/§11 no recorded
#                         intent) and the report marks the socket "instrumentation n/a" by the plan's constraint. One
#                         census-extension proposal is offered so §4 does not stay silent on the new crate; decline it
#                         if a census line is not wanted.
#
# Swept and left unchanged (true as written): :83 and :294 agent-run log ("the stand checks install no escher sink",
# "no user content" — report gate: content-named-key count 0); :230 ("seven_guis_native has since gained…" — historical,
# not exclusive); :292 scrub statement (report disproved-claim 3: "accurate as written"); :320 and :322 "no user data"
# (lifecycle child streams go to null, quiet child has no sink, host_binary runs at info = 0 ids / 0 names);
# keyed contract label otel-sdk-install (report: no OTel, metrics or exporter crate entered).
# Citations :4 (line 8) and :17 (line 11) did not move (both above the insert at root Cargo.toml:18).

proposals:
  - detector: D-obs-pii
    severity: escalate
    section: "§8 PII Scrubbing & Compliance → Values logged as-is (the 'Past escher's scrub' bullet, obs-plan.md:288)"
    change: "Add a third class to the bullet: a record from a target OUTSIDE the engine allowlist prints its message and fields as written — measured on the `escher-session` binary (CRUD task, one `hello`, then `stop`): default `warn` 0 stderr lines; `info` 1 line, 0 stable ids, 0 accessible names; `debug` stable ids (`crud-surname` x12) through Stylo's `style::traversal` / `style::sharing` over the `log` bridge; `trace` ids (x20) and an accessible name (fixture row `Emil` x3) through `dioxus_core::diff::node`; stdout 0 bytes at every level. Typed text NOT measured (no command can type yet); the windowed `seven_guis_native` NOT measured by level (same sink, same components; its one reading is the 10 s boot smoke at `RUST_LOG=info`, no stand id). Cite escher-0.1.0/chunks/2026-10-07-driver-session/evidence/host-stderr-by-level.md."
    sidecar: "§8 'Past escher's scrub' gains the non-allowlisted-target class: the session host is quiet at warn/info, prints stable ids at debug and accessible names at trace through third-party targets; typed text and the windowed stand by level unmeasured (founder's ruling 2026-10-07, «Честно записать + чанк следом»)."
    rationale: "Report 'Spec claims disproved by measurement' 3 names obs-plan.md:288 as incomplete — it omits the measured class; Coverage of new surfaces marks the `escher-session` argv 'raw✗ at debug (stable ids) and trace (ids and accessible names)'. The founder's ruling (Decisions & corrections; directives.md item 5) directs the obs wording be scoped to what was measured, so the escalation stands resolved and this is the scoping edit."
    basis: ".andromeda/obs-plan.md:288 · escher-0.1.0/chunks/2026-10-07-driver-session/evidence/host-stderr-by-level.md:12-17 · packages/escher-telemetry/src/format.rs:17-24"

  - detector: D-obs-pii
    severity: escalate
    section: "§8 PII Scrubbing & Compliance → Scrubbing (the 'Its reach is that sink' bullet, obs-plan.md:293)"
    change: "State the sink's reach for both installers and its limit: the sink is installed by the two binaries of the `seven_guis` package (`seven_guis_native`, `escher-session`), whose engine `tracing` features stay off; the allowlist scrub covers engine-prefixed targets and the content-named fields only — a record from any other target (Stylo's `style::*`, `selectors::matching`, `dioxus_core::*`) passes with its message and fields unredacted (see Values logged as-is); the remedy — the sink dropping records from targets outside its allowlist — is owed by the route entry the founder's ruling places ahead of Settle detection."
    sidecar: "§8 reach bullet names both installing binaries and states that non-allowlisted third-party targets pass the sink unredacted, remedy owed by the route entry ahead of Settle detection."
    rationale: "Same claim as the primary, restated as the scrub's reach: the bullet names `seven_guis_native` alone and reads as if only engine events could carry content. Report: `escher_telemetry::init` gains its second install site (Symbols / APIs, remaining-caller facts); emitting targets are 'none in the sink's ENGINE_TARGET_PREFIXES, so their message and fields print as written'."
    basis: ".andromeda/obs-plan.md:293 · examples/seven_guis/src/session_host.rs:20 · examples/seven_guis/Cargo.toml:29-35 (no `tracing` feature named)"
    dependent-of: D-obs-pii

  - detector: D-obs-pii
    severity: escalate
    section: "§3 → Bootstrap phases (derive for route / setup-project)"
    change: "Label `pii-scrubbing-wire`: no longer 'discharged for escher's own sink' without qualifier — discharged for engine-prefixed targets and the content-named fields in the two binaries that install the sink (`seven_guis_native`, `escher-session`); OPEN for records from targets outside the allowlist, which print unredacted (stable ids at `debug`, accessible names at `trace`, measured on `escher-session`), owed by the route entry ahead of Settle detection; still open for the upstream apps' `fmt::init()` subscribers and the WPT runner's `env_logger`."
    sidecar: "Bootstrap phases: `pii-scrubbing-wire` re-scoped — discharged for engine targets and content-named fields in both sink-installing binaries, open for non-allowlisted third-party targets (ids at debug, names at trace) and for the upstream sinks."
    rationale: "The key file's label asserts the same claim the primary retires ('discharged for escher's own sink — … in `seven_guis_native`'); the report measures the sink printing ids and names from non-allowlisted targets and adds a second installer. A single-site apply to §8 would leave this restatement standing."
    basis: ".andromeda/registries/contracts/obs-plan/bootstrap-phases-derive-for-route-setup-project.md:4"
    dependent-of: D-obs-pii

  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (subscriber installation) — the `seven_guis_native` install bullet, obs-plan.md:67"
    change: "Add the second install site: `escher-session` (the session host binary of the `seven_guis` package, `[[bin]]` at examples/seven_guis/Cargo.toml:45-47) installs `escher_telemetry::init(escher_telemetry::service_identity!())` first in its native `main` — before argv is read and before anything boots — reporting an `Err` with `eprintln!` and continuing (examples/seven_guis/src/session_host.rs:20-22); same subscriber, stderr only, nothing to stdout on any path; `init` now has two install sites (the first is examples/seven_guis/src/main.rs:6)."
    sidecar: "§3 Logging stack: `escher-session` recorded as the second binary installing escher's sink (session_host.rs:20), stderr only, same subscriber."
    rationale: "Report Symbols / APIs: the binary 'installs `escher_telemetry::init(…)` first (session_host.rs:20; an `Err` is `eprintln!`ed and the process continues)' and '`escher_telemetry::init` gains its second install site'. Expected amendments names this site (obs-plan.md:67). The library is on-spec — no off-spec logger or OTel setup (Dependencies: no third-party crate added) — the drift is §3 naming one installer."
    basis: ".andromeda/obs-plan.md:67 · examples/seven_guis/src/session_host.rs:20-22 · examples/seven_guis/Cargo.toml:45-47"

  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack — the headless-stand bullet, obs-plan.md:68"
    change: "Re-scope the conclusion 'so a headless boot has no escher sink' to the in-process path: the stand module and its `stand_*` checks still install no subscriber, and the census now also covers `packages/escher-driver` (no `tracing` dependency, no subscriber, no env read, no print), the five `stand_session_*` checks and their shared module `tests/blitz-tests/tests/session_common/mod.rs` (it names `CARGO_TARGET_TMPDIR` only through compile-time `env!`); but a headless boot made by the `escher-session` binary runs under that binary's sink. Record the two spawning checks: `stand_session_lifecycle`'s child has stdout and stderr sent to null; `stand_session_quiet`'s child is the re-run test binary, which installs no sink, so the `RUST_LOG=trace` it is started with is inert — its zero says nothing about a sink-installing host; `examples/seven_guis/tests/host_binary.rs` spawns the real binary at `RUST_LOG=info` and captures both streams into its assertions."
    sidecar: "§3 headless-stand census extended to escher-driver, the five stand_session_* checks and session_common/mod.rs (0 sinks, 0 env reads, 0 prints); 'a headless boot has no escher sink' scoped to the in-process path — the escher-session binary boots the stand under its own sink."
    rationale: "Report: the binary 'boots through `seven_guis::stand` only' after installing the sink, so the bullet's blanket conclusion is retired; Expected amendments: 'the headless census extended to the library's in-process path; the two new spawning checks'; Insufficient fixes: the quiet child 'installs no log sink, so the `RUST_LOG=trace` it is started with changes nothing there'; Deviations: 'The lifecycle child's streams go to null'."
    basis: ".andromeda/obs-plan.md:68 · packages/escher-driver/Cargo.toml:13-15 · tests/blitz-tests/tests/stand_session_lifecycle.rs:27 · tests/blitz-tests/tests/stand_session_quiet.rs:46-48 · tests/blitz-tests/tests/session_common/mod.rs:71 · examples/seven_guis/tests/host_binary.rs:51-54"
    dependent-of: D-obs-stack

  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract → Service identity and line format (escher's sink), obs-plan.md:81"
    change: "Replace '`seven_guis_native` logs `service.name=seven_guis`' with: both binaries of the `seven_guis` package — the windowed `seven_guis_native` and the session host `escher-session` — log `service.name=seven_guis`, the identity being the package's `CARGO_PKG_NAME`; `service.name` therefore does not tell the two apart."
    sidecar: "§3 Service identity: both seven_guis binaries stamp service.name=seven_guis; the name no longer identifies one binary."
    rationale: "Report Symbols / APIs: 'Its stderr identity is `service.name=seven_guis` — the same as the windowed `seven_guis_native` binary's, both being binaries of one package'; Outcome (obs) criterion met by `host_binary`."
    basis: ".andromeda/obs-plan.md:81 · examples/seven_guis/tests/host_binary.rs:93"
    dependent-of: D-obs-stack

  - detector: D-obs-stack
    severity: warning
    section: "§1 Obs Scope Summary → Instrumentation scope — the **examples** bullet, obs-plan.md:10"
    change: "Name both emitters: '`seven_guis_native`'s and `escher-session`'s escher-telemetry events on stderr (examples/seven_guis/src/main.rs:6-9; examples/seven_guis/src/session_host.rs:20-22)'."
    sidecar: "§1 examples bullet: escher-session added beside seven_guis_native as an emitter of escher-telemetry events."
    rationale: "Report: second install site of `escher_telemetry::init` at session_host.rs:20; the bullet lists the sink's emitters and names one."
    basis: ".andromeda/obs-plan.md:10 · examples/seven_guis/src/session_host.rs:20-22"
    dependent-of: D-obs-stack

  - detector: D-obs-stack
    severity: warning
    section: "§2 Telemetry Strategy → Telemetry mechanism (current truth) — first bullet, obs-plan.md:30"
    change: "Widen the actor: escher-telemetry 'always compiles its startup and panic events into the two native binaries of the `seven_guis` package (`seven_guis_native`, `escher-session`), whose engine `tracing` features stay off' — the stand manifest's native dependency block names no `tracing` feature and `escher-driver` names no feature on either dependency."
    sidecar: "§2 mechanism bullet: startup and panic events are compiled into both seven_guis binaries, engine tracing features off in both."
    rationale: "Report Crates / modules: `seven_guis → escher-driver` added to the native-target block; `escher-driver → blitz-test-harness`, `→ dioxus-native-dom` 'no feature named'; the binary installs the sink. The bullet names `seven_guis_native` as the one binary carrying the events."
    basis: ".andromeda/obs-plan.md:30 · examples/seven_guis/Cargo.toml:29-35 · packages/escher-driver/Cargo.toml:13-15"
    dependent-of: D-obs-stack

  - detector: D-obs-stack
    severity: warning
    section: "§6 Log Coverage → Logged events (current truth) → **examples** — the 'Example output uses `println!`/`eprintln!`' bullet, obs-plan.md:170"
    change: "Exception clause names both binaries ('except `seven_guis_native` and `escher-session`, which log through escher-telemetry (below)'), and add `escher-session`'s own stderr lines: a usage line on any other argv (exit 2), the `SessionError` `Display` on a failed serve (exit 1) and 'telemetry not installed: {error}' — fixed strings holding no path, label, id or value (the `Io` arm appends the kind's `Debug` name); nothing on stdout on any path (examples/seven_guis/src/session_host.rs:20-22, :30, :50)."
    sidecar: "§6 examples: escher-session logs through escher-telemetry; its three eprintln! lines (usage, error Display, init failure) recorded as fixed strings, stdout empty."
    rationale: "Report Symbols / APIs: exit codes 0/1/2 with stderr lines, 'It writes nothing to stdout on any path'; `SessionError` 'every message a fixed string … none interpolating a path, label, id or value'."
    basis: ".andromeda/obs-plan.md:170 · examples/seven_guis/src/session_host.rs:21 · :30 · :50"
    dependent-of: D-obs-stack

  - detector: D-obs-stack
    severity: warning
    section: "§6 Log Coverage → Logged events (current truth) — the '**escher-telemetry (in `seven_guis_native`)**' group heading, obs-plan.md:171"
    change: "Heading becomes '**escher-telemetry (in `seven_guis_native` and `escher-session`)**'; the two events under it (info `telemetry installed`, ERROR `panic`) are unchanged — at `RUST_LOG=info` the session host's whole stderr is that one `telemetry installed` line."
    sidecar: "§6 escher-telemetry event group scoped to both installing binaries; events unchanged."
    rationale: "Report evidence paragraph: `info` 1 line on the `escher-session` binary; second install site of `init`. `init` emits the info event and installs the panic hook in one call."
    basis: ".andromeda/obs-plan.md:171 · packages/escher-telemetry/src/lib.rs:131-132"
    dependent-of: D-obs-stack

  - detector: D-obs-stack
    severity: warning
    section: "§7 Error Capture & Reporting → Panic hooks — the `seven_guis_native` hook bullet, obs-plan.md:241"
    change: "Scope the chaining hook to both installers: 'escher-telemetry's hook, installed by `init` in `seven_guis_native` and in `escher-session`, chains …' — so in the session host too std's default hook still prints the raw panic message to stderr after the redacted ERROR event."
    sidecar: "§7 panic-hook bullet covers both binaries that call init; the raw std panic message reaches the session host's stderr as it does the windowed binary's."
    rationale: "Report: `escher_telemetry::init` gains its second install site (session_host.rs:20); `init` installs the panic hook, so the bullet's single named actor is stale. The raw-message consequence is the first item of §8's 'Past escher's scrub' list, now true of a process that holds a live screen."
    basis: ".andromeda/obs-plan.md:241 · packages/escher-telemetry/src/lib.rs:131 (`panic::install()` inside init)"
    dependent-of: D-obs-stack

  - detector: D-obs-stack
    severity: warning
    section: "§9 CI Integration → Telemetry artifact handling (current truth) — new table row after 'Agent-run harness state', obs-plan.md:322"
    change: "Add row 'Session state directory': the state directory the caller names — in the checks `{CARGO_TARGET_TMPDIR}/ss-life`, `/ss-quiet` (blitz-tests) and `/hb-serve`, `/hb-refuse` (seven_guis), i.e. under `target/tmp/` — created `0700` by the host, holding `session.sock` (`0600`) alone while a session is up and nothing after `stop` (socket removed, then the directory, non-recursively); no log, event or screen content is written to it; local only, uploaded by no CI leg (uploads read `target/ci-logs/` and `target/coverage/` alone); it belongs to the session library and its checks, not to `agent-run.sh`, which still starts no daemon and binds nothing. Source: packages/escher-driver/src/host.rs:20; tests/blitz-tests/tests/session_common/mod.rs:71; examples/seven_guis/tests/host_binary.rs:15."
    sidecar: "§9 gains the session state directory row: socket file only, removed at stop, local, uploaded by no CI leg, not part of the agent-run contract."
    rationale: "Report Expected amendments: 'obs-plan §9 — the session state directory as harness state, uploaded by no CI leg — carried: Harness / gate surface. Site: obs-plan.md:322'; Harness / gate surface lists the four directories 'each removed at `stop`' and 'No CI workflow changed'; Coverage: state directory 'holds the socket file only — the listing asserted in `stand_session_quiet`'."
    basis: ".andromeda/obs-plan.md:322 · tests/blitz-tests/tests/session_common/mod.rs:71 · examples/seven_guis/tests/host_binary.rs:15"

  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage — new census line"
    change: "Add: 'observed absent — spans, events or a `tracing` dependency in `packages/escher-driver` (the session library: `Session::start`, `serve`, `attach`, `stop`, `start` and the lifecycle wire) — silent by its chunk's constraint, its two dependencies being `blitz-test-harness` and `dioxus-native-dom`; one span per driver command is owed by the route entry \"Driver command spans\" (as measured at escher-0.1.0/chunks/2026-10-07-driver-session/report.md)'."
    sidecar: "§4 census extended: escher-driver carries no span, event or tracing dependency by design; per-command spans owed by 'Driver command spans'."
    rationale: "Report Coverage of new surfaces: the session socket 'instrumentation n/a (the library is silent by the plan's constraint — no `tracing` dependency, span or event; per-command spans are the \"Driver command spans\" entry's)'. §4–§6 state no requirement the new operations breach, so this is a census extension rather than a violated requirement — offered so the deliberate absence is on record beside the other 'observed absent' lines; safe to decline."
    basis: "packages/escher-driver/Cargo.toml:13-15 · escher-0.1.0/working-route.md:64"

  - detector: D-obs-stack
    severity: warning
    section: "§1 Obs Scope Summary → Instrumentation scope — first **Workspace** bullet, obs-plan.md:7 (root Cargo.toml citations)"
    change: "Re-point four citations, claim text unchanged: `Cargo.toml:172` → `Cargo.toml:174` (tracing-wasm); `Cargo.toml:175` → `Cargo.toml:177` (console_error_panic_hook); `Cargo.toml:186-188` → `Cargo.toml:188-190` (tracing, tracing-subscriber, tracing-log); `Cargo.toml:289` → `Cargo.toml:291` (env_logger)."
    sidecar: "Root Cargo.toml citations re-pointed +2 at obs-plan §1 Workspace bullet (4 sites); no claim change."
    rationale: "Report Counts / qualifiers moved: 'every line at old :18 or below moved +1, every line at old :61 or below moved +2 … obs-plan 9 … each to be re-pointed by measurement where its cited line moved'. The telemetry-stack dependency lines are the §3 stack's baseline; the cited numbers now land on wasm-bindgen, web-sys, bytes/slotmap/tracing and peniko."
    basis: "Cargo.toml:174 · :177 · :188-190 · :291 (measured on the working tree)"

  - detector: D-obs-stack
    severity: warning
    section: "§1 Obs Scope Summary → Instrumentation scope — second **Workspace** bullet, obs-plan.md:8 (root Cargo.toml citations)"
    change: "Re-point one citation, claim text unchanged: `Cargo.toml:56` → `Cargo.toml:57` (the `debug_timer` workspace-dependency line); `Cargo.toml:4` (the member line) stays."
    sidecar: "Root Cargo.toml citation re-pointed +1 at obs-plan §1 debug_timer bullet (1 site, 1 unmoved)."
    rationale: "Same shift rule (report Counts): old :56 lies between the two inserts, so +1; :56 now reads the `dioxus-native-dom` entry."
    basis: "Cargo.toml:4 · Cargo.toml:57"
    dependent-of: D-obs-stack

  - detector: D-obs-stack
    severity: warning
    section: "§5 Metric Coverage → In-process timing and counting — the 'Frame / phase timing' row, obs-plan.md:113 (root Cargo.toml citation)"
    change: "Re-point one citation, claim text unchanged: `Cargo.toml:256-259` → `Cargo.toml:258-261` (`[features]`, `log-times`, `log-frame-times`, `log-phase-times`)."
    sidecar: "Root Cargo.toml citation re-pointed +2 at obs-plan §5 frame/phase timing row (1 site)."
    rationale: "Same shift rule (report Counts): old :256-259 is below both inserts, so +2; :256-257 now read `publish = false` and a blank line."
    basis: "Cargo.toml:258-261"
    dependent-of: D-obs-stack

  - detector: D-obs-stack
    severity: warning
    section: "§7 Error Capture & Reporting → Panic hooks — first bullet, obs-plan.md:238 (root Cargo.toml citation)"
    change: "Re-point one citation, claim text unchanged: `Cargo.toml:175` → `Cargo.toml:177` (console_error_panic_hook)."
    sidecar: "Root Cargo.toml citation re-pointed +2 at obs-plan §7 console_error_panic_hook bullet (1 site)."
    rationale: "Same shift rule (report Counts); :175 now reads `web-sys`. With lines 7, 8 and 113 this accounts for all 9 obs-plan sites the report counts: 7 moved, 2 unmoved (:4, :17)."
    basis: "Cargo.toml:177"
    dependent-of: D-obs-stack
```

## a11y-plan

verdict: 0 proposal(s) · parsed · entities 0 found, 0 after decode · 46 comment line(s) around the YAML (the agent's notes, kept below, not proposals)

```yaml
proposals: []

# Both a11y-plan detectors hold against the report's Changes section.
#
# D-a11y-surface — no drift. The chunk adds no interactive UI element: all five
#   "Coverage of new surfaces" lines (report.md:71-75) read `a11y n/a`, except the held
#   instance, which reads "a11y ✓ unchanged surface, read not altered ... no markup, role,
#   name or focusability changed" and `tokens n/a (no UI rendered)`. The new surfaces are a
#   socket, an argv, a label, a state directory and an in-process session. The stand's
#   sources are held unchanged by the preservation gate (report.md:13). Nothing for §5-§7
#   to cover.
#
# D-a11y-obs-schema — no drift. Neither schema changed: "Scrub and redaction shapes are
#   unchanged — packages/escher-telemetry is untouched" (report.md:26), and the a11y
#   violation schema is still undefined (a11y-plan.md:87). The restatement of the obs log
#   format at a11y-plan.md:87 (engine-target field allowlist; `text`/`value`/`html`/`attrs`
#   redacted at any target) is the kind of statement the report calls "accurate as written"
#   (report.md:49). The disproved claims 1-3 concern third-party targets outside the
#   allowlist, which a11y-plan.md:87 makes no claim about.
#
# The one keyed contract (bootstrap-phases-derive-for-route-setup-project) is touched by
# neither a detector nor a Change; read, nothing to amend.
#
# Outside these two detectors — not proposed, because neither invariant covers them, but
# the report assigns them to a11y-plan and they still need an owner in this wrap:
#
#   1. Root Cargo.toml citation re-point (report.md:32, :57; a11y-plan's 4 sites).
#      Measured against the current root Cargo.toml:
#        a11y-plan.md:6   `Cargo.toml:3`      -> unmoved (accesskit_xplat member line is :3)
#        a11y-plan.md:6   `Cargo.toml:57`     -> now :58 (accesskit_xplat workspace dep)
#        a11y-plan.md:6   `Cargo.toml:141`    -> now :143 (`accesskit = "0.25"`)
#        a11y-plan.md:14  `Cargo.toml:54-55`  -> now :55-56 (dioxus-native / dioxus-native-dom,
#                                                 `default-features = false`)
#      The stand-manifest cites on a11y-plan.md:14 (`examples/seven_guis/Cargo.toml:22`, `:31`)
#      are unmoved; lines 1-34 are byte-identical (report.md:33).
#
#   2. Expected amendment "a11y-plan §1 · §2" (report.md:66), site a11y-plan.md:14: the
#      session library names no accessibility feature, the stand binary's build is unchanged,
#      and the feature decision is owed by the entry that first reads the snapshot through
#      the session. The existing claim there ("names none and builds no platform adapter")
#      stays true. This is an addition, not a retirement.
#
#   3. PROVISIONAL discharge (report.md:91, founder's ruling 3 «Утверждаю оба»): 3 hits on
#      a11y-plan.md:36 (two: "The refresh on change is PROVISIONAL ..." and "recorded as
#      provisional ...") and a11y-plan.md:37 ("— PROVISIONAL, the same item —"). The windowed
#      witness is still owed, so the "No windowed run witnesses the refresh ..." sentence on
#      :36 stays. Item 1 (the 27 boolean attributes, a11y-plan.md:179) carries no PROVISIONAL
#      text in the body; its provisional status lives only in the sidecar.
```

## Index (ids used by §Validate)

- A1 · D-arch-resources · warning · primary · §Occupied Resources → Network ports and listeners
- A2 · D-arch-resources · warning · dep · §Inherited Defaults → API style
- A3 · D-arch-resources · warning · dep · §Occupied Resources → Process-wide state and threads (the re-exec clause)
- A4 · D-arch-resources · warning · primary · §Occupied Resources → Process-wide state and threads (the telemetry-install clause)
- A5 · D-arch-resources · warning · dep · §Cross-cutting Patterns → Logging and timing
- A6 · D-arch-resources · warning · primary · §Occupied Resources → Filesystem
- A7 · D-arch-resources · warning · primary · §Occupied Resources → Names
- A8 · D-arch-resources · warning · primary · §Standard Contracts → Driver session (escher-driver) [new bullet, after Telemetry bootstrap (escher-telemetry)]
- A9 · D-arch-resources · warning · primary · §Standard Contracts → CLIs
- A10 · D-arch-resources · warning · primary · §Inherited Defaults → Publishability
- A11 · D-arch-resources · warning · primary · §Infrastructure Patterns → Directory structure
- A12 · D-arch-resources · warning · primary · §Existing Scopes (new row: escher-driver)
- A13 · D-arch-resources · warning · dep · §Existing Scopes → seven_guis
- A14 · D-arch-resources · warning · dep · §Existing Scopes → blitz-tests
- A15 · D-arch-resources · warning · dep · §Conventions → Tests
- A16 · D-arch-resources · warning · primary · §Stack and Technologies → Testing
- A17 · D-arch-decisions · warning · primary · §Established Decisions → [Driver session] (new keyed entry)
- S1 · D-security-input · escalate · primary · §Input Validation (new table row: Driver session socket — lifecycle wire)
- S2 · D-security-input · escalate · dep · §Input Validation (new table row: Driver session state directory)
- S3 · D-security-input · escalate · dep · §Input Validation (new table row: Driver session label)
- S4 · D-security-input · escalate · dep · §Input Validation (new table row: CLI arguments (escher-session))
- S5 · D-security-input · escalate · dep · §API Security (lead sentence, security-plan.md:185)
- S6 · D-security-input · escalate · dep · §API Security (controls table, new row: Local session socket)
- S7 · D-security-input · escalate · dep · §Threat Model Summary → Attack surface (new vector: local IPC — driver session socket)
- S8 · D-security-input · escalate · primary · §Input Validation → Markup attributes · `aria-label` · `<label for>` (accessible names) row (security-plan.md:108)
- S9 · D-security-input · escalate · dep · §Input Validation → Markup attributes · `id` (stable element id) row — the clause "the id is computed on demand and writ
- S10 · D-security-input · escalate · dep · §Input Validation → `id` row — first restatement, "so the platform adapter stays the id's only exit" (after the `Snapsho
- S11 · D-security-input · escalate · dep · §Input Validation → `id` row — second restatement, "so the platform adapter still stays the id's only exit" (after the `
- S12 · D-security-input · escalate · dep · §Input Validation → `id` row — third restatement, "so the platform adapter still stays the id's only exit" (after the `S
- S13 · D-security-input · warning · dep · §Logging & Monitoring → Log format and backends — escher's own sink, the "Its reach is that sink only" list (security-pl
- S14 · D-security-input · warning · dep · §Logging & Monitoring → Log format and backends — escher's own sink, the install clause "`seven_guis_native` installs `e
- S15 · D-security-input · warning · dep · §Logging & Monitoring → Log format and backends — Stdout output bullet, "`seven_guis_native` additionally logs through e
- S16 · D-security-input · warning · dep · §Bootstrap phases → logging-redaction-wire (security-plan.md:248)
- S17 · D-security-auth · warning · primary · §Authentication & Authorization (requirements table, new row; the "Auth approach: none observed" lead stays)
- S18 · D-security-auth · warning · dep · §Secret Management → Environment values read (none secret-bearing) (security-plan.md:278-283)
- S19 · D-security-deps · warning · primary · §Dependency Security → Pinning — "Git dependencies are pinned by commit rev (Cargo.toml:104; Cargo.toml:114)" (security-
- S20 · D-security-deps · warning · dep · §Dependency Security → Pinning — "every dev-dependency of blitz-tests (tests/blitz-tests/Cargo.toml:15-38)" (security-pl
- L1 · D-layout-surface · warning · primary · §Surface: cli → Primary screens
- L2 · D-layout-surface · warning · primary · §Surface: desktop-native → Primary screens
- T1 · D-tests-coverage · warning · primary · §5 Integration Test Strategy → Boundaries covered (new bullet: session host quietness)
- T2 · D-tests-coverage · warning · primary · §1 Test Scope Summary → Coverage scope (new bullet: escher-driver)
- T3 · D-tests-coverage · warning · primary · §1 Test Scope Summary → Coverage scope → tests/blitz-tests (test-plan.md:25)
- T4 · D-tests-coverage · warning · dep · §2 Test Strategy → Test directory + naming conventions → Directory pattern (test-plan.md:63)
- T5 · D-tests-coverage · warning · dep · §3 Test Harness Contract → Crate-local test helpers → blitz-tests (stand checks) (test-plan.md:97)
- T6 · D-tests-coverage · warning · primary · §1 Test Scope Summary → Coverage scope → apps (rdme, bump, examples, accesskit_xplat) (test-plan.md:10)
- T7 · D-tests-coverage · warning · dep · §5 Integration Test Strategy → Observed absent (test-plan.md:178)
- T8 · D-tests-coverage · warning · dep · §6 E2E Test Strategy → Observed absent (test-plan.md:201)
- T9 · D-tests-coverage · warning · primary · §4 Unit Test Strategy → What unit tests cover (new bullet: escher-driver)
- T10 · D-tests-coverage · warning · primary · §5 Integration Test Strategy → Boundaries covered (new bullets: held session; session host ↔ lifecycle socket)
- T11 · D-tests-coverage · warning · primary · §2 Test Strategy → Test levels observed (new bullet: process-lifecycle checks)
- T12 · D-tests-coverage · warning · primary · §3 Test Harness Contract → Agent-run contract → Proof (test-plan.md:112)
- T13 · D-tests-coverage · warning · primary · §9 CI Integration → Pipeline facts → Local baseline (`cargo test --workspace` re-count chain, test-plan.md:315)
- T14 · D-tests-coverage · warning · primary · §5 Integration Test Strategy → Boundaries covered → tests/blitz-tests crate (test-plan.md:164)
- T15 · D-tests-coverage · warning · dep · §9 CI Integration → Pipeline facts → Local baseline (test-plan.md:312)
- T16 · D-tests-obs-harness · warning · primary · §3 Test Harness Contract → Agent-run contract → Invocation (test-plan.md:103)
- T17 · D-tests-obs-harness · warning · dep · §3 Test Harness Contract → Agent-run contract → State (test-plan.md:111)
- T18 · D-tests-obs-harness · warning · primary · §3 Test Harness Contract → Stand log format (test-plan.md:99)
- T19 · D-tests-obs-harness · warning · primary · §3 → Session lifecycle (new keyed contract: a row in .andromeda/registries/test-plan-contracts.toml + a file under regis
- O1 · D-obs-pii · escalate · primary · §8 PII Scrubbing & Compliance → Values logged as-is (the 'Past escher's scrub' bullet, obs-plan.md:288)
- O2 · D-obs-pii · escalate · dep · §8 PII Scrubbing & Compliance → Scrubbing (the 'Its reach is that sink' bullet, obs-plan.md:293)
- O3 · D-obs-pii · escalate · dep · §3 → Bootstrap phases (derive for route / setup-project)
- O4 · D-obs-stack · warning · primary · §3 Observability Harness Contract → Logging stack (subscriber installation) — the `seven_guis_native` install bullet, ob
- O5 · D-obs-stack · warning · dep · §3 Observability Harness Contract → Logging stack — the headless-stand bullet, obs-plan.md:68
- O6 · D-obs-stack · warning · dep · §3 Observability Harness Contract → Service identity and line format (escher's sink), obs-plan.md:81
- O7 · D-obs-stack · warning · dep · §1 Obs Scope Summary → Instrumentation scope — the **examples** bullet, obs-plan.md:10
- O8 · D-obs-stack · warning · dep · §2 Telemetry Strategy → Telemetry mechanism (current truth) — first bullet, obs-plan.md:30
- O9 · D-obs-stack · warning · dep · §6 Log Coverage → Logged events (current truth) → **examples** — the 'Example output uses `println!`/`eprintln!`' bullet
- O10 · D-obs-stack · warning · dep · §6 Log Coverage → Logged events (current truth) — the '**escher-telemetry (in `seven_guis_native`)**' group heading, obs
- O11 · D-obs-stack · warning · dep · §7 Error Capture & Reporting → Panic hooks — the `seven_guis_native` hook bullet, obs-plan.md:241
- O12 · D-obs-stack · warning · primary · §9 CI Integration → Telemetry artifact handling (current truth) — new table row after 'Agent-run harness state', obs-pla
- O13 · D-obs-instrumentation · warning · primary · §4 Span / Trace Coverage — new census line
- O14 · D-obs-stack · warning · primary · §1 Obs Scope Summary → Instrumentation scope — first **Workspace** bullet, obs-plan.md:7 (root Cargo.toml citations)
- O15 · D-obs-stack · warning · dep · §1 Obs Scope Summary → Instrumentation scope — second **Workspace** bullet, obs-plan.md:8 (root Cargo.toml citations)
- O16 · D-obs-stack · warning · dep · §5 Metric Coverage → In-process timing and counting — the 'Frame / phase timing' row, obs-plan.md:113 (root Cargo.toml c
- O17 · D-obs-stack · warning · dep · §7 Error Capture & Reporting → Panic hooks — first bullet, obs-plan.md:238 (root Cargo.toml citation)

## Validate — dispositions (orchestrator, by the ids of the index above)

**One rule applied to the whole set first.** Nearly every proposal's `change` or `basis` cites a source or manifest
line the report does not carry (the agents read the tree: `host.rs:81`, `session_host.rs:30`, `Cargo.toml:174`, the
`mod common;` reader count). By amendment-flow §Validate that is the re-derivation tell: the proposal as written is
not applied. Each proposal's FACT was then checked against the report — all 75 rest on a Changes bullet, a disproved
claim, an expected amendment or a Decisions-and-corrections entry — and re-entered through the orchestrator (check 5,
check 6, or the operator's recorded direction), its text re-derived from the report and every cited coordinate
re-measured by the orchestrator before it was written. "applied" below means that.

| ids | check that decided | disposition |
|---|---|---|
| A1 · A2 · S1-S7 | 1 — playbook "Boundary widening" (never routine) | **escalated and resolved**: the session socket is the founder's own choice among three options, relayed verbatim (`inputs#I1`) — the form a boundary widening is ratified by; no halt was taken on a question already answered in his words. Applied; the ratification is in the architecture and security-plan sidecar entries. |
| S8-S12 · O1-O3 | detector severity `escalate` (D-security-input · D-obs-pii) · check 6 | **escalated and resolved**: the founder's ruling given at this wrap, «Честно записать + чанк следом» (`directives.md` item 5) — the wording scoped to the measurement, the remedy a route entry next. Applied. S10-S12 (the three "the id's only exit" clauses) applied in one wording: "the only exit escher's own code gives the id". |
| A3-A16 | 1 — "Accurate this-chunk addition" · 5 | applied. A14's and T4's reader counts re-derived: `mod common;` in 11 `stand_*.rs` files, `mod session_common;` in 4. A16's manifest ranges re-measured (`:14-25; :27-29; :31-40`, not the proposal's). |
| A17 | 1 — no rule; the operator's recorded direction settles it (`inputs#I1`, `inputs#I2`) | applied as a keyed entry of §Established Decisions (a body registry, not a Decisions Log). |
| S13-S18 | 1 — "Accurate this-chunk addition" · 6 (S13, S16) | applied. |
| S19 · O14-O17 · T14 · T15 | 5 — the plan's re-point entry | applied by the measured re-point of all 82 root-`Cargo.toml` sites (`cargo-repoint.txt`), not site by site. |
| S20 | 5 | applied, re-measured: the blitz-tests dev-dependency block is `:15-40`. |
| L1 · L2 | 5 — the plan's two layout entries | applied. |
| T1 | 6 — disproved claim 1 · the founder's ruling | applied as a §5 boundary bullet naming what `stand_session_quiet` proves and does not. |
| T2-T13 · T16-T18 | 1 — "Accurate this-chunk addition" · 5 | applied. T16/T17 (test-plan §3) and O5/O12 (obs-plan §3, §9) land together, so the §3 ↔ §3 bind holds on both sides. |
| T19 | 5 — the plan names the key | applied: new key file `registries/contracts/test-plan/session-lifecycle.md` and its `[[row]]`; `registry.py check --all` 0 defects. Its labels are `session-*` so none collides with a bold label of the §3 preamble. |
| O4-O12 | 1 — "Accurate this-chunk addition" · 5 | applied. |
| O13 | 1 — "Sequencing deferral": the owner is a live markerless entry, "Driver command spans" (working-route.md) | applied as a census line. |
| design-system · a11y-plan | — | `proposals: []`, both confirmed against the report. |

**Raised by the orchestrator (no proposal carried them):**
- check 5 — a11y-plan §1 (the session library names no accessibility feature; the decision is owed by the entry that first reads the snapshot through a session), with a11y-plan's four root-`Cargo.toml` sites and the stale blitz-tests `accesskit` line. The a11y agent named all three outside its detectors.
- the operator's directive, items 1-3 (the founder's ratifications, relayed verbatim): 4 + 2 clauses in architecture, 8 in security-plan, 2 in a11y-plan, and 7 leaf lines; the falsy clear carries no body text and is ratified in the architecture and a11y-plan sidecar entries.
- the operator's directive, item 4: the playbook rule "Provisional discharge", appended.

**Checks 2-4.** 2 — no two proposals edit one section in opposing directions (S13 · O1 · T1 · T18 state one measurement, compared field by field). 3 — the report's deviations are each justified; the scope record's one line (`session_common/mod.rs`, in-intent, serves step 12) holds: it adds no behaviour. 4 — every absence and caught-all claim of the pass rests on the sweep listing dispositioned in `cascade-dispositions.md`.

**Escalations open at exit: 0.** Three were raised — the socket as a boundary widening; raw ids and names at `debug` / `trace` (two escalate-severity detectors); the plan's quiet criterion, unmet as a statement about a sink-installing host — and each stands resolved on a word recorded before or at this wrap. Not applied: nothing was rejected outright.
