fn create_fn() -> impl Fn(){ // these functions can not called directly!
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut(){
    let text = "FnMut".to_owned();

    move ||  println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce(){
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    //putting them into a variable!!
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
