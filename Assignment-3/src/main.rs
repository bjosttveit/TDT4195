extern crate nalgebra_glm as glm;
use gl::types::*;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::{mem, os::raw::c_void, ptr, str, ffi::CString};

mod shader;
mod util;
mod mesh;

use glutin::event::{
    ElementState::{Pressed, Released},
    Event, KeyboardInput,
    VirtualKeyCode::{self, *},
    WindowEvent,
};
use glutin::event_loop::ControlFlow;

const SCREEN_W: u32 = 600;
const SCREEN_H: u32 = 600;

// Helper functions to make interacting with OpenGL a little bit prettier. You will need these!
// The names should be pretty self explanatory
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

//=====TASK 1B=====
unsafe fn create_vao(vertices: &Vec<f32>, indices: &Vec<u32>, vertex_colors: &Vec<f32>, vertex_normals: &Vec<f32>) -> u32 {
    let mut array_id: u32 = 0;
    gl::GenVertexArrays(1, &mut array_id);
    gl::BindVertexArray(array_id);

    let mut vertex_buffer_id: u32 = 0;
    gl::GenBuffers(1, &mut vertex_buffer_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_id);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(vertices),
        pointer_to_array(vertices),
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
    gl::EnableVertexAttribArray(0);

    let mut index_buffer_id: u32 = 0;
    gl::GenBuffers(1, &mut index_buffer_id);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer_id);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(indices),
        pointer_to_array(indices),
        gl::STATIC_DRAW,
    );

    let mut vertex_color_buffer_id: u32 = 0;
    gl::GenBuffers(1, &mut vertex_color_buffer_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_color_buffer_id);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(vertex_colors),
        pointer_to_array(vertex_colors),
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
    gl::EnableVertexAttribArray(1);

    let mut vertex_normal_buffer_id: u32 = 0;
    gl::GenBuffers(1, &mut vertex_normal_buffer_id);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_normal_buffer_id);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(vertex_normals),
        pointer_to_array(vertex_normals),
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
    gl::EnableVertexAttribArray(2);

    return array_id;
}

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(SCREEN_W, SCREEN_H));
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Send a copy of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers. This has to be done inside of the renderin thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());
        }

        // == // Set up your VAO here
        
        let terrain = mesh::Terrain::load("./resources/lunarsurface.obj");
        let terrainVAO = unsafe { create_vao(&terrain.vertices, &terrain.indices, &terrain.colors, &terrain.normals) };
        
        let shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link()
        };

        unsafe {
            gl::UseProgram(shader.program_id);
        }


        //Camera variables
        let (mut x, mut y, mut z, mut a, mut b) = (0.0, 0.0, -2.0, 0.0, 0.0);
        let speed = 100.0;


        let first_frame_time = std::time::Instant::now();
        let mut last_frame_time = first_frame_time;
        // The main rendering loop
        loop {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = now;

            

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        VirtualKeyCode::W => {
                            z += delta_time * speed;
                        }
                        VirtualKeyCode::S => {
                            z -= delta_time * speed;
                        }

                        VirtualKeyCode::A => {
                            x += delta_time * speed;
                        }
                        VirtualKeyCode::D => {
                            x -= delta_time * speed;
                        }

                        VirtualKeyCode::Q => {
                            y += delta_time * speed;
                        }
                        VirtualKeyCode::E => {
                            y -= delta_time * speed;
                        }

                        VirtualKeyCode::Down => {
                            a += delta_time;
                        }
                        VirtualKeyCode::Up => {
                            a -= delta_time;
                        }

                        VirtualKeyCode::Right => {
                            b += delta_time;
                        }
                        VirtualKeyCode::Left => {
                            b -= delta_time;
                        }

                        _ => {}
                    }
                }
            }

            unsafe {
                //Camera transform
                let translate: glm::Mat4 = glm::mat4(
                    1.0, 0.0, 0.0, x, 
                    0.0, 1.0, 0.0, y, 
                    0.0, 0.0, 1.0, z, 
                    0.0, 0.0, 0.0, 1.0,
                );
                let rotatex: glm::Mat4 = glm::mat4(
                    1.0, 0.0, 0.0, 0.0, 
                    0.0, a.cos(), -a.sin(), 0.0, 
                    0.0, a.sin(), a.cos(), 0.0, 
                    0.0, 0.0, 0.0, 1.0,
                );
                let rotatey: glm::Mat4 = glm::mat4(
                    b.cos(), 0.0, b.sin(), 0.0, 
                    0.0, 1.0, 0.0, 0.0, 
                    -b.sin(), 0.0, b.cos(), 0.0, 
                    0.0, 0.0, 0.0, 1.0,
                );
                let perspective_transform: glm::Mat4 = glm::perspective(1.0, 1.0, 1.0, 2000.0);

                let transformation = gl::GetUniformLocation(shader.program_id, CString::new("transformation").expect("Convert to c-string").as_ptr());
                gl::UniformMatrix4fv(transformation, 1, 0, (perspective_transform * rotatex * rotatey * translate).as_ptr());


                gl::ClearColor(0.163, 0.163, 0.163, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::Clear(gl::DEPTH_BUFFER_BIT);

                // Issue the necessary commands to draw your scene here
                gl::BindVertexArray(terrainVAO);
                gl::DrawElements(gl::TRIANGLES, terrain.index_count, gl::UNSIGNED_INT, ptr::null());
                
            }

            context.swap_buffers().unwrap();
        }
    });

    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events get handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: key_state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        }
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle escape separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}
