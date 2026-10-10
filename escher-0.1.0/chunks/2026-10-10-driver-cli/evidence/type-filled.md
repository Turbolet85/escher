# `type` on a filled control — the as-built reading (step 1)

Taken on the base commit `0493d26a` before any source file changed, 2026-10-10, in process, in both layout modes
(`incremental` false and true; the two agree on every figure). Instrument: a throwaway test target under
`tests/blitz-tests/tests/` that held a stand session through `session_common::hold` and ran the driver's `type`
through `Session::run`; it was deleted once these readings were recorded and is in no commit.

| Case | Control | Value before | Call | Value after | Diff `changed` |
|---|---|---|---|---|---|
| filled control, text | Flight Booker `flight-start` | `01.01.2026` | `type` text `ZQ` | `01.01.2026ZQ` | `flight-start`, `flight-book` |
| filled control, empty text | Flight Booker `flight-start` | `01.01.2026` | `type` text `` | `01.01.2026` | `flight-start` |
| empty control, text | CRUD `crud-name` | `` | `type` text `Ada` | `Ada` | `crud-name` |

The engine's editor held the same text as the snapshot's `value` in the first case (`01.01.2026ZQ`).

## What the reading says of the hypothesis

The route's hypothesis was that the typed text is "inserted inside the old value", because `type` clicks the centre
of the control and a click moves the caret to the click point. **Corrected by the measurement:** on the stand's date
input the text is appended after the old value. The input is wider than its ten characters, so the click at its
centre falls past the end of the text and the caret lands at the end. The mechanism (caret at the click point) is
not contradicted; where the caret lands depends on the control's width against its text, which is why the answer an
agent gets was not a stable one.

Either way the control did not read what was typed: `01.01.2026ZQ` is not a date, and `flight-book` joins the diff
because the booking is disabled by it. With an empty text the value is untouched — the diff names the control only
because it gained focus. Step 4 replaces this behaviour (the founder's answer to fork 8): `type` leaves exactly the
text, and an empty text clears.
