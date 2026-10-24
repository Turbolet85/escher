# security extract

## Relevance
relevant — the take moves the dependency graph (pins, a new crate, an advisory fix) and the engine's untrusted-content and logging surfaces; no new escher crossing is in scope.

## Constraints
- RUSTSEC-2026-0386 (tokio, unsound) is fixed by a semver-compatible update, never a blanket allow: `advisories` ignores stay per ID, each with a written reason, and `unsound` stays at its default (per security-plan §Dependency Security, Audit tool). Whether `cargo update -p tokio` reaches ≥ 1.53.3 inside the lock as it stands is research's question. The existing eight per-ID ignores must each be re-read on the take's graph and stay justified, or be dropped where their chain moved.
- No git dependency stands in the root manifest, and `Cargo.lock` stays committed with `--locked` builds (per security-plan §Dependency Security, Pinning). A take that brings in upstream's git Taffy (`802e497b…`) or git Stylo (`d6502e00…`) reverses that recorded state, so it needs the founder's ruling before it lands, not a silent pin change.
- An upstream sync either takes upstream's dependency set whole or, by cherry-pick, moves none. Its lock is read name by name against upstream's at the pin, or against the tree before it, with the audit leg green after (per security-plan §Dependency Security, Pinning; detail `.andromeda/security-plan-detail/dependency-security.md`, the sync records under group `values · deps`).
- A crate entering the lock — `mimalloc` 0.1.48 with `9eeebb302`, or any package Stylo 0.23-pre or Dioxus 0.8.0-alpha.3 brings — is reviewed by hand for licence, repository, build script, proc-macro flag and direct dependent, as the 2026-10-10 sync recorded (per security-plan §Dependency Security, Pinning). It must also pass the licence gate over the whole workspace's graph (per security-plan §Dependency Security › Supply chain integrity, Licence compliance).
- escher's sink admits only engine and `escher_*` targets. An engine target prints its seven safe fields alone, and a record from any other target is dropped whole (per security-plan §Logging & Monitoring). Every new `tracing`/`log` site, print or target that upstream's delta brings lands under that rule and adds no user-content log field. Whether the merged tree adds any such site is research's question, beyond the scope's reading of the added `.rs` lines.
- An env read in source is registered in the environment-values list (per security-plan §Secret Management, Environment values read). Whether upstream's `examples/multi_doc.rs` reads (`MULTI_DOC_N` and one named by a parameter) are source env reads to register, or fall under how earlier syncs treated upstream's root examples, is research's question.
- Markup attributes from parsed content, remote pages included, are read by in-process readers with no log site or panic path, through closed parses that carry no attribute text (per security-plan §Input Validation, Markup attributes). A stale node id resolves to `None` and is skipped, never a panic (per security-plan §Error Handling, Graceful degradation). Conflict resolutions in `accessibility.rs`, `mutator.rs`, `events/pointer.rs` and `node/node.rs` (hit testing) must keep both rules.

## Patterns to follow
- The earlier partial-take shape: no-commit cherry-picks of upstream's text, with `Cargo.toml`, `Cargo.lock` and `deny.toml` byte-identical to the tree before them and the audit leg reading `advisories ok` and `licenses ok`, each recorded in the chunk's `evidence/lock.md` (per security-plan §Dependency Security; detail `.andromeda/security-plan-detail/dependency-security.md`, the 2026-10-18 and 2026-10-19 sync records).
- The whole-take shape: upstream's dependency set taken whole, the lock's `name@version` set equal to upstream's at the pin with the two `escher-*` entries set aside, and each entering crate reviewed by hand (per security-plan §Dependency Security; detail, the 2026-10-10, 2026-10-13 and 2026-10-17 sync records).
- External decay: an advisory against a pinned crate, with the lock and `deny.toml` unchanged, is closed by a reachable fix first. An ID ignore with a written reason is used only when no fix is reachable, as the zip and zbus-family records show (per security-plan §Dependency Security, Audit tool).

## Anti-patterns to avoid
- A blanket advisory allow, `unsound`/`unmaintained` set to none, or an ID ignore for an advisory whose semver-compatible fix is reachable (per security-plan §Dependency Security, Audit tool).
- A git dependency left in the root manifest, or `--locked` dropped from a leg, without a recorded ruling (per security-plan §Dependency Security, Pinning).
- A new log field or print that carries URL, attribute, text or other page content under an allowlisted target (per security-plan §Logging & Monitoring).

## Contract bindings
- Dependency audit ↔ tests CI integration: ci.yml's `audit` job runs `bash .github/scripts/ci-leg.sh audit` on every push, so "CI green" for this chunk includes that job (per security-plan §Dependency Security, CI integration).
- Sink allowlist ↔ obs-plan (escher-telemetry's scrub sets and target allowlist) ↔ tests: the capture checks (`stand_act_spans`, `host_log`, `mcp_log`) are the standing readings of the re-check (per security-plan §Logging & Monitoring).
- Markup-attribute and accessibility readers ↔ a11y-plan: the accessibility checks and `inert_subtree` are the readings that would show a moved reader after the merge (per security-plan §Input Validation, Markup attributes).

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh audit` reads `advisories ok` and `licenses ok` over the whole workspace's graph on the take's tree. RUSTSEC-2026-0386 is cleared by a semver-compatible lock update with no ignore added for it, and every remaining ignore is per ID with its written reason (per security-plan §Dependency Security, Audit tool).
- The root manifest names no git dependency, unless a recorded founder ruling says otherwise. The chunk's lock evidence reads the lock's `name@version` set against upstream's at the measured pin, or against the tree before it, and records a hand review for each entering crate (per security-plan §Dependency Security, Pinning).
- On the merged tree, the sink capture checks read 0 typed-sentinel occurrences, 0 id needles and 0 name needles. Upstream's added lines are read for new log sites, prints and targets, and the census is recorded (per security-plan §Logging & Monitoring).
- Every env read in the take's added `.rs` lines is either registered in the environment-values list or recorded with the reason it is outside that list (per security-plan §Secret Management, Environment values read).
