"""Mutation (C1) per-unit summarizer -> RUN_DIR/c-mutation-{unit}.json.

usage: sum_mutation.py RUN_DIR UNIT STATE [WALL_S CAP_S]
STATE: read | budget-exhausted | baseline-test-failure.
Reads {RUN_DIR}/mutants-{UNIT}/mutants.out/ only. Complete = outcomes.json end_time set AND
total_mutants == len(mutants.json). Score = caught/(caught+missed); missed mutants whose whole
span the host's build excludes are `not measured on this host` (collectors.md pinned cover).
unviable-dominant = unviable > caught + missed + not_measured + timeout.
Nothing is written for an invocation that exited incomplete before its cap (surfaced instead).
"""
import json
import os
import re
import subprocess
import sys

rd, unit, state = sys.argv[1:4]
wall = int(sys.argv[4]) if len(sys.argv) > 4 else None
cap = int(sys.argv[5]) if len(sys.argv) > 5 else None
repo = os.getcwd()
mo = os.path.join(rd, f'mutants-{unit}', 'mutants.out')
cmdinfo = json.load(open(os.path.join(rd, f'cmd-mutation-{unit}.json'), encoding='utf-8'))


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
# ---- end pinned recipe ----


def site(m):
    return f"{m['file']}:{m['span']['start']['line']}:{m['span']['start']['column']}"


def text(m):
    n = m.get('name', '')
    return n.split(': ', 1)[1] if ': ' in n else n


res = {'unit': unit, 'command': cmdinfo['command'], 'cap_s': cap, 'wall_s': wall}
if state == 'baseline-test-failure':
    res.update({'state': 'baseline-test-failure'})
else:
    oc = json.load(open(os.path.join(mo, 'outcomes.json'), encoding='utf-8'))
    ml = json.load(open(os.path.join(mo, 'mutants.json'), encoding='utf-8'))
    complete = bool(oc.get('end_time')) and oc.get('total_mutants') == len(ml)
    muts = [o for o in oc['outcomes'] if isinstance(o.get('scenario'), dict) and 'Mutant' in o['scenario']]
    tally = {'caught': 0, 'missed': 0, 'timeout': 0, 'unviable': 0}
    key = {'CaughtMutant': 'caught', 'MissedMutant': 'missed', 'Timeout': 'timeout', 'Unviable': 'unviable'}
    survivors, not_measured = [], []
    cfg = host_cfg(repo)
    for o in muts:
        k = key.get(o['summary'])
        if k is None:
            continue
        m = o['scenario']['Mutant']
        if k == 'missed':
            p = cover(repo, m, cfg)
            if p:
                not_measured.append([site(m), text(m), f'cfg({p})'])
                continue
            survivors.append([site(m), text(m)])
        tally[k] += 1
    tested = sum(tally.values()) + len(not_measured)
    counts = {'mutants': len(ml), 'caught': tally['caught'], 'missed': tally['missed'],
              'not_measured': len(not_measured), 'timeout': tally['timeout'], 'unviable': tally['unviable']}
    assert len({tuple(s) for s in survivors}) == len(survivors) == counts['missed'], 'survivor key coarser than tool'
    assert counts['missed'] + counts['not_measured'] == sum(1 for o in muts if o['summary'] == 'MissedMutant')
    tool_wall = None
    if complete and oc.get('start_time') and oc.get('end_time'):
        from datetime import datetime
        f = lambda s: datetime.fromisoformat(s.replace('Z', '+00:00'))
        tool_wall = round((f(oc['end_time']) - f(oc['start_time'])).total_seconds())
    if state == 'budget-exhausted' and not complete:
        st = 'budget-exhausted'
    elif not complete:
        st = 'incomplete-exit'
    elif tally['unviable'] > tally['caught'] + tally['missed'] + len(not_measured) + tally['timeout']:
        st = f'unviable-dominant {tally["unviable"]}/{tested}'
    else:
        st = f'complete {tested}/{len(ml)}'
    res.update({'state': st, 'counts': counts, 'survivors': survivors, 'not_measured': not_measured,
                'unit_states': f"{st} · total_mutants {oc.get('total_mutants')}/len(mutants.json) {len(ml)}",
                'score': (round(100.0 * tally['caught'] / (tally['caught'] + tally['missed']), 2)
                          if st.startswith('complete') and tally['caught'] + tally['missed'] else None),
                'score_formula': 'caught/(caught+missed)', 'tested': tested, 'planned': len(ml),
                'tool_wall_s': tool_wall, 'start_time': oc.get('start_time'), 'end_time': oc.get('end_time'),
                'files': sorted({m['file'] for m in ml})})
    if st == 'incomplete-exit':
        json.dump(res, open(os.path.join(rd, f'surfaced-mutation-{unit}.json'), 'w', encoding='utf-8'), indent=1)
        print(json.dumps({'unit': unit, 'state': st}))
        sys.exit(0)
json.dump(res, open(os.path.join(rd, f'c-mutation-{unit}.json'), 'w', encoding='utf-8'), indent=1)
print(json.dumps({k: res.get(k) for k in ('unit', 'state', 'counts', 'score')}))
