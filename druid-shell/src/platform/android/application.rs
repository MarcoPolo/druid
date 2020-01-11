//! Android implementation of features at the application scope.

use super::clipboard::Clipboard;
pub struct Application;

impl Application {
    // If you're here app is already started!
    pub fn init() {}

    /// Terminate the application.
    pub fn quit() {
        // unimplemented!("Quit")
    }

    /// Returns a handle to the system clipboard.
    pub fn clipboard() -> Clipboard {
        Clipboard
    }

    /// Returns the current locale string.
    ///
    /// This should a [Unicode language identifier].
    ///
    /// [Unicode language identifier]: https://unicode.org/reports/tr35/#Unicode_language_identifier
    pub fn get_locale() -> String {
        unimplemented!("TODO: get_locale with Context")
    }
}
