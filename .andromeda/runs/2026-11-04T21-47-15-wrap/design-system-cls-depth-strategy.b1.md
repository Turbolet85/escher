@answer n=1 entity=depth-strategy#1.1 v=no
why: the changed paths (the probe test, Cargo.lock, deny.toml) do not hold the browser z-index values, and no apps/browser file changed.
@answer n=2 entity=depth-strategy#1.2 v=no
why: no apps/browser file changed, so the shared browser menu and suggestions shadow lines stand.
@answer n=3 entity=depth-strategy#1.3 v=no
why: the seven_guis shadows live in layout-templates, and the session touched no surface or layout text.
@answer n=4 entity=depth-strategy#1.4 v=no
why: the todomvc stylesheet lines are not among the changed paths and no CSS changed.
@answer n=5 entity=depth-strategy#1.5 v=no
why: the wgpu_texture, custom_widget and transforms z-index lines sit in examples that the session did not change.
@answer n=6 entity=depth-strategy#1.6 v=no
why: the blitz-dom default.css dialog backdrop is untouched, as no stylesheet or paint code changed.
@answer n=7 entity=depth-strategy#2.1 v=no
why: the paint-children build and the public re-exports in blitz-dom are untouched, and the probe reads only public Dioxus API.
@answer n=8 entity=depth-strategy#2.2 v=no
why: the stacking-context root rules at node.rs:1343-1419 are untouched, as no layout or paint code moved.
@answer n=9 entity=depth-strategy#3.1 v=no
why: the paint and hit-test walk cited here (node.rs, render.rs, document.rs) moved nothing, and the task marks hit-testing untouched.
@answer n=10 entity=depth-strategy#3.2 v=no
why: the open select list answer in hit_with_scrollbar is untouched, as no select popup or hit code changed.
@answer n=11 entity=depth-strategy#3.3 v=no
why: the non-invertible transform hit rule at node.rs:1485-1490 is untouched, as no transform or hit code moved.
@answer n=12 entity=depth-strategy#3.4 v=no
why: the visible_region code in scrolling.rs is untouched, as no clipping or transform code moved.
@answer n=13 entity=depth-strategy#4.1 v=no
why: paint_order.rs and the shared positioned-descendant paint level are untouched, as no paint code changed.
@answer n=14 entity=depth-strategy#4.2 v=no
why: the select list paint placement in paint_scene at render.rs:248-249 is untouched, as no paint code moved.
@answer n=15 entity=depth-strategy#4.3 v=no
why: box_shadow.rs and outset_box_shadow_shape.rs are untouched, as no shadow code changed.
@answer n=16 entity=depth-strategy#4.4 v=no
why: the inset shadow path in box_shadow.rs:90-157 is untouched, as no shadow code changed.
@answer n=17 entity=depth-strategy#4.5 v=no
why: the paints_own_box rule for inline boxes with block children in node.rs and construct.rs is untouched.
@answer n=18 entity=depth-strategy#4.6 v=no
why: the opacity, filter and backdrop-filter layer code at render.rs:530-563 is untouched.
@answer n=19 entity=depth-strategy#4.7 v=no
why: blitz-paint's blend module and its blend layers in render.rs are untouched.
@answer n=20 entity=depth-strategy#4.8 v=no
why: the blend witnesses in layers.rs and contrast.rs are untouched, and no test changed.
@answer n=21 entity=depth-strategy#4.9 v=no
why: the open select list paint in select_list.rs and select_popup_painted.rs is untouched.
@answer n=22 entity=depth-strategy#5.1 v=no
why: examples/assets filters.html and github_profile_reduced2.html are untouched.
@answer n=23 entity=depth-strategy#5.2 v=no
why: examples/assets google.html and graphite.html are untouched.
@answer n=24 entity=depth-strategy#5.3 v=no
why: the examples/assets hr.html and servo.css fixtures and the preact and box_shadow examples are untouched.
@answer n=25 entity=depth-strategy#6.1 v=no
why: none of the three changed paths is among the 17 listed s06 files the absence search covers, so the absence still holds.
