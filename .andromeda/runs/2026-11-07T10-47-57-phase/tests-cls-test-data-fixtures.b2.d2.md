@answer n=1 entity=test-data-fixtures#3.24 v=no
why: browser entry helpers in apps/browser are not reached by the Dioxus render path
@answer n=2 entity=test-data-fixtures#3.25 v=no
why: the .test TLD test URLs belong to apps/browser tests, which the chunk does not reach
@answer n=3 entity=test-data-fixtures#3.26 v=no
why: the favicon bytes belong to apps/browser favicon tests, which the chunk does not reach
@answer n=4 entity=test-data-fixtures#3.27 v=no
why: the fetch body belongs to blitz-vibey-script tests, and ScriptDocument is not a Dioxus document
@answer n=5 entity=test-data-fixtures#3.28 v=yes
why: the LoadNet answer LOADED_CSS is read by stand_load_page, stand_keyboard and stand_contrast, which the chunk re-asserts as witnesses of unchanged stand behaviour through the buffered poll
@answer n=6 entity=test-data-fixtures#3.29 v=no
why: the render_blocking_stylesheet answer cases are not a Dioxus document check the chunk re-asserts or changes
@answer n=7 entity=test-data-fixtures#3.30 v=no
why: the loopback listeners belong to blitz-net unit tests, a crate the chunk does not reach
@answer n=8 entity=test-data-fixtures#4.1 v=no
why: the absence of fixture files in slice s08 binds nothing, since the chunk adds no fixture or loader
