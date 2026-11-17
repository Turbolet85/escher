@row_add n=1 row=decision/scroll-offset-clamped-to-range area=motion fact=F1 detector=none
name: Scroll offset clamped to the scroll range
rule: An element's scroll offset is clamped to its scroll range when layout settles, after a viewport resize and when its content shrinks, so a list emptied by a delete reads offset 0 in both layout modes.
why: Taken from upstream, ratified as upstream's behaviour.
guard: stand_act_scroll; percentage_height_scroller_clamps_after_viewport_resize
basis: fct-ti3zu7r4
because: The upstream take ratified the layout-time scroll clamp and the stand's CRUD list now reads offset 0 after a delete, which no motion row says.
@row_add n=2 row=contract/dioxus-native-renderer-features/vello-gpu area=surface fact=F2 detector=none
boundary: dioxus-native renderer features
item: vello-gpu
promise: The vello-gpu feature, renderer options and recipe select the Vello GPU window renderer; no renderer is named hybrid.
basis: fct-y4zyhjnd
because: The public feature vello-hybrid was renamed vello-gpu and the renderer options now alias the Vello GPU renderer, a name others build against.
@row_add n=3 row=decision/svg-external-images-through-image-cache area=new fact=F4 detector=none
section: SVG rendering
name: SVG external images through the image cache
rule: An inline or top-level svg renders the external images of its image and feImage children, loaded through the document's image cache; a use element's href is not fetched.
why: Ratified at its measured scope, image and feImage href only.
basis: fct-vaq3umam
because: The upstream take made inline svg load its image and feImage children's hrefs, a scope the founder ratified as measured, and design-system holds no row on it.
@row_none n=1 fact=F3 type=4
what: How every fill, stroke and glyph draw sets anti-aliasing is how the paint calls are built; the code holds it.
