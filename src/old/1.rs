use glium::Surface;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("event loop");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let mut frame = display.draw();

    frame.clear_color(0.5, 0.5, 0.5, 1.0);
    frame.finish().unwrap();
    
    let _ = event_loop.run(move | event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, ..} => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            _ => (),

            }
        });
    
}