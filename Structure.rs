fn Structure_main() {
    println!("A Structure is a user-defined data type that represents not only a single data type but can represent multiple data types using the same structure name.");
}

//////////////////////////////
// Structure initialization
/////////////////////////////
fn Structure_initialization(){

    let value = Structure__name {
    field_1: data1,
    field_2: data2
    };
}

//////////////////////////////////////////
// Example Programme print student info
///////////////////////////////////////////
struct Student {
  name: String,
  school: String,
  roll: u32,
  grade: char

}
fn Example_struct() {
  let value = Student {
    name: String::from("Fitas"),
    school: String::from("Chercher.tech"),
    roll: 01,
    grade: 'A'

  };
  println!("Student: {} of {} school bearing roll {} has obtained {} grade", value.name, value.school, value.roll, value.grade);
}

//////////////////////////////////////////
// Structure as function parmeter 
//////////////////////////////////////////
struct Student {
    name: String,
    school: String,
    roll: u32,
    grade: char
}
fn main() {
    let value1 = Student {
        name: String::from("Fitas"),
        school: String::from("Chercher.tech"),
        roll: 01,
        grade: 'A'
    };
    display(value1);
    fn display(value: Student) {
        println!("Student: {} of {} school bearing roll {} has obtained {} grade", value.name, value.school, value.roll, value.grade);
    }
}

/////////////////////////////////////////////
// Initializing structure out of its scope
/////////////////////////////////////////////
struct Student {
    name: String,
    school: String,
    roll: u32,
    grade: char
}
fn main() {
    println!("In the program, the value is an instance of the structure Student. And to overwrite the field grade we have muted the instance, and then reassigned the variable value with a character C outside the structure initialization.");
    let mut value = Student {
        name: String::from("Fitas"),
        school: String::from("Chercher.tech"),
        roll: 01,
        grade: 'A'
    };
    value.grade = 'C';
    println!("Student: {} of {} school bearing roll {} has obtained {} grade", value.name, value.school, value.roll, value.grade);
}

/////////////////////////////////////////////
// Returning structure using a function
/////////////////////////////////////////////
fn function_name(instance_name: Structure_name) -> Structure_Name {
    println!("A structure instance can be returned by a function, in the following way");
    return instance_name;
}
// Example
struct Student {
    name: String,
    school: String,
    roll: u32,
    grade: char
}
fn main() {
    fn student_detail() - > Student {
        let value1 = Student {
            name: String::from("Fitas"),
            school: String::from("Chercher.tech"),
            roll: 01,
            grade: 'A'
        };
        return value1;
    }
    let order = student_detail();
    println!("Student: {} of {} school bearing roll {} has obtained {} grade", order.name, order.school, order.roll, order.grade);
    println!("the details in which roll is less is printed above");
}
