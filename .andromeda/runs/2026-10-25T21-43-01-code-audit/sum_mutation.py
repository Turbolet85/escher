"""Mutation (C1) summarizer, per unit.

usage: sum_mutation.py RUN_DIR UNIT [UNIT ...]   (run from the repo root)
Completion (cargo-mutants' own markers): outcomes.json top-level end_time set AND total_mutants == len(mutants.json).
Counts from outcomes[].summary; score = caught/(caught+missed); survivors = missed.txt rows (site file:line:col,
mutation text), unique on (site, mutation), their number == missed; a missed mutant whose whole span a #[cfg]
proven false on this host covers -> not_measured (collectors.md pinned cover recipe).
unviable-dominant: unviable > caught + missed + not_measured + timeout.
Writes {run_dir}/c-mutation-{unit}.json.
"""
import json, os, re, subprocess, sys
from datetime import datetime

rd, units = sys.argv[1], sys.argv[2:]
repo = os.getcwd()


# --- collectors.md §Host-excluded mutants, pinned recipe (verbatim) ---
def host_cfg(repo):  # at the repo root: the project's pinned toolchain answers
    out = subprocess.run(['rustc', '--print', 'cfg'], cwd=repo, capture_output=True, text=True, check=True).stdout
    return {(k, v.strip('"') if v else None) for k, _, v in (l.partition('=') for l in out.splitlines())}

def ev(p, cfg):  # three-valued: True / False / None (UNKNOWN)
    p = p.strip()
    m = re.fullmatch(r'(all|any|not)\s*\((.*)\)', p, re.S)
    if m:
        args, depth, cur = [], 0, ''
        for ch in m.group(2):
            depth += (ch == '(') - (ch == ')')
            if ch == ',' and depth == 0: args.append(cur); cur = ''
            else: cur += ch
        vals = [ev(a, cfg) for a in args + [cur] if a.strip()]
        if m.group(1) == 'not': return None if len(vals) != 1 or vals[0] is None else not vals[0]
        if m.group(1) == 'all': return False if False in vals else None if None in vals else True
        return True if True in vals else None if None in vals else False
    m = re.fullmatch(r'([A-Za-z_]\w*)\s*(?:=\s*"([^"]*)")?', p)
    if not m: return None
    if (m.group(1), m.group(2)) in cfg: return True
    return False if m.group(1) in ('unix', 'windows') or m.group(1).startswith('target_') else None

def mask(src):  # comments, strings and char literals blanked; offsets and newlines kept
    out, i, n = list(src), 0, len(src)
    def blank(a, b):
        for k in range(a, b): out[k] = out[k] if src[k] == '\n' else ' '
    while i < n:
        if src.startswith('//', i): j = src.find('\n', i); j = n if j < 0 else j; blank(i, j); i = j
        elif src.startswith('/*', i):
            d, j = 1, i + 2
            while j < n and d: d += src.startswith('/*', j) - src.startswith('*/', j); j += 2 if src[j:j+2] in ('/*', '*/') else 1
            blank(i, j); i = j
        elif m := re.match(r'b?r(#*)"', src[i:i+300]) if (i == 0 or not (src[i-1].isalnum() or src[i-1] == '_')) else None:
            j = src.find('"' + m.group(1), i + m.end()); j = n if j < 0 else j + 1 + len(m.group(1)); blank(i, j); i = j
        elif src[i] == '"':
            j = i + 1
            while j < n and src[j] != '"': j += 2 if src[j] == '\\' else 1
            blank(i, j + 1); i = j + 1
        elif src[i] == "'" and (m := re.match(r"'(\\.[^']*|[^\\'])'", src[i:i+12])): blank(i, i + m.end()); i += m.end()
        else: i += 1
    return ''.join(out)

ITEM = {'pub', 'fn', 'impl', 'mod', 'struct', 'enum', 'union', 'trait', 'unsafe', 'async', 'const', 'static', 'type',
        'use', 'extern', 'macro_rules'}

def extents(src):  # [(start, end, P)]: the attribute .. the close of its node's first top-level `{`, or its `;`
    mk, res = mask(src), []
    for a in re.finditer(r'#(!?)\[\s*cfg\s*\(', mk):
        d, j = 1, a.end()
        while j < len(mk) and d: d += (mk[j] == '(') - (mk[j] == ')'); j += 1
        pred, j = src[a.end():j-1], mk.find(']', j) + 1
        if a.group(1): res.append((0, len(src), pred)); continue
        k = re.match(r'(\s*#\[[^\]]*\])*\s*(\w+)', mk[j:])
        item = bool(k) and k.group(2) in ITEM  # an item ends only at its block or `;` (a `,` may sit in `<…>`)
        d = 0
        while j < len(mk):
            c = mk[j]
            if c in '([': d += 1
            elif c in ')]': d -= 1
            elif d == 0 and c == '{':
                b = 1; j += 1
                while j < len(mk) and b: b += (mk[j] == '{') - (mk[j] == '}'); j += 1
                break
            elif d == 0 and (c == ';' or c == ',' and not item): j += 1; break
            elif d == 0 and c == '}': break
            j += 1
        res.append((a.start(), j, pred))
    return res

def off(src, line, col):  # cargo-mutants: 1-based line, 1-based char column
    starts = [0] + [k + 1 for k, c in enumerate(src) if c == '\n']
    return starts[line - 1] + col - 1

def file_excluded(repo, rel, cfg, seen=()):  # a false predicate on the `mod` bringing the file in
    d, stem = os.path.split(rel)
    name = os.path.basename(d) if stem == 'mod.rs' else stem[:-3]
    if name in ('lib', 'main') or rel in seen: return None
    parent_dir = os.path.dirname(d) if stem == 'mod.rs' else d
    cands = [os.path.join(parent_dir, x) for x in ('lib.rs', 'main.rs', 'mod.rs')] + [parent_dir + '.rs']
    for c in cands:
        try: src = open(os.path.join(repo, c), encoding='utf-8').read()
        except OSError: continue
        m = re.search(r'\bmod\s+' + re.escape(name) + r'\s*;', mask(src))
        if not m: continue
        for s, e, p in extents(src):
            if s <= m.start() and m.end() <= e and ev(p, cfg) is False: return p
        return file_excluded(repo, c.replace('\\', '/'), cfg, seen + (rel,))
    return None

def cover(repo, mutant, cfg):  # the predicate proving the host excludes the WHOLE span, else None
    try: src = open(os.path.join(repo, mutant['file']), encoding='utf-8').read()
    except OSError: return None
    fp = file_excluded(repo, mutant['file'], cfg)
    if fp: return fp
    sp = mutant['span']
    a = off(src, sp['start']['line'], sp['start']['column'])
    b = off(src, sp['end']['line'], sp['end']['column'])
    for s, e, p in extents(src):
        if s <= a and b <= e and ev(p, cfg) is False: return p
    return None
# --- end pinned recipe ---


def ts(s):
    return datetime.fromisoformat(s.replace('Z', '+00:00')) if s else None


cfg = host_cfg(repo)
host = subprocess.run(['rustc', '-vV'], capture_output=True, text=True, check=True).stdout
host = re.search(r'^host: (\S+)', host, re.M).group(1)
for u in units:
    run = json.load(open(f'{rd}/run-{u}.json', encoding='utf-8'))
    res = {'unit': u, 'host': host, 'run': run}
    mo = f'{rd}/mutants-{u}/mutants.out'
    try:
        o = json.load(open(f'{mo}/outcomes.json', encoding='utf-8'))
    except (OSError, ValueError):
        o = None
    try:
        ml = json.load(open(f'{mo}/mutants.json', encoding='utf-8'))
    except (OSError, ValueError):
        ml = None
    if run.get('state') == 'baseline-test-failure':
        res['state'] = 'baseline-test-failure'
    elif o is None:
        res['state'] = 'no-outcomes'
    else:
        complete = bool(o.get('end_time')) and (ml is None or o.get('total_mutants') == len(ml))
        outs = o['outcomes']
        base = [x for x in outs if x['scenario'] == 'Baseline']
        muts = [x for x in outs if x['scenario'] != 'Baseline']
        tally = {}
        for x in muts:
            tally[x['summary']] = tally.get(x['summary'], 0) + 1
        caught, missed = tally.get('CaughtMutant', 0), tally.get('MissedMutant', 0)
        timeout, unviable = tally.get('Timeout', 0), tally.get('Unviable', 0)
        missed_rows = []
        p = f'{mo}/missed.txt'
        if os.path.exists(p):
            for line in open(p, encoding='utf-8'):
                m = re.match(r'^(\S+?:\d+:\d+): (.*)$', line.rstrip('\n'))
                if m:
                    missed_rows.append([m.group(1), m.group(2)])
        missed_objs = [x['scenario']['Mutant'] for x in muts if x['summary'] == 'MissedMutant']
        nm, surv = [], []
        by_site = {}
        for mobj in missed_objs:
            s = mobj['span']['start']
            by_site.setdefault(f"{mobj['file']}:{s['line']}:{s['column']}", []).append(mobj)
        for site, text in missed_rows:
            objs = by_site.get(site, [])
            preds = {cover(repo, x, cfg) for x in objs} if objs else {None}
            if len(preds) == 1 and None not in preds:
                nm.append([site, text, f'cfg({preds.pop()})'])
            else:
                surv.append([site, text])
        assert len(set(map(tuple, surv))) == len(surv), 'duplicate survivor key'
        assert len(surv) + len(nm) == missed == len(missed_rows), (len(surv), len(nm), missed, len(missed_rows))
        counts = {'mutants': o.get('total_mutants'), 'caught': caught, 'missed': len(surv),
                  'not_measured': len(nm), 'timeout': timeout, 'unviable': unviable}
        wall = None
        if o.get('start_time') and o.get('end_time'):
            wall = round((ts(o['end_time']) - ts(o['start_time'])).total_seconds())
        tested = len(muts)
        res.update({'complete': complete, 'baseline_outcome': base[0]['summary'] if base else 'skipped (hand pass)',
                    'counts': counts, 'tool_tally': {k: o.get(k) for k in ('total_mutants', 'caught', 'missed',
                                                                            'timeout', 'unviable', 'success')},
                    'survivors': surv, 'not_measured': nm, 'tested': tested, 'listed': len(ml) if ml else None,
                    'wall_s': wall if wall is not None else run.get('wall_s'),
                    'timeout_sites': []})
        p = f'{mo}/timeout.txt'
        if os.path.exists(p):
            res['timeout_sites'] = [l.rstrip('\n') for l in open(p, encoding='utf-8') if l.strip()]
        if not complete:
            res['state'] = 'budget-exhausted' if run.get('stopped') else 'incomplete-exit'
        elif base and base[0]['summary'] != 'Success' and o.get('total_mutants') == 0:
            res['state'] = 'baseline-test-failure'
        elif unviable > caught + len(surv) + len(nm) + timeout:
            res['state'] = 'unviable-dominant'
        else:
            res['state'] = 'complete'
            den = caught + len(surv)
            res['score'] = round(100 * caught / den, 2) if den else None
        res['unit_state'] = (f"complete {tested}/{o.get('total_mutants')} (total_mutants/mutants.json "
                             f"{o.get('total_mutants')}/{len(ml) if ml else 'absent'})" if res['state'] == 'complete'
                             else res['state'])
    json.dump(res, open(f'{rd}/c-mutation-{u}.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    print(u, res['state'], res.get('score'), res.get('counts'))
