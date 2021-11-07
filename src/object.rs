// Rules
#![allow(dead_code)]
#[allow(unused_imports)]

// Use
use glium::{glutin, Surface, glutin::window::Window, glutin::dpi::PhysicalSize};

#[derive(Clone)]
pub struct Vector2 // A Struct containing 2 values pertaining to X and Y in space.
{
	pub x : f32,
	pub y : f32,
}

#[derive(Clone)]
pub struct Vector3 // A Struct containing 3 values pertaining to X, Y, and Z in space.
{
	pub x : f32,
	pub y : f32,
	pub z : f32,
}


// Object Definition.
pub struct Object
{
	position : Vector2,
	size : Vector2,
}

impl Object
{
	pub fn new(position : Vector2, size : Vector2) -> Object
	{
		Object
		{
			position,
			size,
		}
	}

	pub fn set_position(&mut self, v: Vector2)
	{
		self.position = v;
	}

	pub fn set_size(&mut self, s: Vector2)
	{
		self.size = s;		
	}
}

pub struct RenderableObject
{
	parent : Object,
	display : glium::Display,
	vertex_shader: String,
	fragment_shader: String,	
}

impl RenderableObject
{
	// Function to create a new Object.
	pub fn new(position : Vector2, size : Vector2, screen_ratio : f32, display : glium::Display, 
			vertex_shader : String, fragment_shader : String) -> RenderableObject
	{
		return RenderableObject
		{
			parent: Object::new(position, size),
			display,
			vertex_shader,
			fragment_shader,
		}
	}

	pub fn set_position(&mut self, v: Vector2)
	{
		self.parent.position = v;
	}
	pub fn get_position(&self) -> Vector2
	{
		return self.parent.position.clone();
	}

	pub fn set_size(&mut self, s: Vector2)
	{
		self.parent.size = s;		
	}
	pub fn get_size(&mut self) -> Vector2
	{
		return self.parent.size.clone();
	}
}