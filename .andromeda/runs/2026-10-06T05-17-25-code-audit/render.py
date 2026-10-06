"""Render {run_dir}/proposals.md (proposal-template.md, baseline mode). Row counts asserted programmatically."""
import json

RD = '.andromeda/runs/2026-10-06T05-17-25-code-audit'
L = lambda n: json.load(open(f'{RD}/c-{n}.json', encoding='utf-8'))
recs = []
for line in open('.andromeda/code-metrics.ndjson', encoding='utf-8'):
    try:
        recs.append(json.loads(line))
    except ValueError:
        pass
rec = recs[-1]
dup, cx, sz, gr, dd, cov, mu, mf = (L('duplication'), L('complexity'), L('sizes'), L('graph'), L('dead'), L('coverage'),
                                    L('mutation-escher-telemetry'), L('mutation-escher-telemetry-faulted'))
out = []
w = out.append


def table(header, rows, n, key):
    assert len(rows) == n, f'{header}: {len(rows)} rows != n {n}'
    assert len({key(r) for r in rows}) == len(rows), f'{header}: rows not unique on the natural key'
    w('| ' + ' | '.join(header) + ' |')
    w('|' + '---|' * len(header))
    for r in rows:
        w('| ' + ' | '.join(str(c) for c in r) + ' |')
    w('')


w(f"# Code Audit — escher · {rec['epoch']} · {rec['ts']}")
w(f"mode baseline · HEAD {rec['sha'][:8]} · baseline none · span — (first record)")
w(f"overshoot 0 commits — HEAD is the epoch boundary ({rec['head_overshoot']['boundary_sha'][:8]}, the 2026-10-06-cold-agent-run-pipe complete flip)")
w('')
w('Trend judgments begin at the next boundary: this record is the baseline every later diff reads. '
  'Below are absolute-only findings. Nothing here is applied; each line is a direction for the founder\'s judgment.')
w('')
w('## Baseline findings')
w('')
w(f"### B1 — dependency cycles · {gr['cycles']}")
w(f"The canonical unit-cycle walk over `crate_edges` ({gr['cross_unit_edges']} usage-based edges, 21 source units, graph `{gr['db_state']}`) returns no cycle. "
  'Any cycle at a later boundary fires `new-cycle`.')
w('')
w(f"### B2 — mutation survivors · escher-telemetry · {mu['counts']['missed']}")
c = mu['counts']
w(f"**Score:** {mu['score']} = caught/(caught+missed) = {c['caught']}/{c['caught'] + c['missed']} · "
  f"{c['mutants']} mutants: {c['caught']} caught · {c['missed']} missed · {c['unviable']} unviable · {c['timeout']} timeout · "
  f"{c['not_measured']} not measured on this host ({mu['host']}) · state {mu['unit_state']}")
w('')
table(['site', 'mutation'], mu['survivors'], c['missed'], lambda r: (r[0], r[1]))
w(f"Not measured on this host ({mu['host']}): none — {mu['union_verdict']}.")
w('')
w('**Suspected shape:** the one survivor is the `Display` text of `InitError::ForeignSubscriber`. No test reads the rendered message: '
  'neither the four `telemetry_*` blitz-tests nor the crate\'s lib tests name `ForeignSubscriber` or its text.')
w('**Proposal:** if the message is a contract (operators read it when init refuses), one assertion on `InitError::ForeignSubscriber.to_string()` kills it; '
  'if it is not, the survivor is accepted noise.')
w('')
w('**Scope and method (disclose):** the test set is escher-telemetry\'s lib tests + blitz-tests\' lib + `telemetry_panic_hook`, '
  '`telemetry_init_idempotent`, `telemetry_stdout_silent`, `telemetry_scrub` (`--cargo-test-arg`). '
  'The tool\'s unmutated baseline was SKIPPED (`--baseline=skip --timeout 120`). cargo-mutants runs that baseline against the mutated package alone, '
  'where the `--test=telemetry_*` filters do not exist, so it aborts before testing any mutant. '
  'The narrowed unmutated build was verified by hand and `ci-leg.sh coverage` ran the whole suite green at this sha. '
  f"The 3 unviable are genuine type errors: `Ok(Default::default())` for `init`/`init_with_writer` (`InitOutcome` has no `Default`) and `Default::default()` for `decide` (`Verdict`).")
w('')
fc = mf['counts']
w(f"**Faulted first invocation (kept as `c-mutation-escher-telemetry-faulted.json`, not scored):** the same 12 mutants, tested against all of blitz-tests at `-j 3`, came back "
  f"{fc['caught']} caught · {fc['missed']} missed · {fc['unviable']} unviable, which is `unviable-dominant`. "
  'Six of the nine unviable builds failed with `Disk quota exceeded (os error 122)` on the usrquota tmpfs `/tmp`, where each cargo-mutants build copy needs about 12–13 GB. '
  'This is a host fact for later boundaries: a mutation unit that links blitz-tests fits `/tmp` only at `-j 1`.')
w('')
w(f"### B3 — dead-code candidates · {dd['zero_ref_candidates']}")
fp = dd['fp_classes']
w(f"Zero-ref rows {dd['zero_ref_rows']} → test-path excluded {dd['excluded_test_path']} → false-positive classes: "
  + ' · '.join(f'{k} {v}' for k, v in fp.items() if k != 'candidate')
  + f" → **{dd['zero_ref_candidates']} candidates** (never \"dead\"). The recipe is pinned in `c-dead.json` (`recipe`); reuse it verbatim or `dead-growth` fires on filter drift.")
w('')
w('Per crate: ' + ' · '.join(f'{k} {v}' for k, v in dd['candidates_by_crate'].items()) + '. Per kind: '
  + ' · '.join(f'{k} {v}' for k, v in dd['candidates_by_kind'].items()) + '.')
w('')
w('**Reading:** most candidates are public API of the upstream engine crates (blitz-dom `BaseDocument` setters and stylesheet accessors, '
  'blitz-test-harness input verbs, blitz-traits `ShellProvider` methods). An embeddable library\'s public surface is consumed by embedders outside this workspace, '
  'so a zero in-workspace reference does not prove it dead. The share nearest escher\'s roadmap is the 13 blitz-test-harness Harness verbs (`mouse_down_at`, `drag`, `tap`, `touch_*`, `ime`, `hovered`, `tick`, `time` …). '
  'That crate is upstream (#586), Epoch 1 changed 10 of its lines, and the driver epochs (Epoch 4) are their expected callers.')
w('**Proposal:** none at baseline. If `dead-growth` fires later, the growth rows are the ones to read.')
w('')
table(['crate', 'kind', 'symbol path', 'def site'], dd['candidates'], dd['zero_ref_candidates'], lambda r: (r[0], r[2], r[3]))
w(f"### B4 — unused dependencies (cargo-machete) · {len(dd['unused_deps'])}")
w('cargo-machete reads source text, not resolution: a dependency kept only to pin a feature or a version (the four `idna_adapter` rows are the likely case) '
  'or reached only through a macro reads as unused. These are candidates.')
w('')
table(['crate', 'dependency'], dd['unused_deps'], len(dd['unused_deps']), lambda r: tuple(r))
w('**Proposal:** a per-crate check of the non-pin rows (e.g. `rdme` → `blitz-paint`/`image`/`reqwest`, `wpt` → `markup5ever`/`style`/`taffy`, '
  '`dioxus-native` → `futures-util`/`keyboard-types`/`rustc-hash`). A confirmed unused dep is a cheaper build. A pin gets a `[package.metadata.cargo-machete] ignored` entry with its reason. '
  'Any change here touches the coupled dependency pins (CLAUDE.md) and is the founder\'s call.')
w('')
w('### Starting tables')
w('')
w(f"**Totals:** {rec['totals']['loc']} Rust code lines · {rec['totals']['files']} files · {rec['totals']['units']} units (27 members + the root `blitz-examples` package).")
w('')
w(f"**Duplication** (jscpd): {dup['pct']} % · {dup['duplicated_lines']} duplicated / {dup['total_lines']} lines · {dup['clones']} clones over {dup['sources']} files · "
  f"split src {dup['split']['src']['pairs']} pairs/{dup['split']['src']['lines']} L · test {dup['split']['test']['pairs']}/{dup['split']['test']['lines']} · mixed {dup['split']['mixed']['pairs']}/{dup['split']['mixed']['lines']}")
w('')
table(['first', 'second', 'lines'], dup['top_sites'], min(10, dup['clones']), lambda r: (r[0], r[1], r[2]))
w(f"**Complexity** (rust-code-analysis, {cx['functions']} function spaces): cognitive p50 {cx['cognitive_p50']} · p90 {cx['cognitive_p90']} · "
  f"cyclomatic p50 {cx['cyclomatic_p50']} · p90 {cx['cyclomatic_p90']} · max {cx['cyclomatic_max']} · **{cx['over_ceiling']} over the cognitive ceiling (>15)** · "
  f"function sloc p50 {cx['fn_sloc_p50']} · p90 {cx['fn_sloc_p90']} · max {cx['fn_sloc_max']}")
w('')
table(['function', 'site', 'cognitive', 'cyclomatic'], [[t['fn'], t['file'], t['value'], t['cyclomatic']] for t in cx['top']], 10, lambda r: (r[0], r[1]))
w('All ten top offenders are upstream engine code (blitz-dom layout, style, events; blitz-paint gradients). Epoch 1 touched two of their files by a few lines (`5dc809a1`: `layout/replaced.rs`, `node/node.rs`, about 4 lines between them).')
w('')
w(f"**Sizes** (tokei): file p50 {sz['file_p50']} · p90 {sz['file_p90']} · max {sz['file_max']} · {sz['over_800']} files over 800 code lines")
w('')
table(['file', 'code lines'], sz['top'], 10, lambda r: r[0])
w(f"**Graph:** {gr['cycles']} cycles · {gr['cross_unit_edges']} cross-unit edges · fan-out top: "
  + ' · '.join(f'{u} {n}' for u, n in gr['fan_out'][:5]) + '. Fan-in top 5: '
  + ' · '.join(f'`{s}` {n}' for s, n in gr['fan_in_top'][:5]) + ' (20 in `c-graph.json`).')
w('')
w(f"**Coverage** (`ci-leg.sh coverage`): line {cov['line']} % ({cov['lines_total'] - cov['lines_missed']}/{cov['lines_total']}) · regions {cov['regions_pct']} % · "
  f"functions {cov['functions_pct']} % · branch null (the leg does not instrument branches). A falling line figure is the signal; a rising one is not celebrated.")
w('')
w('## Informational')
w('- Churn and hotspots are not computed in baseline mode. They start at the next boundary, and churn stays informational until the ledger holds 2+ records.')
w('- `code-graph.py` is versionless and recorded by blob hash `d0425fb1`. Any edit to it is a trend-break on graph and dead.')
w('- The adoption trace `tree-query-code-audit.json` is 200 KB. Most of it is the 319-row dead-code residual pull that the classing reads.')
w('')
w('## Below threshold — no action')
w('- None: baseline mode has no deltas.')
w('')
w('## Skips')
for s in rec['skips']:
    note = {'churn': 'no prior record', 'hotspots': 'needs churn', 'mutation:seven_guis': '231 mutants would exceed the 15-min per-unit cap and score nothing (overseer, under the founder\'s standing delegation)'}.get(s['metric'], '')
    w(f"- {s['metric']} — {s['reason']}" + (f' ({note})' if note else ''))
open(f'{RD}/proposals.md', 'w', encoding='utf-8').write('\n'.join(out) + '\n')
print('proposals.md', len(out), 'lines')
