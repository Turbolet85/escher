@carried n=1 label=design-system/typography fact=fct-zmi56wtl
why: a probe check placed in tests/blitz-tests that reads layout (both layout modes) must build blitz-dom with the system-fonts feature, so that text is measured and no font-dependent reading passes vacuously.
quote: `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously
@carried n=2 label=design-system/typography fact=fct-vdyur6r3
why: the probe runs headlessly on the seven_guis stand; the stand's font registration stays an architecture contract, so the probe must not add it or any other value as a design-system typography value.
quote: The seven_guis headless stand's font registration is held in architecture §Standard Contracts → Headless stand, beside `font_ctx()`, not as a design-system typography value
@binding n=1
**typography system-fonts guard ↔ test-plan**: a probe check in tests/blitz-tests whose verdict depends on layout must run with text measured (system-fonts on), not at 0x0 (design-system/typography fct-zmi56wtl)
@binding n=2
**typography stand-font pointer ↔ architecture §Standard Contracts → Headless stand**: the stand's font comes from architecture's `font_ctx()`; the probe adds no typography value (design-system/typography fct-vdyur6r3)
