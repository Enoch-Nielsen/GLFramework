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
		let mut target = self.display.draw();
		target.clear_color(0.0, 0.0, 0.0, 1.0);

		for i in 0..self.render_list.len()
		{
			let indices = glium::IndexBuffer::new(&self.display, glium::index::PrimitiveType::TrianglesList,
				&self.render_list[i].indice_arr).unwrap();
			let vertex_buffer = glium::VertexBuffer::new(&self.display, &self.render_list[i].vertex_list).unwrap();
			let program = glium::Program::from_source(&self.display, self.render_list[i].vertex_shader.as_str(), self.render_list[i].fragment_shader.as_str(), None).unwrap();

			let uniforms = uniform! 
			{
				matrix : 
				[
					[self.render_list[i].get_rotation_as_euler().cos(), self.render_list[i].get_rotation_as_euler().sin(), 0.0, 0.0],
					[-self.render_list[i].get_rotation_as_euler().sin(), self.render_list[i].get_rotation_as_euler().cos(), 0.0, 0.0],
					[0.0, 0.0, 1.0, 0.0],
					[0.0, 0.0, 0.0, 1.0f32],
				],	
				
				r : self.render_list[i].get_color().x, g: self.render_list[i].get_color().y, b: self.render_list[i].get_color().z, a: self.render_list[i].get_color().o,
			};
	
			target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
		}
		
		target.finish().unwrap();
	}
}
