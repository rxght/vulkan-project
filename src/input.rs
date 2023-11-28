use std::{
    collections::hash_map::DefaultHasher,
    hash::{BuildHasher, Hasher},
    mem::transmute,
    sync::Arc,
};

use winit::{event::Event, window::Window};

mod keyboard;
pub use keyboard::Keyboard;

mod mouse;
pub use mouse::Mouse;

#[derive(Clone, Debug)]
pub enum ButtonState {
    Pressed(std::time::Instant),
    Held(std::time::Instant),
    Released,
}

const BUTTON_HELD_THRESHOLD: std::time::Duration = std::time::Duration::from_millis(300);

pub struct Input {
    window: Arc<Window>,

    pub keyboard: Keyboard,
    keyboard_event_handler: fn(&Keyboard, &Event<'_, ()>) -> bool,

    pub mouse: Mouse,
    mouse_event_handler: fn(&Mouse, &Event<'_, ()>, Arc<Window>) -> bool,
}

impl Input {
    pub fn new(window: Arc<Window>) -> Arc<Self> {
        let (keyboard, keyboard_event_handler) = Keyboard::new();
        let (mouse, mouse_event_handler) = Mouse::new();

        Arc::new(Self {
            window: window,
            keyboard: keyboard,
            keyboard_event_handler: keyboard_event_handler,
            mouse: mouse,
            mouse_event_handler: mouse_event_handler,
        })
    }

    /// returns true if the event was handled and false if it should be passed on.
    pub fn handle_event(&self, event: &Event<'_, ()>, window: Arc<Window>) -> bool {
        (self.keyboard_event_handler)(&self.keyboard, event)
            | (self.mouse_event_handler)(&self.mouse, event, window)
    }

    /// call this at the end of each frame to make sure every key press is only counted as a press for one frame
    pub fn clear_presses(&self) {
        self.mouse.clear_presses();
        self.keyboard.clear_presses();
    }
}

/// Should only be used with HashMap where key type is T
struct BypassHasher {}

impl BuildHasher for BypassHasher {
    type Hasher = DefaultHasher;

    fn build_hasher(&self) -> Self::Hasher {
        unimplemented!()
    }

    #[inline]
    fn hash_one<T: std::hash::Hash>(&self, x: T) -> u64
    where
        Self: Sized,
        Self::Hasher: Hasher,
    {
        unsafe { *transmute::<&T, &u32>(&x) as u64 }
    }
}
