fn main() {
    let n = 1500;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase the ten-fold");
            10 * n
        } else {
            println!(", and is bigger number, half the number");
            n / 2 
        };

    println!("{} -> {}", n, big_n);
}
