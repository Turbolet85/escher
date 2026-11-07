@answer n=1 entity=quality-gates-coverage-targets#1.0 v=no
why: the 2026-10 readings (53.23 %, 53.25 %, 67.06 %) are dated measurements of earlier trees, and the session moves no figure they cite
@answer n=2 entity=quality-gates-coverage-targets#2.1 v=yes mode=stale
why: dioxus-native-dom now reads 2937/3387 = 86.71 % on CI#42371051720, moving the 2791/3241 = 86.12 % read, and the new mutation_buffer.rs reading (F4, 116 lines, 93.97 %) joins it
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=no
why: the stand-repeat leg and its 228 · 1155 · 0 · 21 tally are unchanged on CI#42371051720, no leg, step or selection moved, and the probe stays out of stand_*
@answer n=4 entity=quality-gates-coverage-targets#4.1 v=no
why: the scoped mutation runs over timer.rs, mcp/framing.rs and mutation_writer.rs are unreached, mutation_writer.rs is byte-identical, and the new passing dioxus-native-dom test leaves each unit's baseline green
@answer n=5 entity=quality-gates-coverage-targets#5.1 v=yes mode=stale
why: the session adds functions to reached files (mutation_buffer.rs, the ported dioxus_document.rs test, the probe's spawned_focus_acts helper), and the 85-function over-ceiling count read on 2026-10-26 is not shown to hold on them
