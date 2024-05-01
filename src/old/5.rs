#[macro_use]
extern crate glium;
use glium::{implement_vertex, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3]
}
implement_vertex!(Vertex, position, color);

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("event loop");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let shape = vec![
        Vertex { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0] },
        Vertex { position: [ 0.0,  0.5], color: [0.0, 1.0, 0.0] },
        Vertex { position: [ 0.5, -0.25], color: [0.0, 0.0, 1.0] }
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
    #version 140

    in vec2 position;
    in vec3 color;      // our new attribute
    out vec3 vertex_color;
    
    uniform mat4 matrix;
    
    void main() {
        vertex_color = color; // we need to set the value of each `out` variable.
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;

    let fragment_shader_src = r#"
    #version 140

    in vec3 vertex_color;
    out vec4 color;
    
    void main() {
        color = vec4(vertex_color, 1.0);   // We need an alpha value as well
    }
    "#;


    

    let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

    
    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
    frame.finish().unwrap();
    let mut t: f32 = 0.0;
    let _ = event_loop.run(move | event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, ..} => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into()); // what does the "into" do?
                }
                winit::event::WindowEvent::RedrawRequested => {
                    t += 0.02;
                    let x = t.sin() * 0.5;
                
                    let uniforms = uniform! {
                        matrix: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [ x , 0.0, 0.0, 1.0f32],
                    ]
                };
                    let vertex_buffer = glium::vertex::VertexBuffer::new(&display, &shape).unwrap();
                    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

                    let program = glium::program::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
                    let mut frame = display.draw();
                    frame.clear_color(0.0, 0.0, 1.0, 1.0);
                    frame.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
                    frame.finish().unwrap();
                }
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                _window.request_redraw();
            },
            _ => (),

            };
        });
    
}