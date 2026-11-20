//! The keys a `press` names, one per name of the schema's key table.

use crate::schema::KEY_NAMES;

/// A key a `press` names: one per name of [`KEY_NAMES`], in its order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    /// `tab`
    Tab,
    /// `enter`
    Enter,
    /// `escape`
    Escape,
    /// `backspace`
    Backspace,
    /// `delete`
    Delete,
    /// `space`
    Space,
    /// `arrow-up`
    ArrowUp,
    /// `arrow-down`
    ArrowDown,
    /// `arrow-left`
    ArrowLeft,
    /// `arrow-right`
    ArrowRight,
    /// `home`
    Home,
    /// `end`
    End,
}

impl Key {
    /// Every key, in the order of [`KEY_NAMES`].
    pub const ALL: [Key; 12] = [
        Key::Tab,
        Key::Enter,
        Key::Escape,
        Key::Backspace,
        Key::Delete,
        Key::Space,
        Key::ArrowUp,
        Key::ArrowDown,
        Key::ArrowLeft,
        Key::ArrowRight,
        Key::Home,
        Key::End,
    ];

    /// The key's name in [`KEY_NAMES`].
    pub const fn name(self) -> &'static str {
        match self {
            Key::Tab => "tab",
            Key::Enter => "enter",
            Key::Escape => "escape",
            Key::Backspace => "backspace",
            Key::Delete => "delete",
            Key::Space => "space",
            Key::ArrowUp => "arrow-up",
            Key::ArrowDown => "arrow-down",
            Key::ArrowLeft => "arrow-left",
            Key::ArrowRight => "arrow-right",
            Key::Home => "home",
            Key::End => "end",
        }
    }

    /// The key `name` names exactly, or `None`.
    pub fn from_name(name: &str) -> Option<Key> {
        KEY_NAMES
            .into_iter()
            .zip(Key::ALL)
            .find_map(|(listed, key)| (listed == name).then_some(key))
    }
}
