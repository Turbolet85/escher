//! Tool-result writing: the call a tool call's arguments spell, how the call ended, and its
//! result's blocks, held to the answer bound.

use serde_json::{Map, Value};

use super::framing::list;
use crate::command::{ArgValue, Call};
use crate::error::SessionError;
use crate::image;
use crate::json::{self, Object};
use crate::refusal::{Fault, Refusal};
use crate::schema::VerbSpec;
use crate::wire::MAX_ANSWER_BYTES;

/// The call a tool call's arguments spell: one argument per member. A value that is neither
/// text, a whole number nor a flag refuses the call as malformed.
pub(super) fn call_of(
    verb: &'static VerbSpec,
    arguments: &Map<String, Value>,
) -> Result<Call, Refusal> {
    let mut args = Vec::with_capacity(arguments.len());
    for (name, value) in arguments {
        let value = match value {
            Value::String(text) => Some(ArgValue::Text(text.clone())),
            Value::Bool(flag) => Some(ArgValue::Flag(*flag)),
            Value::Number(number) => number.as_i64().map(ArgValue::Number),
            Value::Null | Value::Array(_) | Value::Object(_) => None,
        };
        let Some(value) = value else {
            let fault = match verb.args.iter().find(|arg| arg.name == name) {
                Some(arg) => Fault::WrongKind(arg.name),
                None => Fault::Unnamed,
            };
            return Err(Refusal::malformed(fault));
        };
        args.push((name.clone(), value));
    }
    Ok(Call {
        verb: verb.name.to_string(),
        args,
    })
}

/// How a tool call ended, as its span records it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Ending {
    Accepted,
    Refused,
    Error,
}

impl Ending {
    pub(super) const fn word(self) -> &'static str {
        match self {
            Ending::Accepted => "accepted",
            Ending::Refused => "refused",
            Ending::Error => "error",
        }
    }
}

/// A tool call's result: its text block, the image a `screenshot` returns, and how it ended.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct ToolResult {
    text: String,
    /// The PNG as base64, written as an image block before the text block.
    image: Option<String>,
    pub(super) ending: Ending,
}

impl ToolResult {
    pub(super) fn accepted(text: String) -> ToolResult {
        ToolResult {
            text,
            image: None,
            ending: Ending::Accepted,
        }
    }

    /// What an accepted `screenshot` returns: its image, then its line without the image.
    pub(super) fn shot(width: u32, height: u32, png: &[u8]) -> ToolResult {
        ToolResult {
            text: json::shot_without_image(width, height),
            image: Some(image::base64(png)),
            ending: Ending::Accepted,
        }
    }

    pub(super) fn refused(refusal: Refusal) -> ToolResult {
        ToolResult {
            text: refusal.to_json(),
            image: None,
            ending: Ending::Refused,
        }
    }

    pub(super) fn failed(error: SessionError) -> ToolResult {
        ToolResult {
            text: error.to_json(),
            image: None,
            ending: Ending::Error,
        }
    }

    /// The result, or the session's answer-size error in its place when it is over the bound:
    /// its text, or for a result that holds an image its whole written form.
    pub(super) fn bounded(self) -> ToolResult {
        let size = match self.image {
            Some(_) => self.written().len(),
            None => self.text.len(),
        };
        if size > MAX_ANSWER_BYTES {
            ToolResult::failed(SessionError::AnswerTooLarge)
        } else {
            self
        }
    }

    pub(super) fn written(&self) -> String {
        let mut blocks = Vec::with_capacity(2);
        if let Some(data) = &self.image {
            let mut block = Object::new();
            block.text("type", "image");
            block.text("data", data);
            block.text("mimeType", "image/png");
            blocks.push(block.end());
        }
        let mut block = Object::new();
        block.text("type", "text");
        block.text("text", &self.text);
        blocks.push(block.end());
        let mut result = Object::new();
        result.json("content", &list(blocks));
        result.flag("isError", self.ending != Ending::Accepted);
        result.end()
    }
}
