//! Android implementation of window creation.
//! This doesn't really map quite right because you don't have the idea of windows

use super::menu::Menu;
use crate::common_util::IdleCallback;
use crate::dialog::{FileDialogOptions, FileInfo};
use crate::error::Error;
use crate::keyboard::KeyModifiers;
use crate::kurbo::{Point, Size, Vec2};
use crate::mouse::{Cursor, MouseButton, MouseEvent};
use crate::window::{Text, TimerToken, WinCtx, WinHandler};
use jni_android_sys::android::content::Context as AContext;
use jni_android_sys::android::graphics::Canvas;
use jni_android_sys::android::os::{Handler, Message};
use jni_android_sys::android::view::{KeyEvent, MotionEvent, View};
use jni_android_sys::java::lang::Runnable;
use jni_glue::{Argument, Env, Global};
use jni_sys::jobject;
use piet_common::{self, with_current_env, AndroidRenderContext, CanvasContext};
use std::any::Any;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{Arc, Mutex, Weak};

// Store Android Context for other stuff
thread_local!(static ANDROID_CONTEXT: RefCell<Option<Rc<Global<AContext>>>> = RefCell::new(None));
thread_local!(static DRUID_VIEW: RefCell<Option<Rc<Global<View>>>> = RefCell::new(None));
thread_local!(static IDLE_RUNNABLE: RefCell<Option<Global<Runnable>>> = RefCell::new(None));
thread_local!(static ANDROID_HANDLER: RefCell<Option<Global<Handler>>> = RefCell::new(None));
thread_local!(static INITIAL_SIZE: RefCell<(i32, i32)> = RefCell::new((0, 0)));

// Currently we only have one window active at a time.
thread_local!(static CURRENT_WINHANDLE: RefCell<Option<WindowHandle>> = RefCell::new(None));

pub fn setup(
    env: &Env,
    android_ctx: Global<AContext>,
    druid_view: Global<View>,
    android_handler: Global<Handler>,
) {
    piet_common::set_current_env(env);
    ANDROID_CONTEXT.with(|rc| {
        *rc.borrow_mut() = Some(Rc::new(android_ctx));
    });
    DRUID_VIEW.with(|rc| {
        *rc.borrow_mut() = Some(Rc::new(druid_view));
    });
    ANDROID_HANDLER.with(|rc| {
        *rc.borrow_mut() = Some(android_handler);
    });
}

pub(crate) fn get_druid_view() -> Rc<Global<View>> {
    DRUID_VIEW.with(|rc| rc.borrow().as_ref().expect("Druid View not set!").clone())
}

pub(crate) fn get_android_context() -> Rc<Global<AContext>> {
    ANDROID_CONTEXT.with(|rc| {
        rc.borrow()
            .as_ref()
            .expect("Android Context not set!")
            .clone()
    })
}

pub(crate) fn with_android_context<F, R>(f: F) -> R
where
    F: FnOnce(&AContext) -> R,
{
    ANDROID_CONTEXT.with(|rc| {
        with_current_env(|env| {
            f(&rc
                .borrow()
                .as_ref()
                .expect("Android Context not set!")
                .with(env))
        })
    })
}

pub(crate) fn with_android_handler<F, R>(f: F) -> R
where
    F: FnOnce(&Handler) -> R,
{
    ANDROID_HANDLER.with(|rc| {
        with_current_env(|env| {
            f(&rc
                .borrow()
                .as_ref()
                .expect("Android Handler not set!")
                .with(env))
        })
    })
}

pub(crate) fn with_current_windowhandle<F, G, R>(f: F, fallback: G) -> R
where
    F: FnOnce(&mut WindowHandle) -> R,
    G: FnOnce() -> R,
{
    CURRENT_WINHANDLE.with(|rc| match rc.borrow_mut().as_mut() {
        Some(window_handle) => f(window_handle),
        None => fallback(),
    })
}

#[derive(Clone)]
pub(crate) struct IdleHandle {
    idle_queue: Weak<Mutex<Vec<Box<dyn IdleCallback>>>>,
}

fn request_idle_callback() {
    with_current_env(|env| {
        // with_android_context(|android_ctx| {
        // let looper = android_ctx
        //     .getMainLooper()
        //     .expect("Failed to get looper")
        //     .unwrap();
        // let handler = Handler::new_Looper(env, &looper as &Looper)
        //     .expect("Handler creating failed for idle callback");
        with_android_handler(|handler| {
            let message = Message::new(env).expect("Failed to make Message");
            //  0 maps to an idle callback
            message.set_arg1(0);
            handler
                .sendMessage(Some(&*message))
                .expect("Failed to send idle request message");

            // IDLE_RUNNABLE.with(|idle_runnable_rc| {
            //     handler
            //         .post(&idle_runnable_rc.borrow().as_ref().unwrap().with(env) as &Runnable)
            //         .expect("Failed to post idle runnable");
            // })
            // })
        });
    })
}

impl IdleHandle {
    pub fn add_idle<F>(&self, callback: F)
    where
        F: FnOnce(&dyn Any) + Send + 'static,
    {
        if let Some(queue) = self.idle_queue.upgrade() {
            let mut queue = queue.lock().expect("queue lock");
            if queue.is_empty() {
                request_idle_callback();
            }
            queue.push(Box::new(callback));
        }
    }
}

pub(crate) struct WindowBuilder {
    handler: Option<Box<dyn WinHandler>>,
}

#[derive(Clone)]
pub(crate) struct WindowHandle {
    android_ctx: Rc<Global<AContext>>,
    idle_queue: Arc<Mutex<Vec<Box<dyn IdleCallback>>>>,
    druid_view: Rc<Global<View>>,
    handler: Option<Rc<RefCell<Box<dyn WinHandler>>>>,
}

impl Default for WindowHandle {
    fn default() -> WindowHandle {
        WindowHandle {
            android_ctx: get_android_context(),
            druid_view: get_druid_view(),
            idle_queue: Default::default(),
            handler: None,
        }
    }
}

impl WindowHandle {
    pub fn show(&self) {}

    pub fn close(&self) {}

    pub fn bring_to_front_and_focus(&self) {}

    /// Request invalidation of the entire window contents.
    pub fn invalidate(&self) {
        with_current_env(|env| {
            self.druid_view
                .as_ref()
                .with(env)
                .invalidate()
                .expect("Invalidate failed");
        })
    }

    pub fn set_title(&self, _title: &str) {}

    pub fn set_menu(&self, _menu: Menu) {}

    pub fn show_context_menu(&self, _menu: Menu, _pos: Point) {}

    pub fn get_idle_handle(&self) -> Option<IdleHandle> {
        Some(IdleHandle {
            idle_queue: Arc::downgrade(&self.idle_queue),
        })
    }

    /// Get the dpi of the window.
    ///
    /// TODO: we want to migrate this from dpi (with 96 as nominal) to a scale
    /// factor (with 1 as nominal).
    pub fn get_dpi(&self) -> f32 {
        with_android_context(|android_context| {
            let res = android_context
                .getResources()
                .expect("Get Resources Failed")
                .expect("Get Resources Failed");
            let display_metrics = res
                .getDisplayMetrics()
                .expect("Get Display Metrics Failed")
                .expect("Get Display Metrics Failed");
            display_metrics.density()
        })
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
    pub fn set_size(&mut self, _size: Size) {}

    /// No-op, not sure what this means
    pub fn set_title(&mut self, _title: impl Into<String>) {}

    /// No-op, we don't have menus
    pub fn set_menu(&mut self, _menu: Menu) {}

    /// Attempt to construct the platform window.
    ///
    /// If this fails, your application should exit.
    pub fn build(self) -> Result<WindowHandle, Error> {
        // TODO create druid view after this is done
        let handle = WindowHandle {
            android_ctx: get_android_context(),
            druid_view: get_druid_view(),
            idle_queue: Default::default(),
            handler: self.handler.map(|h| Rc::new(RefCell::new(h))),
        };

        {
            let mut handler = handle.handler.as_ref().unwrap().borrow_mut();
            handler.connect(&handle.clone().into());
            handler.connected(&mut WinCtxImpl::default());
            INITIAL_SIZE.with(|initial_size| {
                let (width, height) = *initial_size.borrow();
                handler.size(width as u32, height as u32, &mut WinCtxImpl::default());
            });
        }

        CURRENT_WINHANDLE.with(|rc| {
            *rc.borrow_mut() = Some(handle.clone());
        });

        // Invalidate the view so we get our onDraw handler called
        handle.invalidate();
        Ok(handle)
    }
}

struct WinCtxImpl<'a> {
    phantom: PhantomData<&'a ()>,
    text: Text<'static>,
    druid_view: Rc<Global<View>>,
}

impl Default for WinCtxImpl<'_> {
    fn default() -> Self {
        WinCtxImpl {
            phantom: Default::default(),
            text: piet_common::AndroidText,
            druid_view: get_druid_view(),
        }
    }
}

impl<'a> WinCtx<'a> for WinCtxImpl<'a> {
    /// Invalidate the entire window.
    ///
    /// TODO: finer grained invalidation.
    fn invalidate(&mut self) {
        with_current_env(|env| {
            self.druid_view
                .as_ref()
                .with(env)
                .invalidate()
                .expect("Invalidate failed");
        })
    }

    /// Get a reference to an object that can do text layout.
    fn text_factory(&mut self) -> &mut Text<'a> {
        &mut self.text
    }

    /// Set the cursor icon – noop
    fn set_cursor(&mut self, _cursor: &Cursor) {}

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
        with_android_handler(|handler| {
            with_current_env(|env| {
                let message = Message::new(env).expect("Failed to make Message");
                let token = next_timer_id();
                let now = std::time::Instant::now();
                let timeout_millis = deadline.duration_since(now).as_millis();

                message.set_arg1(token);
                message.set_arg2(timeout_millis as i32);
                handler
                    .sendMessage(Some(&*message))
                    .expect("Failed to send message");

                TimerToken::new(token as usize)
            })
        })
    }

    /// Prompt the user to chose a file to open.
    ///
    /// Blocks while the user picks the file.
    fn open_file_sync(&mut self, _options: FileDialogOptions) -> Option<FileInfo> {
        unimplemented!("TODO");
    }

    /// Prompt the user to chose a path for saving.
    ///
    /// Blocks while the user picks a file.
    fn save_as_sync(&mut self, _options: FileDialogOptions) -> Option<FileInfo> {
        unimplemented!("TODO");
    }
}

fn next_timer_id() -> i32 {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static TIMER_ID: AtomicUsize = AtomicUsize::new(1);
    TIMER_ID.fetch_add(1, Ordering::Relaxed) as i32
}

// Externs that will be called from DruidView

/// # Safety
///
/// This function should be safe. Marked unsafe since there's no gaurantee env relates to the argument jobjects, but that should always be true.
/// TODO: This may need to be called something else so it can be called by druid view's parent.
#[no_mangle]
pub unsafe extern "system" fn Java_io_marcopolo_druid_DruidView_setup(
    env: &Env,
    _this: jobject,
    android_ctx: Argument<AContext>,
    druid_view: Argument<View>,
    android_handler: Argument<Handler>,
) {
    // let jnienv = env.as_jni_env();
    // k
    // Local::from_env_object(jnienv, &mut android_ctx as &mut jobject);
    setup(
        env,
        android_ctx
            .into_global(env)
            .expect("Android Context is Null"),
        druid_view.into_global(env).expect("Druid View is Null"),
        android_handler
            .into_global(env)
            .expect("Idle Runnable is Null"),
    )
}

/// Called by Android at the end of the current work queue. No Gaurantee this is really idle. Is there a way to run on idle?
#[no_mangle]
pub extern "system" fn Java_io_marcopolo_druid_DruidView_onIdle(_env: &Env, _this: jobject) {
    with_current_windowhandle(
        |window_handle| {
            let queue: Vec<_> = std::mem::replace(
                &mut window_handle.idle_queue.lock().expect("queue"),
                Vec::new(),
            );
            let mut handler = window_handle.handler.as_ref().unwrap().borrow_mut();
            let handler_as_any = handler.as_any();

            for callback in queue {
                callback.call(handler_as_any);
            }
        },
        || {},
    );
}

/// Called by Android for a timer
#[no_mangle]
pub extern "system" fn Java_io_marcopolo_druid_DruidView_onTimer(
    _env: &Env,
    _this: jobject,
    token_id: i32,
) {
    with_current_windowhandle(
        |window_handle| {
            let token = TimerToken::new(token_id as usize);
            window_handle
                .handler
                .as_ref()
                .unwrap()
                .borrow_mut()
                .timer(token, &mut WinCtxImpl::default());
        },
        || {},
    );
}

/// Our draw call
#[no_mangle]
pub extern "system" fn Java_io_marcopolo_druid_DruidView_onDraw(
    env: &Env,
    _this: jobject,
    canvas: Argument<Canvas>,
) {
    with_current_windowhandle(
        |window_handle| {
            let canvas = unsafe { canvas.with_unchecked(env).unwrap() };
            let mut canvas_context = CanvasContext::new_from_canvas(&canvas);
            let mut android_render_context = AndroidRenderContext::new(&mut canvas_context);
            let mut handler = window_handle.handler.as_ref().unwrap().borrow_mut();

            handler.paint(&mut android_render_context, &mut WinCtxImpl::default());
        },
        || {},
    );
}

/// Called on touch events
#[no_mangle]
pub extern "system" fn Java_io_marcopolo_druid_DruidView_onTouchEvent(
    env: &Env,
    _this: jobject,
    motion_event: Argument<MotionEvent>,
) -> bool {
    with_current_windowhandle(
        |window_handle| {
            let motion_event = unsafe { motion_event.with_unchecked(env).unwrap() };
            let mut handler = window_handle.handler.as_ref().unwrap().borrow_mut();

            let pos = Point::new(
                motion_event.getX().unwrap() as f64,
                motion_event.getY().unwrap() as f64,
            );

            let mods = KeyModifiers::default();

            let button = MouseButton::Left;

            // Hack – A touch screen is like a mouse, right? /s
            match motion_event
                .getAction()
                .expect("Failed to read touch action")
            {
                MotionEvent::ACTION_DOWN => handler.mouse_down(
                    &MouseEvent {
                        pos,
                        mods,
                        button,
                        count: 1,
                    },
                    &mut WinCtxImpl::default(),
                ),
                MotionEvent::ACTION_UP => handler.mouse_up(
                    &MouseEvent {
                        pos,
                        mods,
                        button,
                        count: 0,
                    },
                    &mut WinCtxImpl::default(),
                ),
                // Ignore other actions
                _ => {}
            }
        },
        || {},
    );
    true
}

/// Called when the screen size changes. Including the first time
#[no_mangle]
pub extern "system" fn Java_io_marcopolo_druid_DruidView_onSizeChanged(
    _env: &Env,
    _this: jobject,
    width: i32,
    height: i32,
    _old_width: i32,
    _old_height: i32,
) {
    with_current_windowhandle(
        |window_handle| {
            let mut handler = window_handle
                .handler
                .as_ref()
                .expect("Handler is not set")
                .borrow_mut();
            handler.size(width as u32, height as u32, &mut WinCtxImpl::default());
        },
        || {
            INITIAL_SIZE.with(|initial_size| {
                *initial_size.borrow_mut() = (width, height);
            })
        },
    );
}

/// Called when the screen size changes. Including the first time
#[no_mangle]
pub extern "system" fn Java_io_marcopolo_druid_DruidView_onKeyPreIme(
    _env: &Env,
    _this: jobject,
    _keycode: i32,
    _key_event: Argument<KeyEvent>,
) {
    // TODO to support a hardware keyboard. We will not get keycodes from the soft keyboard.
}
