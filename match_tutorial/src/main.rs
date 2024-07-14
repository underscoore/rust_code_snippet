struct Foo{
    x:(u32, u32),
    y: u32,
}

#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name
    Red,
    Blue,
    Green,

    //These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}


fn main() {
    // match is like a switch in other language
    //
    let number = 11;

    println!("Tell  me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is prime number!!"),
        13..=19 => println!("A Teen"), 
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);

    println!("=== Destructuring the tuple");
    let triple = (5, -2, 4);
    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("Last is 2 and the rest doesn't matter"),
        (3, .., 4) => println!("First is 3 and last is 4 and rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }

    println!("\n=== Like tuple arrays and slices can be destructured this way\n");
    let array = [0, -2, 6];

    match array {
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        [1, _, third] => println!("array[0] = 1, array[2] = {} and array[1] was ignored", third),
        [-1, second, ..] => println!("array[0] = -1, array[1] = {} and all the other ones are ignored", second),
        [3, second, tail @ ..] => println!("array[0] = 3, array[1] = {} and the other elements were {:?}", second, tail),
        [first, middle @ .., last] => println!("array[0] = {}, middle = {:?} and array[2] = {}", first, middle, last),
        _ => println!("Temp Cases"),
    }

    println!("\n=== Like tuple enums can be destructured!\n");

    let color = Color::RGB(122,17,40);

    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, Green: {}, Blue:  {}", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, Saturation: {}, Value: {}", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, Saturation: {}, lightness: {}", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, Magenta: {}, Yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) => println!("Cyan: {}, Magenta: {}, Yellow: {}, Key (black): {}", c, m, y, k),
    }

    println!("\n=== Pointes/refs!\n");

    // Assign a reference of type `i32`. The & signifies there
    // is a referene being assigned
    let reference = &7u32;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let foo = Foo {x: (3,2), y: 2};

    match foo {
        Foo { x: (1, b), y } => println!(" First of x is 1, b ={}, y={}", b, y),
        Foo {y: 2, x: i} => println!("y is 2, i = {:?}", i),
        _ => print!("Not found")
    }
}
