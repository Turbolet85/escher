@row_retire n=1 row=decision/only-stand-repeat-frees-disk seen=1 detector=none
why: The default-features test job now frees disk too, so stand-repeat is no longer the only job that does; the rule's home is the test plan's CI integration.
basis: fct-w2ipnyht
because: test-features-default gained the same SHA-pinned free-disk step and its guard was renamed, so the row is false, and the one fact keeps its home in test-plan.
@row_retire n=2 row=decision/free-disk-space-only-in-stand-repeat seen=1 detector=none
why: The default-features test job now runs the free-disk-space action too, so stand-repeat is no longer its one run; the rule's home is the test plan's CI integration.
basis: fct-w2ipnyht
because: test-features-default gained the same SHA-pinned free-disk step and its guard was renamed, so the row is false, and the one fact keeps its home in test-plan.
