#![allow(dead_code)]

use std::sync::Arc;

use app::App;
use graphics::Graphics;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

#[path = "app.rs"]
mod app;
mod graphics;
mod input;

fn main() {
    // initialize subsystems
    let (mut gfx, event_loop) = Graphics::new();
    let input = input::Input::new(gfx.get_window());

    // initialize app and pass it a reference to each subsystem
    let app = App::new(&mut gfx, input.clone());

    let mut minimized = false;

    event_loop.run(move |event, _window_target, control_flow| {
        let event_handled = input.handle_event(&event, gfx.get_window());

        if event_handled {
            return;
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                minimized = is_minimized(gfx.get_window());

                if !minimized {
                    app.resize_callback(&mut gfx);
                }
            },
            Event::WindowEvent {
                event: WindowEvent::Focused(false),
                ..
            } => {
                minimized = is_minimized(gfx.get_window());
            },
            Event::RedrawEventsCleared => {
                app.run(&gfx);
                if !minimized {
                    gfx.draw_frame()
                }
                input.clear_presses();
            }
            _ => (),
        }
    });
}

fn is_minimized(window: Arc<Window>) -> bool {
    let extent = window.inner_size();

    let min = window.is_minimized().unwrap_or(false);
    let zero_area = extent.width == 0 || extent.height == 0;

    min || zero_area
}
