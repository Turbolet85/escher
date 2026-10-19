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

fn tool_call(id: usize, name: &str, arguments: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"{name}","arguments":{arguments}}}}}"#
    )
}

/// The text block of a tool result's reply, unescaped over exactly the escapes the driver's
/// writer writes, and whether the result is marked as an error.
#[track_caller]
fn result(reply: &str) -> (bool, String) {
    let (_, rest) = reply
        .split_once(r#""content":[{"type":"text","text":""#)
        .expect("the reply is a tool result");
    let mut text = String::new();
    let mut chars = rest.chars();
    loop {
        match chars.next().expect("the text block ends") {
            '"' => break,
            '\\' => match chars.next().expect("an escape is whole") {
                '"' => text.push('"'),
                '\\' => text.push('\\'),
                'n' => text.push('\n'),
                'r' => text.push('\r'),
                't' => text.push('\t'),
                'u' => {
                    let hex: String = chars.by_ref().take(4).collect();
                    let code = u32::from_str_radix(&hex, 16).expect("four hex digits");
                    text.push(char::from_u32(code).expect("a character"));
                }
                _ => panic!("an escape the writer does not write"),
            },
            other => text.push(other),
        }
    }
    (reply.ends_with(r#""isError":true}}"#), text)
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

        let read: Vec<(bool, String)> = replies.iter().map(|reply| result(reply)).collect();
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
