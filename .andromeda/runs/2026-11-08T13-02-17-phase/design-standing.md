# design · standing conventions

- S1 · Every output of an agent-read driver surface is JSON only on stdout, one line per command, no colour or ESC byte, with the exit status giving the class; each new command or answer the chunk adds follows it. · Surface: cli, Component Patterns (body and file surface)
  (design-system/surface fct-6fb4pz6c) cli-output
- S2 · A test that depends on text layout runs with blitz-dom's system-fonts feature, so font-dependent assertions do not pass vacuously; each new stand or fixture test follows it. · Typography, Loading (body)
  (design-system/typography fct-zmi56wtl) typography
- S3 · A new example or test fixture is recorded under its section's Example fixtures list. · Example fixtures entries of the body sections (file depth-strategy, file motion)
