@answer n=1 entity=screen-reader-support#11.6 v=no
why: the fixture image's missing alt is text naming the chunk does not change, and the chunk names nothing
@answer n=2 entity=screen-reader-support#12.1 v=no
why: focus-driven IME enabling is the engine's focus path, which the chunk changes no part of and the actions verb does not carry
@answer n=3 entity=screen-reader-support#12.2 v=no
why: the pinned mount-time IME test is an existing check in dioxus_document.rs that the chunk's read-only additions leave holding, a re-check every change owes
@answer n=4 entity=screen-reader-support#12.3 v=no
why: IME commit, preedit and delete-surrounding events are text-editor input the chunk does not touch
@answer n=5 entity=screen-reader-support#13.1 v=yes
why: the chunk adds the AccessKit author_id and reads role and name from accessibility_tree, the integration this absence says the listed files lack
