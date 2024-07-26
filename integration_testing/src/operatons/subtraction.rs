use crate::operatons::structs::Number;

pub fn sub(na: i32, nb: i32) -> i32 {
    let value: Number = Number {a: na, b: nb };

    match value {
        Number {a, b}  if a > b => a - b,
        Number { a, b } if a < b => b - a,
        _ => 0,
    }
 
}

#[cfg(test)]
mod tests {
    use super::sub;

    #[test]
    fn a_b(){
        let result = sub(10, 5);
        assert_eq!(result, 5);
    }

    #[test]
    fn b_a(){
        let result = sub(10, 20);
        assert_eq!(result, 10);
    }

    #[test]
    fn eq(){
        let result = sub(10, 10);
        assert_eq!(result, 0);
    }

    #[test]
    fn ueq(){
        let result = sub(100, 50);
        assert_ne!(result, 100);
    }
}
