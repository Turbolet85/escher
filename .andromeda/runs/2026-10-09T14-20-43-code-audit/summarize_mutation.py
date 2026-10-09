"""Per-unit mutation summary: c-mutation-{key}.json, written only for a COMPLETE invocation
(outcomes.json's end_time set AND its total_mutants equal to the length of mutants.json).

usage: summarize_mutation.py <key>
"""
import datetime
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cover import cover, host_cfg  # noqa: E402

RUN = '.andromeda/runs/2026-10-09T14-20-43-code-audit'
SCOPES = {'escher-driver': 'unit', 'escher-telemetry': 'unit', 'dioxus-native-dom': 'files',
          'blitz-test-harness': 'files', 'dioxus-native-dom-snapshot': 'files', 'blitz-dom-scrolling': 'filter',
          'blitz-dom-changed-set': 'filter'}
FILTERS = {'blitz-dom-scrolling': 'scrolling-touched', 'blitz-dom-changed-set': 'changed-set'}
key = sys.argv[1]
out = f'{RUN}/mutants-{key}/mutants.out'
o = json.load(open(f'{out}/outcomes.json', encoding='utf-8'))
listing = json.load(open(f'{out}/mutants.json', encoding='utf-8'))
st = json.load(open(f'{RUN}/mutation-status-{key}.json', encoding='utf-8'))
hand = json.load(open(f'{RUN}/mutation-hand-{key}.json', encoding='utf-8'))

complete = bool(o.get('end_time')) and o.get('total_mutants') == len(listing)
print('end_time', o.get('end_time'), '| total_mutants', o.get('total_mutants'), '| listing', len(listing),
      '| complete', complete, '| status', st.get('state'), 'rc', st.get('returncode'))
if not complete:
    sys.exit('NOT complete by the tool own markers: no tally read, no artifact written')

# a function-filtered unit: the mutants its rule names are tallied as the unit, the field-deletion mutants the
# tool's name filter does not reach (riders) separately - the operator's word at the blitz-dom confirm
flt = json.load(open(f'{RUN}/mutation-filter-blitz-dom-{FILTERS[key]}.json', encoding='utf-8')) \
    if key in FILTERS else None
if flt:
    assert sorted(m['name'] for m in listing) == sorted(flt['names'] + flt['riders']), 'listing != filter set'
all_rows = [x for x in o['outcomes'] if isinstance(x['scenario'], dict) and 'Mutant' in x['scenario']]
assert len(all_rows) == o['total_mutants'], (len(all_rows), o['total_mutants'])
tool_tally = {}
for x in all_rows:
    tool_tally[x['summary']] = tool_tally.get(x['summary'], 0) + 1
for field, cls in (('caught', 'CaughtMutant'), ('missed', 'MissedMutant'), ('unviable', 'Unviable'),
                   ('timeout', 'Timeout')):
    assert o.get(field, 0) == tool_tally.get(cls, 0), (field, o.get(field), tool_tally.get(cls, 0))
rider_names = set(flt['riders']) if flt else set()
rider_rows = [x for x in all_rows if x['scenario']['Mutant']['name'] in rider_names]
rows = [x for x in all_rows if x['scenario']['Mutant']['name'] not in rider_names]
assert len(rider_rows) == len(rider_names) and len(rows) == (flt['in_rule'] if flt else len(listing))
others = [x['scenario'] for x in o['outcomes'] if not (isinstance(x['scenario'], dict) and 'Mutant' in x['scenario'])]
by = {}
for x in rows:
    by.setdefault(x['summary'], []).append(x['scenario']['Mutant'])
known = {'CaughtMutant', 'MissedMutant', 'Unviable', 'Timeout'}
assert set(by) <= known, f'unknown outcome classes: {set(by) - known}'


def site_mut(m):
    site, mutation = m['name'].split(': ', 1)
    return [site, mutation]


repo = os.getcwd()
cfg = host_cfg(repo)
host = [l.split(': ', 1)[1] for l in __import__('subprocess').run(
    ['rustc', '-vV'], capture_output=True, text=True, check=True).stdout.splitlines() if l.startswith('host: ')][0]
survivors, not_measured = [], []
for m in by.get('MissedMutant', []):
    pred = cover(repo, m, cfg)
    if pred:
        not_measured.append(site_mut(m) + [f'cfg({pred})'])
    else:
        survivors.append(site_mut(m))
survivors.sort()
not_measured.sort()
tool_missed = len(by.get('MissedMutant', []))
assert len({tuple(s) for s in survivors}) == len(survivors), 'survivor rows not unique on (site, mutation)'
assert len({tuple(s[:2]) for s in not_measured}) == len(not_measured), 'not_measured rows not unique'
assert len(survivors) + len(not_measured) == tool_missed
rider = None
if flt:
    rc = {}
    for x in rider_rows:
        rc[x['summary']] = rc.get(x['summary'], 0) + 1
    r_missed = [x['scenario']['Mutant'] for x in rider_rows if x['summary'] == 'MissedMutant']
    r_surv = sorted(site_mut(m) for m in r_missed if not cover(repo, m, cfg))
    r_nm = sorted(site_mut(m) + [f'cfg({cover(repo, m, cfg)})'] for m in r_missed if cover(repo, m, cfg))
    rider = {'counts': {'mutants': len(rider_rows), 'caught': rc.get('CaughtMutant', 0), 'missed': len(r_surv),
                        'not_measured': len(r_nm), 'timeout': rc.get('Timeout', 0),
                        'unviable': rc.get('Unviable', 0)},
             'sites': sorted(site_mut(x['scenario']['Mutant']) + [x['summary']] for x in rider_rows),
             'survivors': r_surv, 'not_measured': r_nm, 'note': flt['riders_note']}
    assert sum(v for k, v in rider['counts'].items() if k != 'mutants') == len(rider_rows)
    # two sources: the unit's and the riders' missed together are the tool's own missed tally
    assert tool_missed + len(r_missed) == o.get('missed', 0)
else:
    assert tool_missed == o.get('missed', 0)

caught, missed = len(by.get('CaughtMutant', [])), len(survivors)
timeout, unviable = len(by.get('Timeout', [])), len(by.get('Unviable', []))
dominant = o.get('unviable', 0) > o['total_mutants'] - o.get('unviable', 0)  # over the whole invocation
ts = lambda s: datetime.datetime.fromisoformat(s.replace('Z', '+00:00'))
wall = round((ts(o['end_time']) - ts(o['start_time'])).total_seconds())
scope = SCOPES[key]
timing = {'scope': scope, 'planned': len(listing), 'tested': o['total_mutants'], 'wall_s': wall,
          'jobs': st['jobs'], 'cap_s': st['cap_s']}
if scope == 'files':
    timing['files'] = sorted({m['file'] for m in listing})
if scope == 'filter':
    timing.update(files=sorted(flt['files']), rule=flt['rule'], filter=flt['regexes'],
                  functions=[f"{x['file']}: {x['function']}" for x in flt['functions']],
                  in_rule=flt['in_rule'], riders=len(flt['riders']))
per_file = {}
for cls, ms in by.items():
    for m in ms:
        per_file.setdefault(m['file'], {}).setdefault(cls, 0)
        per_file[m['file']][cls] += 1
obj = {
    'key': key, 'unit': st['unit'],
    'unit_state': f"complete {o['total_mutants']}/{len(listing)}" if not dominant
    else f"unviable-dominant {unviable}/{o['total_mutants']}",
    'counts': {'mutants': len(rows), 'caught': caught, 'missed': missed,
               'not_measured': len(not_measured), 'timeout': timeout, 'unviable': unviable},
    'score': None if dominant or caught + missed == 0 else round(100 * caught / (caught + missed), 2),
    'score_formula': 'caught/(caught+missed)',
    'survivors': survivors, 'not_measured': not_measured, 'host': host,
    'timeouts': sorted(site_mut(m) for m in by.get('Timeout', [])),
    'unviable_sites': sorted(site_mut(m) for m in by.get('Unviable', [])),
    'per_file': dict(sorted(per_file.items())),
    'riders': rider, 'timing': timing, 'per_mutant_s': round(wall / o['total_mutants'], 2) if o['total_mutants'] else None,
    'tool': {'cargo_mutants_version': o.get('cargo_mutants_version'), 'start_time': o['start_time'],
             'end_time': o['end_time'], 'exit': st.get('returncode'), 'non_mutant_scenarios': others,
             'test_timeout_s': st['timeout_s']},
    'command': st['command'], 'hand_pass': hand, 'footprint': st.get('footprint'), 'pin': st.get('pin'),
    'cap': {k: st.get(k) for k in ('resized', 'rate_s_per_mutant', 'rate_at', 'resized_cap_s', 'cap_s')},
}
with open(f'{RUN}/c-mutation-{key}.json', 'w', encoding='utf-8', newline='') as f:
    json.dump(obj, f, ensure_ascii=False, indent=1)
    f.write('\n')
print(json.dumps({k: obj[k] for k in ('unit_state', 'counts', 'score', 'timing', 'per_mutant_s')}, indent=1))
print('survivors', len(survivors))
for s in survivors:
    print('  ', s[0], '|', s[1])
print('not measured on this host', len(not_measured))
for s in not_measured:
    print('  ', s)
print('timeouts', obj['timeouts'])
if rider:
    print('riders (tallied apart)', rider['counts'])
    for r in rider['sites']:
        print('  ', r)
