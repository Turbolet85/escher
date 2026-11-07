@answer n=1 entity=screen-reader-support#8.2 v=yes
why: the chunk edits dioxus-native-dom lib.rs and renders the windowed dioxus-native launch, whose platform-adapter tree refresh sits behind the accessibility feature this entity states
@answer n=2 entity=screen-reader-support#9.1 v=no
why: lang reading is blitz-dom's own element-language behaviour, and no scope unit names lang or the attribute path that would carry it
@answer n=3 entity=screen-reader-support#9.2 v=no
why: the dir attribute selectors in blitz-dom's default stylesheet are styling of a plain attribute the scope does not name
@answer n=4 entity=screen-reader-support#9.3 v=no
why: html lang attributes in the eleven fixtures are fixture content the chunk neither edits nor re-asserts
@answer n=5 entity=screen-reader-support#9.4 v=no
why: page lang declarations in the examples are not read by the focus, tree or changed-set checks the chunk re-asserts
@answer n=6 entity=screen-reader-support#10.1 v=no
why: the absence of lang in other s02 fixtures is a fixture fact with no rule the chunk's code or tests must honour
@answer n=7 entity=screen-reader-support#11.1 v=no
why: the sr-only tooltip in the GitHub fixture is fixture markup, and the chunk changes no fixture or text alternative path
@answer n=8 entity=screen-reader-support#11.2 v=no
why: aria-label values on the google fixture links are accessible names, which the chunk does not touch
@answer n=9 entity=screen-reader-support#11.3 v=no
why: aria-hidden and aria-atomic in the google fixture are fixture facts outside the focus, inert and tree rules the chunk reaches
@answer n=10 entity=screen-reader-support#11.4 v=no
why: aria-hidden bar spans in the servo-new fixture are a hidden-content rule the chunk does not change
@answer n=11 entity=screen-reader-support#11.5 v=no
why: alt text on servo.html images is a fixture text alternative the chunk does not touch
@answer n=12 entity=screen-reader-support#11.6 v=no
why: a missing alt on a servo_header_reduced image is a fixture fact the chunk neither reads nor changes
@answer n=13 entity=screen-reader-support#12.1 v=yes
why: set_focus from a render-time task is the call that carries focus into the IME enable and cursor-area path this entity states for text inputs, so the replayed focus must keep it
@answer n=14 entity=screen-reader-support#12.2 v=yes
why: a render-time set_focus can land on an input before its text editor is built, and this entity pins the IME-on-mount case that the buffered render reorders
@answer n=15 entity=screen-reader-support#12.3 v=no
why: IME commit, preedit and disable event application is not touched by the focus, buffer or tree changes
@answer n=16 entity=screen-reader-support#13.1 v=no
why: an absence survey over other s06 files states no obligation on the chunk's code or tests
