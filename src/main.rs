// Author: Jackson Y
//
// Purpose: A simple temperature converter that takes in a tuple
// of the temperature and the corresponding abbreviation, and prints 
// the converted temp.
//
// Instructions: Type in a temperature (23) followed by (C or F) and the program will convert it (C to F) or (F to C).
use std::{io, str::SplitWhitespace};

fn main() {
    //string that will hold the input
    let mut user_input: String = String::new();
	
    println!("Please type a temperature and then F/C (ex: 23 F)");
	
    //getting user input
    io::stdin()
		.read_line(&mut user_input)
		.expect("Failed to read line");
	
    //This splits the input into SplitWhitespace (a vector) so it can be changed into a tuple
    let mut parts: SplitWhitespace = user_input.split_whitespace();
    
    //defines 'temp' as a tuple of the float of the temp and the corresponding abbreviation as a char.
    let temp_int: f32 = parts.next().unwrap().parse().expect("Failed to parse integer");
    let temp_char: char = parts.next().unwrap().chars().next().expect("Failed to get char");
    let temp: (f32, char) = (temp_int.clone(), temp_char.clone());

    //checks whether the abbreviation is F or C
    if temp.1 == 'F' {
        //creates result variable, calling the F to C converter function
        let result: f32 = convert_f_to_c(temp);
        println!("F to C: {}F -> {}C", temp.0, result);
    
    } else if temp.1 == 'C' {
        // creates result variable, calling the C to F converter function
        let result: f32 = convert_c_to_f(temp);
        println!("C to F: {}C -> {}F", temp.0, result);

    } else {
        //handles invalid input
        println!("Invalid input")
    }
}

fn convert_f_to_c(temp: (f32, char)) -> f32 {
    //pulls the int from the tuple, then returns the updated value
    let temp_int: f32 = temp.0;
    (temp_int - 32.0) * (5.0/9.0)
}

fn convert_c_to_f(temp: (f32, char)) -> f32 {
    //pulls the int from the tuple, then returns the updated value
    let temp_int: f32 = temp.0;
    (temp_int * 9.0/5.0) + 32.0
}