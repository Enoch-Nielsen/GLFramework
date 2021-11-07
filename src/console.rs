/**
 * Contains a set of functions that make it easer to debug or accept input.  
 */

pub fn getln() -> String // Is a shortened version of stdin, allows for much faster input.
{
	let mut line = String::new();
	let _l = std::io::stdin().read_line(&mut line).expect("Failed to Read Line");

	return line;
}

pub fn str_to_int(s : String) -> i32 // Converts the given string to an integer if the string is a valid integer.
{
	let r : i32;
	r = s.trim().parse().expect("Not an integer!");
	return r;
}

pub fn print_vec(vec : Vec<i32>, c : i32) // Add to other files later
{
	let mut temp : String = String::new();
	let mut _s : String = String::new();

	for i in 0..c as usize
	{
		if i % 25 == 0 as usize
		{
			temp += "\n";
		}

		_s = String::from(vec[i].to_string().trim());
		temp += &(format!("{:width$} | ", _s, width = 3));
	}

	print!("{}", temp);
}

pub fn print_vec_asbar(vec : Vec<i32>, c : i32) // Prints a given vector of integers to the console in the form of a bar graph, only works if the values are sorted.
{
	let mut temp : String = String::new();
	let mut _s : String = String::new();
	let max : i32 = vec[vec.len() - 1];

	for i in 0..max
	{
		temp += "\n";
		for x in 1..c+1
		{
			if (x as i32) == 1
			{
				temp += &format!("{:width$}. ", max - i, width = 5)
			}

			if vec[(x - 1) as usize] >= max - i
			{
				temp += "#";
			}else 
			{
				temp += " ";
			}				
		}
	}

	print!("{}", temp);
}

pub fn clear_console() // Clears the console.
{
	println!("\x1B[2J\x1B[1;1H");
}