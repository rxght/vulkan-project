use app::App;
use winit::{event::{Event, WindowEvent}, event_loop::ControlFlow};

#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_imports)]

#[path ="app.rs"]
mod app;

fn main()
{
    let (mut app, event_loop) = App::new();
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
                app.resize_callback();
            },
            Event::RedrawEventsCleared => {
                app.run();
            }
            _ => (),
        }
    });
}