// Syntax Rules
#![allow(unused_assignments)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// Use
use crate::object::{Vector2, Vector3, Vector4, RenderableObject, Object, Vert};
use glium::{glutin, Surface, glutin::window::Window, glutin::dpi::PhysicalSize};
use std::time::Instant;

pub struct Display
{
	screen_size : Vector2,
	screen_ratio : f32,
	render_list : Vec<RenderableObject>,
	display : glium::Display,
}

impl Display
{
	pub fn new(screen_size: Vector2, display : &glium::Display) -> Display
	{
		return Display 
		{
			screen_size : screen_size.clone(),
			screen_ratio : (screen_size.x / screen_size.y) as f32,
			render_list: Vec::new(),
			display : display.clone(),
		};
	}

	pub fn render(&mut self)
	{		
		//let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
		//*control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

		let uniforms = uniform! 
		{ 
			//let : t.abs(), 
			matrix: 
			[
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 0.0, 1.0],
			]
		};

		let mut target = self.display.draw();
		target.clear_color(0.0, 0.0, 0.0, 1.0);

		for i in 0..self.render_list.len()
		{
			//target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
		}

		target.finish().unwrap();

		// Event Flow Control.

		//delta_time = timer.elapsed().as_secs_f32() / 5.0; // Set Delta Time
	}
}
