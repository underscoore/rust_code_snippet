#![allow(overflowing_literals)]

fn main(){
    let decimal = 65.4321_f32;

    //let integer: u8 = decimal; // this is error prone!


    let integer = decimal as u8;
    println!("This is integer: {integer}");
    let character = integer as char;
    println!("This is character: {character}");
    println!("So, the whole transition is: {decimal} -> {integer} -> {character}");

    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("10000 as a u16 is {}", 10000 as u16);
    println!("100000 as a u16 is {}", 100000 as u16);

    println!("1000 as a u8 is: {}", 1000 as u8);
    println!(" -1 as a u8 is: {}", (-1i8) as u8);
    println!(" -55 as a u8 is: {}", (-56i8) as u8);
    println!("1000 mod 256 is: {}", 1000%256);

    println!("128 as a i16 is: {}", 128 as i16);
    println!("128 as a i16 is: {}", 130 as i16);

    println!("1000 as a u8 is: {}", 1000 as u8);
    println!("232 as a i8 is: {}", 232 as i8);

    println!("300.0 as u8 is: {}", 300.0_f32 as u8); //255
    println!("-100.0 as u8 is: {}", -100.0_f32 as u8); //0
    println!("NAN as u8 is:{}", f32::NAN as u8); // 0


    unsafe {
        println!("300.0 as u8 is: {}", 300.0_f32.to_int_unchecked::<u8>()); // 44 = 300-256
        println!("-100.0 as u8 is: {}", (-100.0_f32).to_int_unchecked::<u8>());  // 156 = 256-100
        println!("NAN as u8 is: {}", f32::NAN.to_int_unchecked::<u8>()); //0
    }

}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
} 
