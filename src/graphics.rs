use cgmath;
use gfx;
use glfw;
use std;
use color;
use gfx_window_glfw;

use byteorder::{LittleEndian, ReadBytesExt};
use cgmath::FixedArray;
use cgmath::{Basis3, Matrix, Matrix4, Point3, Rotation3, Vector3};
use cgmath::{Transform, AffineMatrix3, Decomposed};
use gfx::traits::{Stream, ToIndexSlice, ToSlice, FactoryExt};
use image::{self, GenericImage};

use std::io::BufReader;
use std::fs::File;

// Declare the vertex format suitable for drawing.
// Notice the use of FixedPoint.
gfx_vertex!( Vertex {
    a_Pos@ pos: [f32; 3],
    a_Color@ color: [f32; 4],
});

impl Vertex {
    fn new(pos: [f32; 3], color: [f32; 4]) -> Vertex {
        Vertex {
            pos: pos,
            color: color
        }
    }
}

// The shader_param attribute makes sure the following struct can be used to
// pass parameters to a shader.
gfx_parameters!( Params {
    u_Transform@ transform: [[f32; 4]; 4],
});


//----------------------------------------

fn xy_to_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

pub fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
    let (mut window, events) = glfw
        .create_window(1200, 800, "Cube example", glfw::WindowMode::Windowed)
        .unwrap();
    window.set_key_polling(true);

    let (mut stream, mut device, mut factory) = gfx_window_glfw::init(window);

    let width: usize = 225;
    let height: usize = 225;
    let mut file_in = BufReader::new(File::open("heightmap_chunks/83").unwrap());
    let color_data = image::open("color_chunks/83.png").unwrap();

    let default_vertex = Vertex::new([0.0; 3], [0.0; 4]);
    let mut vertex_data = vec![default_vertex; width * height];
    for y in 0..height {
        for x in 0..width {
            let elev_raw = file_in.read_u16::<LittleEndian>().unwrap();
            let elev_actual = elev_raw as f32 - 10803.0;
            let elev_scaled = elev_actual / 500.0;

            let pos_x = x as f32;
            let pos_y = height as f32 - y as f32;

            let pos = [pos_x, pos_y, elev_scaled];

            let color = color_data.get_pixel(x as u32, y as u32).data;
            let color = [color[0] as f32, color[1] as f32, color[2] as f32, color[3] as f32];
            let color = [color[0] / 255.0, color[1] / 255.0, color[2] / 255.0, color[3] / 255.0];
            vertex_data[y * width + x] = Vertex::new(pos, color);
        }
    }

    let mut index_data: Vec<u32> = Vec::new();
    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let a = xy_to_index(x, y, width) as u32;
            let b = xy_to_index(x + 1, y, width) as u32;
            let c = xy_to_index(x, y + 1, width) as u32;
            let d = xy_to_index(x + 1, y + 1, width) as u32;

            index_data.extend(&[c, b, a]);
            index_data.extend(&[b, c, d]);
        }
    }

    let mesh = factory.create_mesh(&vertex_data);

    let program = {
        let vs = gfx::ShaderSource {
            glsl_120: Some(include_bytes!("cube_120.glslv")),
            .. gfx::ShaderSource::empty()
        };
        let fs = gfx::ShaderSource {
            glsl_120: Some(include_bytes!("cube_120.glslf")),
            .. gfx::ShaderSource::empty()
        };
        factory.link_program_source(vs, fs).unwrap()
    };

    let data = Params {
        transform: Matrix4::identity().into_fixed(),
        _r: std::marker::PhantomData,
    };

    let mut batch = gfx::batch::Full::new(mesh, program, data).unwrap();
    batch.slice = index_data.to_slice(&mut factory, gfx::PrimitiveType::TriangleList);

    let mut camera_position = Point3::new(0.0, -200.0, 200.0);

    while !stream.out.window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) =>
                    stream.out.window.set_should_close(true),
                glfw::WindowEvent::Key(glfw::Key::Up, _, glfw::Action::Press, _) =>
                    camera_position.y += 10.0,
                glfw::WindowEvent::Key(glfw::Key::Down, _, glfw::Action::Press, _) =>
                    camera_position.y -= 10.0,
                glfw::WindowEvent::Key(glfw::Key::Left, _, glfw::Action::Press, _) =>
                    camera_position.x -= 10.0,
                glfw::WindowEvent::Key(glfw::Key::Right, _, glfw::Action::Press, _) =>
                    camera_position.x += 10.0,
                glfw::WindowEvent::Key(glfw::Key::LeftShift, _, glfw::Action::Press, _) =>
                    camera_position.z += 10.0,
                glfw::WindowEvent::Key(glfw::Key::LeftControl, _, glfw::Action::Press, _) =>
                    camera_position.z -= 10.0,
                _ => {},
            }
        }

        let look_at = Point3::new(camera_position.x, camera_position.y + 200.0, camera_position.z - 200.0);
        let view: AffineMatrix3<f32> = Transform::look_at(
            &camera_position,
            &look_at,
            &Vector3::unit_z(),
        );
        let proj = cgmath::perspective(cgmath::deg(45.0f32),
                                       stream.get_aspect_ratio(), 1.0, 10000.0);

        let scale = 1.0;
        let rotation: Basis3<_> = Rotation3::from_euler(
            cgmath::rad(0.0),
            cgmath::rad(0.0),
            cgmath::rad(0.0));

        let disp = Vector3::new(0.0, 0.0, 0.0);
        let model = Decomposed {
            scale: scale,
            rot: rotation,
            disp: disp
        }.into();

        let transform = proj.mul_m(&view.mat.mul_m(&model));
        batch.params.transform = transform.into_fixed();

        stream.clear(gfx::ClearData {
            color: [0.3, 0.3, 0.3, 1.0],
            depth: 1.0,
            stencil: 0,
        });
        stream.draw(&batch).unwrap();
        stream.present(&mut device);
    }
}
