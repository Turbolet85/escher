@row_set n=1 row=decision/no-screenshot-in-cross-os-byte-check cell=except seen=1 fact=F1 detector=none
value: A canvas-only shot, held byte-equal on linux x86_64, windows x86_64 and macOS aarch64 by one committed digest table, no pixels committed; a shot holding document text stays out.
basis: fct-rpckbvls, fct-o4fpla67, fct-u6a6lt4g
because: The capability statement's shot-bytes limit is now scoped to shots holding document text, a canvas-only shot read byte-equal on the three CI hosts, so the cross-OS no-screenshot rule gains the ratified canvas-only exception.
@row_set n=2 row=decision/committed-stand-checks cell=why seen=1 detector=none
value: A shot holding document text differs in its PNG bytes by OS, so no screenshot step is kept.
basis: fct-v6nbc4k3, fct-rpckbvls
because: A shot's PNG bytes now differ by OS only where the shot holds document text, so the reason is scoped while the rule of no screenshot step in the committed checks stands.
