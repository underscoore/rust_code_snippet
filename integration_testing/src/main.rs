mod operatons;

use operatons::{addition, multiplication, subtraction};

fn main() {
    let add = addition::add(1012, 1038);
    println!("Additioon |> : {add}");

    let sub = subtraction::sub(1000, 500);
    println!("Subtraction |> : {sub}");

    let sub = subtraction::sub(1000, 5000);
    println!("Subtraction |> : {sub}");

    let mul = multiplication::mul(100, 200);
    println!("Multiplication |> : {}", mul);
}
