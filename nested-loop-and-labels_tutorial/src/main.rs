#![allow(unreachable_code, unsed_labels)]
fn main() {
    'outer: loop {
        println!("Enter the outer loop!");

        'inner: loop {
            println!("Entered the inner loop!");

            // This would break only inner loop
            // break;

            // this breaks the outer loop
            break 'outer;
        }
        println!("This point will never be reached");

    }
    println!("Exited the outer loop!");
    println!("RUNNING AGAIN");

}
