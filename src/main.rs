#![allow(unused_assignments)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Crates
#[macro_use]
extern crate glium;

// Use
use crate::object::RenderableObject;
use crate::object::Object;
use crate::object::Vector2;
use glium::{glutin, Surface, glutin::window::Window, glutin::dpi::PhysicalSize};
use std::time::Instant;

// Mods
mod misc;
mod object;

fn main() 
{   
    // Declare Vertex.
    #[derive(Copy, Clone)]
    struct Vertex 
    {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let mut screen_size : Vector2 = Vector2 {x: 600.0, y: 600.0};
    // Declare window.
    let window_size = PhysicalSize::new(screen_size.x, screen_size.y);
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_inner_size(window_size);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    
    let _ob : RenderableObject = RenderableObject::new(Vector2 {x: 100.0, y: 100.0}, Vector2 {x: 100.0, y: 100.0}, Vector2 {x: 100.0, y: 100.0}, display.clone(), "".to_string(), "".to_string());

    // Define Vertex Buffer and indeces add shape to it.
    // let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    // let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &[0u16, 1, 2, 2, 0, 3,]).unwrap();

    // Misc Variables
    let mut t : f32 = -0.5; 
    let mut uniforms = uniform! 
    {
        t : t.abs(),
        matrix: 
        [
            [t.cos(), -t.sin(), 0.0, 0.0],
            [t.sin(), t.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ]
    };

    // Define base shader.
    let vertex_shader_src = 
    r#"
        #version 140

        in vec2 position;
        uniform mat4 matrix;

        void main() 
        {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = 
    r#" 
        #version 140
        out vec4 color;
        out vec3 pcolor;
        uniform float t;
        
        void main()
        {
            pcolor = vec3(t, t, t);
            color = vec4(pcolor, 1.0);
        }
    "#;

    // Define Program.
    //let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    
    let mut timer : Instant = Instant::now();
    let mut delta_time : f32 = 1.0;

    // Main Loop.
    event_loop.run(move |event, _, control_flow| 
    {
        //let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        //*control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);


        timer = Instant::now(); // Reset Timer
        
        t = if(t>=0.8){-0.8}else{t+(delta_time)}; // Multiply time by delta time.

        uniforms = uniform! 
        { 
            t : t.abs(), 
            matrix: 
            [
                [t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [t, 0.0, 0.0, 1.0],
            ]
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        //target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        match event 
        {
            glutin::event::Event::WindowEvent { event, .. } => match event 
            {
                glutin::event::WindowEvent::CloseRequested => 
                {
                    *control_flow = glutin::event_loop::ControlFlow::Exit; return;
                }, _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause 
            {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (), _ => return,
            }, _ => return,  
        }

        delta_time = timer.elapsed().as_secs_f32() / 5.0; // Set Delta Time
    });
}