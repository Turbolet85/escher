# escher — «Dependency and CI debts»: the founder's forks

Chunk `2026-11-21-dependency-and-ci-debts`, taken up 2026-11-21 (escher-0.4.0, Epoch 2's last entry, capability
`v040-07`). Written in the away form on the operator's directive (inputs#I1): researched and costed. The phase stops
here until his answers come back.

Already ruled, so no fork here: **skia's verified archive** («Проверенный архив (Recommended)», 2026-11-20). Skia stays
a default. Each platform's prebuilt archive is fetched once and its sha256 recorded in the repository. CI and the local
gates check the sum and point the build at the verified file through `SKIA_BINARIES_URL`. No line of upstream's code
changes.

How it will be built:
- one stdlib Python script per target, run on each platform;
- `FORCE_SKIA_BINARIES_DOWNLOAD` is set too, so a missing or wrong file stops the build instead of silently building
  skia from source;
- the five archives needed (Linux, macOS, iOS, Windows, Android) are all published with a sha256 digest, and the dev
  host's Linux copy matches its digest.

Needing no fork either — each is a fix, a closure or a wait on another project's release:
- **zip** (four advisories): already closed. Upstream's AnyRender 0.15 brought zip 9.6, and the ignores left on
  2026-11-17. Only the record is closed here.
- **zbus family** (three, unmaintained): wait on an `accesskit_unix` release on zbus 6. 0.24.1 is the latest and is
  still on zbus 5.
- **paste** (unmaintained): wait on a `rav1e` release without paste. It reaches only the opt-in `avif` feature, never
  a default build.
- **ttf-parser** (unmaintained): wait on a release of the winit 0.31 beta chain. Every crate on that chain is at its
  latest.
- The operator's technical calls (inputs#I3):
  - the `CI=true` rebuild is stated as a limit, with its measured cause: Cargo itself turns incremental builds off
    under `CI`;
  - one lock-only update replaces the yanked `async-executor`;
  - `v040-08` is claimed, gated on the operator's word at the Epoch 2 boundary audit.

---

## F1 — The SQLite advisory in the browser's history store (RUSTSEC-2026-0459)

**Measured.**
- `libsqlite3-sys` 0.30.1 bundles a C SQLite with CVE-2026-119375. The fix is `libsqlite3-sys` ≥ 0.39, which comes
  with `rusqlite` ≥ 0.41.
- It is reached only through the reference browser app's history store: `rusqlite` 0.32 ← `browser-persistence` ←
  `browser`. Nothing in the driver, the stand or the engine libraries reaches it.
- The store opens only its own `history.sqlite3` in the app's data directory, or an in-memory database (always on
  Android and iOS).
- The manifest pin (`rusqlite "0.32"`, `rusqlite_migration "1.3"`) and the store's code (507 lines) are upstream's,
  byte-equal to upstream's main as of 2026-11-19. Upstream is still on 0.32.
- A move means `rusqlite` 0.41 with `rusqlite_migration` 2.7. The store uses only plain calls (open, prepare, query,
  execute, one transaction, one migration list).
- The audit ignores the advisory by ID today and re-reads it on every push.

**Options** (the recommended one first):

**A — Keep the ignore, stated as a wait on upstream's move (Recommended; least change).**
- What happens: the ignore stays, its reason naming its owner record. It drops when an upstream sync brings
  `rusqlite` ≥ 0.41.
- Cost: none now. The reference browser keeps the vulnerable SQLite until upstream moves.
- Why first: it matches his 2026-11-20 rulings to keep upstream's code as upstream's. The exposure is an app-private
  database the fork's own surfaces never open.

**B — Move the fork to rusqlite 0.41 now.**
- What happens: two manifest lines in upstream's app change, perhaps a few lines of the store's code too. The lock
  moves (every CI cache re-keys once), and the entering crates are reviewed.
- Cost: a standing divergence in an upstream-owned file. Each upstream sync re-resolves it until upstream moves.
- Gain: the advisory closes now.

**C — Offer the move to upstream (a pull request to DioxusLabs/blitz) and wait for it.**
- What happens: same as A until merged; the fix then arrives with a sync, with no divergence.
- Cost: a public, outward-facing contribution, which is his call, and its timing is upstream's.

---

## F2 — The cancelled-runner watch (hosted macOS/iOS jobs cancelled with no runner)

**Measured** over the fork's last 60 CI runs (2026-11-16 → 2026-11-21):
- 8 runs had a macOS or iOS job cancelled before any step ran, because GitHub assigned no runner: 13 cancelled
  attempts in all.
- Every one went green on a re-run. The worst needed 6 attempts.
- The watch's retire clause reads a recurrence as the trigger to act. The recurrence came on 2026-11-19 at
  `d9af170dc`.
- Since then: 21 runs, every macOS and iOS job green, 20 of them on the first attempt.
- Today the operator pass re-runs a no-runner cancel by hand, up to six times.
- The CI token is read-only by design (`contents: read`, no job-level grant).

**Options:**

**A — Retire the watch, stating the cause as a limit (Recommended; least change).**
- What happens: a no-runner cancel is recorded as GitHub's hosted-runner availability, not a defect of the fork. The
  operator pass's hand re-run (up to six) is the stated remedy. No CI change. This is retiring by the watch's own rule
  and needs no ruling unless he prefers B or C.
- Cost: a cancel still costs a manual re-run when it happens.

**B — Re-run such jobs automatically.**
- What happens: a workflow step or job re-runs a matrix job cancelled with no runner.
- Cost: it needs an `actions: write` token, which widens the CI token past read-only. That is the playbook's «Boundary
  widening» pattern, so it is his word and the wrap halts on it. It also adds a new pinned and tested workflow piece.

**C — Keep the watch open with a new end condition** (for example, retire after two weeks with no recurrence).
- What happens: the observation carries into Epoch 3.
- Cost: nothing now; the question returns later.

---

## F3 — memmap2's unsound function, inside the browser's HTTP disk cache (RUSTSEC-2026-0186)

**Measured.**
- `memmap2` 0.5.10 is reached at default features through `cacache` 13.1.0 ← `http-cache` ← `blitz-net` (its `cache`
  feature, a browser default).
- The advisory names one function, `Mmap::advise_range`. `cacache` uses only `MmapMut` and never calls it.
- `cacache` 13.1.0 is the latest release (2024-11) and still requires memmap2 0.5, so no compatible fix exists.
- The audit keeps the advisory out through cargo-deny's `unsound` default. This was measured and ruled with the
  whole-workspace audit on 2026-10-14.

**Options:**

**A — State it as a wait on a `cacache` release, with the measured non-use (Recommended; least change).**
- Cost: none. The record names the function that is never called.

**B — Move blitz-net off `cacache` (for example, to http-cache's in-memory `moka` manager).**
- Cost: a change to upstream's `blitz-net`. The browser's HTTP cache stops persisting to disk, a behaviour change. It
  adds `packages/blitz-net/` to this chunk, which re-runs the plan's master read.

**C — Turn the audit's `unsound` reporting on and ignore memmap2 by ID.**
- Cost: the gate begins failing on unsound advisories everywhere, so more ignores will follow. It fixes nothing; it
  only makes this one visible.

---

**Answer form:** one option per fork, F1 · F2 · F3. A typed answer in his own words stands as given and is relayed
verbatim.
