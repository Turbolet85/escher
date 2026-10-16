"""Phase 2 driver: one cargo-mutants invocation per scoped unit, sequential, resumable by artifact.

usage: python3 mutate.py {run_dir_rel}     (cwd = repo root)

Per unit: the unmutated hand pass of the unit's exact test set (recorded) -> cargo mutants with
--baseline=skip -> poll; cap = 30-min floor, re-sized ONCE at the floor from the unit's own rate
(planned x elapsed/tested x 1.5); past the cap the tool's process group gets SIGINT and the unit is
budget-exhausted. A tally is read only when the invocation is complete by the tool's markers:
outcomes.json `end_time` set AND `total_mutants` == len(mutants.json). Artifact: c-mutation-{key}.json
(complete units only); every other ending lands in state-{key}.json. Host-excluded mutants per the
collectors.md pinned recipe (copied verbatim below).
"""
import json
import os
import re
import signal
import subprocess
import sys
import time
from datetime import datetime

RD = sys.argv[1]
REPO = os.getcwd()
JOBS = 4
TIMEOUT = 300
FLOOR = 1800
ENV = dict(os.environ, CARGO_BUILD_JOBS='4', RUST_TEST_THREADS='4',
           TMPDIR=os.path.join(REPO, 'target', 'mutants-tmp'))

UNITS = [
    # key, scope, mutants scope args, targets file, listing file
    ('escher-telemetry', 'unit', ['-p', 'escher-telemetry'], 'targets-escher-telemetry.txt', 'unit-escher-telemetry.json'),
    ('blitz-test-harness', 'unit', ['-p', 'blitz-test-harness'], 'targets-blitz-test-harness.txt', 'unit-blitz-test-harness.json'),
    ('seven_guis', 'unit', ['-p', 'seven_guis'], 'targets-seven_guis.txt', 'unit-seven_guis.json'),
    ('dioxus-native-dom', 'unit', ['-p', 'dioxus-native-dom'], 'targets-dioxus-native-dom.txt', 'unit-dioxus-native-dom.json'),
    ('escher-driver', 'unit', ['-p', 'escher-driver'], 'targets-escher-driver.txt', 'unit-escher-driver.json'),
    ('blitz-dom/range-keyboard', 'files',
     ['-p', 'blitz-dom', '--file', 'packages/blitz-dom/src/node/range.rs',
      '--file', 'packages/blitz-dom/src/events/keyboard.rs'],
     'targets-blitz-dom.txt', 'files-blitz-dom-range-keyboard.json'),
]


# ---- collectors.md §Host-excluded mutants, pinned recipe (verbatim) ----
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


def slug(key):
    return key.replace('/', '-')


def log(msg):
    with open(os.path.join(RD, 'mutate-progress.log'), 'a', encoding='utf-8') as f:
        f.write('%s %s\n' % (datetime.utcnow().strftime('%Y-%m-%dT%H:%M:%SZ'), msg))


def read_json(p):
    try:
        return json.load(open(p, encoding='utf-8'))
    except (OSError, ValueError):
        return None


def iso(s):
    return datetime.fromisoformat(s.replace('Z', '+00:00'))


def summary_of(o):
    return o.get('summary')


def mutant_of(o):
    sc = o.get('scenario')
    return sc.get('Mutant') if isinstance(sc, dict) else None


def write_state(key, state):
    json.dump(state, open(os.path.join(RD, 'state-%s.json' % slug(key)), 'w', encoding='utf-8'), indent=1)


def run_unit(key, scope, scope_args, tfile, lfile, cfg):
    s = slug(key)
    if os.path.isfile(os.path.join(RD, 'c-mutation-%s.json' % s)):
        log('%s: artifact present, skipped (resume)' % key)
        return
    st = read_json(os.path.join(RD, 'state-%s.json' % s))
    if st and st.get('state') in ('budget-exhausted', 'baseline-test-failure', 'unviable-dominant'):
        log('%s: state %s present, skipped (resume)' % (key, st['state']))
        return
    lines = [l.strip() for l in open(os.path.join(RD, 'lists', tfile), encoding='utf-8') if l.strip()]
    pkgs, tests = lines[0].split(','), lines[1:]
    listing = json.load(open(os.path.join(RD, 'lists', lfile), encoding='utf-8'))
    planned = len(listing)

    hp = ['cargo', 'test', '--locked'] + sum((['-p', p] for p in pkgs), []) + ['--lib'] + tests
    t0 = time.time()
    with open(os.path.join(RD, 'handpass-%s.log' % s), 'w', encoding='utf-8') as f:
        rc = subprocess.run(hp, env=ENV, stdout=f, stderr=subprocess.STDOUT).returncode
    hand = {'command': 'CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4 ' + ' '.join(hp), 'exit': rc,
            'wall_s': round(time.time() - t0)}
    json.dump(hand, open(os.path.join(RD, 'handpass-%s.json' % s), 'w', encoding='utf-8'), indent=1)
    log('%s: hand pass exit %d in %ds' % (key, rc, hand['wall_s']))
    if rc != 0:
        write_state(key, {'state': 'baseline-test-failure', 'hand': hand})
        return

    out = os.path.join(RD, 'mutants-%s' % s)
    cmd = (['cargo', 'mutants'] + scope_args + ['--test-package=' + ','.join(pkgs), '--cargo-test-arg=--lib']
           + ['--cargo-test-arg=' + t for t in tests]
           + ['--baseline=skip', '--timeout', str(TIMEOUT), '-j', str(JOBS), '--output', out])
    os.makedirs(ENV['TMPDIR'], exist_ok=True)
    start = time.time()
    with open(os.path.join(RD, 'mutants-%s.stdout' % s), 'w', encoding='utf-8') as f:
        proc = subprocess.Popen(cmd, env=ENV, stdout=f, stderr=subprocess.STDOUT, start_new_session=True)
    log('%s: started pid %d, planned %d, floor cap %ds' % (key, proc.pid, planned, FLOOR))
    cap, resized, stopped = FLOOR, False, False
    oj = os.path.join(out, 'mutants.out', 'outcomes.json')
    while True:
        time.sleep(20)
        el = time.time() - start
        o = read_json(oj)
        tested = sum(1 for x in (o or {}).get('outcomes', []) if mutant_of(x)) if o else 0
        if proc.poll() is not None:
            break
        if not resized and el >= FLOOR:
            resized = True
            if tested == 0:
                cap = FLOOR
            else:
                cap = max(FLOOR, int(planned * el / tested * 1.5))
            log('%s: re-sized at %ds from own rate (%d tested of %d): cap %ds' % (key, el, tested, planned, cap))
        if el > cap:
            log('%s: cap %ds spent at %d tested; SIGINT to the process group' % (key, cap, tested))
            os.killpg(proc.pid, signal.SIGINT)
            try:
                proc.wait(timeout=300)
            except subprocess.TimeoutExpired:
                os.killpg(proc.pid, signal.SIGKILL)
                proc.wait()
            stopped = True
            break
    el = round(time.time() - start)
    o = read_json(oj)
    mj = read_json(os.path.join(out, 'mutants.out', 'mutants.json'))
    complete = bool(o and o.get('end_time') and mj is not None and o.get('total_mutants') == len(mj)
                    and not stopped)
    tested = sum(1 for x in (o or {}).get('outcomes', []) if mutant_of(x)) if o else 0
    timing = {'scope': scope, 'planned': planned, 'tested': tested, 'jobs': JOBS, 'cap_s': cap}
    if scope == 'files':
        timing['files'] = sorted({m['file'] for m in listing})
    rec_cmd = ('mkdir -p target/mutants-tmp && TMPDIR=$PWD/target/mutants-tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4 '
               + ' '.join(cmd))
    if not complete:
        timing['wall_s'] = el
        state = 'budget-exhausted' if stopped else 'exited-incomplete'
        if not stopped and o and o.get('total_mutants') == 0:
            state = 'baseline-test-failure'
        write_state(key, {'state': state, 'exit': proc.returncode, 'tested': tested, 'planned': planned,
                          'timing': timing if state == 'budget-exhausted' else None,
                          'hand': hand, 'command': rec_cmd})
        log('%s: ended %s (exit %s, %d/%d tested, %ds)' % (key, state, proc.returncode, tested, planned, el))
        return
    timing['wall_s'] = round((iso(o['end_time']) - iso(o['start_time'])).total_seconds())
    counts = {'mutants': len(mj), 'caught': 0, 'missed': 0, 'not_measured': 0, 'timeout': 0, 'unviable': 0}
    survivors, not_measured, other = [], [], {}

    def ident(m):
        sp = m['span']
        return (m['file'], sp['start']['line'], sp['start']['column'], sp['end']['line'], sp['end']['column'],
                m.get('replacement'), m.get('genre'))
    names = {ident(m): m.get('name') for m in mj}
    for x in o['outcomes']:
        m = mutant_of(x)
        if not m:
            continue
        sm = summary_of(x)
        name = m.get('name') or names.get(ident(m)) or ''
        assert name, (key, 'mutant with no name in outcomes or mutants.json')
        site, _, mutation = name.partition(': ')
        if sm == 'CaughtMutant':
            counts['caught'] += 1
        elif sm == 'MissedMutant':
            p = cover(REPO, m, cfg)
            if p:
                counts['not_measured'] += 1
                not_measured.append([site, mutation, 'cfg(%s)' % p])
            else:
                counts['missed'] += 1
                survivors.append([site, mutation])
        elif sm == 'Timeout':
            counts['timeout'] += 1
        elif sm == 'Unviable':
            counts['unviable'] += 1
        else:
            other[sm] = other.get(sm, 0) + 1
    survivors.sort()
    not_measured.sort()
    assert len({tuple(r) for r in survivors}) == len(survivors) == counts['missed'], key
    assert len({tuple(r) for r in not_measured}) == len(not_measured) == counts['not_measured'], key
    assert counts['missed'] + counts['not_measured'] == o.get('missed'), (key, o.get('missed'))
    dominant = counts['unviable'] > counts['caught'] + counts['missed'] + counts['not_measured'] + counts['timeout']
    score = None if dominant or not (counts['caught'] + counts['missed']) else round(
        100 * counts['caught'] / (counts['caught'] + counts['missed']), 2)
    res = {'unit': key, 'scope': scope, 'complete': True,
           'unit_state': ('unviable-dominant %d/%d' % (counts['unviable'], tested)) if dominant
           else 'complete %d/%d' % (o['total_mutants'], len(mj)),
           'counts': counts, 'score': score, 'score_formula': 'caught/(caught+missed)',
           'tool_tallies': {k: o.get(k) for k in ('total_mutants', 'caught', 'missed', 'timeout', 'unviable', 'success')},
           'other_outcomes': other, 'survivors': survivors, 'not_measured': not_measured,
           'timing': timing, 'hand': hand, 'command': rec_cmd, 'exit': proc.returncode,
           'start_time': o['start_time'], 'end_time': o['end_time']}
    json.dump(res, open(os.path.join(RD, 'c-mutation-%s.json' % s), 'w', encoding='utf-8'), indent=1)
    if dominant:
        write_state(key, {'state': 'unviable-dominant', 'counts': counts})
    log('%s: complete %d/%d · caught %d missed %d not_measured %d timeout %d unviable %d · score %s · %ds'
        % (key, o['total_mutants'], len(mj), counts['caught'], counts['missed'], counts['not_measured'],
           counts['timeout'], counts['unviable'], score, timing['wall_s']))


def main():
    cfg = host_cfg(REPO)
    log('driver start; units %s; jobs %d; timeout %ds' % ([u[0] for u in UNITS], JOBS, TIMEOUT))
    for u in UNITS:
        try:
            run_unit(*u, cfg)
        except Exception as e:  # surfaced, never swallowed into a score
            log('%s: DRIVER ERROR %r' % (u[0], e))
            write_state(u[0], {'state': 'driver-error', 'error': repr(e)})
    log('driver end')


main()
