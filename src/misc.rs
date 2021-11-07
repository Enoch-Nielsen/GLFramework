extern crate rand;

use rand::thread_rng;
use rand::Rng;

// Syntax Rules
#[allow(unused_assignments)]
#[allow(unused_parens)]

#[allow(dead_code)]
pub fn random_set(s : &mut Vec<i32>, min : i32,  max : i32, count : i32, ) // Creates a random set of integers to be later used.
{
	let mut rng = thread_rng();
	let mut num: i32; 

	for _i in 0..count 
	{
		num = rng.gen_range(min..max);
		s.push(num);
	}
}

#[allow(dead_code)]
pub fn bubble_sort(s : &mut Vec<i32>) // Sorts a given vector of integers.
{
	let mut a : i32;

	for _i in 0..s.len() as i32 
	{
		for x in 0..s.len() as i32-1 
		{
			a = s[x as usize];
			
			if a > s[(x+1) as usize]
			{
				s[x as usize] = s[(x+1) as usize];
				s[(x+1) as usize] = a;					
			}		
		}
	}
}

