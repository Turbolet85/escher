@row_add n=1 row=decision/stand-families-read-bundled-font area=typography fact=F1 detector=none
name: Every font family reads the stand's bundled font
rule: On the stand's bundled set every generic family (serif, monospace, system-ui, cursive, fantasy) and any unavailable named family lays out in DejaVu Sans at the sans-serif extent, in both layout modes.
why: DejaVu Sans is the only font loaded, so every family falls back to it, registered for that family or not.
guard: stand_bundled_font_families
basis: fct-ir5p5a5n, fct-rxjqhksp (Expected amendment 3), fct-s3fmqo55 (Spec claims disproved 2), fct-noaylofa
because: The chunk measured every generic family and an unavailable family at the sans-serif extent on the bundled set, and the operator ruled the row states the fallback to the only loaded font as the reason, so Typography must carry it.
@row_add n=2 row=decision/bundled-typography-same-on-every-os area=typography fact=F2 detector=none
name: Bundled typography reads the same on every CI OS
rule: The stand's bundled typography reads the same on linux x86_64, windows x86_64 and macOS aarch64 for all six served tasks, in both layout modes, at boot and after one step, compared by the driver's answer digests over snapshot, lint and actions, with no screenshot.
guard: stand_cross_os_readings
basis: fct-j7ehdiy5, fct-smo47az5, fct-uetdow5h
because: The cross-OS reading's reach moved from the two committed Counter checks to the six served tasks, read green on the three CI OSes, and Typography holds no row saying the bundled set reads alike across them.
@row_none n=1 fact=F3 type=9
what: the resolution of the plan's conditional deferral (nothing deferred for cursive and fantasy) is the route's; the reading itself stands in the families row added for F1
