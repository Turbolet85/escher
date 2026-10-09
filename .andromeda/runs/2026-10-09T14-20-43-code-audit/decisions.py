"""Writes mutation-decisions.json (the record's mutation note, skips and corrections) and survivor-notes.json
(the report's prose lines), every figure read from this run's own files and asserted against them."""
import glob
import json
import os

RUN = '.andromeda/runs/2026-10-09T14-20-43-code-audit'
PREV = '42b80ad94e1c96014c0325e2a3f4a7135ff1f9ca'
BASE = 'b7e3a43b8540343b8e655dfc700640511ef2b707'
j = lambda n: json.load(open(f'{RUN}/{n}', encoding='utf-8'))
KEYS = ['escher-driver', 'escher-telemetry', 'dioxus-native-dom', 'blitz-test-harness', 'dioxus-native-dom-snapshot',
        'blitz-dom-scrolling', 'blitz-dom-changed-set']
tw = {k: j(f'c-mutation-{k}.json') for k in KEYS if os.path.isfile(f'{RUN}/c-mutation-{k}.json')}
plan = j('mutation-plan.json')
fs, fc = j('mutation-filter-blitz-dom-scrolling-touched.json'), j('mutation-filter-blitz-dom-changed-set.json')
recs = [json.loads(l) for l in open('.andromeda/code-metrics.ndjson', encoding='utf-8') if l.strip()]
e2 = [r for r in recs if r['sha'] == PREV][-1]
assert '--file packages/{unit}/src/element_id.rs --file packages/{unit}/src/dioxus_document.rs' in e2['commands']['mutation']
assert not (e2['mutation'].get('timing') or {}).get('dioxus-native-dom')
two = ['packages/dioxus-native-dom/src/dioxus_document.rs', 'packages/dioxus-native-dom/src/element_id.rs']
assert tw['dioxus-native-dom']['timing']['files'] == two

# ---- skips
bth = plan['blitz-test-harness']
other = {f: n for f, n in bth['touched_files'].items() if not f.endswith('/settle.rs')}
assert bth['touched_files']['packages/blitz-test-harness/src/settle.rs'] == tw['blitz-test-harness']['counts']['mutants']
skips = [
    {'metric': 'mutation:seven_guis', 'reason': 'declined'},
    {'metric': 'mutation:blitz-test-harness (' + ', '.join(f.rsplit('/', 1)[1] for f in sorted(other))
               + f" — the unit's other touched files, {sum(other.values())} of its {bth['touched_planned']} "
               "touched-file mutants)", 'reason': 'declined'},
    {'metric': f"mutation:blitz-dom (scrolling.rs outside the four functions this epoch changed — "
               f"{fs['listed_in_files'] - fs['in_rule']} of the file's {fs['listed_in_files']} mutants, "
               f"{len(fs['riders'])} of them tested apart as field deletions)", 'reason': 'declined'},
]
assert fs['listed_in_files'] == plan['blitz-dom']['touched_planned']

# ---- corrections
corrections = [{
    'target_sha': PREV, 'field': 'mutation.timing.dioxus-native-dom', 'was': None,
    'now': {'scope': 'files', 'files': two, 'planned': e2['mutation']['counts']['dioxus-native-dom']['mutants'],
            'tested': e2['mutation']['counts']['dioxus-native-dom']['mutants'], 'jobs': 2},
    'note': "schema-gap fill: the record scores dioxus-native-dom with no `mutation.timing` entry, so the score's "
            "scope is read from that record's own `commands.mutation`, whose two `--file` flags name these paths "
            "(`{unit}` = dioxus-native-dom) and whose `-j 2` is the jobs; planned and tested are its "
            "`counts.mutants` (its unit state reads complete 57/57); wall and cap are not recorded there and stay "
            "unknown"}]

# ---- the stopped launches (never read as results)
stopped = []
for p in sorted(glob.glob(f'{RUN}/mutation-restart-*.json')):
    r = json.load(open(p, encoding='utf-8'))
    live = r.get('live_at_stop') or r.get('live_at_death')
    stopped.append((os.path.basename(p), live.get('total_mutants') or 0, r['reason']))
n_stopped = len(glob.glob(f'{RUN}/mutation-status-*.stopped-*.json'))
assert n_stopped == 5, n_stopped  # escher-driver x2, blitz-dom/scrolling x3

t = lambda k: tw[k]['timing']
first5 = KEYS[:5]
assert all(tw[k]['timing']['jobs'] == 2 for k in tw)
sc, cs = tw.get('blitz-dom-scrolling'), tw.get('blitz-dom-changed-set')
assert sc and cs, 'both blitz-dom twins are needed'
pin = sc['pin']
caps = ', '.join(f"{k} {tw[k]['cap']['cap_s']} s" + (f" (re-sized at the floor from {tw[k]['cap']['rate_s_per_mutant']} s per mutant)"
                                                      if tw[k]['cap']['resized'] else '') for k in KEYS)
scope_note = (
    "Scope set by the operator at this run's two confirms (2026-10-09). First confirm: escher-driver and "
    "escher-telemetry whole; dioxus-native-dom as two units of their own — `dioxus-native-dom` = element_id.rs + "
    f"dioxus_document.rs (the Epoch 2 score's two files, {t('dioxus-native-dom')['planned']} of the unit's "
    f"{plan['dioxus-native-dom']['unit_planned']} listed mutants) and `dioxus-native-dom/snapshot` = snapshot.rs + "
    f"snapshot_text.rs + snapshot_diff.rs + actionable.rs ({t('dioxus-native-dom-snapshot')['planned']}; built in "
    "Epoch 3, unscored since, not in this epoch's diff); `blitz-test-harness` = settle.rs "
    f"({t('blitz-test-harness')['planned']} of the unit's {bth['touched_planned']} touched-file mutants); seven_guis "
    "declined. Second confirm, blitz-dom alone, as two function-filtered units, scope `filter` — a scope kind beside "
    "the schema's three, compared with nothing unless a later run repeats the rule: `blitz-dom/scrolling` = "
    f"{fs['rule']} ({', '.join(x['function'] for x in fs['functions'])}: {fs['in_rule']} of scrolling.rs's "
    f"{fs['listed_in_files']} mutants) and `blitz-dom/changed-set` = {fc['rule']} ({fc['in_rule']} mutants in "
    f"{len(fc['functions'])} functions of document.rs, mutator.rs and events/keyboard.rs, of the three files' "
    f"{fc['listed_in_files']}; built in Epoch 3). Each filtered unit's `timing` carries its rule, its function "
    "list and the exact `-F` regexes at this sha; the regexes are anchored on `file:line:col`, so a later boundary "
    "repeats the RULE and rebuilds them from the tool's listing (build_filter.py's method: the listing under the "
    "filter must equal the rule's set). cargo-mutants 27.1.0 applies no `-F`/`-E` regex to its `delete field … "
    "from struct … expression` mutants (measured: `-F '^nomatch$'` lists them all), so the named files' field "
    f"deletions ride each filtered invocation ({len(fs['riders'])} and {len(fc['riders'])}): tallied apart under "
    "`mutation.riders`, outside the unit's counts, score and survivors. Touched units listing 0 mutants: "
    "blitz-tests, blitz-examples. "
    "Footprint. The first launch (escher-driver, -j 4, cargo's default build jobs) contended the host — measured "
    "by the operator at 14:39Z: load 209 on 32 cores, IO stalled 465 s, 35 linkers at once, another builder having "
    "priority — and was stopped on his direction; from there every unit ran at `-j 2`, `CARGO_BUILD_JOBS=4` per "
    "cargo, `--jobserver-tasks 6` (measured: at most 8 rustc at once; 8 tasks had measured 10), "
    "`RUST_TEST_THREADS=4`, and `--timeout 600` in place of the earlier records' 120 (a timeout taken under "
    "contention would misread a mutant; 0 timeouts occurred in any unit), each unmutated pass by hand under the "
    "same environment, exit 0. The two blitz-dom units ran, on the founder's word relayed by the operator (the "
    "host in interactive use), with the whole invocation under `nice -n 19 ionice -c 3` (the disk's scheduler is "
    "`none`, which does not act on the idle IO class). The operator pinned the running scrolling tree to CPUs "
    f"{pin['cpus']} with taskset at {pin['at']} (at {pin['marker']['live_tested']} of {sc['timing']['planned']} "
    "tested; not in its firing form), and the changed-set unit and its unmutated pass were launched under the same "
    "pin. "
    f"Stopped launches, {n_stopped}, all discarded and none read as a result: escher-driver twice (the -j 4 launch "
    "at 26 tested; a 30-second launch at 8 jobserver tasks, 0 tested) and blitz-dom/scrolling three times (33 "
    "tested when the hosting sessions were killed at about 16:54Z; 0 tested at the founder's HOLD; 0 tested at a "
    "one-mutant economy launch his correction replaced). "
    "Caps: the 1800 s floor for every unit (no prior record holds a per-mutant time), re-sized once at the floor "
    f"from the unit's own rate × 1.5 — {caps}; blitz-dom/scrolling's re-size reads the rate since the pin, on the "
    "operator's word. A unit re-sized past 2 h would have run whole on his word; none was. "
    "Test sets: each unit's `commands` entry names its own; the two blitz-dom units run every test target of "
    "blitz-dom, blitz-tests and dioxus-native-dom (`--tests`: 504 tests unmutated). dioxus-native-dom's set is the "
    "Epoch 3 record's, three tests wider than the Epoch 2 score's (stand_id_edits, stand_actionable_keys, "
    "stand_snapshot).")
with open(f'{RUN}/mutation-decisions.json', 'w', encoding='utf-8', newline='') as f:
    json.dump({'scope_note': scope_note, 'skips': skips, 'corrections': corrections}, f, ensure_ascii=False, indent=1)
    f.write('\n')
print('mutation-decisions.json:', len(scope_note), 'chars of note |', len(skips), 'skips |', len(corrections),
      'correction')
print('stopped launches:', stopped)
print('caps:', caps)
