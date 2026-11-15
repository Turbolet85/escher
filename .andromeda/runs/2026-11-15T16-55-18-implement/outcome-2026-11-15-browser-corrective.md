# Outcome — 2026-11-15-browser-corrective

**Outcome:** green — 0 fix iterations; smoke skipped (no boot-path or UI-surface change).
**Run:** 2026-11-15T16-55-18-implement · mode: pending (not gated)

## Files changed
- `apps/browser/src/document_loader.rs` — a `#[cfg(test)] mod tests` appended (lines 274-390, `.andromeda/runs/2026-11-15T16-55-18-implement/new-text.md`): a `NoShell` stand-in, a `Probe` root context, `config_app` / `probe_config`, and the three tests. No line above it changed.
- `apps/browser/src/favicon.rs` — one `//` justification line (76) above the test module's allow.
- `apps/browser/src/tab.rs` — two `//` justification lines (178-179) above `TabWebView`'s allow.

## Deviations (with justification)
- `tab.rs`'s justification is two comment lines, not the plan's one (step 7): one line would pass the file's 100-column width; the awk probe reads the line directly above the allow, so it holds (entry green, `last line 0`).
- Plan step 2 wrote `Arc::new(NoShell) as Arc<dyn ShellProvider>` and `*const ()` casts; written as a typed `let shell: Arc<dyn ShellProvider>` and `Arc::as_ptr(..).cast::<()>()` — the same values, no `as` cast (the operator's global rule: no `as` casts unless justified).
- The two root contexts are given by `.with_root_context(shell).with_root_context(Rc::clone(&probe))`; the probe is an `Rc<RefCell<Probe>>` read back by `RefCell::take` — the plan's named pattern (`dioxus_document.rs:639`).
- Assertion messages name the reading (`base_url`, `net_provider`, `navigation_provider`, `current url`); `assert_eq!` on a failure still prints its two operands, the fixture URL literals the test writes — the same form as `history.rs`'s tests; no call-supplied value exists here.
- Scope record: none — `gate.py scope` clean, 0 recorded (changed 3 · listed 3).

## Gates (`gate.py run`, trail `.andromeda/runs/2026-11-15T16-55-18-implement/gate-2026-11-15-browser-corrective.json`)
- `cargo test -p browser --locked` — green · exit 0 · the three `document_loader::tests::… ... ok` atoms held; 38 passed.
- `cargo test -p browser --locked -- --list` — green · exit 0 · both `…: test` atoms held.
- the awk allow-justification probe — green · exit 0 · `last line 0` (2 → 0).
- the silence probe (`git diff 73a168d5a -- apps/browser | grep -cE …`) — green · exit 1 · `last line 0`.
- `git diff --quiet 73a168d5a -- Cargo.lock apps/browser/Cargo.toml` — green · exit 0.
- `cargo nextest run -p browser --locked` — green · exit 0.
- the cargo-mutants kill read — green · exit 2 (not asserted) · `Found 11 mutants to test` · `lacks` `:52:5:`, `:56:9:`, `:57:9:` held. Summary line: `11 mutants tested in 87s: 7 missed, 4 caught` (predicted 7). MISSED lines, verbatim:
  - `MISSED   apps/browser/src/document_loader.rs:58:9: delete field shell_provider from struct DocumentConfig expression in make_doc_config in 2s build + 1s test`
  - `MISSED   apps/browser/src/document_loader.rs:54:9: delete field viewport from struct DocumentConfig expression in make_doc_config in 78s build + 1s test`
  - `MISSED   apps/browser/src/document_loader.rs:60:9: delete field font_ctx from struct DocumentConfig expression in make_doc_config in 2s build + 1s test`
  - `MISSED   apps/browser/src/document_loader.rs:59:9: delete field html_parser_provider from struct DocumentConfig expression in make_doc_config in 2s build + 1s test`
  - `MISSED   apps/browser/src/document_loader.rs:61:9: delete field media_type from struct DocumentConfig expression in make_doc_config in 2s build + 1s test`
  - `MISSED   apps/browser/src/document_loader.rs:62:9: delete field abort_signal from struct DocumentConfig expression in make_doc_config in 2s build + 0s test`
  - `MISSED   apps/browser/src/document_loader.rs:63:9: delete field style_threading from struct DocumentConfig expression in make_doc_config in 2s build + 0s test`
- `bash .github/scripts/ci-leg.sh fast` — green · exit 0 · 299.67 s.
- operator entries (hygiene, push, CI read) — `not run — leg operator`: the operator pass's.
- No `gate.py guard` entry in the plan. No deferral.

## Surfaced / stuck
- none.

## Process hygiene
`gate.py census --comm cargo --comm cargo-mutants --comm cargo-nextest --comm rustc --args mutants-tmp` → `census: none running`.

| process | started by | final state |
|---|---|---|
| cargo test / cargo nextest / cargo mutants / ci-leg.sh fast (and their rustc children) | this run (gate entries 1-8) | terminated |

## Decisions & corrections
- The operator's global rule «no `as` casts unless justified with a comment» (`~/.claude/CLAUDE.md`) shaped the test's two casts the plan wrote with `as` — typed `let` and `.cast::<()>()` instead.

## Next
The OPERATOR PASS — the plan's `operator` entries 9 (`gate.py hygiene`), 10 (the push) and 11 (`ci.py conclusion --sha HEAD --wait 1800`) — then `/andromeda-wrap-session`.
