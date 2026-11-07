@answer n=1 entity=spacing#1.1 v=no
why: rdme spacing tokens in the readme app's CSS, which the buffered render and its tests never read
@answer n=2 entity=spacing#2.1 v=no
why: browser chrome padding is paint of the browser UI, which the chunk neither changes nor measures
@answer n=3 entity=spacing#2.2 v=no
why: about:history padding is a browser page's paint value, which the chunk neither changes nor measures
@answer n=4 entity=spacing#2.3 v=no
why: rdme markdown body width and padding are a readme-app value outside the bridge render path
@answer n=5 entity=spacing#2.4 v=no
why: a pointer to layout-templates that states no value, and the stand_* layout-identity checks compare two layouts that share the same padding
@answer n=6 entity=spacing#2.5 v=yes
why: the ported test's get_client_rect re-measures a node's box in a blitz-dom document, which reads the default body margin the document is painted with
@answer n=7 entity=spacing#2.6 v=no
why: the default padding of multiple select rows applies to no select the chunk renders or measures
@answer n=8 entity=spacing#2.7 v=no
why: multiple-select size hints concern select lists, which the chunk does not touch
@answer n=9 entity=spacing#3.1 v=no
why: hspace and vspace are a stylo attribute mapping the buffer never interprets, and no embed or image is in the chunk's checks
@answer n=10 entity=spacing#3.2 v=no
why: body marginwidth and related attribute mappings are a stylo mapping the buffer never reads, and no test sets them
@answer n=11 entity=spacing#3.3 v=no
why: overlay scrollbar thumb geometry is a paint value that the programmatic scroll offsets and sizes do not read
@answer n=12 entity=spacing#3.4 v=no
why: open select list rows and constants belong to the select popup, which the chunk does not touch
@answer n=13 entity=spacing#3.5 v=no
why: select list placement inside the viewport belongs to the select popup, which the chunk does not touch
@answer n=14 entity=spacing#3.6 v=no
why: vertical centering places text inside a single-line input's content box, which the box read and focus checks do not measure
@answer n=15 entity=spacing#3.7 v=no
why: list marker padding is a paint value for outside markers, which the chunk neither changes nor measures
@answer n=16 entity=spacing#3.8 v=no
why: double text decoration spacing is a paint value the chunk neither changes nor measures
@answer n=17 entity=spacing#4.1 v=no
why: grid and flex gaps in the examples directory are outside the bridge render path and the chunk's checks
@answer n=18 entity=spacing#4.2 v=no
why: graphite example custom properties are an examples-page value outside the chunk's scope
@answer n=19 entity=spacing#4.3 v=no
why: hr and body padding on an examples page are outside the chunk's scope
@answer n=20 entity=spacing#4.4 v=no
why: servo.css column and button padding are an examples-page stylesheet outside the chunk's scope
@answer n=21 entity=spacing#4.5 v=no
why: Preact TodoMVC layout values are an examples-page stylesheet outside the chunk's scope
