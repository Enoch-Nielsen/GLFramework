#![allow(unused_assignments)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Crates
#[macro_use]
extern crate glium;

// Use
use crate::object::{Vector2, Vector3, Vector4, RenderableObject, Object, Vert};
use glium::{glutin, Surface, glutin::window::Window, glutin::dpi::PhysicalSize};
use std::time::Instant;
use crate::render::Display;

// Mods
mod misc;
mod object;
mod render;

fn main() 
{   
	let mut window_size : Vector2 = Vector2 {x: 1280.0, y: 720.0};
	let event_loop = glutin::event_loop::EventLoop::new();
	let mut display = Display::new(window_size.clone(), &event_loop);

	// Declare Vertex.
	implement_vertex!(Vert, position);
	
	//let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
	let mut timer : Instant = Instant::now();
	let mut delta_time : f32 = 1.0;

	let shape_color : Vector4 = Vector4::new(1.0, 1.0, 1.0, 1.0);
	let mut ob : RenderableObject = RenderableObject::new(Vector2{x: 0.0, y: 100.0}, Vector2{x: 5.0, y: 1.0}, shape_color,
		&window_size, 0);

	display.render_list.push(ob);

	// Main Loop.
	event_loop.run(move |event, _, control_flow| 
	{
		// Render
		display.render();

		// Event handling.
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
	});
}