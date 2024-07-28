#[path = "../data.rs"] mod data;
// This algorithm shorts the values
pub fn dsc_sorting<T: PartialOrd + Copy>(list: &Vec<T>) -> Vec<T> {
    let mut list = list.clone();
    for i in 0..list.len() {
        for j in i + 1..list.len() {
            if list[i] < list[j] {
                let tem = list[i];
                list[i] = list[j];
                list[j] = tem;
            }
        }
    }
    list
}

pub fn asc_sorting<T: PartialOrd + Copy>(list: &Vec<T>) -> Vec<T> {
    let mut list = list.clone();
    for i in 0..list.len() {
        for j in i + 1..list.len() {
            if list[i] > list[j] {
                let tem = list[i];
                list[i] = list[j];
                list[j] = tem;
            }
        }
    }
    list
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn unit_dsc_test(){
        let result = dsc_sorting(&data::list());
        assert_eq!(result, [5,4,3,2,1])
    }
    #[test]
    fn unit_asc_test(){
        let result = asc_sorting(&data::list());
        assert_eq!(result, [1,2,3,4,5])
    }

}
