group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/docs/security-summary.md:14   class: omission
  leaf-text: «default `ScriptFetcher` accepts only `file:` / `data:`;»
  source: .andromeda/security-plan.md:101 «a `file:` URL loads only for a `file:` document, `fetch()`, a module `import` and `<script src>` each refused otherwise with a `FetchError` — a narrowing»
  replace-with: «default `ScriptFetcher` accepts only `file:` / `data:`, and a `file:` URL loads only for a `file:` document — `fetch()`, a module `import` and `<script src>` each refused otherwise with a `FetchError` (`document_may_load`, remote pages included; a narrowing);»

- leaf: .claude/docs/security-summary.md:13   class: omission
  leaf-text: «(target: next); and, since 2026-10-20-select-by-id»
  source: .andromeda/security-plan-detail/input-validation.md:186 «Read by its presence on a `details` or a `dialog`, in every document, remote pages included: `ElementData::flush_open_state` sets `ElementState::OPEN` from it»
  replace-with: «(target: next); since the 2026-10-27 upstream sync `open` is read by its presence on a `details` or a `dialog`, in every document, remote pages included, setting `ElementState::OPEN`, which `:open` matches — no attribute text carried out, no print or log site, an engine change for every document; and, since 2026-10-20-select-by-id»

- leaf: .claude/rules/security.md:15   class: omission
  leaf-text: «- The default `ScriptFetcher` accepts only `file:` and `data:`; same-origin/CORS checks are absent»
  source: .andromeda/security-plan-detail/threat-model-summary.md:18 «A `file:` URL loads only for a `file:` document: `document_may_load` refuses it for any other document, remote pages included, with a `FetchError`, checked by JS `fetch()`, module `import` and `<script»
  replace-with: «- The default `ScriptFetcher` accepts only `file:` and `data:`, and a `file:` URL loads only for a `file:` document (`document_may_load`: JS `fetch()`, a module `import` and `<script src>` each refused otherwise with a `FetchError`, remote pages included — a narrowing); same-origin/CORS checks are absent»

- leaf: .claude/rules/security.md:9   class: stale
  leaf-text: «plus `TMPDIR` through `std::env::temp_dir()` in escher-driver's unix-only client unit test (test code only, not secret)»
  source: .andromeda/security-plan.md:302 «`TMPDIR`, through `std::env::temp_dir()` — on unix that variable, else `/tmp` — at two test-only sites: escher-driver's unix-only client unit test `client::imp::tests`»
  replace-with: «plus `TMPDIR` through `std::env::temp_dir()` at two test-only sites, escher-driver's unix-only client unit test and blitz-vibey-script's `tests/dom.rs` helper `temp_dir_with_files` (test code only, not secret)»
