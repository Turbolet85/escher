"""C1 mutation summary for one unit -> c-mutation-{unit}.json (written only for a COMPLETE invocation).

Usage: mut_sum.py {run_dir} {unit}
Completion: outcomes.json end_time set AND total_mutants == len(mutants.json). Host-excluded mutants per
collectors.md §Host-excluded mutants (pinned recipe, verbatim below).
"""
import json, os, re, subprocess, sys

RD, UNIT = sys.argv[1], sys.argv[2]
REPO = os.getcwd()
M = f"{RD}/mutants-{UNIT}/mutants.out"


# ---- pinned recipe (collectors.md) ----
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

o = json.load(open(f"{M}/outcomes.json", encoding="utf-8"))
planned = len(json.load(open(f"{M}/mutants.json", encoding="utf-8")))
if not o.get("end_time"):
    sys.exit("INCOMPLETE: end_time unset — not read")
has_base = o["outcomes"] and o["outcomes"][0]["scenario"] == "Baseline"  # absent under --baseline=skip
base_summary = o["outcomes"][0]["summary"] if has_base else "skipped (--baseline=skip; unmutated tree probed by hand)"
if o.get("total_mutants", 0) != planned:
    sys.exit(f"INCOMPLETE/odd: total_mutants={o.get('total_mutants')} planned={planned} base={base_summary} — surface and re-run")
cfg = host_cfg(REPO)
triple = subprocess.run(["rustc", "-vV"], capture_output=True, text=True).stdout
triple = re.search(r"host: (\S+)", triple).group(1)

counts = {"mutants": planned, "caught": 0, "missed": 0, "not_measured": 0, "timeout": 0, "unviable": 0}
survivors, not_measured = [], []
for oc in o["outcomes"][1 if has_base else 0:]:
    mu = oc["scenario"]["Mutant"]
    site = f"{mu['file']}:{mu['span']['start']['line']}:{mu['span']['start']['column']}"
    name = re.sub(r"^\S+:\d+:\d+: ", "", mu.get("name") or "")
    s = oc["summary"]
    if s == "CaughtMutant":
        counts["caught"] += 1
    elif s == "MissedMutant":
        p = cover(REPO, mu, cfg)
        if p:
            counts["not_measured"] += 1
            not_measured.append([site, name, f"cfg({p})"])
        else:
            counts["missed"] += 1
            survivors.append([site, name])
    elif s == "Timeout":
        counts["timeout"] += 1
    elif s == "Unviable":
        counts["unviable"] += 1
    else:
        sys.exit(f"unknown summary {s}")

missed_txt = [l for l in open(f"{M}/missed.txt", encoding="utf-8").read().splitlines() if l.strip()]
assert counts["missed"] + counts["not_measured"] == len(missed_txt), (counts, len(missed_txt))
assert len({(a, b) for a, b in survivors}) == len(survivors) == counts["missed"], "survivor key coarser than tool's"
tested = counts["caught"] + counts["missed"] + counts["not_measured"] + counts["timeout"]
state = f"complete {o['total_mutants']}/{planned}"
score = None
if counts["unviable"] > tested:
    state = f"unviable-dominant {counts['unviable']}/{counts['unviable'] + tested}"
elif counts["caught"] + counts["missed"]:
    score = round(100 * counts["caught"] / (counts["caught"] + counts["missed"]), 2)
out = {"unit": UNIT, "unit_state": state, "score": score, "counts": counts, "score_formula": "caught/(caught+missed)",
       "survivors": survivors, "not_measured": not_measured, "host": triple,
       "missed_txt_rows": len(missed_txt), "total_mutants": o["total_mutants"], "mutants_json_len": planned,
       "baseline_outcome": base_summary,
       "union_verdict": "no project union verdict"}
json.dump(out, open(f"{RD}/c-mutation-{UNIT}.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps({k: out[k] for k in ("unit_state", "score", "counts")}), len(survivors), "survivors")
for s in survivors:
    print("  ", s)
