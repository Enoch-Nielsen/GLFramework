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
	let mut screen_size : Vector2 = Vector2 {x: 600.0, y: 600.0};
	let mut screen_ratio : f32 = screen_size.x / screen_size.y;

	let window_size = PhysicalSize::new(screen_size.x, screen_size.y);
	let event_loop = glutin::event_loop::EventLoop::new();
	let wb = glutin::window::WindowBuilder::new().with_inner_size(window_size);
	let cb = glutin::ContextBuilder::new();
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();

	let mut disp = Display::new(screen_size, &display);

	// Declare Vertex.
	implement_vertex!(Vert, position);
	
	//let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
	let mut timer : Instant = Instant::now();
	let mut delta_time : f32 = 1.0;

	// Main Loop.
	event_loop.run(move |event, _, control_flow| 
	{
		// Render
		disp.render();

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