"""Pinned summarizers: A5 unused deps (cargo-machete), A6 coverage (ci-leg.sh coverage), C1 mutation (cargo-mutants).

Run from the repo root: python3 .andromeda/runs/2026-10-06T05-17-25-code-audit/summarize2.py [mutation]
"""
import json, os, re, sys

RD = '.andromeda/runs/2026-10-06T05-17-25-code-audit'
REPO = os.getcwd()


def dump(name, obj):
    with open(f'{RD}/c-{name}.json', 'w', encoding='utf-8') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)


# ---------- A5 unused deps (cargo-machete stdout) ----------
unused, crate = [], None
for line in open(f'{RD}/machete.stdout', encoding='utf-8'):
    m = re.match(r'^(\S+) -- (\S+Cargo\.toml):$', line.rstrip())
    if m:
        crate = m.group(1)
        continue
    if crate and line.startswith('\t') and line.strip():
        unused.append([crate, line.strip()])
    elif not line.strip():
        crate = None
dead = json.load(open(f'{RD}/c-dead.json', encoding='utf-8'))
dead['unused_deps'] = unused
dead['unused_deps_note'] = ('cargo-machete reads source text, not resolution: a dep kept only to pin a feature or a '
                            'version (e.g. idna_adapter) or used only through a macro reads unused — candidates, never verdicts')
dump('dead', dead)

# ---------- A6 coverage (llvm-cov report TOTAL row) ----------
tot = [l for l in open(f'{RD}/coverage.stdout', encoding='utf-8') if l.startswith('TOTAL')]
f = tot[-1].split()
# TOTAL regions missed cover% functions missed cover% lines missed cover% branches missed cover
lines_total, lines_missed = int(f[7]), int(f[8])
br_total = int(f[10])
cov = {
    'line': round(100 * (lines_total - lines_missed) / lines_total, 2), 'branch': None if br_total == 0 else float(f[12].rstrip('%')),
    'lines_total': lines_total, 'lines_missed': lines_missed,
    'regions_pct': float(f[3].rstrip('%')), 'functions_pct': float(f[6].rstrip('%')),
    'note': 'branch null: llvm-cov reports 0 branches (branch instrumentation not enabled by the leg)',
}
dump('coverage', cov)

# ---------- C1 mutation (cargo-mutants, completion by the tool's own markers) ----------
if 'mutation' in sys.argv:
    sys.path.insert(0, RD)
    from cover import host_cfg, cover  # the pinned collectors.md §Host-excluded recipe, copied verbatim
    unit = 'escher-telemetry'
    mo = f'{RD}/mutants-{unit}/mutants.out'
    oc = json.load(open(f'{mo}/outcomes.json', encoding='utf-8'))
    planned = json.load(open(f'{mo}/mutants.json', encoding='utf-8'))
    assert oc.get('end_time'), 'invocation incomplete: end_time unset'
    assert oc['total_mutants'] == len(planned), 'total_mutants != len(mutants.json)'
    base = next((o for o in oc['outcomes'] if o['scenario'] == 'Baseline'), None)  # absent under --baseline=skip
    muts = [o for o in oc['outcomes'] if isinstance(o['scenario'], dict) and 'Mutant' in o['scenario']]
    cfg = host_cfg(REPO)
    counts = {'mutants': oc['total_mutants'], 'caught': 0, 'missed': 0, 'not_measured': 0, 'timeout': 0, 'unviable': 0}
    survivors, not_measured = [], []
    for o in muts:
        s = o['summary']
        mut = o['scenario']['Mutant']
        site = f"{mut['file']}:{mut['span']['start']['line']}:{mut['span']['start']['column']}"
        label = mut['name'].split(': ', 1)[1]  # the tool's own text for the replacement
        key = {'CaughtMutant': 'caught', 'MissedMutant': 'missed', 'Timeout': 'timeout', 'Unviable': 'unviable'}[s]
        if key == 'missed':
            p = cover(REPO, mut, cfg)
            if p:
                counts['not_measured'] += 1
                not_measured.append([site, label, f'cfg({p})'])
                continue
            survivors.append([site, label])
        counts[key] += 1
    tested = counts['caught'] + counts['missed']
    rec = {
        'unit': unit, 'baseline_outcome': base['summary'] if base else 'skipped (--baseline=skip; the unmutated narrowed build was verified by hand and the coverage leg ran the suite green at this sha)', 'counts': counts,
        'score': round(100 * counts['caught'] / tested, 2) if tested else None,
        'score_formula': 'caught/(caught+missed)',
        'unit_state': f"complete {len(muts)}/{len(planned)}",
        'survivors': survivors, 'not_measured': not_measured,
        'host': os.popen('rustc -vV').read().split('host: ')[1].split()[0],
        'missed_total_tool': sum(1 for o in muts if o['summary'] == 'MissedMutant'),
        'union_verdict': 'no project union verdict',
    }
    assert len({tuple(s) for s in survivors}) == len(survivors) == counts['missed']
    assert counts['missed'] + counts['not_measured'] == rec['missed_total_tool']
    dump(f'mutation-{unit}', rec)
    print(json.dumps(rec, indent=1))

print(json.dumps({'unused_deps': len(unused), 'coverage': cov}, indent=0))
