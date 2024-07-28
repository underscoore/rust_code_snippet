#[path = "../data.rs"] mod data;

pub fn get_max<T:PartialOrd + Copy>(list: &Vec<T>) -> T{
    let mut largest:T= list[0];
    for number in list{
        if number > &largest{
            largest = *number;
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use crate::operations::max::data;

    use super::get_max;

    #[test]
    fn unit_test_int_lrgst_t(){
        let result = get_max(&data::get_small_list());
        assert_eq!(result, 9);
    }
    #[test]
    fn unit_test_int_lrgst_f(){
        let result = get_max(&data::get_small_list());
        assert_ne!(result, 8);
    }
    #[test]
    fn unit_tests_float_lrgst(){
        let result = get_max(&data::get_float_list());
        assert_eq!(result, 5.5);
    }
    #[test]
    fn unit_tests_char_lrgst(){
        let result = get_max(&data::get_char_list());
        assert_eq!(result, 'y');
    }
}

