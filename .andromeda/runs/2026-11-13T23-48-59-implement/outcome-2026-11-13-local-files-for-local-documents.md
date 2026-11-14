# Implement outcome — 2026-11-13-local-files-for-local-documents

**Outcome: green** (smoke skipped: no boot-path or UI-surface change). The plan's 11 local entries all passed on the first run, with no fix iterations. All three controls hold. Entries 12–14 are the operator's.

## Files changed
- New: `packages/blitz-dom/clippy.toml` · `tests/blitz-tests/tests/local_files_for_local_documents.rs`
- Modified: `packages/blitz-traits/src/net.rs` · `packages/blitz-traits/src/navigation.rs` (scope record) · `packages/blitz-vibey-script/src/fetch.rs` · `packages/blitz-net/src/lib.rs` · `packages/blitz-dom/src/{net,document,iframe,mutator,cssom}.rs` · `packages/blitz-dom/src/layout/damage.rs` · `tests/blitz-tests/Cargo.toml` · `Cargo.lock`. The lockfile gains `blitz-net` and `tokio` in blitz-tests' list only; no new package.

## Gates (trail `.andromeda/runs/2026-11-13T23-48-59-implement/gate-2026-11-13-local-files-for-local-documents.json`)
`entries 14 · green 11 · red 0 · recorded 0 · timeout 0 · not-run 3`. Entries 12, 13 and 14 read `not run — leg operator`.
- Entry 1 ran `net::tests::file_urls_load_only_for_file_documents ... ok`, 1 passed.
- Entry 6 printed `held` for every file and ended on `0`.
- Entry 10 (`fast`) passed in 228.74 s, and entry 11 (`audit`) passed.
- No entry carries `defer`, so nothing was deferred or voided. The header reads `walk-class 0 marked`.

## Controls (`gate.py control`, `control-1.toml`; lines in `.andromeda/cache/chunks/2026-11-13-local-files-for-local-documents/evidence/control-1.txt`)
Summary: `control: 3 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`. With each guard in place, entries 4 and 9 passed in the clone.

- **refusal** (entry 4; the guard is `return Err(ProviderError::FileForbidden);` in blitz-net's `file:` arm)
  - The guard's own case: `a_remote_page_reads_no_local_file_and_a_file_page_reads_them_all`, the remote half (a remote page reads none of the four files).
  - Removed firing `4.without-refusal.log`: **the case ran and failed.**
    - The log reads `test a_remote_page_reads_no_local_file_and_a_file_page_reads_them_all ... FAILED`, panicked at `local_files_for_local_documents.rs:261:9`.
    - `left: Readings { width_37: true, height_23: true, image: Some((2, 3)), sub_document: true }` against the remote expectation `right: Readings { width_37: false, … sub_document: false }`.
    - The other test, `every_engine_request_carries_its_document_as_initiator`, still passed (`... ok`), as it should, since the stamp was still in place.
- **stamp** (entry 4; the guard is `.initiator(initiator.clone())` in `stamped_request`)
  - The guard's own case: `every_engine_request_carries_its_document_as_initiator`.
  - Removed firing `4.without-stamp.log`: **the case ran and failed.**
    - The log reads `test every_engine_request_carries_its_document_as_initiator ... FAILED`, panicked at `:130:13` with `left: None` against `right: Some(Url { scheme: "http", … host: Some(Domain("remote.test")), … path: "/page.html" … })`.
    - The read-through test also failed (line 261, the same `Readings` as above). With no initiator, blitz-net served the remote page's files.
    - Result line: `0 passed; 2 failed`.
- **expect** (entry 9; the guard is the `#[expect(clippy::disallowed_methods, …)]` attribute above `stamped_request`'s `Request::get`)
  - The guard's own case: the `disallowed-methods` lint at that call.
  - Removed firing `9.without-expect.log`: **the case ran and failed.**
    - The log reads `error: use of a disallowed method `blitz_traits::net::Request::get`` at `--> packages/blitz-dom/src/net.rs:32:19`, with `requested on the command line with -D clippy::disallowed-methods`.
    - Then `could not compile blitz-dom (lib)` and `(lib test)`, exit 101.
    - It fired at the one site the P5 known-positive named, and at no other.

## Deviations (with justifications)
- **`packages/blitz-traits/src/navigation.rs`** — scope record: ``- `packages/blitz-traits/src/navigation.rs` · companion · serves packages/blitz-traits/src/net.rs · self``. `NavigationOptions::into_request` builds a `Request` literal inside blitz-traits, so it needed the new field. It sets `initiator: None`, because a navigation is a top-level load.
- **The stamp test's recording stand-in answers each `.css` request with an empty sheet.** The plan said it "answers nothing". `reload_resource_by_href` only walks stylesheets already applied (`nodes_to_stylesheet`), so a linked sheet that never answers could not be reloaded, and the reload path would record no request. Every other request is left in flight. Its `<link>` sits in `<body>`, so it does not block render while the CSS background image is styled.
- **The read-through page puts its `@import` in an inline `<style>`, not inside the linked sheet.** This way the remote page's `@import` is refused on its own path, and not trivially because its parent sheet was refused.
- **The predicate table has 6 rows**, and the row count is asserted. The plan's last bullet («an `https:` URL and a `data:` URL for an `https:` document») is two rows.
- **`fetch_font_face` gained the `document_url` parameter too.** It is the wrapper that calls `fetch_font_face_rules`, at `document.rs` `add_stylesheet_for_node` and `net.rs` `NestedStylesheetHandler`, both in listed files. A linked sheet's `@font-face` is stamped there with `&self.url`. `StylesheetHandler.document_url` fills the loader it builds for that sheet's `@import`s.
- No guard entry (`gate.py guard`) is in the plan, so there is no pass line to copy.

## Matrix
No capability rows for this chunk (`coverage where chunk = …`: 0 rows). No capability is claimed, as the plan states.

## Surfaced / stuck
None.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo · rustc · clippy-driver · test binaries (gate entries and control firings) | this run (via gate.py) | terminated — `census: none running` (comm cargo, rustc, clippy-driver, local_files_for_local_documents, escher-session · args andromeda-control) |
| control clone `~/.cache/andromeda-control/7a374c5c` | this run (gate.py control) | removed — `clone removed` |

## Decisions & corrections
- The operator's directive at invocation: "For every gate.py control you fire, write in the outcome: the guard's own case, and whether the removed firing's log shows that case run and failed." It is applied above, per control, read from each `*.without-*.log`.
- Guard refusal: twice, the bash-guard `cd` arm refused a `cd` out of the project root (references dir, `packages/blitz-dom/src`). Both were re-issued with absolute paths.

## Next
The OPERATOR PASS, entries 12 (hygiene), 13 (push) and 14 (CI read), in that order. Then `/andromeda-wrap-session`.
