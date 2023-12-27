// Author: Jackson Y
//
// Purpose: A simple temperature converter that takes in a tuple
// of the temperature and the corresponding abbreviation, and prints 
// the converted temp.
//
// Instructions: There is no direct user interaction, to change the desired temp, change the value of 'temp'.


fn main() {
    //defines 'temp' as a tuple of the float of the temp and the corresponding abbreviation as a char.
    let temp: (f32, char) = (-12.0,'C');

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