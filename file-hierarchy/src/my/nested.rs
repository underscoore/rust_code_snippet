pub fn function(){
    println!("Called |>`my::nested::function()`");
    println!("Shipping from nested mod")
}

#[allow(dead_code)]
fn private_function(){
    println!("Called |>`my::nested::private_function()`");
}
