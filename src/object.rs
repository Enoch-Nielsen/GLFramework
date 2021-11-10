// Rules
#![allow(dead_code)]
#[allow(unused_imports)]

// Use
use glium::{glutin, Surface, glutin::window::Window, glutin::dpi::PhysicalSize};

#[derive(Copy, Clone)]
pub struct Vert 
{
	pub position: [f32; 2],
}

#[derive(Clone)]
pub struct Vector2 // A Struct containing 2 values pertaining to X and Y in space.
{
	pub x : f32,
	pub y : f32,
} 
impl Vector2 
{
	pub fn new(x : f32, y : f32) -> Vector2 
	{
		return Vector2{x, y};
	}
}

#[derive(Clone)]
pub struct Vector3 // A Struct containing 3 values pertaining to X, Y, and Z in space.
{
	pub x : f32,
	pub y : f32,
	pub z : f32,
}
impl Vector3
{
	pub fn new(x : f32, y : f32, z : f32) -> Vector3
	{
		return Vector3{x, y, z};
	}
}

#[derive(Clone)]
pub struct Vector4 // A Struct containing 3 values pertaining to X, Y, and Z in space.
{
	pub x : f32,
	pub y : f32,
	pub z : f32,
	pub o : f32,
}
impl Vector4
{
	pub fn new(x : f32, y : f32, z : f32, o : f32) -> Vector4 
	{
		return Vector4{x, y, z, o}
	}
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
		return Object
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

/**
*	0 for rect, 1 for circle.	
*/	
pub fn generate_vertices(position : Vector2, size : Vector2, shape : u8, window_size : &Vector2) -> Vec<Vert>
{
	let x_conv : f32 = (2.0 / window_size.x);
	let y_conv : f32 = (-2.0 / window_size.y);

	let x_pos = (position.x * x_conv) - 1.0;
	let y_pos = (position.y * y_conv) + 1.0;

	let x_total = (position.x + size.x * x_conv) - 1.0;
	let y_total = (position.y + size.y * y_conv) + 1.0;

	let mut vertex_list : Vec<Vert> = Vec::new();

	if shape == 0 // FIX THIS, OPEN GL OBJECTS ARE CENTERED.
	{
		vertex_list.push(Vert {position : [x_pos, y_pos]}); // Vertex 1
		vertex_list.push(Vert {position : [(x_total), (y_pos)]}); // Vertex 2
		vertex_list.push(Vert {position : [(x_total), (y_total)]}); // Vertex 3
		vertex_list.push(Vert {position : [(x_pos), (y_total)]}); // Vertex 4.
	}

	return vertex_list;
}

// RenderableObject Defenition.
pub struct RenderableObject
{
	pub parent : Object,
	pub color : Vector4,
	window_size : Vector2,
	pub vertex_shader: String,
	pub fragment_shader: String,
	pub vertex_list: Vec<Vert>,
}

impl RenderableObject
{
	// Function to create a new RenderableObject.
	pub fn new(position : Vector2, size : Vector2, color : Vector4, window_size : &Vector2, shape_type : u8) -> RenderableObject
	{
		// Define base shader.
		let vertex_shader = String::from
		(
			r#"
				#version 140
		
				in vec2 position;
				uniform mat4 matrix;
		
				void main() 
				{
					gl_Position = matrix * vec4(position, 0.0, 1.0);
				}
			"#
		);

		let fragment_shader : String = String::from
		(
			r#" 
				#version 140
				out vec4 color;
				uniform float r, g, b, a;

				void main()
				{
					color = vec4(r, g, b, a);
				}
			"#
		);

		return RenderableObject
		{
			parent : Object::new(position.clone(), size.clone()),
			color,
			window_size : window_size.clone(),
			vertex_shader,
			fragment_shader,
			vertex_list : generate_vertices(position.clone(), size.clone(), shape_type, &window_size),
		};
	}

	// Getters and Setters for the objects position.
	pub fn set_position(&mut self, v: Vector2)
	{
		self.parent.position = v;
	}
	pub fn get_position(&self) -> Vector2
	{
		return self.parent.position.clone();
	}

	// Getters and setters for the objects size.
	pub fn set_size(&mut self, s: Vector2)
	{
		self.parent.size = s;		
	}
	pub fn get_size(&mut self) -> Vector2
	{
		return self.parent.size.clone();
	}
}