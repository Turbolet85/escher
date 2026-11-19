"""Mutation (C1) summarizer. usage: python3 sum_mutation.py {run_dir} {repo} {unit}...
Reads {run_dir}/mutants-{unit}/mutants.out/ only. Complete = outcomes.json end_time set AND total_mutants ==
len(mutants.json). Tallies by each mutant outcome's summary; a MissedMutant whose whole span a host-false #[cfg] covers is
not_measured (collectors.md pinned cover recipe), never a survivor. Score = caught/(caught+missed).
unviable-dominant = unviable > caught + missed + not_measured + timeout. Writes c-mutation-{unit}.json per unit read.
"""
import json, os, re, subprocess, sys

rd, repo = sys.argv[1], sys.argv[2]
units = sys.argv[3:]

# ---- pinned cover recipe (collectors.md §Host-excluded mutants), verbatim ----
def host_cfg(repo):
    out = subprocess.run(['rustc', '--print', 'cfg'], cwd=repo, capture_output=True, text=True, check=True).stdout
    return {(k, v.strip('"') if v else None) for k, _, v in (l.partition('=') for l in out.splitlines())}

def ev(p, cfg):
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

def mask(src):
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

def extents(src):
    mk, res = mask(src), []
    for a in re.finditer(r'#(!?)\[\s*cfg\s*\(', mk):
        d, j = 1, a.end()
        while j < len(mk) and d: d += (mk[j] == '(') - (mk[j] == ')'); j += 1
        pred, j = src[a.end():j-1], mk.find(']', j) + 1
        if a.group(1): res.append((0, len(src), pred)); continue
        k = re.match(r'(\s*#\[[^\]]*\])*\s*(\w+)', mk[j:])
        item = bool(k) and k.group(2) in ITEM
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

def off(src, line, col):
    starts = [0] + [k + 1 for k, c in enumerate(src) if c == '\n']
    return starts[line - 1] + col - 1

def file_excluded(repo, rel, cfg, seen=()):
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

def cover(repo, mutant, cfg):
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
# ---- end pinned recipe ----

cfg = host_cfg(repo)
for u in units:
    mo = f'{rd}/mutants-{u}/mutants.out'
    try:
        o = json.load(open(f'{mo}/outcomes.json', encoding='utf-8'))
    except (OSError, ValueError):
        print(u, 'no outcomes.json'); continue
    try:
        planned = len(json.load(open(f'{mo}/mutants.json', encoding='utf-8')))
    except (OSError, ValueError):
        planned = None
    complete = bool(o.get('end_time')) and (planned is None or o.get('total_mutants') == planned)
    c = {'mutants': 0, 'caught': 0, 'missed': 0, 'not_measured': 0, 'timeout': 0, 'unviable': 0}
    surv, nm, base = [], [], None
    for x in o['outcomes']:
        sc = x['scenario']
        if sc == 'Baseline':
            base = x['summary']; continue
        m = sc['Mutant']
        c['mutants'] += 1
        s = x['summary']
        site = f"{m['file']}:{m['span']['start']['line']}:{m['span']['start']['column']}"
        fn = (m.get('function') or {}).get('function_name')
        text = m['name'].split(': ', 1)[1] if ': ' in m.get('name', '') else m['replacement']  # the tool's own text
        if s == 'CaughtMutant': c['caught'] += 1
        elif s == 'MissedMutant':
            p = cover(repo, m, cfg)
            if p: c['not_measured'] += 1; nm.append([u, site, m['replacement'] if False else text, f'cfg({p})'])
            else: c['missed'] += 1; surv.append([u, site, text])
        elif s == 'Timeout': c['timeout'] += 1
        elif s == 'Unviable': c['unviable'] += 1
        else: c.setdefault('other', 0); c['other'] += 1
    tested = o.get('total_mutants')
    tool_missed = o.get('missed')
    ud = c['unviable'] > c['caught'] + c['missed'] + c['not_measured'] + c['timeout']
    den = c['caught'] + c['missed']
    score = round(100 * c['caught'] / den, 1) if den and complete and not ud else None
    keys = [(s[1], s[2]) for s in surv]
    assert len(keys) == len(set(keys)) == c['missed'], (u, 'survivor key coarser than the tool')
    assert c['missed'] + c['not_measured'] == (tool_missed if tool_missed is not None else c['missed'] + c['not_measured'])
    res = {'unit': u, 'complete': complete, 'planned': planned, 'tested': tested, 'baseline': base, 'counts': c,
           'score': score, 'unviable_dominant': ud, 'survivors': surv, 'not_measured': nm,
           'start_time': o.get('start_time'), 'end_time': o.get('end_time'), 'score_formula': 'caught/(caught+missed)'}
    json.dump(res, open(f'{rd}/c-mutation-{u}.json', 'w', encoding='utf-8'), indent=1)
    print(u, 'complete' if complete else 'INCOMPLETE', f'{tested}/{planned}', c, 'score', score, 'ud' if ud else '')
