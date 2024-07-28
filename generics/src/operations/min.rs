#[path = "../data.rs"] mod data;

pub fn get_min<T: PartialOrd + Copy>(list: &Vec<T>) -> T{
let mut minimum:T = list[0];
    for number in list{
        if number < &minimum{
            minimum = *number;
        } 
    }
    minimum
}

#[cfg(test)]
mod tests{
    use crate::operations::min::data;
    use super::get_min;

    #[test]
    fn unit_test_min(){
    let result = get_min(&data::get_small_list());
        assert_eq!(result, 1);
    }

}
