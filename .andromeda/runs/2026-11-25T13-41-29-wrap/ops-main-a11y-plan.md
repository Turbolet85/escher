@row_set n=1 row=decision/ui-graph-one-node-per-snapshot-node cell=rule seen=2 detector=none
value: The read-only UI graph holds one node per snapshot node, a canvas's virtual nodes among them, each owned by its canvas with no handlers or reads, so the actions answer and exploration hold them too; names and closed words only.
basis: fct-x6ohav4t, fct-w25w7t3u, fct-kiimqirg
because: The graph no longer excepts a canvas's virtual nodes, and the actions answer and exploration now hold them; handler coverage lists none, a virtual node carrying no handler, so the row names it nowhere.
