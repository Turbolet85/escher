# P5 review — 2026-11-13-local-files-for-local-documents · «Local files for local documents»

The plan opens in an editor at `.andromeda/cache/docs/2026-11-13-local-files-for-local-documents/plan.md`. That file is
only a working copy: no plan lives in the chunk's folder, and writing in it changes no record until a
`rec.py put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-13-local-files-for-local-documents`
diffs the chunk's records.

## Goal
blitz-net's `file:` branch serves a `file:` URL in two cases only: to a request whose loading document is itself
`file:`, or to an embedder's own top-level load. A request from any other document reads nothing from disk and gets
its handler's `error`, so a remote page reads no local file through the engine's sub-resource fetches.

This is the founder's 2026-11-12 word, «Как в коде + работа» (`inputs#I1`). It narrows what every document can do,
remote pages included, and widens no boundary.

## How
1. **The request.** `Request` gains `initiator: Option<Url>`. `Request::get` leaves it `None`, so every embedder's
   top-level load stays as it is.
2. **The rule.** The one predicate, `document_may_load`, moves to `blitz_traits::net`. blitz-vibey-script re-exports
   it, so its public path stays.
3. **The refusal.** blitz-net's `file:` arm refuses with `ProviderError::FileForbidden`, fixed text with no path in it.
4. **The stamp.** blitz-dom stamps its document's URL on all 7 fetch sites. These are the linked stylesheet and its
   reload, `@import`, `@font-face`, the CSS image, `<img>` and `<iframe>`. The URL is threaded through
   `StylesheetLoader`, `StylesheetHandler` (a public field, a public API change) and `fetch_font_face_rules`.
5. **The witness.** A new blitz-tests target, `local_files_for_local_documents`, runs in both layout modes:
   - a recording stand-in shows each of the 7 paths stamped with the document's URL;
   - a stand-in that hands each request to blitz-net's real `Provider::fetch_async` shows the difference. An `http:`
     page's `file:` stylesheet, `@import`, image and iframe read nothing. A `file:` page reads all four.

## Top acceptance criteria
- The witness, both modes: a remote page's four local loads read nothing, and a `file:` page's read all.
- Every engine sub-resource request carries its document's URL (7 paths).
- blitz-net serves no-initiator and `file:`-initiator requests and refuses an `https:` one, offline.
- `document_may_load` has one body, and `blitz_vibey_script::document_may_load` still resolves.
- The census shows no new log site, print, env read or panic path in changed production code. The witness has no
  print, env read, listener, sleep or wall-clock read.
- `fast` and `audit` pass locally, and the operator's CI read shows `verdict: green` (windows and macos run the
  `file:` paths).

## Touchpoints
- **New:** `tests/blitz-tests/tests/local_files_for_local_documents.rs`.
- **Modified:**
  - `packages/blitz-traits/src/net.rs`
  - `packages/blitz-vibey-script/src/fetch.rs`
  - `packages/blitz-net/src/lib.rs`
  - `packages/blitz-dom/src/{net,document,iframe,mutator,cssom}.rs`
  - `packages/blitz-dom/src/layout/damage.rs`
  - `tests/blitz-tests/Cargo.toml`, which gains the dev-dependencies `blitz-net` and `tokio`
  - `Cargo.lock`, where only blitz-tests' own dependency list moves

## Test commands (13 entries)
| Entries | What they run | Baseline on the untouched tree |
|---|---|---|
| 1–3 | unit: `cargo test -p blitz-traits`, `-p blitz-net`, `-p blitz-vibey-script` | green |
| 4 | the witness | red: no such target yet |
| 5 | 5 load-path checks under the workspace build | green |
| 6 | the silence and panic census | green, controlled red |
| 7 | the witness's silence | red: file absent; must-pass green and control red in a clone |
| 8 | shared modules and harness untouched | green, controlled red |
| 9 | `ci-leg.sh fast` | seen green |
| 10 | `ci-leg.sh audit` | green |
| 11–13 | operator: hygiene, push, `ci.py` read | — |

The controls are recorded in `.andromeda/runs/2026-11-13T23-09-24-phase/p5-controls.md`.

## Capabilities
None claimed. No 0.3.0 capability in the pool (v030-08 … v030-15) is advanced by this chunk, so there is no
concretization preview.

## Flags
- **Aggregate B:** 1 master read `picked 0` (layout-templates, N/A to a net change). That passes.
- **No fork asked by dialog.** The founder is away. His one fork is written whole in
  `.andromeda/runs/2026-11-13T23-09-24-phase/founder-forks.md` as F1, with lean A. The plan stands without it.
- **Expected amendments (wrap)** are listed in the plan's Implementation notes:
  - architecture: the clause on decision/file-script-load-refused (`inputs#I1`), the rows for the file URL rule,
    scheme routing, the blitz-net fetch contract, the predicate's home, `Request`'s field, and the witness's test-only
    directory;
  - security-plan: the `file:` rules extended to sub-resources, and `CARGO_TARGET_TMPDIR` gaining a reader;
  - test-plan: the guards, the retired «file: read has no test» absence, and the target counts;
  - obs-plan: decision/no-file-loads-from-non-file-documents.

## Beyond the plan
| # | Proposal | Kind | Owner proposed |
|---|---|---|---|
| 1 | The browser's favicon probe (`apps/browser/src/favicon.rs:49`) still reads a remote page's `file:` icon from disk on its behalf, and navigation from a remote page to a `file:` URL still loads it as a top-level document. Both stay open after this chunk. | route insertion (the founder's fork F1, lean A) | A new working-route entry «Remote pages and local files in the browser», placed at the end of Epoch 4 after «Auto-exploration»: the favicon probe stamped with its page, and `file:` navigation from a non-`file:` document refused. It stays pending the founder's word on F1 (B: a stated limit; C: fold here). |

Apply? (yes / review / cancel)
