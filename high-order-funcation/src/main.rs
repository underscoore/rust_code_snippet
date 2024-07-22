fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn imparitive_approach(upper: u32) -> u32{
    //imparitive approach
    //Declare accumator variable
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
           acc += n_squared;
        }
    }
    acc
}

fn functional_approach(upper: u32) -> u32{
    //functional approach
    let sum_of_squared_odd_number: u32 = 
    (0..).map(|n| n*n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .sum();
    sum_of_squared_odd_number
}
fn main() {
    let upper = 1000;
    println!("Imparitive Style: {}", imparitive_approach(upper));
    println!("Functional Style: {}", functional_approach(upper));
}


#[cfg(test)]
mod tests{
    use crate::{functional_approach, imparitive_approach};
    #[test]
    fn trad_style(){
        let result = imparitive_approach(100);
        assert_eq!(result, 165);
    }

    #[test]
    fn fun_style(){
    let result = functional_approach(150);
        assert_eq!(result, 286)
    }
}
