# Classified — design-system · border-radius.b1

## Answers
border-radius#1.1 · no · the browser chrome radii sit in apps/browser/assets/browser.css, which the session did not edit
border-radius#1.2 · no · the about-page radii sit in apps/browser about-history.css and about-newtab.css, which the session did not edit
border-radius#1.3 · no · the stand's radii are held in layout-templates and the stand's own styles, none reached; the session edited examples/seven_guis tests only
border-radius#1.4 · no · examples/transparent/src/app.rs is not in the changed files
border-radius#1.5 · no · default.css edits all start past line 183, so the blitz-dom default button rule at line 108 still holds
border-radius#1.6 · no · form_controls.rs is not in the changed files, and the checkbox frame it holds is unmoved
border-radius#1.7 · yes · stale · render.rs gains lines from line 8 on (paint_select_list at 246-247), so the scrollbar thumb citation at 826-836 has moved
border-radius#2.1 · yes · stale · render.rs gains lines from line 8 on, so the per-corner radius citation at 1398-1415 has moved
border-radius#2.2 · no · clip_path.rs is not in the changed files, and the list is not clipped by an inset() clip-path
border-radius#2.3 · yes · stale · stylo.rs edits start past line 1124, so the image border attribute citation at 1140-1161 has moved
border-radius#3.1 · no · examples/assets fixtures are not in the changed files
border-radius#3.2 · no · examples/assets fixtures are not in the changed files
border-radius#3.3 · no · the inline-flex-transform fixture and examples/transforms.rs are not in the changed files
border-radius#3.4 · no · servo.css, shadow.html and the preact examples are not in the changed files
border-radius#4.1 · no · the report names no radius for the list, so the border-radius absence sweep still holds
border-radius#4.2 · no · the list stacks by paint order rather than z-index, and the report names no radius or shadow for it, so the handling absence still holds
border-radius#4.3 · no · the report names no radius for the list, so the radius absence sweep still holds
border-radius#4.4 · no · the report names no radius for the list, so the radius absence sweep still holds
border-radius#4.5 · no · the report names no radius for the list, so the radius definitions absence sweep still holds
border-radius#4.6 · no · the report names no radius, shadow or elevation for the list, so the absence sweep still holds
