# arch extract

## Relevance
relevant — the take touches the pinned dependency set, the engine crates' public contracts and every merge surface architecture records as owned by this sync; no new crate or surface is to land.

## Constraints
- The take's pin set is bound by the manifest rule that no git dependency stands, the parser, skrifa and svgtypes match lines and winit's exact beta pin; a shape that breaks any of them (the whole merge's git Stylo and git Taffy) reverses a recorded decision, so it stands only by the founder's fork. Which shape holds it is P3's measurement per shape (per architecture §Established Decisions → Dependency pinning).
- The framework versions the plan records (Stylo, Taffy, Parley, anyrender, Dioxus, winit, rust-version 1.91.0) stay as stated unless the chosen take moves them as upstream's one set, and a moved one is an amendment owed at wrap (per architecture §Inherited Defaults → Framework).
- The plan's upstream-sync line names the merge base, the last-read sha and the left-out delta, and says git Stylo 0.23-pre, the six Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version; whether this chunk counts as that version is the founder's fork, not a reading, and a take that moves the pin owes this line's rewrite at wrap (per architecture §Project Intent → Upstream sync).
- The engine-debts changes, the document clock, the select model and the select list are direct engine changes whose merge surfaces this sync owns, `hit_inner` a named textual conflict with `0980cce5f`; each is re-read row by row and nothing is sent upstream (per architecture §Established Decisions → Scrolling and input).
- Hit stops, the open-list-first hit order, `absolute_position`, `scroll_into_view` and `visible_region` keep their stated contracts through any conflict resolution in `node/node.rs`, `scrolling.rs` and `layout/writing_mode.rs`; whether upstream's incoming hunks disturb them is research's question (per architecture §Standard Contracts → Scrolling, selection, tree).
- The changed set's five kinds of write site and its no-write list stay exact, the shell's refresh keyed to a non-empty set; `fe33b778b`'s narrower invalidation and any `mutator.rs` resolution are read against this rule (per architecture §Cross-cutting Patterns → Invalidation and state integrity).
- The sink's prefix-set drop and scrub stand as stated, and no port, listener, env var or crate name is added; whether the taken delta brings any `tracing` or `log` site, target, print or env read is research's question (per architecture §Cross-cutting Patterns → Logging and timing; per architecture §Occupied Resources → Environment variables).

## Patterns to follow
- Code stays in the crates it already occupies, with no `members` change: the delta lands across `packages/blitz-dom`, `blitz-paint`, `blitz-vibey-script`, `dioxus-native-dom` and `tests/blitz-tests` (per architecture §Infrastructure Patterns → Directory structure).
- Any dependency entering or moving is declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`, in-repo crates with `default-features = false` but for the `seven_guis` path entry (per architecture §Conventions → Manifests).
- Upstream's new tests land under the fork's rule of one file and one target per behaviour, each opening with a `//!` doc, scenarios in both layout modes, `tests/all.rs` declared and held at `test = false`; shared fixtures stay in `tests/blitz-tests/tests/fixtures/mod.rs` (per architecture §Conventions → Tests).
- Every gate runs through `.github/scripts/ci-leg.sh {leg}`, the one runner of CI and the dev host, so a `font-embolden` local gate and the free-disk `if:` fix are read against that runner and the workflow's stated shape; `ci.yml` keeps its SHA-pinned `uses:` and its install script (per architecture §Standard Contracts → CI contracts; per architecture §Infrastructure Patterns → CI/CD).
- A conflict in `README.md` resolves to escher's version, and `docs/capabilities.md` stands beside upstream's `docs/` files; the orphan «Re-export taffy» comment in `packages/blitz-dom/src/lib.rs` is settled under the plan's comment discipline (per architecture §Project Intent → Front page; per architecture §Conventions → Documentation).

## Anti-patterns to avoid
- A one-sided pin fix or a git dependency in the root manifest outside the founder's fork (per architecture §Established Decisions → Dependency pinning).
- An engine time read outside `clock.rs`, in blitz-dom, brought in with an upstream hunk (per architecture §Standard Contracts → Document core).
- Resolving the `node.rs:1775` panic by a read that survives on layout-generated or stale ids: persistent interaction state references DOM nodes only and an id is read through a path that does not alias or panic; the fix itself is the founder's fork and is not chosen here (per architecture §Design Philosophy → One incremental pipeline, safe identities).

## Contract bindings
- tests: the stand checks, the incremental/non-incremental oracle and moved numbers bind the harness's `settle`, `screenshot` and `text_contrast` (per architecture §Standard Contracts → Test harness), and blitz-tests' target rule (per architecture §Conventions → Tests) to the test plan's per-OS tallies.
- security: `91ef65e21`'s script API sits on the `document_may_load` gate and the JS bootstrap (per architecture §Standard Contracts → JavaScript); the audit leg reads any entering crate (per architecture §Inherited Defaults → Code quality).
- obs: the sink allowlist re-check binds architecture §Cross-cutting Patterns → Logging and timing to `packages/escher-telemetry/src/format.rs` and the standing capture checks (per architecture §Standard Contracts → Telemetry bootstrap).
- a11y: the builder's state writes and naming, a select's native `expanded` included, bind the a11y leg's rows (per architecture §Standard Contracts → Mutation, query and CSSOM).
- design: `6fca6308d`, `ac6f9f582` and `e5929f9e2` meet the paint order that puts an open select's list last, past clips and `opacity`, and the masked password paint (per architecture §Standard Contracts → Paint and shell).
- graph entries: `6ef4b532d`'s mounted-events hunk and any `286874cc1` meet the stable-id tiers and read-only snapshot that later entries read (per architecture §Standard Contracts → Dioxus DOM bridge).

## Acceptance criteria contributions
- The root manifest and lock, read at the take, hold exactly the pin set of the founder's chosen shape: under a registry-pinned shape no git source, and under any shape no pin outside upstream's one set at the measured sha (per architecture §Established Decisions → Dependency pinning).
- The workspace `members` list and the plan's occupied names, ports, listeners and env vars read unchanged after the take; any new engine `tracing`, `log`, print or env-read site is listed and the standing capture checks read 0 needles (per architecture §Occupied Resources → Names; per architecture §Cross-cutting Patterns → Logging and timing).
- Each row of the three merge-surface records is re-read against the take and given a named outcome, with `hit_inner` resolved keeping the non-invertible answer (per architecture §Established Decisions → Scrolling and input).
- Where the take moves the pin, the plan's upstream-sync line reads the new pin and commit at wrap, and its left-out delta matches the take's own list (per architecture §Project Intent → Upstream sync).
