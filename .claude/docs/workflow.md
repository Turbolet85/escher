# Development Workflow

_Extracted from architecture.md and project conventions by `/andromeda-setup-project`. How work actually flows on escher._

## Git workflow
- **Branching:** one long-lived build branch per version — `build/escher-0.1.0` for 0.1.0; chunks commit on it (no per-chunk feature branches).
- **Commit format:** conventional commits (`feat:`, `fix:`, `chore:`, `refactor:`, `docs:`, `test:`, `perf:`, `build:`, `ci:`).
- **Main branch:** `main` (upstream Blitz CI runs on `main` and `v0.*`; escher's `ci.yml` also runs on every `build/**` push); the fork's GitHub default branch is the version's build branch, `build/escher-0.3.0`, whose root `README.md` is the front page GitHub renders.
- **Before a push:** `bash .github/scripts/ci-leg.sh fast` — the local pre-push gate, the same leg commands CI runs.
- **Never force push** to main/master. Use `gh` for GitHub operations.
- **Upstream:** escher is a fork of Blitz; keep upstream conventions (spec citations, one-behaviour test files) so merges stay cheap.

## Andromeda workflow

This project was adopted into the Andromeda pipeline (`/andromeda-adopt` at `0f60502e`); its masters describe the code as built. Upstream DioxusLabs/blitz `main` was last merged at `23b9b26a9` (merge commit `3eda07bc`, 2026-10-17), the next sync's merge base; upstream `main` was last read at `f9cbb2756`, thirteen of its commits taken by no-commit cherry-picks with the merge base unmoved; git Stylo 0.23-pre, the six Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version, so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3".

**Initial planning (done):** adopt → architecture + 6 specialist plans → `/andromeda-route` (escher-0.1.0 working route) → `/andromeda-setup-project` (this ecosystem).

**Per chunk (main development loop):**
1. `/andromeda-new-session` — load context, run the health check, propose the next action
2. `/andromeda-phase` — promote and plan the next working-route chunk
3. `/andromeda-implement` — execute the plan (writes code and tests, runs the gates green; it never commits)
4. `/andromeda-wrap-session` — reconcile, capture learnings, commit, push
5. Repeat; when the version's verification matrix is complete, `/andromeda-route` starts the next version

## Daily session lifecycle
- **Start:** `/andromeda-new-session` — reads the handoff, dashboard, health check, next action.
- **During:** follow the current chunk's plan; run the gates (`bash .github/scripts/ci-leg.sh fmt` / `clippy` — or `fast` for all four fast legs — and targeted `cargo test -p …`).
- **End:** `/andromeda-wrap-session` — commits, updates `.claude/session-handoff.md`, curates learnings into the right tier.

**Don't skip session boundaries** — new-session and wrap-session keep CLAUDE.md and the masters current.

## Code review
- **Locally:** the **code-reviewer** agent at `.claude/agents/code-reviewer.md` — "review this" / "does this make sense?".
- **Architecture review:** check `.andromeda/architecture.md` §Established Decisions and §Occupied Resources for any new resource.

## Release process
- Upstream model: main is a prepatch until 1.0, one minor ahead of the release; blitz packages are versioned together through the one workspace version and the in-repo `=` pins — no in-repo release tool remains since upstream deleted the bump crate (merged at the 2026-10-17 upstream sync); the browser is bundled by `publish-browser.yml` (signed builds only on main / `ci-test`, and only in `DioxusLabs/blitz` — the fork never runs it).
- escher's fork CI runs on the build branch (fast legs fmt · clippy · test · CI scripts, then the slow legs — builds, docs, dependency and licence audit, a11y, coverage with its per-crate floors, the stand checks repeated three rounds at a zero-failure budget (`stand-repeat`), the matrix; cached; failure logs kept 7 days), every action SHA-pinned and the `GITHUB_TOKEN` read-only.

## Troubleshooting workflow
- Tests failing? `.claude/rules/testing.md`; harness behaviour? `.claude/rules/verification-harness.md`.
- Hooks not running? `.claude/settings.json` (needs `jq`, `rustfmt` on PATH).
- CLAUDE.md stale? `/andromeda-new-session` health check, possibly re-run `/andromeda-setup-project`.

## Philosophy
A reference-based CLAUDE.md: a thin index, with depth in `.claude/docs/` (on demand) and `.claude/rules/` (path-scoped).
