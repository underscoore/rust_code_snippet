enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

const OPTIONAL:Option<i32> = Some(7); 
fn main() {
    match OPTIONAL {
        Some(i) => {
            println!("This is a really long string and {:?}", i);
        },
        _ => {},
    }

    let number = Some(7);

    let letter: Option<i32> = Some(12);
    let emoticon: Option<i32> = None;
    let i_like_it = true;

    if let Some(i) = number {
        println!("Matched {:?}", i);
    }
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter.");
    }
    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_it {
        println!("Didn't match number. Let's go with a letter");
    } else {
        println!("I don't like letters. Let's go with the emoticon :");
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Baz = b{
        println!("b is foobaz")
    }

    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred {}", value);
    }
}
