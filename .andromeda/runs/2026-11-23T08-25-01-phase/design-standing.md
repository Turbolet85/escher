# design · standing conventions

- S1 · each new test that paints or shapes text is built under the tests' own font build and draws from the bundled fonts, never from fonts of the host · typography
  decision/test-engine-with-system-fonts · Test engine built with system fonts — The blitz-dom engine is built with the system-fonts feature for the blitz-tests crate. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
  decision/tests-built-with-system-fonts · Tests built with system fonts — The blitz-tests crate builds `blitz-dom` with the `system-fonts` feature, which is also enabled by default when testing the whole workspace. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
- S2 · each new screenshot check reads an 800 x 600 shot over opaque white, byte-identical across processes on one host, in both layout modes · surface
