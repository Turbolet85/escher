"""Graph (A4) + churn (B1) + hotspots (B2) summarizer.

usage: python3 sum_graph_churn.py {run_dir} {numstat.txt}
numstat.txt = git log --reverse --numstat --format=@%H {baseline}..HEAD -- {source paths}
churn: per file, adds in its 2nd..nth touching commit are churn; churn % = churned adds / all adds;
binary `-` rows skipped; a rename counts to its new path.
hotspots: per-file commit count x max cognitive per file (c-file-maxcog.json); a measured 0 scores 0.
"""
import json
import os
import re
import sys

PREFIX = re.compile(r'^rust-analyzer cargo (\S+) \S+ ')


def short(sym):
    return PREFIX.sub(lambda m: m.group(1) + ' ', sym)


def rename(p):
    m = re.match(r'^(.*)\{(.*) => (.*)\}(.*)$', p)
    if m:
        return re.sub('//+', '/', m.group(1) + m.group(3) + m.group(4))
    if ' => ' in p:
        return p.split(' => ')[1]
    return p


def main():
    rd, ns = sys.argv[1:3]
    cyc = json.load(open(os.path.join(rd, 'q-cycles.json')))
    fin = json.load(open(os.path.join(rd, 'q-fanin.json')))
    fout = json.load(open(os.path.join(rd, 'q-fanout.json')))
    edges = json.load(open(os.path.join(rd, 'q-edges.json')))
    graph = {'cycles': len(cyc), 'cycle_paths': [c['path'] for c in cyc],
             'fan_in_top': [[short(x['callee']), x['n']] for x in fin],
             'fan_out': [[x['from_crate'], x['n']] for x in fout],
             'cross_unit_edges': list(edges[0].values())[0]}
    json.dump(graph, open(os.path.join(rd, 'c-graph.json'), 'w', encoding='utf-8'), indent=1)

    touches, adds_all, churned = {}, 0, 0
    commits = 0
    for raw in open(ns, encoding='utf-8'):
        l = raw.rstrip('\n')
        if l.startswith('@'):
            commits += 1
            continue
        parts = l.split('\t')
        if len(parts) != 3 or parts[0] == '-':
            continue
        a, path = int(parts[0]), rename(parts[2])
        n = touches.get(path, 0)
        adds_all += a
        if n >= 1:
            churned += a
        touches[path] = n + 1
    files_churned = sum(1 for v in touches.values() if v >= 2)
    churn = {'pct': round(100 * churned / adds_all, 2) if adds_all else None,
             'files_churned': files_churned, 'commits': commits, 'adds': adds_all,
             'churned_adds': churned, 'files_touched': len(touches),
             'per_file_commits': dict(sorted(touches.items(), key=lambda x: (-x[1], x[0])))}
    json.dump(churn, open(os.path.join(rd, 'c-churn.json'), 'w', encoding='utf-8'), indent=1)

    maxcog = json.load(open(os.path.join(rd, 'c-file-maxcog.json')))
    hs = sorted(((f, n * maxcog[f]) for f, n in touches.items() if f in maxcog),
                key=lambda x: (-x[1], x[0]))[:10]
    json.dump([list(h) for h in hs], open(os.path.join(rd, 'c-hotspots.json'), 'w', encoding='utf-8'), indent=1)
    print(json.dumps({'graph': {k: graph[k] for k in ('cycles', 'cross_unit_edges')},
                      'churn': {k: churn[k] for k in ('pct', 'files_churned', 'commits', 'adds', 'files_touched')},
                      'hotspots': hs}))


main()
