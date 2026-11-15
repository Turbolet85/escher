# P5 controls — 2026-11-15-browser-corrective

Clone: `git clone --shared` of the project at HEAD 73a168d5af28348ddcfe890e4f48675ca7bd5839, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/2761e57f-5ef1-417d-8012-6509efc62214/scratchpad/ctl-clone`
(role: the control clone). Each entry fired as itself: `gate.py run --chunk 2026-11-15-browser-corrective --docs-root
{root} --entry {n} --project-root {clone}`, no `--run-dir`. Inputs by
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/2761e57f-5ef1-417d-8012-6509efc62214/scratchpad/ctl_edit.py`.

Unmodified (must read as on the tree):
- 3 probe red · last line '0' ✗ (last: '2') · exit 0 — as on the tree
- 4 probe green · exit 1 — as on the tree
- 5 probe green · exit 0 — as on the tree

Input `justify` (a `//` line added directly above `favicon.rs:76`'s and `tab.rs:178`'s allow, as steps 6–7 build):
- 3 probe green · exit 0 — the must-pass reading
- 4 probe green · exit 1 — a benign added line passes

Input `println` (an added `fn _control() { println!("x"); }` line in `apps/browser/src/tab.rs`):
- 4 probe red · exit 1 ✗ (exit 0) — an added print fails

Input `lock` (a `# control` line appended to `Cargo.lock`):
- 5 probe red · exit 0 ✗ (exit 1) — a changed lockfile fails
