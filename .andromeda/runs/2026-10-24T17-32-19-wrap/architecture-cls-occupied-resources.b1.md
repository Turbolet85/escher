# Classified — architecture · occupied-resources.b1

## Answers
occupied-resources#1.1 · no · the session adds no listener, port or socket: host_log's new runs reuse the one session socket site, and the task keeps listeners and ports unchanged
occupied-resources#2.1 · no · the session adds no outbound host, and the task keeps outbound hosts unchanged
occupied-resources#3.1 · yes · add-to · host_log's two new state directories hl-client and hl-client-rec join the session checks' list under target/tmp; the cited ci-leg.sh:88-89 moves to 93-94, and host_log.rs:156 may move too
occupied-resources#4.1 · no · the recording is unchanged, and host_log's new start --record runs use the same launcher-named directory rules
occupied-resources#4.2 · no · the two committed Counter checks are not rewritten, and no reached file records or reads them
occupied-resources#5.1 · yes · add-to · host_log now also runs start --record clients and drains each client's own stderr at trace, which this inventory does not carry; its cited host_log.rs and stand_act_spans.rs lines may have moved
occupied-resources#6.1 · no · common/mod.rs is unchanged and host_log stays one of the two log checks reading stderr with these helpers, so the new client reads add no reader
occupied-resources#6.2 · no · no engine source changed, so the stylo.rs lock and its headless-document rule stand
occupied-resources#6.3 · no · the session changes no dependency, so the upstream-only allocators stand
occupied-resources#7.1 · no · the session adds no environment read; RUST_LOG on the client command is the registered read, and the task keeps environment variables unchanged
occupied-resources#8.1 · no · the session adds no crate, binary, feature or name, and the task keeps dependencies and the crate list unchanged
occupied-resources#9.1 · no · the task keeps the CI jobs, caches and permissions unchanged, the stand-repeat job runs the edited leg unchanged, and this row is re-read unmoved
