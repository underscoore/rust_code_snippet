fn call_me<F: Fn()>(fun: F){
    fun();
}

fn function(){
    println!("I'm a function");
}
fn main() {

    let closure = ||println!("I'm closure");
    call_me(closure);
    call_me(function)
}
