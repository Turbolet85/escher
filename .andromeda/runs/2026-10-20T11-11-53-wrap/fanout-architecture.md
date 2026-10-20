proposals:
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "The crate re-exports 57 names, not 56, and the command and refusal schema 29, not 28: since 2026-10-20-large-screen-in-parts `MAX_PART` (`pub const MAX_PART: u32 = 1000`) is re-exported from the crate root."
    sidecar: "architecture §Standard Contracts → Driver session: re-export counts 56 → 57 and 28 → 29, `MAX_PART` added (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Symbols / APIs: `pub const MAX_PART: u32 = 1000` (`schema.rs:17-19`), re-exported from the crate root (`lib.rs:156-157`). architecture.md:196 opens with «the crate re-exports 56 names … the 28 of the command and refusal schema», which no longer counts it. No pattern in the report's sweep covers this count."
    basis: "packages/escher-driver/src/lib.rs:156-157"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "In the verb table, `snapshot` (no argument; result `text`) becomes `snapshot` (one optional `part`, `ArgKind::Part`, help «the part of the screen to read, counted from 1; a screen over the budget is read in parts without it from part 1»; result `text`, then `part` and `parts`, both `FieldKind::Count` with `always: false`: the part this answer holds and how many parts the screen reads as, present only when the screen is read in parts). Its help and `text`'s help state the five paging rules (since 2026-10-20-large-screen-in-parts)."
    sidecar: "architecture §Standard Contracts → Driver session: `snapshot` takes an optional `part` and gains the result fields `part` and `parts` (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Symbols / APIs (SNAPSHOT) and Counts / qualifiers moved (`snaparg`: architecture.md:196 «`snapshot` (no argument; result `text`)», changed); Schema / config: snapshot's optional `part` argument and two `always: false` result fields."
    basis: "packages/escher-driver/src/schema.rs:178-216"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "The `ArgKind` list gains a seventh kind, `Part` (a whole number, 1 to `MAX_PART` = 1000; `snapshot`'s `part`, since 2026-10-20-large-screen-in-parts), after `Name`, the sixth."
    sidecar: "architecture §Standard Contracts → Driver session: argument kinds six → seven, `ArgKind::Part` 1 to `MAX_PART` = 1000 (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Counts / qualifiers moved: «Argument kinds: six → seven» — architecture.md:196 @c11617, the `ArgKind` list, changed; Symbols / APIs: `ArgKind::Part` (`schema/spec.rs:19-21`), name `part`, bound «a whole number from 1 to 1000»."
    basis: "packages/escher-driver/src/schema/spec.rs:19-21"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "`Command` is exactly `Snapshot { part: Option<u32> }` (no longer a bare `Snapshot`) · `Click { id }` · … The validation admits `snapshot` with no argument, or with a `part` in `1..=MAX_PART` read through `u32::try_from`. Any other number is `OutOfBound` and a text or flag value is `WrongKind`, both `malformed`."
    sidecar: "architecture §Standard Contracts → Driver session: `Command::Snapshot` now carries `part: Option<u32>`, validated to 1..=`MAX_PART` (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Symbols / APIs: «`Command::Snapshot` is now `Snapshot { part: Option<u32> }`», validate's admission and refusals (`command.rs:139-144`). architecture.md:196 states «`Command` is exactly `Snapshot` · `Click { id }` · …»."
    basis: "packages/escher-driver/src/command.rs:139-144"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "`Outcome` is exactly `Screen { text }` · `Acted { … }` · `Advanced { … }` · `Scrolled { … }` · `Shot { … }` · `Linted { findings }` · `Part { text: String, part: u32, parts: u32 }` (since 2026-10-20-large-screen-in-parts, packages/escher-driver/src/execute.rs:32-41)."
    sidecar: "architecture §Standard Contracts → Driver session: `Outcome::Part { text, part, parts }` added to the exhaustive `Outcome` list (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Symbols / APIs: «`Outcome::Part { text: String, part: u32, parts: u32 }` (`execute.rs:32-41`)», a public enum matched exhaustively. architecture.md:196's «`Outcome` is exactly …» list ends at `Linted`."
    basis: "packages/escher-driver/src/execute.rs:32-41"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "«`snapshot` returns the text of `DioxusDocument::snapshot` through `Snapshot::to_text` and dispatches nothing» now states the part read. The screen is read once through `read_screen`, which records every id of the whole screen. With no `part` and `to_text()` at most `SNAPSHOT_TEXT_BUDGET` bytes, the answer is `Outcome::Screen { text }` as before. Otherwise the screen is cut by `text_parts(SNAPSHOT_TEXT_BUDGET)` and the answer is `Outcome::Part` with part `n` (`part`, or 1 when absent) of `parts` (saturating `u32::try_from`), or the empty text when `n` > `parts`. Nothing is dispatched (since 2026-10-20-large-screen-in-parts, the founder's F1/F2)."
    sidecar: "architecture §Standard Contracts → Driver session: the executor's `snapshot` arm pages a screen over the budget, answers `Outcome::Part`, and answers the empty part past the last (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Symbols / APIs: the `Command::Snapshot { part }` arm (`execute.rs:188-204`), default paging over the budget and the empty text past the last part; Decisions: the founder's F1 «Аргумент part, части по умолчанию (Рекомендую)» and F2 «Пустая часть (Рекомендую)» (inputs#I2). architecture.md:196 says `snapshot` returns `to_text` alone."
    basis: "packages/escher-driver/src/execute.rs:188-204"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "The record of ids, fed by «the screen `snapshot` returns», states that a part read records every id of the whole screen it read, not only the ids of the part it answers (`read_screen`)."
    sidecar: "architecture §Standard Contracts → Driver session: a part read feeds the record of ids from the whole screen (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Symbols / APIs: the arm «reads the screen once through `read_screen`, which records every id of the whole screen (`execute.rs:188-204`)»; Deviations: «Step 7 was followed (`read_screen`)». Expected amendments: «a part read feeding the record of ids as `snapshot` does». architecture.md:196's record-of-ids clause names «the screen `snapshot` returns», and with a part read that is only a part."
    basis: "packages/escher-driver/src/execute.rs:188-204"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "The `Outcome::to_json` forms gain a paged `snapshot`'s outcome, `{\"text\":…,\"part\":n,\"parts\":m}` (packages/escher-driver/src/json.rs:330-336). A whole screen's line is still `{\"text\":…}` alone, since `part` and `parts` are written only on a paged answer."
    sidecar: "architecture §Standard Contracts → Driver session: `Outcome::to_json` writes `Outcome::Part` as `{\"text\",\"part\",\"parts\"}` (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Symbols / APIs: «`Outcome::to_json` writes `Outcome::Part` as `{\"text\":…,\"part\":n,\"parts\":m}` (`json.rs:330-336`)»; Schema / config: «A whole screen's line is still `{\"text\":…}` alone». architecture.md:196's to_json passage lists each outcome's written form, the shot's and the lint's included."
    basis: "packages/escher-driver/src/json.rs:330-336"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "The command line's flag list «`--id`, `--text`, `--key` and `--ms` each followed by one value» adds `--part`, its value read as a number as `--ms`'s is. The usage line, built from the table, spells `snapshot [--part <part>]` (since 2026-10-20-large-screen-in-parts)."
    sidecar: "architecture §Standard Contracts → Driver session: the command line reads `--part` as a number; usage `snapshot [--part <part>]` (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Symbols / APIs: «The command line reads a `Part` flag's value as a number, as `Milliseconds` (`cli.rs:253-257`). The usage line now spells `snapshot [--part <part>]`». architecture.md:196's argv-reader clause names the four valued flags."
    basis: "packages/escher-driver/src/cli.rs:253-257"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "The MCP result clause «an accepted `snapshot` one text block, the screen text itself» now says the paged form too: a whole screen is still the bare screen text, and a paged `snapshot` is one text block holding its JSON line `{\"text\",\"part\",\"parts\"}` (mcp.rs's generic arm, unchanged). The tool's `inputSchema` gives `part` as `{\"type\":\"integer\",\"minimum\":1,\"maximum\":1000}` (packages/escher-driver/src/mcp/listing.rs:85-89)."
    sidecar: "architecture §Standard Contracts → Driver session: the MCP `snapshot` result is two-formed (bare text whole, JSON line paged), and its `part` is listed as integer 1..1000 (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Counts / qualifiers moved: «The MCP `snapshot` result is now two-formed. A whole screen is still the bare `to_text`; a paged one is the JSON line … (mcp.rs's generic arm)»; Symbols / APIs: the MCP listing for `ArgKind::Part` (`mcp/listing.rs:85-89`). architecture.md:196 states the snapshot result as the screen text alone."
    basis: "packages/escher-driver/src/mcp/listing.rs:85-89"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "The `command` span clause «`snapshot` … record `verb` alone» adds that a part read records `verb` alone too. Its never-fielded list gains a part's number, the part count and a part's size or text."
    sidecar: "architecture §Standard Contracts → Driver session: a part read's `command` span records `verb` alone, and no field holds a part's number, count, size or text (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Symbols / APIs: «`record_result` puts `Outcome::Part` in the arm that records `verb` alone (`execute.rs:308-313`)»; Schema / config: the span records `verb` alone for a part read «with no field for a part's number, count, size or text», escher-driver keeping 2 `info_span!` sites."
    basis: "packages/escher-driver/src/execute.rs:308-313"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: "The text form's «no `Display` impl, no second entry point, no parser of the text, no JSON form and no truncation or paging» now names the parts reader. `to_text` itself still never truncates or pages. Beside it, since 2026-10-20-large-screen-in-parts, is `Snapshot::text_parts(&self, budget: usize) -> Vec<TextPart>`, with `pub struct TextPart { pub text: String, pub ids: Vec<String> }` (`Debug`, `Clone`, `Default`, `PartialEq`, `Eq`) re-exported from the crate root under `accessibility`. The reader takes consecutive pre-order lines of `to_text` while a part stays within `budget` bytes and never splits a line: a line longer than `budget` is a part alone, and an empty snapshot is one empty part. The parts concatenated are `to_text()`, and the two share the private `each_line` walk. It has no document access, failure path, log or print. `SNAPSHOT_TEXT_BUDGET` is still never enforced by `to_text`, and `text_parts` is the reader that cuts the text into parts within it."
    sidecar: "architecture §Standard Contracts → Dioxus DOM bridge: `Snapshot::text_parts` and `TextPart` added beside `to_text`, which still never pages (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Symbols / APIs (dioxus-native-dom): `TextPart` (snapshot_text.rs:19-28, `lib.rs:39`), `text_parts` (59-81), `each_line` shared and `to_text` unchanged, `SNAPSHOT_TEXT_BUDGET`'s doc (15-16); Counts / qualifiers moved: `budget` architecture.md:209 @c21207 changed; Spec claims: «no truncation or paging» stays true of `to_text»."
    basis: "packages/dioxus-native-dom/src/snapshot_text.rs:59-81"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: "«`to_text` has one caller outside the tests, escher-driver's executor, which returns the text as `Outcome::Screen`» adds what the executor now does with it. It compares `to_text()` with `SNAPSHOT_TEXT_BUDGET`, and since 2026-10-20-large-screen-in-parts it calls `text_parts(SNAPSHOT_TEXT_BUDGET)` for a screen over the budget or a `part` asked, returning that part as `Outcome::Part`. `text_parts`' one caller outside the tests is that executor."
    sidecar: "architecture §Standard Contracts → Dioxus DOM bridge: the executor is the outside caller of `text_parts` and returns `Outcome::Part` (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Symbols / APIs: the `Command::Snapshot { part }` arm (`execute.rs:188-204`) answers `Outcome::Screen` within the budget and otherwise cuts `text_parts(SNAPSHOT_TEXT_BUDGET)` into `Outcome::Part`. architecture.md:209 names `Outcome::Screen` as the executor's only return of the text."
    basis: "packages/escher-driver/src/execute.rs:188-204"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Dioxus DOM bridge"
    change: "«to an MCP client as the `snapshot` tool's result, the screen text itself» now says: the screen text itself for a whole screen, and for a screen read in parts (since 2026-10-20-large-screen-in-parts) the paged JSON line `{\"text\",\"part\",\"parts\"}` holding one part's text. Likewise the command line's stdout line gains `part` and `parts` beside `text` on a paged answer."
    sidecar: "architecture §Standard Contracts → Dioxus DOM bridge: the snapshot text's exits include the paged answer line (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Counts / qualifiers moved: «The MCP `snapshot` result is now two-formed … a paged one is the JSON line `{\"text\",\"part\",\"parts\"}`», and the paged answer line on the cli surface. architecture.md:209 states the MCP result as «the screen text itself»."
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Inherited Defaults → workspace crates (dioxus-native-dom module row)"
    change: "The snapshot_text module entry «the snapshot's text form `Snapshot::to_text`, its `SNAPSHOT_TEXT_BUDGET` constant re-exported» adds the parts reader `Snapshot::text_parts`. It reads «its `SNAPSHOT_TEXT_BUDGET` constant and, since 2026-10-20-large-screen-in-parts, its `TextPart` type re-exported»."
    sidecar: "architecture dioxus-native-dom module row: snapshot_text gains `Snapshot::text_parts` and the re-exported `TextPart` (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Counts / qualifiers moved (`budget`): architecture.md:351, dioxus-native-dom's module row «its `SNAPSHOT_TEXT_BUDGET` constant re-exported», gains `TextPart`; Symbols / APIs: `pub use snapshot_text::{SNAPSHOT_TEXT_BUDGET, TextPart}` (`lib.rs:39`)."
    basis: "packages/dioxus-native-dom/src/lib.rs:39"
  - detector: D-arch-resources
    severity: warning
    section: "§Inherited Defaults → workspace crates (blitz-tests shared modules)"
    change: "`tests/session_common/mod.rs` is read by 29 checks (was 28). `tests/fixtures/mod.rs` is read by 24 (was 23), 22 of them `stand_*` files (was 21), `font_face_sniffed_source` and `select_interaction`. The new `stand_snapshot_parts` declares both modules."
    sidecar: "architecture blitz-tests shared modules: session_common readers 28 → 29, fixtures readers 23 → 24 (`stand_*` 21 → 22), from `stand_snapshot_parts` (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Counts / qualifiers moved: «Shared-module readers. `tests/session_common/mod.rs` readers go 28 → 29, and `tests/fixtures/mod.rs` readers 23 → 24 (`stand_*` 21 → 22), since `stand_snapshot_parts` declares both — `readers`: … architecture.md:368» changed."
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Driver session]"
    change: "After the lint's-findings clause, add the paged screen. Since 2026-10-20-large-screen-in-parts a screen over `SNAPSHOT_TEXT_BUDGET` crosses those carriers in parts: `snapshot`'s optional `part` comes in, a whole number from 1 to `MAX_PART` = 1000. Out goes one part's snapshot text with the closed counts `part` and `parts`, a screen within the budget still answering whole. A part past the last answers the empty text, with no new cause. On MCP the paged answer is its JSON line. There is no new verb, carrier, exit, cause, session error, span field, wire form or dependency. This is the founder's F1 «Аргумент part, части по умолчанию (Рекомендую)» and F2 «Пустая часть (Рекомендую)» (inputs#I2), as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md."
    sidecar: "architecture §Established Decisions → [Driver session]: the paged snapshot (`part` in, part text with `part`/`parts` out, the empty part past the last) recorded on the founder's F1/F2 (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Decisions & corrections: «The founder's rulings on the fork, by dialog (inputs#I2): F1 … an optional `part`, default paging over the budget, the paged MCP answer as its JSON line; F2 … a part past the last answers the empty text, no new cause». Expected amendments name the playbook «Boundary widening», answered by that word. This [Driver session] decision records each ratified change to what crosses the carriers (the shot's pixels, the lint's findings), and the paged answer's shape is not yet among them."
note: "Every Driver-session and Dioxus-bridge proposal, and the Established Decisions entry, rides the founder's F1/F2 (inputs#I2), which the report says P2 halts on once as the answer card. Not proposed: the §Occupied Resources socket and MCP passage, because the paged text is snapshot text and the part number and counts are closed numbers inside the ratified carriers. Also not proposed: architecture.md:354's «six shape types», which counts types (the report says no-change), and the MCP listing's byte size, which the report does not measure."
