"""Reading aid for the diff pass: this run's twins against the baseline record (found by sha, never as the last line)."""
import json
import os

RUN = '.andromeda/runs/2026-10-09T14-20-43-code-audit'
BASE = 'b7e3a43b8540343b8e655dfc700640511ef2b707'
recs = []
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try:
        recs.append(json.loads(line))
    except ValueError:
        pass
b = [r for r in recs if r['sha'] == BASE][-1]
c = lambda n: json.load(open(os.path.join(RUN, f'c-{n}.json'), encoding='utf-8'))

d, bd = c('duplication'), b['duplication']
print('== DUP top (this run); * = pair+lines absent from the baseline top')
bt = {tuple(x) for x in bd['top']}
for row, frag in zip(d['top'], d['top_fragments']):
    print('  ', '*' if tuple(row) not in bt else ' ', row, frag['class'], frag['a_start'], frag['b_start'])
print('   baseline rows gone from top:', [x for x in bd['top'] if tuple(x) not in {tuple(r) for r in d['top']}])
print('   split', bd['split'], '->', d['split'])

x, bx = c('complexity'), b['complexity']
print('== CPLX top; * = entrant')
bn = {(t[0], t[1].rsplit(':', 1)[0]) for t in bx['top']}
for t in x['top']:
    print('  ', '*' if (t[0], t[1].rsplit(':', 1)[0]) not in bn else ' ', t)
print('   over-ceiling by file:')
byf = {}
for fn, loc, v in x['over_ceiling_list']:
    byf.setdefault(loc.rsplit(':', 1)[0], []).append((fn, v))
for f in sorted(byf, key=lambda f: -len(byf[f])):
    print('     ', len(byf[f]), f, [f'{n}={int(v)}' for n, v in byf[f]][:8])

s, bs = c('sizes'), b['sizes']
print('== SIZES top; * = entrant or changed')
bm = dict(map(tuple, bs['top']))
for f, n in s['top']:
    print('  ', ' ' if bm.get(f) == n else f'* (was {bm.get(f)})', f, n)

g, bg = c('graph'), b['graph']
print('== FAN-IN; * = entrant')
bf = dict(map(tuple, bg['fan_in_top']))
for sym, n in g['fan_in_top']:
    print('  ', f'{n:4d}', f'(was {bf[sym]})' if sym in bf else '* entrant', sym)
print('   left the top-20:', [k for k in bf if k not in dict(map(tuple, g['fan_in_top']))])
bo = dict(map(tuple, bg['fan_out']))
print('== FAN-OUT changes:', [(u, bo.get(u), n) for u, n in g['fan_out'] if bo.get(u) != n])
print('   edges', bg['cross_unit_edges'], '->', g['cross_unit_edges'])
print('   edges of escher-driver:', [e for e in g['edge_list'] if 'escher-driver' in e])

dd, bdd = c('dead'), b['dead']
print('== DEAD', bdd['zero_ref_candidates'], '->', dd['zero_ref_candidates'], '| classes', bdd['fp_classes'], '->',
      dd['fp_classes'])
print('   unused deps equal:', sorted(map(tuple, bdd['unused_deps'])) == sorted(map(tuple, dd['unused_deps'])))
full = {t[0] for t in dd['candidates_full']}
print('   baseline top still candidates:', [(t[0] in full) for t in bdd['top']])
print('   by unit', dd['candidates_by_unit'])
print('   candidates in escher crates / seven_guis:')
for t in dd['candidates_full']:
    if t[0].split(' ')[0] in ('escher-driver', 'escher-telemetry', 'seven_guis', 'dioxus-native-dom',
                              'blitz-test-harness'):
        print('     ', t)

ch = c('churn')
print('== CHURN', b['churn'], '->', {k: ch[k] for k in ('pct', 'files_churned', 'files_touched', 'all_adds',
                                                           'churned_adds', 'commits_touching_source')})
for row in ch['top']:
    print('  ', row)
h = c('hotspots')
print('== HOTSPOTS; * = entrant')
bh = dict(map(tuple, b['hotspots']))
for t in h['top_detail']:
    print('  ', '*' if t['file'] not in bh else ' ', t)
print('   absent at head:', h['touched_files_absent_at_head'])
print('== COVERAGE', b['coverage'], '->', c('coverage'))
print('== SCALARS chain')
chain = [b]
prev = [r for r in recs if r['sha'] == b['baseline_sha']]
if prev:
    chain.append(prev[-1])
for r in reversed(chain):
    print('  ', r['epoch'], r['duplication']['pct'], r['complexity']['over_ceiling'], r['dead']['zero_ref_candidates'],
          r['sizes']['file_max'], r['sizes']['over_800'], r['coverage']['line'], '| clones', r['duplication']['clones'],
          r['duplication']['duplicated_lines'], r['duplication']['total_lines'], r['totals'])
print('   this run', d['pct'], x['over_ceiling'], dd['zero_ref_candidates'], s['file_max'], s['over_800'],
      c('coverage')['line'], '| clones', d['clones'], d['duplicated_lines'], d['total_lines'], s['totals'])
