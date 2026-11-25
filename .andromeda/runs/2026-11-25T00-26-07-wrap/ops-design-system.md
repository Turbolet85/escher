@row_add n=1 row=decision/canvas-label-truncated-region area=surface fact=F1 detector=none
name: Canvas label read truncated against its region
rule: The lint reads a label a canvas widget paints as truncated when its glyph ink box is not inside its region, the element's content box cut by each pushed clip, layer or mask path's bounding box and by its clip region, once per label in paint order, with no label text in the finding.
guard: a_label_past_its_region_reads_truncated_on_the_planted_page
state: provisional
basis: fct-nolgh3rx
because: The lint's truncated reading now reaches the labels a canvas paints, a region chosen on a delegate's answer, and no row of the surface says it.
@row_none n=1 fact=F2 type=9
what: The four canvas checks left out (labels overlapping one another, a label's contrast against the fill under it, a command wholly outside its region, a shape with no area) are residuals the route and the deferred-from-0.4.0 list hold.
