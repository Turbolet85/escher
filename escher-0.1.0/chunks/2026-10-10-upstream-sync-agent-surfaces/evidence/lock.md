# Lock — the merged `Cargo.lock`

Read 2026-10-10, 01:21Z–01:24Z, on the resolved, uncommitted merge.

## Resolution
- `cargo metadata --locked --format-version 1` → exit 0 on the resolved manifests. Cargo did not rewrite the lock: `git status --short Cargo.lock` reads `M ` (staged by the merge, no work-tree change on top). Step 10's re-resolve was not needed and `cargo update` was not run.
- Conflict markers in the merged lock: 0.

## Package set (`name` + `version` + `source`, parsed with `tomllib` from each blob)
| lock | names | entries |
|---|---|---|
| merge base `23354585` | 888 | 990 |
| chunk start `09f479b8` | 890 | 992 |
| upstream pin `7832c177` | 892 | 990 |
| merged tree | 894 | 992 |

- The merged set equals upstream's set plus every entry of the names ours added since the base, exactly (difference empty both ways). Those names are two — `escher-driver` and `escher-telemetry` — so 990 + 2 = 992. Research states "plus the 3 entries ours added": the set equality it reports holds, the count measured here is 2.
- Names entering against the chunk start, 6: `core_detect`, `fearless_simd_macros`, `icu_casemap`, `icu_casemap_data`, `memchr-n`, `multiversion_no_op`. Names leaving, 2: `jetscii` 0.5.3, `tinyvec_macros` 0.1.1.
- One version of a multi-version name leaves: `base64` 0.22.1 (0.21.7 and 0.23.1 stay), `phf` · `phf_codegen` · `phf_generator` 0.13.1 (0.14.0 stays), `phf_shared` 0.13.1 (0.11.3 and 0.14.0 stay). One enters: `fearless_simd` 1.1.0 beside 0.7.0.

## The coupled pins, as merged
| name | chunk start | merged |
|---|---|---|
| `taffy` | 0.14.0, git `4142c9d8` | 0.14.0, git `7d33901c` |
| `parley` · `fontique` · `parley_data` · `parley_emoji` · `parley_engine` | 0.11.0, git `e41dfea5` | 0.12.0, registry |
| `parlance` | 0.1.0, git `e41dfea5` | 0.1.1, registry |
| `skrifa` | 0.44.0 | 0.44.0 — one version |
| `vello` · `vello_cpu` · `harfrust` | 0.11.0 · 0.3.0 · 0.12.0 | unmoved |
| `winit` | 0.31.0-beta.3 | unmoved |
| `stylo` | 0.22.0 | unmoved |
| `accesskit` | 0.25.0 | 0.25.1 |
| `accesskit_unix` · `accesskit_android` | 0.23.0 · 0.8.0 | 0.24.0 · 0.9.0 |
| `accesskit_consumer` · `accesskit_macos` · `accesskit_windows` · `accesskit_atspi_common` | 0.39.0 · 0.27.0 · 0.35.0 · 0.20.0 | 0.39.1 · 0.27.1 · 0.35.1 · 0.21.0 |
| `comrak` | 0.55.0 | 0.56.0 |
| `libc` | 0.2.189 | 0.2.189 — unmoved (upstream's pin, #1162, is the version the lock already held) |
| `ttf-parser` | 0.25.1 | 0.25.1 — the ignored advisory is still reached |
| `rustls` | 0.23.45 | 0.23.45 — the recorded fix is kept |
| `tracing` · `tracing-subscriber` · `tracing-log` | 0.1.44 · 0.3.23 · 0.2.0 | unmoved |

In all, 101 names read a different version set at the chunk start and in the merged lock: the moved rows above, the six multi-version names, the workspace's own `accesskit_xplat` (0.2.0 → 0.2.1), and registry names that move by a patch or minor version — `tokio`, `hyper`, `syn`, `zerocopy`, the `wasm-bindgen` family, the `toml` family among them. Each is upstream's version: the set equality above leaves no version of ours beside one of upstream's.

## Hand review of the six entering names
Licence, repository, build script and proc-macro flag are read from each crate's own manifest in the registry source cargo downloaded; "pulled in by" is the direct dependent in the merged lock. All six are crates.io registry packages with a checksum in the lock.

| crate | version | licence | repository | pulled in by | notes |
|---|---|---|---|---|---|
| `core_detect` | 1.0.0 | MIT/Apache-2.0 | thomcc/core_detect | `encoding_rs` 0.8.42 (itself under `reqwest` and `stylo`) | no build script; its manifest: "A `no_std` version of the `std::is_x86_feature_detected!` macro" |
| `multiversion_no_op` | 1.0.0 | Apache-2.0 OR MIT | hsivonen/multiversion_no_op | `encoding_rs` 0.8.42 | proc-macro, no build script; its manifest: "a pass-through function attribute called multiversion for optimizing build times" |
| `icu_casemap` | 2.3.0 | Unicode-3.0 | unicode-org/icu4x | `blitz-dom`, optional under its `text-transform-icu` feature (upstream #947, renamed in #1093) | no build script; `icu_properties` and `icu_segmenter` of the same family were in the graph at the chunk start |
| `icu_casemap_data` | 2.3.0 | Unicode-3.0 | unicode-org/icu4x | `icu_casemap` 2.3.0 | carries a build script |
| `memchr-n` | 0.1.9 | MIT OR Apache-2.0 | Dr-Emann/memchr_n | `comrak` 0.56.0, whose one dependent is `rdme` | no build script; `jetscii`, which leaves, had `comrak` 0.55.0 as its one dependent |
| `fearless_simd_macros` | 0.1.0 | Apache-2.0 OR MIT | linebender/fearless_simd | `memchr-n` 0.1.9 | proc-macro, no build script; `memchr-n` also brings `fearless_simd` 1.1.0 beside the 0.7.0 `vello_common` uses |

Every licence above is inside upstream's `[licenses]` allow list. Not read here: each crate's source beyond its manifest. The advisory reading over the merged lock is the `audit` leg's (the gate block); the licence reading over the whole graph is `licenses.md`.
