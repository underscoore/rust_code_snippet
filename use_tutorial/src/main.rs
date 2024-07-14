#![allow(dead_code)]

enum Stage{
    Begineer,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

enum Number {
Zero,
One,
Seven,
Ten,
}
fn main() {
    // Explicitly 'use' each name so they are avaiable without
    // manual scoping.
    use crate::Stage::{Begineer, Advanced};
    // Automatically 'use' each name inside 'Role'
    use crate::Role::*;

    // Equivalent to 'Stage::Begineer'
    let stage = Advanced;

    // Equivalent to 'Role::Student'
    let role = Student;

    match stage {
        //Note the lack of scoping bacause of the explicit 'use' above.
        Begineer => println!("Begineers are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastring their subject..."),
    }

    match role {
        // Note again the lack of scope/
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }

    // enums can be cast in to integers
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("seven is {}", Number::Seven as i32);
    println!("ten is {}", Number::Ten as i32);
}
