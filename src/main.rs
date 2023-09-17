#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_imports)]

#[path ="app.rs"]
mod app;

fn main()
{
    let app = app::App::new();

    app.run();
}