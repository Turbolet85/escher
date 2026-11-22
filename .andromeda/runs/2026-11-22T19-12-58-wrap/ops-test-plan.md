@row_add n=1 row=decision/lock-move-measured-by-locked-build area=9-ci-integration fact=F1 detector=none
name: Lock move measured by the locked build
rule: A change of the lockfile is measured by the locked workspace build as well as by the audit leg; an audit-only reading of a moved lock proves no build.
why: The audit leg reads the lock against advisories and licenses alone, so a lock the workspace cannot compile on reads green there.
basis: fct-nejzrmgf, fct-s6vdi6yn
because: A lock move measured by the audit leg alone read green while the workspace did not compile on it, so the gate a lock move owes must name the locked build too.
