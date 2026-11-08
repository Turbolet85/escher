@answer n=1 entity=screen-reader-support#11.6 v=no
why: the fixture image's missing alt sits in the text-alternatives reading of servo_header_reduced.html, which the chunk neither names nor changes.
@answer n=2 entity=screen-reader-support#12.1 v=no
why: IME enable on focus is blitz-dom's focus behaviour, which the chunk does not drive, change or name, so it is a re-check every change owes.
@answer n=3 entity=screen-reader-support#12.2 v=no
why: IME on for an input focused before its editor exists is create_text_editor's behaviour, and the test pinned at dioxus_document.rs:775-809 is a unit the chunk leaves alone, so it is a re-check every change owes.
@answer n=4 entity=screen-reader-support#12.3 v=no
why: IME commit, preedit, disable events and the DeleteSurrounding TODO are text-editor event handling the chunk neither reads nor changes.
@answer n=5 entity=screen-reader-support#13.1 v=yes
why: the chunk's stable author key is carried as an AccessKit author_id on every actionable element (needs n=3), so the recorded absence of accessibility API integration no longer holds in the files the chunk reaches.
