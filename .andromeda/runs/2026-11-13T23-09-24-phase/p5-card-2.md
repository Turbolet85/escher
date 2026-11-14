# P5 review, card 2 — 2026-11-13-local-files-for-local-documents · «Local files for local documents»

This card folds the operator's review (`.andromeda/runs/2026-11-13T23-09-24-phase/relay-1.md`, verbatim). The plan is
at rev 4 and the research at rev 2. The working copy is `.andromeda/cache/docs/2026-11-13-local-files-for-local-documents/plan.md`:
writing there changes no record until a `rec.py put`, and no plan lives in the chunk's folder.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-13-local-files-for-local-documents`
diffs the chunk's records. Every mechanical check ran again after the edits and passes.

## What changed since card 1
- **The construction guard (your review).** The choice is to make the constructor unreachable, not to keep a census.
  - **The mechanism.** A new `packages/blitz-dom/clippy.toml` disallows `blitz_traits::net::Request::get`. The one
    call, in `stamped_request`, stands under `#[expect(clippy::disallowed_methods, reason = "…")]` with its written
    justification. `expect`, not `allow`, so the attribute itself fails if the call ever leaves.
  - **Why it closes the gap.** `Request` is `#[non_exhaustive]`, so `Request::get` is blitz-dom's only way to build
    one. A future fetch site that skips the stamp fails the workspace clippy leg (`-D warnings`), which runs on every
    push through `fast` and in CI.
  - **Why not the census.** A census in the plan would run only through this chunk's gates, and a text count misses a
    call through an alias.
  - **Measured now, clippy 1.99.0, scratch clone at `8a166d6aa`.** With the config and no `expect`,
    `cargo clippy -p blitz-dom --locked --all-targets -- -D clippy::disallowed-methods` exits 101. It reports exactly
    one finding, `use of a disallowed method blitz_traits::net::Request::get --> packages/blitz-dom/src/net.rs:32:19`.
    So the crate-level config is read, and today's one site is the one it names (`p5-controls.md`).
  - **The gate.** It is a new entry 9 (`role = 'lint'`), baselined green but vacuous: no config on the untouched tree.
    Its red control, the `expect` removed, is implement's `gate.py control` (step 7).
  - **The rest.** A new acceptance criterion, a rejected-approach line for the census, and an Expected amendment:
    an architecture conventions row, plus a new upstream merge surface (upstream's blitz-dom has no `clippy.toml`).
- **F1 answered PROVISIONAL by the operator: A, the route entry, for the founder to confirm or replace.** It is
  recorded in `founder-forks.md` and the plan's Implementation notes.

## Goal (unchanged)
blitz-net's `file:` branch serves a `file:` URL in two cases only: to a request whose loading document is `file:`,
or to an embedder's own top-level load. A remote page reads no local file through the engine's sub-resource fetches.
This is the founder's word of 2026-11-12, «Как в коде + работа» (`inputs#I1`), and narrows what every document can do.

## Top acceptance criteria
- The witness, both layout modes, through blitz-net's real `Provider`: a remote page's local stylesheet, `@import`,
  image and iframe read nothing, and a `file:` page reads all four.
- All 7 engine fetch paths carry the document's URL as `initiator`.
- **New:** blitz-dom builds a `Request` only through `stamped_request`, enforced by the clippy guard. Entry 9 reads
  green, and red with the `expect` removed.
- blitz-net serves the no-initiator and `file:`-initiator cases and refuses `https:`, offline.
- `document_may_load` has one body, in blitz-traits, and its vibey-script path still resolves.
- The census shows no new log site, print, env read or panic path, and the witness is silent and has no wall clock.
- `fast` and `audit` pass locally, and the operator's CI read shows `verdict: green`.

## Touchpoints
- **New:**
  - `packages/blitz-dom/clippy.toml`
  - `tests/blitz-tests/tests/local_files_for_local_documents.rs`
- **Modified:**
  - `packages/blitz-traits/src/net.rs`
  - `packages/blitz-vibey-script/src/fetch.rs`
  - `packages/blitz-net/src/lib.rs`
  - `packages/blitz-dom/src/{net,document,iframe,mutator,cssom}.rs`
  - `packages/blitz-dom/src/layout/damage.rs`
  - `tests/blitz-tests/Cargo.toml`
  - `Cargo.lock`

## Test commands (14 entries)
| Entries | What they run | Baseline on the untouched tree |
|---|---|---|
| 1–3 | unit: blitz-traits, blitz-net, blitz-vibey-script | green |
| 4 | the witness | red: no such target yet |
| 5 | 5 load-path checks under the workspace build | green, 13 passed |
| 6 | the census | green, controlled red |
| 7 | the witness's silence | red: file absent; must-pass and control in a clone |
| 8 | shared modules and harness untouched | green, controlled red |
| 9 | the construction guard | green but vacuous; its known-positive is red at `net.rs:32` |
| 10 | `fast` | seen green |
| 11 | `audit` | green |
| 12–14 | operator: hygiene, push, `ci.py` | — |

## Capabilities
None claimed (pool v030-08 … v030-15 untouched), so there is no concretization preview.

## Beyond the plan
| # | Proposal | Kind | Owner proposed |
|---|---|---|---|
| 1 | The browser's favicon probe (`apps/browser/src/favicon.rs:49`) still reads a remote page's `file:` icon on its behalf, and navigation from a remote page to a `file:` URL still loads it as a top-level document. | route insertion — the founder's fork F1, answered PROVISIONAL by the operator: A | A new working-route entry «Remote pages and local files in the browser» at the end of Epoch 4, after «Auto-exploration»: the favicon probe stamped with its page, and `file:` navigation from a non-`file:` document refused. PROVISIONAL until the founder confirms or replaces it (B: a stated limit; C: fold here). |

Apply? (yes / review / cancel)
