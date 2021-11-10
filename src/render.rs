// Syntax Rules
#![allow(unused_assignments)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// Use
use crate::object::{Vector2, Vector3, Vector4, RenderableObject, Object, Vert};
use glium::{glutin, Surface, glutin::window::Window, glutin::dpi::PhysicalSize};
use std::time::Instant;

pub fn generate_display(size : Vector2, event_loop : &glutin::event_loop::EventLoop<()>) -> glium::Display
{
	let window_size = PhysicalSize::new(size.x, size.y);
	let wb = glutin::window::WindowBuilder::new().with_inner_size(window_size);
	let cb = glutin::ContextBuilder::new();
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();

	return display;
}

pub struct Display
{
	pub screen_size : Vector2,
	pub render_list : Vec<RenderableObject>,
	display : glium::Display,
}

impl Display
{
	pub fn new(screen_size: Vector2, event_loop : &glutin::event_loop::EventLoop<()>) -> Display
	{
		return Display 
		{
			screen_size : screen_size.clone(),
			render_list: Vec::new(),
			display : generate_display(screen_size.clone(), &event_loop),
		};
	}

	pub fn render(&mut self)
	{		
		
		let indices = glium::IndexBuffer::new(&self.display, glium::index::PrimitiveType::TrianglesList,
			&[0u16, 1, 2, 2, 0, 3,]).unwrap();

		let mut target = self.display.draw();
		target.clear_color(0.0, 0.0, 0.0, 1.0);

		for i in 0..self.render_list.len()
		{
			//println!("{} | {}", self.render_list[i].vertex_list[0].position[0], self.render_list[i].vertex_list[0].position[1]);
			let vertex_buffer = glium::VertexBuffer::new(&self.display, &self.render_list[i].vertex_list).unwrap();
			
			let program = glium::Program::from_source(&self.display, self.render_list[i].vertex_shader.as_str(), self.render_list[i].fragment_shader.as_str(), None).unwrap();
			target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
		}
		
		target.finish().unwrap();
	}
}
