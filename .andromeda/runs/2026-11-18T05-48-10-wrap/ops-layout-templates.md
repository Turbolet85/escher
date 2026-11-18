@row_add n=1 row=decision/randomness-seed-in-process-only area=surface-cli fact=F1 detector=none
name: Randomness seed in process only
rule: A randomness seed lives in process only: no escher-session verb, argument, command-line flag, MCP tool field, kept-check key or span carries it.
basis: fct-uhxr2ibi, fct-e7wostsw
because: The capability statement now states the in-process-only seed as a limit at the escher-session surface, and no row of surface-cli says that none of its carriers holds a seed.
