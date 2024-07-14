fn main() {
    let s1 = String::from("Hello, World");
    println!("{}", s1);

    let s2 = s1;
    println!("{}",s2);

    //println!("{}",s1);

    let s = String::from("Ajay");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    let s3 = gives_ownership();
    let s4 = String::from("Kumar");
    let s5 = takes_and_give_back(s4);
    println!("{s3}");
    println!("{s5}");

    let s6 = String::from("New String S6");
    //Using referencing concept
    let (s8, len) = r_calculate_length(&s6);
    println!("This is the lenght of {len} referenced string '{s8}'");
    let (s7, len) = calculate_length(s6); //here we are giving the ownsership to the function and getting back new values
    println!("The Length of '{s7}' is {len}.");

}

fn takes_ownership(some_string: String){
    println!("{some_string}");
}

fn makes_copy(some_integer: i32){
    println!("{some_integer}");
}

fn gives_ownership() -> String{
    let some_string = String::from("Coming from 'gives_owership function!!");
    some_string
}

fn takes_and_give_back(a_string: String) -> String{
    a_string
} 

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}

fn r_calculate_length(s: &String) -> (String, usize){
    let length = s.len();

    ((&s).to_string(), length)
}
