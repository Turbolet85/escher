@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dir
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/stac
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=12 class=omission
leaf-text: - Consuming builder methods named `with_*` / `without_*` (`with_virtual_time`, `without_timer_thread`).
source: dec-xrlcqzpf «decision/builder-method-naming · Consuming builder method names — Consuming builder methods are named `with_…` / `without_…`. · except: blitz-traits' Request builders are named by their field, `signal` and `initiator`.»
replace-with: - Consuming builder methods named `with_*` / `without_*` (`with_virtual_time`, `without_timer_thread`), except blitz-traits' `Request` builders, named by their field: `signal` and `initiator`.
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=22 class=omission
leaf-text: public APIs use `NodeId`.
source: dec-gcqb4uyb «decision/blitz-dom-stamped-requests · blitz-dom requests only through the stamping constructor — blitz-dom builds a network Request only through its one crate-private constructor, which stamps the loading document as the request's initiator.»
replace-with: public APIs use `NodeId`. blitz-dom builds a network `Request` only through its one crate-private constructor, which stamps the loading document as the request's initiator (guarded by blitz-dom's clippy `disallowed-methods` rule on `Request::get`, one local expect of the lint at that constructor).
@leaf_edit n=3 leaf=.claude/docs/services/blitz-dom.md line=11 class=omission
leaf-text: `ResourceHandler<StylesheetHandler>::error` responds `Err(error)`
source: ctr-f6uhjel3 «contract/stylesheet-handler/document-url · blitz-dom StylesheetHandler · document_url — StylesheetHandler carries a public document_url, the URL of the document whose linked sheet it loads, used to stamp that sheet's @import and @font-face requests.»
replace-with: `StylesheetHandler` carries a public `document_url`, the URL of the document whose linked sheet it loads, used to stamp that sheet's `@import` and `@font-face` requests; `ResourceHandler<StylesheetHandler>::error` responds `Err(error)`
@leaf_edit n=4 leaf=.claude/docs/services/blitz-dom.md line=21 class=omission
leaf-text: - Layout runs over a crate-private `LayoutPassState`
source: dec-gcqb4uyb «decision/blitz-dom-stamped-requests · blitz-dom requests only through the stamping constructor — blitz-dom builds a network Request only through its one crate-private constructor, which stamps the loading document as the request's initiator.»
replace-with: - Every network `Request` is built only through the crate's one crate-private constructor, which stamps the loading document as the request's initiator (clippy `disallowed-methods` on `Request::get`, one local expect at that constructor). Layout runs over a crate-private `LayoutPassState`
@leaf_edit n=5 leaf=.claude/docs/services/blitz-traits.md line=14 class=omission
leaf-text: `NetWaker`; `Request` / `Body` / `FormData`;
source: ctr-ih25kzqf «contract/blitz-traits-net/request-initiator · blitz-traits net types · Request initiator — Request carries a public initiator, the URL of the document a sub-resource is loaded for, None for a top-level load, set by the consuming builder initiator.»
replace-with: `NetWaker`; `Request` (a public `initiator`, the URL of the document a sub-resource is loaded for, `None` for a top-level load, set by the consuming builder `initiator`) / `Body` / `FormData`;
