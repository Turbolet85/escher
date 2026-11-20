@row_add n=1 row=decision/readme-version-sections area=project-intent fact=F1 detector=none
name: Front page states each version's status
rule: The root README states each version's status in its own section: a closed version's section stays as written, the next version is a planned section with one bullet per stage named as its route names them, and a Later line lists the user-facing items out of that version.
basis: fct-bhkgdn4h, fct-ayqd374h, fct-ausbd2si
because: The README now holds the 0.4.0 planned section with its stage bullets and a restated Later line while the closed sections stay as written, a convention for the front page that no row of Project Intent states.
@row_add n=2 row=decision/no-clipboard-crate area=stack-and-technologies fact=F2 detector=none
name: No clipboard crate
rule: Clipboard text goes through winit and no clipboard crate is taken: arboard is named in no manifest and is absent from the lockfile.
basis: fct-usf3skrs, fct-7dhvshfr
because: The stack statement now places clipboard text with winit and names no arboard, a dependency deliberately not taken that no row of Stack and Technologies states.
