import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()

def rep(old, new, count=1):
    global s
    n = s.count(old)
    if n != count:
        raise SystemExit(f"expected {count} of {old[:70]!r}, found {n}")
    s = s.replace(old, new)

# (1) step 1 removes nothing
rep(s[s.index("1. **Preconditions.**"):s.index("2. **Start the merge")],
    "1. **Preconditions.** The worktree holds no tracked change outside `.andromeda/` and `.claude/session-handoff.md` "
    "(the bookkeeping). The trial worktree `../escher-trial-inside-canvas` (probe `2cddaea0a`, detached, on no branch) "
    "and its build directories `target/trial-inside-canvas-build` and `target/tic` stay in place, on the operator's word "
    "at the P5 review: nothing is removed. The merge does not need them out of the way. A detached worktree holds no "
    "branch the merge moves and no path of this tree, and the `fast` leg's sweep reads `target/debug` alone "
    "(`scripts/target-sweep.py:23`). Then run `git fetch upstream`.\n")

# step 9: the layout readings, leg membership, new legs
rep("   - A red in the coverage leg's floor",
    "   - The layout reason's readings read the same after the Taffy move: `stand_layout`, the `layout` verb's answer and "
    "a kept check's layout step in `stand_replay`, all inside `fast`. A moved reading is re-measured and recorded, never "
    "loosened (architecture · decision/layout-answer-crossing; contract/driver-session/layout).\n"
    "   - Upstream's `flex_br` and `drag_and_drop` join no named leg list. No leg list in `.github/scripts/ci-leg.sh` and no "
    "count in `docs/capabilities.md` is edited (test-plan · decision/joining-file-keeps-its-legs).\n"
    "   - A red in the coverage leg's floor")

# step 12: env sweep of the two new test files and the sink host under both builds
rep("The standing capture checks stay at 0 needles. This keeps security-standing S1 and obs-standing S2–S3.",
    "The two new test files are swept for env reads too (security sidecar `amd-xyjfnhq4`). The standing capture checks "
    "stay at 0 needles: `host_log` and `mcp_log` under the workspace build inside `fast`, and under the package-alone "
    "build at entry 19 (obs-plan · decision/sink-host-read-at-trace). This keeps security-standing S1 and obs-standing "
    "S2–S3.")

# step 14 entry numbers
rep("14. **Operator pass** (entries 17–20 of `## Test Commands`)", "14. **Operator pass** (entries 20–23 of `## Test Commands`)")

# Test Commands prose
rep("The merge-shape probes 1–6 read the staged merge before the operator's commit, and entry 20 reads the committed parents. "
    "The legs follow CI's own list (`.github/workflows/ci.yml`): `build`, `msrv`, `fast`, `doc`, `a11y`, `audit`, `coverage` "
    "and `stand-repeat`. The adapter probe follows the lock move (a11y-plan history, the 2026-10-27 take). The `counter` and "
    "`wasm` jobs are read by CI alone.",
    "The merge-shape probes 1–6 read the staged merge before the operator's commit, and entry 23 reads the committed parents. "
    "The legs follow CI's own list (`.github/workflows/ci.yml`): `build`, `msrv`, `fast`, `doc`, `a11y`, `audit`, `coverage`, "
    "`stand-repeat`, `counter` and `wasm` (architecture · contract/ci-leg-legs/counter; contract/ci-leg-legs/wasm). The "
    "adapter probe follows the lock move (a11y-plan history, the 2026-10-27 take). Entry 19 reads the sink-installing "
    "host under the package-alone build, which `fast`'s workspace build does not.")

# new entries 17-19 before the hygiene entry; renumber operator notes
new_entries = (
    "[[gate]]\n"
    "run = 'bash .github/scripts/ci-leg.sh counter'\n"
    "role = 'build'\n"
    "timeout = 3600\n"
    "new = true\n"
    "note = '17 — the counter example built, locked (CI job counter)'\n\n"
    "[[gate]]\n"
    "run = 'bash .github/scripts/ci-leg.sh wasm'\n"
    "role = 'build'\n"
    "timeout = 3600\n"
    "new = true\n"
    "note = '18 — the wasm32 builds: wasm_hello, seven_guis and todomvc libs (CI job wasm); taffy 0.16.0 and the merged blitz-dom on wasm32'\n\n"
    "[[gate]]\n"
    "run = 'cargo test -p seven_guis --locked --test host_log --test mcp_log'\n"
    "role = 'integration'\n"
    "timeout = 3600\n"
    "new = true\n"
    "note = '19 — the sink-installing host and the mcp role read at trace under the package-alone build, 0 needles (obs-plan · decision/sink-host-read-at-trace); the workspace build reads them inside fast'\n\n"
    "[[gate]]\n"
    "run = 'python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene'\n")
rep("[[gate]]\nrun = 'python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene'\n", new_entries)
rep("note = '17 — fired by the operator pass", "note = '20 — fired by the operator pass")
rep("note = '18 — the push, clean-tree form;", "note = '21 — the push, clean-tree form;")
rep("note = \"19 — after the push;", "note = \"22 — after the push;")
rep("note = '20 — after the pre-CI commit:", "note = '23 — after the pre-CI commit:")
rep("WATCH `cry-womrv2l4` is read off this chunk's CI runs (entry 19).", "WATCH `cry-womrv2l4` is read off this chunk's CI runs (entry 22).")

# acceptance
rep("- (workspace) `bash .github/scripts/ci-leg.sh build`, `msrv`, `fast`, `doc`, `a11y`, `audit` and `coverage` each exit 0.",
    "- (workspace) `bash .github/scripts/ci-leg.sh build`, `msrv`, `fast`, `doc`, `a11y`, `audit`, `coverage`, `counter` and "
    "`wasm` each exit 0, and `cargo test -p seven_guis --locked --test host_log --test mcp_log` exits 0.")

# provenance: the lean replaced by the re-run
rep(s[s.index("- Leaned «no second row read»"):s.index("\n\n## Constraints")],
    "- The row read ran again whole on the corrected scope, on the operator's word at the P5 review "
    "(`.andromeda/runs/2026-11-24T06-26-40-phase/p5-card-1.md`, review 1): T1 A added `layout_reason.rs`, a file the scope "
    "as pinned (tx 3533) did not name. The scope at rev 4 (the founder's answers and the two adaptations named) was pinned "
    "anew. Then 7 tasks, 117 pickers in two parts and the union ran (picked 1380 of 3592: architecture 631, security-plan "
    "217, design-system 53, layout-templates 18, test-plan 170, obs-plan 78, a11y-plan 213), then 7 history agents, every "
    "check passing. The rows it added and the plan folds are (architecture · decision/layout-answer-crossing), "
    "(architecture · contract/driver-session/layout), (architecture · contract/ci-leg-legs/counter), (architecture · "
    "contract/ci-leg-legs/wasm), (architecture · decision/stand-offline), (test-plan · decision/joining-file-keeps-its-legs) "
    "and (obs-plan · decision/sink-host-read-at-trace).")

# constraints
rep("- A probe worktree under `target/`: the trial stood outside it (inputs#I1), and step 1 removes it.",
    "- A probe worktree under `target/`: the trial stood outside it (inputs#I1).\n"
    "- Removing the trial worktree or its build directories: they stay in place on the operator's word at the P5 review. "
    "No step deletes them, and the merge does not need them gone (step 1).")

# implementation notes: added amendments from the new history read
rep("- v040-02 stays pooled.",
    "- Expected amendments (wrap): architecture §Project Intent — the sync record names `29a439e05` as the next sync's merge "
    "base, under (architecture · decision/upstream-sync-merge-base) (architecture history, the 2026-10-06 sync's "
    "`Upstream sync:` line).\n"
    "- Expected amendments (wrap): design-system §Depth — C1 A's one-axis `overflow: clip` paint is recorded as a depth row "
    "beside the hit-stop rows (design-system · decision/hit-test-stops-at-paint-clip), ratified as upstream's "
    "(design-system history, the 2026-11-13 and 2026-11-01 takes' form).\n"
    "- Expected amendments (wrap): layout-templates §Surface: cli — whether the two root examples `examples/dnd.rs` and "
    "`examples/file_drop.rs` are command-line surfaces is read at the wrap; both are windowed examples (layout-templates "
    "history, the 2026-10-24 take's `multi_doc` precedent).\n"
    "- v040-02 stays pooled.")

open(p, "w", encoding="utf-8").write(s)
print("folded")
