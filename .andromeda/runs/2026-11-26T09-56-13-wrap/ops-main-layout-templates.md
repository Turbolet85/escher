@row_add n=1 row=decision/circle-drawer-dialog-disables-other-circles area=surface-desktop-native detector=none
name: Circle Drawer dialog disables other circles
rule: While Circle Drawer's diameter dialog is open, every circle but the one being adjusted declares disabled and is refused disabled.
state: provisional
basis: fct-yen3arbg, fct-a6uxljfm
because: The chunk makes the open dialog disable every other circle, a state no row of the surface holds; it rests on a delegate's answer, so it stands provisional.
@row_add n=2 row=decision/circle-drawer-canvas-keeps-size area=surface-desktop-native detector=none
name: Circle Drawer canvas keeps its size
rule: Circle Drawer's canvas keeps 700 x 400 and never shrinks, so on the 600 px stand view the dialog's Close lies below the fold and is scrolled into view before a click.
state: provisional
basis: fct-pobztq6c, fct-fpvtcznd, fct-7kzlkcnh
because: The canvas keeping its full size was chosen on a delegate's answer and moves Close below the stand view, which no row of the surface states.
@row_add n=3 row=decision/circle-drawer-canvas-selection-attributes area=surface-desktop-native detector=none
name: Circle Drawer canvas selection attributes
rule: Circle Drawer's canvas reads the selected circle and the circle being adjusted from two attributes beside data-circles, data-selected and data-adjusting, each a circle's id or empty.
state: provisional
basis: fct-wndwrhax, fct-pobztq6c
because: The canvas gains two attributes beside data-circles, which decision/circle-drawer-circles-painted-on-canvas's rule cannot take within its bound; the selection riding its own attributes rests on a delegate's answer.
