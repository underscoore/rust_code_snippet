pub fn public_function(){
    println!("Called rary's `public_fnction()`");
}

fn private_function(){
    println!("Called rary's `private_function`");
}

pub fn indirect_function(){
    println!("Called rary's `inderict_function`");
    private_function();
}
