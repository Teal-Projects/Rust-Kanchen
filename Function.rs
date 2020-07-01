//////////////////////////////
// Function with parameter
// ///////////////////////////
fn main_function()
{
function_integer(5);
function_character('a');
function_string("Fitas");
}
fn function_integer(number: i32) {
    println!("The value is is: {}", number);
}
fn function_character(value: char) {
    println!("The character value is: {}", value);
}
fn function_string(name: &str) {
    println!("My name is  {} ", name );
}

//////////////////////////////
// Function with Return valu
//////////////////////////////
fn function_return() {
fn ten() -> i32

}

//////////////////////
//Diverging function
//////////////////////
fn diverge_function() -> ! {
    println!("Diverging Functions in Rust are used for crashing the current execution of a thread.");
}

////////////////////
//Function Pointer
////////////////////
fn function_pointer(i: i32) -> i32 {
    
    i + 1
}
fn main()
{
    println!("the variable f will work the same as that of the function add_one() itself. Therefore, we directly pass the value 5 and the returned result is assigned to the variable value.");
    let f: fn(i32) -> i32 = function_pointer; // Assignment without type inferencing
    let _f = function_pointer; // Assignment with type inference.
    let value = f(5);
    println!("{}", value);
}

////////////////////////////
//Function as a parameter
////////////////////////////
fn my_function(value: i32, f: &Fn(i32) -> i32) -> i32 {
    println!("{}", f(value));
    value
}
fn multiply(value: i32) -> i32 {
    value * value}
fn main() {
    println!("when the address of the function multiply() is passed as a parameter inside the function my_function() , it takes the data 5 and invokes the function f which in turn uses the function multiply() and produces the return value x*x");
    my_function(5, &multiply);
}
