@row_add n=1 row=decision/cross-os-readings-by-answer-digest area=test-strategy fact=F1 detector=none
name: Cross-OS readings of the served tasks
rule: The six served tasks are read on every CI OS by one stand check that compares the driver's answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both layout modes, to one committed table, with no shot and no kept check file.
guard: stand_cross_os_readings
basis: fct-y25hmj6c, fct-smo47az5, fct-j7ehdiy5
because: The cross-OS reading's reach moved from the two committed Counter checks to the six served tasks, read by answer digest against one committed table, and no standing row states that strategy.
@row_set n=1 row=decision/stand-checks-bundled-font cell=guard seen=1 fact=F2 detector=none
value: stand_bundled_font_families
basis: fct-ir5p5a5n, fct-rxjqhksp
because: Every generic family and an unavailable named family now read the bundled font's sans-serif extent in both layout modes under a stand check, so the bundled-font stand decision gains that check as its guard.
@row_set n=2 row=decision/mutation-controlled-tests cell=except seen=1 fact=F3 detector=none
value: A font-family reading's red control is a font context holding other fonts, since on a one-font context removing a generic family's registration is vacuous: fallback reaches that font anyway.
basis: fct-ntltai4w, fct-rdsd5trr
because: The registration-removal control for the families reading stayed green on the one-font context, so the controlled-test rule holds for font-family readings only with a font context that holds other fonts.
