//! A password typed through the driver's MCP server reads the mask and never the typed text. The
//! server runs in process over an in-memory request buffer, booting a password fixture beside
//! the stand with the stand's options, in both layout modes: `start`, a `type` of a synthetic
//! sentinel into the password input, then a `snapshot`. Neither tool result's text holds the
//! sentinel: the `type` result's changed node reads the mask as its value and names what
//! differs, and the snapshot's line for the input reads the mask, role `PasswordInput` and a
//! non-empty name. A failure message carries a layout mode and a call index only.

use std::time::Duration;

use dioxus_native_dom::MASKED_VALUE;
use escher_driver::{Session, SessionError, serve_mcp};

mod fixtures;
mod session_common;
use session_common::{mcp_tool_result, tool_call};

/// What the check types into the password input: synthetic, and no part of any id or name.
const SENTINEL: &str = "Mcp-Typed-Sentinel-4kPw";

/// The name the server boots the fixture by.
const APP: &str = "masked";

fn boot(incremental: bool) -> impl FnMut(&str) -> Result<Session, SessionError> {
    move |app| {
        Session::start(app, || {
            fixtures::boot(fixtures::password_named, incremental)
        })
    }
}

#[test]
fn a_password_typed_over_mcp_reads_the_mask_and_never_the_typed_text() {
    assert!(!SENTINEL.contains(MASKED_VALUE) && !MASKED_VALUE.is_empty());
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };
        let requests = [
            tool_call(1, "start", &format!(r#"{{"app":"{APP}"}}"#)),
            tool_call(
                2,
                "type",
                &format!(r#"{{"id":"fx-secret","text":"{SENTINEL}"}}"#),
            ),
            tool_call(3, "snapshot", "{}"),
        ];
        let mut input = requests.join("\n");
        input.push('\n');
        let mut output = Vec::new();
        serve_mcp(
            input.as_bytes(),
            &mut output,
            &[APP],
            boot(incremental),
            Duration::from_secs(1800),
        )
        .expect("the exchange ends");
        let output = String::from_utf8(output).expect("replies are text");
        let replies: Vec<&str> = output.lines().collect();
        assert!(replies.len() == 3, "{mode}: one reply per call");
        assert!(
            output.matches(SENTINEL).count() == 0,
            "{mode}: the replies hold the typed text {} times",
            output.matches(SENTINEL).count()
        );

        let read: Vec<(bool, String)> =
            replies.iter().map(|reply| mcp_tool_result(reply)).collect();
        for (index, (is_error, text)) in read.iter().enumerate() {
            assert!(!is_error, "{mode}: call {} ran", index + 1);
            assert!(
                text.matches(SENTINEL).count() == 0,
                "{mode}: call {}'s text holds the typed text",
                index + 1
            );
        }

        let typed = &read[1].1;
        let (_, changed) = typed
            .split_once(r#""changed":["#)
            .expect("the type's result names what it changed");
        let (_, node) = changed
            .split_once(r#"{"id":"fx-secret","#)
            .unwrap_or_else(|| panic!("{mode}: call 2 names the password input changed"));
        let node = node.split_once('}').map_or(node, |(node, _)| node);
        assert!(
            node.contains(&format!(r#""value":"{MASKED_VALUE}""#)),
            "{mode}: call 2's changed node reads the mask as its value"
        );
        assert!(
            node.contains(r#""differs":["#) && node.contains(r#""value""#),
            "{mode}: call 2's changed node names its value as what differs"
        );

        let screen = &read[2].1;
        let line = screen
            .lines()
            .find(|line| line.contains(r#" id="fx-secret" "#))
            .unwrap_or_else(|| panic!("{mode}: call 3 lists the password input"));
        let line = line.trim_start();
        assert!(
            line.starts_with("PasswordInput \"") && !line.starts_with("PasswordInput \"\""),
            "{mode}: call 3 reads the input's role and a non-empty name"
        );
        assert!(
            line.contains(&format!(" value=\"{MASKED_VALUE}\" ")),
            "{mode}: call 3 reads the mask as the input's value"
        );
    }
}
