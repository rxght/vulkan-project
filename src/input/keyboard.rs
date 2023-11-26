use std::{sync::{atomic::{Ordering, AtomicBool}, Arc, RwLock}, collections::HashMap};

use winit::event::{Event, ElementState, WindowEvent, DeviceEvent, KeyboardInput, VirtualKeyCode};

use super::ButtonState;

const KEY_COUNT: usize = 128;

pub struct Keyboard
{
    key_map: RwLock<HashMap<u32, ButtonState>>,
}

impl Keyboard
{
    pub fn is_key_pressed(&self, keycode: u32) -> bool
    {
        match self.get_key_state(keycode) {
            Some(ButtonState::Pressed(_)) => true,
            _ => false,
        }
    }

    pub fn is_key_held(&self, keycode: u32) -> Option<std::time::Duration>
    {
        match self.get_key_state(keycode) {
            Some(ButtonState::Held(start)) => Some(std::time::Instant::now() - start),
            _ => None,
        }
    }

    pub fn get_key_state(&self, keycode: u32) -> Option<ButtonState>
    {
        self.key_map.read().ok()?.get(&keycode).cloned()
    }

    pub fn new() -> (Self, fn(&Keyboard, &Event<'_, ()>) -> bool)
    {
        (
            Self {
                key_map: RwLock::new(HashMap::new()),
            },
            Keyboard::_event_handler,
        )
    }

    fn _event_handler(&self, event: &Event<'_, ()>) -> bool
    {
        match event
        {
            Event::WindowEvent{
                event,
                ..
            } => {

                if let WindowEvent::ReceivedCharacter(chr) = event {
                    return true;
                }

                if let WindowEvent::KeyboardInput { input, .. } = event {
                    match input.state {
                        ElementState::Pressed => {

                            let previous_state = match self.key_map.read() {
                                Ok(guard) => guard.get(&input.scancode).cloned(),
                                _ => None,
                            };

                            match previous_state {
                                None | Some(ButtonState::Released) => {
                                    if let Ok(mut guard) = self.key_map.write() {
                                        guard.insert(input.scancode, ButtonState::Pressed(std::time::Instant::now()));
                                    }
                                },
                                _ => {
                                    /* ignore */
                                }
                            }
                        },
                        ElementState::Released => {
                            if let Ok(mut guard) = self.key_map.write() {
                                guard.insert(input.scancode, ButtonState::Released);
                            }
                        }
                    }

                    return true;
                }

                return false;
            },
            _ => false,
        }
    }

    pub fn clear_presses(&self)
    {
        match self.key_map.write() {
            Ok(mut guard) => {
                guard.iter_mut().for_each(|(_, state)| {
                    if let ButtonState::Pressed(time) = *state {
                        *state = ButtonState::Held(time);
                    }
                });
            },
            Err(e) => {
                println!("Failed to access key s {e}");
            }
        }
    }
}