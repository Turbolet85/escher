"""Phases 4-5: diff + judge (audit-pass.md §Thresholds) and render proposals.md.

usage: python3 render.py {run_dir_rel}   (cwd = repo root; after the P3 append)
The baseline is re-found by sha (the LAST record before this run's append whose sha == baseline_sha),
the monotonic chain through the baseline's own baseline_sha — never the ledger's last lines.
"""
import json
import os
import sys

RD = sys.argv[1]
cur = json.load(open(os.path.join(RD, 'record.json'), encoding='utf-8'))
LEDGER = sys.argv[2] if len(sys.argv) > 2 else '.andromeda/code-metrics.ndjson'  # a dry-run copy only
recs = [json.loads(l) for l in open(LEDGER, encoding='utf-8')]
assert recs[-1]['ts'] == cur['ts'], 'record not appended (or ledger moved)'
prior = recs[:-1]
base = [r for r in prior if r['sha'] == cur['baseline_sha']][-1]
base_i = max(i for i, r in enumerate(prior) if r['sha'] == cur['baseline_sha'])
prev = [r for r in prior[:base_i] if r['sha'] == base['baseline_sha']][-1]
ev = json.load(open(os.path.join(RD, 'c-evidence-monotonic.json'), encoding='utf-8'))
out = []
w = out.append


def g(r, path):
    v = r
    for k in path.split('.'):
        v = None if v is None else v.get(k)
    return v


def fmt(v):
    return 'null' if v is None else ('%g' % v if isinstance(v, float) else str(v))


def table(head, rows, n):
    assert len(rows) == n, (head, len(rows), n)
    w('| ' + ' | '.join(head) + ' |')
    w('|' + '---|' * len(head))
    for r in rows:
        w('| ' + ' | '.join(str(c) for c in r) + ' |')


# ---------- judge ----------
TRACKED = [('duplication.pct', 1), ('complexity.over_ceiling', 1), ('dead.zero_ref_candidates', 1),
           ('sizes.file_max', 1), ('sizes.over_800', 1), ('coverage.line', -1)]
mono, below = [], []
for path, sign in TRACKED:
    a, b, c = g(prev, path), g(base, path), g(cur, path)
    if None in (a, b, c):
        below.append('`%s` monotonic not evaluable (null at one of the three records)' % path)
        continue
    if sign * (b - a) > 0 and sign * (c - b) > 0:
        mono.append((path, a, b, c))
    else:
        below.append('`%s` %s → %s → %s (%s → %s → %s): not worse at both diffs' % (
            path, fmt(a), fmt(b), fmt(c), prev['epoch'], base['epoch'], cur['epoch']))

single = []
dp, bp = cur['duplication']['pct'], base['duplication']['pct']
single.append(('duplication-up', dp >= bp + 0.5 and dp >= bp * 1.15, 'duplication.pct %s → %s (%+.3f pt, %+.1f %%)'
               % (fmt(bp), fmt(dp), dp - bp, 100 * (dp - bp) / bp)))
oc, bo = cur['complexity']['over_ceiling'], base['complexity']['over_ceiling']
single.append(('complexity-creep', oc >= bo + 3 and oc >= bo * 1.25, 'complexity.over_ceiling %d → %d (%+d, %+.1f %%; '
               'fires at +3 AND +25 %%)' % (bo, oc, oc - bo, 100 * (oc - bo) / bo)))
dc, bd = cur['dead']['zero_ref_candidates'], base['dead']['zero_ref_candidates']
single.append(('dead-growth', dc >= bd + 5, 'dead.zero_ref_candidates %d → %d (%+d)' % (bd, dc, dc - bd)))
cl, bl = cur['coverage']['line'], base['coverage']['line']
single.append(('coverage-drop', cl <= bl - 2, 'coverage.line %s → %s (%+.2f pt; a rise is never celebrated)'
               % (fmt(bl), fmt(cl), cl - bl)))
single.append(('new-cycle', cur['graph']['cycles'] > base['graph']['cycles'], 'graph.cycles %d → %d'
               % (base['graph']['cycles'], cur['graph']['cycles'])))
for cid, fired, line in single:
    assert not fired, cid  # none expected at this boundary; a fire would need its own proposal block
    below.append('%s — %s' % (cid, line))

# mutation-drop: last SCORED value at EQUAL scope, walking back from the baseline
mut_lines, mut_props = [], []
for u in cur['mutation']['scoped_units']:
    s = cur['mutation']['scores'].get(u)
    if s is None:
        continue
    t = cur['mutation']['timing'][u]
    last = None
    for r in reversed(prior):
        if u in (r.get('mutation') or {}).get('scores', {}):
            last = r
            break
    if last is None:
        mut_lines.append('`%s` %.2f (scope %s, %s) — no prior score: the first scored value' % (u, s, t['scope'], cur['epoch']))
        continue
    lt = (last['mutation'].get('timing') or {}).get(u)
    lscope = lt['scope'] if lt else 'UNKNOWN'
    same = lt is not None and lt['scope'] == t['scope'] and (
        t['scope'] == 'unit' or (t['scope'] == 'files' and lt.get('files') == t.get('files')))
    ls = last['mutation']['scores'][u]
    if not same:
        mut_lines.append('`%s` not comparable — %.2f (scope %s%s, %s) vs %.2f (scope %s%s, %s); no delta' % (
            u, s, t['scope'], (' ' + ', '.join(t['files'])) if t.get('files') else '', cur['epoch'], ls, lscope,
            (' ' + ', '.join(lt['files'])) if lt and lt.get('files') else '', last['epoch']))
    elif s <= ls - 10:
        mut_props.append((u, ls, s, last['epoch']))
    else:
        below.append('mutation-drop — `%s` %.2f → %.2f (scope unit both; %s → %s; fires at −10 pt)' % (
            u, ls, s, last['epoch'], cur['epoch']))

# top-N entrants
entr = []


def entrants(label, cur_top, base_top, key):
    bk = {key(x) for x in base_top}
    new = [x for x in cur_top if key(x) not in bk]
    for x in new:
        entr.append('%s entrant: %s' % (label, ' · '.join(fmt(c) if isinstance(c, (int, float)) else str(c) for c in x)))


entrants('complexity top-10', cur['complexity']['top'], base['complexity']['top'], lambda x: (x[0], x[1].split(':')[0]))
entrants('sizes top-10', cur['sizes']['top'], base['sizes']['top'], lambda x: x[0])
entrants('duplication top-10', cur['duplication']['top'], base['duplication']['top'], lambda x: tuple(x))
entrants('hotspots top-10', cur['hotspots'], base['hotspots'], lambda x: x[0])
entrants('fan-in top-20', cur['graph']['fan_in_top'], base['graph']['fan_in_top'], lambda x: x[0])
entrants('dead-candidate top-10', cur['dead']['top'], base['dead']['top'], lambda x: x[0])
bud = {tuple(x) for x in base['dead']['unused_deps']}
for x in cur['dead']['unused_deps']:
    if tuple(x) not in bud:
        entr.append('unused-dependency entrant (cargo-machete): %s → %s' % tuple(x))
for x in base['dead']['unused_deps']:
    if tuple(x) not in {tuple(y) for y in cur['dead']['unused_deps']}:
        below.append('unused dependency gone: %s → %s' % tuple(x))

# ---------- render ----------
w('# Code Audit — escher · %s · %s' % (cur['epoch'], cur['ts']))
w('mode %s · HEAD %s · baseline %s (%s) · span %d' % (cur['mode'], cur['sha'][:9], cur['baseline_sha'][:9],
                                                     base['epoch'], cur['span']))
w('')
w('Overshoot: this run 0 commits (HEAD is the boundary, the 2026-10-15-cold-agent-test complete flip) · the '
  'baseline\'s 0 commits (%s). No ancestry break; no trend-break (every tool token equals the baseline\'s).'
  % base['head_overshoot']['note'])
w('')
w('Scope of the mutation tier (the founder\'s answers, relayed verbatim by the overseer, record '
  '`~/dev/projects/additional/escher-overseer/relays/v010-end-founder-word.md`): escher\'s five units whole '
  '(«Давай полностью, можно в 4 потока я не за компом»), blitz-dom on range.rs + keyboard.rs '
  '(«range.rs + keyboard.rs (Рекомендую)»), the upstream-only units not run («Не запускать (Рекомендую)»), `-j 4`.')
w('')
w('## Proposals')
w('')
n = 0
for path, a, b, c in mono:
    n += 1
    w('### M%d — monotonic · `%s` — worse at both of the last two diffs' % (n, path))
    w('**Movement:** %s → %s → %s (%s → %s → %s)' % (fmt(a), fmt(b), fmt(c), prev['epoch'], base['epoch'], cur['epoch']))
    if path == 'complexity.over_ceiling':
        e = ev['over_ceiling']
        w('**Evidence:** functions over cognitive 15 at the baseline %d, at HEAD %d. Entered (%d):' % (
            e['baseline'], e['head'], len(e['entered'])))
        w('')
        table(['file', 'function', 'line · cognitive'],
              [[x[0], '`%s`' % x[1], ', '.join('%d · %g' % (l, v) for l, v in x[2])] for x in e['entered']],
              len(e['entered']))
        w('')
        w('Left (%d): %s. Standing over-ceiling functions whose value moved (%d):' % (
            len(e['left']), '; '.join('`%s` %s:%d (%g)' % (x[1], x[0], x[2][0][0], x[2][0][1]) for x in e['left']),
            len(e['changed_value'])))
        w('')
        table(['file', 'function', 'cognitive baseline → HEAD'],
              [[x[0], '`%s`' % x[1], '%s → %s' % ('/'.join('%g' % v for v in x[2]), '/'.join('%g' % v for v in x[3]))]
               for x in e['changed_value']], len(e['changed_value']))
        w('')
        w('**Suspected shape:** the growth sits in blitz-dom\'s event handlers — `handle_click` 69 → 74, '
          '`handle_pointerdown` 35 → 40, `handle_key_or_input_event` 50 → 56 carry this epoch\'s keyboard-activation, '
          'button-focus and range default actions (direct engine changes ratified 2026-10-13); the range model\'s '
          '`parse_float` (19) is new — `events/pointer.rs`, `events/keyboard.rs` and `node/range.rs` were touched by '
          'escher\'s chunks alone this epoch (2026-10-13-range-input-interaction, 2026-10-13-stand-keyboard-harness); '
          '`dispatch_hover_change` and `refresh_anonymous_block_styles` came with the upstream merge (upstream #1176, '
          '#1174), and `set_attribute` / `clear_attribute` moved with both (upstream #1187 and the range re-seed). '
          'The series\' 74 → 80 step at Epoch 5 is the earlier half.')
        w('**Proposal:** for the escher-authored handlers, extract the per-element default-action arms out of '
          '`handle_click` / `handle_pointerdown` / `handle_key_or_input_event` into named functions (the range and '
          'activation arms are self-contained), and split `parse_float` into its sign/digits/exponent steps; leave the '
          'upstream-merged entrants to upstream, or name them for an upstream issue.')
    elif path == 'sizes.file_max':
        w('**Evidence:** the max file is `packages/blitz-dom/src/document.rs` at all three records — 2659 (Epoch 4) → '
          '2756 → 2793 code lines (tokei). Of its 2793, 688 are its five inline `#[cfg(test)]` modules from line 2897 '
          '(`zoom_tests` · `hover_state_tests` · `hover_invalidation_tests` · `font_face_override_tests` · '
          '`changed_set_tests`; the first 2896 lines read 2105). This epoch\'s +104/−58 raw lines in it '
          '(`git diff --numstat %s..HEAD`): the upstream merge +88/−41 (upstream #1189 shared UA stylesheets, #1177 '
          'sub-document cursor) and escher\'s 2026-10-14-quality-gates edit to `hover_state_tests` +16/−17.'
          % cur['baseline_sha'][:9])
        w('**Suspected shape:** the file carries upstream\'s `BaseDocument` body and, inline, a test volume a quarter '
          'its size, two of the five modules escher\'s (`changed_set_tests`, the `hover_state_tests` edits); this '
          'epoch\'s growth there is upstream\'s, not an escher feature.')
        w('**Proposal:** move escher\'s inline test modules out of `document.rs` into a sibling test file '
          '(`document/tests.rs` or a `blitz-tests` target) — the escher-side lever on this metric that keeps '
          'upstream\'s body untouched, so later upstream syncs stay mostly additive; whether upstream\'s own body '
          'is split is upstream\'s call.')
    elif path == 'sizes.over_800':
        e = ev['over_800']
        w('**Evidence:** files over 800 code lines: %d (Epoch 4) → %d → %d. Every row at HEAD with its baseline value:'
          % (a, b, c))
        w('')
        table(['file', 'baseline', 'HEAD'], [[x[0], fmt(x[1]) if x[1] is not None else '— (entered)', x[2]]
                                             for x in e['deltas']], c)
        w('')
        w('**Suspected shape:** the one entrant, `packages/stylo_taffy/src/wrapper.rs` (819), grew with the upstream '
          'merge, upstream #1175 (UA flex centering, +32/−5); of escher\'s own files only `packages/escher-driver/src/mcp.rs` '
          '(1140) and `schema.rs` (840) stand over the line, both unchanged this epoch.')
        w('**Proposal:** split `escher-driver/src/mcp.rs` along its existing seams (protocol framing and bounds · '
          'tool listing · tool-result writing) and `schema.rs`\'s verb table from its help texts; the upstream-owned '
          'rows are upstream\'s to split.')
    w('')
for u, ls, s, ep in mut_props:
    n += 1
    w('### M%d — mutation-drop · `%s` — %.2f → %.2f' % (n, u, ls, s))
    w('')
if n == 0:
    w('None.')
    w('')

# mutation tier block (informational, evidence complete)
w('## Informational')
w('')
w('### Mutation tier (host %s; score = caught/(caught+missed))' % cur['mutation']['host'])
w('')
rows = []
for u in cur['mutation']['scoped_units']:
    c = cur['mutation']['counts'].get(u)
    t = cur['mutation']['timing'].get(u)
    st = cur['mutation']['unit_states'][u]
    if c:
        rows.append(['`%s`' % u, t['scope'] + ((' (' + ', '.join(t['files']) + ')') if t.get('files') else ''), st,
                     c['caught'], c['missed'], c['not_measured'], c['timeout'], c['unviable'],
                     fmt(cur['mutation']['scores'].get(u)), '%d s' % t['wall_s']])
    else:
        rows.append(['`%s`' % u, t['scope'] if t else '—', st, '—', '—', '—', '—', '—', '—',
                     ('%d s' % t['wall_s']) if t else '—'])
table(['unit', 'scope', 'state', 'caught', 'missed', 'not measured', 'timeout', 'unviable', 'score', 'wall (-j 4)'],
      rows, len(cur['mutation']['scoped_units']))
w('')
for line in mut_lines:
    w('- ' + line)
w('')
nsurv = sum(c['missed'] for c in cur['mutation']['counts'].values())
w('Survivors — %d, complete (each a mutant that built and passed every test in its unit\'s set):' % nsurv)
w('')
table(['site', 'mutation'], [['`%s`' % s, '`%s`' % m] for s, m in cur['mutation']['survivors']], nsurv)
w('')
nnm = sum(c['not_measured'] for c in cur['mutation']['counts'].values())
w('Not measured on this host (%s) — %d%s' % (cur['mutation']['host'], nnm, ':' if nnm else '.'))
if nnm:
    w('')
    table(['site', 'mutation', 'predicate'], [['`%s`' % a, '`%s`' % b, '`%s`' % c]
                                              for a, b, c in cur['mutation']['not_measured']], nnm)
w('')
tos = json.load(open(os.path.join(RD, 'c-mutation-timeouts.json'), encoding='utf-8'))
nto = sum(c['timeout'] for c in cur['mutation']['counts'].values())
w('Timeouts — %d (scored neither caught nor missed; each a mutant whose test run exceeded the %s s per-mutant '
  'timeout):' % (nto, '300'))
w('')
table(['unit', 'site', 'mutation'], [['`%s`' % u, '`%s`' % s, '`%s`' % m] for u in sorted(tos) for s, m in tos[u]],
      nto)
w('')
w('Union verdict: no project union verdict (no `.andromeda/test-plan.md` line names cargo-mutants).')
w('')
w('### Other movements')
w('')
dd = cur['duplication']
bdup = base['duplication']
w('- count-under-ratio (not fired: `pct` rose with the counts) — clones %d → %d (%+d), duplicated lines %d → %d (%+d), '
  '`pct` %s → %s, population `total_lines` %d → %d (%+.1f %%); split src %d/%d L · test %d/%d L · mixed %d/%d L '
  '(baseline src %d/%d · test %d/%d · mixed %d/%d); the top standing pair `%s` ↔ `%s` (%d L) stands since before '
  'this epoch.' % (bdup['clones'], dd['clones'], dd['clones'] - bdup['clones'], bdup['duplicated_lines'],
                   dd['duplicated_lines'], dd['duplicated_lines'] - bdup['duplicated_lines'], fmt(bdup['pct']),
                   fmt(dd['pct']), bdup['total_lines'], dd['total_lines'],
                   100 * (dd['total_lines'] - bdup['total_lines']) / bdup['total_lines'],
                   dd['split']['src']['pairs'], dd['split']['src']['lines'], dd['split']['test']['pairs'],
                   dd['split']['test']['lines'], dd['split']['mixed']['pairs'], dd['split']['mixed']['lines'],
                   bdup['split']['src']['pairs'], bdup['split']['src']['lines'], bdup['split']['test']['pairs'],
                   bdup['split']['test']['lines'], bdup['split']['mixed']['pairs'], bdup['split']['mixed']['lines'],
                   dd['top'][0][0], dd['top'][0][1], dd['top'][0][2]))
for e in entr:
    w('- ' + e)
w('')
w('## Below threshold — no action')
w('')
bs, cs = base['sizes'], cur['sizes']
below += ['sizes file_p50 %d → %d · file_p90 %d → %d' % (bs['file_p50'], cs['file_p50'], bs['file_p90'], cs['file_p90']),
          'complexity cyclomatic p50/p90 %s/%s → %s/%s · cognitive p50/p90 %s/%s → %s/%s · max `%s` %s → %s' % (
              fmt(base['complexity']['cyclomatic_p50']), fmt(base['complexity']['cyclomatic_p90']),
              fmt(cur['complexity']['cyclomatic_p50']), fmt(cur['complexity']['cyclomatic_p90']),
              fmt(base['complexity']['cognitive_p50']), fmt(base['complexity']['cognitive_p90']),
              fmt(cur['complexity']['cognitive_p50']), fmt(cur['complexity']['cognitive_p90']),
              cur['complexity']['max']['fn'], fmt(base['complexity']['max']['val']), fmt(cur['complexity']['max']['val'])),
          'graph cross-unit edges %d → %d (population, not a movement) · cycles 0 at both ends' % (
              base['graph']['cross_unit_edges'], cur['graph']['cross_unit_edges']),
          'churn %s %% → %s %% · files churned %d → %d' % (fmt(base['churn']['pct']), fmt(cur['churn']['pct']),
                                                        base['churn']['files_churned'], cur['churn']['files_churned']),
          'dead FP classes at HEAD: %s (rows %d, test-path excluded %d)' % (
              ', '.join('%s %d' % kv for kv in sorted(cur['dead']['fp_classes'].items())), cur['dead']['zero_ref_rows'],
              cur['dead']['excluded_test_path']),
          'totals (population) loc %d → %d · files %d → %d · units %d → %d' % (
              base['totals']['loc'], cur['totals']['loc'], base['totals']['files'], cur['totals']['files'],
              base['totals']['units'], cur['totals']['units'])]
for b in below:
    w('- ' + b)
w('')
w('## Skips')
w('')
for s in cur['skips']:
    w('- %s — %s' % (s['metric'], s['reason']))
w('')
open(os.path.join(RD, 'proposals.md'), 'w', encoding='utf-8').write('\n'.join(out))
print('proposals.md: %d proposals · %d informational lines · %d below · %d skips' % (
    n, len(mut_lines) + len(entr) + 1, len(below), len(cur['skips'])))
