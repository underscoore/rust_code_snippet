use crate::operatons::structs::Number;

pub fn mul(na: i32, nb: i32) -> i32{
    let value: Number = Number {a: na, b: nb};
    value.a * value.b
}

#[cfg(test)]
mod test {
    use super::mul;

    #[test]
    fn mul_test(){
        let result = mul(20, 20);
        assert_eq!(result, 400);
    }
}
