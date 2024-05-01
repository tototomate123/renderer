#[macro_use]
extern crate glium;
extern crate image;
use glium::{implement_vertex, Surface};
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

fn main() {
    let image = image::load(std::io::Cursor::new(include_bytes!("./img/Grosser_Panda.png")),
                        image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);


    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("event loop");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let shape = vec![
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0] },
        Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },
    
        Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },
        Vertex { position: [-0.5,  0.5], tex_coords: [0.0, 1.0] },
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
    #version 140

    in vec2 position;
    in vec2 tex_coords;
    out vec2 v_tex_coords;
    
    uniform mat4 matrix;
    
    void main() {
        v_tex_coords = tex_coords;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
    "#;
    
    let fragment_shader_src = r#"
    #version 140

    in vec2 v_tex_coords;
    out vec4 color;
    
    uniform sampler2D tex;
    
    void main() {
        color = texture(tex, v_tex_coords);
    }
    "#;


    let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

    let texture = glium::texture::Texture2d::new(&display, image).unwrap();
    
    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
    let _ = frame.finish();
    let mut t: f32 = 0.0;
    let _ = event_loop.run(move | event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                }
                winit::event::WindowEvent::RedrawRequested => {
                    t += 0.02;
                    let x = t.sin() * 0.5;
                
                    let uniforms = uniform! {
                        matrix: [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [ x , 0.0, 0.0, 1.0],
                        ],
                        tex: &texture,
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
            winit::event::Event::AboutToWait => window.request_redraw(),
            _ => (),
        };
    });
    
}