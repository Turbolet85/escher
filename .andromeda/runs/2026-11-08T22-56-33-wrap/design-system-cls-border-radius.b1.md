@answer n=1 entity=border-radius#1.1 v=no
why: the browser chrome radii live in apps/browser/assets CSS, which the session's change does not reach
@answer n=2 entity=border-radius#1.2 v=no
why: the browser about-page radii live in apps/browser/assets about-*.css, which the session does not reach
@answer n=3 entity=border-radius#1.3 v=no
why: the seven_guis stand radii are held in layout-templates, and the session's seven_guis edits are host and CLI plumbing only
@answer n=4 entity=border-radius#1.4 v=no
why: the transparent example card radius sits in examples/transparent/src/app.rs, which the session does not reach
@answer n=5 entity=border-radius#1.5 v=no
why: the blitz-dom default button radius sits in packages/blitz-dom/assets/default.css, which the session does not reach
@answer n=6 entity=border-radius#1.6 v=no
why: the checkbox frame radius sits in packages/blitz-paint form_controls.rs, and the session does not reach blitz-paint
@answer n=7 entity=border-radius#1.7 v=no
why: the scrollbar thumb radius sits in packages/blitz-paint render.rs, and the session does not reach blitz-paint
@answer n=8 entity=border-radius#2.1 v=no
why: the per-corner radius resolution sits in packages/blitz-paint render.rs, which the session does not reach
@answer n=9 entity=border-radius#2.2 v=no
why: the inset clip-path TODO sits in packages/blitz-paint clip_path.rs, which the session does not reach
@answer n=10 entity=border-radius#2.3 v=no
why: the image border attribute mapping sits in packages/blitz-dom stylo.rs, which the session does not reach
@answer n=11 entity=border-radius#3.1 v=no
why: the radius syntax examples cite examples/assets HTML and CSS files, none of which the session reaches
@answer n=12 entity=border-radius#3.2 v=no
why: the google, graphite, hr and input fixture radii cite examples/assets HTML files, none of which the session reaches
@answer n=13 entity=border-radius#3.3 v=no
why: the inline-flex-transform and transforms example radii cite files the session does not reach
@answer n=14 entity=border-radius#3.4 v=no
why: the servo, shadow and preact radii cite example files the session does not reach
@answer n=15 entity=border-radius#4.1 v=no
why: the no-match claim for border-radius or border_radius holds, since the task prose names no border-radius change and the session's coverage and CLI code adds none
@answer n=16 entity=border-radius#4.2 v=no
why: the no-match claim for border-radius, box-shadow and z-index holds, since the task prose names no change to those terms
@answer n=17 entity=border-radius#4.3 v=no
why: the no-match claim for radius holds, since the task prose names no radius change in the session's files
@answer n=18 entity=border-radius#4.4 v=no
why: the no-match claim for border-radius holds, since the task prose names no radius change in the session's files
@answer n=19 entity=border-radius#4.5 v=no
why: the radius definitions claim holds, since the task prose names no radius change in the session's files
@answer n=20 entity=border-radius#4.6 v=no
why: the no-match claim for radius, shadow, elevation and z-index holds, since the task prose names none of those terms as changed
