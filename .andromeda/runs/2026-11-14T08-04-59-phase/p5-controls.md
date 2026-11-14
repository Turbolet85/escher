# P5 controls — 2026-11-14-bundled-fonts

Each fired as the entry, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --chunk 2026-11-14-bundled-fonts --docs-root . --entry {n} --project-root $TMPDIR/{scratchpad}/ctl-bf`, over a `git clone --shared` of the project at HEAD `557f1c98d4e5639ec7e065025d092feda234b3b7`; the inputs lived in the clone alone.

- entry 6 (the no-production-change guard), unmodified clone: `6 probe green · exit 0` — the reading on the tree.
- entry 6, `// control` appended to `packages/escher-driver/src/lib.rs` (must fail): `6 probe red · exit 0 ✗ (exit 1)`.
- entry 6, the clone holding only the two new untracked test files (must pass): `6 probe green · exit 0`. The read is `git diff` against the base, so an untracked production file passes it until the operator's pre-CI commit; the wrap's light gate re-runs it on the committed tree, where a new file under a guarded path reads red.
- entry 5 (the silence census), clean stubs of both new files (must pass): `5 probe green · exit 0` (last line 0).
- entry 5, a `println!` appended to `stand_bundled_font_families.rs` (must fail): `5 probe red · last line '0' ✗ (last: '1')`.
