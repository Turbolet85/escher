@row_set n=1 row=decision/driver-session-unix-socket-only cell=rule seen=1 detector=none
value: The driver session socket binds one Unix-domain socket in a caller-named state directory, never a TCP or UDP port, and exists on unix only, while an in-process session runs on linux, windows and macOS alike.
basis: fct-y25hmj6c, fct-smo47az5
because: The served tasks are now held through in-process sessions read on the windows and macOS CI legs as well as linux, so the row's claim that the driver session runs on unix only holds for its socket alone.
@row_add n=1 row=decision/cross-os-readings-committed-as-digests area=data-protection fact=F1 detector=none
name: Cross-OS readings committed as answer digests
rule: The cross-OS reading of the six served tasks is committed only as driver answer digests in a stand test's table read on linux, windows and macOS, never as a kept check file or a shot.
guard: stand_cross_os_readings
basis: fct-rxjqhksp, fct-y25hmj6c, fct-smo47az5
because: The chunk commits the six served tasks' cross-OS reading as a digest table rather than kept check files, a new committed medium the data-protection rows of committed checks do not yet state.
