/////////
// IF
// //////
fn IF_STATEMENT() {
    let condition
    let number = if condition {
        5
    } else {
        6
    };

    if n < 0 {
            print!("{} is negative", n);
        } else if n > 0 {
            print!("{} is positive", n);
        } else {
            print!("{} is zero", n);
        }
}

/////////////////
// Match (Cases) 
// //////////////
fn Match() 
{
    let my_string = "hello";

    match my_string 
    {
        "bonjour" => {
            println!("francais");
        }
        "ciao" => {
            println!("italien");
        }
        "hello" => {
            println!("anglais");
        }
        "hola" => {
            println!("espagnol");
        }
        _ => {
            println!("je ne connais pas cette langue...");
        }
    }
}

//////////
// loop
// ///////
fn main() {
    loop {
        println!("again!");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2 ;
        }
    };
        println!("The result is {}", result);
}

/////////////
// For loop
/////////////
fn For_with_continue()
{
for value in 0..10 {
    if value % 2 == 0 { continue; }

    println!("{}", value);
}
}

/////////
// While
// /////
fn While_loop()
{
    let mut value = 5; // mut x: i32
    let mut bool_variable = false; // mut done: bool

while !bool_variable {
    value=value + 3;

    println!("{}", value);

if value % 5 == 0 {
        bool_variable = true;
    }
}
}

