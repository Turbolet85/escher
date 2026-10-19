"""Assemble the ledger record (audit-pass.md §Schema) -> RUN_DIR/record.json.

usage: assemble.py RUN_DIR
Reads the c-*.json twins, the baseline record (re-found by baseline_sha, the last before this
run's append) for tool_versions and recipes, and asserts the per-unit survivor invariants.
"""
import json
import os
import re
import subprocess
import sys
import time

rd = sys.argv[1]
R = os.path.relpath(rd)
BASE = '93336e886aee198b345af2a85d0756e0d0124149'
EPOCH = 'Epoch 3 — Determinism and traces'
head = subprocess.run(['git', 'rev-parse', 'HEAD'], capture_output=True, text=True, check=True).stdout.strip()

recs = []
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try:
        recs.append(json.loads(line))
    except ValueError:
        pass
base = [r for r in recs if r['sha'] == BASE][-1]


def c(name):
    return json.load(open(os.path.join(rd, f'c-{name}.json'), encoding='utf-8'))


sizes, cx, dup, graph, dead, cov, churn = (c('sizes'), c('complexity'), c('duplication'), c('graph'),
                                           c('dead'), c('coverage'), c('churn'))
g = graph['graph']
g['fan_in_top'] = [[re.sub(r'^rust-analyzer cargo (\S+) \S+ ', r'\1 ', s), n] for s, n in g['fan_in_top']]

SRC = "-- '*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**'"
MUT_SCOPE = {'dioxus-native-dom': 'files', 'blitz-dom': 'files', 'escher-driver': 'shard 3/5'}
mutation = {'scoped_units': ['dioxus-native-dom', 'blitz-dom', 'escher-driver', 'seven_guis'],
            'unit_states': {}, 'scores': {}, 'counts': {}, 'timing': {},
            'score_formula': 'caught/(caught+missed)', 'survivors': [],
            'host': 'x86_64-unknown-linux-gnu', 'not_measured': []}
commands = {}
skips = [{'metric': 'mutation:seven_guis (examples/seven_guis/src/app.rs: 4 listed, each an `Element` '
                    'return replaced with Default::default())', 'reason': 'declined'}]
for u in ['dioxus-native-dom', 'blitz-dom', 'escher-driver']:
    p = os.path.join(rd, f'c-mutation-{u}.json')
    if not os.path.isfile(p):
        sys.exit(f'missing {p}')
    m = json.load(open(p, encoding='utf-8'))
    hp = json.load(open(os.path.join(rd, 'handpass-' + ('escher-driver' if u == 'escher-driver' else 'broad') + '.json'),
                        encoding='utf-8'))
    commands[f'mutation:{u}'] = (hp['command'] + f' (the unmutated pass, by hand, exit {hp["exit"]}) · '
                                 + m['command'].replace(os.getcwd() + '/', ''))
    st = m['state']
    mutation['unit_states'][u] = st.split(' · ')[0]
    if st == 'baseline-test-failure':
        skips.append({'metric': f'mutation:{u}', 'reason': 'baseline-test-failure'})
        continue
    mutation['counts'][u] = m['counts']
    assert len(m['survivors']) == m['counts']['missed']
    assert len({tuple(s) for s in m['survivors']}) == len(m['survivors'])
    assert len(m['not_measured']) == m['counts']['not_measured']
    mutation['survivors'] += m['survivors']
    mutation['not_measured'] += m['not_measured']
    if st.startswith('complete'):
        mutation['scores'][u] = m['score']
    elif st.startswith('unviable-dominant'):
        skips.append({'metric': f'mutation:{u}', 'reason': 'unviable-dominant'})
    elif st == 'budget-exhausted':
        skips.append({'metric': f'mutation:{u} ({MUT_SCOPE[u]})', 'reason': 'budget-exhausted'})
    if st.startswith('complete') or st == 'budget-exhausted':
        t = {'scope': MUT_SCOPE[u], 'planned': m['planned'], 'tested': m['tested'],
             'wall_s': m['tool_wall_s'] if m.get('tool_wall_s') is not None else m['wall_s'],
             'jobs': 2, 'cap_s': m['cap_s']}
        if MUT_SCOPE[u] == 'files':
            t['files'] = m['files']
        else:
            t['files_listed'] = m['files']
        mutation['timing'][u] = t
skips += [
    {'metric': 'mutation:escher-driver (shards 0/5 · 1/5 · 2/5 · 4/5 of its touched-files listing: 476 of 595)',
     'reason': 'budget-exhausted'},
    {'metric': 'mutation:escher-driver · dioxus-native-dom · blitz-dom (each unit beyond its window-touched files: '
               'escher-driver 100 more of 695 listed whole; dioxus-native-dom and blitz-dom whole listings not taken)',
     'reason': 'declined'},
]
commands['mutation'] = (f'per scoped unit: its `mutation:{{unit}}` entry (the unmutated pass by hand per test set, then '
                        f'cargo mutants on nextest\'s `mutants` profile with --baseline=skip, -j 2, CARGO_BUILD_JOBS=4, '
                        f'taskset -c 12-15,28-31, --timeout 600, TMPDIR=$PWD/target/mutants-tmp), driven sequentially by '
                        f'python3 {R}/mutate.py {R}; test sets in {R}/testsets.json = the baseline record\'s set for the '
                        f'unit (escher-driver; dioxus-native-dom\'s as `broad` for dioxus-native-dom and blitz-dom) + every '
                        f'test target added since {BASE} (stand_replay, stand_session_recording, recording); planned counts '
                        f'from cargo mutants -p {{unit}} {{scope}} --list --json; summarized by python3 {R}/sum_mutation.py')
commands.update({
    'sizes': (f"git ls-files '*.rs' ':!:.andromeda/**' ':!:.claude/**' ':!:scripts/**' ':!:docs/**' ':!:refs/**' > "
              f"{R}/srcfiles.txt && tokei --output json $(cat {R}/srcfiles.txt) > {R}/tokei.json (Rust code lines per "
              f"file; population = srcfiles.txt, {sizes['totals']['files']} files; totals.units = cargo metadata --no-deps "
              f"--format-version 1 --locked workspace_members; summarized by python3 {R}/sum_sizes_cx.py {R}/tokei.json "
              f"{R}/rca {R}/srcfiles.txt {R}/c-; reproduced the baseline record's sizes, totals and complexity exactly on a "
              f"git archive of {BASE})"),
    'complexity': (f"mkdir -p {R}/rca && rust-code-analysis-cli -m -O json -o {R}/rca $(sed 's/^/-p /' {R}/srcfiles.txt) "
                   f"(every space of kind function, closures included: cognitive.sum / cyclomatic.sum; summarized with sizes)"),
    'duplication': (f"jscpd --format rust --absolute --reporters json --output {R}/jscpd-abs packages apps tests examples "
                    f"wpt/runner (paths made repo-relative by python3 {R}/sum_dup.py {R}/jscpd-abs/jscpd-report.json . "
                    f"{R}/c-duplication.json; default min-tokens 50, .gitignore respected; top = 10 largest fragments, ties in "
                    f"report order; split by a `tests` directory segment; reproduced the baseline record exactly on a git "
                    f"archive of {BASE})"),
    'graph': (f"python3 {R}/graph_queries.py {R} (fires scripts/code-graph.py query {R} code-audit \"<audit-pass.md "
              f"canonical cycle / fan-in / fan-out / edge-count SQL, verbatim>\" one at a time; db_state fresh (built.head "
              f"f15c28fcb, no .rs/.toml/.lock change to HEAD); fan-in symbols printed less their 'rust-analyzer cargo "
              f"{{crate}} {{version}}' prefix as '{{crate}} {{path}}'; the SQL as fired is in {R}/tree-query-code-audit.json)"),
    'dead': (f"cargo machete > {R}/machete.out + python3 scripts/code-graph.py query {R} code-audit \"SELECT s.symbol, "
             f"s.crate, s.name, s.kind, s.file, s.def_line FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE "
             f"r.callee IS NULL\" (fired by graph_queries.py) + python3 {R}/sum_dead.py {R} {R}/machete.out (recipes.dead, "
             f"verbatim from the baseline record; def_line is 0-based in the DB, printed +1)"),
    'coverage': ('bash .github/scripts/ci-leg.sh coverage (cargo llvm-cov --workspace --locked --lcov --output-path '
                 'target/coverage/lcov.info && cargo llvm-cov report --workspace --locked; TOTAL row, Lines cover column; '
                 'exit 0)'),
    'churn': (f"git log --reverse --numstat --format=@%H {BASE}..HEAD {SRC} (file-level: adds in a file's 2nd..nth touching "
              f"commit are churn; a rename counts to its new path; python3 {R}/sum_git.py {R} {BASE})"),
    'hotspots': f'churn per-file commit counts ({BASE}..HEAD) x max cognitive per file from complexity (a measured 0 scores 0)',
    'head_overshoot': (f"git log --format=%H -1 -S'2026-10-19-recorded-check-replay · complete' -- .andromeda/master-route.md "
                       f"· git rev-list --count {head}..HEAD · git diff --numstat {head}..HEAD {SRC}"),
})
units = len(json.loads(subprocess.run(['cargo', 'metadata', '--no-deps', '--format-version', '1', '--locked'],
                                      capture_output=True, text=True, check=True).stdout)['workspace_members'])
rec = {
    'ts': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()), 'epoch': EPOCH, 'mode': 'trend', 'sha': head,
    'baseline_sha': BASE, 'span': 1, 'ancestry_broken': False,
    'head_overshoot': {'boundary_sha': head, 'commits': 0, 'files': [],
                       'note': 'at the boundary: HEAD is 2026-10-19-recorded-check-replay\'s complete flip, the last of the '
                               'epoch\'s four markers; no marker of the epoch was ever gated. The baseline record\'s boundary '
                               'state already held this epoch\'s first two chunks (attributed there), so this window holds '
                               'session-recording and recorded-check-replay'},
    'tool_versions': base['tool_versions'],
    'totals': {**sizes['totals'], 'units': units},
    'duplication': dup['duplication'], 'complexity': cx['complexity'], 'sizes': sizes['sizes'], 'graph': g,
    'dead': dead['dead'], 'coverage': cov['coverage'], 'churn': churn['churn'], 'hotspots': churn['hotspots'],
    'mutation': mutation, 'commands': commands, 'recipes': base['recipes'], 'corrections': [], 'skips': skips,
}
json.dump(rec, open(os.path.join(rd, 'record.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print('record assembled', len(json.dumps(rec, ensure_ascii=False)), 'bytes; states', mutation['unit_states'])
