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

#[derive(Clone)]
pub struct Vector3 // A Struct containing 3 values pertaining to X, Y, and Z in space.
{
	pub x : f32,
	pub y : f32,
	pub z : f32,
}

#[derive(Clone)]
pub struct Vector4 // A Struct containing 3 values pertaining to X, Y, and Z in space.
{
	pub x : f32,
	pub y : f32,
	pub z : f32,
	pub o : f32,
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

// RenderableObject Defenition.
pub struct RenderableObject
{
	parent : Object,
	color : Vector4,
	window_size : Vector2,
	vertex_shader: String,
	fragment_shader: String,
	vertex_list: Vec<Vert>,
}

impl RenderableObject
{
	// Function to create a new RenderableObject.
	pub fn new(&mut self, position : Vector2, size : Vector2, color : Vector4, window_size : Vector2, shape_type : u8) -> RenderableObject
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
				out vec3 pcolor;
				uniform float t;
				
				void main()
				{
					pcolor = vec3(t, t, t);
					color = vec4(pcolor, 1.0);
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
			vertex_list : self.generate_shape(position.clone(), size.clone(), shape_type, &window_size),
		};
	}

	/**
	*	0 for rect, 1 for circle.	
	*/
	pub fn generate_shape(&mut self, position : Vector2, size : Vector2, shape : u8, window_size : &Vector2) -> Vec<Vert>
	{
		let x_conv : f32 = 1.0 / window_size.x;
		let y_conv : f32 = 1.0 / window_size.y;

		let mut vertex_list : Vec<Vert> = Vec::new();

		if shape == 0 // FIX THIS, OPEN GL OBJECTS ARE CENTERED.
		{
			for i in 0..3
			{
				if i == 0
				{
					vertex_list.push(Vert {position : [position.x * x_conv, position.y * y_conv]});
				}
				else
				{
					vertex_list.push(Vert {position : [(size.x * x_conv) + (position.x * x_conv), (size.y * y_conv) + (position.y * y_conv)]});
				}
			}

			//let x : f32 = 
			
		}

		return vertex_list;
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