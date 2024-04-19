use std::{
    cell::Cell,
    collections::HashMap,
    sync::{Arc, RwLock},
};

use cgmath::Vector2;
use winit::{
    event::{DeviceEvent, ElementState, Event, WindowEvent},
    window::Window,
};

use super::{ButtonState, BypassHasher};

pub struct Mouse {
    pub cursor_position: Cell<Vector2<f32>>,
    pub mouse_movement: Cell<Vector2<f32>>,
    button_map: RwLock<HashMap<u32, ButtonState, BypassHasher>>,
}

impl Mouse {
    pub fn new() -> (Self, fn(&Mouse, &Event<'_, ()>, Arc<Window>) -> bool) {
        (
            Self {
                cursor_position: Cell::new(Vector2 { x: 0.0, y: 0.0 }),
                mouse_movement: Cell::new(Vector2 { x: 0.0, y: 0.0 }),
                button_map: RwLock::new(HashMap::with_hasher(super::BypassHasher {})),
            },
            Mouse::_event_handler,
        )
    }

    pub fn is_button_pressed(&self, button_id: u32) -> bool {
        match self.get_button_state(button_id) {
            Some(ButtonState::Pressed(_)) => true,
            _ => false,
        }
    }

    pub fn is_button_held(&self, button_id: u32) -> Option<std::time::Duration> {
        match self.get_button_state(button_id) {
            Some(ButtonState::Held(start)) => Some(std::time::Instant::now() - start),
            _ => None,
        }
    }

    pub fn get_button_state(&self, button_id: u32) -> Option<ButtonState> {
        self.button_map.read().ok()?.get(&button_id).cloned()
    }

    fn _event_handler(&self, event: &Event<'_, ()>, window: Arc<Window>) -> bool {
        match event {
            Event::DeviceEvent {
                event,
                device_id: _,
            } => {
                if let DeviceEvent::Button { button, state } = event {
                    let previous_state = self
                        .button_map
                        .read()
                        .ok()
                        .and_then(|p| p.get(button).cloned());

                    match previous_state {
                        Some(ButtonState::Pressed(_)) | Some(ButtonState::Held(_)) => {
                            if *state == ElementState::Released {
                                if let Ok(mut guard) = self.button_map.write() {
                                    guard.insert(*button, ButtonState::Released);
                                }
                            }
                        }
                        _ => {
                            if *state == ElementState::Pressed {
                                if let Ok(mut guard) = self.button_map.write() {
                                    guard.insert(
                                        *button,
                                        ButtonState::Pressed(std::time::Instant::now()),
                                    );
                                }
                            }
                        }
                    }
                }
                if let DeviceEvent::MouseMotion { delta } = event {
                    self.mouse_movement.set(
                        self.mouse_movement.get() + Vector2::new(delta.0 as f32, delta.1 as f32),
                    );
                    return true;
                }
                return false;
            }
            Event::WindowEvent { event, .. } => {
                if let WindowEvent::CursorMoved { position, .. } = event {
                    let window_size = window.inner_size();

                    self.cursor_position.set(Vector2 {
                        x: -((window_size.width / 2) as f32) + position.x as f32,
                        y: (window_size.height / 2) as f32 - position.y as f32,
                    });
                    return true;
                }

                return false;
            }
            _ => false,
        }
    }

    pub fn clear_presses(&self) {
        match self.button_map.write() {
            Ok(mut guard) => {
                guard.iter_mut().for_each(|(_, state)| {
                    if let ButtonState::Pressed(time) = *state {
                        *state = ButtonState::Held(time);
                    }
                });
            }
            Err(e) => {
                println!("Failed to access key state: {e}");
            }
        }
    }
}
