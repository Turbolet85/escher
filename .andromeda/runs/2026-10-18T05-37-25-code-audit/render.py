"""P4 judge + P5 render: proposals.md from snapshot.json and the baseline record (re-found by sha).

usage: render.py {run_dir}
Every evidence table's row count is asserted against an independent n, rows unique on their key.
"""
import json
import os
import sys

rd = sys.argv[1]
BASE = '143b6013229d785f4613895e16c1c82b1c1a5be2'
s = json.load(open(os.path.join(rd, 'snapshot.json'), encoding='utf-8'))
recs = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8') if l.strip()]
b = [r for r in recs if r['sha'] == BASE][-1]
bb = [r for r in recs if r['sha'] == b['baseline_sha']][-1]
dup_all = json.load(open(os.path.join(rd, 'c-duplication.json'), encoding='utf-8'))
churn = json.load(open(os.path.join(rd, 'c-churn.json'), encoding='utf-8'))


def table(hdr, rows, n, key):
    assert len(rows) == n, (hdr, len(rows), n)
    assert len({key(r) for r in rows}) == len(rows), ('duplicate rows', hdr)
    out = ['| ' + ' | '.join(hdr) + ' |', '|' + '---|' * len(hdr)]
    out += ['| ' + ' | '.join(str(c) for c in r) + ' |' for r in rows]
    return '\n'.join(out)


# ---- judging (span 1; single-epoch thresholds live) ----
P, I, B = [], [], []
d_pct = s['duplication']['pct'] - b['duplication']['pct']
d_rel = d_pct / b['duplication']['pct'] * 100
dup_fire = d_pct >= 0.5 and d_rel >= 15
oc = s['complexity']['over_ceiling'] - b['complexity']['over_ceiling']
oc_fire = oc >= 3 and oc / b['complexity']['over_ceiling'] * 100 >= 25
dead_fire = s['dead']['zero_ref_candidates'] - b['dead']['zero_ref_candidates'] >= 5
cov_fire = s['coverage']['line'] <= b['coverage']['line'] - 2
cyc_fire = s['graph']['cycles'] > b['graph']['cycles']
TRACK = [('duplication.pct', lambda r: r['duplication']['pct'], 1),
         ('complexity.over_ceiling', lambda r: r['complexity']['over_ceiling'], 1),
         ('dead.zero_ref_candidates', lambda r: r['dead']['zero_ref_candidates'], 1),
         ('sizes.file_max', lambda r: r['sizes']['file_max'], 1),
         ('sizes.over_800', lambda r: r['sizes']['over_800'], 1),
         ('coverage.line', lambda r: r['coverage']['line'], -1)]
mono = []
for name, f, sign in TRACK:
    a, m, c = f(bb), f(b), f(s)
    worse = (m - a) * sign > 0 and (c - m) * sign > 0
    mono.append((name, a, m, c, worse))
assert not any([dup_fire, oc_fire, dead_fire, cov_fire, cyc_fire] + [w for *_, w in mono])

hdr = f"""# Code Audit — escher · Epoch 2 — Engine debts · {s['ts']}
mode ad-hoc · HEAD {s['sha'][:9]} · baseline {BASE[:9]} (`{b['epoch']}`, trend record of {b['ts']}) · span 1
ad-hoc: epoch-in-progress — `2026-10-18-engine-debts` reads `gated` in the master record (waiting on the founder's word on the PROVISIONAL batch); snapshot saved to `snapshot.json`, NOT appended to the ledger
overshoot: none defined — the epoch has no complete flip yet; the baseline record sits at its own boundary (`commits: 0`); its window `{BASE[:9]}..HEAD` holds 15 commits (10 carry source changes; the merge commit 34bcf71c6 lists with no diff), two chunks: the upstream sync (merge of upstream/main at 23b9b26a9) and the engine-debts work, which landed in the pre-CI commit 48680dfb9 (the chunk commit fb8426711 touches no `.rs`)
no ancestry break · no trend-break: every tool's leading version token equals the baseline's (jscpd 5.4.0 · tokei 14.0.0 · rust-code-analysis 0.0.25 · code-graph.py f22becbb · cargo-machete 0.9.2 · cargo-llvm-cov 0.9.1 · rustc 1.99.0)
population check: the summarizers reproduced the baseline record's sizes, totals, complexity and duplication exactly on a git archive of {BASE[:9]} (0 mismatches); the dead-code recipe is the baseline record's `recipes.dead`, verbatim
"""

body = ['## Proposals', '', 'None — no check in the threshold table fires at span 1 (new-cycle · duplication-up · '
        'complexity-creep · dead-growth · coverage-drop · monotonic); mutation-drop is not evaluable this run (the '
        'tier was declined).', '']

# ---- informational ----
hot_old = {r[0] for r in b['hotspots']}
hot_rows = [[i + 1, f'`{f}`', sc, churn['commits_per_file'][f], 'entrant' if f not in hot_old else 'standing']
            for i, (f, sc) in enumerate(s['hotspots'])]
inf = ['## Informational', '',
       f"- **top-N entrants — hotspots** (B2, score = commits in the window × max cognitive in the file): "
       f"{sum(1 for r in hot_rows if r[4] == 'entrant')} of 10 rows are entrants; the window moved from Epoch 1's "
       f"splits to the upstream sync and the engine debts, so the list is this window's, not a decay signal by itself.",
       '',
       table(['#', 'file', 'score', 'commits', 'vs baseline top-10'], hot_rows, min(10, len(s['hotspots'])),
             lambda r: r[1]),
       '',
       f"- **top-N order — complexity max** moved: `compute_inline_layout_inner` "
       f"(`packages/blitz-dom/src/layout/inline.rs:230`) cognitive 131 → 139, now above "
       f"`synthesize_presentational_hints_for_legacy_attributes` (135). The +8 arrived with upstream 8d35e562c "
       f"\"Honor baseline-source when aligning atomic inlines\" (+28/−20 in inline.rs). `hit_inner` 70 → 74 "
       f"(`node/node.rs`, the engine-debts hit fixes). The top-10 members are unchanged; there are no entrants.",
       f"- **new file — `packages/dioxus-native-dom/src/keyboard_compat.rs`**, 556 code lines: the epoch's largest "
       f"single growth, from upstream 3f483ff2a (\"compat code in dioxus-native-dom temporarily\"). The handoff "
       f"names it, and the working route carries it as a CARRY on «Upstream sync ahead of determinism and traces», Epoch 3's head.",
       f"- **population** (never a movement): totals.loc {b['totals']['loc']} → {s['totals']['loc']}, files "
       f"{b['totals']['files']} → {s['totals']['files']} (+8 new, −1 `apps/bump/src/main.rs`), units "
       f"{b['totals']['units']} → {s['totals']['units']} (the `bump` crate deleted upstream, 23b9b26a9); jscpd "
       f"total_lines {b['duplication']['total_lines']} → {s['duplication']['total_lines']}.",
       '']

# ---- below threshold ----
sp, bsp = s['duplication']['split'], b['duplication']['split']
over_new = [['`build_accessibility_tree`', '`packages/blitz-dom/src/accessibility.rs`', 36.0],
            ['`build_accessibility_node`', '`packages/blitz-dom/src/accessibility.rs`', 27.0]]
mono_lines = '; '.join(f"{n} {a} → {m} → {c}" for n, a, m, c, _ in mono)
bel = ['## Below threshold — no action', '',
       f"- **duplication-up** — pct {b['duplication']['pct']} → {s['duplication']['pct']} "
       f"(+{d_pct:.3f} pt, +{d_rel:.1f}% relative; the floor is +0.5 pt AND +15%). Clones "
       f"{b['duplication']['clones']} → {s['duplication']['clones']}, duplicated lines "
       f"{b['duplication']['duplicated_lines']} → {s['duplication']['duplicated_lines']}. By split: test pairs "
       f"{bsp['test']['pairs']} → {sp['test']['pairs']} (lines {bsp['test']['lines']} → {sp['test']['lines']}); "
       f"src {bsp['src']['pairs']} → {sp['src']['pairs']} and mixed {bsp['mixed']['pairs']} → {sp['mixed']['pairs']} "
       f"unchanged. At the fragment level, 43 rows are new and 17 gone, all under `tests/blitz-tests/tests/`. The new "
       f"ones concentrate in the new `accessibility_*` tests: active_descendant_unread 16, label 12, description 11, "
       f"merged_states 6. The top-10 has no entrants. count-under-ratio does not apply: the ratio rose with the count.",
       f"- **complexity-creep** — over_ceiling {b['complexity']['over_ceiling']} → {s['complexity']['over_ceiling']} "
       f"(+{oc}; the floor is +3 AND +25%). Both functions that crossed cognitive 15 are in "
       f"`accessibility.rs`, which grew 267 → 437 code lines: upstream's ARIA merges fabd70e78 · 8114cd505 · "
       f"dd6f46b55 (+180/−7) and the engine-debts pre-CI commit 48680dfb9 (+49/−11). None left. Percentiles "
       f"unchanged (cyclomatic p50/p90 1/5, cognitive 0/3).",
       '',
       table(['function', 'file', 'cognitive'], over_new, s['complexity']['over_ceiling'] - b['complexity']['over_ceiling'],
             lambda r: (r[0], r[1])),
       '',
       f"- **dead-growth** — zero-ref candidates {b['dead']['zero_ref_candidates']} → "
       f"{s['dead']['zero_ref_candidates']} (+0); top-10 identical. FP classes unchanged except entry-point 52 → 50 "
       f"(the deleted `bump` crate). Zero-ref rows {b['dead']['zero_ref_rows']} → {s['dead']['zero_ref_rows']}; "
       f"test-path exclusions {b['dead']['excluded_test_path']} → {s['dead']['excluded_test_path']}. Unused deps: the "
       f"same {len(s['dead']['unused_deps'])} rows.",
       f"- **coverage-drop** — line {b['coverage']['line']} → {s['coverage']['line']} (+"
       f"{s['coverage']['line'] - b['coverage']['line']:.2f}; the leg's per-crate floors held 3 of 3). A rising "
       f"number is trend-only and is not credited; branch is null, as in the baseline.",
       f"- **new-cycle** — cycles 0 → 0; cross-unit edges 97 → 97; fan-out per unit unchanged; the fan-in top-20 has "
       f"the same members (`NodeId#` 668 → 678, `get_node()` 330 → 337).",
       f"- **sizes** — file_max {b['sizes']['file_max']} → {s['sizes']['file_max']} "
       f"(`packages/blitz-dom/src/document.rs`; +107/−0 in 48680dfb9, the engine-debts code). over_800 "
       f"{b['sizes']['over_800']} → {s['sizes']['over_800']}: `packages/blitz-dom/src/events/pointer.rs` 747 → 826 "
       f"(+93/−6 in 48680dfb9). p50 {b['sizes']['file_p50']} → {s['sizes']['file_p50']}, p90 "
       f"{b['sizes']['file_p90']} → {s['sizes']['file_p90']}; the top-10 has no entrants.",
       f"- **monotonic** (the six tracked scalars, chain {bb['sha'][:9]} → {BASE[:9]} → {s['sha'][:9]}): none "
       f"worsened at both diffs — {mono_lines}.",
       f"- **churn** — {s['churn']['pct']}% ({churn['churned_adds']} of {churn['adds']} source adds), files churned "
       f"{b['churn']['files_churned']} → {s['churn']['files_churned']}: `accessibility.rs` 4 commits, "
       f"`layout/inline.rs` 3, `events/keyboard.rs` 2, `tests/blitz-tests/tests/all.rs` 2. The baseline window read "
       f"{b['churn']['pct']}%. This window is mostly upstream merge traffic (9 of its 10 source-changing commits; the tenth is 48680dfb9).",
       '']

sk = ['## Skips', '']
for x in s['skips']:
    sk.append(f"- {x['metric']} — {x['reason']} (the founder stopped mutation runs until his word, and he is away; "
              f"the operator declined the tier at the epoch dialogue)")
sk.append('')
sk.append("The touched units (`git diff --name-only 143b60132..HEAD`, mapped by manifest directory; the root "
          "Cargo.toml/Cargo.lock and the deleted `apps/bump` mark `blitz-examples`) were listed for the tier and "
          "none ran. So no unit has a score at this sha, and mutation-drop is not evaluable.")
assert len(s['skips']) == 8
doc = hdr + '\n' + '\n'.join(body + inf + bel + sk) + '\n'
open(os.path.join(rd, 'proposals.md'), 'w', encoding='utf-8').write(doc)
print('proposals 0 · informational', 4, '· below', 8, '· skips', len(s['skips']))
