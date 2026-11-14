@row_add n=1 row=decision/stand-readings-same-on-every-ci-os area=screen-reader-support fact=F1 detector=none
name: Stand readings the same on every CI OS
rule: The snapshot, lint and actions readings of the six served tasks, in both layout modes, at boot and after one acting step, read the same on linux x86_64, windows x86_64 and macOS aarch64, compared by the driver's answer digests against one stated table, with no screenshot.
guard: stand_cross_os_readings
basis: fct-j7ehdiy5, fct-smo47az5
because: The chunk widens the cross-OS reading from the Counter's committed checks to the six served tasks' snapshot, lint and actions answers, a test strategy future work on the accessibility readings must keep, and no row of the area states it.
