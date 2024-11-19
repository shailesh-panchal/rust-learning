use std::io;

fn main() {
    println!("Programm for rust common programming concepts!");

    variable_constant();
    variable_shadow();
    variable_scalar();
    variable_compound();

    rust_function();
}

fn variable_constant() {
    const MINUTE_IN_HOUR: u32 = 60;

    println!("minute in hout {}", MINUTE_IN_HOUR);
}

fn variable_shadow() {
    let i_var = 10;

    let i_var = i_var + 5;
    {
        println!("variable value inner scope {}", i_var);
        //now update variable
        let i_var = i_var * 10;

        println!("variable value inner scope {} after update", i_var);
    }

    println!("variable value outer scope {}", i_var);
}

fn variable_scalar() {
    let i_u64: u64 = 64;
    let i_u128: u128 = 128;
    let i_size: isize = 256;
    let u_size: usize = 512;
    let i_u32: u32 = 32;
    let i_u8: u8 = 8;
    let i_bool: bool = true;
    let i_f32: f32 = 32.0;
    let i_f64: f64 = 64.4;
    println!("scalar variable i_32={i_u32},i_u8={i_u8},i_bool={i_bool}");
    println!("scalar variable i_128={i_u128},i_u64={i_u64},i_size={i_size},u_size={u_size}");
    println!("float value f_32={i_f32}, f_64={i_f64}");
}

fn variable_compound() {
    //two type of compund variable tuple and array

    let tuple: (i8, u8, char) = (100, 200, 's');
    //let us access the tuple variable

    let first_tuple_parameter = tuple.0;
    let (first, second, third) = tuple;

    println!("tuple first parameter value {}", first_tuple_parameter);
    println!("tuple first {first} second {second} and third {third}");

    let array = [1, 2, 3, 4, 5];

    println!("array value {}", array[0]);
}

fn rust_function() {
    //rust function syntex

    //function syntex with input paramter
    //fn funtion_name(variable: i32, variable_2 : u32)

    //function syntext with return parameter

    //note the return statemet do not have semicolon at the end of line
    //if you add the semicolon to the end of expression , you turn it into a statement and it will not a return value.

    //so in function return statement do not have semicolon at the end line

    //fn function_name() -> u32

    let return_value = function_return(10);

    println!("the value of return_value {}", return_value);
}

fn function_return(input_parameter: u32) -> u32 {
    println!("inside the function input paramter {}", input_parameter);

    //return statement
    input_parameter * 10
}
