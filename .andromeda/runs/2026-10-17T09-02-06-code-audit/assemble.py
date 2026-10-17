"""Phase 3: assemble this run's ledger record (audit-pass.md §Schema) from the c-*.json twins -> record.json.

usage: assemble.py RUN_DIR LEDGER BASELINE_SHA
"""
import json, os, re, shlex, subprocess, sys, time

rd, ledger, base = sys.argv[1:4]
RDR = rd.rstrip('/')
L = lambda n: json.load(open(f'{rd}/c-{n}.json', encoding='utf-8'))
recs = [json.loads(l) for l in open(ledger, encoding='utf-8') if l.strip()]
b = [r for r in recs if r['sha'] == base][-1]
head = subprocess.run(['git', 'rev-parse', 'HEAD'], capture_output=True, text=True, check=True).stdout.strip()
SRC = "-- '*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**'"

sizes, cx, dup, graph, dead, cov, churn, hs = (L(n) for n in
    ('sizes', 'complexity', 'duplication', 'graph', 'dead', 'coverage', 'churn', 'hotspots'))


def short(sym):
    m = re.match(r'^rust-analyzer cargo (\S+) \S+ (.*)$', sym)
    return f'{m.group(1)} {m.group(2)}' if m else sym


# cargo-machete's report: "{crate} -- {manifest}:" then tab-indented dependency names
unused, cur = [], None
for line in open(f'{rd}/machete.out', encoding='utf-8'):
    m = re.match(r'^(\S+) -- \S+:$', line.rstrip())
    if m: cur = m.group(1); continue
    if cur and line.startswith('\t') and line.strip(): unused.append([cur, line.strip()]); continue
    if not line.strip(): cur = None

# ---- mutation: the driver's units, keyed as the baseline keys them
UNITS = [('seven_guis', 'seven_guis'), ('escher-driver', 'escher-driver'), ('blitz-dom', 'blitz-dom/range-keyboard')]
drv = open(f'{rd}/mutation-driver.log', encoding='utf-8').read().splitlines()
mut = {'scoped_units': [], 'unit_states': {}, 'scores': {}, 'counts': {}, 'timing': {},
       'score_formula': 'caught/(caught+missed)', 'survivors': [], 'host': None, 'not_measured': []}
commands_mut, skips = {}, []
for unit, key in UNITS:
    hand = next((l.split(' hand pass: ', 1)[1] for l in drv if f' {unit} hand pass: ' in l), None)
    hexit = next((l.rsplit(' ', 1)[1] for l in drv if f' {unit} hand pass exit ' in l), None)
    mform = next((l.split(' mutants: ', 1)[1] for l in drv if f' {unit} mutants: ' in l), None)
    p = f'{rd}/c-mutation-{unit}.json'
    mut['scoped_units'].append(key)
    if mform:
        mform = mform.replace(RDR, RDR)  # already repo-relative (the driver is run with a relative run dir)
    commands_mut[f'mutation:{key}'] = (f'{hand} ({"the unmutated pass, by hand, exit " + str(hexit)}) · mkdir -p target/mutants-tmp && {mform}'
                                       if hand else None)
    if hexit is not None and hexit != '0':
        mut['unit_states'][key] = 'baseline-test-failure'; skips.append({'metric': f'mutation:{key}', 'reason': 'baseline-test-failure'}); continue
    if not os.path.isfile(p):
        mut['unit_states'][key] = 'budget-exhausted'; skips.append({'metric': f'mutation:{key}', 'reason': 'budget-exhausted'}); continue
    c = json.load(open(p, encoding='utf-8'))
    mut['host'] = c['host']
    mut['unit_states'][key] = c['state']
    mut['counts'][key] = {k: c['counts'][k] for k in ('mutants', 'caught', 'missed', 'not_measured', 'timeout', 'unviable')}
    mut['timing'][key] = c['timing']
    mut['survivors'] += c['survivors']
    mut['not_measured'] += c['not_measured']
    if c['state'].startswith('unviable-dominant'):
        skips.append({'metric': f'mutation:{key}', 'reason': 'unviable-dominant'})
    elif c['score'] is not None:
        mut['scores'][key] = c['score']
    assert len([s for s in c['survivors']]) == c['counts']['missed']
    assert len(c['not_measured']) == c['counts']['not_measured']

skips += [
    {'metric': 'mutation:dioxus-native-dom', 'reason': 'declined'},
    {'metric': 'mutation:seven_guis (the unit beyond timer.rs: 239 listed whole)', 'reason': 'declined'},
    {'metric': 'mutation:escher-driver (the unit beyond its touched mcp/schema files: 525 listed whole)', 'reason': 'declined'},
    {'metric': 'mutation:blitz-dom (its touched document.rs 500 + events/pointer.rs 265 beyond range.rs + keyboard.rs; 5617 whole)', 'reason': 'declined'},
]

ts = time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())
P = f'.andromeda/runs/{os.path.basename(RDR)}'
rec = {
    'ts': ts,
    'epoch': 'Epoch 1 — Foundation',
    'mode': 'trend', 'sha': head, 'baseline_sha': base, 'span': 1, 'ancestry_broken': False,
    'head_overshoot': {'boundary_sha': head, 'commits': 0, 'files': [],
                       'note': 'at the boundary: HEAD is the 2026-10-16-fast-failing-mutants complete flip. The label repeats the first record\'s (0.1.0 Epoch 1, d4113768): this is escher-0.2.0\'s Epoch 1, a different boundary, recorded as trend against 0.1.0\'s Epoch 6 at span 1 (the operator\'s choice at the epoch dialogue)'},
    'tool_versions': {'jscpd': '5.4.0', 'tokei': '14.0.0', 'rust-code-analysis': '0.0.25',
                      'code-graph.py': subprocess.run(['git', 'rev-parse', 'HEAD:scripts/code-graph.py'], capture_output=True, text=True).stdout.strip()[:8],
                      'cargo-machete': '0.9.2', 'cargo-llvm-cov': '0.9.1', 'cargo-mutants': '27.1.0', 'cargo-nextest': '0.9.146', 'rustc': '1.99.0'},
    'totals': {'loc': sizes['totals']['loc'], 'files': sizes['totals']['files'],
               'units': len(json.load(open(f'{rd}/metadata.json', encoding='utf-8'))['packages'])},
    'duplication': {k: dup[k] for k in ('pct', 'duplicated_lines', 'total_lines', 'clones', 'top', 'split')},
    'complexity': {k: cx[k] for k in ('cyclomatic_p50', 'cyclomatic_p90', 'cognitive_p50', 'cognitive_p90', 'over_ceiling', 'max', 'top')},
    'sizes': {k: sizes[k] for k in ('file_p50', 'file_p90', 'file_max', 'over_800', 'top')},
    'graph': {'cycles': graph['cycles'], 'cycle_paths': graph['cycle_paths'],
              'fan_in_top': [[short(s), n] for s, n in graph['fan_in_top']], 'fan_out': graph['fan_out'],
              'cross_unit_edges': graph['cross_unit_edges']},
    'dead': {'unused_deps': unused, 'zero_ref_candidates': dead['zero_ref_candidates'], 'top': dead['top'],
             'fp_classes': {'candidate': dead['zero_ref_candidates'], **{k: v for k, v in dead['fp_classes'].items() if k != 'candidate'}},
             'zero_ref_rows': dead['zero_ref_rows'], 'excluded_test_path': dead['excluded_test_path']},
    'coverage': {'line': cov['line'], 'branch': cov['branch']},
    'churn': {'pct': churn['pct'], 'files_churned': churn['files_churned']},
    'hotspots': hs['hotspots'],
    'mutation': mut,
    'commands': {
        'sizes': f"git ls-files '*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**' > {P}/srcfiles.txt && tokei --output json $(cat {P}/srcfiles.txt) > {P}/tokei.json (Rust code lines per file; population = srcfiles.txt, {sizes['totals']['files']} files; totals.units = cargo metadata --no-deps --format-version 1 --locked workspace packages; summarized by python3 {P}/sum_sizes_cx.py {P}/tokei.json {P}/rca {P}/srcfiles.txt {P}/c-; the summarizer reproduced the baseline record's sizes, totals and complexity exactly on a git archive of {base} — {P}/check_repro.py, 0 mismatches)",
        'duplication': f"jscpd --format rust --absolute --reporters json --output {P}/jscpd-abs packages apps tests examples wpt/runner (paths made repo-relative by python3 {P}/sum_dup.py {P}/jscpd-abs/jscpd-report.json . {P}/c-duplication.json; default min-tokens 50, .gitignore respected; split: a file under a `tests` directory segment is test, a pair of two is test, of none is src, else mixed; reproduced the baseline record exactly on a git archive of {base})",
        'complexity': f"mkdir -p {P}/rca && rust-code-analysis-cli -m -O json -o {P}/rca $(sed 's/^/-p /' {P}/srcfiles.txt) (every space of kind function, closures included: cognitive.sum / cyclomatic.sum; summarized with sizes)",
        'graph': f"python3 {P}/graph_queries.py {P} (fires scripts/code-graph.py query {P} code-audit \"<audit-pass.md canonical cycle / fan-in / fan-out / edge-count SQL, verbatim>\" one at a time; db_state fresh; the SQL as fired is in {P}/tree-query-code-audit.json)",
        'dead': f"cargo machete (unused deps) + python3 scripts/code-graph.py query {P} code-audit \"SELECT s.symbol, s.crate, s.name, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL\" (fired by graph_queries.py) + python3 {P}/sum_dead.py {P} {P}/machete.out (recipes.dead; def_line is 0-based in the DB, printed +1)",
        'coverage': 'bash .github/scripts/ci-leg.sh coverage (cargo llvm-cov --workspace --locked --lcov --output-path target/coverage/lcov.info && cargo llvm-cov report --workspace --locked; TOTAL row, Lines cover column)',
        'churn': f"git log --reverse --numstat --format=@%H {base}..HEAD {SRC} (file-level: adds in a file's 2nd..nth touching commit are churn; a rename counts to its new path; python3 {P}/sum_git.py {P} {base} {P}/coverage.raw.log)",
        'hotspots': f'churn per-file commit counts ({base}..HEAD) x max cognitive per file from complexity (a measured 0 scores 0)',
        'mutation': f"per scoped unit: its `mutation:{{unit}}` entry (the unmutated pass, by hand, then cargo mutants on nextest's `mutants` profile with --baseline=skip under the founder's footprint, driven sequentially by python3 {P}/mutate.py {P} .andromeda/code-metrics.ndjson {base}; each unit's test set is the one the baseline record's `mutation:{{unit}}` command names; summarized by python3 {P}/sum_mutation.py)",
        'head_overshoot': f"git log --format=%H -1 -S'2026-10-16-fast-failing-mutants · complete' -- .andromeda/master-route.md · git rev-list --count {head}..HEAD · git diff --numstat {head}..HEAD {SRC}",
        **commands_mut,
    },
    'recipes': {'dead': b['recipes']['dead']},
    'corrections': [],
    'skips': skips,
}
json.dump(rec, open(f'{rd}/record.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('record.json written; mutation states', mut['unit_states'], 'scores', mut['scores'], 'skips', len(skips))
