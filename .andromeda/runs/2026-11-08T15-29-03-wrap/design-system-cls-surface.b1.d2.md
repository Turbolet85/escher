@answer n=1 entity=surface#1.1 v=no
why: the session adds driver actions and check_target only, leaving the blitz-dom default user-agent sheet unchanged
@answer n=2 entity=surface#1.2 v=no
why: the select border, padding and background in default.css are engine CSS the session never touches
@answer n=3 entity=surface#1.3 v=no
why: the multiple-select row rules in default.css are unchanged engine layout, outside every reached file
@answer n=4 entity=surface#1.4 v=no
why: the multiple-select chosen-row colour rule is unchanged, and the session changes no colour
@answer n=5 entity=surface#1.5 v=no
why: the SelectListView highlight and border constants are unchanged, as the task's untouched item confirms
@answer n=6 entity=surface#1.6 v=no
why: the single-select optgroup display rules are unchanged engine CSS the session does not reach
@answer n=7 entity=surface#1.7 v=no
why: opening the list by click, Enter or Space is engine behaviour the session leaves as it was
@answer n=8 entity=surface#1.8 v=no
why: the list's paint_scene overlay path in blitz-paint is untouched by the driver verbs and actions read
@answer n=9 entity=surface#1.9 v=no
why: keyboard and pointer highlight movement is blitz-dom event behaviour the session does not change
@answer n=10 entity=surface#1.10 v=no
why: choosing a row and its input and change events are engine behaviour, and the session adds no event path
@answer n=11 entity=surface#1.11 v=no
why: the close rules for Escape, outside click and scroll are blitz-dom behaviour the session does not change
@answer n=12 entity=surface#1.12 v=no
why: the refusals to open a disabled, inert, hidden or multiple select are engine rules the session leaves alone
@answer n=13 entity=surface#1.13 v=no
why: focus and key handling after the container lost its box is engine behaviour, and the session adds no key path
@answer n=14 entity=surface#1.14 v=no
why: the group header and label reading in the open list is engine behaviour the session does not change
@answer n=15 entity=surface#1.15 v=no
why: the accessibility tree reading of ComboBox and expanded lives in blitz-dom accessibility.rs, which the session does not reach
@answer n=16 entity=surface#1.16 v=no
why: a row's snapshot bounds and its screenshot crop are unchanged, the session moving only the screenshot refusal decision after the crop
@answer n=17 entity=surface#1.17 v=no
why: the initial theme default in blitz-shell is unchanged by the session
@answer n=18 entity=surface#1.18 v=no
why: the default viewport and logical-size rule in blitz-traits is unchanged by the session
@answer n=19 entity=surface#1.19 v=no
why: the JS innerWidth, innerHeight and devicePixelRatio bootstrap in blitz-vibey-script is unchanged
@answer n=20 entity=surface#2.1 v=no
why: the macOS browser tabstrip padding and height in apps/browser are untouched by the session
@answer n=21 entity=surface#2.2 v=no
why: the shell's minimize, maximize, decorations and drag provider in blitz-shell is unchanged
@answer n=22 entity=surface#3.1 v=no
why: the custom widget's HTML layer opacity in examples/custom_widget.rs is unchanged by the session
@answer n=23 entity=surface#4.1 v=no
why: devtools overlay drawing in blitz-traits is unchanged, and the session does not reach it
@answer n=24 entity=surface#5.1 v=no
why: the browser window title following the active tab in apps/browser and blitz-shell is unchanged
@answer n=25 entity=surface#5.2 v=no
why: the rdme window title and light and dark toggle in apps/readme are unchanged by the session
