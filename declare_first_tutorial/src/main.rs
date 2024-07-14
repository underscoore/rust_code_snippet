fn main() {
    let a_binding: String;
    //println!("Printing Variable before declare: {}", a_binding);
    {
        //    vprintln!("Printing Variable before declare: {}", a_binding); << This will generate an error
        let a_binding = "This is declared inside the block".to_string();
        println!("Printing variable after the initialize inside the block: {}", a_binding);
    }

    //println!("Printing Variable before declare but after the block: {}", a_binding); //<< This will generate an error
    // declaring the variable outside the block
    a_binding = "This is declared outside the block".to_string();
    println!("Printing Variable before declare but after the block: {}", a_binding); //<< This will generate an error

}
