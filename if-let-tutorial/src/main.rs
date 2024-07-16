// Enum example
enum Foo {
    Bar,
    Baz,
    Quz(u32)
}
fn main() {
    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // Some(i), evaluate the block (`{}`).

    if let Some(i) = number {
        println!("Matched {:?}!", i)
    }

    // if you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched2 {:?}!", i)
    } else {
        // destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failiing condition/
    let i_like_letter = false;

    if let Some(i) = emoticon{
        println!("Match3 {:?}", i);
    } else if i_like_letter {
        println!("Didn't match a number. Let's go with a letter");
    } else {
        // The condition evaluateed false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    // Create example variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Quz(100);

    // varaible a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is Foobar");
    }

    // variables b does not match Foo::Bar
    // So this will print nothing!
    if let Foo::Bar = b {
        println!("b is not foobar");
    }

    if let Foo::Quz(value @ 100) = c {
        println!("c is one hundred {}", value);
    }

}
