"""Assemble the ledger record (audit-pass.md §Schema) from the run dir's c-*.json twins.

usage: assemble.py RUN_DIR
Reads RUN_DIR/c-{sizes,complexity,duplication,graph,dead,coverage,churn}.json, the optional
RUN_DIR/c-mutation.json (sum_mutation.py) and RUN_DIR/meta.json (ts, epoch, mode, sha, baseline_sha,
span, head_overshoot, tool_versions, units, commands, recipes, corrections, skips).
Writes RUN_DIR/record.json; asserts the per-unit survivor / not_measured invariants before writing.
"""
import json, os, sys

rd = sys.argv[1]


def load(n):
    p = f'{rd}/{n}'
    return json.load(open(p, encoding='utf-8')) if os.path.isfile(p) else None


meta = load('meta.json')
sz = load('c-sizes.json')
cx = load('c-complexity.json')['complexity']
dup = load('c-duplication.json')
g = load('c-graph.json')
dead = load('c-dead.json')
cov = load('c-coverage.json') or {'line': None, 'branch': None}
ch = load('c-churn.json')
mut = load('c-mutation.json')

rec = {
    'ts': meta['ts'], 'epoch': meta['epoch'], 'mode': meta['mode'], 'sha': meta['sha'],
    'baseline_sha': meta['baseline_sha'], 'span': meta['span'], 'ancestry_broken': False,
    'head_overshoot': meta['head_overshoot'], 'tool_versions': meta['tool_versions'],
    'totals': {**sz['totals'], 'units': meta['units']},
    'duplication': dup,
    'complexity': {k: cx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90',
                                      'over_ceiling', 'max', 'top')},
    'sizes': sz['sizes'],
    'graph': g,
    'dead': {k: dead[k] for k in ('unused_deps', 'zero_ref_candidates', 'top')},
    'coverage': {'line': cov['line'], 'branch': cov['branch']},
    'churn': {'pct': ch['churn']['pct'], 'files_churned': ch['churn']['files_churned']},
    'hotspots': ch['hotspots'],
    'mutation': mut if mut is not None else {
        'scoped_units': [], 'unit_states': {}, 'scores': {}, 'counts': {}, 'timing': {},
        'score_formula': 'caught/(caught+missed)', 'survivors': [], 'host': None, 'not_measured': []},
    'commands': meta['commands'], 'recipes': meta['recipes'],
    'corrections': meta['corrections'], 'skips': meta['skips'],
}
m = rec['mutation']
# per-unit uniqueness and count were asserted by sum_mutation.py on each unit's own list; here the fold
keys = [(s[0], s[1]) for s in m['survivors']]
assert len(set(keys)) == len(keys) == sum(c['missed'] for c in m['counts'].values()), 'survivors vs missed'
nm = [(s[0], s[1]) for s in m['not_measured']]
assert len(set(nm)) == len(nm) == sum(c['not_measured'] for c in m['counts'].values()), 'not_measured'
assert len(rec['dead']['top']) <= 10 and len(rec['duplication']['top']) <= 10 and len(rec['hotspots']) <= 10
assert len(rec['graph']['fan_in_top']) <= 20 and len(rec['graph']['cycle_paths']) == rec['graph']['cycles']
json.dump(rec, open(f'{rd}/record.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('record.json written', len(json.dumps(rec, ensure_ascii=False)), 'bytes')
