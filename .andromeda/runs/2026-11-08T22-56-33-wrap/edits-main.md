@fact id=fct-egnkfdyv expect_rev=3
old: · since 2026-10-19-recorded-check-replay `stand_replay` 3, not unix-gated (each committed check
new: · since 2026-10-19-recorded-check-replay `stand_replay` 4 (3 until 2026-11-08-handler-coverage), not unix-gated (a recorded Counter session with two `actions` steps replaying on a fresh Counter, all 4 steps run, and its copy with one `keyed` flipped stopping at step 3, keys exactly `nodes`, in both layout modes; each committed check
@fact id=fct-76fzwvj7 expect_rev=3
old: and `mcp role: mcp [--record <dir>]`; nothing the caller typed
new: and `mcp role: mcp [--record <dir>]` and, since 2026-11-08-handler-coverage, `coverage role: coverage <app> <check>...`; nothing the caller typed
old: and `3` on a session error (was `1`).
new: and `3` on a session error (was `1`); the `coverage` role — `coverage <app> <check>...`, no `--session` or `--record` word among its paths, a ratified boundary widening — reads each named kept check bounded and through `Check::from_bytes`: accepted, the coverage line on stdout, status `0`; a file it cannot read, one past the bound, one that is no check or a check of another app refused `malformed`, `1`, holding nothing of the path or the file; any other shape usage, `2`; a failing boot its session error, `3`.
@fact id=fct-adkwyvky expect_rev=4
old: is read by `host_log`'s fourth test, which runs `serve cells`
new: is read by `host_log`'s fifth test, which runs `serve cells`
@fact id=fct-prdavblg expect_rev=2
old: and `host_log`'s fourth test reads a paged part
new: and `host_log`'s fifth test reads a paged part
