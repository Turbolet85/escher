# Scope — 2026-10-06-project-readme

**Working entry (verbatim):** Project README — the repository front page describes escher, not Blitz: what it is, why it exists, its Blitz lineage, the plans, a contact  CONTEXT: operator-requested adaptation (operator, 2026-10-06, at the 2026-10-06-id-persistence wrap) — the root `README.md` rewritten for escher, short and plain: what escher is (an agent-first UI framework built on Blitz); why it exists (an agent with no prior context learns the framework from the tool alone, and drives and checks a UI headlessly — per intent.md); how it relates to Blitz (a fork of DioxusLabs/blitz kept in reach by an "Upstream sync" at each epoch boundary; the upstream's credit and license kept); the plans (the 0.1.0 epochs from the working route, one line each, and what lies beyond 0.1.0 — intent's "Out of this version"); a closing Author / Contact section with the address turbolet85@gmail.com. No other file changes; facts cite the intent and the masters, never invent capability

**Epoch:** Epoch 2 — Element identity · **Version:** escher-0.1.0 · **Freight on the entry:** one `CONTEXT:` block (`route.py pins` row 32, 809 chars — the only block on this entry; the tail's other six sit on lines 34, 39, 41, 60, 84 and 86)

## What this chunk builds
The root `README.md` is today upstream Blitz's front page (115 lines: the Blitz logo, upstream CI/crates.io/docs.rs badges, the Dioxus Discord, Blitz's beta status, its examples, goals, crate architecture, the git-Dioxus-Native recipe, the license). This chunk rewrites it as escher's front page, short and plain, in five parts the CONTEXT names:

- **What escher is** — an agent-first UI framework built on Blitz: a UI an agent can see, drive and verify headless, with no display and no outside driver (intent.md opening paragraph).
- **Why it exists** — an agent with no prior context learns the framework from the tool alone, and drives and checks a UI headlessly (intent.md §Principles "The API explains itself"; §Findings 1–9 name the gaps).
- **How it relates to Blitz** — a fork of DioxusLabs/blitz (adopted at `0f60502e`, intent.md), kept in reach by an "Upstream sync" chunk at each epoch boundary (intent.md §Principles "Upstream stays in reach"); the upstream's credit and its license kept.
- **The plans** — the 0.1.0 epochs from the working route, one line each (Epoch 1 Foundation · 2 Element identity · 3 Observation model · 4 Driver core · 5 Agent surfaces · 6 Polish & ship — `escher-0.1.0/working-route.md` headers), and what lies beyond 0.1.0 (intent.md §Out of this version).
- **Author / Contact** — a closing section with the address turbolet85@gmail.com (the operator's word, on the entry).

- **Honesty of capability claims** — "never invent capability" means the README separates what is BUILT from what is PLANNED. Built = what a `complete` master record and a `verified` matrix entry carry (at take-up: 10 complete records; v010-01 stable element ids and v010-02 id persistence verified, 2/15). The snapshot, diff, settle, act-by-id, refusals, CLI, MCP, screenshot and cold-agent test are planned, and the README must say so in those words, not present them as features.
- **Upstream-sync merge surface** `[premise-corrected: FOUR syncs remain, not three — working-route lines 37, 48, 63, 74 (route.py markerless); upstream touched README.md 6 times in the last 12 months and 0 times since the 23354585 merge base (research.md §Patterns detected)]` — a rewritten `README.md` diverges wholesale from upstream's, so an upstream commit touching `README.md` conflicts at the next "Upstream sync" chunk — at some syncs, not every one. The resolution is to keep escher's README. Whether that needs recording beyond this chunk (arch / playbook) is a wrap amendment question, not a phase edit.
- **README consumers** — verified: nothing reads the root README at build, doc or test time. `apps/readme` (`rdme`) resolves `README.md` at runtime only (main.rs:206-219); there is no `include_str!` of a README and no Cargo `readme =` field; the root package `blitz-examples` is `publish = false` (Cargo.toml:245-254); the five `.github/scripts/*.py` files carry 0 README references (research.md §Patterns detected). The `doc` and `ci-scripts` legs cannot move on this rewrite.
- **Upstream links and badges** — upstream's badges (DioxusLabs CI, crates.io `blitz`, docs.rs, deps.rs) and the Discord/blitz.is links describe Blitz, not escher, and no escher crate is published (arch §Inherited Defaults → Publishability). Verified context for the P4 decision: the fork is PUBLIC with default branch `build/escher-0.1.0` (gh repo view), so a fork CI badge would render that branch's real status. Dropped vs repointed is a P4 decision.
- **A "try it" section** — the CONTEXT lists five parts and says "short and plain"; whether a minimal how-to-run belongs is a P4 decision, decided against "short and plain". Verified: `just seven_guis` exists (justfile:64-65) and `bash scripts/agent-run.sh boot` / `run stand` is the registered harness grammar (test-plan §3).

## Boundaries
- In: the root `README.md`, whole.
- Out: every other file — "No other file changes" (CONTEXT). `LICENSE-APACHE`, `LICENSE-MIT`, `CONTRIBUTING.MD`, `HOWTO_WASM.md`, `docs/`, per-package `README.md`s and every crate stay as they are.
- Out: any capability itself. The README describes; it builds nothing.
- License text kept: the dual Apache-2.0 / MIT statement (Cargo.toml:36) and `stylo_taffy`'s additional MPL-2.0 licence (packages/stylo_taffy/Cargo.toml:3), plus credit to the upstream. `[premise-corrected: neither LICENSE file names a copyright holder (research.md §Files inspected), so the credit is an attribution line to DioxusLabs/blitz and its contributors, not a copyright line to preserve]`

## Surfaces and contracts touched
- `README.md` (root) — rewritten.
- Security: the contact address is the operator's own, given on the entry for a public front page; it is not a credential. No secret, token or host path enters the README.
- a11y: a Markdown page — any image keeps alt text; headings form one outline.
- Telemetry: none.

## CI verdict read at Setup (Setup 5a)
- `0ab4513f35f5` (the last wrap's flip, HEAD), CI#37454564262: **verdict not yet available**. It was in progress with 4 of 4 checks; the oldest running check was "Test [default features]" at 90 s when read at 2026-10-06T11:11Z. There is no red or not-green to disposition. The commit before it, `8c1dd035`, the id-persistence pre-CI commit, read green (CI#37449052968, 16/16, per the handoff). It is re-read at P5.
- Re-read at P5 (2026-10-06T11:24Z): `0ab4513f35f5`, CI#37454564262, **verdict: green**, checks 16/16, wall 435 s.
