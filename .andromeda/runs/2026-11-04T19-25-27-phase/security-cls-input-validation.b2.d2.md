@answer n=1 entity=input-validation#1.26 v=no
why: data: URL scheme handling sits in the windowed shell's net provider, which the probe fetches nothing through (not_needed: file: and data: URLs).
@answer n=2 entity=input-validation#1.27 v=no
why: invalid data: URL and non-UTF-8 script byte errors concern document script fetching, which the probe does not run (not_needed: script execution).
@answer n=3 entity=input-validation#1.28 v=no
why: the file: URL gate on script fetch, fetch(), module import and script src is document-script loading, outside the probe (not_needed: script execution).
@answer n=4 entity=input-validation#1.29 v=no
why: the script type attribute gate governs document script execution, which the probe does not run (not_needed: script execution).
@answer n=5 entity=input-validation#1.30 v=no
why: scripting-disabled HTML parsing concerns script and noscript content; the probe reads Dioxus listeners and runs no document scripts.
@answer n=6 entity=input-validation#1.31 v=yes
why: the stand's one stylesheet load (stand::boot_load, load.css) is the load in flight that the harness settles must not wait on, and this entry states that a failed, empty or non-CSS stylesheet releases render.
@answer n=7 entity=input-validation#1.32 v=no
why: the probe passes no href, and the stand's one stylesheet URL is fixed and resolvable, so the unresolvable-href panic path is not reached.
@answer n=8 entity=input-validation#1.33 v=no
why: iframe HTML decoding; the probe fetches no iframes (not_needed: iframes).
@answer n=9 entity=input-validation#1.34 v=no
why: fetched image format decoding; the probe fetches no images (not_needed: images).
@answer n=10 entity=input-validation#1.35 v=no
why: favicon image validation belongs to the browser app, which the probe does not run (not_needed: favicons).
@answer n=11 entity=input-validation#1.36 v=no
why: font format sniffing; the probe fetches no fonts and the stand boots its bundled font (not_needed: fonts).
@answer n=12 entity=input-validation#1.37 v=no
why: font byte length and WOFF fallback; the probe fetches no fonts (not_needed: fonts).
@answer n=13 entity=input-validation#1.38 v=no
why: @font-face source URL skipping; no @font-face source or agent-supplied URL reaches the stand (not_needed: @font-face).
@answer n=14 entity=input-validation#1.39 v=no
why: SVG data encoding and DTD handling; the probe fetches no SVG (not_needed: SVG data).
@answer n=15 entity=input-validation#1.40 v=no
why: SVGZ decompression applies to SVG data the probe never fetches (not_needed: SVG data).
@answer n=16 entity=input-validation#1.41 v=no
why: SVG viewBox number checks; the probe's apps carry no SVG (not_needed: SVG data).
@answer n=17 entity=input-validation#1.42 v=no
why: inline SVG parse during layout construction; the probe's apps carry no inline SVG (not_needed: SVG data).
@answer n=18 entity=input-validation#1.43 v=no
why: CSS declaration validation is CSS parsing, which the probe neither changes nor reads as a verdict (not_needed: CSS parsing).
@answer n=19 entity=input-validation#1.44 v=no
why: mix-blend-mode and isolation are CSS paint the probe neither sets nor reads as a verdict (not_needed: CSS parsing).
@answer n=20 entity=input-validation#1.45 v=no
why: set_style_property and remove_style_property are CSSOM calls the probe never makes, and the bridge stays read-only.
@answer n=21 entity=input-validation#1.46 v=no
why: the selectorText setter is a CSSOM mutation the probe never makes (not_needed: selector parsing).
@answer n=22 entity=input-validation#1.47 v=yes
why: the scope's query helpers panic on an invalid selector and the probe must pass only fixed valid ones, and this entry states the parse error query strings return.
@answer n=23 entity=input-validation#1.48 v=no
why: style element text entity decoding is style-element parsing, which the probe neither changes nor reads as a verdict (not_needed: style element parsing).
@answer n=24 entity=input-validation#1.49 v=no
why: bgcolor legacy attribute parsing, which the probe neither changes nor reads as a verdict (not_needed: legacy attribute parsing).
@answer n=25 entity=input-validation#1.50 v=no
why: legacy dimension attribute parsing, which the probe neither changes nor reads as a verdict (not_needed: legacy attribute parsing).
