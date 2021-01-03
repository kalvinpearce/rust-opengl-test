extern crate gl;
extern crate sdl2;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate render_gl_derive;
extern crate vec_2_10_10_10;

mod debug;
pub mod render_gl;
pub mod resources;

use failure::err_msg;
use render_gl::buffer;
use render_gl::data;
use resources::Resources;
use std::path::Path;

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
}

fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}

fn run() -> Result<(), failure::Error> {
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    let window = video_subsystem
        .window("OpenGL Test", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().map_err(err_msg)?;
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    unsafe {
        gl.Viewport(0, 0, 900, 700);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let shader_program = render_gl::Program::from_res(&gl, &res, "shaders/triangle").unwrap();

    let vertices: Vec<Vertex> = vec![
        Vertex {
            pos: (0.5, -0.5, 0.0).into(),
            clr: (1.0, 0.0, 0.0, 1.0).into(),
        },
        Vertex {
            pos: (-0.5, -0.5, 0.0).into(),
            clr: (0.0, 1.0, 0.0, 1.0).into(),
        },
        Vertex {
            pos: (0.0, 0.5, 0.0).into(),
            clr: (0.0, 0.0, 1.0, 1.0).into(),
        },
    ];

    let vbo = buffer::ArrayBuffer::new(&gl);
    vbo.bind();
    vbo.static_draw_data(&vertices);
    vbo.unbind();

    let vao = buffer::VertexArray::new(&gl);
    vao.bind();
    vbo.bind();
    Vertex::vertex_attrib_pointers(&gl);
    vbo.unbind();
    vao.unbind();

    let mut event_pump = sdl.event_pump().map_err(err_msg)?;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();
        vao.bind();
        unsafe {
            gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window();
    }

    Ok(())
}
