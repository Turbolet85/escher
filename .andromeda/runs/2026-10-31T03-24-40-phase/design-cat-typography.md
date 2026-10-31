# design · Typography — category read

## Carried
- (.andromeda/design-system-detail/typography.md:35) Every check this chunk adds or extends in the blitz-tests crate must build with `system-fonts`, so that a font-dependent assertion on a text target's bounds or an option's rect fails when it should rather than passing on text measured 0x0.
  > `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously, and the feature is stated to be enabled by default when testing the whole workspace (tests/blitz-tests/Cargo.toml:17; tests/blitz-tests/tests/br_trailing_line.rs:11-13; tests/blitz-tests/tests/inline_box_baseline.rs:4-6)
- (.andromeda/design-system.md:46) The broad form of the same guard binds this chunk's fixture proofs and its new rows: no font-dependent row may count as proof unless `system-fonts` is on.
  > `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate: without it text measures 0x0 and font-dependent assertions pass vacuously.
- (.andromeda/design-system-detail/typography.md:33) A debt proven through a driver `screenshot` must run with a font context that resolves `sans-serif`, and must treat a `fonts-unavailable` refusal as no proof, never as a pass and never as a shot with its text skipped.
  > Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check (packages/blitz-dom/src/document.rs:1134-1147; as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)
- (.andromeda/design-system.md:44) The broad form of the same refusal binds any screenshot-based proof, on the stand or on a fixture, in whichever disposition the founder's word selects.
  > A driver `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped.
- (.andromeda/design-system-detail/typography.md:15) If the founder's word takes a painted select popup, its option text must resolve to the family the default stylesheet states for selects; the row states no weight, size or tracking, so a popup that fixes one must record it as a new value at the wrap.
  > | blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated |

## Bindings
- **Stand font ↔ architecture §Standard Contracts → Headless stand**: a screenshot-based proof on the stand depends on the bundled font registered beside `font_ctx()`; any change to that registration belongs to architecture, not to a typography value here, (.andromeda/design-system-detail/typography.md:29) and (.andromeda/design-system-detail/typography.md:33)
- **`system-fonts` guard ↔ test-plan**: the chunk's new rows and fixtures run `for incremental in [false, true]` under the test plan, and each font-dependent assertion in either mode needs the feature on to mean anything, (.andromeda/design-system-detail/typography.md:35)
- **Popup option text ↔ a11y-plan**: if a popup is painted, the family this row states carries the a11y-plan's minimum-size and SC 1.4.3 contrast requirements for that text, (.andromeda/design-system-detail/typography.md:15)
