"""Screen printer for the q-*.json twins (read-only)."""
import json
import os
import sys

RUN = os.path.dirname(os.path.abspath(__file__))
which = sys.argv[1]
d = json.load(open(os.path.join(RUN, which), encoding='utf-8'))
if which == 'q-typed.json':
    for sect in ('per_step', 'universal_by_type', 'by_type_alone_observation'):
        print('=====', sect)
        for g in d[sect]:
            print(f"{g['key']} n={g['n']} w={g['weight']} rate={g['rate']} halt={g['halt_or_soft_exit']} "
                  f"above={g['above_threshold']} chunks={g['chunks']}")
            if sect != 'by_type_alone_observation':
                for c in g['cases']:
                    print('   -', c['id'], c['chunk'], c['step'], json.dumps(c['impact']), '|', c['what'],
                          '| ev:', c['evidence'], '| art:', c['artifacts'], '| clause:', c['clause_retracted'])
    print('step_runs', d['step_runs'])
elif which == 'q-untyped.json':
    for k in ('target', 'previous_epoch_lookback'):
        print('=====', k)
        for r in d[k]:
            print('-', r['id'], r['chunk'], r['step'], '|', r['what'])
elif which == 'q-chains.json':
    for c in d['chains']:
        print('-', json.dumps(c, ensure_ascii=False))
    print('=====')
    for s in d['shapes']:
        print(s)
elif which == 'q-facts-raw.json':
    sect = sys.argv[2]
    for r in d[sect]:
        print('-', json.dumps(r, ensure_ascii=False))
