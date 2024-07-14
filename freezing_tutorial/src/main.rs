fn main() {
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;
println!("Printing the mutable varibale inside the block: {}", _mutable_integer);
       //_mutable_integer = 50; 
    }

    _mutable_integer = 10i32;
    println!("Printing the mutable variable outside the block: {}", _mutable_integer);
}
