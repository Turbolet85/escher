@row_add n=1 row=contract/skia-archive-cli/usage area=surface-cli fact=F1 detector=none
boundary: skia_archive.py command line
item: usage
promise: skia_archive.py TARGET DIR, called with exactly a target triple and a directory; the helper reads no environment variable.
basis: fct-z3khe52c
because: The chunk adds the skia archive helper command, whose call form is a contract the leg runner and the CI matrix step rely on.
@row_add n=2 row=contract/skia-archive-cli/exit-0 area=surface-cli fact=F1 detector=none
boundary: skia_archive.py command line
item: exit 0
promise: A verified archive exits 0 and prints one stdout line, the file URL template for SKIA_BINARIES_URL with the {key} placeholder left literal for skia-bindings to fill.
basis: fct-z3khe52c
because: The chunk adds the helper's success exit and its one-line output, which callers export as SKIA_BINARIES_URL.
@row_add n=3 row=contract/skia-archive-cli/exit-1 area=surface-cli fact=F1 detector=none
boundary: skia_archive.py command line
item: exit 1
promise: Exits 1 after three failed attempts, each printing the fixed stderr line skia-archive: attempt {n} of 3 failed in fetch or verify, ending with the fixed line skia-archive: no verified archive.
basis: fct-z3khe52c
because: The chunk adds the helper's failure exit with fixed stderr lines kept in the leg or step log.
@row_add n=4 row=contract/skia-archive-cli/exit-2 area=surface-cli fact=F1 detector=none
boundary: skia_archive.py command line
item: exit 2
promise: A bad call or an unrecorded target exits 2 with the fixed stderr line skia-archive: usage or skia-archive: no recorded archive for this target, before anything is fetched.
basis: fct-z3khe52c
because: The chunk adds the helper's usage exit, refused before any fetch.
