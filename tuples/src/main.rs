fn reverse(pair: (i32, bool)) -> (bool, i32){
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64, 
                      -1i8, -2i16, -3i32, 4i64, 
                      0.1f32, 0.2f64,
                      'a', true);
    //accessing the single value of abovr tupple
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);
    println!("Long tuple last value: {}", long_tuple.11);

    let tuple_of_tuple = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16 );
    println!("tuple of tuple: {:?}", tuple_of_tuple);
    let pair =(1, true);
    println!("Pair is {:?}", pair);
    println!("The reverse pair is {:?}", reverse(pair));

    // To create one element tuple, the comma is required
    // from a literal surrounded by parantheses

    println!("One element tuple: {:?}", (5u32, ));
    println!("Just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let(a, b, c, d) = tuple;

    println!("{:?}, {:?}, {}, {}", a, b, c, d);

    let matrix = Matrix(1.1, 2.2, 3.3, 4.4);
    println!("{:?}", matrix);
}
