//! Android implementation of window creation.
//! This doesn't really map quite right because you don't have the idea of windows

use super::menu::Menu;
use crate::common_util::IdleCallback;
use crate::dialog::{FileDialogOptions, FileInfo};
use crate::error::Error;
use crate::kurbo::{Point, Size, Vec2};
use crate::mouse::{Cursor, MouseEvent};
use crate::window::{Text, TimerToken, WinCtx, WinHandler};
use piet_common;
use std::any::Any;
use std::sync::{Arc, Mutex, Weak};

#[derive(Clone)]
pub(crate) struct IdleHandle {
    idle_queue: Weak<Mutex<Vec<Box<dyn IdleCallback>>>>,
}

impl IdleHandle {
    pub fn add_idle<F>(&self, callback: F)
    where
        F: FnOnce(&dyn Any) + Send + 'static,
    {
        unimplemented!("TODO add_idle");
    }
}

pub(crate) struct WindowBuilder {
    handler: Option<Box<dyn WinHandler>>,
}

#[derive(Debug, Clone)]
pub(crate) struct WindowHandle {}

impl Default for WindowHandle {
    fn default() -> WindowHandle {
        WindowHandle {}
    }
}

impl WindowHandle {
    pub fn show(&self) {}

    pub fn close(&self) {}

    pub fn bring_to_front_and_focus(&self) {}

    /// Request invalidation of the entire window contents.
    pub fn invalidate(&self) {
        unimplemented!("TODO")
    }

    pub fn set_title(&self, title: &str) {}

    pub fn set_menu(&self, menu: Menu) {}

    pub fn show_context_menu(&self, menu: Menu, pos: Point) {}

    pub fn get_idle_handle(&self) -> Option<IdleHandle> {
        unimplemented!("TODO")
    }

    /// Get the dpi of the window.
    ///
    /// TODO: we want to migrate this from dpi (with 96 as nominal) to a scale
    /// factor (with 1 as nominal).
    pub fn get_dpi(&self) -> f32 {
        unimplemented!("TODO DPI")
    }
}

impl WindowBuilder {
    /// Create a new `WindowBuilder`
    pub fn new() -> WindowBuilder {
        WindowBuilder { handler: None }
    }

    /// Set the [`WinHandler`]. This is the object that will receive
    /// callbacks from this window.
    ///
    /// [`WinHandler`]: trait.WinHandler.html
    pub fn set_handler(&mut self, handler: Box<dyn WinHandler>) {
        self.handler = Some(handler)
    }

    /// No-op, full size only
    pub fn set_size(&mut self, size: Size) {}

    /// No-op, not sure what this means
    pub fn set_title(&mut self, title: impl Into<String>) {}

    /// No-op, we don't have menus
    pub fn set_menu(&mut self, menu: Menu) {}

    /// Attempt to construct the platform window.
    ///
    /// If this fails, your application should exit.
    pub fn build(self) -> Result<WindowHandle, Error> {
        Ok(WindowHandle {})
    }
}

struct WinCtxImpl<'a> {
    foo: &'a (),
    text: Text<'static>,
}

impl<'a> WinCtx<'a> for WinCtxImpl<'a> {
    /// Invalidate the entire window.
    ///
    /// TODO: finer grained invalidation.
    fn invalidate(&mut self) {
        unimplemented!("TODO invalidate view")
    }

    /// Get a reference to an object that can do text layout.
    fn text_factory(&mut self) -> &mut Text<'a> {
        &mut self.text
    }

    /// Set the cursor icon – noop
    fn set_cursor(&mut self, cursor: &Cursor) {}

    /// Schedule a timer.
    ///
    /// This causes a [`WinHandler::timer()`] call at the deadline. The
    /// return value is a token that can be used to associate the request
    /// with the handler call.
    ///
    /// Note that this is not a precise timer. On Windows, the typical
    /// resolution is around 10ms. Therefore, it's best used for things
    /// like blinking a cursor or triggering tooltips, not for anything
    /// requiring precision.
    ///
    /// [`WinHandler::timer()`]: trait.WinHandler.html#tymethod.timer
    fn request_timer(&mut self, deadline: std::time::Instant) -> TimerToken {
        unimplemented!("TODO")
    }

    /// Prompt the user to chose a file to open.
    ///
    /// Blocks while the user picks the file.
    fn open_file_sync(&mut self, options: FileDialogOptions) -> Option<FileInfo> {
        unimplemented!("TODO");
    }

    /// Prompt the user to chose a path for saving.
    ///
    /// Blocks while the user picks a file.
    fn save_as_sync(&mut self, options: FileDialogOptions) -> Option<FileInfo> {
        unimplemented!("TODO");
    }
}
