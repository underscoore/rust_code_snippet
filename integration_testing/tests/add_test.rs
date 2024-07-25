use operations::addition::add;

#[test]
fn test_add(){
    let result = operations::addition::add(100, 100);
    assert_eq!(result, 200);
}  
