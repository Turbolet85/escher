# layouts · standing conventions

- S1 · each new stand check boots through the shared stand boot and its held instance, never a boot of its own · surface-desktop-native
- S2 · each new driver argument is bounded and validated before anything runs, its refusals closed names with their exit codes, and nothing the caller typed written back · surface-cli
  contract/escher-session-snapshot/part · escher-session snapshot · --part — Optional whole number 1 to 1000, part 1 without it; outside the bound refused malformed, exit 1.
