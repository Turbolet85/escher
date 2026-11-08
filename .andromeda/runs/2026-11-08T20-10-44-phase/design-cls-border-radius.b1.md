@answer n=1 entity=border-radius#1.1 v=no
why: the browser chrome radii are on the browser app's CSS, which the handler-coverage command and the replay test never touch
@answer n=2 entity=border-radius#1.2 v=no
why: the browser about-page radii are on the browser app's CSS, which the chunk does not draw or measure
@answer n=3 entity=border-radius#1.3 v=no
why: the stand's radii are held in layout-templates, not design-system values, and the chunk paints no stand card or select
@answer n=4 entity=border-radius#1.4 v=no
why: the transparent example card's 16px radius is an example app value the chunk does not touch
@answer n=5 entity=border-radius#1.5 v=no
why: the blitz-dom default button radius is engine styling the chunk does not paint or measure
@answer n=6 entity=border-radius#1.6 v=no
why: the engine checkbox frame radius is painted by blitz-paint, which the chunk only reads through the graph and does not paint through
@answer n=7 entity=border-radius#1.7 v=no
why: the engine scrollbar thumb radius is painted by blitz-paint, and the chunk adds no scrolling and measures no radius
@answer n=8 entity=border-radius#2.1 v=no
why: per-corner elliptical radius resolution in blitz-paint is engine painting the chunk neither calls nor changes
@answer n=9 entity=border-radius#2.2 v=no
why: the inset clip-path TODO is a blitz-paint clip rule the chunk does not exercise, and the coverage read dispatches no pointer and clips nothing
@answer n=10 entity=border-radius#2.3 v=no
why: the image border attribute maps to blitz-dom styling the coverage count and the owed replay test do not read
@answer n=11 entity=border-radius#3.1 v=no
why: the uniform, percentage and elliptical radius examples are example fixtures the chunk neither draws nor re-measures
@answer n=12 entity=border-radius#3.2 v=no
why: the google, graphite, hr and input fixture radii are example page values the chunk does not render or measure
@answer n=13 entity=border-radius#3.3 v=no
why: the inline-flex-transform button radius is an example fixture value the chunk does not render or measure
@answer n=14 entity=border-radius#3.4 v=no
why: the servo, shadow and preact example radii are example fixture values the chunk does not render or measure
@answer n=15 entity=border-radius#4.1 v=no
why: this absence over the s05 files states no radius value the chunk adds, and the chunk writes no radius code or token
@answer n=16 entity=border-radius#4.2 v=no
why: this absence over the s06 files states no radius or shadow handling the chunk adds, and the coverage read changes no screen
@answer n=17 entity=border-radius#4.3 v=no
why: this absence over the s07 slice files states no radius value the chunk adds, and the chunk writes no radius code or token
@answer n=18 entity=border-radius#4.4 v=no
why: this absence over the s08 files records no radius match, and the chunk adds no radius to find
@answer n=19 entity=border-radius#4.5 v=no
why: this absence over the s10 slice files states no radius definition the chunk adds, and the chunk writes no radius code or token
@answer n=20 entity=border-radius#4.6 v=no
why: this absence over the s11 files states no radius or elevation the chunk adds, and the chunk writes no radius code or token
