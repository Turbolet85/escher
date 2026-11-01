# New groups — architecture · occupied-resources

## Verdicts
- fanout-architecture--cascade-g9.md 2 · contradiction · lines 8–8 · «no network port — no TCP or UDP port» · the standing line says the tree holds no TCP port and names one test-only bind, a Unix-domain one, while the proposal registers two test-only TCP binds on loopback; its "no served port" narrows the claim the line states without narrowing
- cascade-compare-architecture.md 6 · contradiction · lines 8–8 · «no network port — no TCP or UDP port» · the same conflict over the same two binds as fanout-architecture--cascade-g9.md 2, which this proposal restates at another place: two test-only TCP binds on loopback against a line saying no TCP port and one test-only Unix-domain bind
- cascade-compare-architecture.md 7 · distinct · no group names `init_stylo_prefs` or its `Once`; the process-wide group's Stylo entries are the rayon pool and `PARALLEL_TRAVERSAL_LOCK`, other state
