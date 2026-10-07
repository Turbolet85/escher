"""P4/P5 — judge against the baseline record and render proposals.md, with the table asserts.

Usage: render.py <run_dir>   (run AFTER the ledger append; the baseline is re-found by baseline_sha, never as the last line)
"""
import json
import os
import re
import sys

rd = sys.argv[1]
L = lambda n: json.load(open(os.path.join(rd, n), encoding='utf-8'))
recs = []
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try:
        recs.append(json.loads(line))
    except ValueError:
        pass
cur = L('record.json')
mine = [i for i, r in enumerate(recs) if r['ts'] == cur['ts'] and r['sha'] == cur['sha']]
assert len(mine) == 1 and mine[0] == len(recs) - 1, 'this run\'s record is not the ledger\'s last line'
assert recs[mine[0]] == cur, 'the appended line differs from record.json'
before = recs[:mine[0]]
base = [r for r in before if r['sha'] == cur['baseline_sha']][-1]
prev = [r for r in before if r['sha'] == base['baseline_sha']][-1]  # the chain by sha, for `monotonic`
assert (prev['epoch'], base['epoch']) == ('Epoch 1 — Foundation', 'Epoch 2 — Element identity')
assert cur['tool_versions'] == base['tool_versions'], 'a tool token moved: trend-break handling needed'

cx, dup, sizes, dead, churn, hot = (L('c-complexity.json'), L('c-duplication.json'), L('c-sizes.json'), L('c-dead.json'),
                                    L('c-churn.json'), L('c-hotspots.json'))
out = []
w = out.append


def table(header, rows, n, key):
    """Row count == n (n from an independent source) and rows unique on the natural key."""
    assert len(rows) == n, (header, len(rows), n)
    assert len({key(r) for r in rows}) == len(rows), ('duplicate key', header)
    w('| ' + ' | '.join(header) + ' |')
    w('|' + '---|' * len(header))
    for r in rows:
        w('| ' + ' | '.join(str(c) for c in r['cells']) + ' |')
    w('')


# ---- judging
TRACKED = [('duplication.pct', lambda r: r['duplication']['pct'], +1),
           ('complexity.over_ceiling', lambda r: r['complexity']['over_ceiling'], +1),
           ('dead.zero_ref_candidates', lambda r: r['dead']['zero_ref_candidates'], +1),
           ('sizes.file_max', lambda r: r['sizes']['file_max'], +1),
           ('sizes.over_800', lambda r: r['sizes']['over_800'], +1),
           ('coverage.line', lambda r: r['coverage']['line'], -1)]
mono = {}
for name, get, sign in TRACKED:
    a, b, c = get(prev), get(base), get(cur)
    assert None not in (a, b, c)
    mono[name] = ((b - a) * sign > 0 and (c - b) * sign > 0, a, b, c)
fired_mono = [k for k, v in mono.items() if v[0]]
assert fired_mono == ['complexity.over_ceiling'], fired_mono

d_pct = cur['duplication']['pct'] - base['duplication']['pct']
dup_up = d_pct >= 0.5 and cur['duplication']['pct'] >= base['duplication']['pct'] * 1.15
oc_b, oc_c = base['complexity']['over_ceiling'], cur['complexity']['over_ceiling']
creep = oc_c >= oc_b + 3 and oc_c >= oc_b * 1.25
dead_growth = cur['dead']['zero_ref_candidates'] >= base['dead']['zero_ref_candidates'] + 5
cov_drop = cur['coverage']['line'] <= base['coverage']['line'] - 2
new_cycle = cur['graph']['cycles'] > base['graph']['cycles']
assert not (dup_up or creep or dead_growth or cov_drop or new_cycle)
UNIT = 'dioxus-native-dom'
mut_scored = UNIT in cur['mutation']['scores']
last_scored = base['mutation']['scores'][UNIT]
mut_drop = mut_scored and cur['mutation']['scores'][UNIT] <= last_scored - 10

proposals = 1 + (1 if mut_drop else 0)
ISO = cur['ts']
h8 = lambda s: s[:8]

w('# Code Audit — escher · %s · %s' % (cur['epoch'], ISO))
w('mode trend · HEAD %s · baseline %s (%s) · span 1' % (h8(cur['sha']), h8(base['sha']), base['epoch']))
w('')
w('Overshoot: 0 commits. HEAD is the epoch boundary itself, the commit that flipped `2026-10-07-change-tracking-and-diff` '
  'to `complete`. The baseline record sat 1 commit past its own boundary with 0 source files in that delta, so nothing of '
  'Epoch 2\'s is carried into this diff.')
w('')
w('No ancestry break and no trend-break: all eight tool version tokens equal the baseline record\'s.')
w('')
w('How far the numbers can be trusted: before use, each summarizer was run on an export of the baseline commit and '
  'reproduced the Epoch 2 record\'s stored values exactly (sizes, duplication with its split and top list, complexity, '
  'churn, hotspots). The dead-code classing cannot be replayed at the baseline (it needs that commit\'s code graph); at '
  'HEAD it reproduces five of the baseline\'s seven class counts exactly and its top-10 list symbol for symbol.')
w('')
w('This record carries one correction to an earlier record: the Epoch 1 record\'s `commands.complexity`, as the Epoch 2 '
  'record corrected it, still held an unresolved `{run_dir}`; it now names that run\'s directory.')
w('')
w('The ledger now holds %d records:' % len(recs))
w('')
table(['Recorded (UTC)', 'Epoch', 'Mode', 'HEAD', 'Source files', 'Rust code lines'],
      [{'k': r['ts'], 'cells': [r['ts'], r['epoch'], r['mode'], h8(r['sha']), r['totals']['files'], r['totals']['loc']]}
       for r in recs], len(recs), lambda r: r['k'])

# ---- proposals
w('## Proposals')
w('')
w('### M1 — monotonic · complexity.over_ceiling — 74 → 77 → 78')
w('')
w('`over_ceiling` counts functions whose cognitive complexity is above 15. It is one of the six tracked scalars, and it '
  'worsened at both of the last two diffs, which is what the `monotonic` check fires on.')
w('')
w('**Movement:** %d → %d (%s → %s), after %d → %d (%s → %s). The single-epoch rule `complexity-creep` did not fire: '
  'it needs +3 and +25%%, and this epoch moved +1.' % (oc_b, oc_c, base['epoch'], cur['epoch'],
                                                       prev['complexity']['over_ceiling'], oc_b, prev['epoch'], base['epoch']))
w('')
w('**Evidence:** the ledger stores only each record\'s top-10, so the functions that crossed the ceiling were found by '
  're-running the same collector (rust-code-analysis 0.0.25) on an export of each record\'s commit. The recomputed counts '
  'equal the stored ones (74, 77, 78). Every crossing over the two diffs:')
w('')
CROSS = [
    ('Epoch 1 → 2', 'in', '`walk`', 'packages/dioxus-native-dom/src/element_id.rs', 'source', '— → 26', 'escher, the stable-element-ids chunk'),
    ('Epoch 1 → 2', 'in', '`inline_fragment_boxes`', 'packages/blitz-dom/src/node/node.rs', 'source', '— → 22', 'upstream merge (Blitz #1060)'),
    ('Epoch 1 → 2', 'out', '`inline_fragment_rects`', 'packages/blitz-dom/src/document.rs', 'source', '29 → 0', 'upstream merge (the same change)'),
    ('Epoch 1 → 2', 'in', '`unkeyed_elements_read_their_component_path`', 'tests/blitz-tests/tests/stand_element_ids.rs', 'test', '— → 21', 'escher, stand check'),
    ('Epoch 1 → 2', 'in', '`ids_hold_across_a_remount`', 'tests/blitz-tests/tests/stand_id_persistence.rs', 'test', '— → 17', 'escher, stand check'),
    ('Epoch 2 → 3', 'in', '`every_node_is_one_line_with_its_five_fields`', 'tests/blitz-tests/tests/stand_snapshot_text.rs', 'test', '— → 16', 'escher, stand check'),
]
ins = {(1, 2): sum(1 for c in CROSS if c[0] == 'Epoch 1 → 2' and c[1] == 'in'), (2, 3): sum(1 for c in CROSS if c[0] == 'Epoch 2 → 3' and c[1] == 'in')}
outs = {(1, 2): sum(1 for c in CROSS if c[0] == 'Epoch 1 → 2' and c[1] == 'out'), (2, 3): 0}
net12 = oc_b - prev['complexity']['over_ceiling']
net23 = oc_c - oc_b
assert ins[(1, 2)] - outs[(1, 2)] == net12 and ins[(2, 3)] - outs[(2, 3)] == net23
# the HEAD entrant and the standing escher functions must be in this run's own over-ceiling list
ocl = {(f[0], f[1].rsplit(':', 1)[0]): f[2] for f in cx['over_ceiling_list']}
assert len(cx['over_ceiling_list']) == oc_c
for c in CROSS:
    if c[1] == 'in':
        assert (c[2].strip('`'), c[3]) in ocl, c
table(['Diff', 'Direction', 'Function', 'File', 'Kind', 'Cognitive', 'Origin'],
      [{'k': (c[0], c[2], c[3]), 'cells': c} for c in CROSS], abs(net12) + 2 * outs[(1, 2)] + abs(net23) + 2 * outs[(2, 3)],
      lambda r: r['k'])
w('Split by path, the count of source functions over the ceiling went 74 → 75 → 75 and the count of test functions '
  '0 → 2 → 3. So this epoch\'s +1 is a test function, and source held level.')
w('')
w('Three source functions already over the ceiling grew this epoch, all in `packages/blitz-dom/src/mutator.rs`, where '
  'the changed-set marks were added: `add_children_to_parent` 17 → 21, `set_attribute` 34 → 35, `clear_attribute` 24 → 25. '
  'They do not move the count.')
w('')
w('The complete list of the %d functions over the ceiling at HEAD is in `c-complexity.json` (`over_ceiling_list`).' % oc_c)
w('')
w('**Suspected shape:** over two epochs the scalar was moved by escher\'s own code (one id-walking source function and '
  'three stand-check test functions); the one upstream change in the table moved an over-ceiling body from '
  '`document.rs` to `node.rs` and nets to zero. The 70-odd inherited engine functions above the ceiling did not change '
  'membership.')
w('')
w('**Proposal:** two directions for the founder\'s judgment. One is to flatten the stand checks that crossed the '
  'ceiling: each of the three wraps its whole body in the `for task` × `for incremental` double loop and loops over nodes '
  'inside it, so moving the per-task, per-mode body into its own function (or one shared driver for that double loop) '
  'would remove the nesting that puts them over. The other is to decide whether test functions belong in this scalar at '
  'all: read as source-only it is 74 → 75 → 75 and would not have fired. Separately, `walk` in `element_id.rs` (26) is '
  'the one escher-authored source function above the ceiling.')
w('')
if mut_drop:
    m = L('c-mutation-%s.json' % UNIT)
    w('### M2 — mutation-drop · %s — %.2f → %.2f' % (UNIT, last_scored, m['score']))
    w('')
    w('**Movement:** %.2f → %.2f (%s → %s), on the same two files (`element_id.rs`, `dioxus_document.rs`).'
      % (last_scored, m['score'], base['epoch'], cur['epoch']))
    w('')
    w('**Evidence:** %d mutants, %d caught, %d missed, %d timeout, %d unviable; score = caught/(caught+missed). '
      'The %d survivors:' % (m['counts']['mutants'], m['counts']['caught'], m['counts']['missed'], m['counts']['timeout'],
                             m['counts']['unviable'], m['counts']['missed']))
    w('')
    table(['Site', 'Mutation'], [{'k': tuple(s), 'cells': ['`%s`' % s[0], '`%s`' % s[1]]} for s in m['survivors']],
          m['counts']['missed'], lambda r: r['k'])
    w('**Suspected shape:** see the survivor sites above; the test set this epoch is the Epoch 2 set plus three stand '
      'checks, so the drop is not a narrower test set.')
    w('')
    w('**Proposal:** add killing tests for the survivors above, or record which of them are equivalent mutants.')
    w('')

# ---- informational
w('## Informational')
w('')
bt = {tuple(x) for x in base['duplication']['top']}
ct = {tuple(x) for x in cur['duplication']['top']}
dup_in = [x for x in cur['duplication']['top'] if tuple(x) not in bt]
dup_out = [x for x in base['duplication']['top'] if tuple(x) not in ct]
rows_all = L('dup-rows.json')
assert all(any(r[0] == x[0] and r[1] == x[1] and r[2] == x[2] for r in rows_all) for x in dup_out)
w('- **Duplication top-10, %d entrants.** `%s` ↔ `%s` (%d lines) and `%s` ↔ `%s` (%d lines) are the per-task control '
  'tables the stand checks each restate. `%s` ↔ `%s` (%d lines) is the 27-name boolean-attribute list, written once in the '
  'bridge and once in its test as the test\'s own oracle. The %d pairs that left the top-10 still exist, at lower rank.'
  % (len(dup_in), dup_in[0][0].split('/')[-1], dup_in[0][1].split('/')[-1], dup_in[0][2],
     dup_in[1][0].split('/')[-1], dup_in[1][1].split('/')[-1], dup_in[1][2],
     dup_in[2][0].split('/')[-1], dup_in[2][1].split('/')[-1], dup_in[2][2], len(dup_out)))
assert len(dup_in) == 3 and 'stand_' in dup_in[0][0] and 'stand_' in dup_in[1][0] and 'mutation_writer' in dup_in[2][0]
stand = [r for r in rows_all if r[0].startswith('tests/blitz-tests/tests/stand_') and r[1].startswith('tests/blitz-tests/tests/stand_')]
w('- **Clones among the stand checks.** Pairs where both sides are a `stand_*.rs` check went from 1 pair (11 lines) at '
  'the baseline to %d pairs (%d lines). The working route proves Epoch 4\'s driver entries on the same stand tasks.'
  % (len(stand), sum(r[2] for r in stand)))
assert (len(stand), sum(r[2] for r in stand)) == (13, 220)
bh = {x[0] for x in base['hotspots']}
ch = {x[0] for x in cur['hotspots']}
h_in = [x for x in cur['hotspots'] if x[0] not in bh]
w('- **Hotspots top-10, %d entrants:** %s. Hotspot score is commits touching the file in the window times the file\'s '
  'highest cognitive complexity. The window held no upstream merge this epoch, so the engine files that led the '
  'baseline\'s list (`layout/inline.rs` 387, `node/node.rs` 189) left it, and the list now shows where the epoch\'s own '
  '6 source-touching commits landed. The highest score is 50, against 387 at the baseline.'
  % (len(h_in), ', '.join('`%s` (%s)' % (x[0].split('/', 1)[1] if x[0].startswith('packages/') else x[0], x[1]) for x in h_in)))
assert cur['hotspots'][0][1] == 50 and base['hotspots'][0][1] == 387 and churn['commits_in_range'] == 6
for label, c_, b_, key in (('complexity top-10', cur['complexity']['top'], base['complexity']['top'], lambda x: (x[0], x[1].rsplit(':', 1)[0])),
                           ('sizes top-10', cur['sizes']['top'], base['sizes']['top'], lambda x: x[0]),
                           ('fan-in top-20', cur['graph']['fan_in_top'], base['graph']['fan_in_top'], lambda x: x[0]),
                           ('dead-code top-10', cur['dead']['top'], base['dead']['top'], lambda x: x[0])):
    assert {key(x) for x in c_} == {key(x) for x in b_}, label
w('- **No entrants** in the complexity top-10, the sizes top-10, the fan-in top-20 or the dead-code candidate top-10.')
w('- **Churn %.2f%% → %.2f%%.** Churn is the share of added lines that landed in a file\'s second or later commit inside '
  'the window. %d of %d added lines were churn, in %d files; %d of them are `snapshot.rs`, which three commits wrote in turn.'
  % (base['churn']['pct'], churn['pct'], churn['adds_churned'], churn['adds_all'], churn['files_churned'], churn['top'][0][1]))
assert churn['top'][0][0].endswith('snapshot.rs')
if not mut_scored:
    w('- **Mutation score: none this epoch.** The one scoped unit ran out of its 15-minute budget (see Skips), so '
      '`mutation-drop` has nothing to read. Its last scored value stays %.2f, from %s.' % (last_scored, base['epoch']))
else:
    m = L('c-mutation-%s.json' % UNIT)
    w('- **Mutation score %.2f → %.2f on `%s`** (`element_id.rs` and `dioxus_document.rs`, the same two files as the '
      'baseline): %d mutants, %d caught, %d missed, %d timeout, %d unviable. The test set is the Epoch 2 set plus three '
      'stand checks this epoch added, so the two scores are not from an identical test set.'
      % (last_scored, m['score'], UNIT, m['counts']['mutants'], m['counts']['caught'], m['counts']['missed'],
         m['counts']['timeout'], m['counts']['unviable']))
    if not mut_drop and m['counts']['missed']:
        w('')
        w('  The %d survivors (a survivor is a mutant no test caught):' % m['counts']['missed'])
        w('')
        table(['Site', 'Mutation'], [{'k': tuple(s), 'cells': ['`%s`' % s[0], '`%s`' % s[1]]} for s in m['survivors']],
              m['counts']['missed'], lambda r: r['k'])
        w('  Not measured on this host (%s): %d. No project union verdict is registered in the test plan.'
          % (m['host'], m['counts']['not_measured']))
w('')

# ---- below threshold
w('## Below threshold — no action')
w('')
bd, cd = base['duplication'], cur['duplication']
w('- **Duplication %.3f%% → %.3f%%** (+%.3f points, +%.1f%% relative; the rule needs +0.5 points and +15%%). Clones %d → %d, '
  'duplicated lines %d → %d, scanned lines %d → %d. By path: source pairs %d → %d (lines %d → %d), test pairs %d → %d '
  '(lines %d → %d), mixed pairs %d → %d (lines %d → %d). The ratio rose with the counts, so this is not the '
  'count-under-ratio case.'
  % (bd['pct'], cd['pct'], d_pct, 100 * d_pct / bd['pct'], bd['clones'], cd['clones'], bd['duplicated_lines'],
     cd['duplicated_lines'], bd['total_lines'], cd['total_lines'],
     bd['split']['src']['pairs'], cd['split']['src']['pairs'], bd['split']['src']['lines'], cd['split']['src']['lines'],
     bd['split']['test']['pairs'], cd['split']['test']['pairs'], bd['split']['test']['lines'], cd['split']['test']['lines'],
     bd['split']['mixed']['pairs'], cd['split']['mixed']['pairs'], bd['split']['mixed']['lines'], cd['split']['mixed']['lines']))
w('- **Complexity, single-epoch rule:** over the ceiling %d → %d (+1). Percentiles unchanged: cyclomatic p50 1, p90 6; '
  'cognitive p50 0, p90 3. The maximum is unchanged (`%s`, %d).' % (oc_b, oc_c, cur['complexity']['max']['fn'], cur['complexity']['max']['val']))
assert all(cur['complexity'][k] == base['complexity'][k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90', 'max'))
w('- **Largest file %d → %d lines** (`packages/blitz-dom/src/document.rs`, +254 / −10 this epoch, the changed set and its '
  'seven in-file unit tests). Files over 800 lines: %d → %d. File size p50 %d → %d, p90 %d → %d.'
  % (base['sizes']['file_max'], cur['sizes']['file_max'], base['sizes']['over_800'], cur['sizes']['over_800'],
     base['sizes']['file_p50'], cur['sizes']['file_p50'], base['sizes']['file_p90'], cur['sizes']['file_p90']))
assert cur['sizes']['top'][0][0] == 'packages/blitz-dom/src/document.rs'
w('- **Dead-code candidates %d → %d.** %d of them sit in files this epoch touched; none was added this epoch (all are '
  'inherited `BaseDocument` and `View` methods). Unused dependencies %d → %d, the same list. These are candidates, never '
  'proven dead: entry points, trait methods reached by dispatch, derive-invoked functions, macro-generated bindings, '
  'test-only helpers and format-string captures are classed out first.'
  % (base['dead']['zero_ref_candidates'], cur['dead']['zero_ref_candidates'], len(dead['candidates_in_touched_files']),
     len(base['dead']['unused_deps']), len(cur['dead']['unused_deps'])))
assert cur['dead']['unused_deps'] == base['dead']['unused_deps']
w('- **Line coverage %.2f%% → %.2f%%.** A rising number is not evidence of better tests; only a fall is a signal. Branch '
  'coverage is not instrumented.' % (base['coverage']['line'], cur['coverage']['line']))
w('- **Dependency graph:** cycles %d → %d, cross-crate edges %d → %d, per-crate fan-out identical.'
  % (base['graph']['cycles'], cur['graph']['cycles'], base['graph']['cross_unit_edges'], cur['graph']['cross_unit_edges']))
assert sorted(map(tuple, cur['graph']['fan_out'])) == sorted(map(tuple, base['graph']['fan_out']))
w('- **Populations (never a movement):** Rust code lines %d → %d, source files %d → %d, crates %d → %d.'
  % (base['totals']['loc'], cur['totals']['loc'], base['totals']['files'], cur['totals']['files'],
     base['totals']['units'], cur['totals']['units']))
w('')
w('The six tracked scalars across the three records. All six were measured at every record, so all were evaluable; '
  'only the second row worsened at both diffs.')
w('')
table(['Tracked scalar', prev['epoch'], base['epoch'], cur['epoch'], 'Worsened at both diffs'],
      [{'k': k, 'cells': ['`%s`' % k, v[1], v[2], v[3], 'yes' if v[0] else 'no']} for k, v in mono.items()],
      6, lambda r: r['k'])

# ---- skips
w('## Skips')
w('')
skips = cur['skips']
for s in skips:
    if s['reason'] == 'budget-exhausted':
        st = L('mutation-stopped.json')
        w('- `%s` — budget-exhausted. The invocation was stopped at its 15-minute cap with %d of %d mutants tested, so '
          'by rule it has no score, no counts and no survivor list in the record. It was not re-run, on the operator\'s '
          'word at the confirm. Two measured facts for sizing the next run: the rate was %d mutants in 15 minutes at '
          '`-j 4`, cold builds included; and each mutant\'s build compiled every `blitz-tests` test target, not only the '
          'seven named ones, because the `--cargo-test-arg` filters reach the test phase only.'
          % (s['metric'], st['tested_when_stopped']['total_mutants'], st['planned'], st['tested_when_stopped']['total_mutants']))
    else:
        w('- `%s` — %s at this run\'s confirm.' % (s['metric'], s['reason']))
w('')
assert len([l for l in out if l.startswith('- `mutation:')]) == len(skips)
open(os.path.join(rd, 'proposals.md'), 'w', encoding='utf-8').write('\n'.join(out) + '\n')
print('proposals: %d · informational: %d · skips: %d' % (
    proposals, sum(1 for l in out[out.index('## Informational'):out.index('## Below threshold — no action')] if l.startswith('- **')),
    len(skips)))
