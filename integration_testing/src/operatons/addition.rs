use crate::operatons::structs::Number;

pub fn add(na: i32, nb: i32) -> i32{
  let value: Number = Number {a: na, b: nb};
    value.a + value.b
}

#[cfg(test)]
mod tests{
   use super::add;

    #[test]
    fn addition_test(){
        let result = add(20, 80);
        assert_eq!(result, 100);
    }
}
