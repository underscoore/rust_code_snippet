fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // infinite loop!

    loop {
        count +=1;

        if count == 3 {
            println!("three");

            // skip the rest of this iteration
            continue;
        }

        println!("{}", count);
        
        if count == 5 {
            println!("OK, that's enough!");

            // exit this loop
            break;
        }
    }
}
