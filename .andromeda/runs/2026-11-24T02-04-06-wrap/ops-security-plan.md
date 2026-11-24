@row_add n=1 row=decision/custom-widget-click-synthesis area=input-validation fact=F1 detector=D-security-input
name: Click synthesized on a custom-widget target
rule: Behind the custom-widget feature, after a custom widget hears a pointer event the engine synthesizes the UA click on it, pointerup into click on the main button or context menu on the secondary and a two-count click queuing a double click, with no click walk and no focus clear; no input crosses.
why: An engine change for every document behind the feature, ratified; upstream's widget arm returned before the synthesis.
guard: stand_canvas_click
basis: fct-t6akh6q6
because: The chunk adds the click synthesis on a custom-widget target so an app's click and double-click handlers on a widget element fire, a ratified engine change no standing custom-widget row states.
@row_none n=1 fact=F2 type=4
relied: The Circle Drawer sample's checks rely on its widget reading malformed data-circles text as no circle, without panic or log; the code holds the parse.
