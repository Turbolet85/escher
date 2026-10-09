"""Phase 4 + 5: diffs this run's record against its baseline (re-found by sha, never as the ledger's last line),
applies the threshold table and writes proposals.md. Every evidence table is asserted here: its row count equals
an n read from an independent source, its rows are unique on the natural key.

usage: render.py            (after the append; before it, a dry run against record.json)
"""
import json
import os

RUN = '.andromeda/runs/2026-10-09T14-20-43-code-audit'
norm = lambda s: (s or '').lstrip('# \t').strip().replace('–', '-').replace('—', '-')
recs, bad = [], 0
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try:
        recs.append(json.loads(line))
    except ValueError:
        bad += 1
this = json.load(open(f'{RUN}/record.json', encoding='utf-8'))
own = [i for i, r in enumerate(recs) if r['ts'] == this['ts']]
appended = bool(own)
before = recs[:own[-1]] if appended else recs
if appended:
    assert recs[own[-1]] == this, 'the appended line is not record.json'
base = [r for r in before if r['sha'] == this['baseline_sha']][-1]
prev = [r for r in before[:before.index(base)] if r['sha'] == base['baseline_sha']][-1]
c = lambda n: json.load(open(f'{RUN}/c-{n}.json', encoding='utf-8'))
notes = json.load(open(f'{RUN}/survivor-notes.json', encoding='utf-8'))
ep = lambda r: r['epoch'].split(' — ')[0]
out = []
w = out.append


def table(headers, rows, n, key, what):
    assert len(rows) == n, f'{what}: {len(rows)} rows against n = {n}'
    keys = [key(r) for r in rows]
    assert len(set(keys)) == len(keys), f'{what}: rows not unique on the natural key'
    w('| ' + ' | '.join(headers) + ' |')
    w('|' + '---|' * len(headers))
    for r in rows:
        w('| ' + ' | '.join(str(x) for x in r['cells']) + ' |')
    w('')


def sgn(x, nd=None):
    x = round(x, nd) if nd is not None else x
    return ('+' if x > 0 else '−' if x < 0 else '±') + str(abs(x))


M, bM = this['mutation'], base['mutation']
dup, bdup, pdup = this['duplication'], base['duplication'], prev['duplication']

# ---------------------------------------------------------------- judging (the fixed threshold table, span 1)
assert this['span'] == 1 and this['mode'] == 'trend'
tv, btv = this['tool_versions'], base['tool_versions']
breaks = [k for k in tv if btv.get(k) and tv[k].split()[0] != btv[k].split()[0]]
unknown_tokens = [k for k in tv if not btv.get(k)]
checks = []  # (id, rule, reading, fires)
g, bg = this['graph'], base['graph']
checks.append(('new-cycle', 'cycles > baseline', f"{bg['cycles']} → {g['cycles']}", g['cycles'] > bg['cycles']))
d_pt = dup['pct'] - bdup['pct']
checks.append(('duplication-up', 'pct ≥ baseline + 0.5 pt and ≥ +15 %',
               f"{bdup['pct']} % → {dup['pct']} % ({sgn(d_pt, 3)} pt)",
               d_pt >= 0.5 and dup['pct'] >= bdup['pct'] * 1.15))
oc, boc = this['complexity']['over_ceiling'], base['complexity']['over_ceiling']
checks.append(('complexity-creep', 'functions over the ceiling ≥ baseline + 3 and ≥ +25 %', f'{boc} → {oc}',
               oc >= boc + 3 and oc >= boc * 1.25))
zc, bzc = this['dead']['zero_ref_candidates'], base['dead']['zero_ref_candidates']
checks.append(('dead-growth', 'zero-reference candidates ≥ baseline + 5', f'{bzc} → {zc}', zc >= bzc + 5))
cl, bcl = this['coverage']['line'], base['coverage']['line']
checks.append(('coverage-drop', 'line coverage ≤ baseline − 2 pt', f'{bcl} % → {cl} % ({sgn(cl - bcl, 2)} pt)',
               cl <= bcl - 2))

# mutation: each scored unit against its LAST scored value, at equal scope only
corr = {(x['target_sha'], x['field']): x for x in this['corrections']}


def scope_of(rec, unit):
    t = (rec['mutation'].get('timing') or {}).get(unit)
    if t:
        return t
    fill = corr.get((rec['sha'], f'mutation.timing.{unit}'))
    return fill['now'] if fill else None  # None = a scope the record does not settle


def same_scope(a, b):
    if not a or not b or a['scope'] != b['scope']:
        return False
    if a['scope'] == 'unit':
        return True
    if a['scope'] == 'files':
        return sorted(a['files']) == sorted(b['files'])
    return False  # shard and filter scopes: no earlier score of that kind exists


mut_rows, first, not_comp, compared = [], [], [], []
for unit in M['scoped_units']:
    last = next((r for r in reversed(before) if unit in (r['mutation'].get('scores') or {})), None)
    score = M['scores'].get(unit)
    if last is None:
        first.append(unit)
    elif score is not None:
        ls, s_now, s_then = last['mutation']['scores'][unit], scope_of(this, unit), scope_of(last, unit)
        if same_scope(s_now, s_then):
            compared.append((unit, last, ls, score))
            checks.append((f'mutation-drop · {unit}', 'score ≤ its last scored value − 10 pt, at equal scope',
                           f"{ls} ({ep(last)}) → {score} ({sgn(score - ls, 2)} pt)", score <= ls - 10))
        else:
            not_comp.append((unit, last, ls, score, s_then))
TRACKED = [('duplication.pct', lambda r: r['duplication']['pct'], 1),
           ('complexity.over_ceiling', lambda r: r['complexity']['over_ceiling'], 1),
           ('dead.zero_ref_candidates', lambda r: r['dead']['zero_ref_candidates'], 1),
           ('sizes.file_max', lambda r: r['sizes']['file_max'], 1),
           ('sizes.over_800', lambda r: r['sizes']['over_800'], 1),
           ('coverage.line', lambda r: r['coverage']['line'], -1)]
mono = []
for name, get, worse in TRACKED:
    a, b, cur = get(prev), get(base), get(this)
    assert None not in (a, b, cur), name
    mono.append((name, a, b, cur, (b - a) * worse > 0 and (cur - b) * worse > 0))
fired = [x for x in checks if x[3]] + [x for x in mono if x[4]]

# ---------------------------------------------------------------- entrants
fan_e = [x for x in g['fan_in_top'] if x[0] not in {y[0] for y in bg['fan_in_top']}]
hot_e = [x for x in this['hotspots'] if x[0] not in {y[0] for y in base['hotspots']}]
dup_e = [x for x in dup['top'] if tuple(x) not in {tuple(y) for y in bdup['top']}]
dup_gone = [x for x in bdup['top'] if tuple(x) not in {tuple(y) for y in dup['top']}]
strip = lambda t: (t[0], t[1].rsplit(':', 1)[0])
cplx_e = [x for x in this['complexity']['top'] if strip(x) not in {strip(y) for y in base['complexity']['top']}]
size_e = [x for x in this['sizes']['top'] if x[0] not in {y[0] for y in base['sizes']['top']}]

# ---------------------------------------------------------------- the document
ov, bov = this['head_overshoot'], base['head_overshoot']
w(f"# Code Audit — escher · {this['epoch']} · {this['ts']}")
w(f"mode {this['mode']} · HEAD {this['sha']} · baseline {this['baseline_sha']} ({base['epoch']}) · span {this['span']}")
w(f"overshoot: this run {ov['commits']} commits ({ov['note']}) · the baseline record {bov['commits']} commits")
w('ancestry: the baseline is an ancestor of HEAD · trend-break: '
  + ('none — every tool version token equals the baseline record\'s' if not breaks else ', '.join(breaks))
  + (f" · tokens the baseline does not carry: {', '.join(unknown_tokens)}" if unknown_tokens else ''))
w(f"ledger: {len(recs)} parseable records, {bad} unparseable"
  + ('' if appended else ' — DRY RUN, this record not yet appended'))
w('')
w('Nothing here is applied, queued or gated on. A finding is a reading for the founder; a fix, if any, goes '
  'through a route entry or a chunk at his call.')
w('')

w('## Proposals')
if not fired:
    w(f'None. No check of the threshold table fires at this boundary: the {len(checks)} single-epoch readings and '
      f'the {len(mono)} tracked scalars are listed under **Below threshold**. The one test gap this run found — a '
      'surviving mutant — has no threshold rule in trend mode and is listed in full under **Informational**, '
      'in the survivors listing.' if sum(len(v) for v in [M['survivors']]) == 1 else
      f'None. No check of the threshold table fires at this boundary: the {len(checks)} single-epoch readings and '
      f'the {len(mono)} tracked scalars are listed under **Below threshold**. Surviving mutants have no threshold '
      'rule in trend mode; they are listed in full under **Informational**, in the survivors listing.')
else:
    raise SystemExit(f'a check fires and needs its written proposal: {fired}')
w('')

# ------------------------------------------------ Informational
w('## Informational')
w('')
w('### Mutation scores')
w(f"Host `{M['host']}` · score = {M['score_formula']} · every unit's unmutated test set was run by hand first and "
  'was green. In every unit each caught mutant built and then failed a test; a mutant that did not build is '
  '`unviable` and is outside the score.')
w('')
rows = []
for unit in M['scoped_units']:
    cn, t = M['counts'][unit], M['timing'][unit]
    scope = {'unit': 'whole unit', 'files': f"{len(t.get('files', []))} file(s)",
             'filter': f"{len(t.get('functions', []))} functions"}[t['scope']]
    rows.append({'k': unit, 'cells': [f'`{unit}`', scope, cn['mutants'], cn['caught'], cn['missed'], cn['unviable'],
                                      cn['timeout'], M['scores'].get(unit, '—'), f"{round(t['wall_s'] / 60)} min"]})
table(['Unit', 'Scope', 'Mutants', 'Caught', 'Missed', 'Unviable', 'Timeout', 'Score', 'Wall'], rows,
      len(M['unit_states']), lambda r: r['k'], 'mutation scores')
for unit in M['scoped_units']:
    t = M['timing'][unit]
    if t['scope'] == 'files':
        w(f"- `{unit}` covers " + ', '.join(f'`{f}`' for f in t['files']) + '.')
    if t['scope'] == 'filter':
        w(f"- `{unit}` covers {t['rule']}: " + ', '.join(f'`{f.split(": ", 1)[1]}`' for f in t['functions'])
          + f" — in {', '.join('`' + f + '`' for f in t['files'])}. The exact filter is in the record's "
          f"`mutation.timing` and `commands` entries for this unit.")
w('')
w('How each score reads against the ledger:')
for unit in first:
    w(f"- `{unit}` — a first score: no earlier record scores this unit, so there is no movement to judge.")
for unit, last, ls, score, s_then in not_comp:
    t = M['timing'][unit]
    w(f"- `{unit}` — **not comparable**, no delta: {score} over the whole unit ({t['planned']} mutants) at "
      f"{ep(this)}; {ls} at {ep(last)} over {last['mutation']['counts'][unit]['mutants']} mutants, a scope its "
      'record does not settle (its command names a file pattern, which names no list).')
for unit, last, ls, score in compared:
    w(f"- `{unit}` — comparable with {ep(last)} at equal scope (the same two files): {ls} → {score}. Read under "
      '**Below threshold**.')
w('')

w('### Survivors — the corrective listing')
surv = M['survivors']
n_missed = sum(v['missed'] for v in M['counts'].values())
w(f'{n_missed} mutant(s) survived across the {len(M["unit_states"])} units: the test set ran green with the '
  'mutation in place.')
w('')
unit_of = {}
for unit in M['scoped_units']:
    t = M['timing'][unit]
    for f in t.get('files') or []:
        unit_of.setdefault(f, []).append(unit)
rows = []
for site, mutation in surv:
    note = notes['survivors'][f'{site}: {mutation}']
    rows.append({'k': (site, mutation), 'cells': [f'`{site}`', f'`{mutation}`', note]})
table(['Site', 'Mutation', 'What the run measured about it'], rows, n_missed, lambda r: r['k'], 'survivors')
nm = M['not_measured']
w(f"Not measured on this host (`{M['host']}`): {len(nm)}"
  + (' — no missed mutant sits in code this host\'s build excludes.' if not nm else ':'))
if nm:
    rows = [{'k': (a, b), 'cells': [f'`{a}`', f'`{b}`', f'`{p}`']} for a, b, p in nm]
    table(['Site', 'Mutation', 'Predicate'], rows, sum(v['not_measured'] for v in M['counts'].values()),
          lambda r: r['k'], 'not measured')
w('')
riders = M.get('riders') or {}
if riders:
    w('### Field-deletion mutants tallied apart')
    w('cargo-mutants 27.1.0 applies no name filter to its `delete field … from struct … expression` mutants '
      "(measured: a filter that matches nothing still lists them). A function-filtered unit's invocation therefore "
      'also tests the field deletions of its files that lie outside its functions. They are outside the '
      "unit's counts and score.")
    w('')
    rows, n = [], 0
    for unit, r in riders.items():
        n += r['counts']['mutants']
        for site, mutation, outcome in r['sites']:
            rows.append({'k': (site, mutation), 'cells': [f'`{unit}`', f'`{site}`', f'`{mutation}`',
                                                          {'CaughtMutant': 'caught', 'MissedMutant': 'missed',
                                                           'Unviable': 'unviable', 'Timeout': 'timeout'}[outcome]]})
    table(['Invocation', 'Site', 'Mutation', 'Outcome'], rows, n, lambda r: r['k'], 'riders')
    rs = [(u, s) for u, r in riders.items() for s in r['survivors']]
    if rs:
        w(f'{len(rs)} of them survived:')
        rows = [{'k': tuple(s), 'cells': [f'`{u}`', f'`{s[0]}`', f'`{s[1]}`', notes['riders'][f'{s[0]}: {s[1]}']]}
                for u, s in rs]
        table(['Invocation', 'Site', 'Mutation', 'What the run measured about it'], rows,
              sum(r['counts']['missed'] for r in riders.values()), lambda r: r['k'], 'rider survivors')

w('### Top-list entrants')
w(f"- **Fan-in** ({len(fan_e)} entrants in the top {len(g['fan_in_top'])}): "
  + '; '.join(f'`{s}` {n}' for s, n in fan_e)
  + '. All three are test-side: the stand\'s task type and the headless harness, called from the new driver checks.')
w(f"- **Hotspots** ({len(hot_e)} entrants in the top {len(this['hotspots'])}; score = touching commits × the "
  "file's highest cognitive complexity): " + '; '.join(f'`{f}` {n}' for f, n in hot_e) + '.')
seen_in = lambda row: [ep(r) for r in before if any(tuple(x) == tuple(row) for x in r['duplication']['top'])]
w(f"- **Duplication top {len(dup['top'])}** ({len(dup_e)} rows absent from the baseline's list, both of them "
  're-entries, not new clones): '
  + '; '.join(f"`{a}` ↔ `{b}` {n} lines (listed at {', '.join(seen_in([a, b, n]))})" for a, b, n in dup_e)
  + '. They return because ' + ' and '.join(f'`{a.rsplit("/", 1)[1]}` ↔ `{b.rsplit("/", 1)[1]}` ({n} lines)'
                                             for a, b, n in dup_gone)
  + ' left the list: jscpd reports neither pair at HEAD.')
assert all(seen_in(x) for x in dup_e) and len(dup_e) == len(dup_gone)
w(f"- **Complexity top {len(this['complexity']['top'])}**: {len(cplx_e)} entrants. "
  f"**Sizes top {len(this['sizes']['top'])}**: {len(size_e)} entrants.")
w('')

w('### Count under ratio — duplication')
sp, bsp = dup['split'], bdup['split']
top = dup['top'][0]
first_seen = seen_in(top)[0]
w(f"Clones {bdup['clones']} → {dup['clones']} ({sgn(dup['clones'] - bdup['clones'])}) and duplicated lines "
  f"{bdup['duplicated_lines']} → {dup['duplicated_lines']} ({sgn(dup['duplicated_lines'] - bdup['duplicated_lines'])}) "
  f"while the ratio fell {bdup['pct']} % → {dup['pct']} % ({sgn(d_pt, 3)} pt): the population (jscpd's total lines) "
  f"grew {bdup['total_lines']} → {dup['total_lines']} "
  f"({sgn(100 * (dup['total_lines'] / bdup['total_lines'] - 1), 1)} %). Split by path, pairs/lines — "
  + ' · '.join(f"{k} {bsp[k]['pairs']}/{bsp[k]['lines']} → {sp[k]['pairs']}/{sp[k]['lines']}" for k in ('src', 'test', 'mixed'))
  + f". Top standing pair: `{top[0]}` ↔ `{top[1]}`, {top[2]} lines, listed since the {first_seen} record "
  '(the first record, so its entry predates the ledger).')
w('')

ch, bch = c('churn'), base['churn']
w('### Churn')
w(f"{bch['pct']} % → {ch['pct']} % of added source lines landed in a file's second or later touching commit; "
  f"files churned {bch['files_churned']} → {ch['files_churned']} of {ch['files_touched']} touched, over "
  f"{ch['commits_touching_source']} source-touching commits ({ch['churned_adds']} of {ch['all_adds']} added lines). "
  'The threshold table holds no rule for churn once the ledger has two records, so this is a reading only. '
  'The epoch built the driver in successive chunks over the same files:')
w('')
rows = [{'k': r[0], 'cells': [f'`{r[0]}`', r[1], r[2], r[3]]} for r in ch['top']]
table(['File', 'Touching commits', 'Churned adds', 'All adds'], rows, min(10, ch['files_churned']),
      lambda r: r['k'], 'churn top')

w('### Dependency graph')
new_edges = [e for e in c('graph')['edge_list'] if 'escher-driver' in e]
w(f"Units {base['totals']['units']} → {this['totals']['units']}; cross-unit edges {bg['cross_unit_edges']} → "
  f"{g['cross_unit_edges']}; cycles {bg['cycles']} → {g['cycles']}. The "
  f"{g['cross_unit_edges'] - bg['cross_unit_edges']} new edges are all the new unit's: "
  + ', '.join(f'`{a}` → `{b}`' for a, b in new_edges) + '.')
assert len(new_edges) == g['cross_unit_edges'] - bg['cross_unit_edges']
w('')

w('### How the mutation tier ran')
for line in notes['tier']:
    w(f'- {line}')
w('')
w('### Measurement notes')
for line in notes['measurement']:
    w(f'- {line}')
w('')
w('### Corrections carried in this record')
if this['corrections']:
    rows = [{'k': (x['target_sha'], x['field']),
             'cells': [f"`{x['target_sha'][:8]}`", f"`{x['field']}`", x['note']]} for x in this['corrections']]
    table(['Target record', 'Field', 'What and why'], rows, len(this['corrections']), lambda r: r['k'], 'corrections')
else:
    w('None.')
    w('')

# ------------------------------------------------ Below threshold
w('## Below threshold — no action')
rows = [{'k': k, 'cells': [k, rule, reading, 'no']} for k, rule, reading, f in checks]
table(['Check', 'Fires when', 'Baseline → this run', 'Fires'], rows, 5 + len(compared), lambda r: r['k'], 'checks')
w(f"`monotonic` — a tracked scalar worse at both of the last two diffs ({ep(prev)} → {ep(base)} → {ep(this)}):")
w('')
rows = [{'k': n, 'cells': [f'`{n}`', a, b, cur, 'no']} for n, a, b, cur, f in mono]
table(['Scalar', ep(prev), ep(base), ep(this), 'Fires'], rows, 6, lambda r: r['k'], 'monotonic')
cx, bcx, sz, bsz = this['complexity'], base['complexity'], this['sizes'], base['sizes']
w('Other movements, for the eye only:')
w(f"- Source size: {base['totals']['files']} → {this['totals']['files']} files, {base['totals']['loc']} → "
  f"{this['totals']['loc']} code lines. File size p50 {bsz['file_p50']} → {sz['file_p50']}, p90 {bsz['file_p90']} "
  f"→ {sz['file_p90']}, largest {bsz['file_max']} → {sz['file_max']} (`{sz['top'][0][0]}`).")
w(f"- Complexity: cyclomatic p90 {bcx['cyclomatic_p90']:g} → {cx['cyclomatic_p90']:g}, cognitive p90 "
  f"{bcx['cognitive_p90']:g} → {cx['cognitive_p90']:g}; the highest stays `{cx['max']['fn']}` at "
  f"{cx['max']['val']:g}. No function of the two escher crates is over the ceiling of 15.")
assert not [x for x in c('complexity')['over_ceiling_list'] if '/escher-' in x[1]]
bd_, d_ = base['dead'], this['dead']
w(f"- Dead-code candidates: unused dependencies {len(bd_['unused_deps'])} → {len(d_['unused_deps'])}, the same "
  f"list; zero-reference rows {bd_['zero_ref_rows']} → {d_['zero_ref_rows']}, of which the test-path exclusion "
  f"takes {bd_['excluded_test_path']} → {d_['excluded_test_path']}. None of the {zc} candidates is in "
  '`escher-driver` or `escher-telemetry`.')
assert sorted(map(tuple, bd_['unused_deps'])) == sorted(map(tuple, d_['unused_deps']))
assert not [x for x in c('dead')['candidates_full'] if x[0].split(' ')[0] in ('escher-driver', 'escher-telemetry')]
w(f"- Coverage: branch coverage is not instrumented by the project's coverage leg (null in every record).")
w('')

# ------------------------------------------------ Skips
w('## Skips')
rows = [{'k': x['metric'], 'cells': [x['metric'], x['reason']]} for x in this['skips']]
table(['Metric', 'Reason'], rows, len(this['skips']), lambda r: r['k'], 'skips')
for line in notes['skips']:
    w(f'- {line}')
w('')

assert sum(len(M['survivors']) for _ in [0]) == n_missed
with open(f'{RUN}/proposals.md', 'w', encoding='utf-8', newline='') as f:
    f.write('\n'.join(out).rstrip('\n') + '\n')
P = len(fired)
I = (len(first) + len(not_comp) + n_missed + (1 if riders else 0) + len(fan_e) + len(hot_e) + len(dup_e) + 1 + 1 + 1)
print(f"proposals at {RUN}/proposals.md · mode {this['mode']} · {P} proposals · informational: "
      f"{len(first)} first scores, {len(not_comp)} not comparable, {n_missed} survivors, "
      f"{len(fan_e) + len(hot_e) + len(dup_e)} top-list entrants, count-under-ratio, churn, graph · "
      f"{len(checks) + len(mono)} below threshold · {len(this['skips'])} skips"
      + ('' if appended else ' · DRY RUN'))
