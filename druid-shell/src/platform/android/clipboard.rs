//! Interactions with the system pasteboard on Android.
//! Work here is deferred until necessary.

use crate::clipboard::{ClipboardFormat, FormatId};

#[derive(Debug, Clone, Default)]
pub struct Clipboard;

impl Clipboard {
    /// Put a string onto the system clipboard.
    pub fn put_string(&mut self, s: impl AsRef<str>) {}

    /// Put multi-format data on the system clipboard.
    pub fn put_formats(&mut self, formats: &[ClipboardFormat]) {}

    /// Get a string from the system clipboard, if one is available.
    pub fn get_string(&self) -> Option<String> {
        None
    }

    /// Given a list of supported clipboard types, returns the supported type which has
    /// highest priority on the system clipboard, or `None` if no types are supported.
    pub fn preferred_format(&self, formats: &[FormatId]) -> Option<FormatId> {
        None
    }

    /// Return data in a given format, if available.
    ///
    /// It is recommended that the `fmt` argument be a format returned by
    /// [`Clipboard::preferred_format`]
    pub fn get_format(&self, fmt: FormatId) -> Option<Vec<u8>> {
        None
    }

    pub fn available_type_names(&self) -> Vec<String> {
        vec![]
    }
}
