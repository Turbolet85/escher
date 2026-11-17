@row_set n=1 row=contract/paint-bench-cli/backend cell=promise seen=1 detector=none
value: One of vello (the default), cpu or gpu, vello_gpu also accepted for gpu.
basis: fct-roenbczq
because: The paint_bench backend list was renamed with upstream from [vello|cpu|hybrid] to [vello|cpu|gpu], so the backend name hybrid no longer exists and the row must name gpu.
@row_add n=1 row=decision/scroll-offset-clamped-to-range area=surface-desktop-native fact=F2 detector=none
name: Scroll offsets clamped to their scroll range
rule: Every scroller's offset is held within its scroll range at layout rounding and after a viewport resize, and a removed subtree's scroll state is reset, so no reading holds an offset past the content, in every document.
why: An engine behaviour change taken from upstream and ratified.
guard: stand_act_scroll
basis: fct-ti3zu7r4
because: The chunk takes upstream's scroll clamp as a ratified engine behaviour change, and no row of the surface says that a scroller's offset never stands past its content.
@row_none n=1 fact=F1 type=5
@row_none n=2 fact=F3 type=6
