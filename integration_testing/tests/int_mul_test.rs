use integration_testing::operatons::multiplication::mul;

#[test]
fn mul_test(){
    let result = mul(30, 30);
    assert_eq!(result, 900);
}
