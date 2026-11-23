# P5 controls — revision 1 (2026-11-23)

Scratch clone `$TMPDIR/scratchpad/cookieclone` of `d43bc0e7f`, HEAD `237024408` (the chunk's tree as the stopped implement
run left it, committed in the clone). Each entry fired through `gate.py run --docs-root . --project-root {clone} --entry {n}`.

| entry | clone role | input | reading |
|---|---|---|---|
| 9 lock probe | unmodified (the edit stashed) | lock = the chunk's (`parley` line) | red · last line `1+0` (as on the tree) |
| 12 audit | unmodified | `cookie` 0.18.2 | red · exit 1, last line `advisories FAILED, licenses ok` (as on the tree) |
| 9 lock probe | must-pass | `cookie`'s two lines edited | green · last line `3+2` |
| 12 audit | must-pass | `cookie`'s two lines edited | green · exit 0, last line `advisories ok, licenses ok` |
| 14 build | must-pass | `cookie`'s two lines edited | green · exit 0, no `warning:` line (the full leg run: `evidence/cookie-lock-build.log`) |

Each run left the clone's lock at 2+ 2− (`git diff --stat`), so no `--locked` cargo run rewrote it.
Not controlled: a fail side for entry 14's `lacks warning:` atom (no input on hand makes the workspace build warn); the
`exit 0` atom carries the entry's verdict.
