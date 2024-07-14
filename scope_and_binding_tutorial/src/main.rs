fn main() {
    // === Scope ===
    // This binding lives in the main function
    let long_live_binding = 1;

    // This is a block, and has a small scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding =2;
        println!("inner short: {}", short_lived_binding);
    }
    // end of the block

    // error!
    //println!("inner short: {}", short_lived_binding);
    println!("outer long: {}", long_live_binding);

    // === Shadowing ===

    let shadowed_binding = 1;
    let sb = "STRING";
    {
        println!("before being shadowed: {}", shadowed_binding);
        println!("string coming from the outside the block: {}", sb);
        let shadowed_binding = "abc";
        let sb = "UPDATED INSIDE THE BLOCK";

        println!("Shadowed in the inner block: {}", shadowed_binding);
        println!("Updated string updated inside the block: {}", sb);
    }

    println!("Outside innder block: {}", shadowed_binding);

    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);

    println!("Actual String: {}", sb);
    let sb = ("UPDATED OUTSIDE THE BLOCK");
    println!("Actual String: {}", sb);


}
