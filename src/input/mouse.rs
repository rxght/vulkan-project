use std::{sync::{atomic::{AtomicBool, Ordering}, Mutex, RwLock}, mem::transmute, cell::Cell, collections::HashMap, marker::PhantomData};

use cgmath::{Vector2, Array};
use winit::event::{Event, DeviceEvent, ElementState, WindowEvent};

use super::{ButtonState, BypassHasher};
use super::BUTTON_HELD_THRESHOLD;

pub struct Mouse
{
    pub os_position: Cell<Vector2<f64>>,
    pub raw_delta: Cell<Vector2<f64>>,
    button_map: RwLock<HashMap<u32, ButtonState, BypassHasher>>,
}

impl Mouse
{
    pub fn new() -> (Self, fn(&Mouse, &Event<'_, ()>) -> bool)
    {
        (
            Self {
                os_position: Cell::new(Vector2 { x: 0.0, y: 0.0 }),
                raw_delta: Cell::new(Vector2 { x: 0.0, y: 0.0 }),
                button_map: RwLock::new(HashMap::with_hasher(super::BypassHasher{})),
            },
            Mouse::_event_handler
        )
    }

    pub fn is_button_pressed(&self, button_id: u32) -> bool
    {
        match self.get_button_state(button_id) {
            Some(ButtonState::Pressed(_)) => true,
            _ => false,
        }
    }

    pub fn is_button_held(&self, button_id: u32) -> Option<std::time::Duration>
    {
        match self.get_button_state(button_id) {
            Some(ButtonState::Held(start)) => Some(std::time::Instant::now() - start),
            _ => None,
        }
    }

    pub fn get_button_state(&self, button_id: u32) -> Option<ButtonState>
    {
        self.button_map.read().ok()?.get(&button_id).cloned()
    }
    
    fn _event_handler(&self, event: &Event<'_, ()>) -> bool
    {
        match event
        {
            Event::DeviceEvent{
                event,
                device_id
            } => {
                if let DeviceEvent::Button { button, state } = event {
                    let previous_state =
                        self.button_map.read().ok()
                        .and_then(|p| p.get(button).cloned());

                    match previous_state {
                        Some(ButtonState::Pressed(_)) => { 
                            if *state == ElementState::Released {
                                if let Ok(mut guard) = self.button_map.write() {
                                    guard.insert(*button, ButtonState::Released);
                                }
                            }
                        },
                        _ =>
                        {
                            if *state == ElementState::Pressed {
                                if let Ok(mut guard) = self.button_map.write() {
                                    guard.insert(*button, ButtonState::Pressed(std::time::Instant::now()));
                                }
                            }
                        }
                    }
                }
                if let DeviceEvent::MouseMotion{delta} = event {
                    self.raw_delta.set(Vector2::from(*delta));
                    return true;
                }
                return false;
            },
            Event::WindowEvent{
                event,
                ..
            } => {
                if let WindowEvent::CursorMoved { position, .. } = event {
                    self.os_position.set(Vector2{x: position.x, y: position.y});
                    return true;
                }

                return false;
            },
            _ => false,
        }
    }

    pub fn clear_presses(&self)
    {
        match self.button_map.write() {
            Ok(mut guard) => {
                guard.iter_mut().for_each(|(_, state)| {
                    if let ButtonState::Pressed(time) = *state {
                        *state = ButtonState::Held(time);
                    }
                });
            },
            Err(e) => {
                println!("Failed to access key state: {e}");
            }
        }
    }
}