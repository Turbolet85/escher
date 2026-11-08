@answer n=1 entity=screen-reader-support#8.2 v=yes
why: the graph is built through the accesskit tree in dioxus-native-dom, which exists only behind the `accessibility` feature this entity names, so the new code must sit behind that gate
@answer n=2 entity=screen-reader-support#9.1 v=no
why: the lang/xml:lang element-language rule; language and direction are not_needed item 8, and the chunk touches no language reading
@answer n=3 entity=screen-reader-support#9.2 v=no
why: the default stylesheet's dir-attribute bidi isolation; language and direction are not_needed item 8
@answer n=4 entity=screen-reader-support#9.3 v=no
why: the fixtures' html lang attributes; language is not_needed item 8 and fixture text alternatives are not_needed item 8
@answer n=5 entity=screen-reader-support#9.4 v=no
why: pages declaring lang="en"; language is not_needed item 8
@answer n=6 entity=screen-reader-support#10.1 v=no
why: the absent html lang attribute in the other s02 fixtures; language is not_needed item 8
@answer n=7 entity=screen-reader-support#11.1 v=no
why: the github fixture's sr-only tooltip element; fixture text alternatives are not_needed item 8
@answer n=8 entity=screen-reader-support#11.2 v=no
why: the google fixture's descriptive aria-labels; fixture text alternatives are not_needed item 8
@answer n=9 entity=screen-reader-support#11.3 v=no
why: the google fixture's aria-hidden decoration and aria-atomic options; hidden-content exclusion is not_needed item 6 and fixture text alternatives are not_needed item 8
@answer n=10 entity=screen-reader-support#11.4 v=no
why: the servo-new burger's aria-hidden bar spans; hidden-content exclusion is not_needed item 6 and fixture text alternatives are not_needed item 8
@answer n=11 entity=screen-reader-support#11.5 v=no
why: fixture image alt text on servo.html; fixture text alternatives are not_needed item 8
@answer n=12 entity=screen-reader-support#11.6 v=no
why: the servo_header_reduced fixture image with no alt attribute; fixture text alternatives are not_needed item 8
@answer n=13 entity=screen-reader-support#12.1 v=no
why: IME enable and cursor area on text-input focus; IME is not_needed item 8 and the chunk changes no focus rule
@answer n=14 entity=screen-reader-support#12.2 v=no
why: IME enabled for an input focused before its editor is built; IME is not_needed item 8 and the chunk changes no focus rule
@answer n=15 entity=screen-reader-support#12.3 v=no
why: IME commit, preedit and disable events applied to the text editor; IME is not_needed item 8
@answer n=16 entity=screen-reader-support#13.1 v=no
why: the absence of accessibility API use across the listed s06 files; the chunk reads no s06 file and changes none of them
