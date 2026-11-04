@answer n=1 entity=screen-reader-support#9.2 v=no
why: the blitz-dom default stylesheet's dir-attribute bidi isolation is not read or written by the probe, which adds no attribute path
@answer n=2 entity=screen-reader-support#9.3 v=no
why: a fixture count of html lang attributes; the probe reads no lang value and the stand carries none
@answer n=3 entity=screen-reader-support#9.4 v=no
why: pages declaring lang="en" name fixtures and preact examples outside the stand; the probe reads no language
@answer n=4 entity=screen-reader-support#10.1 v=no
why: an observed absence of lang attributes across other s02 fixtures; the probe reads no language and no fixture
@answer n=5 entity=screen-reader-support#11.1 v=no
why: the github fixture's sr-only tooltip element is a fixture fact; the probe does not reach that fixture or its text alternative
@answer n=6 entity=screen-reader-support#11.2 v=no
why: google fixture link aria-labels are a fixture fact; the probe's names come from the tree and no google node is on the stand
@answer n=7 entity=screen-reader-support#11.3 v=no
why: aria-hidden decoration and aria-atomic suggestions on the google fixture are hidden-content and live-region matters the chunk leaves not_needed
@answer n=8 entity=screen-reader-support#11.4 v=no
why: aria-hidden burger bars in the servo-new fixture are hidden-content matters the chunk leaves not_needed
@answer n=9 entity=screen-reader-support#11.5 v=no
why: alt text on servo.html fixture images states an image name outside the probe's reads
@answer n=10 entity=screen-reader-support#11.6 v=no
why: a servo_header_reduced fixture image with no alt attribute states an image outside the probe's reads
@answer n=11 entity=screen-reader-support#12.1 v=no
why: IME enabling on focusing a text input is not named by the scope, which focuses no text input (its keys are Enter, Space and the list's arrows)
@answer n=12 entity=screen-reader-support#12.2 v=no
why: IME on for an input focused before its editor exists is a mount-time text-editor reading the chunk neither creates nor focuses
@answer n=13 entity=screen-reader-support#12.3 v=no
why: IME commit, preedit and disable events on the text editor, with DeleteSurrounding a TODO, are text-editor behaviour the scope does not name
@answer n=14 entity=screen-reader-support#13.1 v=yes
why: the absence of AccessKit or aria integration over the listed s06 files states how the accessibility tree and author_id the probe reads through behave, which the scope names
