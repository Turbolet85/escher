# Curation — 2026-10-05-ci-gate-legs (wrap 2026-10-05T23-50-17)

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "The project's Bash guards refuse a heredoc written to a file and a leading cd, and read payload prose too"
  Filters: 4 dup · 1 task-specific · 0 conflict · 0 deferred · 2 below threshold (exactly 0.6)
  CLAUDE.md size: 126/200 · T1 0.4 KB, 0 over 600 B
```

## Applied
- **T3 — the project's Bash guards** (confidence 0.9: repeated pattern +0.3 — three separate events · verified by
  measurement +0.4 — three real refusals · specific technical detail +0.2).
  Proof: implement P1 — a python heredoc writing `edit_tests.py` to the scratchpad was refused ("Blocked: a cat/tee
  heredoc with a file target"); implement's code-step evolve append was refused because a friction record's `what`
  quoted that shell form (the same append re-sent with the prose reworded landed, friction-log lines 55→58); this
  wrap's P1 per-master sweep was refused for a leading `cd .andromeda` ("Blocked: a leading cd … moves the session
  cwd"), re-run as a scratch script over absolute paths.

## Rejected
- `cargo llvm-cov report` needs `--workspace` here (exit 0, empty table without it) — 0.6 (measured +0.4, detail
  +0.2): exactly the threshold → reject; the trap rides the test-plan sidecar entry's Why and the body states the
  working form.
- cargo-llvm-cov does not create `--output-path`'s parent — 0.6 → reject (fixed in the leg script itself).
- cargo-deny's resolved graph prunes optional chains (paste / memmap2 unseen) — dup: amended into security-plan
  §Dependency Security and re-derived into `.claude/rules/security.md`'s body this wrap.
- a SHA-pinned `dtolnay/rust-toolchain` needs an explicit `toolchain` input — dup: architecture body + the
  `test_k` invariant this chunk wrote.
- `cargo doc --workspace` needs `libfontconfig1-dev` (yeslogic-fontconfig-sys build script) — dup: CLAUDE.md's
  key-commands line ("Linux needs `libfontconfig1-dev`") and arch §CI/CD ("Ubuntu jobs install `libfontconfig1-dev`").
- the lockfile re-key evicts the whole Actions cache — dup: architecture §Occupied Resources → CI infrastructure
  (amended this wrap).
- a per-test tally over unittest output must anchor the full test name (`test_d` also matches `test_doc_leg`) —
  task-specific (one-off, a fixture's names).

## Corpus used, not added
The 2026-10-05 Tier-3 entry "A chunk that moves cited source lines stales the masters' file:line citations" was
followed as written this wrap (re-point by a measured line map, before the semantic amendments): 70 citations on
28 lines, no double shift.
