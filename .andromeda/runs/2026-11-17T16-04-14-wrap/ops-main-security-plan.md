@row_retire n=1 row=decision/free-disk-only-on-stand-repeat seen=1 detector=none
why: The default-features test job now frees runner disk too, and the guard the row names was renamed; the rule's home is the test plan's CI integration.
basis: fct-w2ipnyht
because: The free-disk step moved from one job to two, so a row keyed and ruled on the stand-repeat job alone is false, and the one fact keeps its home in test-plan.
