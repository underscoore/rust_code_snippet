fn main() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let vec = vec![1,2,3,4,5,6,7,8,9];

    for num in vec{
        println!("{:?}", num);

    }
    println!("New way");
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // `iter` - This borrows each element of the collection through each eteration.
    // Thus leaving the collection untouched and available for reuse after loop.
    println!("==> This is the example of `iter()` method on iterator");
    let names1 = vec!["Boob", "Frank", "Ferris"];

    for name in names1.iter(){
        match name {
            &"Ferris" => println!("There is a rustacean among us!!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("Names1 {:?}", names1);


    println!("==> This is the example of `into_iter()` method on iterator");
    let names2 = vec!["Boob", "Frank", "Ferris"];
    for name in names2.into_iter(){
        match name {
            "Ferris" => println!("There is a rustacean among us!!"),
            _ => println!("Hello {:?}", name),
        }
    }
    //    println!("Neams2: {:?}", names2); // Comment out this and see that the values are moved somewhere :>

    println!("==> This is the example of `into_mut()` method on iterator");
    let mut names3 = vec!["Boob", "Frank", "Ferris", "Ajay", "No Match 1", "No Match 2"];
    for name in names3.iter_mut(){
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!!",
            &mut "Boob" => "Boob is replaced with This String",
            &mut "Ajay" => "100 + 100",
            _ => "Hello",
        }
    }
    println!("Neams3: {:?}", names3);


}
