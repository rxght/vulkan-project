#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use app::App;
use graphics::Graphics;
use winit::{event::{Event, WindowEvent}, event_loop::ControlFlow};

#[path ="app.rs"]
mod app;

mod graphics;

fn main()
{
    let (mut gfx, event_loop) = Graphics::new();
    let mut app = App::new(&mut gfx);

    let mut is_minimized = false;

    event_loop.run(move 
        |event, _window_target, control_flow|
    {
        match event
        {
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
                let window = gfx.get_window();
                let extent = window.inner_size();

                let min = window.is_minimized().unwrap_or(false);
                let zero_area = extent.width == 0 || extent.height == 0;

                if min || zero_area {
                    is_minimized = true;
                }
                else {
                    is_minimized = false;
                }

                if !is_minimized {
                    app.resize_callback();
                }
            },
            Event::RedrawEventsCleared => {
                app.run();
                if !is_minimized {
                    gfx.draw_frame()
                }
            }
            _ => (),
        }
    });
}