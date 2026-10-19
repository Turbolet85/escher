"""Mutation (C1) per-unit summarizer — reads a COMPLETE invocation's `{output}/mutants.out/` only.

usage: sum_mutation.py OUTPUT_DIR UNIT OUT_JSON [STATE]
Completion (cargo-mutants markers): outcomes.json top-level `end_time` set AND `total_mutants` == len(mutants.json).
--baseline=skip: no baseline outcome; the recorded hand pass stands in for outcomes[0].
Score formula: caught/(caught+missed); a `missed` mutant whose whole span the host's build excludes is
not_measured (hostcfg.cover), never a survivor. unviable > caught+missed+not_measured+timeout → unviable-dominant.
"""
import json, os, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from hostcfg import cover, host_cfg  # noqa: E402

out_dir, unit, out_json = sys.argv[1:4]
mo = os.path.join(out_dir, 'mutants.out')
oc = json.load(open(os.path.join(mo, 'outcomes.json'), encoding='utf-8'))
ml = json.load(open(os.path.join(mo, 'mutants.json'), encoding='utf-8'))
complete = bool(oc.get('end_time')) and oc.get('total_mutants') == len(ml)
repo = os.getcwd()
cfg = host_cfg(repo)
counts = {'mutants': len(ml), 'caught': 0, 'missed': 0, 'not_measured': 0, 'timeout': 0, 'unviable': 0}
survivors, not_measured, other = [], [], []
tested = 0
for o in oc['outcomes']:
    sc = o['scenario']
    if not isinstance(sc, dict) or 'Mutant' not in sc:
        continue
    m = sc['Mutant']
    tested += 1
    s = o['summary']
    name = m.get('name') or ''
    site = '{}:{}:{}'.format(m['file'], m['span']['start']['line'], m['span']['start']['column'])
    text = name.split(': ', 1)[1] if ': ' in name else name
    if s == 'CaughtMutant':
        counts['caught'] += 1
    elif s == 'MissedMutant':
        p = cover(repo, m, cfg)
        if p:
            counts['not_measured'] += 1
            not_measured.append([site, text, f'cfg({p})'])
        else:
            counts['missed'] += 1
            survivors.append([site, text])
    elif s == 'Timeout':
        counts['timeout'] += 1
    elif s == 'Unviable':
        counts['unviable'] += 1
    else:
        other.append([site, s])
assert len({(a, b) for a, b in survivors}) == len(survivors) == counts['missed'], 'survivor key coarser than tool'
assert counts['missed'] + counts['not_measured'] == oc.get('missed', counts['missed'] + counts['not_measured'])
dominant = counts['unviable'] > counts['caught'] + counts['missed'] + counts['not_measured'] + counts['timeout']
den = counts['caught'] + counts['missed']
res = {
    'unit': unit, 'complete': complete, 'tool_total_mutants': oc.get('total_mutants'), 'listed': len(ml),
    'tested': tested, 'counts': counts,
    'score': None if dominant or not complete or den == 0 else round(100.0 * counts['caught'] / den, 2),
    'unviable_dominant': dominant, 'survivors': sorted(survivors), 'not_measured': sorted(not_measured),
    'other_outcomes': other, 'start_time': oc.get('start_time'), 'end_time': oc.get('end_time'),
    'tool_tallies': {k: oc.get(k) for k in ('caught', 'missed', 'timeout', 'unviable', 'success', 'failure')},
    'host': next((v for k, v in cfg if k == 'target_arch'), None),
}
if len(sys.argv) > 4:
    res['state'] = sys.argv[4]
json.dump(res, open(out_json, 'w', encoding='utf-8'), indent=1)
print(json.dumps({k: res[k] for k in ('unit', 'complete', 'tested', 'counts', 'score', 'unviable_dominant')}))
