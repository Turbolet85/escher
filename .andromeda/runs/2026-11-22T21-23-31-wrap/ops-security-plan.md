@row_add n=1 row=decision/upstream-only-advisories-held-as-waits area=dependency-security fact=F1 detector=none
name: Upstream-only advisories held as waits on upstream
rule: An open advisory ignore that only an upstream release can clear is held as a written wait on that upstream release, its residual re-carried, and is neither patched nor forked in escher.
why: Ratified.
basis: fct-v3qjpmym; fct-k5tujcic; fct-y3dtvt5r
because: The founder ratified F1 A, re-carrying the open advisory ignores as stated waits on upstream releases, a standing rule for the dependency audit that no row yet says.
