@apply_row n=1 ref_file=fanout-design-system--cascade.md ref_n=1 v=applied edit=E1,E3
@apply_row n=2 ref_file=fanout-design-system--cascade.md ref_n=2 v=applied edit=E2
@fact after=fct-ufq6kvf4 file=surface gid=new:cli-graph kind=values subject=cli-graph as=n1
- `escher-session graph` takes no argument, and an argument is refused `malformed` with nothing run; it answers the screen's whole UI graph as one uncoloured line of JSON `{"nodes":[…],"states":[…]}` on stdout, status `0`, the same answer on the session socket's `call` and in a kept check, and over the `mcp` role its tool result is the text block of that one line; the line carries ids, owner ids, component names, registered state names, handler event names, the closed flag words `undeclared` · `unconfirmed` and booleans only, never a value or typed text (packages/escher-driver/src/schema.rs:382-427; examples/seven_guis/tests/cli_commands.rs:467-492; as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md)
@fact after=@n1 file=surface gid=new:cli-unpaged-graph kind=pending subject=cli-unpaged-graph as=n2
- The `graph` answer is unpaged, the whole screen in one line: Cells at boot answers 293,869 bytes — the stated limit «An unpaged graph» (docs/capabilities.md:252-255) · owner: residual `res-7cc5bi4k`, target next (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md)
@fact id=fct-h64spsrj expect_rev=1
old: cli-output · web-spa-tokens
new: cli-output · cli-graph · cli-unpaged-graph · web-spa-tokens
