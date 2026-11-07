@answer n=1 entity=visual-design-verification#8.6 v=yes
why: the entity names tests/blitz-tests/tests/stand_cells_served.rs:216-232, a stand_* check the chunk re-asserts; a driver click in the scrollbar band right after a scroll still needs an advance past the fade
@answer n=2 entity=visual-design-verification#8.7 v=yes
why: the entity names tests/blitz-tests/tests/stand_act_popup.rs:228-245, a stand_* check the chunk re-asserts; a driver click under the open list is refused covered
@answer n=3 entity=visual-design-verification#8.8 v=no
why: paint order under mix-blend-mode and isolation is not touched by the buffered render, and the entity names no check in the chunk's scope (select_popup_painted is outside it)
@answer n=4 entity=visual-design-verification#8.9 v=no
why: devtools layout outlines and highlight overlay settings in packages/blitz-traits/src/devtools.rs are not touched and not named by the chunk
