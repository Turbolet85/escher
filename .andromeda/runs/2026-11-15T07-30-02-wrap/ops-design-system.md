@row_add n=1 row=decision/font-face-registered-with-width area=typography fact=F1 detector=none
name: Web font faces registered with their declared width
rule: An @font-face rule registers its face under the CSS-declared family, weight, style and font-stretch (font-width), so font matching selects a web font face by its declared width as well.
why: Taken from upstream, ratified as a behaviour change for every document.
basis: fct-ysqotf4p
because: The chunk took upstream's font-stretch read into font-face registration and the founder ratified it as a behaviour change for every document, a typography decision no standing row of the area states.
