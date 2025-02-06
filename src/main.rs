use glium::{implement_vertex, program::ProgramCreationInput, Surface, VertexBuffer};
use winit::dpi::*;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    window.set_max_inner_size(Some(PhysicalSize::new(20, 20)));
    window.set_min_inner_size(Some(PhysicalSize::new(20, 20)));
    window.set_resizable(false);
    window.set_title("meowmeowmeow");

    let points = (-2..=2)
        .map(|pt| Vertex {
            position: [pt as f32 / 2.0, 0.0],
        })
        .chain((-2..=2).map(|pt| Vertex {
            position: [0.0, pt as f32 / 2.0],
        }))
        .collect::<Vec<_>>();

    let vertex_buffer = VertexBuffer::new(&display, &points).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    let vertex_shader_src = include_str!("vertex.glsl");

    let fragment_shader_src = include_str!("fragment.glsl");

    // let program =
    //     glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None)
    //         .unwrap();
    // dbg!(program.uses_point_size());

    let program = glium::Program::new(
        &display,
        ProgramCreationInput::SourceCode {
            vertex_shader: vertex_shader_src,
            fragment_shader: fragment_shader_src,
            uses_point_size: true,

            geometry_shader: None,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            transform_feedback_varyings: None,
            outputs_srgb: true,
        },
    )
    .unwrap();

    let mut frame = display.draw();
    frame.clear_color_srgb(0.094,0.098,0.149, 1.0);
    frame
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
    frame.finish().unwrap();

    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            _ => (),
        };
    });
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);
