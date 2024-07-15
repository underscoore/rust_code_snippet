fn some_number() -> Option<u32> {
    Some(00)
}

fn age() -> u32 {
    23 
}

fn main() {
    println!("Tell me what type if person you are!!");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet!!"),
        n @ 1 ..=12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..=19 => println!("I'm a teen a age {:?}", n),
        n => println!("I'm older person of age {:?}", n),
    }

    match some_number() {
        Some(n @ 42 ) => println!("The Answer: {}!", n),
        Some(n) => println!("Not Interested... {}", n),
        _ => (),
    }
}
