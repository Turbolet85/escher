@answer n=1 entity=aria-patterns-roles#4.8 v=no
why: graphite.html fixture carousel aria-label text; the chunk edits and reads no fixture and the scope names none
@answer n=2 entity=aria-patterns-roles#4.9 v=no
why: servo.html nav and burger roles and labels; the chunk edits and reads no fixture and the scope names none
@answer n=3 entity=aria-patterns-roles#4.10 v=no
why: servo.html download links role=button; the chunk edits and reads no fixture and the scope names none
@answer n=4 entity=aria-patterns-roles#4.11 v=no
why: svg.html icons aria-hidden and focusable; the chunk edits and reads no fixture and the scope names none
@answer n=5 entity=aria-patterns-roles#5.1 v=no
why: absence of ARIA in app/example markup; the chunk writes no markup and the stand roles come from shared tables, so the absence holds after the change
@answer n=6 entity=aria-patterns-roles#5.2 v=yes
why: absence of accesskit and roles over s06 files; the chunk adds AccessKit author_id on element nodes and accessibility_tree reads, which the scope names, so the record stops holding where the change lands
@answer n=7 entity=aria-patterns-roles#5.3 v=no
why: paint-tree slice whose only match is the local name role_unchanged; the chunk changes no paint code
@answer n=8 entity=aria-patterns-roles#5.4 v=yes
why: only commented accesskit lines in s08; the chunk adds live AccessKit author_id and accessibility_tree reads, which the scope names
@answer n=9 entity=aria-patterns-roles#5.5 v=no
why: absence of aria- attributes; the chunk adds no aria- attribute and the ARIA state attribute mapping is not_needed
@answer n=10 entity=aria-patterns-roles#5.6 v=yes
why: absence of role usage over s10 files; the chunk's role derivation (click-reactive element reads GenericContainer, role untouched) is named by the scope
@answer n=11 entity=aria-patterns-roles#5.7 v=yes
why: absence of aria, role and accesskit over s11 files; the scope names the AccessKit author_id and the accessibility tree the chunk changes
@answer n=12 entity=aria-patterns-roles#5.8 v=no
why: absence of aria- and role= markup over s13 files; the chunk writes no markup and changes no role attribute
