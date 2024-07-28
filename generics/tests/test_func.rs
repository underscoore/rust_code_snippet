use generics::operations::max::get_max;
#[test]
fn intg_test_int_max_t(){
    let number_list: Vec<i32> = vec![1,2,3,4,5,6,7,8,9];
    let result = get_max(&number_list);
    assert_eq!(result, 9);
}
#[test]
fn intg_test_int_max_f(){
    let number_list: Vec<i32> = vec![1,2,3,4,5,6,7,8,9];
    let result = get_max(&number_list);
    assert_ne!(result, 8);
}
#[test]
fn intg_test_float_max(){
    let number_list: Vec<f32> = vec![1.1,2.2,3.3,4.4,5.5,1.1];
    let result = get_max(&number_list);
    assert_eq!(result, 5.5);
}
#[test]
fn intg_test_char_max_char(){
    let number_list: Vec<char> = vec!['a','q','w', 'e', 'r', 't', 'y'];
    let result = get_max(&number_list);
    assert_eq!(result, 'y');
}


