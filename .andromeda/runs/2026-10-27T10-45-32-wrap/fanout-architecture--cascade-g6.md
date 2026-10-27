proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 78
    section: "## Occupied Resources"
    change: "Replace the `TMPDIR` clause with: `TMPDIR`, through `std::env::temp_dir()` — on unix that variable, else `/tmp` — at two test-only sites, test code only and not secret: escher-driver's unix-only client unit test, to place its test-only listener's directory (packages/escher-driver/src/client.rs:263-308; since 2026-10-14-quality-gates), and, since 2026-10-27-upstream-sync-engine-debts (upstream `cee6b8731`), blitz-vibey-script's `tests/dom.rs` helper `temp_dir_with_files`, which places the `file:` load tests' directory (packages/blitz-vibey-script/tests/dom.rs:860-869)"
    sidecar: "occupied-resources Environment variables: `TMPDIR` read at a second test-only site, blitz-vibey-script `tests/dom.rs` `temp_dir_with_files` (2026-10-27-upstream-sync-engine-debts)"
    rationale: "Report Changes › Symbols / APIs: one new test-only env read, `TMPDIR` through `std::env::temp_dir()` in `tests/dom.rs` `temp_dir_with_files` (860-869 @861, the call at :863); Counts `tmpdir` 1 → 2 sites, `architecture-detail/occupied-resources.md:78` changed"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:32
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture.md
    near: 216
    section: "## Occupied Resources"
    change: "Replace `TMPDIR`, in one unit test only` with `TMPDIR`, at two test-only sites (escher-driver's client unit test and blitz-vibey-script's `tests/dom.rs`)` — the rest of the broad line unchanged"
    sidecar: "architecture §Occupied Resources: the broad Environment variables line counts two test-only `TMPDIR` sites"
    rationale: "Counts `tmpdir`: `architecture.md:216` \"in one unit test only\" changed — the second site is blitz-vibey-script `tests/dom.rs:863`"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:32
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 32
    section: "## Occupied Resources"
    change: "After the `eab-{pid}` clause add: since 2026-10-27-upstream-sync-engine-debts (upstream `cee6b8731`), blitz-vibey-script's `tests/dom.rs` helper `temp_dir_with_files` also makes a directory under the system temporary directory (`std::env::temp_dir()`) and writes the `file:` load tests' files into it — `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls` load from it — test code only, local only and uploaded by no CI leg (packages/blitz-vibey-script/tests/dom.rs:860-869; packages/blitz-vibey-script/tests/dom.rs:904-950)"
    sidecar: "occupied-resources Filesystem: blitz-vibey-script's `tests/dom.rs` `temp_dir_with_files` test directory under the system temporary directory registered (2026-10-27-upstream-sync-engine-debts)"
    rationale: "Report New text: `tests/dom.rs` adds `temp_dir_with_files(name, files) -> PathBuf` at 860-869 (`let dir =` 862-863, `for (file, contents) in files` 865-867), used by the two `file:` load tests at 904-928 and 930-950 — a test filesystem write outside `target/tmp/`, the class the entity registers for `eab-{pid}`; the report does not state whether the directory is removed, so the change says nothing of removal — the orchestrator should confirm that before it lands"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:621
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 21
    section: "## Cross-cutting Patterns"
    change: "After the clause on redraws being skipped while critical resources are pending or the window is occluded, add: in the windowed shell a redraw request travels through its event loop (since 2026-10-27-upstream-sync-engine-debts, upstream `6587e7e79`; packages/blitz-shell/src/lib.rs:93-98; packages/blitz-shell/src/window.rs:193-194), and the shell provider's `set_ime_enabled(true)` is also called when a text input that already holds focus has its editor constructed (upstream `7fc9ee3af`; packages/blitz-dom/src/layout/construct.rs:313-317)"
    sidecar: "cross-cutting-patterns Service wiring: the windowed shell's redraw route through its event loop and the IME enable at a focused input's editor construction (2026-10-27-upstream-sync-engine-debts)"
    rationale: "Report Changes › Symbols / APIs: blitz-shell `lib.rs:90`, `:93-98`, `:104-106` and `window.rs:193-194` route the windowed shell's redraw requests through its event loop (`6587e7e79`), and `create_text_editor` (`construct.rs:313-317`) calls `shell_provider.set_ime_enabled(true)` for an input already focused (`7fc9ee3af`); Counts `redraw` reads `cross-cutting-patterns.md:21` at P2 for the shell and its provider"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:15
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/project-intent.md
    near: 9
    section: "## Project Intent"
    change: "Upstream sync: DioxusLabs/blitz `main` last merged at 23b9b26a9b6af5947b8f003d2c2cc768bff10b8a (merge commit 3eda07bc, parents `143b6013` · `23b9b26a9`, 2026-10-17) — the next sync's merge base, unmoved; upstream `main` last read at `ebacd68bd` (it was `f9cbb2756` at 2026-10-24-upstream-sync-polish-and-ship, whose thirteen no-commit cherry-picks stand), and eleven more of its commits taken on the founder's word by no-commit cherry-picks of upstream's text in one commit (`e795a088c`), each with its `(cherry picked from commit …)` trailer: `d63ed2c8c` (browser tab lenses kept across a webview props replacement) · `6587e7e79` (the windowed shell's redraw requests through its event loop) · `e6f4615ec` (blitz-vibey-script's runtime and element bindings split into modules) · `a7ea9631b` · `b5b163450` (publish-browser) · `7fc9ee3af` (IME enabled at a focused input's editor construction) · `2963b65a2` (`::before`/`::after` voting in flow classification) · `cee6b8731` (a `file:` script load only for a `file:` document) · `53e9d81c6` (`:open`) · `6fee0db25` (the WPT pin) · `efdc5472a` (CSS properties as `CSSStyleDeclaration` accessors); sixteen left out, each with its measured reason — git Stylo 0.23-pre (`06ecec886`, with `3954ecb85` and `1c5265a8b`), the six Taffy-main commits (`b8f4ba6fd` and `9babd796a`, a git Taffy, with `1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6`) and Dioxus 0.8.0-alpha.3 (`286874cc1`) still wait for escher's next version, owned by a `.andromeda/residuals.md` line (target: next), so Stylo stays \"0.22.0\", Taffy the registry \"0.15\" and Dioxus \"0.7.3\", with no git dependency (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/lock.md)"
    sidecar: "project-intent Upstream sync: upstream main read at `ebacd68bd`, eleven commits taken by no-commit cherry-pick on the founder's word, sixteen left out, merge base `23b9b26a9` unmoved (2026-10-27-upstream-sync-engine-debts)"
    rationale: "Report Counts `upstream`: `f9cbb2756` → `ebacd68bd`, merge base `23b9b26a9` unmoved, eleven taken, sixteen left out, `architecture-detail/project-intent.md:9` changed; Symbols / APIs lists the eleven shas in take order and attributes ten of them, so `53e9d81c6` is `:open` by elimination; Dependencies: none, so the Stylo / Taffy / Dioxus pins and their wait stand"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:34
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 287
    section: "## Project Intent"
    change: "Upstream sync: DioxusLabs/blitz `main` last merged at 23b9b26a9b6af5947b8f003d2c2cc768bff10b8a — the next sync's merge base; upstream `main` last read at `ebacd68bd`, eleven more of its commits taken by no-commit cherry-picks on the founder's word and sixteen left out, with the merge base unmoved; git Stylo 0.23-pre, the six Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version, so Stylo stays \"0.22.0\", Taffy the registry \"0.15\" and Dioxus \"0.7.3\"."
    sidecar: "architecture §Project Intent: the broad Upstream sync line reads upstream main at `ebacd68bd`, with eleven taken and sixteen left out"
    rationale: "Report Counts `upstream`: `architecture.md:287` changed — the \"last read at\" reading moves from `f9cbb2756` to `ebacd68bd`, and the take count from thirteen to this sync's eleven"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:34
    dependent-of: D-cascade
note: "F1–F9 belong to established-decisions, standard-contracts and existing-scopes, not to this group's categories; the redraw route (F4) and IME-on-construct (F3) placed by the new-detail pass in [Shell and windowing] should agree with the Service wiring addition above; the `js_console` and uncaught-JS-error citations at occupied-resources.md:52 and cross-cutting-patterns.md:15 and :27 are the citation sweep's"
