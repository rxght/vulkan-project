use std::sync::atomic::{AtomicBool, Ordering};

use cgmath::Vector2;
use winit::event::{Event, DeviceEvent, ElementState, WindowEvent};



pub struct Mouse
{
    // buttons: Vec
    os_position: Vector2<f32>,
    raw_delta: Vector2<u32>,
}

impl Mouse
{
    pub fn new() -> (Self, fn(&Mouse, &Event<'_, ()>) -> bool)
    {
        (
            Self {
                os_position: Vector2 { x: 0.0, y: 0.0 },
                raw_delta: Vector2 { x: 0, y: 0 },
            },
            Mouse::event_handler
        )
    }

    fn event_handler(&self, event: &Event<'_, ()>) -> bool
    {
        match event
        {
            Event::DeviceEvent{
                event,
                device_id
            } => {

                if let DeviceEvent::MouseMotion{delta} = event {

                    //println!("{event:#?}");
                    return true;
                }
                return false;
            },
            Event::WindowEvent{
                event,
                ..
            } => {
                if let WindowEvent::CursorMoved { position, .. } = event {

                    //println!("{event:#?}");
                    return true;
                }

                return false;
            },
            _ => false,
        }
    }
}