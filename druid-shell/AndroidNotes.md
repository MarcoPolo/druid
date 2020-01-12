# Notes for work on Android

## Open Questions

* How do I popup the soft keyboard?

## Mappings

`IdleCallback` becomes `handler.post`
`invalidate` becomes `view.invalidate`


## Difficulty rating (1-5)
~`error` 1~
`~runloop~` 2?
`~menu~` 1? // Defer this (applies weirdly to Android)
`~clipboard~` 4 // can defer this work
`~keycodes~` 4 // Use key preime – Defer
`~application~` 3
`window` 4

## Setup process

`build` calls something on the android side to insert the DruidView and then calls setup to set our environment properly.

// OR

We can defer onDraw/onFoo calls until we have been built, then invalidate the view and start over. Could be easier. We'd have to keep track of the initial onSizeChanged call though. Not too bad.

## WinHandler

Druid view needs to implement handler. Handle messages that maps to the timers.

extern fns in rust:
Setup
Call idle
Call Timer
draw
touch event
key event
size changed event

Android side:
call setup
handle idle & timer message
hookup draw
hookup touch event
hookup size changed event

// Defer
hookup keyevent

```rs

    /// Provide the handler with a handle to the window so that it can
    /// invalidate or make other requests.
    ///
    /// This method passes the `WindowHandle` directly, because the handler may
    /// wish to stash it.
    fn connect(&mut self, handle: &WindowHandle);

    /// Called immediately after `connect`.
    ///
    /// The handler can implement this method to perform initial setup.
    #[allow(unused_variables)]
    fn connected(&mut self, ctx: &mut dyn WinCtx) {}

        /// Called on a key down event.
    ///
    /// Return `true` if the event is handled.
    #[allow(unused_variables)]
    fn key_down(&mut self, event: KeyEvent, ctx: &mut dyn WinCtx) -> bool {
        false
    }

    /// Called when a key is released. This corresponds to the WM_KEYUP message
    /// on Windows, or keyUp(withEvent:) on macOS.
    #[allow(unused_variables)]
    fn key_up(&mut self, event: KeyEvent, ctx: &mut dyn WinCtx) {}

    /// Called when this window becomes the focused window.
    #[allow(unused_variables)]
    fn got_focus(&mut self, ctx: &mut dyn WinCtx) {}

    /// Called when the window is being destroyed. Note that this happens
    /// earlier in the sequence than drop (at WM_DESTROY, while the latter is
    /// WM_NCDESTROY).
    #[allow(unused_variables)]
    fn destroy(&mut self, ctx: &mut dyn WinCtx) {}

```