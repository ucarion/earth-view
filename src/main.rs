// Copyright 2014 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate byteorder;
extern crate cgmath;
extern crate genmesh;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glfw;
extern crate glfw;
extern crate rand;

use cgmath::FixedArray;
use cgmath::{Basis3, Matrix, Matrix3, Matrix4, Point3, Rotation3, Vector3};
use cgmath::{Transform, AffineMatrix3, Decomposed};
use gfx::attrib::Floater;
use gfx::traits::{Factory, Stream, ToIndexSlice, ToSlice, FactoryExt};

use std::f32::consts::PI;
use std::io::BufReader;
use std::fs::File;

mod elevation;
mod color;

use elevation::{Elevation, ElevationIterator};

// Declare the vertex format suitable for drawing.
// Notice the use of FixedPoint.
gfx_vertex!( Vertex {
    a_Pos@ pos: [f32; 3],
    a_Color@ color: [f32; 4],
});

impl Vertex {
    fn new(x: f32, y: f32, elev: &Elevation) -> Vertex {
        let z = match *elev {
            Elevation::Sea => 0.0,
            Elevation::Land { elevation: land_elev } => land_elev as f32 / 500.0
        };

        Vertex {
            pos: [x, y, z],
            color: Self::elev_to_color(elev)
        }
    }

    fn elev_to_color(elevation: &Elevation) -> [f32; 4] {
        let (r, g, b) = color::find_color(elevation);
        [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 0.0]
    }
}

// The shader_param attribute makes sure the following struct can be used to
// pass parameters to a shader.
gfx_parameters!( Params {
    u_Transform@ transform: [[f32; 4]; 4],
});


//----------------------------------------

fn xy_to_index(x: usize, y: usize, row_width: usize) -> usize {
    y * row_width + x
}

pub fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
    let (mut window, events) = glfw
        .create_window(1200, 800, "Cube example", glfw::WindowMode::Windowed)
        .unwrap();
    window.set_key_polling(true);

    let (mut stream, mut device, mut factory) = gfx_window_glfw::init(window);


//    let elevation = [
//        [rand::random(), rand::random(), rand::random(), rand::random()],
//        [rand::random(), rand::random(), rand::random(), rand::random()],
//        [rand::random(), rand::random(), rand::random(), rand::random()],
//        [rand::random(), rand::random(), rand::random(), rand::random()]
//    ];

    let width: usize = 450;
    let height: usize = 450;

    let mut vertex_data = Vec::new();
    let mut index_data: Vec<u32> = Vec::new();

    let tile_paths = vec![
        "elevation_data/derived/transformed_final_tiles/84",
        "elevation_data/derived/transformed_final_tiles/85",
        "elevation_data/derived/transformed_final_tiles/86"
    ];

    for (tile_index, tile_path) in tile_paths.iter().enumerate() {
        let file_in = File::open(tile_path).unwrap();
        let mut elevation_iter = ElevationIterator(BufReader::new(file_in));

        for y in 0..height {
            for x in 0..width {
                let elevation = elevation_iter.next().unwrap();

                // TODO: Move the data back down by 500, maybe?
                let elevation = Elevation::new(elevation.to_raw() - 500);

                let pos_x = (width * tile_index + x) as f32;
                let pos_y = (height - y) as f32;

                // println!("{} {}", pos_x, pos_y);

                vertex_data.push(Vertex::new(pos_x, pos_y, &elevation));
            }
        }
    }

    for y in 0..height {
        for x in 0..width * tile_paths.len() - 1 {

        }
    }

    for tile_index in 0..tile_paths.len() {
        for y in 0..height - 1 {
            for x in 0..width - 1 {
                let row_width = tile_paths.len() * width;

                let pos_x = width * tile_index + x;
                let pos_y = height - y;

                let a = xy_to_index(pos_x, pos_y, row_width);
                let b = xy_to_index(pos_x + 1, pos_y, row_width);
                let c = xy_to_index(pos_x, pos_y + 1, row_width);
                let d = xy_to_index(pos_x + 1, pos_y + 1, row_width);

                index_data.extend(&[c as u32, b as u32, a as u32]);
                index_data.extend(&[b as u32, c as u32, d as u32]);
            }
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

    let mut time: f32 = 0.0;

    while !stream.out.window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) =>
                    stream.out.window.set_should_close(true),
                _ => {},
            }
        }

        let camera_x = width as f32 * 1.5 + time.sin() * 100.0;
        let view: AffineMatrix3<f32> = Transform::look_at(
            &Point3::new(camera_x, 0.0, 150.0),
            &Point3::new(camera_x, height as f32 * 0.5, 0.0),
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

        time += 0.01;
    }
}
