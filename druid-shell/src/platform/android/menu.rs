//! Android implementation of menus.

// Not doing anything with Menus for now
pub struct Menu;
use crate::hotkey::HotKey;

impl Menu {
    pub fn new() -> Menu {
        Menu
    }

    /// Create a new empty context menu.
    ///
    /// Some platforms distinguish between these types of menus, and some
    /// do not.
    pub fn new_for_popup() -> Menu {
        Menu
    }

    /// Add the provided `Menu` as a submenu of self, with the provided title.
    pub fn add_dropdown(&mut self, menu: Menu, text: &str, enabled: bool) {}

    /// Add an item to this menu.
    ///
    /// The `id` should uniquely identify this item. If the user selects this
    /// item, the responsible [`WindowHandler`]'s [`command()`] method will
    /// be called with this `id`. If the `enabled` argument is false, the menu
    /// item will be grayed out; the hotkey will also be disabled.
    /// If the `selected` argument is `true`, the menu will have a checkmark
    /// or platform appropriate equivalent indicating that it is currently selected.
    /// The `key` argument is an optional [`HotKey`] that will be registered
    /// with the system.
    ///
    ///
    /// [`WindowHandler`]: trait.WindowHandler.html
    /// [`command()`]: trait.WindowHandler.html#tymethod.command
    /// [`HotKey`]: struct.HotKey.html
    pub fn add_item(
        &mut self,
        id: u32,
        text: &str,
        key: Option<&HotKey>,
        enabled: bool,
        selected: bool,
    ) {
    }

    /// Add a seperator to the menu.
    pub fn add_separator(&mut self) {}
}
