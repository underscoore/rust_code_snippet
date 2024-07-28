#[path = "../operations/n_sorting.rs"] mod n_sorting;
#[path = "../data.rs"] mod data;

pub struct SortingStruct<L>{
    list: Vec<L>,
}

pub fn d_sorting<T: PartialOrd + Copy>(list: &Vec<T>) -> Vec<T> {
    let list = SortingStruct {list: list.clone()};
    n_sorting::dsc_sorting(&list.list)
}

pub fn a_sorting<T: PartialOrd + Copy>(list: &Vec<T>) -> Vec<T> {
    let list = SortingStruct {list: list.clone()};
    n_sorting::asc_sorting(&list.list)
}

#[cfg(test)]
mod tests {
    use crate::sorting::sorting::{d_sorting, data, a_sorting};
    #[test]
    fn unit_test_s_d(){
        assert_eq!(d_sorting(&data::list()), [5,4,3,2,1]);        
    }
    #[test]
    fn unit_test_s_a(){
        assert_eq!(a_sorting(&data::list()), [1,2,3,4,5]);        
    }

}

