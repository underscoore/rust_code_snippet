use integration_testing::operatons::addition::add;

#[test]
fn test_add(){
    let result = add(100, 100);
    assert_eq!(result, 200);
}  
