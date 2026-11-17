# Guard entry 4 (revision 1) — baseline and control readings

## Baseline — the tree as the stopped implement run left it (HEAD 66fbe33be, the three touchpoints uncommitted)
`gate.py run --chunk 2026-11-17-stand-requirement-sweep --run-dir .andromeda/runs/2026-11-17T22-25-01-phase --entry 4`:
```
  4 probe       green · exit 0 · 0.72s · 465 B → 4.log
guard: allowed 3 — apps/browser/src/document_loader.rs, tests/blitz-tests/tests/stand_masks_on_new_answers.rs, tests/blitz-tests/tests/stand_requirement_sweep.rs · companions excluded 0 · changed 191 · excluded 188
guard: clean
```

## Control guard-stray — fired as the entry over a scratch clone (validation.md check 4 (9))
Clone: `git clone --shared --no-checkout` of the project into
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/0fefd225-c2bd-4fc2-8427-68c60ce0943a/scratchpad/guardclone`, `checkout --detach`
at HEAD `66fbe33be80919f0c92810b982e8ca169581b6b3`; the three touchpoints copied from the tree. Fired
`gate.py run --chunk 2026-11-17-stand-requirement-sweep --docs-root . --entry 4 --project-root {clone}`, no run dir.

- must pass — input: the clone holding the three touchpoints alone:
  ```
    4 probe       green · exit 0 · 0.71s · 461 B → 4.log
  guard: allowed 3 — apps/browser/src/document_loader.rs, tests/blitz-tests/tests/stand_masks_on_new_answers.rs, tests/blitz-tests/tests/stand_requirement_sweep.rs · companions excluded 0 · changed 3 · excluded 0
  guard: clean
  ```
- must fail — input: the same clone, one line (`stray line`) appended to its `README.md`:
  ```
    4 probe       red · last line 'guard: clean' ✗ (last: 'guard: outside 1 files') · exit 0 · 0.72s · 489 B → 4.log
  outside README.md
  guard: allowed 3 — apps/browser/src/document_loader.rs, tests/blitz-tests/tests/stand_masks_on_new_answers.rs, tests/blitz-tests/tests/stand_requirement_sweep.rs · companions excluded 0 · changed 4 · excluded 0
  guard: outside 1 files
  ```

The clone is left in the session scratchpad: the host's permission settings refuse a recursive `rm`.
