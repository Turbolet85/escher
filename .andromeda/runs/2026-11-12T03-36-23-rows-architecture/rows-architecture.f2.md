@row_add n=1 row=decision/render-blocking-head-stylesheets area=established-decisions rec=1,2
name: Render-blocking head stylesheets
rule: `<head>` stylesheets are render-blocking critical resources, and `resolve` returns before styling while any are pending.
guard: render_blocking_stylesheet
@row_add n=2 row=decision/srcdoc-over-src area=established-decisions rec=1,2
name: srcdoc takes precedence over src
rule: An iframe's `srcdoc` takes precedence over its `src`.
@row_add n=3 row=decision/image-cache-by-url area=established-decisions rec=1,2
name: Images cached by URL, one fetch
rule: Loaded images are cached by URL, and concurrent requests for one URL are queued on one fetch.
@row_add n=4 row=decision/author-stylesheet-order area=established-decisions rec=1,2
name: Author stylesheets ordered by owner node id
rule: Author stylesheets are ordered by owner node id.
@row_add n=5 row=decision/overflow-auto-as-scroll area=established-decisions rec=3,4
name: overflow auto laid out as scroll
rule: `overflow: auto` is laid out as scroll overflow.
why: Stylo or Taffy lack the feature, so the engine approximates it.
@row_add n=6 row=decision/min-max-sizing-keywords-auto area=established-decisions rec=3,4
name: Sizing keywords on min/max size as auto
rule: Sizing keywords on min/max size resolve to auto.
why: Stylo or Taffy lack the feature, so the engine approximates it.
@row_add n=7 row=absence/shadow-dom area=established-decisions rec=5,6
capability: Shadow DOM
@row_add n=8 row=absence/container-queries area=established-decisions rec=5,6
capability: Container queries
@row_add n=9 row=absence/3d-transforms area=established-decisions rec=5,6
capability: 3D transforms (a z-axis rotate is applied as planar)
@row_add n=10 row=absence/css-offset-in-transforms area=established-decisions rec=5,6
capability: CSS `offset` in transforms
@row_add n=11 row=absence/visited-link-styling area=established-decisions rec=5,6
capability: Visited-link styling
@row_add n=12 row=absence/anchor-positioning area=established-decisions rec=5,6
capability: Anchor positioning
@row_add n=13 row=absence/grid-subgrid area=established-decisions rec=5,6
capability: Grid subgrid
@row_add n=14 row=absence/grid-masonry area=established-decisions rec=5,6
capability: Grid masonry
@row_add n=15 row=absence/anonymous-table-wrappers area=established-decisions rec=5,6
capability: Anonymous table wrapper boxes
@row_add n=16 row=decision/unsupported-pseudo-classes-false area=established-decisions rec=5,6
name: Unsupported pseudo-classes never match
rule: `:focus-visible`, `:valid`, `:invalid`, `:required` and `:target` always match false.
@row_add n=17 row=decision/focus-within-by-state area=established-decisions rec=5,6
name: :focus-within matches by element state
rule: `:focus-within` matches by element state.
