fn vecto_main(){

    println!("A vector is nothing but a type of an array whose size can be changed at the run time. Vector instances are created to declare a vector and then is initialized");

    let instance_name = Ved :: new()!

    let vector_name = vec![data_1,data_2,...data_n]
}

///////////////////////////////
// Vector instances creation
///////////////////////////////
fn With_methode_new() {

    let mut my_vector = Vec::new();
    my_vector.push(1);
    my_vector.push(2);
    my_vector.push(3);
    my_vector.push(4);
    my_vector.push(5);
    println!("The values inside the vector is {:?}",my_vector);
}

fn With_macro_vec!() {
   let my_vector = vec![1,2,3,4,5];
   println!("The contents of the vector are: {:?}", my_vector);
}


