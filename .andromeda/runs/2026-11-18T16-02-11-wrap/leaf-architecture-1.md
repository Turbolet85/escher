@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dire
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/stac
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/serv
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=26 class=omission
leaf-text: built by hand with `tracing::info_span!` (no `#[instrument]`).
source: dec-peqylslp «decision/app-crate-takes-own-tracing · App crates that log take tracing themselves — An app crate that logs takes tracing itself as a workspace dependency. · why: dioxus-native's prelude re-exports no tracing.»
replace-with: built by hand with `tracing::info_span!` (no `#[instrument]`); an app crate that logs takes `tracing` itself as a workspace dependency, since dioxus-native's prelude re-exports no tracing.
