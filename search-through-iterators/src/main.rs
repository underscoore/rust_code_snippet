fn main() {
    let vec1 = vec![1,2,3];
    let vec2 = vec![4,5,6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1,2,3];
    let array2 = [4,5,6];

    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x ==3));
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| x ==2));

    println!("=== some more!!");

    let vec = vec![1,9,3,3,13,2];

    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}

#[cfg(test)]
mod tests {
    use std::ops::{Add, Div, Mul, Sub};

    #[test]
    fn it_works() {
        let result = 2.add(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2(){
        let result = 3.sub(2);
        assert_eq!(result, 1);
    }

    #[test]
    fn its_dev(){
        let result = 10.div(2);
        assert_eq!(result, 5)
    }

    #[test]
    fn its_divf(){
        let result = 10.div(4);
        assert_eq!(result, 4);
    }

    #[test]
    fn its_mul(){
        let result = 10.mul(10);
        assert_eq!(result, 100);
    }
}
