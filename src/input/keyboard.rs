use std::sync::{atomic::{Ordering, AtomicBool}, Arc};

use winit::event::{Event, ElementState, WindowEvent, DeviceEvent, KeyboardInput, VirtualKeyCode};

const KEY_COUNT: usize = 128;

pub struct Keyboard
{
    key_states: [AtomicBool; KEY_COUNT],
    pub raw_input: AtomicBool,
}

impl Keyboard
{
    #[inline]
    fn set_key_state(&self, keycode: u32, value: bool)
    {
        self.key_states[keycode as usize].store(value, Ordering::Relaxed);
    }

    #[inline]
    pub fn is_key_pressed(&self, keycode: u32) -> bool
    {
       self.key_states[keycode as usize].load(Ordering::Acquire)
    }

    pub fn new() -> (Self, fn(&Keyboard, &Event<'_, ()>) -> bool)
    {
        (
            Self {
                key_states: [false; KEY_COUNT].map(|p| AtomicBool::new(p)),
                raw_input: AtomicBool::new(false),
            },
            Keyboard::handle_event,
        )
    }

    fn handle_event(&self, event: &Event<'_, ()>) -> bool
    {
        match event
        {
            Event::DeviceEvent{
                event,
                device_id
            } => {

                match event
                {
                    DeviceEvent::Key(input) => {

                        if !self.raw_input.load(Ordering::Acquire) {
                            return true;
                        }

                        self.set_key_state(input.scancode, input.state == ElementState::Pressed);
                        return true;
                    },
                    DeviceEvent::Text{..} => {
                        return true;
                    }  
                    _ => return false,
                }
            },
            Event::WindowEvent{
                event,
                ..
            } => {

                if let WindowEvent::ReceivedCharacter(chr) = event {
                    return true;
                }

                if let WindowEvent::KeyboardInput{
                    input: KeyboardInput{ 
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Return),
                        ..
                    },
                    ..
                } = event {
                    match self.raw_input.load(Ordering::Acquire)
                    {
                        true => {
                            println!("Switching to non-raw input.");
                            self.raw_input.store(false, Ordering::Relaxed);
                        },
                        false => {
                            println!("Switching to raw input.");
                            self.raw_input.store(true, Ordering::Relaxed);
                        }
                    }
                }

                if let WindowEvent::KeyboardInput { input, .. } = event {

                    if self.raw_input.load(Ordering::Acquire) {
                        return true;
                    }

                    self.set_key_state(input.scancode, input.state == ElementState::Pressed);
                    return true;
                }

                return false;
            },
            _ => false,
        }
    }
}