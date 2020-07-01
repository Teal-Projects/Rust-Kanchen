/////////////////////////
// Primitive Data Type
/////////////////////////
fn primitive_data_type(){
    println!("Primitive Data types are the data type which is built into a programming language itself and can be used to derive different data type.");

    println!("bool	The boolean type.");
    println!("char	A character type.");
    println!("i8	The 8-bit signed integer type.");
    println!("i16	The 32-bit signed integer type.");
    println!("i32	The 64-bit signed integer type.");
    println!("i64	The pointer-sized signed integer type.");
    println!("isize	The pointer-sized signed integer type.");
    println!("u8	The 8-bit unsigned integer type.");
    println!("u16	The 16-bit unsigned integer type.");
    println!("u32	The 32-bit unsigned integer type.");
    println!("u64	The 64-bit unsigned integer type.");
    println!("usize	The pointer-sized unsigned integer type.");
    println!("f32	The 32-bit floating point type.");
    println!("f64	The 64-bit floating point type.");
    println!("array	A fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.");
    println!("slice	A dynamically-sized view into a contiguous sequence, [T].");
    println!("str	String slices.");
    println!("tuple	A finite heterogeneous sequence, (T, U, ..).");
}

/////////////
// Boolean
// //////////
fn Bolean() {
    println!("A standard boolean data type. It can be either true or false.");

    let b = true ;
    let b: bool = true;
}

///////////
// Chars
///////////
fn Char() {
    println!("It is a 4-byte character.");
    let a = 'a';
    let b = 'b';
    let keyboard = '⌨';
}

/////////////
// Integer
/////////////
fn Integer() {
    println!("The integer data types include i8, i16, i32, i64, isize, u8, u16, u32, u64 , usize. Here, i means signed and u means unsigned. isize and usize depend on the word size of the machine, that is on the architecture of the machine.");
    let a = 5;
    let count = 42;
    let mobile = 8675309;
}

////////////////////
// Floating-Point
// /////////////////
fn Floating_Point() {
    println!("It includes f32 and f 64.");
    let x = 2.741; //f64
    let y: f32 = 3.0; // f32

}

/////////////////// Compound Types

///////////
// Array
///////////
fn Array() {
    println!("An array is a fixed sized collection of same type elements.");

    let array: [i32; 5] = [0, 1, 2, 3, 4];

    println!("The first element of the array is: {}", array[0]);
    let mut counter = 0;
    for x in array.iter(){
        println!("The element at index {} is {}", counter, x);
        counter += 1;
    } 
}

///////////
// Slice
///////////
fn Slice() {
    println!("A slice is a dynamically sized collection, which is addressed in a contagious memory location.");

    let array: [i32; 5] = [0, 1, 2, 3, 4];

    let slice = &array[0..3];

    for x in slice {
        println!("x is {}", x);
    }
}

////////
//STR
////////
fn Str(){
    println!("A str is the string slice and is the most primitive data types.");

    let str = "Welcome to your first Rust string";
}

///////////
// Tuple
///////////
fn Tuple() {
    println!("Tupes are finite heterogenous, sequences. Tuples have a fixed size and a fixed number of elements. In the array, it contains data of the same type, but in the tuple there can we have different kinds of data types.");

    let tup:(i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let five_hundred = tup.0;
}

////////////////
// Mutability
////////////////
fn Mutability() {
    println!("By default, the variables are immutable in Rust, which means once the variable is assigned one value, it cannot be assigned another or different value in the same program. To overcome this, we can use the keyword mut to mute the variable and then the same variable can be used again.");

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

///////////////
// Constants
///////////////
fn Constants() {
    println!("In case of the constants, the value assigned to it, remains the same through the program and by no means, the value can be changed.the constants are global and can be accessed from anywhere inside the program.");

    const VALUE: U16= 500;
}

//////////////
// Shadowing
// ///////////
fn Shadows() {
    println!("Shadowing a variable means creating a new variable with the same name as the existing variable. Not only the variable can be used twice, but also the same variable's data type can be changed using the concept of shadowing.");

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
}

/////////////
// Pointer
/////////////
fn Pointer() {
    println!("A pointer is nothing but a variable which stores the memory location of another variable. For example on bello, a is holding the memory address of d.");
    
    let a= &d
}
