"""Evidence twins for A4 graph, A5 dead code and A6 coverage, from the query outputs in this run dir.

Usage: collect2.py <run_dir> <coverage.log>
"""
import json
import os
import re
import sys

rd, cov_log = sys.argv[1], sys.argv[2]
PREFIX = re.compile(r'^rust-analyzer cargo (\S+) \S+ ')
load = lambda n: json.load(open(os.path.join(rd, n), encoding='utf-8'))


def dump(name, obj):
    with open(os.path.join(rd, name), 'w', encoding='utf-8') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)
        f.write('\n')


# A4 — the trace file holds the cycle query's rows (the first record); the three others were captured per query
trace = load('tree-query-code-audit.json')
cyc = next(r for r in trace if 'WITH RECURSIVE walk' in r['sql'])
cycle_paths = sorted({row['path'] for row in cyc['result']}) if cyc['result'] else []
fan_in = [[PREFIX.sub(lambda m: m.group(1) + ' ', r['callee']), r['n']] for r in load('q-fanin.out')]
fan_out = [[r['from_crate'], r['n']] for r in load('q-fanout.out')]
edges = list(load('q-edges.out')[0].values())[0]
graph = {'cycles': len(cycle_paths), 'cycle_paths': cycle_paths, 'fan_in_top': fan_in[:20], 'fan_out': fan_out,
         'cross_unit_edges': edges, 'db_state': sorted({r['db_state'] for r in trace})}
dump('c-graph.json', graph)

# A5 — cargo-machete's list + the classed zero-ref rows
unused, crate = [], None
for line in open(os.path.join(rd, 'machete.stdout'), encoding='utf-8'):
    m = re.match(r'^(\S+) -- \./\S+Cargo\.toml:$', line.rstrip())
    if m:
        crate = m.group(1)
    elif line.startswith('\t') and crate:
        unused.append([crate, line.strip()])
    elif not line.strip():
        crate = None
dead = {'unused_deps': unused}
dead.update(load('c-dead-class.json'))
dump('c-dead.json', dead)

# A6 — the TOTAL row of `cargo llvm-cov report`: regions(3) functions(3) lines(3) branches(3)
total = [l for l in open(cov_log, encoding='utf-8', errors='replace') if l.startswith('TOTAL')][-1].split()
pct = lambda s: None if s == '-' else float(s.rstrip('%'))
cov = {'line': pct(total[9]), 'branch': pct(total[12]), 'lines': int(total[7]), 'lines_missed': int(total[8]),
       'regions_pct': pct(total[3]), 'functions_pct': pct(total[6]), 'branches': int(total[10]),
       'note': 'workspace TOTAL row, Lines Cover column; branch coverage is not instrumented (0 branches), so null'}
dump('c-coverage.json', cov)
print(json.dumps({'graph': {k: graph[k] for k in ('cycles', 'cross_unit_edges', 'db_state')},
                  'fan_in_top3': fan_in[:3], 'unused_deps': len(unused), 'coverage': cov}, ensure_ascii=False))
