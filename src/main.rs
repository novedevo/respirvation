use beeswarmers::beeswarm_greedy;
use glium::{implement_vertex, program::ProgramCreationInput, Surface, VertexBuffer, uniform};
use rand::prelude::*;
use rand_distr::Uniform;
use winit::dpi::*;

const POINT_RADIUS_BEES: f32 = 1.0 / 20.0;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    window.set_max_inner_size(Some(PhysicalSize::new(20, 20)));
    window.set_min_inner_size(Some(PhysicalSize::new(20, 20)));
    window.set_resizable(false);
    window.set_title("meowmeowmeow");

    //generate random points
    let mut rng = rand::rng();
    let normal = rand_distr::Normal::new(0.0, 0.4).unwrap();
    let uniform = Uniform::new(-0.7, -0.3).unwrap();
    let mut points = normal
        .sample_iter(rng.clone())
        .take(100)
        .collect::<Vec<f64>>();

    points.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let jittered_vertices = points
        .iter()
        .map(|pt| Vertex {
            position: [pt.clamp(-1.0, 1.0), uniform.sample(&mut rng)],
        })
        .collect::<Vec<_>>();

    let bees = beeswarm_greedy(points.as_slice(), 1.0 / 20.0);
    let bee_vertices = bees
        .into_iter()
        .map(|position: [f64; 2]| Vertex {
            position: [position[0], position[1] + 0.5],
        })
        .collect::<Vec<_>>();

    let bee_vertex_buffer = VertexBuffer::new(&display, &bee_vertices).unwrap();
    let jitter_vertex_buffer = VertexBuffer::new(&display, &jittered_vertices).unwrap();
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

    let aspect_ratio = window.inner_size().height as f32 / window.inner_size().width as f32;
    let min_dimension = window.inner_size().height.min(window.inner_size().width);
    let uniforms = uniform! {
        aspectRatio: aspect_ratio,
        pointSize: POINT_RADIUS_BEES * min_dimension as f32 / 2.0
    };

    let mut frame = display.draw();
    frame.clear_color_srgb(0.094, 0.098, 0.149, 1.0);
    frame
        .draw(
            &bee_vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();
    frame
        .draw(
            &jitter_vertex_buffer,
            &indices,
            &program,
            &uniforms,
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
    position: [f64; 2],
}
implement_vertex!(Vertex, position);
