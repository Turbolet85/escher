- line: .andromeda/architecture.md:196
old: «the crate re-exports 56 names under»
new: «the crate re-exports 57 names under»
- line: .andromeda/architecture.md:196
old: «the 28 of the command and refusal schema (named below;»
new: «the 29 of the command and refusal schema (named below;»
- line: .andromeda/architecture.md:196
old: «`snapshot` (no argument; result `text`),»
new: «`snapshot` (one optional `part`; result `text`, then `part` and `parts`, two counts written only when the screen is read in parts),»
- line: .andromeda/architecture.md:196
old: «the sixth kind, since 2026-10-10-driver-cli). The five acting verbs' result fields»
new: «the sixth kind, since 2026-10-10-driver-cli) · `Part` (a whole number, 1 to `MAX_PART` = 1000 — `snapshot`'s `part`, the seventh kind). The five acting verbs' result fields»
- line: .andromeda/architecture.md:196
old: «`Command` is exactly `Snapshot` · `Click { id }`»
new: «`Command` is exactly `Snapshot { part: Option<u32> }` (a `part` admitted in `1..=MAX_PART` through `u32::try_from`, any other number `OutOfBound`, a text or a flag `WrongKind`) · `Click { id }`»
- line: .andromeda/architecture.md:196
old: «`Outcome` is exactly `Screen { text: String }` · `Acted {»
new: «`Outcome` is exactly `Screen { text: String }` · `Part { text: String, part: u32, parts: u32 }` (packages/escher-driver/src/execute.rs:34-41) · `Acted {»
- line: .andromeda/architecture.md:196
old: «`snapshot` returns the text of `DioxusDocument::snapshot` through `Snapshot::to_text` and dispatches nothing.»
new: «`snapshot` reads the screen once, `DioxusDocument::snapshot`, and dispatches nothing: with no `part` and a text of at most `SNAPSHOT_TEXT_BUDGET` bytes it returns `Snapshot::to_text`'s text as `Outcome::Screen`; otherwise it cuts `text_parts(SNAPSHOT_TEXT_BUDGET)` and returns part `n` — `part`, or 1 when absent — as `Outcome::Part` with the count `parts`, a part past the last the empty text with the same `parts` (packages/escher-driver/src/execute.rs:188-204).»
- line: .andromeda/architecture.md:196
old: «the screen `snapshot` returns and both snapshots of a `press`»
new: «the screen `snapshot` reads — the whole screen, a part read included — and both snapshots of a `press`»
- line: .andromeda/architecture.md:196
old: «packages/escher-driver/src/json.rs:367-371); a `start` answers»
new: «packages/escher-driver/src/json.rs:367-371); a paged `snapshot`'s outcome `{"text":…,"part":n,"parts":m}`, a whole screen's still `{"text":…}` alone (packages/escher-driver/src/json.rs:330-336); a `start` answers»
- line: .andromeda/architecture.md:196
old: «`--id`, `--text`, `--key` and `--ms` each followed by one value»
new: «`--id`, `--text`, `--key`, `--ms` and `--part` each followed by one value, `--ms`'s and `--part`'s read as a number»
- line: .andromeda/architecture.md:196
old: «`start`'s `app` an `enum` of the binary's apps); `tools/call` runs one»
new: «`start`'s `app` an `enum` of the binary's apps, `snapshot`'s `part` an `integer` from 1 to 1000); `tools/call` runs one»
- line: .andromeda/architecture.md:196
old: «an accepted `snapshot` one text block, the screen text itself;»
new: «an accepted `snapshot` one text block, the screen text itself for a whole screen and its JSON line `{"text","part","parts"}` for a paged one;»
- line: .andromeda/architecture.md:196
old: «record `verb` alone, as does an accepted `lint`;»
new: «record `verb` alone, as do an accepted `lint` and a part read;»
- line: .andromeda/architecture.md:196
old: «the screen text or its length, a shot's image,»
new: «the screen text or its length, a part's number, the part count, a shot's image,»
- line: .andromeda/architecture.md:209
old: «which writes it as the `text` field of one line of JSON on stdout,»
new: «which writes it as the `text` field of one line of JSON on stdout (with `part` and `parts` beside it when the screen is read in parts),»
- line: .andromeda/architecture.md:209
old: «as the `snapshot` tool's result, the screen text itself, on the»
new: «as the `snapshot` tool's result, the screen text itself — a paged one its JSON line `{"text","part","parts"}` —, on the»
- line: .andromeda/architecture.md:209
old: «no JSON form and no truncation or paging;»
new: «no JSON form, and `to_text` itself never truncates or pages; beside it `Snapshot::text_parts(&self, budget: usize) -> Vec<TextPart>` (`TextPart { text: String, ids: Vec<String> }`, `Debug` · `Clone` · `Default` · `PartialEq` · `Eq`) cuts that text into parts — consecutive pre-order lines while a part stays within `budget` bytes, never splitting a line, a line longer than `budget` a part alone, an empty snapshot one empty part — the parts concatenated being `to_text()`, the two sharing one private pre-order walk, with no document access, failure path, log or print (packages/dioxus-native-dom/src/snapshot_text.rs:59-81);»
- line: .andromeda/architecture.md:209
old: «never enforced by `to_text`, and it is re-exported from the crate»
new: «never enforced by `to_text`, `text_parts` the reader that cuts the text within it, and it is re-exported, with `TextPart`, from the crate»
- line: .andromeda/architecture.md:209
old: «escher-driver's executor, which returns the text as `Outcome::Screen` to the caller of `Session::run`»
new: «escher-driver's executor, which returns the text as `Outcome::Screen` — or, for a screen over the budget or a `part` asked, one part of `text_parts(SNAPSHOT_TEXT_BUDGET)` as `Outcome::Part`, `text_parts`' one caller outside the tests — to the caller of `Session::run`»
- line: .andromeda/architecture.md:351
old: «the snapshot's text form `Snapshot::to_text`, its `SNAPSHOT_TEXT_BUDGET` constant re-exported)»
new: «the snapshot's text form `Snapshot::to_text` and its parts reader `Snapshot::text_parts`, its `SNAPSHOT_TEXT_BUDGET` constant and `TextPart` type re-exported)»
- line: .andromeda/architecture.md:368
old: «`tests/session_common/mod.rs`, by 28, and»
new: «`tests/session_common/mod.rs`, by 29, and»
- line: .andromeda/architecture.md:368
old: «`HeldNet`, by 23 (21 `stand_*` files,»
new: «`HeldNet`, by 24 (22 `stand_*` files,»
- line: .andromeda/architecture.md:368
old: «as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md.»
new: «as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md.»
- line: .andromeda/architecture.md:118
old: «session error, span field or dependency: a ratified boundary widening — as measured at»
new: «session error, span field or dependency: a ratified boundary widening; a screen over the snapshot budget crosses them in parts — `snapshot`'s optional `part` (a whole number, 1 to `MAX_PART` = 1000) in, one part's text with the closed counts `part` and `parts` out, a part past the last the empty text, a screen within the budget answering whole as before, over MCP a paged answer its JSON line — with no new verb, carrier, exit, cause, session error, span field or dependency: a ratified boundary widening — as measured at»
- line: .andromeda/architecture.md:118
old: «the lint's findings on the carriers at escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md»
new: «the lint's findings on the carriers at escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md, the paged screen at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md»
