use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, mem::size_of};

use winit::{event_loop::ControlFlow, event::{Event, DeviceEvent, WindowEvent, ElementState}, window::Window};

mod keyboard;
pub use keyboard::Keyboard;

mod mouse;
pub use mouse::Mouse;

pub struct Input
{
    window: Arc<Window>,

    pub keyboard: Keyboard,
    keyboard_event_handler: fn(&Keyboard, &Event<'_, ()>) -> bool,

    pub mouse: Mouse,
    mouse_event_handler: fn(&Mouse, &Event<'_, ()>) -> bool,
}

impl Input
{
    pub fn new(window: Arc<Window>) -> Arc<Self>
    {
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

    pub fn handle_event(&self, event: &Event<'_, ()>) -> bool 
    {
        (self.keyboard_event_handler)(&self.keyboard, event) |
        (self.mouse_event_handler)(&self.mouse, event)
    }
}