"""Read-only cross-epoch lookback for Stage 2 / Stage 4 recurrence (keyword screens; the hand read decides).

Run from the project root: python3 -I {run_dir}/lookback.py {scratch_dir}
"""
import json, re, sys, collections

SCR = sys.argv[1]
F = json.load(open(f'{SCR}/facts.json', encoding='utf-8'))
THEMES = {
    'run-bare': r'run bare|bare as|instead of run bare|redirected|through grep|piped|PIPESTATUS|head/tail|into tail',
    'scripted-edit': r'python (heredoc|script|string)|instead of (the )?Edit|exact-once|sed,|by sed|asserted (exact|replacement)',
    'fanout-transcribe': r'transcript|as messages|hand-back|returns arrive',
    'resume-report': r'reuses? (the|its) report|reuse the report|resumed report|report as is',
    'relay-snap': r'relay|inputs\.py snap',
    'founder-forks': r'founder|forks? ',
    'p1-only': r'P1 only|P1 and a stop|after P1|P2 to P7',
    'ledger-parse': r'ledger|evolve nudge',
    'long-line': r'long line|clips a long|splitter|first \d+ characters|-char line',
    'citation-aid': r'citation|re-point|reading aid',
    'hygiene': r'hygiene|host path|temp-dir path|home path',
}
out = {}
for k, pat in THEMES.items():
    rows = [f for f in F if f['note'] and re.search(pat, f['note'], re.I)]
    out[k] = {'by_epoch': dict(collections.Counter(f['epoch'] for f in rows)),
              'rows': [[f['epoch'][:7], f['id'], f['i'], f['step'], f['solution'], f['nature'], f['note'][:200]] for f in rows]}
deg = json.load(open(f'{SCR}/degraded.json', encoding='utf-8'))
out['ok-degraded'] = {'by_epoch': dict(collections.Counter(d['epoch'] for d in deg)), 'rows': deg}
# tooling.* friction across epochs (from ledger)
norm = lambda s: re.sub(r'^\s*#+\s*', '', (s or '')).strip().replace('–', '-').replace('—', '-')
tool = collections.Counter(); unt_kw = collections.defaultdict(list)
UNT = {'pattern-flag': r'--pattern', 'citation': r'citation|cites\.py|re-point', 'relay': r'relay|dialog',
       'count-first-write': r'count|off by one|line citations|cited as'}
for line in open('.andromeda/friction-log.ndjson', encoding='utf-8'):
    try: r = json.loads(line)
    except ValueError: continue
    if r.get('kind') != 'friction': continue
    t = r.get('type') or ''
    if t.startswith('tooling.'):
        tool[(t, norm(r.get('epoch'))[:7])] += 1
    if r.get('untyped') or not t:
        for k, pat in UNT.items():
            if re.search(pat, r.get('what') or '', re.I):
                unt_kw[k].append([norm(r.get('epoch'))[:7], r.get('id'), (r.get('what') or '')[:160]])
out['tooling_by_epoch'] = sorted([[k[0], k[1], v] for k, v in tool.items()])
out['untyped_keyword_lookback'] = unt_kw
print(json.dumps(out, ensure_ascii=False, indent=1))
