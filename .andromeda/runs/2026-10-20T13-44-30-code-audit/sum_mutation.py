"""Mutation summarizer (collectors.md C1) + the pinned host-excluded cover recipe (verbatim).

summarize(repo, mutants_out_dir, base, stopped) -> per-unit summary dict.
Completion: outcomes.json `end_time` set AND `total_mutants` == len(mutants.json). Counts are
recounted from the outcome list (the Baseline scenario left out: --baseline=skip holds none);
score = caught/(caught+missed), `missed` net of host-excluded mutants. Survivors: one row per
MissedMutant, site file:line:col from its span start, mutation text from missed.txt by that site.
"""
import json, os, re, subprocess
from collections import defaultdict
from datetime import datetime

# ---- pinned recipe (collectors.md §Host-excluded mutants), verbatim ----


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

# ---- end of the pinned recipe ----


def ts(s):
    m = re.match(r'^(.*?\d{2}:\d{2}:\d{2})(\.\d+)?(.*)$', s)
    frac = (m.group(2) or '')[:7]
    tz = m.group(3).replace('Z', '+00:00')
    return datetime.fromisoformat(m.group(1) + frac + tz)


def summarize(repo, mo, base, stopped):
    oc = None
    try:
        oc = json.load(open(f'{mo}/outcomes.json', encoding='utf-8'))
    except (OSError, ValueError):
        pass
    try:
        listed = len(json.load(open(f'{mo}/mutants.json', encoding='utf-8')))
    except (OSError, ValueError):
        listed = None
    res = dict(base)
    if oc is None:
        res['state'] = 'no-outcomes (surfaced)'
        return res
    muts = [o for o in oc.get('outcomes', []) if o.get('scenario') != 'Baseline']
    complete = bool(oc.get('end_time')) and listed is not None and oc.get('total_mutants') == listed
    res['unit_states_line'] = f"{oc.get('total_mutants')}/{listed}"
    by = defaultdict(int)
    for o in muts:
        by[o.get('summary')] += 1
    cfg = host_cfg(repo)
    texts = defaultdict(list)
    try:
        for line in open(f'{mo}/missed.txt', encoding='utf-8'):
            m = re.match(r'^(.+?:\d+:\d+): (.*)$', line.rstrip('\n'))
            if m:
                texts[m.group(1)].append(m.group(2))
    except OSError:
        pass
    surv, nm = [], []
    for o in muts:
        if o.get('summary') != 'MissedMutant':
            continue
        mu = o['scenario']['Mutant']
        site = f"{mu['file']}:{mu['span']['start']['line']}:{mu['span']['start']['column']}"
        text = texts[site].pop(0) if texts[site] else None
        p = cover(repo, mu, cfg)
        (nm.append([site, text, f'cfg({p})']) if p else surv.append([site, text]))
    caught, missed, unviable, timeout = by['CaughtMutant'], len(surv), by['Unviable'], by['Timeout']
    res['counts'] = {'mutants': len(muts), 'caught': caught, 'missed': missed, 'not_measured': len(nm),
                     'timeout': timeout, 'unviable': unviable}
    res['tool_tally'] = {k: oc.get(k) for k in ('total_mutants', 'caught', 'missed', 'timeout', 'unviable', 'success')}
    res['survivors'], res['not_measured'] = surv, nm
    res['host'] = next((v for k, v in cfg if k == 'target_arch'), None)
    if oc.get('start_time') and oc.get('end_time'):
        res['wall_s'] = round((ts(oc['end_time']) - ts(oc['start_time'])).total_seconds())
    else:
        res['wall_s'] = base.get('elapsed_s')
    assert len({(s, t) for s, t in surv}) == len(surv) == missed
    assert missed + len(nm) == by['MissedMutant']
    if complete:
        if unviable > caught + missed + len(nm) + timeout:
            res['state'] = f'unviable-dominant {unviable}/{len(muts)}'
        else:
            res['state'] = f'complete {len(muts)}/{base["planned"]}'
            res['score'] = round(100 * caught / (caught + missed), 2) if caught + missed else None
    elif stopped:
        res['state'] = 'budget-exhausted'
    else:
        res['state'] = f'exited incomplete, exit {base.get("exit")} (surfaced)'
        if muts == [] and oc.get('total_mutants') in (0, None):
            res['state'] = 'baseline-test-failure'
    return res
