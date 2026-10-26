"""P5: render {run_dir}/proposals.md from record.json + judge.json + the evidence twins, with the evidence-table
row-count and uniqueness asserts (proposal-template.md: n from a source independent of the row list).

usage: render.py RUN_DIR   (run from the repo root)
"""
import json, sys

rd = sys.argv[1]
J = lambda n: json.load(open(f'{rd}/{n}', encoding='utf-8'))
rec, jd, oc = J('record.json'), J('judge.json'), J('c-over-ceiling.json')
scope = J('mutation-scope.json')
cx = J('c-complexity.json')
m = rec['mutation']
base_sha, base_ep = jd['baseline']['sha'], jd['baseline']['epoch']
out = []
w = out.append


def table(header, rows, n, key):
    assert len(rows) == n, f'{header}: {len(rows)} rows, n {n}'
    keys = [key(r) for r in rows]
    assert len(set(keys)) == len(keys), f'{header}: duplicate natural key'
    w('| ' + ' | '.join(header) + ' |')
    w('|' + '---|' * len(header))
    for r in rows:
        w('| ' + ' | '.join(str(c) for c in r) + ' |')
    w('')


w(f"# Code Audit — escher · {rec['epoch']} · {rec['ts']}")
w(f"mode {rec['mode']} · HEAD {rec['sha']} · baseline {base_sha} ({base_ep}) · span {rec['span']}")
w(f"Overshoot: this run 0 commits (HEAD is the boundary, `{rec['head_overshoot']['boundary_sha'][:10]}`); "
  f"the baseline's 0 (at its boundary). Ancestry OK. No trend-break: every tool's leading version token equals the "
  f"baseline's ({', '.join(f'{k} {v}' for k, v in rec['tool_versions'].items())}).")
w(f"The baseline's sizes, totals, complexity, duplication and dead-code figures were reproduced exactly by this run's "
  f"summarizers on a git archive of `{base_sha[:10]}` (dead: same 115 count, same top-10, same 26 unused deps), so every "
  f"delta below is one population against itself. Ledger: {jd['ledger_records']} parseable records after this run's append.")
w('')
w('## Proposals')
w('')
P = jd['proposals']
pn = 0
for p in P:
    pn += 1
    if p['check'] == 'monotonic' and p['metric'] == 'complexity.over_ceiling':
        a, b, c = p['values']
        e34, e45 = oc['E3->E4'], oc['E4->E5']
        w(f"### M{pn} — monotonic · complexity.over_ceiling — {a} → {b} → {c}")
        w(f"**Movement:** {a} → {b} → {c} (functions over cognitive 15; {p['epochs'][0]} → {p['epochs'][1]} → "
          f"{p['epochs'][2]}) — worse at both of the last two diffs, +{b - a} then +{c - b}. Neither single diff "
          f"fires complexity-creep (≥ +3 AND ≥ +25 %).")
        w(f"**Evidence:** the functions over the ceiling at each record's sha, re-read by rust-code-analysis 0.0.25 on git "
          f"archives of `7eff48f7aa`, `{base_sha[:10]}` and HEAD, matched by (file, name, occurrence) with the upstream "
          f"sync's rename `layout/inline.rs → layout/inline/layout.rs` mapped (`c-over-ceiling.json`). Entered / left "
          f"per diff, the value before in the last column (`—` = the function did not exist in that file):")
        w('')
        w(f"{p['epochs'][0]} → {p['epochs'][1]} ({e34['count'][0]} → {e34['count'][1]}):")
        w('')
        rows = [['+', f'`{x[0]}:{x[2]}`', f'`{x[1]}`', x[3], '—' if x[4] is None else x[4]] for x in e34['entered']] + \
               [['−', f'`{x[0]}:{x[2]}`', f'`{x[1]}`', x[3], '—' if x[4] is None else x[4]] for x in e34['left']]
        table(['', 'site', 'fn', 'cognitive', 'other side'], rows, len(e34['entered']) + len(e34['left']),
              lambda r: (r[0], r[1], r[2]))
        w(f"{p['epochs'][1]} → {p['epochs'][2]} ({e45['count'][0]} → {e45['count'][1]}):")
        w('')
        rows = [['+', f'`{x[0]}:{x[2]}`', f'`{x[1]}`', x[3], '—' if x[4] is None else x[4]] for x in e45['entered']] + \
               [['−', f'`{x[0]}:{x[2]}`', f'`{x[1]}`', x[3], '—' if x[4] is None else x[4]] for x in e45['left']]
        table(['', 'site', 'fn', 'cognitive', 'other side'], rows, len(e45['entered']) + len(e45['left']),
              lambda r: (r[0], r[1], r[2]))
        w("Read net of moves: Epoch 3 → 4 is three real crossings — `clear_interaction_state_for_removed_node` 11 → 17 "
          "(blitz-dom `document.rs`), and two functions new above it, `escher-driver` `lint.rs::findings` (16) and the "
          "`mcp_tests.rs` listing test (18); the `mutator.rs` `<anonymous>` pair is one closure re-keyed by a line move "
          "(19 → 22, over the ceiling on both sides). Epoch 4 → 5 is one real crossing — the "
          "`stand_requirement_sweep.rs` test `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` (18, a "
          "new file section of +482 lines); `build_inline_layout_into` (58) and `build_inline_layout_recursive` (51) "
          "moved unchanged from `layout/construct.rs` to `layout/inline/build.rs` in the upstream sync's split and are "
          "no growth.")
        w('')
        w("**Suspected shape:** the creep is small crossings just over the ceiling (16–18), half of them in test "
          "functions (`mcp_tests.rs`, `stand_requirement_sweep.rs`) that assert a table row by row inline; the "
          "upstream engine's large offenders (max `compute_inline_layout_inner` 139) hold still.")
        w("**Proposal:** a direction for the founder's judgment — where an escher-written function crosses 15, factor "
          "its row-by-row assertion or branch ladder into a table walk or a named helper (the two test functions and "
          "`lint.rs::findings` are escher's own; `clear_interaction_state_for_removed_node` is a fork change to an "
          "engine function); whether test functions belong in this population at all is a measurement question the "
          "ledger's recipe does not settle today.")
        w('')
    else:
        w(f"### M{pn} — {p['check']} · {p.get('metric', p.get('unit'))}")
        w('```')
        w(json.dumps(p, ensure_ascii=False))
        w('```')
        w('')
if not P:
    w('None fired.')
    w('')

w('## Informational')
w('')
for x in jd['informational']:
    if x['check'] == 'top-N entrants':
        ents = x['entrants']
        if x['list'] == 'complexity.top':
            w(f"- top-N entrants · complexity.top — {len(ents)}: " + '; '.join(f"`{e[0]}` `{e[1]}` ({e[2]})" for e in ents) +
              " — the first two by the upstream sync's file moves (`layout/inline.rs` → `layout/inline/layout.rs`, "
              "`build_inline_layout_into` out of `layout/construct.rs`), values unchanged; `compute_replaced_layout` "
              "55 → 56 (an upstream edit of `layout/replaced.rs`, +13/−4)")
        elif x['list'] == 'hotspots':
            w(f"- top-N entrants · hotspots — {len(ents)}: " + '; '.join(f"`{e[0]}` ({e[1]})" for e in ents) +
              " — the window's 3 source commits are the operator pre-CI commits; the entrants are the upstream sync's "
              "layout split and its browser/wpt edits")
        elif x['list'] == 'graph.fan_in_top':
            w(f"- top-N entrants · graph.fan_in_top — {len(ents)}: " + '; '.join(f"`{e[0]}` ({e[1]})" for e in ents))
        elif x['list'] == 'sizes.top':
            w(f"- top-N entrants · sizes.top — {len(ents)}: " + '; '.join(f"`{e[0]}` ({e[1]})" for e in ents) +
              " — `stand_requirement_sweep.rs` 553 → 1007 (the 2026-10-24-stand-requirement-sweep chunk, +482/−5; the "
              "one new file over 800, so `sizes.over_800` 17 → 18); `layout/inline/layout.rs` the moved "
              "`layout/inline.rs` (925 → 927); `node/element.rs` 916 → 927 (an upstream edit, +17)")
        elif x['list'] == 'duplication.top':
            w(f"- top-N entrants · duplication.top — {len(ents)}: " + '; '.join(f"`{e[0]}` × `{e[1]}` ({e[2]} L)" for e in ents))
        else:
            w(f"- top-N entrants · {x['list']} — {len(ents)}: " + '; '.join(f'`{e}`' for e in ents))
    elif x['check'] == 'mutation-drop':
        w(f"- mutation · {x['unit']} — not comparable: {x['now']} at scope {x['now_scope'][0]} (this window's diff, "
          f"{x['now_scope'][2]} planned, {rec['epoch']}) · {x['was']} at scope {x['was_scope'][0] if x['was_scope'] else 'UNKNOWN'} "
          f"({x['was_scope'][2] if x['was_scope'] else '?'} planned, {x['was_epoch']}) — two different windows' diffs, and "
          f"`in-diff` is none of the schema's three scopes, so no delta")
    elif x['check'] == 'mutation':
        w(f"- mutation · {x['unit']} — {x['note']}" + (f": {x['now']} at scope {x['scope'][0]} ({x['scope'][2]} planned)"
                                                       if 'now' in x else ''))
    elif x['check'] == 'count-under-ratio':
        w(f"- count-under-ratio · duplication — clones {x['clones'][0]} → {x['clones'][1]}, lines "
          f"{x['duplicated_lines'][0]} → {x['duplicated_lines'][1]}, pct {x['pct'][0]} → {x['pct'][1]}, population "
          f"{x['population_rel']:+} %")
    else:
        w(f"- {json.dumps(x, ensure_ascii=False)}")
cur_dup = rec['duplication']
sp0 = jd['below'][[i for i, b in enumerate(jd['below']) if b['check'] == 'count-under-ratio'][0]]['split'][0]
w(f"- churn · {rec['churn']['pct']} % over 3 source commits, {rec['churn']['files_churned']} files churned (baseline "
  f"13.7 %, 21) — the window's only `.rs`-touching commits are its three operator pre-CI commits")
w('')

w('### Mutation tier — complete per unit')
w('')
for u in m['scoped_units']:
    c = m['counts'].get(u)
    cm = J(f'c-mutation-{u}.json')
    t = m['timing'].get(u) or {'wall_s': f"{cm['wall_s']} (not in `mutation.timing`: {cm['state']})",
                               'cap_s': cm['run']['cap_s']}
    w(f"**{u}** — {m['unit_states'][u]} · score {m['scores'].get(u, 'none')} "
      f"({m['score_formula']}) · counts {json.dumps(c) if c else '—'} · wall {t.get('wall_s', '—')} s of cap "
      f"{t.get('cap_s', '—')} s · jobs 2 · scope in-diff · the founder's «{scope['units'][u]['answer']}»")
    w('')
    surv = [s for s in m['survivors'] if s[0].startswith(
        {'blitz-dom': 'packages/blitz-dom/', 'seven_guis': 'examples/seven_guis/', 'wpt': 'wpt/'}[u])]
    if c:
        if c['missed']:
            w(f"Survivors ({c['missed']}):")
            w('')
            table(['site', 'mutation'], [[f'`{s[0]}`', s[1]] for s in surv], c['missed'], lambda r: (r[0], r[1]))
        nm = [s for s in m['not_measured'] if s[0].startswith(
            {'blitz-dom': 'packages/blitz-dom/', 'seven_guis': 'examples/seven_guis/', 'wpt': 'wpt/'}[u])]
        if c['not_measured']:
            w(f"Not measured on this host ({m['host']}) ({c['not_measured']}):")
            w('')
            table(['site', 'mutation', 'predicate'], [[f'`{s[0]}`', s[1], f'`{s[2]}`'] for s in nm], c['not_measured'],
                  lambda r: (r[0], r[1]))
            w('Project union verdict: `.andromeda/test-plan.md` §10 (the mutation tier, cargo-mutants) — named by '
              'pointer, never merged.')
            w('')
        tm = J(f'c-mutation-{u}.json').get('timeout_sites') or []
        if tm:
            w(f"Timeouts ({len(tm)}): " + '; '.join(f'`{s}`' for s in tm))
            w('')
w('')

w('## Below threshold — no action')
w('')
for b in jd['below']:
    if b['check'] == 'monotonic':
        w(f"- monotonic · {b['metric']} — {' → '.join(str(v) for v in b['values'])} (not worse at both diffs)")
    elif b['check'] == 'count-under-ratio':
        w(f"- duplication volume — clones {b['clones'][0]} → {b['clones'][1]}, lines {b['duplicated_lines'][0]} → "
          f"{b['duplicated_lines'][1]}, pct {b['pct'][0]} → {b['pct'][1]} (rose with its counts, so not count-under-ratio), "
          f"population {b['population_rel']:+} %; split src {sp0['src']['pairs']} → {cur_dup['split']['src']['pairs']} pairs · "
          f"test {sp0['test']['pairs']} → {cur_dup['split']['test']['pairs']} · mixed {sp0['mixed']['pairs']} → "
          f"{cur_dup['split']['mixed']['pairs']} — the growth is test-side; the top standing pair "
          f"`{cur_dup['top'][0][0]}` × `{cur_dup['top'][0][1]}` ({cur_dup['top'][0][2]} L) stands since Epoch 4")
    elif 'delta' in b:
        w(f"- {b['check']} · {b['metric']} — {b['was']} → {b['now']} ({b['delta']:+})")
    elif b['check'] == 'mutation-drop':
        w(f"- mutation · {b['unit']} — {b['was']} → {b['now']}")
    else:
        w(f"- {b['check']} · {b['metric']} — {b.get('was')} → {b.get('now')}")
w(f"- totals — loc {rec['totals']['loc']} (baseline 100715), files {rec['totals']['files']} (423), units "
  f"{rec['totals']['units']} — a population, never a movement")
w('')

w('## Skips')
w('')
for s in rec['skips']:
    w(f"- {s['metric']} — {s['reason']}")
w('- coverage.branch — not collected by the project\'s coverage leg (`-` in its TOTAL row), null as at the baseline')
w('')

w('## Judgement calls this run made on its own (the founder asleep — the conservative option, named)')
w('')
for line in J('calls.json'):
    w(f'- {line}')
w('')
open(f'{rd}/proposals.md', 'w', encoding='utf-8').write('\n'.join(out))
print('proposals.md written', len(P), 'proposals', len(jd['informational']), 'informational', len(jd['below']), 'below')
