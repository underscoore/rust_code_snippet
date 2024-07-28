pub struct Val{
    pub    val: f64,
}

pub struct GenVal<T>{
    pub    gen_val: T,
}

impl Val {
    pub  fn value(&self) -> &f64 {
        &self.val
    } 
}

impl<T> GenVal<T> {
    pub fn value(&self) -> &T{
        &self.gen_val
    } 
}

//Unit test

#[cfg(test)]
mod tests {
    use super::{Val, GenVal};

    #[test]
    fn unit_test_val(){
        let result = Val{val: 100.100};
        assert_eq!(result.value(), &100.100)
    }

    #[test]
    fn unit_test_gen_val(){
        let result = GenVal{gen_val: "This is String"};
        assert_eq!(result.value(), &"This is String");
    }

}
