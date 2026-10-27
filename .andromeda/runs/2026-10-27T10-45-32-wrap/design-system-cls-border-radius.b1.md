# Classified — design-system · border-radius.b1

## Answers
border-radius#1.1 · no · the cited radii sit in apps/browser/assets/browser.css, which the take does not reach
border-radius#1.2 · no · the cited about-page CSS (about-history.css, about-newtab.css) is not among the reached files
border-radius#1.3 · no · the pointer row is re-read; its layout-templates target is ratified and the wording stands
border-radius#1.4 · no · examples/transparent/src/app.rs is not among the reached files
border-radius#1.5 · no · packages/blitz-dom/assets/default.css is not reached, and the :open touch reaches details and dialog styling, not this button radius
border-radius#1.6 · no · blitz-paint's form_controls.rs is not reached, and the take edits no paint code
border-radius#1.7 · no · blitz-paint's render.rs is not reached, and the take edits no paint code
border-radius#2.1 · no · blitz-paint's render.rs is not reached, and the take edits no paint code
border-radius#2.2 · no · blitz-paint's clip_path.rs is not reached, and the take edits no paint code
border-radius#2.3 · yes · stale · stylo.rs is a merge surface that the :open change reaches, so the cited block at lines 1124–1145 may have moved; re-point it by the take's line map
border-radius#3.1 · no · the examples/assets fixtures it cites are not among the reached files
border-radius#3.2 · no · the examples/assets fixtures it cites are not among the reached files
border-radius#3.3 · no · the inline-flex-transform fixture and examples/transforms.rs are not reached, and the flow change moves no CSS value
border-radius#3.4 · no · the examples/assets and examples/preact files it cites are not among the reached files
border-radius#4.1 · yes · stale · the take reaches style merge surfaces (stylo.rs, node/element.rs, runtime/css.rs) and the 15 s05 files are not named in the batch, so the no-match can have moved; re-run it over the set
border-radius#4.2 · yes · stale · the take reaches style merge surfaces (stylo.rs, node/element.rs, mutator.rs) and the 17 listed s06 files are not named in the batch, so the no-match on border-radius, box-shadow and z-index can have moved; re-run it over the list
border-radius#4.3 · yes · stale · the take reaches style merge surfaces (stylo.rs, node/element.rs) and the 8 s07 slice files are not named in the batch, so the radius no-match can have moved; re-run it over the slice
border-radius#4.4 · yes · stale · the take reaches style merge surfaces (stylo.rs, node/element.rs) and the 16 listed s08 files are not named in the batch, so the recorded no-match can have moved; re-run it over the list
border-radius#4.5 · yes · stale · the take reaches merge surfaces and its module split may add files to the 32 s10 slice, so the radius no-match can have moved; re-run it over the slice
border-radius#4.6 · yes · stale · the take reaches merge surfaces and the 21 listed s11 files are not named in the batch, so the radius, shadow, elevation and z-index no-match can have moved; re-run it over the list
