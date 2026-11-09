@disposition n=1
row: held fct-byb3pmvg packages/blitz/Cargo.toml:14 — refused at 2026-10-10T01-56-50-wrap, not touched since «default = ["n…»
disposition: holds — stands (line 14 is still the `default` feature list, holding `tracing`)
@disposition n=2
row: stretched fct-icom2cq7@c4114 packages/escher-driver/src/json.rs:416-420 → 417-445 — changed inside
disposition: holds (`shot_without_image` writes the shot line without `png` at 443-445; the range now also spans the inserted `impl Coverage`, which detached the doc comment at 417-418 from it)
@disposition n=3
row: stretched fct-77ml32lr examples/seven_guis/tests/common/mod.rs:371-597 → 371-624 — changed inside
disposition: holds (the six named readers stand in the range, used by host_log and mcp_log alone; the insertion is `coverage_needles`, whose assertion message names a count only)
@disposition n=4
row: stretched fct-3ogdy56e packages/escher-driver/src/cli.rs:341-368 → 375-403 — changed inside
disposition: holds (the usage line still names the mcp role after the host role and `start` with its `--record`; it now also names a coverage role after the mcp role)
