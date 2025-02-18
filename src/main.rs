use std::ops::Range;

use beeswarmers::{beeswarm_greedy, distances_from_centre};
use glium::{implement_vertex, program::ProgramCreationInput, uniform, Surface, VertexBuffer};
use rand::prelude::*;
use rayon::prelude::*;
use rand_distr::Uniform;
use winit::dpi::*;

const POINT_RADIUS_BEES: f64 = 1.0 / 20.0;
const OFFSET: f64 = 2.0 / 3.0;

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
    let uniform = Uniform::new(-0.2, 0.2).unwrap();

    let points = get_biggest_difference(100_000).0;

    let mut jittered_vertices = points
        .iter()
        .map(|pt| Vertex {
            position: [pt.clamp(-1.0, 1.0), uniform.sample(&mut rng)],
        })
        .collect::<Vec<_>>();
    shift_vertical(&mut jittered_vertices, -OFFSET);

    let bees = beeswarm_greedy(points.as_slice(), POINT_RADIUS_BEES);
    dbg!(distances_from_centre(&bees));
    let mut bee_vertices = bees2vertices(bees);
    shift_vertical(&mut bee_vertices, OFFSET);

    let mirrored_points = points.iter().rev().map(|p| -p).collect::<Vec<_>>();
    let bees2 = beeswarm_greedy(&mirrored_points, POINT_RADIUS_BEES);
    let bees2 = bees2
        .into_iter()
        .map(|mut bee| {
            bee[0] = -bee[0];
            bee
        })
        .collect::<Vec<_>>();
    dbg!(distances_from_centre(&bees2));
    let beevertices2 = bees2vertices(bees2);

    let bee_vertex_buffer = VertexBuffer::new(&display, &bee_vertices).unwrap();
    let bee2_vertex_buffer = VertexBuffer::new(&display, &beevertices2).unwrap();
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
        pointSize: POINT_RADIUS_BEES as f32 * min_dimension as f32 / 2.0
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
            &bee2_vertex_buffer,
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

fn generate_random(n: usize) -> Vec<f64> {
    let rng = rand::rng();
    let normal = rand_distr::Normal::new(0.0, 0.4).unwrap();
    let points = normal
        .sample_iter(rng.clone())
        .take(n)
        .collect::<Vec<f64>>();

    points
}

fn sort(points: &mut [f64]) {
    points.sort_by(|a, b| a.partial_cmp(b).unwrap());
}

fn get_biggest_difference(n: usize) -> (Vec<f64>, Vec<f64>, f64, f64) {
    fn index_to_range(i: usize) -> Range<usize> {
        i * 100..(i + 1) * 100
    }

    let random_numbers = generate_random(100 * n);
    let rayon_iter = (0..n).into_par_iter().map(|i| {
        let points = &mut random_numbers[index_to_range(i)].to_vec();
        sort(points);
        let mirrored_points = mirror_points(&points);

        let bees: Vec<[f64; 2]> = beeswarm_greedy(points, POINT_RADIUS_BEES);
        let seeb: Vec<[f64; 2]> = beeswarm_greedy(&mirrored_points, POINT_RADIUS_BEES);

        let bees = distances_from_centre(&bees);
        let seeb = distances_from_centre(&seeb);
        let max_distance = (bees.0 - seeb.0).abs();
        let rms_distance = (bees.1 - seeb.1).abs();

        (max_distance, rms_distance, i)
    });

    let max_outlier = rayon_iter
        .clone()
        .max_by(|t1, t2| t1.0.partial_cmp(&t2.0).unwrap())
        .unwrap();
    let rms_outlier = rayon_iter
        .max_by(|t1, t2| t1.1.partial_cmp(&t2.1).unwrap())
        .unwrap();

    let mut max_outlier_vec = random_numbers[index_to_range(max_outlier.2)].to_vec();
    sort(&mut max_outlier_vec);
    let mut rms_outlier_vec = random_numbers[index_to_range(rms_outlier.2)].to_vec();
    sort(&mut rms_outlier_vec);

    (
        max_outlier_vec,
        rms_outlier_vec,
        max_outlier.0,
        rms_outlier.1,
    )
}

fn mirror_points(points: &[f64]) -> Vec<f64> {
    points.iter().rev().map(|p| -p).collect()
}

fn bees2vertices(bees: Vec<[f64; 2]>) -> Vec<Vertex> {
    bees.into_iter()
        .map(|position: [f64; 2]| Vertex {
            position: [position[0], position[1]],
        })
        .collect()
}

fn shift_vertical(vertices: &mut [Vertex], offset: f64) {
    for vertex in vertices {
        vertex.position[1] += offset
    }
}

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f64; 2],
}
implement_vertex!(Vertex, position);
