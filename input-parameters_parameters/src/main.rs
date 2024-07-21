fn apply<F>(f: F) where  
    F: FnOnce(){
    f(); 
}

#[allow(dead_code)]
fn apply_to_3<F>(mut fun: F) -> i32 where 
    F:FnMut(i32) -> i32 {
    let num: i32 = 69;
    fun(num)
}

fn main() {
    use std::mem;
    let greeting = "Hello";
    let mut farewell = "goodBye".to_owned();
    let diary = || {
        println!("I said {}", greeting);
        farewell.push_str("!!!<|");
        println!("Then I scremed {}.", farewell);
        println!("Now I can sleep. zzzz, {}", farewell);
        mem::drop(farewell);
    };
    //diary(); 
    apply(diary);
    let double = |x| 2*x;
    println!("3 doubled: {} |>", apply_to_3(double))
}
