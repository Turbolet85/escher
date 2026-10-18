# Founder forks — 2026-10-18-upstream-sync-determinism-and-traces (phase P4)

The founder is away. On the operator's directive at this take-up (`inputs#I1`), each fork that is his is written here
whole, with its dependences and a marked recommendation. The operator answers in his stead, and each answer is
recorded as the delegate's — `delegate the operator, 2026-10-18` (`gate-contract.md` §Scope, *Whose word*) —
PROVISIONAL until the founder's own word, discharged in the epoch-boundary batch (playbook, «Provisional discharge»).

One fork stands. Two things that could look like his are not asked; the reasons are at the end.

---

## F1 — The take at this sync: Taffy main, upstream's cursor fix, or nothing

**What is on the table** (measured at the take-up; `inputs#I2`, `inputs#I3`, `inputs#I4`):
- `upstream/main` reads `fb8d6c15e`, 5 commits past our merge base `23b9b26a9`:
  - four Taffy-main commits (`1a11761bc`, `4a5ab9cc9`, `510b592ab`, `1e5479ae6`). They move the root manifest's
    `taffy` from the registry `"0.15"` to `git = "https://github.com/DioxusLabs/taffy", rev = "ea07c4c3…"`, move the
    lock's `taffy` source, move one `cssparser-macros` dependency line (`syn 3.0.6` → `syn 2.0.119`), and add one
    argument to `place_floated_box` in `packages/blitz-dom/src/layout/inline.rs`. These are the commits the last sync
    left out (CARRY 1);
  - one behaviour commit, `fb8d6c15e` (#1220): `get_cursor` returns `Default` where it returned `None` when the
    pointer is over no box. Windowed, `None` HID the cursor (`packages/blitz-shell/src/lib.rs:101-110`); after the fix
    it shows a default arrow. Headless it changes nothing: the default `ShellProvider::set_cursor` is a no-op, and no
    escher reader (snapshot, driver, harness, stand) reads a cursor (research.md §Graph impact, §Sweeps).
- `fb8d6c15e`'s parent is `1e5479ae6`, so a merge of upstream's head takes the four Taffy commits with it.
- Taffy `ea07c4c3` is the 0.15.0 release plus 14 unreleased commits: float placement (#1303 / #1304 / #1305), grid
  (#1320 / #1322), baselines (#1323) and five flexbox changes (#1327 / #1328 / #1333 / #1332 / #1335). No Taffy
  release past 0.15.0 exists on crates.io. Upstream moved its git rev three times in two days, after it had once
  reverted to 0.15 (`85340347f`). The stand floats nothing, and it lays out with flexbox in 27 places.
- All three takes merge clean in memory, with no conflicted file. Taffy main declares the same MSRV, 1.71, as 0.15.0.

**The rule A would reverse**: "no git dependency is left in the root manifest … keep both"
(`.claude/rules/security.md` §Dependencies; security-plan §Dependency Security → Pinning; architecture §Established
Decisions → [Dependency pinning] and the Layout row; CLAUDE.md's coupled-pins warning). The founder confirmed it at
2026-10-13-upstream-sync-polish-and-ship («Подтвердить (Рекомендую)»), and the last sync kept it by leaving out these
same four commits.

### Options

**B — Take `fb8d6c15e` alone, as a cherry-pick; Taffy stays on the registry `0.15` (Recommended)**
- What lands: upstream's two `document.rs` hunks exactly as written (the `get_cursor` fallback and one
  `hover_state_tests` test), applied onto the branch as one non-merge change. Nothing else moves: no manifest, lock or
  `inline.rs` change.
- Why it wins: it takes the one behaviour fix upstream shipped (the windowed cursor no longer vanishes below the
  content) and keeps the founder's rule untouched. It also keeps the layout engine fixed at the start of «Determinism
  and traces», the epoch whose work rests on stable layout readings.
- Dependences:
  - The merge base stays `23b9b26a9`. The `Upstream sync:` line records upstream read at `fb8d6c15e`, the cherry-pick
    and its sha, and the four Taffy commits waiting again. The next sync's merge of upstream will meet the identical
    hunk on both sides and resolve it without a conflict (a 3-way merge of identical changes).
  - CARRY 1 (the Taffy commits) moves on to the next upstream-sync entry. CARRY 2 (`keyboard_compat.rs`, still listed
    upstream) and CARRY 3 (the engine-debts merge surface, no PR) move on too, whatever the answer.
  - Design-system §Iconography's cursor line is restated: over no box, the cursor reads `Default`. Its `document.rs`
    citations are re-pointed. blitz-dom's lib test line reads +1 passed.
  - Not a "merge" in git's sense. The sync is recorded as a partial take by cherry-pick, a first for this fork. Our
    edits to upstream-owned code stay zero (upstream's own text is applied).
- Trade-off: the fork stays 4 commits behind upstream on Taffy. Upstream's Taffy-main layout fixes (flexbox negative
  margins, baselines) do not reach us until Taffy releases or the founder lifts the rule.

**A — Merge `upstream/main` (`fb8d6c15e`) whole, a git Taffy back in the root manifest**
- What lands: all five commits as one merge commit with two parents, the shape of the earlier syncs. Taffy is pinned by
  git rev `ea07c4c3`, the lock records a `git+https` source (no crates.io checksum), and `inline.rs` takes the new
  argument.
- Why one would choose it: the fork reconverges with upstream completely (0 behind). It takes Taffy main's float,
  grid, baseline and flexbox fixes, and the next sync starts from upstream's head.
- Dependences:
  - It reverses the founder's confirmed no-git-dependency rule. The delegate's answer makes that reversal PROVISIONAL
    in four places, at the wrap: architecture [Dependency pinning] and the Layout row, security-plan Pinning, the rules
    file's Dependencies section, and CLAUDE.md's coupled-pins warning. The founder discharges or reworks it at the
    epoch boundary. If he reworks it, the next chunk reverts to the registry pin.
  - The audit leg cannot see a git source (`deny.toml` has no `[sources]` table), so nothing but the rule guards it.
    The plan's "no git dependency" gate inverts to "exactly this git rev".
  - Predicted, measured only by doing it: Taffy's five flexbox changes and its baseline change can move stand bounds,
    snapshot byte sizes, screenshot pixels and the contrast probes' landing points. A moved number is re-measured and
    recorded, never loosened. This is the risk at the head of the determinism epoch.
  - Builds fetch Taffy from GitHub at a fixed rev, in CI and on a cold host. `--locked` still holds.
  - CARRY 1 is discharged. CARRY 2 and CARRY 3 move on.
- Trade-off: complete upstream parity, at the price of the founder's rule (provisionally) and a moving layout engine
  just as determinism work begins.

**C — Take nothing: a measured no-op sync**
- What lands: no code. The chunk records upstream read at `fb8d6c15e`, the five commits left for the next sync, and
  its readings (CI, audit and sink on the unchanged tree), as three 0.1.0 syncs did ("0 ahead: measured no-op, no
  merge").
- Why one would choose it: zero risk and the cheapest chunk. The cursor bug is windowed-only and invisible to every
  headless check.
- Dependences: all three CARRYs move on to the next sync's entry, `fb8d6c15e` among the commits waiting. The
  `Upstream sync:` line notes the read. No master moves beyond that line.
- Trade-off: it leaves a shipped upstream fix untaken for a whole epoch, for no gain over B but one fewer test.

**Recommendation: B.** It is the only option that takes upstream's behaviour fix without spending the founder's rule.
It also leaves Taffy fixed through the epoch that measures determinism. A is the right call once Taffy releases a
registry version carrying these commits: then the coupled set moves with no rule broken. C only gives up B's fix.

---

## Asked of no one — leaned on the record

- **`fb8d6c15e` as "an engine change for every document."** It changes what blitz-dom does for every document, but the
  change is upstream's own, taken as upstream wrote it. The playbook's «An engine behaviour change for every document»
  pattern is about a chunk changing the inherited engine itself, with its merge surface owed to a CARRY. Every sync so
  far took upstream's behaviour changes without a founder fork: `autofocus`'s `!= "false"` at 2026-10-10, the default
  link rule's narrowing, the 28-name falsy predicate at 2026-10-13. Only boundary widenings went to him. Leaned: not
  his fork.
- **A boundary widening by the merge.** None: the delta adds no read of page input, no exit for one, and no carrier
  (0 of 16 added Rust lines read, print, log or bind; the cursor icon goes to the window system, a closed enum the
  engine already sent). The playbook's «Boundary widening by upstream merge» does not fire. Leaned: none to ask.

If the operator reads either lean otherwise, it becomes F2 here, asked the same way.
