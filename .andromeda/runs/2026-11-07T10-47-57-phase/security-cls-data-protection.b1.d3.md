@answer n=1 entity=data-protection#1.1 v=yes
why: the chunk must keep kept checks identical (scope needs 7), and this entity fixes their at-rest content, including MASKED_VALUE for password and file inputs and the snapshot text and diffs
@answer n=2 entity=data-protection#2.1 v=yes
why: the committed Counter checks fix the snapshot text and diffs that the chunk must keep identical (scope needs 7), and the stand_* witnesses re-measure them
